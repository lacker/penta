//! "Players can't untap more than one ... during their untap steps."
//!
//! Another player-facing static, found the same way as the rules next door.
//! It descends through conditions, because Winter Orb's cap applies only
//! while the Orb itself is untapped -- a condition on the source, not on the
//! player it caps.

use crate::card::{
    AppliedEffectDef, AppliedRuleDef, DeclarativeAbilityDef, EffectDef, ObjectPredicateDef,
};
use crate::ids::{GameObjectId, PlayerId};

use super::super::{Game, Permanent, TriggerContext};

/// One cap, with the permanent imposing it: the source is what a predicate
/// naming "this" is measured against.
pub(in crate::game) type UntapLimit = (GameObjectId, ObjectPredicateDef);

impl Game {
    /// Every untap cap this player is currently under.
    pub(in crate::game) fn untap_limits(&self, affected_player: PlayerId) -> Vec<UntapLimit> {
        let mut limits = Vec::new();
        let land_type_sources = self.land_type_effect_sources(None);
        for source in self.battlefield.iter().chain(self.emblems.iter()) {
            let Some(rules) = self.effective_rules(source) else {
                continue;
            };
            let source_presentation = Self::effective_rules_source(source);
            if self.rules_text_abilities_removed_from_sources(source, &land_type_sources) {
                continue;
            }
            for attached in rules.indexed_abilities() {
                if !attached.definition.is_executable()
                    || !matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::Static(_)
                    )
                {
                    continue;
                }
                if !self.ability_survives_resolved_operations(
                    source,
                    Self::authored_ability_origin(source_presentation, attached.id),
                ) {
                    continue;
                }
                let Some(effect) = attached.definition.declarative_effect() else {
                    continue;
                };
                self.collect_untap_limits(effect, source, affected_player, true, &mut limits);
            }
        }
        limits
    }

    fn collect_untap_limits(
        &self,
        effect: EffectDef,
        source: &Permanent,
        affected_player: PlayerId,
        enabled: bool,
        limits: &mut Vec<UntapLimit>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.collect_untap_limits(*effect, source, affected_player, enabled, limits);
                }
            }
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                let holds = enabled
                    && self.trigger_condition_holds(
                        conditional.condition,
                        source.card.id,
                        source.controller,
                        TriggerContext::empty(),
                        None,
                        None,
                    );
                self.collect_untap_limits(
                    *conditional.then,
                    source,
                    affected_player,
                    holds,
                    limits,
                );
                if let Some(otherwise) = conditional.otherwise {
                    self.collect_untap_limits(
                        *otherwise,
                        source,
                        affected_player,
                        enabled && !holds,
                        limits,
                    );
                }
            }
            EffectDef::StaticApply { recipient, effect }
                if enabled
                    && self.static_player_recipient_matches(recipient, source, affected_player) =>
            {
                Self::collect_untap_limit_components(effect, source.card.id, limits);
            }
            _ => {}
        }
    }

    fn collect_untap_limit_components(
        effect: AppliedEffectDef,
        source: GameObjectId,
        limits: &mut Vec<UntapLimit>,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::collect_untap_limit_components(*effect, source, limits);
                }
            }
            AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(predicate)) => {
                limits.push((source, predicate));
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => {}
        }
    }

    /// The still-tapped permanents this player controls that one cap covers.
    pub(in crate::game) fn permanents_under_untap_limit(
        &self,
        player: PlayerId,
        limit: UntapLimit,
    ) -> Vec<GameObjectId> {
        let (source, predicate) = limit;
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && !self.skips_turn_based_untap(permanent)
                    && self.trigger_object_matches(
                        predicate,
                        &self.targeting_event_object(permanent),
                        source,
                        false,
                    )
            })
            .map(|permanent| permanent.card.id)
            .collect()
    }
}
