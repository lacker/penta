use serde_json::{Value, json};

use super::action_json::action_json;
use super::json_common::{
    ability_origin_json, cast_signature_json, decision_visibility_name, decision_zone_name,
    defender_json, object_characteristics_json, object_characteristics_name, seat_name, step_name,
    target_json,
};
use super::{ENGINE_VERSION, PROTOCOL_CAPABILITIES, PROTOCOL_VERSION, SIMULATION_FINGERPRINT};
use crate::card::SpellForm;
use crate::casting::CastSignature;
use crate::game::{DecisionKind, DecisionObservation, DecisionOrderSemantics, StackObservation};
use crate::{
    AbilityOrigin, Action, CardCatalog, Format, GameObjectId, GameResult, PhysicalFaceObservation,
    PhysicalFaceSide, PlayerObservation, StackObjectKind, WinReason,
};

fn card_name(catalog: &CardCatalog, definition: crate::CardDefinitionId) -> Value {
    catalog
        .get(definition)
        .map_or(Value::Null, |card| Value::from(card.name.clone()))
}

fn physical_face_json(face: PhysicalFaceObservation) -> Value {
    json!({
        "kind": match face.kind {
            crate::DoubleFacedKind::Transforming => "transforming",
            crate::DoubleFacedKind::Modal => "modal",
        },
        "side": match face.side {
            PhysicalFaceSide::Front => "front",
            PhysicalFaceSide::Back => "back",
        },
    })
}

fn stack_card_name(
    catalog: &CardCatalog,
    characteristics: crate::ObjectCharacteristics,
    signature: Option<&CastSignature>,
) -> Value {
    let crate::ObjectCharacteristics::Card { definition, .. } = characteristics else {
        return object_characteristics_name(catalog, characteristics)
            .map_or(Value::Null, Value::from);
    };
    let Some(card) = catalog.get(definition) else {
        return Value::Null;
    };
    let Some(signature) = signature else {
        return Value::from(card.name.clone());
    };

    let resolved = match signature.form() {
        SpellForm::Part(part) => card.part(*part).map(|part| part.name.clone()),
        SpellForm::Combined(parts) if !parts.is_empty() => parts
            .iter()
            .map(|part| card.part(*part).map(|part| part.name.as_str()))
            .collect::<Option<Vec<_>>>()
            .map(|names| names.join(" // ")),
        SpellForm::Combined(_) => None,
    };
    Value::from(resolved.unwrap_or_else(|| card.name.clone()))
}

fn presented_object_json(
    catalog: &CardCatalog,
    object: GameObjectId,
    characteristics: crate::ObjectCharacteristics,
) -> Value {
    let mut value = json!({
        "objectId": object.0,
        "instance": object.0,
        "characteristics": object_characteristics_json(characteristics),
        "name": object_characteristics_name(catalog, characteristics),
    });
    if let crate::ObjectCharacteristics::Card { definition, part } = characteristics {
        value["definition"] = Value::from(definition.get());
        value["presentedPartId"] = Value::from(part.0);
    }
    value
}

fn card_list_json(
    catalog: &CardCatalog,
    cards: &[(GameObjectId, crate::CardDefinitionId)],
) -> Value {
    Value::from(
        cards
            .iter()
            .map(|(instance, definition)| {
                json!({
                    "objectId": instance.0,
                    "instance": instance.0,
                    "definition": definition.get(),
                    "name": card_name(catalog, *definition),
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn mana_pool_json(pool: &crate::ManaPool) -> Value {
    json!({
        "white": pool.white,
        "blue": pool.blue,
        "black": pool.black,
        "red": pool.red,
        "green": pool.green,
        "colorless": pool.colorless,
    })
}

pub(super) fn decision_json(catalog: &CardCatalog, decision: &DecisionObservation) -> Value {
    let mut value = json!({
        "id": decision.id,
        "seat": seat_name(decision.player),
        "kind": match decision.kind {
            DecisionKind::Choice => "Choice",
            DecisionKind::TriggerOrder => "TriggerOrder",
            DecisionKind::TriggerPlacement => "TriggerPlacement",
        },
        "sourceObjectId": decision.source.map(|source| source.0),
        "prompt": decision.prompt,
        "visibility": decision_visibility_name(decision.visibility),
        "minimum": decision.minimum,
        "maximum": decision.maximum,
        "cancellable": decision.cancellable,
        "options": decision.options.iter().map(|option| json!({
            "id": option.id,
            "triggerId": matches!(decision.kind, DecisionKind::TriggerOrder).then_some(option.id),
            "label": option.label,
            "card": option.card.map(|(object, characteristics)| {
                presented_object_json(catalog, object, characteristics)
            }),
            "members": option.members.iter().map(|(object, characteristics)| {
                presented_object_json(catalog, *object, *characteristics)
            }).collect::<Vec<_>>(),
            "abilityText": option.ability_text,
            "zone": decision_zone_name(option.zone),
        })).collect::<Vec<_>>(),
    });
    if let Some(order_semantics) = decision.order_semantics {
        value["orderSemantics"] = Value::from(match order_semantics {
            DecisionOrderSemantics::Resolution => "resolution",
        });
    }
    value
}

fn result_json(result: GameResult) -> Value {
    match result {
        GameResult::Draw => json!({ "winner": Value::Null, "reason": "Draw" }),
        GameResult::Winner { winner, reason } => json!({
            "winner": seat_name(winner),
            "reason": match reason {
                WinReason::OpponentConceded => "OpponentConceded",
                WinReason::OpponentLostAllLife => "OpponentLostAllLife",
                WinReason::OpponentLostToAnEffect => "OpponentLostToAnEffect",
                WinReason::WonByAnEffect => "WonByAnEffect",
                WinReason::OpponentTriedToDrawFromEmptyLibrary =>
                    "OpponentTriedToDrawFromEmptyLibrary",
                WinReason::OpponentRanOutOfTime => "OpponentRanOutOfTime",
                WinReason::OpponentPoisoned => "OpponentPoisoned",
            },
        }),
    }
}

fn permanent_observation_json(
    catalog: &CardCatalog,
    permanent: &crate::PermanentObservation,
) -> Value {
    let mut value = json!({
        "objectId": permanent.id.0,
        "instance": permanent.id.0,
        "characteristics": object_characteristics_json(permanent.characteristics),
        "name": object_characteristics_name(catalog, permanent.characteristics),
        "token": permanent.token,
        "hasIndividualState": permanent.has_individual_state,
        "controller": seat_name(permanent.controller),
        "faceDown": permanent.face_down,
        "phasedOut": permanent.phased_out,
        "chosenCardName": permanent.chosen_card_name.as_deref(),
        "chosenCreatureType": permanent.chosen_creature_type.as_deref(),
        "chosenBasicLandType": permanent.chosen_basic_land_type.map(crate::card::BasicLandType::subtype),
        "chosenColor": permanent.chosen_color.map(super::json_common::mana_color_name),
        "tapped": permanent.tapped,
        "power": permanent.power,
        "toughness": permanent.toughness,
        "damage": permanent.damage,
        "counters": permanent.counters.iter().map(|counter| json!({
            "name": counter.name,
            "count": counter.count,
        })).collect::<Vec<_>>(),
        "loyalty": permanent.loyalty,
        "loyaltyAbilityUsedThisTurn": permanent.loyalty_ability_used_this_turn,
        "attacking": permanent.attacking,
        "attackDefender": permanent.attack_defender.map(defender_json),
        "blockedThisCombat": permanent.blocked_this_combat,
        "blocking": permanent.blocking.iter().map(|id| id.0).collect::<Vec<_>>(),
        "blockingThisCombat": permanent.blocking_this_combat,
        "attackingBand": permanent.attacking_band,
        "flying": permanent.flying,
        "canAttack": permanent.can_attack,
        "enteredThisTurn": permanent.entered_this_turn,
    });
    if let crate::ObjectCharacteristics::Card { definition, part } = permanent.characteristics {
        value["definition"] = Value::from(definition.get());
        value["presentedPartId"] = Value::from(part.0);
    }
    if let Some(face) = permanent.physical_face {
        value["physicalFace"] = physical_face_json(face);
    }
    value
}

fn emblem_observation_json(emblem: &crate::EmblemObservation) -> Value {
    json!({
        "objectId": emblem.id.0,
        "controller": seat_name(emblem.controller),
        "name": emblem.name,
        "sourceAbility": ability_origin_json(emblem.source_ability),
        "abilityTexts": emblem.ability_texts,
    })
}

/// Serializes one seat's redacted view of the game.
///
/// This is the observation a bot decides from: public zones in full, the
/// opponent's hand as a count, and `legalActions` carrying the indices the
/// bot answers with. `pregame` is true while mulligans are being settled.
/// `actions` is the protocol action list the indices refer to — normally
/// [`super::protocol_actions`] of the same observation.
#[must_use]
pub fn observation_json(
    catalog: &CardCatalog,
    observation: &PlayerObservation,
    pregame: bool,
    actions: &[Action],
) -> Value {
    observation_json_for_format(
        catalog,
        Format::OldSchool9394,
        observation,
        pregame,
        actions,
    )
}

/// Serializes one seat's redacted view together with its governing format.
#[must_use]
pub fn observation_json_for_format(
    catalog: &CardCatalog,
    format: Format,
    observation: &PlayerObservation,
    pregame: bool,
    actions: &[Action],
) -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "protocolCapabilities": PROTOCOL_CAPABILITIES,
        "engineVersion": ENGINE_VERSION,
        "simulationFingerprint": SIMULATION_FINGERPRINT,
        "format": format.slug(),
        "seat": seat_name(observation.viewer),
        "pregame": pregame,
        "turn": observation.turn,
        "activeTurn": observation.active_turn,
        "activeSeat": seat_name(observation.active_player),
        "prioritySeat": seat_name(observation.priority),
        "step": step_name(observation.step),
        "regularCombatDamagePending": observation.regular_combat_damage_pending,
        "life": observation.life_totals,
        "poison": observation.poison_counters,
        "energy": observation.energy_counters,
        "playerCounters": observation.counters.each_ref().map(|counters| counters.iter().map(|counter| json!({
            "name": counter.name,
            "count": counter.count,
        })).collect::<Vec<_>>()),
        // Optional, and present only while someone holds the crown. A game
        // in which nobody ever becomes the monarch has the shape it always
        // had.
        "monarch": observation.monarch.map(seat_name),
        "manaPools": [
            mana_pool_json(&observation.mana_pools[0]),
            mana_pool_json(&observation.mana_pools[1]),
        ],
        "hand": card_list_json(catalog, &observation.hand),
        "opponentHandSize": observation.opponent_hand_size,
        "lastSeenHand": observation.last_seen_hand.as_ref().map(|(player, cards)| json!({
            "seat": seat_name(*player),
            "cards": card_list_json(catalog, cards),
        })),
        "librarySizes": observation.library_sizes,
        "revealedLibraryTop": observation
            .revealed_library_top
            .map(|card| card_list_json(catalog, &[card])),
        "opponentRevealedLibraryTop": observation
            .opponent_revealed_library_top
            .map(|card| card_list_json(catalog, &[card])),
        // The viewer's own companions, still outside the game. Empty for
        // every game nobody brought one to, which is nearly all of them.
        "companions": card_list_json(catalog, &observation.companions),
        "graveyards": [
            card_list_json(catalog, &observation.graveyards[0]),
            card_list_json(catalog, &observation.graveyards[1]),
        ],
        "exiles": [
            card_list_json(catalog, &observation.exiles[0]),
            card_list_json(catalog, &observation.exiles[1]),
        ],
        // A card lying face down in exile is absent from the list above
        // rather than shown, so it is counted here instead -- the same way a
        // hand is a size rather than a list.
        "faceDownExileSizes": observation.face_down_exile_sizes,
        "cardCounters": observation.card_counters.iter().map(|object| json!({
            "objectId": object.object.0,
            "counters": object.counters.iter().map(|counter| json!({
                "name": counter.name,
                "count": counter.count,
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "battlefield": observation.battlefield.iter().map(|permanent| permanent_observation_json(catalog, permanent)).collect::<Vec<_>>(),
        "emblems": observation.emblems.iter().map(emblem_observation_json).collect::<Vec<_>>(),
        "stack": observation
            .stack
            .iter()
            .map(|object| stack_object_json(catalog, object))
            .collect::<Vec<_>>(),
        "decision": observation.decision.as_ref().map(|decision| decision_json(catalog, decision)),
        "result": observation.result.map(result_json),
        "legalActions": actions.iter().enumerate().map(|(index, action)| {
            let mut value = action_json(action);
            if let Value::Object(map) = &mut value {
                map.insert("index".into(), Value::from(index));
            }
            value
        }).collect::<Vec<_>>(),
        "checkpoint": observation.checkpoint,
    })
}

pub(super) fn stack_object_json(catalog: &CardCatalog, object: &StackObservation) -> Value {
    let ability_id = object.ability.and_then(|origin| match origin {
        AbilityOrigin::Printed { ability, .. } => Some(ability.0),
        AbilityOrigin::Token { .. }
        | AbilityOrigin::Emblem { .. }
        | AbilityOrigin::FaceDown { .. }
        | AbilityOrigin::IntrinsicBasicLand(_)
        | AbilityOrigin::IntrinsicCounter(_)
        | AbilityOrigin::Granted { .. }
        | AbilityOrigin::TokenGranted { .. }
        | AbilityOrigin::EmblemGranted { .. }
        | AbilityOrigin::FaceDownGranted { .. } => None,
    });
    let mut value = json!({
        "objectId": object.id.0,
        "stackId": object.id.0,
        // Compatibility alias: this is a game object, not physical lineage.
        "instance": object.id.0,
        "sourceObjectId": object.source.map(|source| source.0),
        "source": object.source.map(|source| source.0),
        "ability": object.ability.map(ability_origin_json),
        // Compatibility projection for clients that only know printed clause IDs.
        "abilityId": ability_id,
        "abilityText": object.ability_text,
        "kind": match object.kind {
            StackObjectKind::Spell => "Spell",
            StackObjectKind::ActivatedAbility => "ActivatedAbility",
            StackObjectKind::TriggeredAbility => "TriggeredAbility",
        },
        "characteristics": object_characteristics_json(object.characteristics),
        "name": stack_card_name(catalog, object.characteristics, object.signature.as_ref()),
        "controller": seat_name(object.controller),
        "counterable": object.counterable,
        "signature": object.signature.as_ref().map(cast_signature_json),
        "targets": object
            .targets
            .iter()
            .copied()
            .map(target_json)
            .collect::<Vec<_>>(),
        "chosenPermanents": object
            .chosen_permanents
            .iter()
            .map(|permanent| permanent.0)
            .collect::<Vec<_>>(),
        "x": object.x,
    });
    if let crate::ObjectCharacteristics::Card { definition, part } = object.characteristics {
        value["definition"] = Value::from(definition.get());
        value["presentedPartId"] = Value::from(part.0);
    }
    value
}
