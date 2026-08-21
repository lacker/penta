use super::{
    AbilityOrigin, CardBehavior, CardDefinitionId, CardType, CardTypeSet, GameObjectId,
    HandcraftedPolicy, PlayerId, PlayerObservation, StackObjectKind, StackObservation, Target,
};

impl HandcraftedPolicy {
    pub(super) fn hand_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .hand
            .iter()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
    }

    pub(super) fn graveyard_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .graveyards
            .iter()
            .flatten()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
    }

    pub(super) fn permanent_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == id)
            .and_then(|permanent| permanent.characteristics.card_definition())
    }

    pub(super) fn permanent_characteristics(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<super::ObjectCharacteristics> {
        observation
            .battlefield
            .iter()
            .find_map(|permanent| (permanent.id == id).then_some(permanent.characteristics))
    }

    /// Spells whose whole purpose is to remove something the opponent
    /// controls. Pump and recursion also take a permanent target, so they are
    /// deliberately excluded — those want a friendly target.
    pub(super) fn is_already_a_creature(observation: &PlayerObservation, id: GameObjectId) -> bool {
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == id && permanent.power.is_some())
    }

    /// Whether this exact ability is already waiting on the stack from this
    /// source. An animation that has not resolved yet is still going to
    /// happen, so activating a second one buys nothing.
    pub(super) fn already_on_the_stack(
        observation: &PlayerObservation,
        source: GameObjectId,
        ability: AbilityOrigin,
    ) -> bool {
        observation
            .stack
            .iter()
            .any(|object| object.source == Some(source) && object.ability == Some(ability))
    }

    /// Whether an ability's target is attacking or blocking right now, which
    /// is the only time a until-end-of-turn pump changes anything.
    pub(super) fn source_is_attacking(
        observation: &PlayerObservation,
        source: GameObjectId,
    ) -> bool {
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == source && permanent.attacking)
    }

    pub(super) fn target_is_fighting(
        observation: &PlayerObservation,
        target: Option<Target>,
    ) -> bool {
        let Some(Target::Permanent(id)) = target else {
            return false;
        };
        observation.battlefield.iter().any(|permanent| {
            permanent.id == id && (permanent.attacking || permanent.blocking_this_combat)
        })
    }

    pub(super) fn stack_spell_is_already_answered(
        observation: &PlayerObservation,
        spell: GameObjectId,
    ) -> bool {
        observation.stack.iter().any(|counter| {
            counter.controller == observation.viewer
                && counter.targets.contains(&Target::Spell(spell))
        })
    }

    pub(super) fn is_effective_counter_target(
        observation: &PlayerObservation,
        object: &StackObservation,
    ) -> bool {
        object.kind == StackObjectKind::Spell
            && object.controller == observation.viewer.opponent()
            && object.counterable
            && !Self::stack_spell_is_already_answered(observation, object.id)
    }

    pub(super) fn counter_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Spell(id) => observation
                .stack
                .iter()
                .find(|object| object.id == id)
                .map_or(-10_000, |object| {
                    if Self::is_effective_counter_target(observation, object) {
                        2_000
                    } else {
                        -10_000
                    }
                }),
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        1_000
                    }
                }),
            Target::Player(_) | Target::Card(_) => -10_000,
        }
    }

    /// A sweeper is worth casting in proportion to how far behind on board it
    /// leaves you -- which is to say, not at all when you are ahead. Without
    /// this the policy treats a wrath as an ordinary sorcery and fires it into
    /// its own creatures.
    pub(super) fn sweeper_score(observation: &PlayerObservation) -> i32 {
        let count = |controller: PlayerId| {
            i32::try_from(
                observation
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        permanent.controller == controller && permanent.power.is_some()
                    })
                    .count(),
            )
            .unwrap_or(0)
        };
        let swing = count(observation.viewer.opponent()) - count(observation.viewer);
        if swing <= 0 {
            -10_000
        } else {
            6_500 + swing * 500
        }
    }

    /// Scores an activated global destroy by the value of every permanent
    /// type it actually sweeps. Unlike a creature-only wrath, Disk can be
    /// worth firing for artifacts or enchantments, and its own destruction
    /// belongs on the controller's side of the exchange.
    pub(super) fn global_destroy_score(
        &self,
        observation: &PlayerObservation,
        destroyed_types: CardTypeSet,
    ) -> i32 {
        let value = |controller: PlayerId| {
            observation
                .battlefield
                .iter()
                .filter(|permanent| {
                    if permanent.controller != controller {
                        return false;
                    }
                    let types = if permanent.types.is_empty() {
                        match permanent.characteristics {
                            super::ObjectCharacteristics::Card { definition, part } => self
                                .catalog
                                .get(definition)
                                .and_then(|card| card.part(part))
                                .map_or_else(CardTypeSet::empty, |part| part.rules.types()),
                            super::ObjectCharacteristics::Token { token, part } => token
                                .part(part)
                                .map_or_else(CardTypeSet::empty, |part| part.rules.types()),
                            super::ObjectCharacteristics::Emblem { .. } => CardTypeSet::empty(),
                        }
                    } else {
                        permanent.types
                    };
                    types.intersects(destroyed_types)
                })
                .map(|permanent| self.characteristics_value(permanent.characteristics))
                .sum::<i32>()
        };
        let swing = value(observation.viewer.opponent()) - value(observation.viewer);
        if swing <= 0 {
            -10_000
        } else {
            6_500_i32.saturating_add(swing.saturating_mul(10))
        }
    }

    pub(super) fn is_hostile_removal(behavior: Option<CardBehavior>) -> bool {
        matches!(behavior, Some(CardBehavior::DustToDust))
    }

    /// Removal aimed at your own board is never worth its base score, so the
    /// penalty has to cancel that base outright rather than merely rank below
    /// a hostile target that may not exist.
    pub(super) fn removal_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        250 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    }
                }),
            Target::Player(_) | Target::Card(_) | Target::Spell(_) => -10_000,
        }
    }

    pub(super) fn target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => 500,
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        -500
                    } else {
                        250
                    }
                }),
            Target::Card(_) | Target::Spell(_) => 100,
        }
    }

    pub(super) fn damage_target_score(
        observation: &PlayerObservation,
        target: Target,
        amount: u16,
    ) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => {
                if observation.life_totals[player.index()]
                    <= i16::try_from(amount).unwrap_or(i16::MAX)
                {
                    10_000
                } else {
                    -2_000
                }
            }
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        return -10_000;
                    }
                    let remaining = permanent
                        .toughness
                        .unwrap_or(0)
                        .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                    if i16::try_from(amount).unwrap_or(i16::MAX) >= remaining {
                        700 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    } else {
                        100
                    }
                }),
            Target::Card(_) | Target::Spell(_) => -500,
        }
    }

    pub(super) fn card_value(&self, definition: CardDefinitionId) -> i32 {
        if let Some(value) = self.declarative_mana_value(definition) {
            return value;
        }
        match self.behavior(definition) {
            Some(CardBehavior::GoblinGrenade) => 75,
            Some(behavior) if behavior.types().is_creature() => 65,
            Some(_) => 55,
            None => self.catalog.get(definition).map_or(0, |card| {
                if card.rules.has_type(CardType::Creature) {
                    65
                } else {
                    55
                }
            }),
        }
    }

    /// Values either a catalog card or inline token characteristics without
    /// manufacturing a card definition for the latter.
    pub(super) fn characteristics_value(
        &self,
        characteristics: super::ObjectCharacteristics,
    ) -> i32 {
        match characteristics {
            super::ObjectCharacteristics::Card { definition, .. } => self.card_value(definition),
            super::ObjectCharacteristics::Token { token, part } => {
                token.part(part).map_or(0, |part| {
                    if part.rules.has_type(CardType::Creature) {
                        65
                    } else {
                        55
                    }
                })
            }
            super::ObjectCharacteristics::Emblem { .. } => 0,
        }
    }
}
