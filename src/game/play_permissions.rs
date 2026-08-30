//! "You may play lands from your graveyard", and the same sentence pointed
//! at the top of a library.
//!
//! The mirror of the play prohibitions next door: those say what a player
//! may not do, and this says what they may do that the ordinary rules would
//! refuse. It is asked where the action is offered, so a card the
//! permission does not name simply is not playable from there.

use std::ops::ControlFlow;

use super::{
    AbilityId, AbilitySourceRef, AppliedEffectDef, AppliedRuleDef, CardInstance,
    CharacteristicContext, DeclarativeAbilityDef, Game, GameObjectId, Permanent, PlayActionKind,
    PlayOptionDef, PlayerId,
};
use crate::card::{
    AbilityDef, CastTimingPermissionDef, GraveyardPlayPermissionDef, ObjectPredicateDef,
    PlayRestrictionDef, TopOfLibraryCostDef, ZoneKind,
};

/// One printed permission to play a card from a zone the ordinary rules
/// would not allow, and what playing it that way costs.
#[derive(Clone, Copy)]
pub(super) enum PlayPermission {
    Graveyard(GraveyardPlayPermissionDef),
    TopOfLibrary {
        restriction: PlayRestrictionDef,
        cost: TopOfLibraryCostDef,
    },
    /// Not a permission to play from a zone but a way to be cast out of one:
    /// the card is already castable from a graveyard by this clause, and
    /// what the grant supplies is the cost.
    GraveyardAlternativeCast {
        object: ObjectPredicateDef,
        ability: &'static AbilityDef,
    },
    /// Not a permission to play from a zone but one to play at a time: the
    /// matching spells may be cast whenever an instant could be.
    AsThoughItHadFlash(CastTimingPermissionDef),
}

impl PlayPermission {
    const fn restriction(self) -> Option<PlayRestrictionDef> {
        match self {
            Self::Graveyard(permission) => Some(permission.restriction),
            Self::TopOfLibrary { restriction, .. } => Some(restriction),
            Self::GraveyardAlternativeCast { .. } | Self::AsThoughItHadFlash(_) => None,
        }
    }
}

impl PlayPermission {
    /// Whether the limits a permission carries leave it open right now.
    /// Only the graveyard one has any: "once during each of your turns" is
    /// two bounds at once, and neither is about what the permission names.
    fn is_open_now(self, game: &Game, player: PlayerId, source: GameObjectId) -> bool {
        let Self::Graveyard(permission) = self else {
            return true;
        };
        if permission.your_turns_only && game.active_player != player {
            return false;
        }
        permission
            .per_turn
            .is_none_or(|allowed| game.graveyard_permission_uses(source) < u16::from(allowed))
    }
}

impl Game {
    /// How many times a permission granted by this source has been used this
    /// turn.
    pub(super) fn graveyard_permission_uses(&self, source: GameObjectId) -> u16 {
        self.graveyard_permission_uses
            .iter()
            .find(|(object, _)| *object == source)
            .map_or(0, |(_, uses)| *uses)
    }

    /// Records a play made under a limited graveyard permission. The one that
    /// authorized it is the first limited one that names the card, since an
    /// unlimited permission covering the same play would have been used
    /// instead and nothing would be spent.
    pub(super) fn record_graveyard_permission_use(
        &mut self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) {
        let Some(source) = self.limited_graveyard_permission_source(card, player, option) else {
            return;
        };
        if let Some(entry) = self
            .graveyard_permission_uses
            .iter_mut()
            .find(|(object, _)| *object == source)
        {
            entry.1 = entry.1.saturating_add(1);
        } else {
            self.graveyard_permission_uses.push((source, 1));
        }
    }

    /// Whether a permission lets this player cast this card as though it had
    /// flash. Such a permission names what it covers with a predicate of its
    /// own rather than through a play restriction: what it grants is a time
    /// rather than a zone, so there is no play action to match.
    pub(super) fn cast_as_though_it_had_flash(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> bool {
        let context = CharacteristicContext::Stack {
            form: option.form.clone(),
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, player, &context)
        else {
            return false;
        };
        self.visit_play_permissions(player, |source, permission| {
            if let PlayPermission::AsThoughItHadFlash(permission) = permission
                && self.trigger_object_matches(permission.object, &object, source.object, true)
            {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    /// Ends every resolving timing permission whose duration includes the
    /// next matching cast and whose predicate names the spell actually cast.
    /// Merely asking whether a cast could begin -- suspend's special action
    /// does exactly that -- expires nothing.
    pub(super) fn expire_cast_timing_permissions_for_cast(
        &mut self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) {
        let context = CharacteristicContext::Stack {
            form: option.form.clone(),
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, player, &context)
        else {
            return;
        };
        let expiring = self
            .resolved_play_permissions
            .iter()
            .filter_map(|resolved| {
                let AppliedRuleDef::MayCastAsThoughItHadFlash(permission) = resolved.rule else {
                    return None;
                };
                (resolved.affected_player == player
                    && resolved.expiration.expires_on_next_matching_cast()
                    && self.continuous_effect_expiration_is_active(
                        resolved.expiration,
                        resolved.source.object,
                    )
                    && self.trigger_object_matches(
                        permission.object,
                        &object,
                        resolved.source.object,
                        true,
                    ))
                .then_some((resolved.source, resolved.definition))
            })
            .collect::<Vec<_>>();
        self.resolved_play_permissions
            .retain(|resolved| !expiring.contains(&(resolved.source, resolved.definition)));
    }

    /// Whether this player may play this card out of a graveyard right now.
    pub(super) fn graveyard_play_is_permitted(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> bool {
        // A permission handed out by a resolution names one card for the
        // turn; the static ones below name a whole class of them for as
        // long as their source is on the battlefield.
        self.graveyard_cast_permission(card.id, player).is_some()
            || self
                .matching_play_permission(card, player, option, |permission| {
                    matches!(permission, PlayPermission::Graveyard(_)).then_some(())
                })
                .is_some()
    }

    /// What playing this card off the top of its owner's library would cost,
    /// or `None` when nothing permits it.
    pub(super) fn library_top_play_cost(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> Option<TopOfLibraryCostDef> {
        self.matching_play_permission(card, player, option, |permission| match permission {
            PlayPermission::TopOfLibrary { cost, .. } => Some(cost),
            PlayPermission::Graveyard(_)
            | PlayPermission::GraveyardAlternativeCast { .. }
            | PlayPermission::AsThoughItHadFlash(_) => None,
        })
    }

    /// What a spell cast off the top of `player`'s library pays in life,
    /// when the permission charges life rather than mana.
    ///
    /// The mana value is the card's own, with X counted as zero: a spell
    /// nobody is paying mana for has no X to choose (CR 202.3b), the same
    /// reading the energy permission next door already uses.
    pub(super) fn library_top_life_cost(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> Option<u16> {
        if self.library_top_play_cost(card, player, option)
            != Some(TopOfLibraryCostDef::LifeEqualToManaValue)
        {
            return None;
        }
        Some(
            self.catalog
                .get(card.definition)?
                .rules
                .printed_mana_cost()
                .mana_value(),
        )
    }

    /// The alternative way to cast this card out of its owner's graveyard
    /// that a static ability on the battlefield grants it, if any.
    ///
    /// Read where the cast is enumerated rather than off the card: nothing
    /// modifies a card lying in a graveyard the way a layer walk modifies a
    /// permanent, so the grant is found by asking the battlefield.
    pub(super) fn granted_graveyard_alternative_cast(
        &self,
        card: &CardInstance,
        player: PlayerId,
    ) -> Option<&'static AbilityDef> {
        if card.owner != player {
            return None;
        }
        let object = self.printed_trigger_event_object(
            card.id,
            card.definition,
            player,
            &CharacteristicContext::Graveyard,
        )?;
        let mut found = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            if let PlayPermission::GraveyardAlternativeCast {
                object: predicate,
                ability,
            } = permission
                && ability.is_executable()
                && self.trigger_object_matches(predicate, &object, source.object, false)
            {
                found = Some(ability);
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        found
    }

    /// The source of the limited permission a play would spend, when no
    /// unlimited one covers the same play.
    fn limited_graveyard_permission_source(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> Option<GameObjectId> {
        let mut limited = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            let PlayPermission::Graveyard(graveyard) = permission else {
                return ControlFlow::Continue(());
            };
            if !self.permission_names_play(
                card,
                player,
                option,
                graveyard.restriction,
                source.object,
            ) {
                return ControlFlow::Continue(());
            }
            if graveyard.per_turn.is_none() {
                limited = None;
                return ControlFlow::Break(());
            }
            if limited.is_none() && permission.is_open_now(self, player, source.object) {
                limited = Some(source.object);
            }
            ControlFlow::Continue(())
        });
        limited
    }

    /// What the permission authorizing this play grants to what it played,
    /// if anything. Read with the same walk that spends a limited
    /// permission's use, so the grant and the bookkeeping agree about which
    /// permission is doing the allowing.
    pub(super) fn graveyard_play_grant(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
    ) -> Option<(AbilitySourceRef, &'static AppliedEffectDef)> {
        let mut granted = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            let PlayPermission::Graveyard(graveyard) = permission else {
                return ControlFlow::Continue(());
            };
            if !self.permission_names_play(
                card,
                player,
                option,
                graveyard.restriction,
                source.object,
            ) || !permission.is_open_now(self, player, source.object)
            {
                return ControlFlow::Continue(());
            }
            // A permission that grants nothing is the ordinary one, and it
            // covers the play whether or not another would have granted
            // something: nothing makes a player choose between them.
            let Some(effect) = graveyard.grants else {
                granted = None;
                return ControlFlow::Break(());
            };
            if granted.is_none() {
                granted = Some((source, effect));
            }
            ControlFlow::Continue(())
        });
        granted
    }

    /// Whether a restriction names this card and this way of playing it.
    fn permission_names_play(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
        restriction: PlayRestrictionDef,
        source: GameObjectId,
    ) -> bool {
        let context = match option.action {
            PlayActionKind::CastSpell => CharacteristicContext::Stack {
                form: option.form.clone(),
            },
            PlayActionKind::PlayLand => CharacteristicContext::Hand,
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, player, &context)
        else {
            return false;
        };
        restriction.action.matches(option.action)
            && self.trigger_object_matches(
                restriction.object,
                &object,
                source,
                option.action == PlayActionKind::CastSpell,
            )
    }

    /// The first live permission that names this card and this play option,
    /// as whatever `wanted` reads off it.
    fn matching_play_permission<T>(
        &self,
        card: &CardInstance,
        player: PlayerId,
        option: &PlayOptionDef,
        wanted: impl Fn(PlayPermission) -> Option<T>,
    ) -> Option<T> {
        // Only your own cards: no printed permission reaches another
        // player's zones, and the enumeration walks only yours.
        if card.owner != player {
            return None;
        }
        let context = match option.action {
            PlayActionKind::CastSpell => CharacteristicContext::Stack {
                form: option.form.clone(),
            },
            PlayActionKind::PlayLand => CharacteristicContext::Hand,
        };
        let object =
            self.printed_trigger_event_object(card.id, card.definition, player, &context)?;
        let mut found = None;
        let _ = self.visit_play_permissions(player, |source, permission| {
            let Some(restriction) = permission.restriction() else {
                return ControlFlow::Continue(());
            };
            if permission.is_open_now(self, player, source.object)
                && restriction.action.matches(option.action)
                && self.trigger_object_matches(
                    restriction.object,
                    &object,
                    source.object,
                    option.action == PlayActionKind::CastSpell,
                )
                && let Some(value) = wanted(permission)
            {
                found = Some(value);
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        found
    }

    fn visit_play_permissions(
        &self,
        affected_player: PlayerId,
        mut visitor: impl FnMut(AbilitySourceRef, PlayPermission) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // A permission that resolved rather than one printed on a permanent:
        // "you may cast spells from your graveyard this turn" is aimed at a
        // player and outlives nothing but the turn, so it is stored on the
        // game the way a resolved prohibition is.
        for resolved in &self.resolved_play_permissions {
            if resolved.affected_player != affected_player
                || !self.continuous_effect_expiration_is_active(
                    resolved.expiration,
                    resolved.source.object,
                )
            {
                continue;
            }
            let mut found = ControlFlow::Continue(());
            Self::visit_play_permission_components(
                AppliedEffectDef::Rule(resolved.rule),
                &mut |permission| {
                    if found.is_continue() {
                        found = visitor(resolved.source, permission);
                    }
                },
            );
            found?;
        }
        for source in &self.battlefield {
            self.visit_static_play_permissions(
                source,
                Some(ZoneKind::Battlefield),
                affected_player,
                &mut visitor,
            )?;
        }
        for source in &self.emblems {
            self.visit_static_play_permissions(source, None, affected_player, &mut visitor)?;
        }
        // Reuse the shared graveyard static sources so play permissions have
        // the same source identity and effective rules as other static effects.
        let graveyard_sources = self.graveyard_static_sources();
        for source in &graveyard_sources {
            self.visit_static_play_permissions(
                source,
                Some(ZoneKind::Graveyard),
                affected_player,
                &mut visitor,
            )?;
        }
        ControlFlow::Continue(())
    }

    fn visit_static_play_permissions(
        &self,
        source: &Permanent,
        required_source_zone: Option<ZoneKind>,
        affected_player: PlayerId,
        visitor: &mut impl FnMut(AbilitySourceRef, PlayPermission) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(rules) = self.effective_rules(source) else {
            return ControlFlow::Continue(());
        };
        // The clause index is the ability's printed id, which is what a
        // grant made under this permission has to record: the effect it
        // hands out is addressed from the ability that printed it.
        for (index, ability) in rules.ability_clauses().iter().enumerate() {
            let DeclarativeAbilityDef::Static(definition) = ability.definition else {
                continue;
            };
            if !ability.is_executable()
                || required_source_zone.is_some_and(|zone| !definition.source_zones.contains(&zone))
            {
                continue;
            }
            let Some(effect) = ability.declarative_effect() else {
                continue;
            };
            // "During your turn, ... have retrace": a permission can be
            // gated, and a gate that is shut is not a permission at all.
            let effect = match effect {
                conditional @ (super::EffectDef::IfCondition { .. }
                | super::EffectDef::IfElseCondition { .. }) => {
                    let conditional = conditional
                        .conditional()
                        .expect("conditional variants expose their shared shape");
                    let condition_holds = self.trigger_condition_holds(
                        conditional.condition,
                        source.card.id,
                        source.controller,
                        super::TriggerContext::empty(),
                        None,
                        None,
                    );
                    let Some(branch) = conditional.branch(condition_holds) else {
                        continue;
                    };
                    *branch
                }
                effect => effect,
            };
            let super::EffectDef::StaticApply { recipient, effect } = effect else {
                continue;
            };
            if !self.static_player_recipient_matches(recipient, source, affected_player) {
                continue;
            }
            let Ok(index) = u8::try_from(index) else {
                continue;
            };
            let origin = AbilitySourceRef {
                object: source.card.id,
                ability: Self::authored_ability_origin(
                    Self::effective_rules_source(source),
                    AbilityId(index),
                ),
            };
            let mut found = ControlFlow::Continue(());
            Self::visit_play_permission_components(effect, &mut |permission| {
                if found.is_continue() {
                    found = visitor(origin, permission);
                }
            });
            found?;
        }
        ControlFlow::Continue(())
    }

    fn visit_play_permission_components(
        effect: AppliedEffectDef,
        visitor: &mut impl FnMut(PlayPermission),
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::visit_play_permission_components(*effect, visitor);
                }
            }
            AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(permission)) => {
                visitor(PlayPermission::Graveyard(permission));
            }
            AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                restriction,
                cost,
            }) => {
                visitor(PlayPermission::TopOfLibrary { restriction, cost });
            }
            AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
                object,
                ability,
            }) => {
                visitor(PlayPermission::GraveyardAlternativeCast { object, ability });
            }
            AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(permission)) => {
                visitor(PlayPermission::AsThoughItHadFlash(permission));
            }
            _ => {}
        }
    }
}
