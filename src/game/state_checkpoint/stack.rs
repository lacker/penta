use super::model::{
    AppliedStackEffectSnapshot, BasicLandTypeChangeSnapshot, CastSignatureSnapshot,
    CounterKindSnapshot, DecisionCardOriginSnapshot, DetachedStackSnapshot,
    EffectResolutionContextSnapshot, ManaSourceSnapshot, SeatSnapshot, SpellFormSnapshot,
    StackAbilitySnapshot, StackObjectKindSnapshot, StackSnapshot, TargetSelectionSnapshot,
    TargetSnapshot, TriggerContextSnapshot,
};
use super::semantics::{
    ability_locator_for_origin, applied_effect_locator, catalog_applied_effect,
    catalog_scoped_effect, scoped_effect_snapshot,
};
use super::{
    AbilityId, AbilityOrigin, AbilitySourceRef, AdditionalCostId, AlternativeCostId,
    AppliedStackEffect, BasicLandType, BasicLandTypeChange, CardPartId, CastChoices, CastSignature,
    CharacteristicSource, CostConfiguration, DeclarativeAbilityDef, EffectResolutionContext, Game,
    GameObjectId, GameStack, GrantId, ManaSource, ModeId, ObjectBacking, ObjectCharacteristics,
    ObjectInstance, ObjectKind, PlayOptionId, PlayerId, RetiredObject, SpellForm,
    StackAbilityPayload, StackObject, StackObjectKind, Target, TargetSelection, TriggerContext,
    Value, ability_locator, ability_origin_from_snapshot, ability_origin_snapshot,
    ability_target_defs, array, basic_land_type_snapshot, card, card_definition_id_field,
    cast_source_zone_from_label, catalog_ability, face_down_characteristics_from_snapshot,
    face_down_characteristics_snapshot, field, object_characteristics_from_snapshot,
    object_characteristics_snapshot, object_kind_from_snapshot, object_kind_snapshot, optional_id,
    parse_basic_land_type, parse_cast_signature, parse_ids, parse_zone_kind, seat_value, str_field,
    u8_field, u32_field, usize_field, zone_kind_snapshot,
};
use crate::card::{ColorSet, ManaColor};

mod ability_kind;
use ability_kind::{StackAbilityCondition, stack_ability_condition, stack_payload_matches};
mod current;
mod hidden_references;
pub(super) use current::current_stack_snapshot;
pub(in crate::game::state_checkpoint) use hidden_references::*;

/// The live stack records where a hidden source sits; everything detached
/// from it does not, and leaves its payload out instead.
///
/// Only the objects on the stack proper are put back through
/// [`super::wire_decision::rebind_stack_source_cards`], so only they can
/// promise that the position they wrote down is honoured.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum HiddenSourcePolicy {
    Locate,
    Refuse,
}

pub(super) fn stack_ability_snapshot(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
) -> Option<StackAbilitySnapshot> {
    stack_ability_snapshot_with(game, viewer, object, &[], HiddenSourcePolicy::Locate)
}

pub(super) fn stack_ability_snapshot_allowing(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
    visible_rebindings: &[GameObjectId],
) -> Option<StackAbilitySnapshot> {
    stack_ability_snapshot_with(
        game,
        viewer,
        object,
        visible_rebindings,
        HiddenSourcePolicy::Refuse,
    )
}

fn stack_ability_snapshot_with(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
    visible_rebindings: &[GameObjectId],
    hidden_source: HiddenSourcePolicy,
) -> Option<StackAbilitySnapshot> {
    let payload = object.ability.as_ref()?;
    // A source the viewer cannot read is carried by where it sits rather
    // than by its object id: the importer mints those zones fresh from the
    // hypothesis it was handed, so the id it left here would name nothing.
    let mut source_origin = None;
    if let Some(source) = object.source
        && stack_source_requires_hidden_rebinding(game, viewer, source)
        && !visible_rebindings.contains(&source)
    {
        if hidden_source == HiddenSourcePolicy::Refuse {
            return None;
        }
        let (seat, zone, index) = super::decision::hidden_card_origin(game, source)?;
        source_origin = Some(DecisionCardOriginSnapshot {
            object_id: source.0,
            seat: seat.index(),
            zone,
            index,
        });
    }
    if stack_payload_has_unrebindable_hidden_reference_except(
        game,
        viewer,
        payload,
        visible_rebindings,
    ) {
        return None;
    }
    let locator = ability_locator_for_origin(&game.catalog, payload.origin, |candidate| {
        stack_payload_matches(payload, candidate)
    })?;
    let target_definition_locator = ability_locator(&game.catalog, |candidate| {
        ability_target_defs(candidate) == payload.target_defs
    })?;
    let ability = catalog_ability(&game.catalog, &locator)?;
    Some(StackAbilitySnapshot {
        ability_locator: Some(locator),
        target_definition_locator: Some(target_definition_locator),
        origin: ability_origin_snapshot(payload.origin),
        presentation: object_characteristics_snapshot(&game.catalog, payload.presentation)?,
        target_selections: payload
            .targets
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        source_origin,
        context: effect_resolution_context_snapshot(&payload.context),
        mode_effects: payload
            .mode_effects
            .iter()
            .copied()
            .map(|effect| scoped_effect_snapshot(&ability, effect))
            .collect::<Option<Vec<_>>>()?,
        x: payload.x,
        sacrificed_mana_value: payload.sacrificed_mana_value,
    })
}

pub(super) fn detached_stack_snapshot_allowing(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
    visible_rebindings: &[GameObjectId],
) -> Option<DetachedStackSnapshot> {
    let ability_payload = if object.kind != StackObjectKind::Spell && object.ability.is_some() {
        Some(stack_ability_snapshot_allowing(
            game,
            viewer,
            object,
            visible_rebindings,
        )?)
    } else {
        None
    };
    if ability_payload
        .as_ref()
        .is_some_and(|payload| payload.ability_locator.is_none())
    {
        return None;
    }
    let (applied_effects, has_runtime_overrides) = applied_stack_effect_snapshots(game, object);
    let face_down = object
        .face_down
        .and_then(face_down_characteristics_snapshot);
    if object.face_down.is_some() && face_down.is_none() {
        return None;
    }
    Some(DetachedStackSnapshot {
        object_id: object.id.0,
        kind: kind_snapshot(object.kind),
        object_kind: object_kind_snapshot(object.card.definition),
        owner: object.card.owner.index(),
        source: object.source.map(|id| id.0),
        ability_payload,
        controller: object.controller.index(),
        signature: object.signature.as_ref().map(signature_snapshot),
        chosen_permanents: object.chosen_permanents.iter().map(|id| id.0).collect(),
        has_runtime_overrides,
        applied_effects,
        text_changes: object
            .text_changes
            .iter()
            .map(|change| super::model::BasicLandTypeChangeSnapshot {
                from: super::basic_land_type_snapshot(change.from),
                to: super::basic_land_type_snapshot(change.to),
            })
            .collect(),
        colors: object.colors.map(ColorSet::to_flags),
        colors_of_mana_spent: object.colors_of_mana_spent.to_flags(),
        phyrexian_symbols_paid_with_life: object.phyrexian_symbols_paid_with_life,
        cast_via_flashback: object.cast_via_flashback,
        cast_via_suspend: object.cast_via_suspend,
        cast_at_instant_speed: object.cast_at_instant_speed,
        cast_from_zone: object.cast_from_zone.map(|zone| zone.label().to_owned()),
        face_down,
        is_copy: object.is_copy,
    })
}

pub(super) fn applied_stack_effect_snapshots(
    game: &Game,
    object: &StackObject,
) -> (Vec<AppliedStackEffectSnapshot>, bool) {
    let snapshots = object
        .applied_effects
        .iter()
        .filter_map(|applied| {
            Some(AppliedStackEffectSnapshot {
                source: applied.source.map(|source| ManaSourceSnapshot {
                    object: source.object.0,
                    ability: ability_origin_snapshot(source.ability),
                }),
                effect: applied_effect_locator(&game.catalog, applied.effect)?,
                granting: applied.granting.map(super::event::ability_source_snapshot),
            })
        })
        .collect::<Vec<_>>();
    let has_runtime_overrides = snapshots.len() != object.applied_effects.len();
    (snapshots, has_runtime_overrides)
}

fn parse_applied_stack_effects(
    snapshots: &[AppliedStackEffectSnapshot],
    game: &Game,
) -> Result<Vec<AppliedStackEffect>, String> {
    snapshots
        .iter()
        .map(|snapshot| {
            Ok(AppliedStackEffect {
                source: snapshot.source.map(|source| ManaSource {
                    object: GameObjectId(source.object),
                    ability: ability_origin_from_snapshot(source.ability),
                }),
                granting: snapshot.granting.map(|source| AbilitySourceRef {
                    object: GameObjectId(source.object),
                    ability: ability_origin_from_snapshot(source.ability),
                }),
                effect: catalog_applied_effect(&game.catalog, &snapshot.effect)
                    .ok_or("stack applied-effect locator is absent from this catalog")?,
            })
        })
        .collect()
}

fn kind_snapshot(kind: StackObjectKind) -> StackObjectKindSnapshot {
    match kind {
        StackObjectKind::Spell => StackObjectKindSnapshot::Spell,
        StackObjectKind::ActivatedAbility => StackObjectKindSnapshot::ActivatedAbility,
        StackObjectKind::TriggeredAbility => StackObjectKindSnapshot::TriggeredAbility,
    }
}

fn signature_snapshot(signature: &CastSignature) -> CastSignatureSnapshot {
    CastSignatureSnapshot {
        play_option: signature.play_option().0,
        form: match signature.form() {
            SpellForm::Part(part) => SpellFormSnapshot::Part { part_id: part.0 },
            SpellForm::Combined(parts) => SpellFormSnapshot::Combined {
                part_ids: parts.iter().map(|part| part.0).collect(),
            },
        },
        modes: signature.modes().iter().map(|mode| mode.0).collect(),
        alternative_cost: signature.costs().alternative().map(|cost| cost.0),
        additional_costs: signature
            .costs()
            .additional()
            .iter()
            .map(|cost| cost.0)
            .collect(),
        x: signature.x(),
        targets: signature
            .targets()
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        spliced: signature.spliced().iter().map(|card| card.0).collect(),
    }
}

pub(super) fn target_selection_snapshot(selection: &TargetSelection) -> TargetSelectionSnapshot {
    TargetSelectionSnapshot {
        slot_id: selection.slot().0,
        targets: selection
            .targets()
            .iter()
            .copied()
            .map(target_snapshot)
            .collect(),
        amounts: selection.amounts().to_vec(),
    }
}

pub(super) fn target_snapshot(target: Target) -> TargetSnapshot {
    match target {
        Target::Player(player) => TargetSnapshot::Player {
            seat: if player == PlayerId::One {
                SeatSnapshot::One
            } else {
                SeatSnapshot::Two
            },
        },
        Target::Card(id) => TargetSnapshot::Card { object_id: id.0 },
        Target::Permanent(id) => TargetSnapshot::Permanent { object_id: id.0 },
        Target::Spell(id) => TargetSnapshot::Spell { object_id: id.0 },
    }
}

pub(super) fn trigger_context_snapshot(context: TriggerContext) -> TriggerContextSnapshot {
    TriggerContextSnapshot {
        object: context.object.map(|id| id.0),
        zone_change_result: context.zone_change_result.map(|id| id.0),
        object_controller: context.object_controller.map(PlayerId::index),
        event_player: context.event_player.map(PlayerId::index),
        amount: context.amount,
        damaged_object: context.damaged_object.map(|id| id.0),
        cast_from_zone: context.cast_from_zone.map(zone_kind_snapshot),
    }
}

pub(super) fn effect_resolution_context_snapshot(
    context: &EffectResolutionContext,
) -> EffectResolutionContextSnapshot {
    EffectResolutionContextSnapshot {
        trigger: trigger_context_snapshot(context.trigger),
        chosen_counter: context.chosen_counter.map(CounterKindSnapshot),
        single_objects: std::array::from_fn(|index| {
            context.single_objects()[index].map(target_snapshot)
        }),
        object_groups: std::array::from_fn(|index| {
            context.object_groups()[index]
                .iter()
                .copied()
                .map(target_snapshot)
                .collect()
        }),
        named_object_groups: context
            .named_object_groups()
            .iter()
            .map(|(label, objects)| {
                (
                    label.clone(),
                    objects.iter().copied().map(target_snapshot).collect(),
                )
            })
            .collect(),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn parse_stack(
    observation: &Value,
    snapshots: &[StackSnapshot],
    game: &Game,
) -> Result<GameStack, String> {
    let visible = array(field(observation, "stack")?)?;
    if visible.len() != snapshots.len() {
        return Err("checkpoint stack does not match observation".into());
    }
    let mut stack = GameStack::default();
    for (shown, state) in visible.iter().zip(snapshots) {
        if state.has_runtime_overrides {
            return Err(
                "stack object has runtime overrides not yet represented by semantic locators"
                    .into(),
            );
        }
        if state.requires_retired_object && game.retired_objects.is_empty() {
            return Err(
                "stack object requires retired-object last-known information absent from the checkpoint"
                    .into(),
            );
        }
        let id = GameObjectId(u32_field(shown, "objectId")?);
        if id.0 != state.object_id {
            return Err("checkpoint stack id does not match observation".into());
        }
        let owner = seat_index_value(state.owner)?;
        let object_kind = object_kind_from_snapshot(state.object_kind, &game.catalog)?;
        let controller = seat_value(field(shown, "controller")?)?;
        let kind = match str_field(shown, "kind")? {
            "Spell" => StackObjectKind::Spell,
            "ActivatedAbility" => StackObjectKind::ActivatedAbility,
            "TriggeredAbility" => StackObjectKind::TriggeredAbility,
            other => return Err(format!("unknown stack object kind {other}")),
        };
        if state.kind != kind_snapshot(kind) {
            return Err("checkpoint stack kind does not match observation".into());
        }
        let (source, ability, signature, card) = match kind {
            StackObjectKind::Spell => {
                let ObjectKind::Card(definition) = object_kind else {
                    return Err("a spell checkpoint must be backed by a card".into());
                };
                let signature = parse_cast_signature(field(shown, "signature")?)?;
                let card = card(id, definition, owner, &game.catalog)?.into();
                (
                    None,
                    game.frozen_spell_payload(definition, &signature),
                    Some(signature),
                    card,
                )
            }
            StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
                if object_kind != ObjectKind::Ability {
                    return Err("a stack ability checkpoint must have ability object kind".into());
                }
                let payload_state = state
                    .ability_payload
                    .as_ref()
                    .ok_or("stack ability is missing its frozen payload")?;
                let observed_origin = parse_ability_origin(field(shown, "ability")?)?;
                let origin = ability_origin_from_snapshot(payload_state.origin);
                if origin != observed_origin {
                    return Err("checkpoint stack ability origin does not match observation".into());
                }
                if !super::semantics::ability_locator_matches_origin(
                    payload_state
                        .ability_locator
                        .as_ref()
                        .ok_or("stack ability lacks a semantic locator")?,
                    origin,
                ) {
                    return Err("checkpoint stack ability locator disagrees with its origin".into());
                }
                let source = optional_id(shown.get("sourceObjectId"));
                let definition_snapshot = payload_state
                    .ability_locator
                    .as_ref()
                    .and_then(|locator| catalog_ability(&game.catalog, locator))
                    .ok_or_else(|| {
                        "stack ability locator is absent from this catalog".to_owned()
                    })?;
                let StackAbilityCondition::Supported(condition) =
                    stack_ability_condition(kind, &definition_snapshot)
                else {
                    return Err(
                        "stack ability locator does not match the observed ability kind".into(),
                    );
                };
                let target_definition = payload_state
                    .target_definition_locator
                    .as_ref()
                    .and_then(|locator| catalog_ability(&game.catalog, locator))
                    .ok_or_else(|| {
                        "stack target-definition locator is absent from this catalog".to_owned()
                    })?;
                let targets = payload_state
                    .target_selections
                    .iter()
                    .map(parse_target_selection)
                    .collect::<Result<Vec<_>, _>>()?;
                let context = parse_effect_resolution_context(payload_state.context.clone())?;
                let presentation = object_characteristics_from_snapshot(
                    &game.catalog,
                    &payload_state.presentation,
                )
                .ok_or("stack ability presentation locator is absent from this catalog")?;
                let ability = StackAbilityPayload {
                    origin,
                    definition: (kind == StackObjectKind::ActivatedAbility)
                        .then(|| Box::new(definition_snapshot)),
                    presentation,
                    text: Some(definition_snapshot.text),
                    target_defs: ability_target_defs(&target_definition).to_vec(),
                    targets,
                    context,
                    resolver: Game::ability_resolver(origin, &definition_snapshot),
                    condition,
                    mode_effects: payload_state
                        .mode_effects
                        .iter()
                        .map(|effect| {
                            catalog_scoped_effect(
                                &game.catalog,
                                payload_state
                                    .ability_locator
                                    .as_ref()
                                    .expect("the locator was validated above"),
                                effect,
                            )
                            .ok_or_else(|| {
                                "stack mode effect locator is absent from this catalog".to_owned()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    resolution_destination: None,
                    x: payload_state.x,
                    sacrificed_mana_value: payload_state.sacrificed_mana_value,
                };
                if usize_field(shown, "x")? != usize::from(payload_state.x) {
                    return Err("checkpoint stack ability X does not match observation".into());
                }
                let card = ability_object(id, owner, presentation);
                (source, Some(ability), None, card)
            }
        };
        stack.push(StackObject {
            id,
            kind,
            card,
            source,
            ability,
            controller,
            signature,
            chosen_permanents: parse_ids(field(shown, "chosenPermanents")?)?,
            applied_effects: parse_applied_stack_effects(&state.applied_effects, game)?,
            text_changes: parse_text_changes(&state.text_changes),
            colors: state.colors.map(color_set_from_flags),
            colors_of_mana_spent: color_set_from_flags(state.colors_of_mana_spent),
            phyrexian_symbols_paid_with_life: state.phyrexian_symbols_paid_with_life,
            cast_via_flashback: state.cast_via_flashback,
            cast_via_suspend: state.cast_via_suspend,
            cast_at_instant_speed: state.cast_at_instant_speed,
            cast_from_zone: state
                .cast_from_zone
                .as_deref()
                .and_then(cast_source_zone_from_label),
            face_down: state.face_down.map(face_down_characteristics_from_snapshot),
            is_copy: state.is_copy,
        });
    }
    Ok(stack)
}

pub(super) fn parse_detached_stack(
    state: &DetachedStackSnapshot,
    game: &Game,
) -> Result<StackObject, String> {
    if state.has_runtime_overrides {
        return Err("detached stack object has runtime overrides not yet represented".into());
    }
    let id = GameObjectId(state.object_id);
    let object_kind = object_kind_from_snapshot(state.object_kind, &game.catalog)?;
    let owner = seat_index_value(state.owner)?;
    let controller = seat_index_value(state.controller)?;
    let replacement_effect = state.kind == StackObjectKindSnapshot::ReplacementEffect;
    let kind = match state.kind {
        StackObjectKindSnapshot::Spell => StackObjectKind::Spell,
        StackObjectKindSnapshot::ActivatedAbility => StackObjectKind::ActivatedAbility,
        StackObjectKindSnapshot::TriggeredAbility | StackObjectKindSnapshot::ReplacementEffect => {
            StackObjectKind::TriggeredAbility
        }
    };
    let signature = state
        .signature
        .as_ref()
        .map(parse_signature_snapshot)
        .transpose()?;
    let ability = match kind {
        StackObjectKind::Spell => {
            let ObjectKind::Card(definition) = object_kind else {
                return Err("a detached spell must be backed by a card".into());
            };
            signature
                .as_ref()
                .and_then(|signature| game.frozen_spell_payload(definition, signature))
        }
        StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
            if replacement_effect {
                if !matches!(object_kind, ObjectKind::Card(_)) {
                    return Err("a detached replacement effect must be backed by a card".into());
                }
            } else if object_kind != ObjectKind::Ability {
                return Err("a detached stack ability must have ability object kind".into());
            }
            let payload = state
                .ability_payload
                .as_ref()
                .ok_or("detached stack ability is missing its frozen payload")?;
            Some(parse_ability_payload(
                kind,
                replacement_effect,
                payload,
                game,
            )?)
        }
    };
    let stack_card = match (object_kind, ability.as_ref()) {
        (ObjectKind::Card(definition), _) => card(id, definition, owner, &game.catalog)?.into(),
        (ObjectKind::Ability, Some(payload)) => ability_object(id, owner, payload.presentation),
        (ObjectKind::Ability, None) => {
            return Err("a detached ability object lacks its frozen payload".into());
        }
        (ObjectKind::Token, _) => return Err("a token cannot be a detached stack object".into()),
        (ObjectKind::Emblem, _) => {
            return Err("an emblem cannot be a detached stack object".into());
        }
    };
    Ok(StackObject {
        id,
        kind,
        card: stack_card,
        source: state.source.map(GameObjectId),
        ability,
        controller,
        signature,
        chosen_permanents: state
            .chosen_permanents
            .iter()
            .copied()
            .map(GameObjectId)
            .collect(),
        applied_effects: parse_applied_stack_effects(&state.applied_effects, game)?,
        text_changes: parse_text_changes(&state.text_changes),
        colors: state.colors.map(color_set_from_flags),
        colors_of_mana_spent: color_set_from_flags(state.colors_of_mana_spent),
        phyrexian_symbols_paid_with_life: state.phyrexian_symbols_paid_with_life,
        cast_via_flashback: state.cast_via_flashback,
        cast_via_suspend: state.cast_via_suspend,
        cast_at_instant_speed: state.cast_at_instant_speed,
        cast_from_zone: state
            .cast_from_zone
            .as_deref()
            .and_then(cast_source_zone_from_label),
        face_down: state.face_down.map(face_down_characteristics_from_snapshot),
        is_copy: state.is_copy,
    })
}

fn ability_object(
    id: GameObjectId,
    owner: PlayerId,
    presentation: ObjectCharacteristics,
) -> ObjectInstance {
    let characteristics = match presentation {
        ObjectCharacteristics::Card { definition, .. } => CharacteristicSource::Ability(definition),
        ObjectCharacteristics::Token { token, .. } => CharacteristicSource::Token(token),
        ObjectCharacteristics::Emblem { emblem } => CharacteristicSource::Emblem(emblem),
        ObjectCharacteristics::FaceDown { face_down } => CharacteristicSource::FaceDown(face_down),
    };
    ObjectInstance {
        id,
        definition: ObjectKind::Ability,
        owner,
        backing: ObjectBacking::None,
        characteristics,
        counters: crate::game::counters::Counters::new(),
    }
}

fn parse_text_changes(
    changes: &[super::model::BasicLandTypeChangeSnapshot],
) -> Vec<BasicLandTypeChange> {
    changes
        .iter()
        .map(|change| BasicLandTypeChange {
            from: parse_basic_land_type(change.from),
            to: parse_basic_land_type(change.to),
        })
        .collect()
}

pub(super) fn color_set_from_flags(flags: [bool; 5]) -> ColorSet {
    let colors = [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ];
    colors
        .into_iter()
        .zip(flags)
        .filter(|(_, present)| *present)
        .fold(ColorSet::empty(), |set, (color, _)| set.with(color))
}

fn parse_ability_payload(
    kind: StackObjectKind,
    replacement_effect: bool,
    state: &StackAbilitySnapshot,
    game: &Game,
) -> Result<StackAbilityPayload, String> {
    let locator = state
        .ability_locator
        .as_ref()
        .ok_or("stack ability lacks a catalog locator")?;
    let definition = catalog_ability(&game.catalog, locator)
        .ok_or_else(|| "stack ability locator is absent from this catalog".to_owned())?;
    let condition = if replacement_effect {
        if !matches!(definition.definition, DeclarativeAbilityDef::Replacement(_)) {
            return Err("replacement-effect locator does not name a replacement ability".into());
        }
        None
    } else {
        let StackAbilityCondition::Supported(condition) =
            stack_ability_condition(kind, &definition)
        else {
            return Err("stack ability locator does not match its ability kind".into());
        };
        condition
    };
    let target_definition = state
        .target_definition_locator
        .as_ref()
        .and_then(|locator| catalog_ability(&game.catalog, locator))
        .ok_or("stack target-definition locator is absent from this catalog")?;
    let origin = ability_origin_from_snapshot(state.origin);
    if !super::semantics::ability_locator_matches_origin(locator, origin) {
        return Err("detached stack ability locator disagrees with its origin".into());
    }
    let presentation = object_characteristics_from_snapshot(&game.catalog, &state.presentation)
        .ok_or("detached stack presentation locator is absent from this catalog")?;
    Ok(StackAbilityPayload {
        origin,
        definition: (kind == StackObjectKind::ActivatedAbility).then(|| Box::new(definition)),
        presentation,
        text: Some(definition.text),
        target_defs: ability_target_defs(&target_definition).to_vec(),
        targets: state
            .target_selections
            .iter()
            .map(parse_target_selection)
            .collect::<Result<Vec<_>, _>>()?,
        context: parse_effect_resolution_context(state.context.clone())?,
        resolver: Game::ability_resolver(origin, &definition),
        condition,
        mode_effects: state
            .mode_effects
            .iter()
            .map(|effect| {
                catalog_scoped_effect(&game.catalog, locator, effect)
                    .ok_or_else(|| "detached stack mode effect locator is absent".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?,
        resolution_destination: None,
        x: state.x,
        sacrificed_mana_value: state.sacrificed_mana_value,
    })
}

fn parse_signature_snapshot(state: &CastSignatureSnapshot) -> Result<CastSignature, String> {
    let form = match &state.form {
        SpellFormSnapshot::Part { part_id } => SpellForm::Part(CardPartId(*part_id)),
        SpellFormSnapshot::Combined { part_ids } => {
            SpellForm::Combined(part_ids.iter().copied().map(CardPartId).collect())
        }
    };
    let choices = CastChoices::new(PlayOptionId(state.play_option))
        .with_modes(state.modes.iter().copied().map(ModeId).collect())
        .with_costs(CostConfiguration::new(
            state.alternative_cost.map(AlternativeCostId),
            state
                .additional_costs
                .iter()
                .copied()
                .map(AdditionalCostId)
                .collect(),
        ))
        .with_x(state.x)
        .with_targets(
            state
                .targets
                .iter()
                .map(parse_target_selection)
                .collect::<Result<Vec<_>, _>>()?,
        )
        .with_spliced(state.spliced.iter().copied().map(GameObjectId).collect());
    Ok(CastSignature::from_validated_choices(form, choices))
}

pub(super) fn parse_trigger_context(
    value: TriggerContextSnapshot,
) -> Result<TriggerContext, String> {
    Ok(TriggerContext {
        object: value.object.map(GameObjectId),
        zone_change_result: value.zone_change_result.map(GameObjectId),
        object_controller: value.object_controller.map(seat_index_value).transpose()?,
        event_player: value.event_player.map(seat_index_value).transpose()?,
        amount: value.amount,
        damaged_object: value.damaged_object.map(GameObjectId),
        cast_from_zone: value.cast_from_zone.map(parse_zone_kind),
    })
}

pub(super) fn parse_effect_resolution_context(
    value: EffectResolutionContextSnapshot,
) -> Result<EffectResolutionContext, String> {
    let mut context = EffectResolutionContext::from_bindings(
        parse_trigger_context(value.trigger)?,
        value.single_objects.map(|object| object.map(parse_target)),
        value
            .object_groups
            .map(|objects| objects.into_iter().map(parse_target).collect()),
    );
    context.restore_named_bindings(
        value
            .named_object_groups
            .into_iter()
            .map(|(label, objects)| (label, objects.into_iter().map(parse_target).collect()))
            .collect(),
    );
    context.chosen_counter = value.chosen_counter.map(|kind| kind.0);
    Ok(context)
}

pub(super) fn parse_target_selection(
    value: &TargetSelectionSnapshot,
) -> Result<TargetSelection, String> {
    let slot = crate::TargetSlotId(value.slot_id);
    let targets = value.targets.iter().copied().map(parse_target).collect();
    if value.amounts.is_empty() {
        Ok(TargetSelection::new(slot, targets))
    } else if value.amounts.len() == targets.len() {
        Ok(TargetSelection::divided(
            slot,
            targets,
            value.amounts.clone(),
        ))
    } else {
        Err("divided target amounts do not match targets".into())
    }
}

pub(super) fn parse_target(value: TargetSnapshot) -> Target {
    match value {
        TargetSnapshot::Player {
            seat: SeatSnapshot::One,
        } => Target::Player(PlayerId::One),
        TargetSnapshot::Player {
            seat: SeatSnapshot::Two,
        } => Target::Player(PlayerId::Two),
        TargetSnapshot::Card { object_id } => Target::Card(GameObjectId(object_id)),
        TargetSnapshot::Permanent { object_id } => Target::Permanent(GameObjectId(object_id)),
        TargetSnapshot::Spell { object_id } => Target::Spell(GameObjectId(object_id)),
    }
}

fn player_from_index(index: usize) -> Option<PlayerId> {
    [PlayerId::One, PlayerId::Two].get(index).copied()
}

fn seat_index_value(index: usize) -> Result<PlayerId, String> {
    player_from_index(index).ok_or_else(|| "seat index must be 0 or 1".into())
}

pub(super) fn parse_ability_origin(value: &Value) -> Result<AbilityOrigin, String> {
    match str_field(value, "kind")? {
        "printed" => Ok(AbilityOrigin::Printed {
            definition: card_definition_id_field(value, "definition")?,
            part: CardPartId(u8_field(value, "partId")?),
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "token" => Ok(AbilityOrigin::Token {
            part: CardPartId(u8_field(value, "partId")?),
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "emblem" => Ok(AbilityOrigin::Emblem {
            ability: AbilityId(u8_field(value, "abilityId")?),
        }),
        "intrinsicBasicLand" => Ok(AbilityOrigin::IntrinsicBasicLand(
            match str_field(value, "landType")? {
                "plains" => BasicLandType::Plains,
                "island" => BasicLandType::Island,
                "swamp" => BasicLandType::Swamp,
                "mountain" => BasicLandType::Mountain,
                "forest" => BasicLandType::Forest,
                other => return Err(format!("unknown intrinsic basic land type {other}")),
            },
        )),
        "intrinsicCounter" => {
            let counter = str_field(value, "counter")?;
            let kind = super::CounterKind::from_name(counter)
                .ok_or_else(|| format!("unknown intrinsic counter kind {counter}"))?;
            Ok(AbilityOrigin::IntrinsicCounter(kind))
        }
        "granted" => Ok(AbilityOrigin::Granted {
            source: GameObjectId(u32_field(value, "source")?),
            source_definition: card_definition_id_field(value, "sourceDefinition")?,
            source_part: CardPartId(u8_field(value, "sourcePartId")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        "tokenGranted" => Ok(AbilityOrigin::TokenGranted {
            source: GameObjectId(u32_field(value, "source")?),
            source_part: CardPartId(u8_field(value, "sourcePartId")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        "emblemGranted" => Ok(AbilityOrigin::EmblemGranted {
            source: GameObjectId(u32_field(value, "source")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        other => Err(format!("unknown ability origin kind {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ObjectBindingIndex, ObjectSetBindingIndex};

    #[test]
    fn effect_resolution_context_round_trips_typed_objects_and_groups() {
        let trigger = TriggerContext {
            object: Some(GameObjectId(10)),
            zone_change_result: Some(GameObjectId(12)),
            object_controller: Some(PlayerId::One),
            event_player: Some(PlayerId::Two),
            amount: Some(3),
            damaged_object: None,
            cast_from_zone: Some(crate::card::ZoneKind::Graveyard),
        };
        let mut context = EffectResolutionContext::new(trigger);
        context.bind_single_object(
            ObjectBindingIndex::PRIMARY,
            Some(Target::Spell(GameObjectId(11))),
        );
        context.bind_object_group(
            ObjectSetBindingIndex::PRIMARY,
            vec![
                Target::Permanent(GameObjectId(12)),
                Target::Card(GameObjectId(13)),
                Target::Player(PlayerId::Two),
            ],
        );
        context.declare_named_object_group("optional_card");
        context.bind_named_object_group("revealed_card", vec![Target::Card(GameObjectId(14))]);
        context.declare_named_object_group("empty_cards");
        context.bind_named_object_group(
            "milled_cards",
            vec![
                Target::Card(GameObjectId(15)),
                Target::Card(GameObjectId(16)),
            ],
        );

        let snapshot = effect_resolution_context_snapshot(&context);
        let rebuilt = parse_effect_resolution_context(snapshot).expect("context should parse");

        assert_eq!(rebuilt, context);
        assert_eq!(
            rebuilt.single_object(ObjectBindingIndex::PRIMARY),
            Some(Target::Spell(GameObjectId(11)))
        );
        assert_eq!(
            rebuilt.object_group(ObjectSetBindingIndex::PRIMARY),
            [
                Target::Permanent(GameObjectId(12)),
                Target::Card(GameObjectId(13)),
                Target::Player(PlayerId::Two),
            ]
        );
        assert!(rebuilt.named_object_groups().contains_key("optional_card"));
        assert!(rebuilt.named_object_group("optional_card").is_empty());
        assert_eq!(
            rebuilt.named_object_group("revealed_card"),
            [Target::Card(GameObjectId(14))]
        );
        assert!(rebuilt.named_object_groups().contains_key("empty_cards"));
        assert!(rebuilt.named_object_group("empty_cards").is_empty());
        assert_eq!(
            rebuilt.named_object_group("milled_cards"),
            [
                Target::Card(GameObjectId(15)),
                Target::Card(GameObjectId(16))
            ]
        );
        let mut referenced = resolution_context_referenced_object_ids(&rebuilt);
        referenced.sort_unstable();
        assert_eq!(
            referenced,
            [
                GameObjectId(10),
                GameObjectId(11),
                GameObjectId(12),
                GameObjectId(13),
                GameObjectId(14),
                GameObjectId(15),
                GameObjectId(16),
            ]
        );
    }
}
