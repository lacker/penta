//! Static rules whose subject is a player.
//!
//! A rule applied to an object is found through that object: damage
//! prevention is read off the source or target permanent, a blocking
//! restriction off the creature it restricts. A rule applied to a *player*
//! has no such anchor, so each of these walks the battlefield and emblems
//! and asks every static ability whether its recipient is the player in
//! question.

use std::ops::ControlFlow;

use crate::card::{
    AppliedEffectDef, AppliedRuleDef, AttackDefenderScopeDef, DamageEventMatcherDef,
    DamageLimitDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EffectRecipientSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef,
};
use crate::ids::{GameObjectId, PlayerId};

use super::super::{
    AppliedAttackRestriction, AppliedPlayRestriction, CardInstance, CharacteristicContext, Game,
    Permanent, TriggerContext, TriggerEventObject,
};

impl Game {
    /// Visits every static and resolved attack restriction applying to one
    /// player. The rule itself decides whether it protects only that player
    /// or their planeswalkers as well.
    pub(in crate::game) fn visit_attack_restrictions(
        &self,
        affected_player: PlayerId,
        mut visitor: impl FnMut(AppliedAttackRestriction) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for resolved in self
            .resolved_attack_restrictions
            .iter()
            .filter(|restriction| {
                restriction.affected_player == affected_player
                    && self.continuous_effect_expiration_is_active(
                        restriction.expiration,
                        restriction.source.object,
                    )
            })
        {
            if visitor(AppliedAttackRestriction {
                source: resolved.source.object,
                affected_player,
                restriction: resolved.restriction,
            })
            .is_break()
            {
                return ControlFlow::Break(());
            }
        }

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
                    || !self.ability_survives_resolved_operations(
                        source,
                        Self::authored_ability_origin(source_presentation, attached.id),
                    )
                {
                    continue;
                }
                let Some(effect) = attached.definition.declarative_effect() else {
                    continue;
                };
                if self
                    .visit_static_attack_restrictions(
                        effect,
                        source,
                        affected_player,
                        true,
                        &mut visitor,
                    )
                    .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_static_attack_restrictions(
        &self,
        effect: EffectDef,
        source: &Permanent,
        affected_player: PlayerId,
        enabled: bool,
        visitor: &mut impl FnMut(AppliedAttackRestriction) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    if self
                        .visit_static_attack_restrictions(
                            *effect,
                            source,
                            affected_player,
                            enabled,
                            visitor,
                        )
                        .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                let condition_holds = enabled
                    && self.trigger_condition_holds(
                        conditional.condition,
                        source.card.id,
                        source.controller,
                        TriggerContext::empty(),
                        None,
                        None,
                    );
                let then_result = self.visit_static_attack_restrictions(
                    *conditional.then,
                    source,
                    affected_player,
                    condition_holds,
                    visitor,
                );
                if then_result.is_break() {
                    return then_result;
                }
                conditional
                    .otherwise
                    .map_or(ControlFlow::Continue(()), |otherwise| {
                        self.visit_static_attack_restrictions(
                            *otherwise,
                            source,
                            affected_player,
                            enabled && !condition_holds,
                            visitor,
                        )
                    })
            }
            EffectDef::StaticApply { recipient, effect } => {
                if !enabled
                    || !self.static_player_recipient_matches(recipient, source, affected_player)
                {
                    return ControlFlow::Continue(());
                }
                Self::visit_attack_restriction_components(effect, source, affected_player, visitor)
            }
            _ => ControlFlow::Continue(()),
        }
    }

    fn visit_attack_restriction_components(
        effect: AppliedEffectDef,
        source: &Permanent,
        affected_player: PlayerId,
        visitor: &mut impl FnMut(AppliedAttackRestriction) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    if Self::visit_attack_restriction_components(
                        *effect,
                        source,
                        affected_player,
                        visitor,
                    )
                    .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction))
                if restriction.defender != AttackDefenderScopeDef::Any =>
            {
                visitor(AppliedAttackRestriction {
                    source: source.card.id,
                    affected_player,
                    restriction,
                })
            }
            AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {
                ControlFlow::Continue(())
            }
        }
    }

    /// Visits static and resolved play prohibitions in timestamp/component
    /// order for one player. Static prohibitions are derived live from their
    /// source; resolving prohibitions use the game-level stored rule list.
    pub(in crate::game) fn visit_play_restrictions(
        &self,
        affected_player: PlayerId,
        mut visitor: impl FnMut(AppliedPlayRestriction) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let mut restrictions = self
            .resolved_play_restrictions
            .iter()
            .filter(|restriction| {
                restriction.affected_player == affected_player
                    && self.continuous_effect_expiration_is_active(
                        restriction.expiration,
                        restriction.source.object,
                    )
            })
            .map(|restriction| AppliedPlayRestriction {
                source: restriction.source.object,
                timestamp: restriction.timestamp,
                component_order: restriction.component_order,
                restriction: restriction.restriction,
            })
            .collect::<Vec<_>>();

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
                let mut component_order = 0;
                self.collect_static_play_restrictions(
                    effect,
                    source,
                    affected_player,
                    true,
                    &mut component_order,
                    &mut restrictions,
                );
            }
        }

        restrictions
            .sort_by_key(|restriction| (restriction.timestamp, restriction.component_order));
        for restriction in restrictions {
            if visitor(restriction).is_break() {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    /// Visits static damage limits that apply to one player, in the same
    /// battlefield-and-emblems walk the play prohibitions use. A limit whose
    /// recipient is a player has no other anchor: unlike a prevention on the
    /// source or target permanent, nothing about the damage event points at
    /// the permanent carrying the rule.
    pub(in crate::game) fn visit_player_damage_limits(
        &self,
        affected_player: PlayerId,
        mut visitor: impl FnMut(GameObjectId, DamageEventMatcherDef, DamageLimitDef) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
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
                let Some(EffectDef::StaticApply { recipient, effect }) =
                    attached.definition.declarative_effect()
                else {
                    continue;
                };
                let AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage { matcher, limit }) = effect
                else {
                    continue;
                };
                if !self.static_player_recipient_matches(recipient, source, affected_player) {
                    continue;
                }
                if visitor(source.card.id, matcher, limit).is_break() {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    /// Whether any live static ability tells `affected_player` they have no
    /// maximum hand size. Walked the same way as the damage limits above:
    /// a player rule has no anchor object, so every static ability on the
    /// battlefield is asked whether its recipient is this player.
    pub(in crate::game) fn player_has_no_maximum_hand_size(
        &self,
        affected_player: PlayerId,
    ) -> bool {
        self.player_static_rule_applies(
            affected_player,
            AppliedEffectDef::Rule(AppliedRuleDef::NoMaximumHandSize),
        )
    }

    /// How many lands beyond the ordinary one this player may play this
    /// turn. Summed rather than merely looked for: two Explorations are two
    /// extra lands, which is the whole reason the rule carries a number.
    pub(in crate::game) fn additional_land_plays(&self, affected_player: PlayerId) -> u16 {
        let mut extra = 0_u16;
        self.visit_player_static_rules(affected_player, |rule| {
            if let AppliedRuleDef::MayPlayAdditionalLands(amount) = rule {
                extra = extra.saturating_add(u16::from(amount));
            }
        });
        extra
    }

    /// Whether any live static ability applies this rule to this player.
    pub(in crate::game) fn player_rule_applies(
        &self,
        player: PlayerId,
        rule: AppliedRuleDef,
    ) -> bool {
        self.resolved_player_rules.iter().any(|resolved| {
            resolved.affected_player == player
                && AppliedRuleDef::PlayerRule(resolved.rule) == rule
                && self.continuous_effect_expiration_is_active(
                    resolved.expiration,
                    resolved.source.object,
                )
        }) || self.player_static_rule_applies(player, AppliedEffectDef::Rule(rule))
    }

    /// Whether any live static ability applies `wanted` to this player.
    fn player_static_rule_applies(
        &self,
        affected_player: PlayerId,
        wanted: AppliedEffectDef,
    ) -> bool {
        let mut found = false;
        self.visit_player_static_rules(affected_player, |rule| {
            found = found || AppliedEffectDef::Rule(rule) == wanted;
        });
        found
    }

    /// Every player rule a live static ability applies to this player, in
    /// battlefield order. The walk is the one every player-facing rule uses:
    /// such a rule has no anchor object, so each static ability on the
    /// battlefield is asked whether its recipient is this player.
    pub(in crate::game) fn visit_player_static_rules(
        &self,
        affected_player: PlayerId,
        mut visitor: impl FnMut(AppliedRuleDef),
    ) {
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
                let Some(EffectDef::StaticApply { recipient, effect }) =
                    attached.definition.declarative_effect()
                else {
                    continue;
                };
                if !self.static_player_recipient_matches(recipient, source, affected_player) {
                    continue;
                }
                Self::visit_player_rule_leaves(effect, &mut visitor);
            }
        }
    }

    /// The rule leaves of one applied effect, flattening the composites a
    /// clause that applies several at once is written as.
    fn visit_player_rule_leaves(
        effect: AppliedEffectDef,
        visitor: &mut impl FnMut(AppliedRuleDef),
    ) {
        match effect {
            AppliedEffectDef::Composite(components) => {
                for component in components {
                    Self::visit_player_rule_leaves(*component, visitor);
                }
            }
            AppliedEffectDef::Rule(rule) => visitor(rule),
            AppliedEffectDef::Characteristic(_) => {}
        }
    }

    fn collect_static_play_restrictions(
        &self,
        effect: EffectDef,
        source: &Permanent,
        affected_player: PlayerId,
        enabled: bool,
        component_order: &mut u16,
        restrictions: &mut Vec<AppliedPlayRestriction>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.collect_static_play_restrictions(
                        *effect,
                        source,
                        affected_player,
                        enabled,
                        component_order,
                        restrictions,
                    );
                }
            }
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                let condition_holds = enabled
                    && self.trigger_condition_holds(
                        conditional.condition,
                        source.card.id,
                        source.controller,
                        TriggerContext::empty(),
                        None,
                        None,
                    );
                self.collect_static_play_restrictions(
                    *conditional.then,
                    source,
                    affected_player,
                    condition_holds,
                    component_order,
                    restrictions,
                );
                if let Some(otherwise) = conditional.otherwise {
                    self.collect_static_play_restrictions(
                        *otherwise,
                        source,
                        affected_player,
                        enabled && !condition_holds,
                        component_order,
                        restrictions,
                    );
                }
            }
            EffectDef::StaticApply { recipient, effect } => {
                let include = enabled
                    && self.static_player_recipient_matches(recipient, source, affected_player);
                Self::collect_play_restriction_components(
                    effect,
                    source,
                    include,
                    component_order,
                    restrictions,
                );
            }
            _ => {}
        }
    }

    fn collect_play_restriction_components(
        effect: AppliedEffectDef,
        source: &Permanent,
        include: bool,
        component_order: &mut u16,
        restrictions: &mut Vec<AppliedPlayRestriction>,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::collect_play_restriction_components(
                        *effect,
                        source,
                        include,
                        component_order,
                        restrictions,
                    );
                }
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => {
                let order = *component_order;
                *component_order = component_order
                    .checked_add(1)
                    .expect("one static ability contains at most 65,536 applied components");
                if include
                    && let AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(restriction)) = effect
                {
                    restrictions.push(AppliedPlayRestriction {
                        source: source.card.id,
                        timestamp: source.timestamp,
                        component_order: order,
                        restriction,
                    });
                }
            }
        }
    }

    pub(in crate::game) fn static_player_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected_player: PlayerId,
    ) -> bool {
        match recipient.0 {
            EffectRecipientSetDef::Players(PlayerSetDef::All) => true,
            EffectRecipientSetDef::Players(PlayerSetDef::One(PlayerRefDef::EffectController)) => {
                affected_player == source.controller
            }
            EffectRecipientSetDef::Players(PlayerSetDef::One(PlayerRefDef::Opponent)) => {
                affected_player == source.controller.opponent()
            }
            EffectRecipientSetDef::Players(PlayerSetDef::Related(PlayerRelation::ChosenPlayer)) => {
                self.chosen_player_of(source.card.id) == Some(affected_player)
            }
            EffectRecipientSetDef::Players(PlayerSetDef::One(PlayerRefDef::EnchantedPlayer)) => {
                source.attached_player == Some(affected_player)
            }
            EffectRecipientSetDef::Players(PlayerSetDef::Related(relation)) => self
                .player_relation_matches_for_source(
                    affected_player,
                    relation,
                    source.controller,
                    source.card.id,
                    TriggerContext::empty(),
                ),
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::Objects(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
            | EffectRecipientSetDef::Players(
                PlayerSetDef::LegalTargets(_)
                | PlayerSetDef::One(
                    PlayerRefDef::EventPlayer
                    | PlayerRefDef::Target(_)
                    | PlayerRefDef::ControllerOf(_)
                    | PlayerRefDef::OpponentOf(_)
                    | PlayerRefDef::OwnerOf(_),
                ),
            )
            | EffectRecipientSetDef::DefenderOf(_) => false,
        }
    }
}

impl Game {
    /// Whether a player is presently barred from activating anything but a
    /// mana ability. Abeyance is the printed form, and it lasts a turn.
    /// Split second (CR 702.19a): while such a spell is on the stack nobody
    /// may cast a spell or activate anything that is not a mana ability.
    /// Read off the stack rather than off any permanent, which is why it
    /// sits beside the restrictions rather than among them.
    pub(in crate::game) fn split_second_is_active(&self) -> bool {
        self.stack.iter().any(|object| {
            object.kind == crate::game::StackObjectKind::Spell
                && object
                    .card
                    .definition
                    .card_definition()
                    .and_then(|definition| self.catalog.get(definition))
                    .is_some_and(|definition| {
                        definition.parts.iter().any(|part| {
                            part.rules
                                .has_executable_keyword(crate::card::KeywordAbility::SplitSecond)
                        })
                    })
        })
    }

    /// Whether a player-facing prohibition names this permanent's nonmana
    /// activated abilities. The object predicate matters: Abeyance names any
    /// source, while Pithing Needle names only the card name it chose.
    pub(in crate::game) fn nonmana_ability_activation_is_prohibited(
        &self,
        player: PlayerId,
        permanent: &Permanent,
    ) -> bool {
        self.nonmana_ability_activation_of_object_is_prohibited(
            player,
            &self.trigger_event_object(permanent),
        )
    }

    pub(in crate::game) fn nonmana_ability_activation_of_object_is_prohibited(
        &self,
        player: PlayerId,
        object: &TriggerEventObject,
    ) -> bool {
        self.visit_play_restrictions(player, |applied| {
            if applied.restriction.action
                == crate::card::PlayActionMatcherDef::ActivateNonManaAbility
                && self.trigger_object_matches(
                    applied.restriction.object,
                    object,
                    applied.source,
                    false,
                )
            {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    pub(in crate::game) fn nonbattlefield_ability_activation_is_prohibited(
        &self,
        player: PlayerId,
        card: &CardInstance,
        context: &CharacteristicContext,
    ) -> bool {
        self.printed_trigger_event_object(card.id, card.definition, player, context)
            .is_some_and(|object| {
                self.nonmana_ability_activation_of_object_is_prohibited(player, &object)
            })
    }
}
