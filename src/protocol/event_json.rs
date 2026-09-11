//! Structured public history, including information that no longer occupies a
//! visible zone. Only serialize events from `Game::events_for[_since]`.

use super::json_common::{seat_name, step_name, target_json};
use super::observation_json::{presented_object_json, result_json};
use crate::{CardCatalog, GameEvent};
use serde_json::{Value, json};

/// Serialize a seat-projected event. Private shuffle seeds are also withheld
/// here, so accidentally passing a game-start event cannot disclose a deal.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn event_json(catalog: &CardCatalog, event: &GameEvent) -> Option<Value> {
    let card = |id: crate::GameObjectId, definition: crate::CardDefinitionId| {
        json!({
            "objectId":id.0, "definition":definition.get(),
            "name":catalog.get(definition).map(|card| &card.name),
        })
    };
    Some(match event {
        GameEvent::GameStarted { .. } => return None,
        GameEvent::FaceDownSpellCast {
            player,
            card,
            targets,
        } => {
            json!({"type":"SpellCast", "seat":seat_name(*player), "card":{"objectId":card.0, "faceDown":true}, "targets":targets.iter().copied().map(target_json).collect::<Vec<_>>()})
        }
        GameEvent::FaceDownSpellResolved { card } => {
            json!({"type":"SpellResolved", "card":{"objectId":card.0, "faceDown":true}})
        }
        GameEvent::CardDrawn { player, card } => {
            json!({"type":"CardDrawn", "seat":seat_name(*player), "objectId":card.0})
        }
        GameEvent::CardRevealed {
            player,
            card: id,
            definition,
        } => {
            json!({"type":"CardRevealed", "seat":seat_name(*player), "card":card(*id, *definition)})
        }
        GameEvent::CardsDiscarded { player, cards } => {
            json!({"type":"CardsDiscarded", "seat":seat_name(*player), "cards":cards.iter().map(|(id, definition)| card(*id, *definition)).collect::<Vec<_>>()})
        }
        GameEvent::LandPlayed {
            player,
            card: id,
            definition,
        } => json!({"type":"LandPlayed", "seat":seat_name(*player), "card":card(*id, *definition)}),
        GameEvent::ManaAdded { player, source } => {
            json!({"type":"ManaAdded", "seat":seat_name(*player), "sourceObjectId":source.0})
        }
        GameEvent::SpellCast {
            player,
            card: id,
            definition,
            targets,
        } => {
            json!({"type":"SpellCast", "seat":seat_name(*player), "card":card(*id, *definition), "targets":targets.iter().copied().map(target_json).collect::<Vec<_>>()})
        }
        GameEvent::SpellResolved {
            card: id,
            definition,
        }
        | GameEvent::SpellFizzled {
            card: id,
            definition,
        } => {
            json!({"type":if matches!(event, GameEvent::SpellResolved { .. }) {"SpellResolved"} else {"SpellFizzled"}, "card":card(*id, *definition)})
        }
        GameEvent::AbilityActivated {
            player,
            object,
            source,
            presentation,
            chosen_permanents,
        } => {
            json!({"type":"AbilityActivated", "seat":seat_name(*player), "objectId":object.0, "source":presented_object_json(catalog, *source, *presentation), "chosenPermanents":chosen_permanents.iter().map(|id| id.0).collect::<Vec<_>>()})
        }
        GameEvent::AbilityResolved {
            object,
            source,
            presentation,
        }
        | GameEvent::AbilityFizzled {
            object,
            source,
            presentation,
        }
        | GameEvent::TriggeredAbilityResolved {
            object,
            source,
            presentation,
        }
        | GameEvent::TriggeredAbilityFizzled {
            object,
            source,
            presentation,
        } => json!({
            "type": match event {
                GameEvent::AbilityResolved { .. } => "AbilityResolved",
                GameEvent::AbilityFizzled { .. } => "AbilityFizzled",
                GameEvent::TriggeredAbilityResolved { .. } => "TriggeredAbilityResolved",
                _ => "TriggeredAbilityFizzled",
            }, "objectId":object.0, "source":presented_object_json(catalog, *source, *presentation),
        }),
        GameEvent::AbilityTriggered {
            player,
            trigger,
            source,
            presentation,
        } => {
            json!({"type":"AbilityTriggered", "seat":seat_name(*player), "triggerId":trigger, "source":presented_object_json(catalog, *source, *presentation)})
        }
        GameEvent::TriggeredAbilityPutOnStack {
            player,
            trigger,
            object,
            source,
            presentation,
        } => {
            json!({"type":"TriggeredAbilityPutOnStack", "seat":seat_name(*player), "triggerId":trigger, "objectId":object.0, "source":presented_object_json(catalog, *source, *presentation)})
        }
        GameEvent::AttackDeclared { player, attackers } => {
            json!({"type":"AttackDeclared", "seat":seat_name(*player), "attackers":attackers.iter().map(|id| id.0).collect::<Vec<_>>()})
        }
        GameEvent::BlockDeclared {
            player,
            assignments,
        } => {
            json!({"type":"BlockDeclared", "seat":seat_name(*player), "assignments":assignments.iter().map(|(blocker, attacker)| json!({"blocker":blocker.0, "attacker":attacker.0})).collect::<Vec<_>>()})
        }
        GameEvent::DamageDealt { player, amount }
        | GameEvent::LifeLost { player, amount }
        | GameEvent::ManaBurn { player, amount } => json!({"type":match event {
            GameEvent::DamageDealt { .. } => "DamageDealt", GameEvent::LifeLost { .. } => "LifeLost", _ => "ManaBurn",
        }, "seat":seat_name(*player), "amount":amount}),
        GameEvent::StepChanged {
            turn,
            active_player,
            step,
        } => {
            json!({"type":"StepChanged", "turn":turn, "activePlayer":seat_name(*active_player), "step":step_name(*step)})
        }
        GameEvent::PermanentLeftBattlefield {
            controller,
            card,
            characteristics,
            destination,
        } => {
            json!({"type":"PermanentLeftBattlefield", "seat":seat_name(*controller), "card":presented_object_json(catalog, *card, *characteristics), "destination":format!("{destination:?}")})
        }
        GameEvent::GameEnded { result } => {
            json!({"type":"GameEnded", "result":result_json(*result)})
        }
    })
}
