//! Proliferate (CR 701.28a).
//!
//! Choose any number of permanents and/or players, then give each another
//! counter of each kind already there. It is a procedure of its own rather
//! than a composition: the choice runs over permanents and players at once,
//! which no object set can say, and what each chosen thing gets is read off
//! what is already on it rather than named by the card.

use crate::card::CounterKind;
use crate::ids::{GameObjectId, PlayerId};

use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, StackObject, Target,
};

impl Game {
    /// Everything a proliferate could add to: a permanent carrying at least
    /// one counter, and a player who has at least one.
    pub(super) fn proliferate_candidates(&self) -> Vec<Target> {
        let mut candidates = self
            .battlefield
            .iter()
            .filter(|permanent| !permanent.counters.is_empty())
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect::<Vec<_>>();
        candidates.extend(
            [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| self.player_counter_kinds(*player) > 0)
                .map(Target::Player),
        );
        candidates
    }

    /// How many kinds of counter this player has.
    fn player_counter_kinds(&self, player: PlayerId) -> usize {
        self.players[player.index()].counters.iter().count()
    }

    /// "Choose any number": a minimum of none, and the whole list on offer.
    pub(super) fn offer_proliferate(&mut self, object: &StackObject) {
        let candidates = self.proliferate_candidates();
        if candidates.is_empty() {
            return;
        }
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, target)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: self.proliferate_label(*target),
                card: match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                        .map(|permanent| (*id, Self::effective_rules_source(permanent))),
                    _ => None,
                },
                members: Vec::new(),
                ability_text: None,
                zone: match target {
                    Target::Permanent(_) => DecisionZone::Battlefield,
                    _ => DecisionZone::None,
                },
            })
            .collect::<Vec<_>>();
        let maximum = options.len();
        self.queue_decision(
            object.controller,
            "Choose any number of permanents and/or players to proliferate",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            0..=maximum,
            false,
            options,
            DecisionContinuation::Proliferate { candidates },
        );
    }

    fn proliferate_label(&self, target: Target) -> String {
        match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .and_then(|permanent| {
                    self.presentation_name(Self::effective_rules_source(permanent))
                })
                .map_or_else(|| "That permanent".to_owned(), std::borrow::Cow::into_owned),
            Target::Player(player) => {
                if player == PlayerId::One {
                    "Player one".to_owned()
                } else {
                    "Player two".to_owned()
                }
            }
            Target::Card(_) | Target::Spell(_) => "That object".to_owned(),
        }
    }

    /// "Another counter of each kind already there." Read once per chosen
    /// thing, so a permanent with three kinds on it gets one more of each
    /// and a permanent with none is not on the menu at all.
    pub(super) fn proliferate(&mut self, chosen: &[Target]) {
        for target in chosen {
            match target {
                Target::Permanent(id) => {
                    let kinds = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *id)
                        .map(|permanent| {
                            permanent
                                .counters
                                .iter()
                                .map(|(kind, _)| kind)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    for kind in kinds {
                        self.add_counters_to_permanent(*id, kind, 1);
                    }
                }
                Target::Player(player) => {
                    let kinds = self.players[player.index()]
                        .counters
                        .iter()
                        .map(|(kind, _)| kind)
                        .collect::<Vec<_>>();
                    for kind in kinds {
                        self.players[player.index()].counters.add(kind, 1);
                    }
                }
                Target::Card(_) | Target::Spell(_) => {}
            }
        }
    }

    pub(super) fn add_counters_to_permanent(
        &mut self,
        id: GameObjectId,
        kind: CounterKind,
        amount: u16,
    ) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
        {
            permanent.add_counters(kind, amount);
        }
        self.capture_counters_placed(&[id], kind, amount);
    }
}
