use crate::card::{
    DamageCoverageDef, DamageKindDef, DamagePreventionCapacityDef, DamagePreventionFollowUpDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DamageSourceMatcherDef, EffectDef,
    EffectRecipientDef,
};

use super::super::prevention_state::{
    ResolvedDamagePrevention, ResolvedDamagePreventionCapacity, ResolvedDamagePreventionCoverage,
    ResolvedDamageRecipientMatcher, ResolvedDamageSourceMatcher,
};
use super::super::{
    AbilityId, AbilitySourceRef, EffectResolutionContext, Game, RelationalSourceFilter,
    ScopedEffect, StackObject, Target,
};

impl Game {
    pub(super) fn resolve_prevention_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let EffectDef::PreventDamage {
            prevention,
            duration,
        } = scoped.effect
        else {
            unreachable!("resolve_prevention_effect called for a non-prevention effect");
        };

        let Some(source) =
            self.resolve_damage_source_matcher(prevention.matcher.source, object, context, scoped)
        else {
            return;
        };
        let recipients = self.resolve_damage_recipient_matchers(
            prevention.matcher.recipient,
            object,
            context,
            scoped,
        );
        if recipients.is_empty() {
            return;
        }

        let capacity = match prevention.capacity {
            DamagePreventionCapacityDef::Amount(amount) => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                if amount == 0 {
                    return;
                }
                ResolvedDamagePreventionCapacity::Amount(amount)
            }
            DamagePreventionCapacityDef::Events(events) => {
                if events == 0 {
                    return;
                }
                ResolvedDamagePreventionCapacity::Events(u16::from(events))
            }
            DamagePreventionCapacityDef::Unlimited => ResolvedDamagePreventionCapacity::Unlimited,
        };
        let coverage = match prevention.coverage {
            DamageCoverageDef::All => ResolvedDamagePreventionCoverage::All,
            DamageCoverageDef::HalfRoundedDown => ResolvedDamagePreventionCoverage::HalfRoundedDown,
        };
        let gain_life = match prevention.follow_up {
            Some(DamagePreventionFollowUpDef::GainLife(player)) => self
                .effect_recipients(EffectRecipientDef::player(player), object, context, scoped)
                .into_iter()
                .find_map(|target| match target {
                    Target::Player(player) => Some(player),
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                }),
            None => None,
        };
        let source_ability = AbilitySourceRef {
            object: object.source.unwrap_or(object.id),
            ability: object.ability_origin().unwrap_or_else(|| {
                Self::authored_ability_origin(object.presentation(), AbilityId::PRIMARY)
            }),
        };
        let timestamp = self.allocate_continuous_effect_timestamp();
        let expiration = Self::continuous_effect_expiration(
            duration,
            object.controller,
            self.turns_started[object.controller.index()],
        );
        let combat_only = matches!(prevention.matcher.kind, DamageKindDef::Combat);

        self.damage_preventions
            .extend(
                recipients
                    .into_iter()
                    .map(|recipient| ResolvedDamagePrevention {
                        source,
                        recipient,
                        combat_only,
                        capacity,
                        coverage,
                        gain_life,
                        source_ability,
                        timestamp,
                        expiration,
                    }),
            );
    }

    fn resolve_damage_source_matcher(
        &self,
        matcher: DamageSourceMatcherDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<ResolvedDamageSourceMatcher> {
        match matcher {
            DamageSourceMatcherDef::Any => Some(ResolvedDamageSourceMatcher::Any),
            DamageSourceMatcherDef::AffectedObject => {
                debug_assert!(
                    false,
                    "AffectedObject is only meaningful for a static effect"
                );
                None
            }
            DamageSourceMatcherDef::Object(reference)
            | DamageSourceMatcherDef::Except(reference) => {
                let referenced = self
                    .effect_recipients(
                        EffectRecipientDef::object(reference),
                        object,
                        context,
                        scoped,
                    )
                    .into_iter()
                    .find_map(target_object_id)?;
                Some(if matches!(matcher, DamageSourceMatcherDef::Object(_)) {
                    ResolvedDamageSourceMatcher::Exact(referenced)
                } else {
                    ResolvedDamageSourceMatcher::Except(referenced)
                })
            }
            DamageSourceMatcherDef::Matching(predicate) => {
                Some(ResolvedDamageSourceMatcher::Matching {
                    predicate,
                    relative_to: object.source.unwrap_or(object.id),
                })
            }
            DamageSourceMatcherDef::Group(group) => {
                Some(ResolvedDamageSourceMatcher::Group(match group {
                    DamageSourceGroupDef::CreaturesWithFlying => {
                        RelationalSourceFilter::CreaturesWithFlying
                    }
                    DamageSourceGroupDef::AttackingCreaturesWithoutFlying => {
                        RelationalSourceFilter::AttackingCreaturesWithoutFlying
                    }
                    DamageSourceGroupDef::Artifacts => RelationalSourceFilter::Artifacts,
                    DamageSourceGroupDef::UnblockedCreatures => {
                        RelationalSourceFilter::UnblockedCreatures
                    }
                }))
            }
        }
    }

    fn resolve_damage_recipient_matchers(
        &self,
        matcher: DamageRecipientMatcherDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<ResolvedDamageRecipientMatcher> {
        match matcher {
            DamageRecipientMatcherDef::Any => vec![ResolvedDamageRecipientMatcher::Any],
            DamageRecipientMatcherDef::AffectedObject => {
                debug_assert!(
                    false,
                    "AffectedObject is only meaningful for a static effect"
                );
                Vec::new()
            }
            DamageRecipientMatcherDef::PlayerOrPlaneswalker => {
                debug_assert!(
                    false,
                    "PlayerOrPlaneswalker is only meaningful for a trigger"
                );
                Vec::new()
            }
            DamageRecipientMatcherDef::Recipients(recipients) => self
                .effect_recipients(recipients, object, context, scoped)
                .into_iter()
                .map(ResolvedDamageRecipientMatcher::Exact)
                .collect(),
            DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => self
                .effect_recipients(EffectRecipientDef::player(player), object, context, scoped)
                .into_iter()
                .filter_map(|target| match target {
                    Target::Player(player) => {
                        Some(ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(player))
                    }
                    Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                })
                .collect(),
        }
    }
}

fn target_object_id(target: Target) -> Option<super::super::GameObjectId> {
    match target {
        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
        Target::Player(_) => None,
    }
}
