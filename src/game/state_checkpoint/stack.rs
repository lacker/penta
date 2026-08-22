use super::model::{
    AppliedStackEffectSnapshot, BasicLandTypeChangeSnapshot, CastSignatureSnapshot,
    DetachedStackSnapshot, EffectResolutionContextSnapshot, ManaSourceSnapshot, SeatSnapshot,
    SpellFormSnapshot, StackAbilitySnapshot, StackObjectKindSnapshot, StackSnapshot,
    TargetSelectionSnapshot, TargetSnapshot, TriggerContextSnapshot,
};
use super::semantics::{
    ability_locator_for_origin, applied_effect_locator, catalog_applied_effect,
    catalog_scoped_effect, scoped_effect_snapshot,
};
use super::{
    AbilityId, AbilityOrigin, AdditionalCostId, AlternativeCostId, AppliedStackEffect,
    BasicLandType, BasicLandTypeChange, CardPartId, CastChoices, CastSignature,
    CharacteristicSource, CostConfiguration, DeclarativeAbilityDef, EffectResolutionContext, Game,
    GameObjectId, GameStack, GrantId, ManaSource, ModeId, ObjectBacking, ObjectCharacteristics,
    ObjectInstance, ObjectKind, PlayOptionId, PlayerId, RetiredObject, SpellForm,
    StackAbilityPayload, StackObject, StackObjectKind, Target, TargetSelection, TriggerContext,
    Value, ability_locator, ability_origin_from_snapshot, ability_origin_snapshot,
    ability_target_defs, array, basic_land_type_snapshot, card, card_definition_id_field,
    cast_source_zone_from_label, catalog_ability, face_down_characteristics_from_snapshot,
    face_down_characteristics_snapshot, field, object_characteristics_from_snapshot,
    object_characteristics_snapshot, object_kind_from_snapshot, object_kind_snapshot, optional_id,
    parse_basic_land_type, parse_cast_signature, parse_ids, seat_value, str_field, u8_field,
    u32_field, usize_field,
};
use crate::card::{ColorSet, ManaColor};

mod ability_kind;
use ability_kind::{StackAbilityCondition, stack_ability_condition};
mod current;
pub(super) use current::current_stack_snapshot;

pub(super) fn stack_ability_snapshot(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
) -> Option<StackAbilitySnapshot> {
    stack_ability_snapshot_allowing(game, viewer, object, &[])
}

pub(super) fn stack_ability_snapshot_allowing(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
    visible_rebindings: &[GameObjectId],
) -> Option<StackAbilitySnapshot> {
    let payload = object.ability.as_ref()?;
    if object.source.is_some_and(|source| {
        object_reference_requires_hidden_rebinding(game, viewer, source)
            && !visible_rebindings.contains(&source)
    }) {
        return None;
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
        context: effect_resolution_context_snapshot(&payload.context),
        mode_effects: payload
            .mode_effects
            .iter()
            .copied()
            .map(|effect| scoped_effect_snapshot(&ability, effect))
            .collect::<Option<Vec<_>>>()?,
        x: payload.x,
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
        cast_via_flashback: object.cast_via_flashback,
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
    }
}

pub(super) fn stack_object_requires_retired(game: &Game, object: &StackObject) -> bool {
    referenced_object_ids(object).any(|id| game.retired_objects.contains_key(&id))
}

pub(super) fn stack_object_has_unrebindable_hidden_reference(
    game: &Game,
    viewer: PlayerId,
    object: &StackObject,
) -> bool {
    object
        .source
        .is_some_and(|source| object_reference_requires_hidden_rebinding(game, viewer, source))
        || object.ability.as_ref().is_some_and(|payload| {
            stack_payload_has_unrebindable_hidden_reference_except(game, viewer, payload, &[])
        })
}

pub(super) fn referenced_object_ids(object: &StackObject) -> impl Iterator<Item = GameObjectId> {
    let mut ids = Vec::new();
    ids.extend(object.source);
    ids.extend(object.chosen_permanents.iter().copied());
    ids.extend(
        object
            .applied_effects
            .iter()
            .filter_map(|effect| effect.source.map(|source| source.object)),
    );
    if let Some(payload) = &object.ability {
        ids.extend(resolution_context_referenced_object_ids(&payload.context));
        ids.extend(lexical_target_referenced_object_ids(payload));
    }
    ids.into_iter()
}

/// Target selections with a declared slot are ordinary targets: once their
/// object changes zones, the id is deliberately left dangling so legality
/// makes the spell or ability fizzle. Extra selections without a declared
/// slot are captured lexical state (for example a delayed follow-up referring
/// to an earlier target), so they still require hidden rebinding and LKI.
fn lexical_target_referenced_object_ids(payload: &StackAbilityPayload) -> Vec<GameObjectId> {
    payload
        .targets
        .iter()
        .filter(|selection| payload.target_defs.get(selection.slot().index()).is_none())
        .flat_map(TargetSelection::targets)
        .filter_map(|target| match target {
            Target::Player(_) => None,
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
        })
        .collect()
}

fn stack_payload_has_unrebindable_hidden_reference_except(
    game: &Game,
    viewer: PlayerId,
    payload: &StackAbilityPayload,
    visible_rebindings: &[GameObjectId],
) -> bool {
    lexical_target_referenced_object_ids(payload)
        .into_iter()
        .chain(resolution_context_referenced_object_ids(&payload.context))
        .any(|object| {
            object_reference_requires_hidden_rebinding(game, viewer, object)
                && !visible_rebindings.contains(&object)
        })
}

pub(super) fn target_selections_referenced_object_ids(
    selections: &[TargetSelection],
) -> Vec<GameObjectId> {
    selections
        .iter()
        .flat_map(TargetSelection::targets)
        .filter_map(|target| match target {
            Target::Player(_) => None,
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
        })
        .collect()
}

/// Captured trigger state has no hidden-zone rebinding table of its own.
/// References to the viewer's hand keep their public observation ids, but
/// libraries, outside-game cards, and the opposing hand are reconstructed
/// from a hypothesis with freshly minted ids. Serializing one of those host
/// ids would both disclose hidden identity and leave a dangling reference in
/// the reconstructed game, so the containing checkpoint must fail closed.
pub(super) fn trigger_capture_has_unrebindable_hidden_reference(
    game: &Game,
    viewer: PlayerId,
    targets: &[TargetSelection],
    context: &EffectResolutionContext,
) -> bool {
    trigger_capture_has_unrebindable_hidden_reference_except(game, viewer, targets, context, &[])
}

pub(super) fn trigger_capture_has_unrebindable_hidden_reference_except(
    game: &Game,
    viewer: PlayerId,
    targets: &[TargetSelection],
    context: &EffectResolutionContext,
    visible_rebindings: &[GameObjectId],
) -> bool {
    target_selections_referenced_object_ids(targets)
        .into_iter()
        .chain(resolution_context_referenced_object_ids(context))
        .any(|object| {
            object_reference_requires_hidden_rebinding(game, viewer, object)
                && !visible_rebindings.contains(&object)
        })
}

pub(super) fn object_reference_requires_hidden_rebinding(
    game: &Game,
    viewer: PlayerId,
    object: GameObjectId,
) -> bool {
    matches!(
        game.retired_objects.get(&object),
        Some(RetiredObject::Card(_))
    ) || [PlayerId::One, PlayerId::Two].into_iter().any(|player| {
        let state = &game.players[player.index()];
        state.library.iter().any(|card| card.id == object)
            || state.outside_game.iter().any(|card| card.id == object)
            || (player != viewer && state.hand.iter().any(|card| card.id == object))
    })
}

pub(super) fn resolution_context_referenced_object_ids(
    context: &EffectResolutionContext,
) -> Vec<GameObjectId> {
    let mut ids = context.trigger.object.into_iter().collect::<Vec<_>>();
    ids.extend(
        context
            .single_objects()
            .iter()
            .flatten()
            .chain(context.object_groups().iter().flatten())
            .copied()
            .filter_map(|target| match target {
                Target::Player(_) => None,
                Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
            }),
    );
    ids
}

fn stack_payload_matches(
    payload: &StackAbilityPayload,
    candidate: &crate::card::AbilityDef,
) -> bool {
    if let Some(definition) = payload.definition.as_deref() {
        return definition == candidate;
    }
    let condition = match candidate.definition {
        DeclarativeAbilityDef::Triggered(triggered) => triggered.condition,
        DeclarativeAbilityDef::AlternativeCast(alternative)
            if candidate.is_executable()
                && alternative.kind == crate::card::AlternativeCastKindDef::Miracle =>
        {
            None
        }
        _ => return false,
    };
    payload.text == Some(candidate.text)
        && payload.condition == condition
        && payload.resolver == Game::ability_resolver(payload.origin, candidate)
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
        object_controller: context.object_controller.map(PlayerId::index),
        event_player: context.event_player.map(PlayerId::index),
        amount: context.amount,
    }
}

pub(super) fn effect_resolution_context_snapshot(
    context: &EffectResolutionContext,
) -> EffectResolutionContextSnapshot {
    EffectResolutionContextSnapshot {
        trigger: trigger_context_snapshot(context.trigger),
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
                    x: payload_state.x,
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
            cast_via_flashback: state.cast_via_flashback,
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
    let kind = match state.kind {
        StackObjectKindSnapshot::Spell => StackObjectKind::Spell,
        StackObjectKindSnapshot::ActivatedAbility => StackObjectKind::ActivatedAbility,
        StackObjectKindSnapshot::TriggeredAbility => StackObjectKind::TriggeredAbility,
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
            if object_kind != ObjectKind::Ability {
                return Err("a detached stack ability must have ability object kind".into());
            }
            let payload = state
                .ability_payload
                .as_ref()
                .ok_or("detached stack ability is missing its frozen payload")?;
            Some(parse_ability_payload(kind, payload, game)?)
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
        cast_via_flashback: state.cast_via_flashback,
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
        counters: [0; super::CounterKind::COUNT],
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
    state: &StackAbilitySnapshot,
    game: &Game,
) -> Result<StackAbilityPayload, String> {
    let locator = state
        .ability_locator
        .as_ref()
        .ok_or("stack ability lacks a catalog locator")?;
    let definition = catalog_ability(&game.catalog, locator)
        .ok_or_else(|| "stack ability locator is absent from this catalog".to_owned())?;
    let StackAbilityCondition::Supported(condition) = stack_ability_condition(kind, &definition)
    else {
        return Err("stack ability locator does not match its ability kind".into());
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
        x: state.x,
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
        );
    Ok(CastSignature::from_validated_choices(form, choices))
}

pub(super) fn parse_trigger_context(
    value: TriggerContextSnapshot,
) -> Result<TriggerContext, String> {
    Ok(TriggerContext {
        object: value.object.map(GameObjectId),
        object_controller: value.object_controller.map(seat_index_value).transpose()?,
        event_player: value.event_player.map(seat_index_value).transpose()?,
        amount: value.amount,
    })
}

pub(super) fn parse_effect_resolution_context(
    value: EffectResolutionContextSnapshot,
) -> Result<EffectResolutionContext, String> {
    Ok(EffectResolutionContext::from_bindings(
        parse_trigger_context(value.trigger)?,
        value.single_objects.map(|object| object.map(parse_target)),
        value
            .object_groups
            .map(|objects| objects.into_iter().map(parse_target).collect()),
    ))
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
            let kind = super::CounterKind::ALL
                .into_iter()
                .find(|kind| kind.name() == counter)
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
            object_controller: Some(PlayerId::One),
            event_player: Some(PlayerId::Two),
            amount: Some(3),
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
        assert_eq!(
            resolution_context_referenced_object_ids(&rebuilt),
            [
                GameObjectId(10),
                GameObjectId(11),
                GameObjectId(12),
                GameObjectId(13),
            ]
        );
    }
}
