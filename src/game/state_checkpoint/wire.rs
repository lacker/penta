#![allow(clippy::wildcard_imports)]

use super::*;

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
    array(value)?
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| u16::try_from(n).ok())
                .map(CardDefinitionId)
                .ok_or_else(|| "card definitions must be u16 integers".into())
        })
        .collect()
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
        return Err(format!("unknown card definition {}", definition.0));
    }
    Ok(CardInstance {
        id,
        definition,
        owner,
        backing: ObjectBacking::None,
        characteristics: CharacteristicSource::Card(definition),
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
            let definition = CardDefinitionId(
                u16::try_from(usize_field(value, "definition")?)
                    .map_err(|_| "definition is too large")?,
            );
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
    let Some(value) = observation.get("poison") else {
        return Ok([0, 0]);
    };
    let values = array(value)?;
    if values.len() != 2 {
        return Err("poison must contain p1 and p2 values".into());
    }
    let mut counters = [0_u16; 2];
    for (slot, value) in counters.iter_mut().zip(values) {
        *slot = value
            .as_u64()
            .and_then(|counters| u16::try_from(counters).ok())
            .ok_or("poison counters must be unsigned integers")?;
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
                crate::card::ManaSelectionDef::Choice(colors) => !colors.contains(&color),
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

pub(super) fn parse_miracle_window(
    checkpoint: &GameSnapshot,
    hidden: &Value,
    viewer: PlayerId,
    hands: &[Vec<CardInstance>; 2],
) -> Result<Option<GameObjectId>, String> {
    if let Some(object) = checkpoint.miracle_window {
        return Ok(Some(GameObjectId(object)));
    }
    let Some(window) = hidden.get("miracleWindow").filter(|value| !value.is_null()) else {
        return Ok(None);
    };
    let player = seat_value(field(window, "seat")?)?;
    if player != viewer.opponent() {
        return Err("hidden miracleWindow must belong to the opposing seat".into());
    }
    let index = usize_field(window, "handIndex")?;
    hands[player.index()]
        .get(index)
        .map(|card| Some(card.id))
        .ok_or_else(|| format!("hidden miracle hand index {index} is out of range"))
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
                CardDefinitionId(
                    u16::try_from(usize_field(card, "definition")?)
                        .map_err(|_| "last-seen definition is too large")?,
                ),
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

pub(super) fn parse_battlefield(
    observation: &Value,
    snapshots: &[PermanentSnapshot],
    catalog: &CardCatalog,
) -> Result<Vec<Permanent>, String> {
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
                    definition: CardDefinitionId(
                        u16::try_from(usize_field(shown, "definition")?)
                            .map_err(|_| "definition too large")?,
                    ),
                    presented: CardPartId(
                        u8::try_from(usize_field(shown, "presentedPartId")?)
                            .map_err(|_| "part id too large")?,
                    ),
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
                    blocking: optional_id(shown.get("blocking")),
                    activated_loyalty_this_turn: bool_field(shown, "loyaltyAbilityUsedThisTurn")?,
                    chosen_creature_type: shown
                        .get("chosenCreatureType")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                    chosen_card_name: shown
                        .get("chosenCardName")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                },
                catalog,
            )
        })
        .collect()
}

#[allow(clippy::struct_excessive_bools)]
struct PermanentPresentation {
    definition: CardDefinitionId,
    presented: CardPartId,
    controller: PlayerId,
    tapped: bool,
    damage: u16,
    attacking: bool,
    attack_defender: Option<AttackDefender>,
    blocked: bool,
    blocking: Option<GameObjectId>,
    activated_loyalty_this_turn: bool,
    chosen_creature_type: Option<String>,
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
    let mut permanent = Permanent::entering(
        card(
            GameObjectId(state.object_id),
            shown.definition,
            owner,
            catalog,
        )?,
        shown.presented,
        shown.controller,
        state.entered_controller_turn,
    );
    permanent.timestamp = ContinuousEffectTimestamp(state.timestamp);
    permanent.tapped = shown.tapped;
    permanent.damage = shown.damage;
    permanent.attacking = shown.attacking;
    permanent.attack_defender = shown.attack_defender;
    permanent.blocked = shown.blocked;
    permanent.blocking = shown.blocking;
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
    permanent.chosen_creature_type = shown.chosen_creature_type;
    permanent.chosen_card_name = shown.chosen_card_name;
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
    permanent.copy_effect = state
        .copy_effect
        .as_ref()
        .map(|copy| parse_copiable_characteristics(copy, catalog))
        .transpose()?;
    permanent.copied_from = state
        .copied_from
        .map(|copy| {
            let definition = CardDefinitionId(copy.definition);
            let part = CardPartId(copy.part_id);
            catalog
                .get(definition)
                .and_then(|card| card.part(part))
                .ok_or("checkpoint copied-from card part is absent from this catalog")?;
            Ok::<_, String>((definition, part))
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
    permanent.attacks_this_turn = state.attacks_this_turn;
    permanent.damage_sources = ids(&state.damage_sources);
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

fn parse_resolved_continuous_effect(
    state: &ResolvedContinuousEffectSnapshot,
    catalog: &CardCatalog,
) -> Result<ResolvedContinuousEffect, String> {
    let source = AbilitySourceRef {
        object: GameObjectId(state.source.object),
        ability: ability_origin_from_snapshot(state.source.ability),
    };
    if !super::semantics::applied_effect_locator_matches_source(&state.definition, source) {
        return Err(
            "checkpoint resolved-effect locator disagrees with its source ability".to_owned(),
        );
    }
    let definition = catalog_applied_effect(catalog, &state.definition)
        .ok_or("checkpoint resolved-effect locator is absent from this catalog")?;
    Ok(ResolvedContinuousEffect {
        definition,
        source,
        timestamp: ContinuousEffectTimestamp(state.timestamp),
        component_order: state.component_order,
        expiration: parse_expiration(state.expiration)?,
        kind: parse_resolved_operation(definition, &state.operation)?,
    })
}

fn parse_resolved_operation(
    definition: AppliedEffectDef,
    state: &ResolvedContinuousOperationSnapshot,
) -> Result<ResolvedContinuousEffectKind, String> {
    let mismatch = || {
        Err("checkpoint resolved-effect operation does not match its authored locator".to_owned())
    };
    match (definition, state) {
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )),
            ResolvedContinuousOperationSnapshot::AbilityAdd { grant_id },
        ) => Ok(ResolvedContinuousEffectKind::Abilities(
            ResolvedAbilityOperation::Add {
                ability: *ability,
                grant: GrantId(*grant_id),
            },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Remove(predicate),
            )),
            ResolvedContinuousOperationSnapshot::AbilityRemove,
        ) => Ok(ResolvedContinuousEffectKind::Abilities(
            ResolvedAbilityOperation::Remove(predicate),
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(value)),
            ResolvedContinuousOperationSnapshot::BasicLandTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::BasicLandTypes(
            parse_set_operation(value, *operation)?,
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(value)),
            ResolvedContinuousOperationSnapshot::CardTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::CardTypes(
            parse_set_operation(value, *operation)?,
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(value)),
            ResolvedContinuousOperationSnapshot::Colors { operation },
        ) => Ok(ResolvedContinuousEffectKind::Colors(parse_set_operation(
            value, *operation,
        )?)),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(value)),
            ResolvedContinuousOperationSnapshot::CreatureTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::CreatureTypes(
            parse_set_operation(value, *operation)?,
        )),
        (AppliedEffectDef::Rule(rule), ResolvedContinuousOperationSnapshot::Rule) => {
            Ok(ResolvedContinuousEffectKind::Rule(rule))
        }
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::SetBase { .. },
            )),
            ResolvedContinuousOperationSnapshot::SetBasePowerToughness { power, toughness },
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::SetBase {
                power: *power,
                toughness: *toughness,
            },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Modify { .. },
            )),
            ResolvedContinuousOperationSnapshot::ModifyPowerToughness { power, toughness },
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Modify {
                power: *power,
                toughness: *toughness,
            },
        )),
        _ => mismatch(),
    }
}

fn parse_set_operation<T: Copy>(
    definition: SetOperationDef<T>,
    state: SetOperationSnapshot,
) -> Result<SetOperationDef<T>, String> {
    match (definition, state) {
        (SetOperationDef::Add(value), SetOperationSnapshot::Add) => Ok(SetOperationDef::Add(value)),
        (SetOperationDef::Remove(value), SetOperationSnapshot::Remove) => {
            Ok(SetOperationDef::Remove(value))
        }
        (SetOperationDef::Set(value), SetOperationSnapshot::Set) => Ok(SetOperationDef::Set(value)),
        _ => Err("checkpoint set operation does not match its authored locator".into()),
    }
}

pub(super) fn parse_copiable_characteristics(
    snapshot: &CopiableCharacteristicsSnapshot,
    catalog: &CardCatalog,
) -> Result<CopiableCharacteristics, String> {
    let definition = CardDefinitionId(snapshot.definition);
    let part = CardPartId(snapshot.part_id);
    catalog
        .get(definition)
        .and_then(|card| card.part(part))
        .ok_or("checkpoint copy-effect card part is absent from this catalog")?;
    let mut added_types = CardTypeSet::empty();
    for (card_type, present) in CardType::ALL.into_iter().zip(snapshot.added_types) {
        if present {
            added_types = added_types.with(card_type);
        }
    }
    Ok(CopiableCharacteristics {
        base: (definition, part),
        added_types,
        added_abilities: snapshot
            .added_abilities
            .iter()
            .map(|ability| {
                Ok(CopiableAbility {
                    origin: ability_origin_from_snapshot(ability.origin),
                    definition: catalog_ability(catalog, &ability.ability).ok_or_else(|| {
                        "checkpoint copied ability locator is absent from this catalog".to_owned()
                    })?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
    })
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
                    CardDefinitionId(snapshot.definition),
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
                definition: CardDefinitionId(definition),
            })
        }
        EntryCompletionSnapshot::AttachSource { source } => Ok(EntryCompletion::AttachSource {
            source: GameObjectId(source),
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
            definition: CardDefinitionId(snapshot.definition),
            presented: CardPartId(snapshot.presented_part_id),
            controller: player_from_index(snapshot.controller)?,
            tapped: snapshot.tapped,
            damage: snapshot.damage,
            attacking: snapshot.attacking,
            attack_defender,
            blocked: snapshot.blocked,
            blocking: snapshot.blocking.map(GameObjectId),
            activated_loyalty_this_turn: snapshot.activated_loyalty_this_turn,
            chosen_creature_type: snapshot.chosen_creature_type.clone(),
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

include!("wire_cast.rs");
