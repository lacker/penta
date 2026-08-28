use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::model::{
    DecisionCardOriginSnapshot, DecisionContinuationSnapshot, DecisionStateSnapshot,
    DecisionZoneSnapshot,
};
use super::wire::{
    array, card_definition_id_field, field, player_from_index, str_field, u32_field,
};
use super::{CardDefinitionId, CardInstance, GameObjectId, PlayerId};

/// Gives the hidden-zone card a stack ability names back the object id the
/// observation published for it.
///
/// The counterpart of [`rebind_visible_decision_cards`], for the sources the
/// stack names rather than the cards a decision offers. A Miracle's trigger
/// is the case that needs it: the card it came from was revealed and its id
/// is published all over the observation, but it is still sitting in a hand
/// the viewer cannot read, and the importer minted that hand fresh.
pub(super) fn rebind_stack_source_cards(
    origins: &[DecisionCardOriginSnapshot],
    hands: &mut [Vec<CardInstance>; 2],
    libraries: &mut [Vec<CardInstance>; 2],
    outside_game: &mut [Vec<CardInstance>; 2],
) -> Result<(), String> {
    for origin in origins {
        let seat = player_from_index(origin.seat)?;
        let cards = match origin.zone {
            DecisionZoneSnapshot::Hand => &mut hands[seat.index()],
            DecisionZoneSnapshot::Library => &mut libraries[seat.index()],
            DecisionZoneSnapshot::OutsideGame => &mut outside_game[seat.index()],
            _ => return Err("a stack source origin must name a hidden zone".into()),
        };
        let card = cards
            .get_mut(origin.index)
            .ok_or("a stack source origin is out of range of its hypothesis")?;
        card.id = GameObjectId(origin.object_id);
    }
    Ok(())
}

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
    let Some(state) = state else {
        return Ok(());
    };

    let origins = state
        .card_origins
        .iter()
        .map(|origin| (GameObjectId(origin.object_id), *origin))
        .collect::<BTreeMap<_, _>>();
    let visible_cards = visible_decision_cards(decision)?;
    let detached = detached_decision_cards(&state.continuation);
    for (object, _, option_zone) in &visible_cards {
        let hidden_option_zone = hidden_decision_zone(*option_zone);
        let origin = origins.get(object);
        if hidden_option_zone && origin.is_none() && !detached.contains(object) {
            return Err("visible hidden-zone decision card lacks a card origin".into());
        }
        if hidden_option_zone && origin.is_some_and(|origin| origin.zone != *option_zone) {
            return Err("visible decision card origin disagrees with its option zone".into());
        }
    }
    rebind_visible_hidden_positions(
        &visible_cards,
        &origins,
        viewer,
        hands,
        libraries,
        outside_game,
    )?;
    let mut rebound_hands = [BTreeSet::new(), BTreeSet::new()];
    let mut rebound_libraries = [BTreeSet::new(), BTreeSet::new()];
    let mut rebound_outside_game = [BTreeSet::new(), BTreeSet::new()];
    for (object, definition, _option_zone) in visible_cards {
        let origin = origins.get(&object);
        let Some(origin) = origin else {
            // Public-zone cards already keep their object ids.
            continue;
        };
        let seat = player_from_index(origin.seat)?;
        let (cards, rebound, description, requires_exact_id) = match origin.zone {
            DecisionZoneSnapshot::Library => (
                &mut libraries[seat.index()],
                &mut rebound_libraries[seat.index()],
                "hidden library hypothesis",
                true,
            ),
            DecisionZoneSnapshot::Hand => (
                &mut hands[seat.index()],
                &mut rebound_hands[seat.index()],
                if seat == viewer {
                    "public hand"
                } else {
                    "hidden hand hypothesis"
                },
                true,
            ),
            DecisionZoneSnapshot::OutsideGame => (
                &mut outside_game[seat.index()],
                &mut rebound_outside_game[seat.index()],
                "hidden outside-game hypothesis",
                true,
            ),
            _ => continue,
        };
        let exact = requires_exact_id
            .then(|| {
                cards
                    .iter()
                    .enumerate()
                    .find(|(index, card)| card.id == object && !rebound.contains(index))
            })
            .flatten();
        let inferred = (!requires_exact_id)
            .then(|| match origin.zone {
                DecisionZoneSnapshot::Library => {
                    cards.iter().enumerate().rev().find(|(index, card)| {
                        card.definition == definition && !rebound.contains(index)
                    })
                }
                _ => cards.iter().enumerate().find(|(index, card)| {
                    card.definition == definition && !rebound.contains(index)
                }),
            })
            .flatten();
        let index = exact
            .or(inferred)
            .map(|(index, _)| index)
            .ok_or_else(|| format!("visible decision card is absent from the {description}"))?;
        cards[index].id = object;
        rebound.insert(index);
    }
    Ok(())
}

fn rebind_visible_hidden_positions(
    visible: &[(GameObjectId, CardDefinitionId, DecisionZoneSnapshot)],
    origins: &BTreeMap<GameObjectId, DecisionCardOriginSnapshot>,
    viewer: PlayerId,
    hands: &mut [Vec<CardInstance>; 2],
    libraries: &mut [Vec<CardInstance>; 2],
    outside_game: &mut [Vec<CardInstance>; 2],
) -> Result<(), String> {
    for seat in [PlayerId::One, PlayerId::Two] {
        for zone in [
            DecisionZoneSnapshot::Hand,
            DecisionZoneSnapshot::Library,
            DecisionZoneSnapshot::OutsideGame,
        ] {
            let cards = match zone {
                DecisionZoneSnapshot::Hand => &mut hands[seat.index()],
                DecisionZoneSnapshot::Library => &mut libraries[seat.index()],
                DecisionZoneSnapshot::OutsideGame => &mut outside_game[seat.index()],
                _ => unreachable!(),
            };
            rebind_visible_hidden_collection(visible, origins, viewer, seat, zone, cards)?;
        }
    }
    Ok(())
}

fn rebind_visible_hidden_collection(
    visible: &[(GameObjectId, CardDefinitionId, DecisionZoneSnapshot)],
    origins: &BTreeMap<GameObjectId, DecisionCardOriginSnapshot>,
    viewer: PlayerId,
    seat: PlayerId,
    zone: DecisionZoneSnapshot,
    cards: &mut Vec<CardInstance>,
) -> Result<(), String> {
    let mut desired = visible
        .iter()
        .filter_map(|(object, definition, _)| {
            let origin = origins.get(object)?;
            (origin.zone == zone && origin.seat == seat.index()).then_some((
                *object,
                *definition,
                origin.index,
            ))
        })
        .collect::<Vec<_>>();
    if desired.is_empty() {
        return Ok(());
    }
    desired.sort_by_key(|(_, _, index)| *index);
    let mut used = vec![false; cards.len()];
    let mut positioned = vec![None; cards.len()];
    for (object, definition, index) in desired {
        if index >= cards.len() || positioned[index].is_some() {
            return Err("visible hidden-zone decision card has an invalid exact index".into());
        }
        let source = cards
            .iter()
            .enumerate()
            .find(|(candidate, card)| {
                !used[*candidate]
                    && if zone == DecisionZoneSnapshot::Hand && seat == viewer {
                        card.id == object
                    } else {
                        card.definition == definition
                    }
            })
            .map(|(candidate, _)| candidate)
            .ok_or("visible decision card is absent from the hidden-zone hypothesis")?;
        used[source] = true;
        let mut card = cards[source].clone();
        card.id = object;
        positioned[index] = Some(card);
    }
    let mut remaining = cards
        .iter()
        .enumerate()
        .filter(|(index, _)| !used[*index])
        .map(|(_, card)| card.clone());
    for slot in &mut positioned {
        if slot.is_none() {
            *slot = remaining.next();
        }
    }
    if remaining.next().is_some() || positioned.iter().any(Option::is_none) {
        return Err("visible hidden-zone origins do not preserve the hypothesis size".into());
    }
    *cards = positioned.into_iter().flatten().collect();
    Ok(())
}

fn detached_decision_cards(continuation: &DecisionContinuationSnapshot) -> BTreeSet<GameObjectId> {
    if let DecisionContinuationSnapshot::ScryTop { top, bottom, .. } = continuation {
        return top
            .iter()
            .chain(bottom)
            .map(|card| GameObjectId(card.object_id))
            .collect();
    }
    let cards = match continuation {
        DecisionContinuationSnapshot::ScryBottom { revealed, .. } => revealed.as_slice(),
        _ => &[],
    };
    cards
        .iter()
        .map(|card| GameObjectId(card.object_id))
        .collect()
}

fn visible_decision_cards(
    decision: &Value,
) -> Result<Vec<(GameObjectId, CardDefinitionId, DecisionZoneSnapshot)>, String> {
    let mut cards = Vec::new();
    let mut seen = BTreeMap::new();
    for option in array(field(decision, "options")?)? {
        let zone = decision_zone(str_field(option, "zone")?)?;
        if let Some(card) = option.get("card").filter(|value| !value.is_null()) {
            insert_visible_card(&mut cards, &mut seen, card, zone)?;
        }
        for member in array(field(option, "members")?)? {
            insert_visible_card(&mut cards, &mut seen, member, zone)?;
        }
    }
    Ok(cards)
}

fn insert_visible_card(
    cards: &mut Vec<(GameObjectId, CardDefinitionId, DecisionZoneSnapshot)>,
    seen: &mut BTreeMap<GameObjectId, (CardDefinitionId, DecisionZoneSnapshot)>,
    value: &Value,
    zone: DecisionZoneSnapshot,
) -> Result<(), String> {
    let object = GameObjectId(u32_field(value, "objectId")?);
    // A decision can offer something that is not a card: a token creature,
    // an emblem, a face-down body. None of them has a catalog definition on
    // the wire, and none of them can have come out of a hidden zone, so
    // there is nothing here to rebind a hypothesis against. Out of a hidden
    // zone the same silence is a real problem, and still says so.
    if value.get("definition").is_none_or(Value::is_null) {
        if hidden_decision_zone(zone) {
            return Err("a decision card in a hidden zone lacks a definition".into());
        }
        return Ok(());
    }
    let definition = card_definition_id_field(value, "definition")?;
    match seen.insert(object, (definition, zone)) {
        Some(previous) if previous != (definition, zone) => {
            return Err("one visible decision card has conflicting definitions or zones".into());
        }
        Some(_) => {}
        None => cards.push((object, definition, zone)),
    }
    Ok(())
}

/// The zones a decision option can name that the viewer cannot simply read
/// off the observation, and whose contents therefore have to be rebound
/// against the supplied hypothesis.
const fn hidden_decision_zone(zone: DecisionZoneSnapshot) -> bool {
    matches!(
        zone,
        DecisionZoneSnapshot::Hand
            | DecisionZoneSnapshot::Library
            | DecisionZoneSnapshot::OutsideGame
    )
}

fn decision_zone(value: &str) -> Result<DecisionZoneSnapshot, String> {
    match value {
        "Hand" => Ok(DecisionZoneSnapshot::Hand),
        "Graveyard" => Ok(DecisionZoneSnapshot::Graveyard),
        "Battlefield" => Ok(DecisionZoneSnapshot::Battlefield),
        "Stack" => Ok(DecisionZoneSnapshot::Stack),
        "Library" => Ok(DecisionZoneSnapshot::Library),
        "Exile" => Ok(DecisionZoneSnapshot::Exile),
        "OutsideGame" => Ok(DecisionZoneSnapshot::OutsideGame),
        "Command" => Ok(DecisionZoneSnapshot::Command),
        "DrawnThisStep" => Ok(DecisionZoneSnapshot::DrawnThisStep),
        "None" => Ok(DecisionZoneSnapshot::None),
        other => Err(format!("unknown decision zone {other}")),
    }
}
