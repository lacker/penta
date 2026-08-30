//! Making tokens: the printed clauses that put a permanent onto the
//! battlefield out of nothing.
//!
//! Split out of the parent module for the source-size budget. What belongs
//! here is the three clauses that mint one: a token the card names, a token
//! the resolving permanent then wears, and a copy of something already on
//! the battlefield.

use super::super::{
    CopiableAbility, EffectResolutionContext, Game, ScopedEffect, StackObject, Target,
};
use crate::card::{EffectDef, TokenCharacteristics};

/// One "create a token that's a copy of it" instruction, whole.
#[derive(Clone, Copy)]
struct TokenCopyRequest {
    recipient: crate::card::EffectRecipientDef,
    exceptions: crate::card::CopyExceptionsDef,
    controller: Option<crate::card::PlayerRefDef>,
    /// How many copies of each named object to make. One for nearly every
    /// clause that copies a permanent; squad buys as many as it was paid
    /// for.
    count: crate::card::ValueDef,
    created: Option<crate::card::CreatedTokensDef>,
}

impl Game {
    /// "Create a <token> attached to it." One host, because one token is
    /// made: a clause naming several would have to say how many it makes.
    fn resolve_token_attached_to(
        &mut self,
        token: crate::card::TokenCharacteristics,
        recipient: crate::card::EffectRecipientDef,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let Some(Target::Permanent(host)) = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .next()
        else {
            return;
        };
        // A doubled token-creation makes the second one with nothing to
        // attach to, which is what an Aura token arriving unattached already
        // is: it is put into the graveyard by the ordinary Aura rule.
        let mut minted = Vec::new();
        for extra in 1..self.tokens_created(object.controller, 1) {
            let _ = extra;
            minted.push(self.create_token_from(object.controller, token, None));
        }
        minted.push(self.create_token_attached_to(object.controller, token, host));
        self.capture_tokens_created(object.controller, &minted);
    }

    /// "Create a token that's a copy of <something>." The copiable values
    /// are frozen from the source and the "except" clauses ride on them, so
    /// a later copy of this token copies those too.
    #[allow(clippy::too_many_arguments)]
    fn resolve_token_copies(
        &mut self,
        request: TokenCopyRequest,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let TokenCopyRequest {
            recipient,
            exceptions,
            controller,
            count,
            created,
        } = request;
        let count = usize::try_from(self.effect_value(count, object, context, scoped).max(0))
            .unwrap_or(usize::MAX);
        // "Each player other than its controller creates a token": whoever
        // the clause names rather than whoever is resolving it.
        let holder = controller
            .and_then(|reference| self.player_reference(reference, object, context, scoped))
            .unwrap_or(object.controller);
        let copies = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .filter_map(|target| {
                // Freeze the source's complete copiable values. Its
                // token nature is deliberately not among them: the
                // newly created object is independently a token.
                //
                // A card rather than a permanent is what eternalize
                // copies: the card it exiled as its cost, whose
                // copiable values are simply what is printed on it.
                let mut copy = self.copiable_values_of(target)?;
                // The exceptions ride on the copy rather than being
                // applied to the token afterwards: each is itself a
                // copiable value.
                if let Some(stats) = exceptions.base_power_toughness {
                    copy.base_power_toughness = Some(stats);
                }
                if let Some(colors) = exceptions.colors {
                    copy.colors = Some(colors);
                }
                copy.added_creature_types
                    .extend(exceptions.added_creature_types.named);
                // "In addition to its other types", so the copied types stay
                // and these join them.
                copy.added_types = copy.added_types.union(exceptions.added_types);
                copy.no_mana_cost |= exceptions.no_mana_cost;
                // "Except it has haste": part of what the token
                // copies rather than a grant made to it afterwards,
                // and attributed to the clause that said so.
                if let Some(payload) = object.ability.as_ref() {
                    copy.added_abilities
                        .extend(exceptions.added_abilities.iter().filter_map(|added| {
                            Some(CopiableAbility {
                                origin: payload.origin,
                                definition: match added {
                                    crate::card::CopyAbilityDef::This => {
                                        *payload.definition.as_deref()?
                                    }
                                    crate::card::CopyAbilityDef::Ability(ability) => **ability,
                                },
                            })
                        }));
                }
                let permanent = match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == id),
                    _ => None,
                };
                let (double_faced, presented) = permanent.map_or_else(
                    || (None, crate::CardPartId::PRIMARY),
                    |permanent| {
                        (
                            self.double_faced_copiable_characteristics(permanent),
                            permanent.presented,
                        )
                    },
                );
                Some((copy, double_faced, presented))
            })
            .collect::<Vec<_>>();
        // "The tokens will all enter the battlefield simultaneously": one
        // batch, so each of them is seen arriving by all the others.
        let mut minted = Vec::new();
        self.entering_together(|game| {
            for (copy, double_faced, presented) in copies {
                for _ in 0..game.tokens_created(holder, count) {
                    minted.push(Target::Permanent(game.create_token_copy(
                        holder,
                        copy.clone(),
                        double_faced.clone(),
                        presented,
                    )));
                }
            }
        });
        self.capture_created_token_batch(holder, &minted);
        // Bound after every copy is made, so a clause naming them
        // names the whole batch rather than the last of them.
        if let Some(created) = created {
            let mut context = context.clone();
            context.bind_object_group(created.binding, minted);
            self.resolve_effect_def(scoped.with_effect(*created.then), object, context);
        }
    }

    /// The batch a token-creating instruction actually made, as the
    /// creation event the "whenever you create one or more" clauses read.
    fn capture_created_token_batch(&mut self, controller: crate::PlayerId, minted: &[Target]) {
        let ids = minted
            .iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => Some(*id),
                _ => None,
            })
            .collect::<Vec<_>>();
        self.capture_tokens_created(controller, &ids);
    }

    pub(super) fn resolve_token_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::CreateToken {
                token,
                copy,
                controller,
                count,
                tapped,
                attacking,
                counters,
                created,
            } => {
                if let Some(copy) = copy {
                    debug_assert!(!tapped && !attacking && counters.is_none());
                    self.resolve_token_copies(
                        TokenCopyRequest {
                            recipient: *copy.object,
                            exceptions: copy.exceptions,
                            controller,
                            count,
                            created,
                        },
                        scoped,
                        object,
                        context,
                    );
                    return;
                }
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
                // "An X/X blue Illusion": the size is worked out once, here,
                // and the tokens arrive that size rather than growing into it.
                let token = self.resolved_token_stats(token, object, context, scoped);
                let mut minted = Vec::new();
                let count =
                    usize::try_from(self.effect_value(count, object, context, scoped).max(0))
                        .unwrap_or(usize::MAX);
                // "Create four 1/1 Myr": they arrive at the same time, so a
                // clause watching arrivals sees all four rather than each
                // against a board the others have not joined yet.
                self.entering_together(|game| {
                    for _ in 0..game.tokens_created(controller, count) {
                        minted.push(Target::Permanent(game.create_token_arriving(
                            controller, token, None, tapped, defender, counters,
                        )));
                    }
                });
                self.capture_created_token_batch(controller, &minted);
                // "Attacking" without a named defender is the controller's
                // choice as the creature enters (CR 506.3d), which only has
                // an answer worth asking for when the defending player has a
                // planeswalker to be sent at instead.
                if attacking {
                    let arriving = minted
                        .iter()
                        .filter_map(|target| match target {
                            Target::Permanent(id) => Some(*id),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    self.queue_arriving_attacker_defender(
                        controller,
                        controller.opponent(),
                        &arriving,
                    );
                }
                // Bound after every one is made, so a clause naming them
                // names the whole batch rather than the last of them.
                if let Some(created) = created {
                    let mut context = context.clone();
                    context.bind_object_group(created.binding, minted);
                    self.resolve_effect_def(scoped.with_effect(*created.then), object, context);
                }
            }
            // The two directions of one instruction: a named host means the
            // token goes onto it, and no host means the resolving permanent
            // goes onto the token.
            EffectDef::CreateAttachedToken { token, host } => match host {
                Some(recipient) => {
                    self.resolve_token_attached_to(token, recipient, scoped, object, context);
                }
                None => {
                    if let Some(source) = object.source {
                        let mut minted = Vec::new();
                        // A doubled living weapon makes the second Germ with
                        // nothing on it, which is what it would be anyway.
                        for extra in 1..self.tokens_created(object.controller, 1) {
                            let _ = extra;
                            minted.push(self.create_token_from(object.controller, token, None));
                        }
                        minted.push(self.create_attached_token(object.controller, token, source));
                        self.capture_tokens_created(object.controller, &minted);
                    }
                }
            },
            _ => unreachable!("the caller admits only token-making clauses"),
        }
    }

    /// The token an effect is actually creating: an authored token whose
    /// size is a pair of amounts becomes one with the numbers those amounts
    /// came to, and every other token is itself.
    pub(super) fn resolved_token_stats(
        &self,
        token: TokenCharacteristics,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> TokenCharacteristics {
        let Some(stats) = token.variable_stats else {
            return token;
        };
        let amount = |value| {
            i16::try_from(
                self.effect_value(value, object, context, scoped)
                    .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
            )
            .expect("the amount was clamped to i16")
        };
        token.with_resolved_stats(amount(stats.power), amount(stats.toughness))
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
    /// "Put it onto the battlefield, then ...": the move, the binding, and
    /// the clause that names what arrived.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn put_onto_battlefield_then(
        &mut self,
        recipient: crate::card::EffectRecipientDef,
        binding: crate::ObjectSetBindingIndex,
        counters: Option<crate::card::TokenCountersDef>,
        then: &'static EffectDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let controller = object.controller;
        let counters = counters.map(|counters| {
            (
                counters.kind,
                u16::try_from(
                    self.effect_value(counters.amount, object, &context, scoped)
                        .max(0),
                )
                .unwrap_or(u16::MAX),
            )
        });
        let mut arrivals = Vec::new();
        for target in self.effect_recipients(recipient, object, &context, scoped) {
            if let Some(arrived) = self.move_target_to_zone(
                target,
                crate::card::ZoneKind::Battlefield,
                super::super::ZoneMoveCause::Effect { controller },
                Some(super::super::BattlefieldArrival::under(controller).with_counters(counters)),
                crate::card::ZonePlacement::Top,
            ) {
                arrivals.push(Target::Permanent(arrived));
            }
        }
        let mut context = context;
        context.bind_object_group(binding, arrivals);
        self.resolve_effect_def(scoped.with_effect(*then), object, context);
    }
}
