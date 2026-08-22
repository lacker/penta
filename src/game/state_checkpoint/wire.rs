#![allow(clippy::wildcard_imports)]

use super::*;

impl Game {
    /// Projection for the current checkpoint format. The checkpoint has one typed schema
    /// internally; only this boundary turns it into JSON.
    pub(in crate::game) fn checkpoint_json(&self, viewer: PlayerId) -> Value {
        serde_json::to_value(self.snapshot(viewer)).expect("GameSnapshot is serializable")
    }
}

pub(super) fn field<'a>(value: &'a Value, name: &str) -> Result<&'a Value, String> {
    value
        .get(name)
        .ok_or_else(|| format!("missing field {name}"))
}

pub(super) fn array(value: &Value) -> Result<&Vec<Value>, String> {
    value.as_array().ok_or_else(|| "expected an array".into())
}

pub(super) fn str_field<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    field(value, name)?
        .as_str()
        .ok_or_else(|| format!("field {name} must be a string"))
}

pub(super) fn bool_field(value: &Value, name: &str) -> Result<bool, String> {
    field(value, name)?
        .as_bool()
        .ok_or_else(|| format!("field {name} must be boolean"))
}

pub(super) fn usize_field(value: &Value, name: &str) -> Result<usize, String> {
    field(value, name)?
        .as_u64()
        .and_then(|v| usize::try_from(v).ok())
        .ok_or_else(|| format!("field {name} must be an unsigned integer"))
}

pub(super) fn u32_field(value: &Value, name: &str) -> Result<u32, String> {
    usize_field(value, name)
        .and_then(|v| u32::try_from(v).map_err(|_| format!("field {name} is too large")))
}

pub(super) fn card_definition_id(value: &Value) -> Result<CardDefinitionId, String> {
    value
        .as_u64()
        .and_then(CardDefinitionId::try_new)
        .ok_or_else(|| {
            format!(
                "card definition must be an integer from 1 through {}",
                CardDefinitionId::MAX
            )
        })
}

pub(super) fn card_definition_id_field(
    value: &Value,
    name: &str,
) -> Result<CardDefinitionId, String> {
    card_definition_id(field(value, name)?).map_err(|error| format!("field {name}: {error}"))
}
pub(super) fn u8_field(value: &Value, name: &str) -> Result<u8, String> {
    usize_field(value, name)
        .and_then(|v| u8::try_from(v).map_err(|_| format!("field {name} is too large")))
}

pub(super) fn seat_label(player: PlayerId) -> &'static str {
    if player == PlayerId::One { "p1" } else { "p2" }
}
pub(super) fn seat_value(value: &Value) -> Result<PlayerId, String> {
    match value.as_str() {
        Some("p1") => Ok(PlayerId::One),
        Some("p2") => Ok(PlayerId::Two),
        _ => Err("seat must be p1 or p2".into()),
    }
}
pub(super) fn definitions(value: &Value) -> Result<Vec<CardDefinitionId>, String> {
    array(value)?.iter().map(card_definition_id).collect()
}
pub(super) fn hidden_definitions(
    hidden: &Value,
    zone: &str,
    player: PlayerId,
) -> Result<Vec<CardDefinitionId>, String> {
    definitions(field(field(hidden, zone)?, seat_label(player))?)
}

pub(super) fn card(
    id: GameObjectId,
    definition: CardDefinitionId,
    owner: PlayerId,
    catalog: &CardCatalog,
) -> Result<CardInstance, String> {
    if catalog.get(definition).is_none() {
        return Err(format!("unknown card definition {definition}"));
    }
    Ok(CardInstance {
        id,
        definition,
        owner,
        backing: ObjectBacking::None,
        characteristics: CharacteristicSource::Card(definition),
        counters: [0; CounterKind::COUNT],
    })
}

pub(super) fn parse_cards(
    value: &Value,
    owner: PlayerId,
    catalog: &CardCatalog,
) -> Result<Vec<CardInstance>, String> {
    array(value)?
        .iter()
        .map(|value| {
            let id = GameObjectId(u32_field(value, "objectId")?);
            let definition = card_definition_id_field(value, "definition")?;
            card(id, definition, owner, catalog)
        })
        .collect()
}

pub(super) fn mint_cards(
    definitions: &[CardDefinitionId],
    owner: PlayerId,
    catalog: &CardCatalog,
    next: &mut u32,
) -> Result<Vec<CardInstance>, String> {
    definitions
        .iter()
        .map(|definition| {
            let id = GameObjectId(*next);
            *next = next
                .checked_add(1)
                .ok_or_else(|| "game object ids exhausted".to_owned())?;
            card(id, *definition, owner, catalog)
        })
        .collect()
}

pub(super) fn parse_two_public_zones(
    value: &Value,
    catalog: &CardCatalog,
) -> Result<[Vec<CardInstance>; 2], String> {
    let zones = array(value)?;
    if zones.len() != 2 {
        return Err("public zone must contain p1 and p2 arrays".into());
    }
    Ok([
        parse_cards(&zones[0], PlayerId::One, catalog)?,
        parse_cards(&zones[1], PlayerId::Two, catalog)?,
    ])
}

pub(super) fn max_public_object_id(observation: &Value) -> Option<u32> {
    [
        "hand",
        "graveyards",
        "exiles",
        "battlefield",
        "emblems",
        "stack",
        "decision",
        "checkpoint",
    ]
    .into_iter()
    .filter_map(|name| observation.get(name))
    .flat_map(walk_object_ids)
    .max()
}

pub(super) fn walk_object_ids(value: &Value) -> Box<dyn Iterator<Item = u32> + '_> {
    match value {
        Value::Array(values) => Box::new(values.iter().flat_map(walk_object_ids)),
        Value::Object(map) => Box::new(
            map.get("objectId")
                .and_then(Value::as_u64)
                .and_then(|id| u32::try_from(id).ok())
                .into_iter()
                .chain(map.values().flat_map(walk_object_ids)),
        ),
        _ => Box::new(std::iter::empty()),
    }
}

pub(super) fn read_u8(v: &Value) -> Result<u8, String> {
    v.as_u64()
        .and_then(|n| u8::try_from(n).ok())
        .ok_or_else(|| "expected u8".into())
}
pub(super) fn read_u16(v: &Value) -> Result<u16, String> {
    v.as_u64()
        .and_then(|n| u16::try_from(n).ok())
        .ok_or_else(|| "expected u16".into())
}
pub(super) fn read_u32(v: &Value) -> Result<u32, String> {
    v.as_u64()
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| "expected u32".into())
}
pub(super) fn read_i16(v: &Value) -> Result<i16, String> {
    v.as_i64()
        .and_then(|n| i16::try_from(n).ok())
        .ok_or_else(|| "expected i16".into())
}
pub(super) fn i16_pair(value: &Value) -> Result<[i16; 2], String> {
    let values = array(value)?;
    if values.len() != 2 {
        return Err("expected a two-element array".into());
    }
    Ok([read_i16(&values[0])?, read_i16(&values[1])?])
}

/// Poison counters, which are additive on the wire: an observation written
/// before poison existed simply has none, and reconstructs with none.
pub(super) fn poison_pair(observation: &Value) -> Result<[u16; 2], String> {
    counter_pair(observation, "poison")
}

pub(super) fn energy_pair(observation: &Value) -> Result<[u16; 2], String> {
    counter_pair(observation, "energy")
}

/// One per-seat counter total off the wire. Absent means none: a payload
/// written before that counter existed describes a game with none of it.
fn counter_pair(observation: &Value, key: &str) -> Result<[u16; 2], String> {
    let Some(value) = observation.get(key) else {
        return Ok([0, 0]);
    };
    let values = array(value)?;
    if values.len() != 2 {
        return Err(format!("{key} must contain p1 and p2 values"));
    }
    let mut counters = [0_u16; 2];
    for (slot, value) in counters.iter_mut().zip(values) {
        *slot = value
            .as_u64()
            .and_then(|counters| u16::try_from(counters).ok())
            .ok_or_else(|| format!("{key} counters must be unsigned integers"))?;
    }
    Ok(counters)
}

pub(super) fn parse_mana_pool(value: &Value) -> Result<super::super::ManaPool, String> {
    Ok(super::super::ManaPool {
        white: u16::try_from(usize_field(value, "white")?).map_err(|_| "white mana too large")?,
        blue: u16::try_from(usize_field(value, "blue")?).map_err(|_| "blue mana too large")?,
        black: u16::try_from(usize_field(value, "black")?).map_err(|_| "black mana too large")?,
        red: u16::try_from(usize_field(value, "red")?).map_err(|_| "red mana too large")?,
        green: u16::try_from(usize_field(value, "green")?).map_err(|_| "green mana too large")?,
        colorless: u16::try_from(usize_field(value, "colorless")?)
            .map_err(|_| "colorless mana too large")?,
    })
}
pub(super) fn parse_mana(
    values: &[ManaSnapshot],
    catalog: &CardCatalog,
) -> Result<Vec<Mana>, String> {
    values
        .iter()
        .map(|snapshot| {
            let color = parse_mana_color(snapshot.color);
            let source = snapshot.source.as_ref().map(|source| ManaSource {
                object: GameObjectId(source.object),
                ability: ability_origin_from_snapshot(source.ability),
            });
            let payload = snapshot
                .payload
                .as_ref()
                .map(|locator| {
                    catalog_mana_payload(catalog, locator).ok_or_else(|| {
                        "mana payload locator is absent from this catalog".to_owned()
                    })
                })
                .transpose()?;
            if payload.is_some_and(|payload| match payload.mana {
                crate::card::ManaSelectionDef::One(expected) => expected != color,
                crate::card::ManaSelectionDef::Choice(colors)
                | crate::card::ManaSelectionDef::Combination(colors) => !colors.contains(&color),
            }) {
                return Err("mana payload cannot produce its checkpoint color".into());
            }
            Ok(Mana {
                color,
                source,
                restrictions: payload.map_or(&[], |payload| payload.restrictions),
                spend_effects: payload.map_or(&[], |payload| payload.spend_effects),
            })
        })
        .collect()
}

pub(super) fn mana_pool_from_units(mana: &[Mana]) -> super::super::ManaPool {
    let mut pool = super::super::ManaPool::default();
    for unit in mana {
        pool.add_color(unit.color, 1);
    }
    pool
}

pub(super) fn ids(values: &[u32]) -> Vec<GameObjectId> {
    values.iter().copied().map(GameObjectId).collect()
}
pub(super) fn parse_ids(value: &Value) -> Result<Vec<GameObjectId>, String> {
    array(value)?
        .iter()
        .map(|value| read_u32(value).map(GameObjectId))
        .collect()
}
pub(super) fn parse_drawn_this_turn(
    checkpoint: &GameSnapshot,
    hidden: &Value,
    viewer: PlayerId,
    hands: &[Vec<CardInstance>; 2],
) -> Result<[Vec<GameObjectId>; 2], String> {
    let mut drawn = [Vec::new(), Vec::new()];
    drawn[viewer.index()] = ids(&checkpoint.drawn_this_turn[viewer.index()]);
    let opponent = viewer.opponent();
    if let Some(indices) = hidden
        .get("drawnThisTurn")
        .and_then(|value| value.get(seat_label(opponent)))
    {
        drawn[opponent.index()] = hidden_hand_indices(indices, &hands[opponent.index()])?;
    }
    Ok(drawn)
}

pub(super) fn hidden_hand_indices(
    value: &Value,
    hand: &[CardInstance],
) -> Result<Vec<GameObjectId>, String> {
    array(value)?
        .iter()
        .map(|value| {
            let index = value
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .ok_or_else(|| "hidden hand indices must be unsigned integers".to_owned())?;
            hand.get(index)
                .map(|card| card.id)
                .ok_or_else(|| format!("hidden hand index {index} is out of range"))
        })
        .collect()
}

/// Object ids from a JSON array, ignoring anything that is not one. An
/// absent or non-array value is no ids rather than an error, which is what a
/// permanent blocking nothing looks like on the wire.
pub(super) fn object_id_list(value: Option<&Value>) -> Vec<GameObjectId> {
    value
        .and_then(Value::as_array)
        .map(|ids| {
            ids.iter()
                .filter_map(Value::as_u64)
                .filter_map(|id| u32::try_from(id).ok())
                .map(GameObjectId)
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn optional_id(value: Option<&Value>) -> Option<GameObjectId> {
    value
        .and_then(Value::as_u64)
        .and_then(|id| u32::try_from(id).ok())
        .map(GameObjectId)
}
pub(super) fn parse_last_seen_hand(
    value: Option<&Value>,
) -> Result<super::super::LastSeenHand, String> {
    let Some(value) = value.filter(|value| !value.is_null()) else {
        return Ok(None);
    };
    let player = seat_value(field(value, "seat")?)?;
    let cards = array(field(value, "cards")?)?
        .iter()
        .map(|card| {
            Ok((
                GameObjectId(u32_field(card, "objectId")?),
                card_definition_id_field(card, "definition")?,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Some((player, cards)))
}
pub(super) fn parse_step(value: &str) -> Result<Step, String> {
    match value {
        "Upkeep" => Ok(Step::Upkeep),
        "Draw" => Ok(Step::Draw),
        "PrecombatMain" => Ok(Step::PrecombatMain),
        "BeginningOfCombat" => Ok(Step::BeginningOfCombat),
        "DeclareAttackers" => Ok(Step::DeclareAttackers),
        "DeclareBlockers" => Ok(Step::DeclareBlockers),
        "CombatDamage" => Ok(Step::CombatDamage),
        "EndOfCombat" => Ok(Step::EndOfCombat),
        "PostcombatMain" => Ok(Step::PostcombatMain),
        "End" => Ok(Step::End),
        "Cleanup" => Ok(Step::Cleanup),
        _ => Err(format!("unknown step {value}")),
    }
}
pub(super) fn parse_pregame(value: Option<PregameSnapshot>) -> Result<Option<Pregame>, String> {
    value
        .map(|value| match value {
            PregameSnapshot::Mulligan { seat } => player_from_index(seat).map(Pregame::Mulligan),
            PregameSnapshot::Bottom { seat } => player_from_index(seat).map(Pregame::Bottom),
        })
        .transpose()
}
pub(super) fn parse_combat_stage(value: &CombatDamageStageSnapshot) -> CombatDamageStage {
    match value {
        CombatDamageStageSnapshot::NotStarted => CombatDamageStage::NotStarted,
        CombatDamageStageSnapshot::Single => CombatDamageStage::Single,
        CombatDamageStageSnapshot::FirstStrike { combatants } => CombatDamageStage::FirstStrike {
            strike_wave_combatants: ids(combatants),
        },
        CombatDamageStageSnapshot::RegularAfterFirstStrike { combatants } => {
            CombatDamageStage::RegularAfterFirstStrike {
                strike_wave_combatants: ids(combatants),
            }
        }
    }
}

/// Rebuilds the battlefield, and the permanents that are phased out beside
/// it. Both come from the observation's battlefield list, which carries the
/// phased-out ones last behind a flag: they are public information, so they
/// are shown rather than hidden, and only the rules treat them as absent.
pub(super) fn parse_battlefield(
    observation: &Value,
    snapshots: &[PermanentSnapshot],
    catalog: &CardCatalog,
) -> Result<(Vec<Permanent>, Vec<Permanent>), String> {
    let visible = array(field(observation, "battlefield")?)?;
    if visible.len() != snapshots.len() {
        return Err("checkpoint battlefield does not match observation".into());
    }
    visible
        .iter()
        .zip(snapshots)
        .map(|(shown, state)| {
            let id = GameObjectId(u32_field(shown, "objectId")?);
            if id.0 != state.object_id {
                return Err("checkpoint permanent id does not match observation".into());
            }
            parse_permanent(
                state,
                PermanentPresentation {
                    controller: seat_value(field(shown, "controller")?)?,
                    tapped: bool_field(shown, "tapped")?,
                    damage: u16::try_from(usize_field(shown, "damage")?)
                        .map_err(|_| "damage too large")?,
                    attacking: bool_field(shown, "attacking")?,
                    attack_defender: shown
                        .get("attackDefender")
                        .filter(|value| !value.is_null())
                        .map(parse_attack_defender)
                        .transpose()?,
                    blocked: bool_field(shown, "blockedThisCombat")?,
                    blocking: object_id_list(shown.get("blocking")),
                    // Absent from an observation written before blocker status
                    // outlived the relationship. `blocking` is still read
                    // above, and a live block answers on its own, so such a
                    // payload loses nothing it was able to record.
                    blocking_this_combat: shown
                        .get("blockingThisCombat")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    attacking_band: shown
                        .get("attackingBand")
                        .and_then(Value::as_u64)
                        .and_then(|band| u8::try_from(band).ok()),
                    activated_loyalty_this_turn: bool_field(shown, "loyaltyAbilityUsedThisTurn")?,
                    chosen_creature_type: shown
                        .get("chosenCreatureType")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                    chosen_basic_land_type: shown
                        .get("chosenBasicLandType")
                        .and_then(Value::as_str)
                        .and_then(BasicLandType::from_subtype),
                    chosen_card_name: shown
                        .get("chosenCardName")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                },
                catalog,
            )
            .map(|permanent| (bool_field(shown, "phasedOut").unwrap_or(false), permanent))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|permanents| {
            let mut battlefield = Vec::new();
            let mut phased_out = Vec::new();
            for (phased, permanent) in permanents {
                if phased {
                    phased_out.push(permanent);
                } else {
                    battlefield.push(permanent);
                }
            }
            (battlefield, phased_out)
        })
}

#[allow(clippy::struct_excessive_bools)]
struct PermanentPresentation {
    controller: PlayerId,
    tapped: bool,
    damage: u16,
    attacking: bool,
    attack_defender: Option<AttackDefender>,
    blocked: bool,
    blocking: Vec<GameObjectId>,
    blocking_this_combat: bool,
    attacking_band: Option<u8>,
    activated_loyalty_this_turn: bool,
    chosen_creature_type: Option<String>,
    chosen_basic_land_type: Option<BasicLandType>,
    chosen_card_name: Option<String>,
}

#[allow(clippy::too_many_lines)]
fn parse_permanent(
    state: &PermanentSnapshot,
    shown: PermanentPresentation,
    catalog: &CardCatalog,
) -> Result<Permanent, String> {
    if state.has_dynamic_characteristics {
        return Err(
            "checkpoint permanent has dynamic characteristics not yet represented by semantic locators"
                .into(),
        );
    }
    // Counter kinds are appended rather than inserted, so a checkpoint
    // written before a kind existed simply carries none of it.
    if state.counters.len() > CounterKind::COUNT {
        return Err("counter vector has the wrong length".into());
    }
    let mut counters = [0; CounterKind::COUNT];
    counters[..state.counters.len()].copy_from_slice(&state.counters);
    let owner = player_from_index(state.owner)?;
    let copy_effect = state
        .copy_effect
        .as_ref()
        .map(|copy| parse_copiable_characteristics(copy, catalog))
        .transpose()?;
    let double_faced_token_copy = state
        .double_faced_token_copy
        .as_ref()
        .map(|faces| parse_double_faced_copiable_characteristics(faces, catalog))
        .transpose()?;
    let token_characteristics = state
        .token_characteristics
        .as_ref()
        .map(|token| {
            catalog_token_characteristics(catalog, token)
                .ok_or_else(|| "checkpoint token locator is absent from this catalog".to_owned())
        })
        .transpose()?;
    let object_kind = object_kind_from_snapshot(state.object_kind, catalog)?;
    let characteristics = match object_kind {
        ObjectKind::Card(definition) => {
            if double_faced_token_copy.is_some() {
                return Err(
                    "checkpoint card permanent carries double-faced token-copy values".into(),
                );
            }
            CharacteristicSource::Card(definition)
        }
        ObjectKind::Token => {
            if token_characteristics.is_some() && double_faced_token_copy.is_some() {
                return Err(
                    "checkpoint token has both authored and copied double-faced values".into(),
                );
            }
            let presented = CardPartId(state.presented_part_id);
            if double_faced_token_copy
                .as_ref()
                .is_some_and(|faces| presented != faces.front_part && presented != faces.back_part)
            {
                return Err(
                    "checkpoint double-faced token presents neither of its physical faces".into(),
                );
            }
            let copied_base = copy_effect.as_ref().map(|copy| copy.base).or_else(|| {
                double_faced_token_copy
                    .as_ref()
                    .and_then(|faces| faces.face(presented))
                    .map(|copy| copy.base)
            });
            match (token_characteristics, copied_base) {
                (Some(token), _) => CharacteristicSource::Token(token),
                (None, Some(ObjectCharacteristics::Card { definition, .. })) => {
                    CharacteristicSource::Copy(definition)
                }
                (None, Some(ObjectCharacteristics::Token { token, .. })) => {
                    CharacteristicSource::Token(token)
                }
                (None, Some(ObjectCharacteristics::FaceDown { face_down })) => {
                    CharacteristicSource::FaceDown(face_down)
                }
                (None, Some(ObjectCharacteristics::Emblem { .. })) => {
                    return Err("an emblem cannot supply copied permanent values".into());
                }
                (None, None) => {
                    return Err(
                        "checkpoint token has neither authored nor copied characteristics".into(),
                    );
                }
            }
        }
        ObjectKind::Emblem => {
            return Err("an emblem cannot appear in the battlefield snapshot".into());
        }
        ObjectKind::Ability => {
            if double_faced_token_copy.is_some() {
                return Err(
                    "checkpoint ability permanent carries double-faced token-copy values".into(),
                );
            }
            return Err("a permanent cannot have ability object kind".into());
        }
    };
    let object = ObjectInstance {
        id: GameObjectId(state.object_id),
        definition: object_kind,
        owner,
        backing: ObjectBacking::None,
        characteristics,
        counters: [0; CounterKind::COUNT],
    };
    let mut permanent = Permanent::entering(
        object,
        CardPartId(state.presented_part_id),
        shown.controller,
        state.entered_controller_turn,
    );
    permanent.token_characteristics = token_characteristics;
    permanent.double_faced_token_copy = double_faced_token_copy;
    permanent.timestamp = ContinuousEffectTimestamp(state.timestamp);
    permanent.tapped = shown.tapped;
    permanent.damage = shown.damage;
    permanent.attacking = shown.attacking;
    permanent.attack_defender = shown.attack_defender;
    permanent.blocked = shown.blocked;
    permanent.blocking.clone_from(&shown.blocking);
    permanent.blocking_this_combat = shown.blocking_this_combat;
    permanent.attacking_band = shown.attacking_band;
    permanent.activated_loyalty_this_turn = shown.activated_loyalty_this_turn;
    permanent.detained_until_turn_of = state
        .detained_until_turn_of
        .map(|(player, turns)| player_from_index(player).map(|player| (player, turns)))
        .transpose()?;
    permanent.destroy_at_end_of_combat = state.destroy_at_end_of_combat;
    permanent.skipped_untap_steps = state.skipped_untap_steps;
    permanent.control_reverts_to = state
        .control_reverts_to
        .map(player_from_index)
        .transpose()?;
    permanent.control_source = state.control_source.map(GameObjectId);
    permanent.control_requires_source_tapped = state.control_requires_source_tapped;
    permanent.reconfigured_timestamp = state
        .reconfigured_timestamp
        .map(super::super::ContinuousEffectTimestamp);
    permanent.chosen_player = state.chosen_player.map(player_from_index).transpose()?;
    permanent.cast_x = state.cast_x;
    permanent.cast_from_zone = state
        .cast_from_zone
        .as_deref()
        .and_then(cast_source_zone_from_label);
    permanent.cast_alternative = state
        .cast_alternative
        .as_deref()
        .and_then(crate::card::AlternativeCastKindDef::from_label);
    permanent.chosen_creature_type = shown.chosen_creature_type;
    permanent.chosen_basic_land_type = shown.chosen_basic_land_type;
    permanent.chosen_card_name = shown.chosen_card_name;
    permanent.face_down = state.face_down.map(face_down_characteristics_from_snapshot);
    permanent.turn_up_for_mana_cost = state.turn_up_for_mana_cost;
    permanent.temporary_keywords = state
        .temporary_keywords
        .iter()
        .copied()
        .map(parse_keyword)
        .collect();
    permanent.keywords_until_upkeep_of = state
        .keywords_until_upkeep_of
        .iter()
        .map(|entry| Ok((player_from_index(entry.seat)?, parse_keyword(entry.keyword))))
        .collect::<Result<Vec<_>, String>>()?;
    permanent.resolved_continuous_effects = state
        .resolved_continuous_effects
        .iter()
        .map(|effect| parse_resolved_continuous_effect(effect, catalog))
        .collect::<Result<Vec<_>, String>>()?;
    permanent.activations_this_turn = state
        .activations_this_turn
        .iter()
        .map(|activation| {
            (
                ability_origin_from_snapshot(activation.origin),
                activation.count,
            )
        })
        .collect();
    permanent.triggers_this_turn = state
        .triggers_this_turn
        .iter()
        .map(|triggered| {
            (
                ability_origin_from_snapshot(triggered.origin),
                triggered.count,
            )
        })
        .collect();
    permanent.resolutions_this_turn = state
        .resolutions_this_turn
        .iter()
        .map(|resolved| {
            (
                ability_origin_from_snapshot(resolved.origin),
                resolved.count,
            )
        })
        .collect();
    permanent.cast_at_instant_speed = state.cast_at_instant_speed;
    permanent.became_aura = state.became_aura;
    permanent.copy_effect = copy_effect;
    permanent.copy_expiration = state.copy_expiration.map(parse_expiration).transpose()?;
    permanent.copied_from = state
        .copied_from
        .as_ref()
        .map(|copy| {
            object_characteristics_from_snapshot(catalog, &copy.characteristics).ok_or_else(|| {
                "checkpoint copied-from characteristics are absent from this catalog".to_owned()
            })
        })
        .transpose()?;
    permanent.text_changes = state
        .text_changes
        .iter()
        .map(|change| BasicLandTypeChange {
            from: parse_basic_land_type(change.from),
            to: parse_basic_land_type(change.to),
        })
        .collect();
    permanent.destroy_at_end = state.destroy_at_end;
    permanent.counters = counters;
    permanent.attached_to = state.attached_to.map(GameObjectId);
    permanent.exile_instead_of_dying = state.exile_instead_of_dying;
    permanent.regeneration_shields = state.regeneration_shields;
    permanent.attacked_this_turn = state.attacked_this_turn;
    permanent.exerted = state.exerted;
    permanent.saddled = state.saddled;
    permanent.last_attacked_turn = state
        .last_attacked_turn
        .map(|(player, turns)| player_from_index(player).map(|player| (player, turns)))
        .transpose()?;
    permanent.attacks_this_turn = state.attacks_this_turn;
    permanent.damage_sources = ids(&state.damage_sources);
    permanent.was_dealt_damage_this_turn = state.was_dealt_damage_this_turn;
    permanent.dealt_damage_this_turn = state.dealt_damage_this_turn;
    permanent.paired_with = state.paired_with.map(GameObjectId);
    permanent.dealt_damage_to_opponent_this_turn = state.dealt_damage_to_opponent_this_turn;
    permanent.deathtouch_damage = state.deathtouch_damage;
    permanent.created_by = state.created_by.map(GameObjectId);
    permanent.combat_damage_assignment = state
        .combat_damage_assignment
        .iter()
        .map(|assignment| crate::CombatDamageAssignment {
            recipient: parse_snapshot_target(assignment.recipient),
            amount: assignment.amount,
        })
        .collect();
    Ok(permanent)
}

pub(super) fn parse_retired_objects(
    snapshots: &[RetiredObjectSnapshot],
    game: &Game,
) -> Result<BTreeMap<GameObjectId, RetiredObject>, String> {
    snapshots
        .iter()
        .map(|snapshot| match snapshot {
            RetiredObjectSnapshot::Card { card: snapshot } => {
                let parsed = card(
                    GameObjectId(snapshot.object_id),
                    snapshot.definition,
                    player_from_index(snapshot.owner)?,
                    &game.catalog,
                )?;
                Ok((parsed.id, RetiredObject::Card(parsed)))
            }
            RetiredObjectSnapshot::Stack { object } => {
                let parsed = parse_detached_stack(object, game)?;
                Ok((parsed.id, RetiredObject::Stack(Box::new(parsed))))
            }
            RetiredObjectSnapshot::Permanent {
                permanent,
                power,
                toughness,
                mana_value,
                keywords,
            } => {
                let parsed = parse_detached_permanent(permanent, &game.catalog)?;
                Ok((
                    parsed.card.id,
                    RetiredObject::Permanent {
                        permanent: Box::new(parsed),
                        power: *power,
                        toughness: *toughness,
                        mana_value: *mana_value,
                        keywords: keywords.iter().copied().map(parse_keyword).collect(),
                    },
                ))
            }
        })
        .collect()
}

pub(super) fn parse_pending_events(
    snapshots: &[PendingEventSnapshot],
    catalog: &CardCatalog,
) -> Result<VecDeque<PendingEvent>, String> {
    snapshots
        .iter()
        .map(|snapshot| {
            Ok(PendingEvent {
                event: ReplaceableEvent::BattlefieldEntry(PendingBattlefieldEntry {
                    permanent: parse_detached_permanent(&snapshot.entry.permanent, catalog)?,
                    from: parse_zone_kind(snapshot.entry.from),
                    completion: parse_completion(snapshot.entry.completion)?,
                    redirected_to: None,
                }),
                applied: snapshot
                    .applied
                    .iter()
                    .copied()
                    .map(|source| AbilitySourceRef {
                        object: GameObjectId(source.object),
                        ability: ability_origin_from_snapshot(source.ability),
                    })
                    .collect(),
                effects: snapshot
                    .effects
                    .iter()
                    .map(|effect| {
                        let context = parse_replacement_context_snapshot(effect.context)?;
                        if !replacement_effect_locator_matches_source(
                            &effect.effect,
                            context.source,
                        ) {
                            return Err(
                                "pending entry replacement locator disagrees with its source"
                                    .into(),
                            );
                        }
                        Ok(PendingReplacementEffect {
                            context,
                            effect: catalog_entry_replacement_effect(catalog, &effect.effect)?,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            })
        })
        .collect()
}

pub(super) const fn parse_zone_kind(zone: ZoneKindSnapshot) -> ZoneKind {
    match zone {
        ZoneKindSnapshot::Library => ZoneKind::Library,
        ZoneKindSnapshot::Hand => ZoneKind::Hand,
        ZoneKindSnapshot::Battlefield => ZoneKind::Battlefield,
        ZoneKindSnapshot::Graveyard => ZoneKind::Graveyard,
        ZoneKindSnapshot::Stack => ZoneKind::Stack,
        ZoneKindSnapshot::Exile => ZoneKind::Exile,
        ZoneKindSnapshot::Command => ZoneKind::Command,
    }
}

pub(super) fn parse_completion(
    completion: EntryCompletionSnapshot,
) -> Result<EntryCompletion, String> {
    match completion {
        EntryCompletionSnapshot::LandPlayed { seat } => Ok(EntryCompletion::LandPlayed {
            player: player_from_index(seat)?,
        }),
        EntryCompletionSnapshot::SpellResolved { card, definition } => {
            Ok(EntryCompletion::SpellResolved {
                card: GameObjectId(card),
                definition,
            })
        }
        EntryCompletionSnapshot::AttachSource { source } => Ok(EntryCompletion::AttachSource {
            source: GameObjectId(source),
        }),
        EntryCompletionSnapshot::AttachToHost { host } => Ok(EntryCompletion::AttachToHost {
            host: GameObjectId(host),
        }),
        EntryCompletionSnapshot::Attacking { defender } => Ok(EntryCompletion::Attacking {
            defender: match defender {
                AttackDefenderSnapshot::Player { seat } => {
                    AttackDefender::Player(player_from_index(seat)?)
                }
                AttackDefenderSnapshot::Planeswalker { object_id } => {
                    AttackDefender::Planeswalker(GameObjectId(object_id))
                }
            },
        }),
        EntryCompletionSnapshot::Setup => Ok(EntryCompletion::Setup),
        EntryCompletionSnapshot::None => Ok(EntryCompletion::None),
    }
}

pub(super) fn parse_detached_permanent(
    snapshot: &DetachedPermanentSnapshot,
    catalog: &CardCatalog,
) -> Result<Permanent, String> {
    let attack_defender = snapshot
        .attack_defender
        .map(|defender| match defender {
            AttackDefenderSnapshot::Player { seat } => {
                player_from_index(seat).map(AttackDefender::Player)
            }
            AttackDefenderSnapshot::Planeswalker { object_id } => {
                Ok(AttackDefender::Planeswalker(GameObjectId(object_id)))
            }
        })
        .transpose()?;
    parse_permanent(
        &snapshot.state,
        PermanentPresentation {
            controller: player_from_index(snapshot.controller)?,
            tapped: snapshot.tapped,
            damage: snapshot.damage,
            attacking: snapshot.attacking,
            attack_defender,
            blocked: snapshot.blocked,
            blocking: snapshot
                .blocking
                .iter()
                .copied()
                .map(GameObjectId)
                .collect(),
            blocking_this_combat: snapshot.blocking_this_combat.unwrap_or(false),
            attacking_band: snapshot.attacking_band,
            activated_loyalty_this_turn: snapshot.activated_loyalty_this_turn,
            chosen_creature_type: snapshot.chosen_creature_type.clone(),
            chosen_basic_land_type: snapshot.chosen_basic_land_type.map(parse_basic_land_type),
            chosen_card_name: snapshot.chosen_card_name.clone(),
        },
        catalog,
    )
}

pub(super) fn player_from_index(index: usize) -> Result<PlayerId, String> {
    match index {
        0 => Ok(PlayerId::One),
        1 => Ok(PlayerId::Two),
        _ => Err("seat index must be 0 or 1".into()),
    }
}

pub(super) fn parse_attack_defender(value: &Value) -> Result<AttackDefender, String> {
    match str_field(value, "type")? {
        "player" => Ok(AttackDefender::Player(seat_value(field(value, "seat")?)?)),
        "planeswalker" => Ok(AttackDefender::Planeswalker(GameObjectId(u32_field(
            value, "objectId",
        )?))),
        other => Err(format!("unknown attack defender type {other}")),
    }
}

include!("wire_continuous.rs");
include!("wire_cast.rs");
include!("wire_copy.rs");
