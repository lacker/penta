//! Commander designation belongs to physical cards (CR 903.3), never copied
//! characteristics. Each designated card owns its own tax and damage history.
use super::{
    CardDefinitionId, CastSourceZone, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone, Game, GameObjectId, ObjectBacking, PhysicalCardId, PlayerId,
    RetiredObject, ZoneCard, ZoneKind, ZoneMoveCause, ZonePlacement,
};

mod returns;
pub(super) use returns::CommanderMove;

#[derive(Clone, Debug)]
pub(super) struct CommanderState {
    pub physical: PhysicalCardId,
    pub definition: CardDefinitionId,
    pub owner: PlayerId,
    pub casts: u16,
    pub damage: [u16; 2],
    /// One graveyard/exile incarnation is considered only at its first SBA.
    pub considered: Option<GameObjectId>,
}

impl CommanderState {
    pub(super) const fn new(
        physical: PhysicalCardId,
        definition: CardDefinitionId,
        owner: PlayerId,
    ) -> Self {
        Self {
            physical,
            definition,
            owner,
            casts: 0,
            damage: [0; 2],
            considered: None,
        }
    }
}

/// Public commander identity and accumulated history. `object` is absent in
/// hidden zones; its absence does not disclose a position or track a shuffle.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommanderObservation {
    #[serde(with = "wire_owner")]
    pub owner: PlayerId,
    pub definition: CardDefinitionId,
    pub command_zone_casts: u16,
    pub combat_damage: [u16; 2],
    #[serde(rename = "objectId", with = "wire_object")]
    pub object: Option<GameObjectId>,
}

impl Game {
    pub(super) fn object_backing(&self, object: GameObjectId) -> Option<&ObjectBacking> {
        self.card_in_nonbattlefield_zone(object)
            .map(|(_, card)| &card.backing)
            .or_else(|| {
                self.battlefield
                    .iter()
                    .chain(&self.phased_out)
                    .find(|p| p.card.id == object)
                    .map(|p| &p.card.backing)
            })
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|s| {
                        s.id == object && !s.is_copy && s.kind == super::StackObjectKind::Spell
                    })
                    .map(|s| &s.card.backing)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Card(card)) => Some(&card.backing),
                Some(RetiredObject::Permanent { permanent, .. }) => Some(&permanent.card.backing),
                Some(RetiredObject::Stack(stack))
                    if !stack.is_copy && stack.kind == super::StackObjectKind::Spell =>
                {
                    Some(&stack.card.backing)
                }
                _ => None,
            })
    }

    pub(super) fn commander_index(&self, object: GameObjectId) -> Option<usize> {
        if self.commanders.is_empty() {
            return None;
        }
        let ObjectBacking::Cards(cards) = self.object_backing(object)? else {
            return None;
        };
        self.commanders
            .iter()
            .position(|commander| cards.contains(&commander.physical))
    }

    /// Whether this object is backed by a designated commander. A copy of a
    /// commander is not designated; a commander copying something else is.
    #[must_use]
    pub fn is_commander(&self, object: GameObjectId) -> bool {
        self.commander_index(object).is_some()
    }

    pub(super) fn can_cast_commander_from_command_zone(&self, object: GameObjectId) -> bool {
        let Some(index) = self.commander_index(object) else {
            return false;
        };
        let commander = &self.commanders[index];
        !self
            .format
            .commander_definition()
            .is_some_and(|rules| rules.one_command_zone_commander)
            || commander.casts > 0
            || !self
                .commanders
                .iter()
                .any(|other| other.owner == commander.owner && other.casts > 0)
    }

    /// Face-up cards currently in this player's command zone.
    #[must_use]
    pub fn command_zone(&self, player: PlayerId) -> Vec<ZoneCard> {
        super::zones::zone_cards(&self.players[player.index()].command)
    }

    pub(super) fn commander_tax(&self, object: GameObjectId) -> u16 {
        self.commander_index(object)
            .map_or(0, |index| self.commanders[index].casts.saturating_mul(2))
    }

    pub(super) fn record_commander_cast(&mut self, spell: GameObjectId) {
        if self.commanders.is_empty() {
            return;
        }
        if !self.stack.iter().any(|object| {
            object.id == spell
                && object
                    .cast
                    .as_ref()
                    .is_some_and(|cast| cast.source_zone == Some(CastSourceZone::Command))
        }) {
            return;
        }
        if let Some(index) = self.commander_index(spell) {
            self.commanders[index].casts = self.commanders[index].casts.saturating_add(1);
        }
    }

    pub(super) fn record_commander_damage(
        &mut self,
        source: Option<GameObjectId>,
        player: PlayerId,
        amount: u16,
    ) {
        if let Some(index) = source.and_then(|source| self.commander_index(source)) {
            self.commanders[index].damage[player.index()] =
                self.commanders[index].damage[player.index()].saturating_add(amount);
        }
    }

    #[must_use]
    pub fn commanders(&self, viewer: PlayerId) -> Vec<CommanderObservation> {
        self.commanders.iter().map(|commander| {
            let backed = |backing: &ObjectBacking| matches!(backing, ObjectBacking::Cards(cards) if cards.contains(&commander.physical));
            let object = self.battlefield.iter().chain(&self.phased_out).find(|p| backed(&p.card.backing)).map(|p| p.card.id)
                .or_else(|| self.stack.iter().find(|s| !s.is_copy && s.kind == super::StackObjectKind::Spell && backed(&s.card.backing)).map(|s| s.id))
                .or_else(|| self.players.iter().flat_map(|p| p.command.iter().chain(&p.graveyard)).find(|c| backed(&c.backing)).map(|c| c.id))
                .or_else(|| self.players.iter().flat_map(|p| &p.exile).find(|c| backed(&c.backing) && self.observed_exile(c.owner, viewer).iter().any(|(id, _)| *id == c.id)).map(|c| c.id))
                .or_else(|| self.players[viewer.index()].hand.iter().find(|c| backed(&c.backing)).map(|c| c.id));
            CommanderObservation { owner: commander.owner, definition: commander.definition, command_zone_casts: commander.casts, combat_damage: commander.damage, object }
        }).collect()
    }

    /// CR 903.9a: make the choices in APNAP order, then move the chosen cards
    /// together. A blink that returns during resolution never offers a choice.
    pub(super) fn check_commander_returns(&mut self) -> bool {
        if self.commanders.is_empty() {
            return false;
        }
        let mut groups = Vec::new();
        for owner in [self.active_player, self.active_player.opponent()] {
            let cards = self.players[owner.index()]
                .graveyard
                .iter()
                .chain(&self.players[owner.index()].exile)
                .filter_map(|card| {
                    self.commander_index(card.id)
                        .filter(|&index| self.commanders[index].considered != Some(card.id))
                        .map(|index| (index, card.id))
                })
                .collect::<Vec<_>>();
            for &(index, object) in &cards {
                self.commanders[index].considered = Some(object);
            }
            if !cards.is_empty() {
                groups.push((owner, cards.into_iter().map(|(_, id)| id).collect()));
            }
        }
        if groups.is_empty() {
            return false;
        }
        self.queue_commander_returns(groups, Vec::new());
        true
    }

    pub(super) fn queue_commander_returns(
        &mut self,
        mut remaining: Vec<(PlayerId, Vec<GameObjectId>)>,
        selected: Vec<GameObjectId>,
    ) {
        if remaining.is_empty() {
            for id in selected {
                self.move_card_target_to_zone(
                    id,
                    ZoneKind::Command,
                    ZoneMoveCause::Rules,
                    None,
                    ZonePlacement::Top,
                );
            }
            return;
        }
        let (owner, cards) = remaining.remove(0);
        let options = cards
            .iter()
            .enumerate()
            .filter_map(|(index, id)| {
                let (zone, card) = self.card_in_nonbattlefield_zone(*id)?;
                Some(DecisionOption {
                    id: u32::try_from(index).ok()?,
                    label: self.catalog.get(card.definition)?.name.clone(),
                    card: Some((
                        *id,
                        super::ObjectCharacteristics::Card {
                            definition: card.definition,
                            part: crate::CardPartId::PRIMARY,
                        },
                    )),
                    members: Vec::new(),
                    ability_text: None,
                    zone: if zone == ZoneKind::Graveyard {
                        DecisionZone::Graveyard
                    } else {
                        DecisionZone::Exile
                    },
                })
            })
            .collect::<Vec<_>>();
        self.queue_decision(
            owner,
            "Choose commanders to return to the command zone",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=options.len(),
            false,
            options,
            DecisionContinuation::CommanderReturn {
                remaining,
                selected,
            },
        );
    }
}

impl Game {
    pub(super) fn restore_commanders(
        &mut self,
        saved: &[CommanderObservation],
        considered: &[Option<u32>],
    ) -> Result<(), String> {
        for (index, commander) in saved.iter().enumerate() {
            let physical = if let Some(object) = commander.object {
                let Some(ObjectBacking::Cards(cards)) = self.object_backing(object) else {
                    return Err("commander checkpoint references no physical card".into());
                };
                *cards
                    .first()
                    .ok_or("commander has empty physical backing")?
            } else {
                // A hidden-zone hypothesis supplies identities, never the real
                // library position. Reject ambiguous duplicate-card hypotheses.
                let candidates = self.players[commander.owner.index()]
                    .library
                    .iter()
                    .chain(&self.players[commander.owner.index()].hand)
                    .chain(&self.players[commander.owner.index()].exile)
                    .filter(|card| card.definition == commander.definition)
                    .flat_map(|card| super::backing_cards(&card.backing))
                    .filter(|physical| !self.commanders.iter().any(|c| c.physical == *physical))
                    .collect::<Vec<_>>();
                let [physical] = candidates.as_slice() else {
                    return Err(
                        "hidden commander hypothesis must identify exactly one physical card"
                            .into(),
                    );
                };
                *physical
            };
            if !self.physical_cards.iter().any(|card| {
                card.id == physical
                    && card.definition == commander.definition
                    && card.owner == commander.owner
            }) || self.commanders.iter().any(|c| c.physical == physical)
            {
                return Err("commander checkpoint disagrees with physical card identity".into());
            }
            self.commanders.push(CommanderState {
                physical,
                definition: commander.definition,
                owner: commander.owner,
                casts: commander.command_zone_casts,
                damage: commander.combat_damage,
                considered: considered.get(index).copied().flatten().map(GameObjectId),
            });
        }
        self.validate_commander_return_checkpoints()
    }

    fn validate_commander_return_checkpoints(&self) -> Result<(), String> {
        for pending in &self.pending_decisions {
            if let DecisionContinuation::CommanderReturn {
                remaining,
                selected,
            } = &pending.continuation
            {
                let mut seen = std::collections::BTreeSet::new();
                let current = pending
                    .observation
                    .options
                    .iter()
                    .map(|option| {
                        option
                            .card
                            .as_ref()
                            .map(|(id, _)| (pending.observation.player, *id))
                            .ok_or("commander return option has no card")
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let later = remaining
                    .iter()
                    .flat_map(|(owner, cards)| cards.iter().map(|id| (*owner, *id)));
                for (owner, id) in current.into_iter().chain(later) {
                    let valid = self.commander_index(id).is_some_and(|index| {
                        self.commanders[index].owner == owner
                            && self.commanders[index].considered == Some(id)
                    });
                    if !valid
                        || !seen.insert(id)
                        || !self
                            .card_in_nonbattlefield_zone(id)
                            .is_some_and(|(zone, _)| {
                                matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile)
                            })
                    {
                        return Err(
                            "commander return checkpoint contains an ineligible card".into()
                        );
                    }
                }
                for id in selected {
                    if !seen.insert(*id)
                        || !self.is_commander(*id)
                        || !self
                            .card_in_nonbattlefield_zone(*id)
                            .is_some_and(|(zone, _)| {
                                matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile)
                            })
                    {
                        return Err(
                            "commander return checkpoint has an invalid selected card".into()
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

mod wire_object {
    use super::GameObjectId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    // Serde calls custom serializers by reference.
    #[allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S: Serializer>(
        object: &Option<GameObjectId>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        object.map(|id| id.0).serialize(serializer)
    }
    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<GameObjectId>, D::Error> {
        Ok(Option::<u32>::deserialize(deserializer)?.map(GameObjectId))
    }
}

mod wire_owner {
    use super::PlayerId;
    use serde::{Deserialize, Deserializer, Serializer};
    // Serde calls custom serializers by reference.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S: Serializer>(
        owner: &PlayerId,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(if *owner == PlayerId::One { "p1" } else { "p2" })
    }
    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<PlayerId, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "p1" => Ok(PlayerId::One),
            "p2" => Ok(PlayerId::Two),
            _ => Err(serde::de::Error::custom("commander owner must be p1 or p2")),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_command_zones(
    designated: &[Vec<CardDefinitionId>; 2],
    players: &mut [super::PlayerState; 2],
    physical_cards: &mut Vec<super::PhysicalCard>,
    next_physical_id: &mut u32,
    next_object_id: &mut u32,
) -> Result<Vec<CommanderState>, super::GameError> {
    use super::{CardInstance, CharacteristicSource, GameError, PhysicalCard};
    let mut commanders = Vec::new();
    for player in [PlayerId::One, PlayerId::Two] {
        for &definition in &designated[player.index()] {
            let physical_id = PhysicalCardId(*next_physical_id);
            *next_physical_id = next_physical_id
                .checked_add(1)
                .ok_or(GameError::TooManyCards)?;
            let id = GameObjectId(*next_object_id);
            *next_object_id = next_object_id
                .checked_add(1)
                .ok_or(GameError::TooManyCards)?;
            physical_cards.push(PhysicalCard {
                id: physical_id,
                definition,
                owner: player,
            });
            players[player.index()].command.push(CardInstance {
                id,
                definition,
                owner: player,
                backing: ObjectBacking::Cards(vec![physical_id]),
                characteristics: CharacteristicSource::Card(definition),
                counters: super::counters::Counters::new(),
            });
            commanders.push(CommanderState::new(physical_id, definition, player));
        }
    }
    Ok(commanders)
}
