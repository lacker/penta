use std::cell::Cell;

#[cfg(test)]
use super::AbilityId;
use super::{
    AbilityOperationDef, AbilityOrigin, AbilityTargetPredicate, AppliedEffectDef,
    AppliedPlayRestriction, AppliedRuleDef, AppliedRuleEffect, CardDefinitionId, CardRules,
    CardSet, CardType, CardTypeSet, CharacteristicOperationDef, ColorSet,
    ContinuousEffectExpiration, ControlFlow, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    EffectRecipientSetDef, Game, GameObjectId, GrantId, KeywordAbility, ManaColor,
    ObjectPredicateDef, ObjectRefDef, ObjectSetDef, Permanent, PlayerId, PlayerRefDef,
    PlayerSetDef, ResolvedContinuousEffect, ResolvedContinuousEffectKind, RetiredObject,
    SetOperationDef, StackAbilityResolver, StackObject, StaticAppliedEffect, StaticEffectTraversal,
    Target, TargetIndex, TriggerContext, ZoneKind,
};

thread_local! {
    /// Guards the live set-characteristic walk when a static recipient query
    /// asks for the same characteristics being assembled.
    static STATIC_SET_CHARACTERISTIC_LAYER_PASS: Cell<bool> = const { Cell::new(false) };
}

struct StaticSetCharacteristicLayerGuard;

impl StaticSetCharacteristicLayerGuard {
    fn enter() -> Option<Self> {
        STATIC_SET_CHARACTERISTIC_LAYER_PASS
            .with(|pass| if pass.replace(true) { None } else { Some(Self) })
    }
}

impl Drop for StaticSetCharacteristicLayerGuard {
    fn drop(&mut self) {
        STATIC_SET_CHARACTERISTIC_LAYER_PASS.with(|pass| pass.set(false));
    }
}

impl Game {
    /// Whether a resolved characteristic component still contributes at this
    /// moment. A source-tapped duration is read through its recorded source
    /// object, not through the affected permanent.
    pub(super) fn resolved_continuous_effect_is_active(
        &self,
        effect: &ResolvedContinuousEffect,
    ) -> bool {
        self.continuous_effect_expiration_is_active(effect.expiration, effect.source.object)
    }

    fn continuous_effect_expiration_is_active(
        &self,
        expiration: ContinuousEffectExpiration,
        source: GameObjectId,
    ) -> bool {
        match expiration {
            ContinuousEffectExpiration::WhileSourceTapped => self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == source && permanent.tapped),
            ContinuousEffectExpiration::EndOfTurn
            | ContinuousEffectExpiration::UpkeepOf(_)
            | ContinuousEffectExpiration::TurnOf { .. }
            | ContinuousEffectExpiration::Never => true,
        }
    }

    /// Visits static and resolved play prohibitions in timestamp/component
    /// order for one player. Static prohibitions are derived live from their
    /// source; resolving prohibitions use the game-level stored rule list.
    pub(super) fn visit_play_restrictions(
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
            let (source_definition, source_part) = Self::effective_rules_source(source);
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
                    AbilityOrigin::Printed {
                        definition: source_definition,
                        part: source_part,
                        ability: attached.id,
                    },
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
            EffectDef::IfCondition { condition, then } => {
                let condition_holds = enabled
                    && self.trigger_condition_holds(
                        condition,
                        source.card.id,
                        source.controller,
                        TriggerContext::empty(),
                        None,
                        None,
                    );
                self.collect_static_play_restrictions(
                    *then,
                    source,
                    affected_player,
                    condition_holds,
                    component_order,
                    restrictions,
                );
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

    fn static_player_recipient_matches(
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
            EffectRecipientSetDef::Players(PlayerSetDef::Related(relation)) => self
                .player_relation_matches(
                    affected_player,
                    relation,
                    source.controller,
                    TriggerContext::empty(),
                ),
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::Objects(_)
            | EffectRecipientSetDef::Players(
                PlayerSetDef::LegalTargets(_)
                | PlayerSetDef::One(
                    PlayerRefDef::EventPlayer
                    | PlayerRefDef::Target(_)
                    | PlayerRefDef::ControllerOf(_)
                    | PlayerRefDef::OwnerOf(_),
                ),
            ) => false,
        }
    }

    /// Visits static and resolved rule leaves in timestamp/component order.
    /// Static rules remain source-derived; resolved rules use the same stored
    /// expiration path as resolved characteristic operations.
    pub(super) fn visit_applied_rules(
        &self,
        affected: &Permanent,
        mut visitor: impl FnMut(AppliedRuleEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let mut rules = affected
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| {
                let ResolvedContinuousEffectKind::Rule(rule) = effect.kind else {
                    return None;
                };
                Some(AppliedRuleEffect {
                    source: effect.source.object,
                    timestamp: effect.timestamp,
                    component_order: effect.component_order,
                    rule,
                })
            })
            .collect::<Vec<_>>();
        let static_result = self.visit_static_applied_effects(affected, |applied| {
            if let AppliedEffectDef::Rule(rule) = applied.effect {
                rules.push(AppliedRuleEffect {
                    source: applied.source,
                    timestamp: applied.timestamp,
                    component_order: applied.component_order,
                    rule,
                });
            }
            ControlFlow::Continue(())
        });
        debug_assert!(static_result.is_continue());
        rules.sort_by_key(|effect| (effect.timestamp, effect.component_order));
        for rule in rules {
            if visitor(rule).is_break() {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn has_applied_rule(&self, affected: &Permanent, expected: AppliedRuleDef) -> bool {
        self.visit_applied_rules(affected, |applied| {
            if applied.rule == expected {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    pub(super) fn visit_static_applied_effects(
        &self,
        affected: &Permanent,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // Emblems sit outside every zone but their abilities apply, so they
        // are walked alongside the battlefield and nowhere else.
        let land_type_sources = self.land_type_effect_sources(None);
        for source in self.battlefield.iter().chain(self.emblems.iter()) {
            let Some(rules) = self.effective_rules(source) else {
                continue;
            };
            let (source_definition, source_part) = Self::effective_rules_source(source);
            let supplies_static_effect = rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                    && ability.declarative_effect().is_some()
            });
            if !supplies_static_effect {
                continue;
            }
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
                    AbilityOrigin::Printed {
                        definition: source_definition,
                        part: source_part,
                        ability: attached.id,
                    },
                ) {
                    continue;
                }
                let Some(effect) = attached.definition.declarative_effect() else {
                    continue;
                };
                let mut traversal = StaticEffectTraversal {
                    source,
                    source_timestamp: source.timestamp,
                    source_definition,
                    source_part,
                    source_ability: attached.id,
                    affected,
                    prospective: None,
                    next_grant: 0,
                    next_component_order: 0,
                };
                if self
                    .visit_static_effect(effect, &mut traversal, &mut visitor)
                    .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn visit_static_applied_effects_with_prospective(
        &self,
        affected: &Permanent,
        prospective: &Permanent,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let prospective_source = (prospective.card.id == affected.card.id).then_some(prospective);
        let land_type_sources = self.land_type_effect_sources(prospective_source);
        for source in self.battlefield.iter().chain(prospective_source) {
            let Some(rules) = self.effective_rules(source) else {
                continue;
            };
            let (source_definition, source_part) = Self::effective_rules_source(source);
            let supplies_static_effect = rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                    && ability.declarative_effect().is_some()
            });
            if !supplies_static_effect {
                continue;
            }
            let rules_text_removed =
                self.rules_text_abilities_removed_from_sources(source, &land_type_sources);
            if rules_text_removed {
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
                    AbilityOrigin::Printed {
                        definition: source_definition,
                        part: source_part,
                        ability: attached.id,
                    },
                ) {
                    continue;
                }
                let Some(effect) = attached.definition.declarative_effect() else {
                    continue;
                };
                let mut traversal = StaticEffectTraversal {
                    source,
                    source_timestamp: if prospective_source
                        .is_some_and(|prospective| std::ptr::eq(source, prospective))
                    {
                        self.prospective_continuous_effect_timestamp()
                    } else {
                        source.timestamp
                    },
                    source_definition,
                    source_part,
                    source_ability: attached.id,
                    affected,
                    prospective: prospective_source,
                    next_grant: 0,
                    next_component_order: 0,
                };
                if self
                    .visit_static_effect(effect, &mut traversal, &mut visitor)
                    .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn visit_static_effect(
        &self,
        effect: EffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.visit_static_effect_if(effect, traversal, true, visitor)
    }

    fn visit_static_effect_if(
        &self,
        effect: EffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        enabled: bool,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    if self
                        .visit_static_effect_if(*effect, traversal, enabled, visitor)
                        .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            EffectDef::IfCondition { condition, then } => {
                let condition_holds = enabled
                    && self.trigger_condition_holds(
                        condition,
                        traversal.source.card.id,
                        traversal.source.controller,
                        TriggerContext::empty(),
                        None,
                        None,
                    );
                self.visit_static_effect_if(*then, traversal, condition_holds, visitor)
            }
            EffectDef::StaticApply { recipient, effect } => {
                // Traverse the whole applied-effect structure even when this
                // recipient does not match. Grant IDs identify structural
                // grant sites, so later grants must not be renumbered by
                // which permanent happens to be queried.
                let include_effect = enabled
                    && self.static_recipient_matches(
                        recipient,
                        traversal.source,
                        traversal.affected,
                        traversal.prospective,
                    );
                Self::visit_static_applied_effect_components(
                    effect,
                    traversal,
                    include_effect,
                    visitor,
                )
            }
            _ => ControlFlow::Continue(()),
        }
    }

    pub(super) fn visit_static_applied_effect_components(
        effect: AppliedEffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        include_effect: bool,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    if Self::visit_static_applied_effect_components(
                        *effect,
                        traversal,
                        include_effect,
                        visitor,
                    )
                    .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => {
                let component_order = traversal.next_component_order;
                traversal.next_component_order = traversal
                    .next_component_order
                    .checked_add(1)
                    .expect("one static ability contains at most 65,536 applied components");
                let grant = if matches!(
                    effect,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                        AbilityOperationDef::Add(_)
                    ))
                ) {
                    let grant = GrantId::from_index(traversal.next_grant)
                        .expect("one static ability contains at most 256 grant sites");
                    traversal.next_grant += 1;
                    Some(grant)
                } else {
                    None
                };
                if include_effect {
                    visitor(StaticAppliedEffect {
                        source: traversal.source.card.id,
                        timestamp: traversal.source_timestamp,
                        source_definition: traversal.source_definition,
                        source_part: traversal.source_part,
                        source_ability: traversal.source_ability,
                        grant,
                        component_order,
                        effect,
                    })
                } else {
                    ControlFlow::Continue(())
                }
            }
        }
    }

    /// Whether this permanent has the shared Aura attachment spell effect.
    /// An Aura attaches as its own spell resolves, which is what separates it
    /// from Equipment: both attach, but only one does so from a spell clause,
    /// and only one goes to the graveyard when it comes loose.
    pub(super) fn is_aura_permanent(&self, permanent: &Permanent) -> bool {
        self.effective_rules(permanent).is_some_and(|rules| {
            rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Spell(_))
                    && ability
                        .declarative_effect()
                        .and_then(Self::immediate_attachment_target)
                        .is_some()
            })
        })
    }

    /// Finds the target an Aura attaches to as part of its spell procedure.
    /// A sequence remains one immediate resolution procedure, so an Attach
    /// nested in one keeps the clause's target scope.
    pub(super) fn immediate_attachment_target(effect: EffectDef) -> Option<TargetIndex> {
        match effect {
            EffectDef::Attach { object } => object.legal_target(),
            EffectDef::Sequence(effects) => effects
                .iter()
                .find_map(|effect| Self::immediate_attachment_target(*effect)),
            EffectDef::IfFormat {
                then, otherwise, ..
            } => Self::immediate_attachment_target(*then)
                .or_else(|| Self::immediate_attachment_target(*otherwise)),
            other => {
                debug_assert!(Self::effect_never_attaches(other));
                None
            }
        }
    }

    /// Every effect that cannot attach anything. Listed exhaustively so a
    /// new effect has to be classified rather than silently answering None.
    fn effect_never_attaches(effect: EffectDef) -> bool {
        matches!(
            effect,
            EffectDef::None
                | EffectDef::Randomized { .. }
                | EffectDef::Choose(_)
                | EffectDef::PayOr(_)
                | EffectDef::SplitIntoPiles(_)
                | EffectDef::PreventDamage { .. }
                | EffectDef::AddMana(_)
                | EffectDef::AddManaEqualTo { .. }
                | EffectDef::DealDamage { .. }
                | EffectDef::DrainLife { .. }
                | EffectDef::GainLife { .. }
                | EffectDef::AddPoisonCounters { .. }
                | EffectDef::DrawCards { .. }
                | EffectDef::Discard { .. }
                | EffectDef::DiscardCards { .. }
                | EffectDef::ShuffleLibrary { .. }
                | EffectDef::EmptyManaPool { .. }
                | EffectDef::LoseLife { .. }
                | EffectDef::LoseTheGame { .. }
                | EffectDef::Regenerate { .. }
                | EffectDef::Tap { .. }
                | EffectDef::RemoveFromCombat { .. }
                | EffectDef::DestroyAtEndOfCombat { .. }
                | EffectDef::SkipNextUntapSteps { .. }
                | EffectDef::RemoveAllCounters { .. }
                | EffectDef::Untap { .. }
                | EffectDef::Destroy { .. }
                | EffectDef::Sacrifice { .. }
                | EffectDef::SacrificeOfChoice { .. }
                | EffectDef::Mill { .. }
                | EffectDef::LookAtTopAndSelect { .. }
                | EffectDef::LookAtHand { .. }
                | EffectDef::SearchZone { .. }
                | EffectDef::ChooseCards { .. }
                | EffectDef::ReplaceNextDrawThisTurn { .. }
                | EffectDef::CreateEmblem { .. }
                | EffectDef::Transform { .. }
                | EffectDef::Counter { .. }
                | EffectDef::AddCounters { .. }
                | EffectDef::ChangeTextBasicLandType { .. }
                | EffectDef::BecomeCopyOf { .. }
                | EffectDef::May { .. }
                | EffectDef::ScheduleTurnPhases(_)
                | EffectDef::TakeExtraTurn { .. }
                | EffectDef::GrantFlashToNextSorcery
                | EffectDef::ExileLinkedToSource { .. }
                | EffectDef::ReturnLinkedExiles { .. }
                | EffectDef::Detain { .. }
                | EffectDef::GainControl { .. }
                | EffectDef::IfCondition { .. }
                | EffectDef::InstallTrigger(_)
                | EffectDef::CannotBeForcedToSacrifice
                | EffectDef::ReduceGenericCostBy(_)
                | EffectDef::LandwalkCanBeBlocked(_)
                | EffectDef::CannotAttackUnless(_)
                | EffectDef::MoveToZone { .. }
                | EffectDef::Attach { .. }
                | EffectDef::CreateToken { .. }
                | EffectDef::CreateTokenCopyOf { .. }
                | EffectDef::StaticApply { .. }
                | EffectDef::Apply { .. }
                | EffectDef::Special(_)
        )
    }

    /// Whether this Aura prints the exception that keeps it attached through
    /// protection. Only an Aura granting protection needs it, and it exempts
    /// that Aura alone rather than weakening the protection itself.
    pub(super) fn remains_attached_through_protection(&self, aura: &Permanent) -> bool {
        self.has_applied_rule(aura, AppliedRuleDef::RemainsAttachedThroughProtection)
    }

    /// Whether a static effect forbids Auras on this permanent. This is not a
    /// targeting restriction like hexproof: it also makes an Aura that somehow
    /// arrived anyway fall off.
    pub(super) fn cannot_be_enchanted(&self, permanent: &Permanent) -> bool {
        self.has_applied_rule(permanent, AppliedRuleDef::CannotBeEnchanted)
    }

    /// Whether a static ability keeps the turn-based untap action from
    /// untapping this permanent. This does not affect an explicit untap
    /// effect. The source may be the permanent itself or another permanent
    /// applying the restriction globally.
    pub(super) fn does_not_untap_during_untap_step(&self, permanent: &Permanent) -> bool {
        if self
            .find_effective_ability(permanent, |effective| {
                effective.ability.is_executable()
                    && matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::Static(_)
                    )
                    && effective
                        .ability
                        .declarative_effect()
                        .is_some_and(|effect| {
                            Self::static_effect_contains_applied_effect(
                                effect,
                                AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                            )
                        })
            })
            .is_some()
        {
            return true;
        }
        self.visit_applied_rules(permanent, |applied| {
            if applied.rule == AppliedRuleDef::DoesNotUntapDuringUntapStep {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    pub(super) fn may_choose_not_to_untap(&self, permanent: &Permanent) -> bool {
        if self
            .find_effective_ability(permanent, |effective| {
                effective.ability.is_executable()
                    && matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::Static(_)
                    )
                    && effective
                        .ability
                        .declarative_effect()
                        .is_some_and(|effect| {
                            Self::static_effect_contains_applied_effect(
                                effect,
                                AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
                            )
                        })
            })
            .is_some()
        {
            return true;
        }
        self.visit_applied_rules(permanent, |applied| {
            if applied.rule == AppliedRuleDef::MayChooseNotToUntap {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    fn static_effect_contains_applied_effect(
        effect: EffectDef,
        expected: AppliedEffectDef,
    ) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(|effect| Self::static_effect_contains_applied_effect(effect, expected)),
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect,
            } => Self::applied_effect_contains(effect, expected),
            _ => false,
        }
    }

    /// Whether a new Aura may attach to this permanent. The narrower
    /// Guardian Beast prohibition joins the general one here, while
    /// `cannot_be_enchanted` remains the state-based check for Auras that are
    /// already attached.
    pub(super) fn cannot_become_enchanted(&self, permanent: &Permanent) -> bool {
        self.visit_applied_rules(permanent, |applied| {
            if matches!(
                applied.rule,
                AppliedRuleDef::CannotBeEnchanted | AppliedRuleDef::CannotBecomeEnchanted
            ) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    /// Whether a continuous effect stops another player from taking this
    /// permanent. Callers still allow a no-op assignment to its current
    /// controller.
    pub(super) fn cannot_change_controller(&self, permanent: &Permanent) -> bool {
        self.has_applied_rule(permanent, AppliedRuleDef::CannotChangeController)
    }

    /// Whether an Aura may stay attached to `host`: the host has to still be
    /// on the battlefield and still satisfy what the Aura enchants.
    pub(super) fn is_legal_aura_host(&self, aura: &Permanent, host: GameObjectId) -> bool {
        let Some(host) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
        else {
            return false;
        };
        if self.cannot_be_enchanted(host) {
            return false;
        }
        if self.effective_rules(aura).is_none() {
            return false;
        }
        let aura_colors = self.permanent_colors(aura);
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        let Some(target) = rules.ability_clauses().iter().find_map(|ability| {
            let target = Self::immediate_attachment_target(ability.declarative_effect()?)?;
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                return None;
            };
            spell.targets().get(target.index())
        }) else {
            return false;
        };
        match target.predicate {
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                        && controller.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.controller,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && owner.is_none_or(|relation| {
                            self.player_relation_matches(
                                host.card.owner,
                                relation,
                                aura.controller,
                                TriggerContext::empty(),
                            )
                        })
                        && self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(host),
                            aura.card.id,
                            false,
                        )
                        // Hexproof only constrains targeting. Protection also
                        // makes an existing attachment illegal, unless this
                        // Aura is the one printing the exception -- which is
                        // what an Aura granting protection from its own color
                        // has to do to survive its own effect.
                        && (self.remains_attached_through_protection(aura)
                            || !self.is_protected_from_colors(host, aura_colors))
            }
            AbilityTargetPredicate::AnyTarget
            | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
            | AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::Player(_) => false,
        }
    }

    /// The permanent an Aura spell targeted, read off its own spell clause.
    pub(super) fn aura_host_for(object: &StackObject) -> Option<GameObjectId> {
        let ability = object.ability.as_ref()?;
        let primary = match ability.resolver {
            StackAbilityResolver::Declarative(effect)
            | StackAbilityResolver::DeclarativeWithCustomFollowup { effect, .. } => Some(effect),
            StackAbilityResolver::Custom(_) | StackAbilityResolver::CardOwned(_) => None,
        };
        primary
            .into_iter()
            .chain(ability.mode_effects.iter().copied())
            .find_map(|scoped| {
                let target = Self::immediate_attachment_target(scoped.effect)?;
                Self::chosen_targets(object, scoped.target_slot(target)).find_map(|target| {
                    match target {
                        Target::Permanent(id) => Some(id),
                        _ => None,
                    }
                })
            })
    }

    pub(super) fn attached_host(&self, aura: GameObjectId) -> Option<GameObjectId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == aura)
            .and_then(|permanent| permanent.attached_to)
    }

    pub(super) fn static_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> bool {
        match recipient.0 {
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::Source)) => {
                source.card.id == affected.card.id
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::AttachedToSource)) => {
                source.attached_to == Some(affected.card.id)
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
                query.zones.contains(&ZoneKind::Battlefield)
                    && self.query_player_constraints_match(
                        Some(affected.controller),
                        affected.card.owner,
                        query,
                        source.controller,
                        TriggerContext::empty(),
                        None,
                    )
                    && self.static_object_predicate_matches(
                        query.object,
                        source,
                        affected,
                        prospective,
                    )
            }
            // None of these name a permanent a static effect could apply to;
            // a static effect has no chosen target either.
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::Objects(
                ObjectSetDef::One(
                    ObjectRefDef::Binding(_)
                    | ObjectRefDef::ResolvingObject
                    | ObjectRefDef::Target(_)
                    | ObjectRefDef::TriggeringObject,
                )
                | ObjectSetDef::Binding(_)
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::SharingNameWith(_),
            )
            | EffectRecipientSetDef::Players(_) => false,
        }
    }
}

include!("continuous_effects/characteristics.rs");
include!("continuous_effects/static_predicates.rs");
