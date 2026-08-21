//! Making tokens: the printed clauses that put a permanent onto the
//! battlefield out of nothing.
//!
//! Split out of the parent module for the source-size budget. What belongs
//! here is the three clauses that mint one: a token the card names, a token
//! the resolving permanent then wears, and a copy of something already on
//! the battlefield.

use super::super::{EffectResolutionContext, Game, ScopedEffect, StackObject, Target};
use crate::card::EffectDef;

impl Game {
    pub(super) fn resolve_token_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::CreateToken {
                token,
                controller,
                count,
                tapped,
                attacking,
                counters,
                created,
            } => {
                // "Its controller creates two Map tokens": the tokens are
                // that player's, and everything else about them -- including
                // who an arriving attacker attacks -- follows from that.
                let controller = controller
                    .and_then(|player| {
                        self.effect_player_reference(player, object, context, scoped)
                    })
                    .unwrap_or(object.controller);
                // Two players, so the one opponent is the only thing an
                // arriving attacker could be attacking (CR 506.3d).
                let defender =
                    attacking.then(|| crate::AttackDefender::Player(controller.opponent()));
                // Worked out once, before any token is made: the number is
                // what the effect found, not what the board looks like part
                // way through creating them.
                let counters = counters.map(|counters| {
                    (
                        counters.kind,
                        u16::try_from(
                            self.effect_value(counters.amount, object, context, scoped)
                                .max(0),
                        )
                        .unwrap_or(u16::MAX),
                    )
                });
                let mut minted = Vec::new();
                for _ in 0..self.effect_value(count, object, context, scoped).max(0) {
                    minted.push(Target::Permanent(self.create_token_arriving(
                        controller, token, None, tapped, defender, counters,
                    )));
                }
                // Bound after every one is made, so a clause naming them
                // names the whole batch rather than the last of them.
                if let Some(created) = created {
                    let mut context = context.clone();
                    context.bind_object_group(created.binding, minted);
                    self.resolve_effect_def(scoped.with_effect(*created.then), object, context);
                }
            }
            EffectDef::CreateAttachedToken { token } => {
                if let Some(source) = object.source {
                    self.create_attached_token(object.controller, token, source);
                }
            }
            EffectDef::CreateTokenCopyOf { object: recipient } => {
                let copies = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(id) => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                            // Freeze the source's complete copiable values. Its
                            // token nature is deliberately not among them: the
                            // newly created object is independently a token.
                            .map(|permanent| {
                                (
                                    Self::copiable_characteristics(permanent),
                                    self.double_faced_copiable_characteristics(permanent),
                                    permanent.presented,
                                )
                            }),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                for (copy, double_faced, presented) in copies {
                    self.create_token_copy(object.controller, copy, double_faced, presented);
                }
            }
            _ => unreachable!("the caller admits only token-making clauses"),
        }
    }

    /// "Put that card onto the battlefield under your control with a
    /// finality counter on it. It gains haste. Sacrifice it at the beginning
    /// of the next end step."
    ///
    /// Everything after the first clause names the permanent that just
    /// entered, which is a new object: the counter, the haste, and the
    /// delayed sacrifice all read the identity the arrival minted rather
    /// than the card that was in exile.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn return_with_haste_and_finality(
        &mut self,
        recipient: crate::card::EffectRecipientDef,
        binding: crate::ObjectSetBindingIndex,
        then: &'static EffectDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let controller = object.controller;
        let mut arrivals = Vec::new();
        for target in self.effect_recipients(recipient, object, &context, scoped) {
            let Some(arrived) = self.move_target_to_zone(
                target,
                crate::card::ZoneKind::Battlefield,
                super::super::ZoneMoveCause::Effect { controller },
                Some(super::super::BattlefieldArrival::under(controller)),
                crate::card::ZonePlacement::Top,
            ) else {
                continue;
            };
            if let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == arrived)
            {
                permanent.add_counters(crate::card::CounterKind::Finality, 1);
                permanent
                    .temporary_keywords
                    .push(crate::card::KeywordAbility::Haste);
            }
            self.capture_counters_placed(&[arrived], crate::card::CounterKind::Finality, 1);
            arrivals.push(Target::Permanent(arrived));
        }
        let mut context = context;
        context.bind_object_group(binding, arrivals);
        self.resolve_effect_def(scoped.with_effect(*then), object, context);
    }
}
