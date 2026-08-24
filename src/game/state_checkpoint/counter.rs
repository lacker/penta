use serde_json::Value;

use super::wire::{array, energy_pair, field, poison_pair, str_field, u32_field, usize_field};
use super::{CardInstance, CounterKind, GameObjectId, PlayerId};
use crate::game::counters::Counters;

/// Every public counter each player carries. The legacy poison and energy
/// arrays remain projections for existing consumers, but checkpoint v9 reads
/// this open named collection so new player counters reconstruct faithfully.
pub(super) fn player_counters(observation: &Value) -> Result<[Counters; 2], String> {
    let poison = poison_pair(observation)?;
    let energy = energy_pair(observation)?;
    let Some(value) = observation.get("playerCounters") else {
        return Ok([PlayerId::One, PlayerId::Two].map(|player| {
            let mut counters = Counters::new();
            counters.set(CounterKind::Poison, poison[player.index()]);
            counters.set(CounterKind::Energy, energy[player.index()]);
            counters
        }));
    };
    let seats = array(value)?;
    if seats.len() != 2 {
        return Err("playerCounters must contain p1 and p2 values".into());
    }
    let parsed: [Result<Counters, String>; 2] = [0, 1].map(|seat| {
        let mut counters = Counters::new();
        for entry in array(&seats[seat])? {
            let name = str_field(entry, "name")?;
            let kind = CounterKind::from_name(name)
                .ok_or_else(|| format!("unknown player counter name {name}"))?;
            let count = u16::try_from(usize_field(entry, "count")?)
                .map_err(|_| "player counter count is too large")?;
            if count == 0 || counters.count(kind) != 0 {
                return Err("player counter entries must be unique and nonzero".into());
            }
            counters.set(kind, count);
        }
        Ok(counters)
    });
    let [one, two] = parsed;
    let counters = [one?, two?];
    for player in [PlayerId::One, PlayerId::Two] {
        if counters[player.index()].count(CounterKind::Poison) != poison[player.index()]
            || counters[player.index()].count(CounterKind::Energy) != energy[player.index()]
        {
            return Err("playerCounters disagree with poison or energy projections".into());
        }
    }
    Ok(counters)
}

fn visible_card_counters(observation: &Value) -> Result<Vec<(GameObjectId, Counters)>, String> {
    let Some(value) = observation.get("cardCounters") else {
        return Ok(Vec::new());
    };
    array(value)?
        .iter()
        .map(|object| {
            let id = GameObjectId(u32_field(object, "objectId")?);
            let mut counters = Counters::new();
            for entry in array(field(object, "counters")?)? {
                let name = str_field(entry, "name")?;
                let kind = CounterKind::from_name(name)
                    .ok_or_else(|| format!("unknown card counter name {name}"))?;
                let count = u16::try_from(usize_field(entry, "count")?)
                    .map_err(|_| "card counter count is too large")?;
                if count == 0 || counters.count(kind) != 0 {
                    return Err("card counter entries must be unique and nonzero".into());
                }
                counters.set(kind, count);
            }
            if counters.is_empty() {
                return Err("cardCounters entries must contain at least one counter".into());
            }
            Ok((id, counters))
        })
        .collect()
}

pub(super) fn restore_visible_card_counters(
    observation: &Value,
    hands: &mut [Vec<CardInstance>; 2],
    graveyards: &mut [Vec<CardInstance>; 2],
    exiles: &mut [Vec<CardInstance>; 2],
) -> Result<(), String> {
    for (id, counters) in visible_card_counters(observation)? {
        let mut found = 0;
        for zones in [&mut *hands, &mut *graveyards, &mut *exiles] {
            for zone in zones {
                if let Some(card) = zone.iter_mut().find(|card| card.id == id) {
                    card.counters = counters.clone();
                    found += 1;
                }
            }
        }
        match found {
            0 => return Err("cardCounters names an object outside its visible zones".into()),
            1 => {}
            _ => return Err("cardCounters contains an ambiguous object id".into()),
        }
    }
    Ok(())
}
