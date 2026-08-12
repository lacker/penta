use super::{
    AbilityId, AbilityOrigin, BasicLandType, CardDefinitionId, CardPartId, CharacteristicSource,
    DeclarativeAbilityDef, Game, GameObjectId, GameStack, GrantId, PlayerId, StackAbilityPayload,
    StackObject, StackObjectKind, Target, TargetSelection, TriggerContext, Value,
    ability_locator_json, array, bool_field, card, catalog_ability, field, json, optional_id,
    parse_cast_signature, parse_ids, parse_target_selection, seat_index, seat_label, seat_value,
    str_field, u8_field, u32_field, usize_field,
};

pub(super) fn stack_ability_checkpoint_json(game: &Game, object: &StackObject) -> Value {
    let Some(payload) = object.ability.as_ref() else {
        return Value::Null;
    };
    let locator = ability_locator_json(&game.catalog, |candidate| {
        stack_payload_matches(payload, candidate)
    });
    json!({
        "abilityLocator": locator,
        "targetSelections": payload.targets.iter().map(target_selection_checkpoint_json).collect::<Vec<_>>(),
        "context": trigger_context_json(payload.context),
    })
}

pub(super) fn stack_object_requires_retired(game: &Game, object: &StackObject) -> bool {
    object
        .source
        .into_iter()
        .chain(
            object
                .ability
                .as_ref()
                .and_then(|payload| payload.context.object),
        )
        .chain(object.iter_targets().copied().filter_map(target_object_id))
        .chain(object.chosen_permanents.iter().copied())
        .any(|id| game.retired_objects.contains_key(&id))
}

fn target_object_id(target: Target) -> Option<GameObjectId> {
    match target {
        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
        Target::Player(_) => None,
    }
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

fn target_selection_checkpoint_json(selection: &TargetSelection) -> Value {
    json!({
        "slotId": selection.slot().0,
        "targets": selection.targets().iter().copied().map(target_checkpoint_json).collect::<Vec<_>>(),
        "amounts": selection.amounts(),
    })
}

fn target_checkpoint_json(target: Target) -> Value {
    match target {
        Target::Player(player) => json!({"type": "player", "seat": seat_label(player)}),
        Target::Card(id) => json!({"type": "card", "objectId": id.0}),
        Target::Permanent(id) => json!({"type": "permanent", "objectId": id.0}),
        Target::Spell(id) => json!({"type": "spell", "objectId": id.0}),
    }
}

fn trigger_context_json(context: TriggerContext) -> Value {
    json!({
        "object": context.object.map(|id| id.0),
        "objectController": context.object_controller.map(PlayerId::index),
        "eventPlayer": context.event_player.map(PlayerId::index),
        "amount": context.amount,
    })
}

#[allow(clippy::too_many_lines)]
pub(super) fn parse_stack(
    observation: &Value,
    checkpoint: &Value,
    game: &Game,
) -> Result<GameStack, String> {
    let visible = array(field(observation, "stack")?)?;
    let raw = array(field(checkpoint, "stack")?)?;
    if visible.len() != raw.len() {
        return Err("checkpoint stack does not match observation".into());
    }
    let mut stack = GameStack::default();
    for (shown, state) in visible.iter().zip(raw) {
        if bool_field(state, "hasRuntimeOverrides")? {
            return Err(
                "stack object has runtime overrides not yet represented by semantic locators"
                    .into(),
            );
        }
        if bool_field(state, "requiresRetiredObject")? {
            return Err(
                "stack object requires retired-object last-known information not yet represented by the checkpoint"
                    .into(),
            );
        }
        let id = GameObjectId(u32_field(shown, "objectId")?);
        if id.0 != u32_field(state, "objectId")? {
            return Err("checkpoint stack id does not match observation".into());
        }
        let definition = CardDefinitionId(
            u16::try_from(usize_field(shown, "definition")?).map_err(|_| "definition too large")?,
        );
        let owner = seat_index(field(state, "owner")?)?;
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
                let payload_state = field(state, "abilityPayload")?;
                if payload_state.is_null() {
                    return Err("stack ability is missing its frozen payload".into());
                }
                let origin = parse_ability_origin(field(shown, "ability")?)?;
                let source = optional_id(shown.get("sourceObjectId"));
                let definition_snapshot =
                    catalog_ability(&game.catalog, field(payload_state, "abilityLocator")?)
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
                let targets = array(field(payload_state, "targetSelections")?)?
                    .iter()
                    .map(parse_target_selection)
                    .collect::<Result<Vec<_>, _>>()?;
                let context = parse_trigger_context(field(payload_state, "context")?)?;
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
                    mode_effects: Vec::new(),
                    x: u16::try_from(usize_field(shown, "x")?)
                        .map_err(|_| "ability X is too large")?,
                };
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
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            is_copy: false,
        });
    }
    Ok(stack)
}

fn parse_trigger_context(value: &Value) -> Result<TriggerContext, String> {
    let optional_seat = |name| {
        value
            .get(name)
            .filter(|value| !value.is_null())
            .map(seat_index)
            .transpose()
    };
    Ok(TriggerContext {
        object: optional_id(value.get("object")),
        object_controller: optional_seat("objectController")?,
        event_player: optional_seat("eventPlayer")?,
        amount: value
            .get("amount")
            .filter(|value| !value.is_null())
            .map(|value| {
                value
                    .as_i64()
                    .and_then(|amount| i32::try_from(amount).ok())
                    .ok_or_else(|| "trigger amount must be an i32".to_owned())
            })
            .transpose()?,
        chosen_objects: [None; crate::ChoiceIndex::COUNT],
    })
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
