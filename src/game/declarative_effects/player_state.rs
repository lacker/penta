//! Declarative effects that directly change a player's tracked state or the
//! game's result.

use super::super::{
    EffectDef, EffectResolutionContext, Game, GameResult, ManaPool, ScopedEffect, StackObject,
    Target, WinReason,
};

impl Game {
    pub(super) fn resolve_player_state_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::GainLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.gain_life(player, amount);
                    }
                }
            }
            EffectDef::LoseLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.lose_life(player, amount);
                    }
                }
            }
            EffectDef::AddPlayerCounters {
                recipient,
                kind,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.add_player_counters(player, kind, amount);
                    }
                }
            }
            EffectDef::EmptyManaPool { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.players[player.index()].mana_pool = ManaPool::default();
                        self.players[player.index()].mana.clear();
                    }
                }
            }
            EffectDef::WinTheGame { player: recipient } => {
                let mut winners = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                winners.sort_unstable();
                winners.dedup();
                // Both players winning at once is a draw, the same way both
                // losing at once is. Only one card in the pool ends a game
                // this way, and it names its own controller.
                if let [winner] = winners.as_slice() {
                    self.finish(GameResult::Winner {
                        winner: *winner,
                        reason: WinReason::WonByAnEffect,
                    });
                }
            }
            EffectDef::LoseTheGame { player: recipient } => {
                let mut losers = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                losers.sort_unstable();
                losers.dedup();
                match losers.as_slice() {
                    [loser] => self.finish(GameResult::Winner {
                        winner: loser.opponent(),
                        reason: WinReason::OpponentLostToAnEffect,
                    }),
                    [_, _] => self.finish(GameResult::Draw),
                    [] => {}
                    _ => unreachable!("a two-player game has at most two losers"),
                }
            }
            _ => unreachable!("player-state dispatcher received an unrelated effect"),
        }
    }
}
