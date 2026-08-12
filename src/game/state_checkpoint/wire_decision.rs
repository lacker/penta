use std::collections::BTreeSet;

use serde_json::Value;

use super::model::{DecisionContinuationSnapshot, DecisionStateSnapshot, ZoneKindSnapshot};
use super::wire::{array, field, player_from_index, seat_value, str_field, u32_field, usize_field};
use super::{CardDefinitionId, CardInstance, GameObjectId, PlayerId};

pub(super) fn rebind_visible_decision_cards(
    observation: &Value,
    state: Option<&DecisionStateSnapshot>,
    viewer: PlayerId,
    hands: &mut [Vec<CardInstance>; 2],
    libraries: &mut [Vec<CardInstance>; 2],
    outside_game: &mut [Vec<CardInstance>; 2],
) -> Result<(), String> {
    let Some(decision) = observation.get("decision").filter(|value| !value.is_null()) else {
        return Ok(());
    };
    if seat_value(field(decision, "seat")?)? != viewer {
        return Ok(());
    }
    let hand_owner = state.and_then(|state| match state.continuation {
        DecisionContinuationSnapshot::ExileFromHand { victim }
        | DecisionContinuationSnapshot::Duress { victim, .. } => player_from_index(victim).ok(),
        DecisionContinuationSnapshot::SearchZone {
            source: ZoneKindSnapshot::Hand,
            ..
        }
        | DecisionContinuationSnapshot::ChooseCards { .. } => Some(viewer),
        _ => None,
    });
    let cards_remain_in_library = !state.is_some_and(|state| {
        matches!(
            state.continuation,
            DecisionContinuationSnapshot::GrislySalvage { .. }
                | DecisionContinuationSnapshot::AugurOfBolas { .. }
                | DecisionContinuationSnapshot::TopCardSelection { .. }
                | DecisionContinuationSnapshot::RevealedPileSplit { .. }
                | DecisionContinuationSnapshot::RevealedPileChoice { .. }
        )
    });
    let mut rebound_hands = [BTreeSet::new(), BTreeSet::new()];
    let mut rebound_libraries = [BTreeSet::new(), BTreeSet::new()];
    let mut rebound_outside_game = [BTreeSet::new(), BTreeSet::new()];
    for option in array(field(decision, "options")?)? {
        let zone = str_field(option, "zone")?;
        let Some(card_value) = option.get("card").filter(|value| !value.is_null()) else {
            continue;
        };
        let object = GameObjectId(u32_field(card_value, "objectId")?);
        let definition = CardDefinitionId(
            u16::try_from(usize_field(card_value, "definition")?)
                .map_err(|_| "decision card definition is too large")?,
        );
        let (cards, rebound, description, requires_exact_id) = match zone {
            "Library" if cards_remain_in_library => (
                &mut libraries[viewer.index()],
                &mut rebound_libraries[viewer.index()],
                "hidden library hypothesis",
                false,
            ),
            "Hand" if hand_owner.is_some() => {
                let owner = hand_owner.expect("the guard proved a hand owner exists");
                (
                    &mut hands[owner.index()],
                    &mut rebound_hands[owner.index()],
                    if owner == viewer {
                        "public hand"
                    } else {
                        "hidden hand hypothesis"
                    },
                    owner == viewer,
                )
            }
            "OutsideGame" => (
                &mut outside_game[viewer.index()],
                &mut rebound_outside_game[viewer.index()],
                "hidden outside-game hypothesis",
                false,
            ),
            _ => continue,
        };
        let index = cards
            .iter()
            .enumerate()
            .find(|(index, card)| {
                card.id == object && card.definition == definition && !rebound.contains(index)
            })
            .or_else(|| {
                if requires_exact_id {
                    None
                } else {
                    cards.iter().enumerate().rev().find(|(index, card)| {
                        card.definition == definition && !rebound.contains(index)
                    })
                }
            })
            .map(|(index, _)| index)
            .ok_or_else(|| format!("visible decision card is absent from the {description}"))?;
        cards[index].id = object;
        rebound.insert(index);
    }
    Ok(())
}
