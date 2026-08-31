use super::{
    CardBehavior, CardTypeSet, CastChoices, DeclarativeSpellProfile, GameObjectId,
    HandcraftedPolicy, PlayerObservation, Target,
};

impl HandcraftedPolicy {
    /// Fireball should not spend a card on no target or mark damage that cannot
    /// kill a creature. The policy has no same-turn combination planner that
    /// could establish a payoff for letting that creature survive.
    fn fireball_is_wasted(
        observation: &PlayerObservation,
        choices: &CastChoices,
        amount: u16,
    ) -> bool {
        choices.iter_targets().next().is_none()
            || choices.iter_targets().any(|target| {
                let Target::Permanent(id) = target else {
                    return false;
                };
                observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == *id)
                    .is_some_and(|permanent| {
                        let Some(toughness) = permanent.toughness else {
                            return false;
                        };
                        let remaining = toughness
                            .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                        i16::try_from(amount).unwrap_or(i16::MAX) < remaining
                    })
            })
    }

    pub(super) fn cast_target_score(
        observation: &PlayerObservation,
        target: Target,
        cards_drawn: Option<u16>,
        counters: bool,
        removes: bool,
        damage: Option<u16>,
    ) -> i32 {
        if let Some(cards_drawn) = cards_drawn {
            return match target {
                Target::Player(player) if player == observation.viewer => {
                    if usize::from(cards_drawn) > observation.library_sizes[player.index()] {
                        -20_000
                    } else {
                        1_000 + i32::from(cards_drawn) * 100
                    }
                }
                Target::Player(player) => {
                    if usize::from(cards_drawn) > observation.library_sizes[player.index()] {
                        20_000
                    } else {
                        -10_000
                    }
                }
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => -10_000,
            };
        }
        if counters {
            return Self::counter_target_score(observation, target);
        }
        if removes {
            return Self::removal_target_score(observation, target);
        }
        damage.map_or_else(
            || Self::target_score(observation, target),
            |amount| Self::damage_target_score(observation, target, amount),
        )
    }

    /// How many creatures the opponent has on the battlefield, which is the
    /// size of the swing a one-sided sweeper is being scored for.
    fn opposing_creature_count(observation: &PlayerObservation) -> i32 {
        i32::try_from(
            observation
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == observation.viewer.opponent()
                        && permanent.power.is_some()
                })
                .count(),
        )
        .unwrap_or(i32::MAX)
    }

    pub(super) fn score_cast(
        &self,
        observation: &PlayerObservation,
        card: GameObjectId,
        choices: &CastChoices,
    ) -> i32 {
        let definition = Self::hand_definition(observation, card)
            .or_else(|| Self::graveyard_definition(observation, card));
        let behavior = definition.and_then(|id| self.behavior(id));
        let declarative = definition.and_then(|id| self.declarative_spell_profile(id, choices));
        let kind = definition
            .and_then(|id| self.catalog.get(id))
            .map(|card| card.rules.types());
        let x = choices.x();
        let target_count = choices.iter_targets().count();
        // Passing scores 0, so anything below it means hold the card instead.
        if x == 0 && definition.is_some_and(|id| self.is_empty_at_zero_x(id, declarative)) {
            return -10_000;
        }
        let fireball_damage = (behavior == Some(CardBehavior::Fireball)).then(|| {
            x.checked_div(u16::try_from(target_count).unwrap_or(u16::MAX))
                .unwrap_or(0)
        });
        if fireball_damage
            .is_some_and(|amount| Self::fireball_is_wasted(observation, choices, amount))
        {
            return -10_000;
        }
        let damage = match behavior {
            Some(CardBehavior::Fireball) => fireball_damage,
            _ => declarative.and_then(|profile| profile.damage),
        };
        let cards_drawn = declarative.and_then(|profile| profile.cards_drawn);
        let counters =
            declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::COUNTERS));
        let removes =
            declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::REMOVES));
        let sweeps_creatures = declarative
            .is_some_and(|profile| profile.has(DeclarativeSpellProfile::SWEEPS_CREATURES));
        let extra_turn =
            declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::EXTRA_TURN));
        let target_score: i32 = choices
            .iter_targets()
            .map(|target| {
                Self::cast_target_score(
                    observation,
                    *target,
                    cards_drawn,
                    counters,
                    removes,
                    damage,
                )
            })
            .sum();
        let opponent_creatures = Self::opposing_creature_count(observation);
        let opponent_spells = i32::try_from(
            observation
                .stack
                .iter()
                .filter(|object| Self::is_effective_counter_target(observation, object))
                .count(),
        )
        .unwrap_or(i32::MAX);
        let base = match behavior {
            Some(CardBehavior::Fireball) => 7_900 + i32::from(x) * 20,
            Some(behavior) if behavior.types().is_permanent() => 6_800,
            _ if extra_turn => 8_300,
            _ if sweeps_creatures => Self::sweeper_score(observation),
            _ if declarative.is_some_and(|profile| profile.opponent_creature_sweep) => {
                if opponent_creatures == 0 {
                    -10_000
                } else {
                    7_500 + opponent_creatures * 900
                }
            }
            _ if declarative.is_some_and(|profile| profile.opponent_spell_sweep) => {
                if opponent_spells == 0 {
                    -10_000
                } else if opponent_spells == 1 {
                    6_000 + opponent_spells * 500
                } else {
                    8_900 + opponent_spells * 2_000
                }
            }
            _ if declarative.is_some_and(|profile| {
                profile.cards_drawn_by_each_player.is_some_and(|n| n >= 3)
            }) =>
            {
                6_600
            }
            _ if cards_drawn.is_some_and(|amount| amount >= 3) => 9_200,
            _ if counters => 8_900,
            _ if removes => 8_400,
            _ if damage.is_some() => 8_000,
            None if kind.is_some_and(CardTypeSet::is_permanent) => 6_800,
            Some(_) | None => 6_200,
        };
        base + target_score
    }
}
