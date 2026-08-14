use super::model::{
    AppliedStackEffectSnapshot, CastSignatureSnapshot, DetachedStackSnapshot, ManaSourceSnapshot,
    SeatSnapshot, SpellFormSnapshot, StackAbilitySnapshot, StackObjectKindSnapshot, StackSnapshot,
    TargetSelectionSnapshot, TargetSnapshot, TriggerContextSnapshot,
};
use super::semantics::{
    applied_effect_locator, catalog_applied_effect, catalog_scoped_effect, scoped_effect_snapshot,
};
use super::{
    AbilityId, AbilityOrigin, AdditionalCostId, AlternativeCostId, AppliedStackEffect,
    BasicLandType, BasicLandTypeChange, CardDefinitionId, CardPartId, CastChoices, CastSignature,
    CharacteristicSource, CostConfiguration, DeclarativeAbilityDef, Game, GameObjectId, GameStack,
    GrantId, ManaSource, ModeId, PlayOptionId, PlayerId, SpellForm, StackAbilityPayload,
    StackObject, StackObjectKind, Target, TargetSelection, TriggerContext, Value, ability_locator,
    ability_origin_from_snapshot, ability_origin_snapshot, array, card, catalog_ability, field,
    optional_id, parse_basic_land_type, parse_cast_signature, parse_ids, seat_value, str_field,
    u8_field, u32_field, usize_field,
};
use crate::card::{ColorSet, ManaColor};

pub(super) fn stack_ability_snapshot(
    game: &Game,
    object: &StackObject,
) -> Option<StackAbilitySnapshot> {
    let payload = object.ability.as_ref()?;
    let locator = ability_locator(&game.catalog, |candidate| {
        stack_payload_matches(payload, candidate)
    })?;
    let ability = catalog_ability(&game.catalog, &locator)?;
    Some(StackAbilitySnapshot {
        ability_locator: Some(locator),
        origin: ability_origin_snapshot(payload.origin),
        target_selections: payload
            .targets
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        context: trigger_context_snapshot(payload.context),
        mode_effects: payload
            .mode_effects
            .iter()
            .copied()
            .map(|effect| scoped_effect_snapshot(&ability, effect))
            .collect::<Option<Vec<_>>>()?,
        x: payload.x,
    })
}

pub(super) fn detached_stack_snapshot(
    game: &Game,
    object: &StackObject,
) -> Option<DetachedStackSnapshot> {
    let ability_payload = if object.kind != StackObjectKind::Spell && object.ability.is_some() {
        Some(stack_ability_snapshot(game, object)?)
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
    Some(DetachedStackSnapshot {
        object_id: object.id.0,
        kind: kind_snapshot(object.kind),
        definition: object.card.definition.0,
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
        cast_via_flashback: object.cast_via_flashback,
        schedule_on_entry: object.schedule_on_entry,
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
        ids.extend(payload.context.object);
        ids.extend(payload.context.source_attachment);
        ids.extend(payload.context.source_linked);
        ids.extend(payload.context.chosen_objects.iter().flatten().copied());
        ids.extend(payload.targets.iter().flat_map(|selection| {
            selection
                .targets()
                .iter()
                .filter_map(|target| match target {
                    Target::Player(_) => None,
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
                })
        }));
    }
    if let Some(signature) = &object.signature {
        ids.extend(signature.targets().iter().flat_map(|selection| {
            selection
                .targets()
                .iter()
                .filter_map(|target| match target {
                    Target::Player(_) => None,
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(*id),
                })
        }));
    }
    ids.into_iter()
}

fn stack_payload_matches(
    payload: &StackAbilityPayload,
    candidate: &crate::card::AbilityDef,
) -> bool {
    if let Some(definition) = payload.definition.as_deref() {
        return definition == candidate;
    }
    let DeclarativeAbilityDef::Triggered(triggered) = candidate.definition else {
        return false;
    };
    payload.text == Some(candidate.text)
        && payload.target_defs == triggered.targets
        && payload.condition == triggered.condition
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
        source_attachment: context.source_attachment.map(|id| id.0),
        source_linked: context.source_linked.map(|id| id.0),
        chosen_objects: context.chosen_objects.map(|object| object.map(|id| id.0)),
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
        let definition = CardDefinitionId(
            u16::try_from(usize_field(shown, "definition")?).map_err(|_| "definition too large")?,
        );
        let owner = seat_index_value(state.owner)?;
        let controller = seat_value(field(shown, "controller")?)?;
        let kind = match str_field(shown, "kind")? {
            "Spell" => StackObjectKind::Spell,
            "ActivatedAbility" => StackObjectKind::ActivatedAbility,
            "TriggeredAbility" => StackObjectKind::TriggeredAbility,
            other => return Err(format!("unknown stack object kind {other}")),
        };
        let (source, ability, signature, card) = match kind {
            StackObjectKind::Spell => {
                let signature = parse_cast_signature(field(shown, "signature")?)?;
                let card = card(id, definition, owner, &game.catalog)?;
                (
                    None,
                    game.frozen_spell_payload(definition, &signature),
                    Some(signature),
                    card,
                )
            }
            StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
                let payload_state = state
                    .ability_payload
                    .as_ref()
                    .ok_or("stack ability is missing its frozen payload")?;
                let observed_origin = parse_ability_origin(field(shown, "ability")?)?;
                let origin = ability_origin_from_snapshot(payload_state.origin);
                if origin != observed_origin {
                    return Err("checkpoint stack ability origin does not match observation".into());
                }
                let source = optional_id(shown.get("sourceObjectId"));
                let definition_snapshot = payload_state
                    .ability_locator
                    .as_ref()
                    .and_then(|locator| catalog_ability(&game.catalog, locator))
                    .ok_or_else(|| {
                        "stack ability locator is absent from this catalog".to_owned()
                    })?;
                let (target_defs, condition) = match (kind, definition_snapshot.definition) {
                    (
                        StackObjectKind::ActivatedAbility,
                        DeclarativeAbilityDef::Activated(activated),
                    ) => (activated.targets, None),
                    (
                        StackObjectKind::TriggeredAbility,
                        DeclarativeAbilityDef::Triggered(triggered),
                    ) => (triggered.targets, triggered.condition),
                    _ => {
                        return Err(
                            "stack ability locator does not match the observed ability kind".into(),
                        );
                    }
                };
                let targets = payload_state
                    .target_selections
                    .iter()
                    .map(parse_target_selection)
                    .collect::<Result<Vec<_>, _>>()?;
                let context = parse_trigger_context(payload_state.context)?;
                let ability = StackAbilityPayload {
                    origin,
                    definition: (kind == StackObjectKind::ActivatedAbility)
                        .then(|| Box::new(definition_snapshot)),
                    presentation_definition: definition,
                    text: Some(definition_snapshot.text),
                    target_defs: target_defs.to_vec(),
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
                let mut card = card(id, definition, owner, &game.catalog)?;
                card.characteristics = CharacteristicSource::Ability(definition);
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
            cast_via_flashback: state.cast_via_flashback,
            schedule_on_entry: state.schedule_on_entry,
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
    let definition = CardDefinitionId(state.definition);
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
        StackObjectKind::Spell => signature
            .as_ref()
            .and_then(|signature| game.frozen_spell_payload(definition, signature)),
        StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
            let payload = state
                .ability_payload
                .as_ref()
                .ok_or("detached stack ability is missing its frozen payload")?;
            Some(parse_ability_payload(kind, definition, payload, game)?)
        }
    };
    let mut stack_card = card(id, definition, owner, &game.catalog)?;
    if kind != StackObjectKind::Spell {
        stack_card.characteristics = CharacteristicSource::Ability(definition);
    }
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
        cast_via_flashback: state.cast_via_flashback,
        schedule_on_entry: state.schedule_on_entry,
        is_copy: state.is_copy,
    })
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

fn color_set_from_flags(flags: [bool; 5]) -> ColorSet {
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
    presentation_definition: CardDefinitionId,
    state: &StackAbilitySnapshot,
    game: &Game,
) -> Result<StackAbilityPayload, String> {
    let locator = state
        .ability_locator
        .as_ref()
        .ok_or("stack ability lacks a catalog locator")?;
    let definition = catalog_ability(&game.catalog, locator)
        .ok_or_else(|| "stack ability locator is absent from this catalog".to_owned())?;
    let (target_defs, condition) = match (kind, definition.definition) {
        (StackObjectKind::ActivatedAbility, DeclarativeAbilityDef::Activated(activated)) => {
            (activated.targets, None)
        }
        (StackObjectKind::TriggeredAbility, DeclarativeAbilityDef::Triggered(triggered)) => {
            (triggered.targets, triggered.condition)
        }
        _ => return Err("stack ability locator does not match its ability kind".into()),
    };
    let origin = ability_origin_from_snapshot(state.origin);
    Ok(StackAbilityPayload {
        origin,
        definition: (kind == StackObjectKind::ActivatedAbility).then(|| Box::new(definition)),
        presentation_definition,
        text: Some(definition.text),
        target_defs: target_defs.to_vec(),
        targets: state
            .target_selections
            .iter()
            .map(parse_target_selection)
            .collect::<Result<Vec<_>, _>>()?,
        context: parse_trigger_context(state.context)?,
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
        source_attachment: value.source_attachment.map(GameObjectId),
        source_linked: value.source_linked.map(GameObjectId),
        chosen_objects: value.chosen_objects.map(|object| object.map(GameObjectId)),
    })
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
            definition: CardDefinitionId(
                u16::try_from(usize_field(value, "definition")?)
                    .map_err(|_| "ability definition is too large")?,
            ),
            part: CardPartId(u8_field(value, "partId")?),
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
        "granted" => Ok(AbilityOrigin::Granted {
            source: GameObjectId(u32_field(value, "source")?),
            source_definition: CardDefinitionId(
                u16::try_from(usize_field(value, "sourceDefinition")?)
                    .map_err(|_| "grant source definition is too large")?,
            ),
            source_part: CardPartId(u8_field(value, "sourcePartId")?),
            source_ability: AbilityId(u8_field(value, "sourceAbilityId")?),
            grant: GrantId(u8_field(value, "grantId")?),
        }),
        other => Err(format!("unknown ability origin kind {other}")),
    }
}
