mod player_rules;
mod untap_limits;

use std::cell::Cell;

#[cfg(test)]
use super::{AbilityId, AbilityOrigin, ObjectCharacteristics};
use super::{
    AbilityOperationDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    AppliedRuleEffect, CardDefinitionId, CardPartId, CardRules, CardSet, CardType, CardTypeSet,
    CharacteristicContext, CharacteristicOperationDef, ColorSet, ContinuousEffectExpiration,
    ControlFlow, DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EffectRecipientSetDef, Game,
    GameObjectId, GrantId, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectRefDef,
    ObjectSetDef, Permanent, PlayerId, PlayerRelation, ResolvedContinuousEffect,
    ResolvedContinuousEffectKind, RetiredObject, SetOperationDef, StackAbilityResolver,
    StackObject, StaticAppliedEffect, StaticEffectTraversal, Target, TargetIndex,
    TriggerConditionDef, TriggerContext, TriggerEventObject, ZoneKind,
};

thread_local! {
    /// Guards the live set-characteristic walk when a static recipient query
    /// asks for the same characteristics being assembled.
    static STATIC_SET_CHARACTERISTIC_LAYER_PASS: Cell<bool> = const { Cell::new(false) };
}

pub(super) struct StaticSetCharacteristicLayerGuard;

impl StaticSetCharacteristicLayerGuard {
    pub(super) fn enter() -> Option<Self> {
        STATIC_SET_CHARACTERISTIC_LAYER_PASS
            .with(|pass| if pass.replace(true) { None } else { Some(Self) })
    }
}

impl Drop for StaticSetCharacteristicLayerGuard {
    fn drop(&mut self) {
        STATIC_SET_CHARACTERISTIC_LAYER_PASS.with(|pass| pass.set(false));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StaticEffectKind {
    Any,
    Rules,
    CardTypes,
    Colors,
    Abilities,
    Subtypes,
    PowerToughness,
}

impl StaticEffectKind {
    const fn includes(self, effect: AppliedEffectDef) -> bool {
        matches!(
            (self, effect),
            (
                Self::Any,
                AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_)
            ) | (Self::Rules, AppliedEffectDef::Rule(_))
                | (
                    Self::CardTypes,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(_)),
                )
                | (
                    Self::Colors,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(_)),
                )
                | (
                    Self::Abilities,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(_)),
                )
                | (
                    Self::Subtypes,
                    AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::CreatureTypes(_)
                            | CharacteristicOperationDef::Subtypes(_),
                    ),
                )
                | (
                    Self::PowerToughness,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(_)),
                )
        )
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

    pub(super) fn continuous_effect_expiration_is_active(
        &self,
        expiration: ContinuousEffectExpiration,
        source: GameObjectId,
    ) -> bool {
        match expiration {
            ContinuousEffectExpiration::WhileSourceTapped => self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == source && permanent.tapped),
            // The same question with the tap dropped: a source that has left
            // takes what it did with it, and one that came back is a new
            // object that never did it.
            ContinuousEffectExpiration::WhileSourceRemains => self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == source),
            ContinuousEffectExpiration::EndOfTurn
            | ContinuousEffectExpiration::EndOfCombat
            | ContinuousEffectExpiration::UpkeepOf(_)
            | ContinuousEffectExpiration::TurnOf { .. }
            | ContinuousEffectExpiration::Never => true,
        }
    }

    /// Whether anything currently forbids activating this permanent's
    /// activated abilities. Read live off the permanent, so an Aura leaving
    /// gives the abilities straight back.
    pub(super) fn activated_abilities_are_prohibited(&self, permanent: &Permanent) -> bool {
        self.visit_applied_rules(permanent, |applied| {
            if applied.rule == AppliedRuleDef::CannotActivateAbilities {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    /// What a creature assigns in combat. Ordinarily its power; a creature
    /// under "assigns combat damage equal to its toughness rather than its
    /// power" assigns that instead, which changes neither number.
    pub(super) fn combat_assigned_power(&self, permanent: &Permanent) -> Option<i16> {
        let assigns_toughness = self
            .visit_applied_rules(permanent, |applied| {
                if applied.rule == AppliedRuleDef::AssignsCombatDamageEqualToToughness {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .is_break();
        if assigns_toughness {
            self.toughness(permanent).or_else(|| self.power(permanent))
        } else {
            self.power(permanent)
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
        let static_result =
            self.visit_static_applied_effects(affected, StaticEffectKind::Rules, |applied| {
                let AppliedEffectDef::Rule(rule) = applied.effect else {
                    unreachable!("the static rule filter admits only rules");
                };
                rules.push(AppliedRuleEffect {
                    source: applied.source,
                    timestamp: applied.timestamp,
                    component_order: applied.component_order,
                    rule,
                });
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
        kind: StaticEffectKind,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // Emblems sit outside every zone but their abilities apply, so they
        // are walked alongside the battlefield and nowhere else.
        let land_type_sources = self.land_type_effect_sources(None);
        for source in self.battlefield.iter().chain(self.emblems.iter()) {
            if self.rules_text_abilities_removed_from_sources(source, &land_type_sources) {
                continue;
            }
            if self
                .visit_static_source_effects(
                    StaticEffectSource::battlefield(source, source.timestamp),
                    affected,
                    None,
                    kind,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        for source in self.graveyard_static_sources() {
            if self
                .visit_static_source_effects(
                    StaticEffectSource::graveyard(&source),
                    affected,
                    None,
                    kind,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn visit_static_applied_effects_with_prospective(
        &self,
        affected: &Permanent,
        prospective: &Permanent,
        kind: StaticEffectKind,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let prospective_source = (prospective.card.id == affected.card.id).then_some(prospective);
        let land_type_sources = self.land_type_effect_sources(prospective_source);
        for source in self.battlefield.iter().chain(prospective_source) {
            let rules_text_removed =
                self.rules_text_abilities_removed_from_sources(source, &land_type_sources);
            if rules_text_removed {
                continue;
            }
            let timestamp = if prospective_source
                .is_some_and(|prospective| std::ptr::eq(source, prospective))
            {
                self.prospective_continuous_effect_timestamp()
            } else {
                source.timestamp
            };
            if self
                .visit_static_source_effects(
                    StaticEffectSource::battlefield(source, timestamp),
                    affected,
                    prospective_source,
                    kind,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        for source in self.graveyard_static_sources() {
            if self
                .visit_static_source_effects(
                    StaticEffectSource::graveyard(&source),
                    affected,
                    prospective_source,
                    kind,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn visit_static_effect(
        &self,
        effect: EffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        kind: StaticEffectKind,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.visit_static_effect_tree(effect, traversal, &mut Vec::new(), kind, visitor)
    }

    fn visit_static_effect_tree(
        &self,
        effect: EffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        conditions: &mut Vec<&'static TriggerConditionDef>,
        kind: StaticEffectKind,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    if self
                        .visit_static_effect_tree(*effect, traversal, conditions, kind, visitor)
                        .is_break()
                    {
                        return ControlFlow::Break(());
                    }
                }
                ControlFlow::Continue(())
            }
            EffectDef::IfCondition { condition, then } => {
                conditions.push(condition);
                let result =
                    self.visit_static_effect_tree(*then, traversal, conditions, kind, visitor);
                conditions.pop();
                result
            }
            EffectDef::StaticApply { recipient, effect } => {
                // CR 613.6 keeps one recipient set for every component of a
                // continuous effect once that effect starts to apply. A
                // compound animation therefore chooses its objects in layer
                // 4 and keeps those same objects for its layer-7 body, even
                // though its own type component has made them creatures by
                // then. Re-entering the static set-characteristic walk while
                // matching the recipient would instead let the later layer
                // observe the animation itself and switch its own body off.
                //
                // The existing guarded layer-below view is also the honest
                // boundary for dependencies the engine does not yet model:
                // other static set-characteristic transformations are not
                // guessed into the answer. Resolved and structural changes
                // remain visible, as they are below this live static walk.
                let _type_layer_selection = Self::applied_effect_starts_in_type_layer(effect)
                    .then(StaticSetCharacteristicLayerGuard::enter)
                    .flatten();
                // Traverse the whole applied-effect structure even when this
                // recipient does not match. Grant IDs identify structural
                // grant sites, so later grants must not be renumbered by
                // which permanent happens to be queried.
                let recipient_matches = self.static_recipient_matches(
                    recipient,
                    traversal.source,
                    traversal.affected,
                    traversal.prospective,
                );
                self.visit_static_applied_effect_components(
                    effect,
                    traversal,
                    recipient_matches,
                    conditions,
                    kind,
                    visitor,
                )
            }
            _ => ControlFlow::Continue(()),
        }
    }

    /// Whether this one applied effect begins choosing its recipients in
    /// layer 4. Later components keep that selection under CR 613.6.
    fn applied_effect_starts_in_type_layer(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_starts_in_type_layer),
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::ChosenBasicLandType
                | CharacteristicOperationDef::BasicLandTypes(_)
                | CharacteristicOperationDef::CardTypes(_)
                | CharacteristicOperationDef::CreatureTypes(_)
                | CharacteristicOperationDef::Subtypes(_),
            ) => true,
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::Abilities(_)
                | CharacteristicOperationDef::Colors(_)
                | CharacteristicOperationDef::PowerToughness(_),
            )
            | AppliedEffectDef::Rule(_) => false,
        }
    }

    pub(super) fn visit_static_applied_effect_components(
        &self,
        effect: AppliedEffectDef,
        traversal: &mut StaticEffectTraversal<'_>,
        recipient_matches: bool,
        conditions: &[&'static TriggerConditionDef],
        kind: StaticEffectKind,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    if self
                        .visit_static_applied_effect_components(
                            *effect,
                            traversal,
                            recipient_matches,
                            conditions,
                            kind,
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
                // Resolve the recipient and component layer before checking
                // surrounding conditions. A source-only conditional static
                // should not inspect live board state while an unrelated
                // permanent or continuous-effect layer is being assembled.
                let include_effect = recipient_matches
                    && kind.includes(effect)
                    && conditions.iter().all(|condition| {
                        self.trigger_condition_holds(
                            condition,
                            traversal.source.card.id,
                            traversal.source.controller,
                            TriggerContext::empty(),
                            None,
                            None,
                        )
                    });
                if include_effect {
                    visitor(StaticAppliedEffect {
                        source: traversal.source.card.id,
                        timestamp: traversal.source_timestamp,
                        source_presentation: traversal.source_presentation,
                        source_origin: traversal.source_origin,
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

    /// Whether this permanent is currently an Aura. The effective subtype
    /// gate makes an Aura that loses the subtype become unattached and stay
    /// on the battlefield. The attachment-semantic gate preserves the window
    /// for permanents such as Necromancy: its model carries the eventual Aura
    /// subtype, but its own trigger has not made it an Aura until it attaches.
    /// Bestow (CR 702.103c): a permanent cast for its bestow cost is an Aura
    /// for exactly as long as it is attached to something. Structural rather
    /// than a clause the card writes, the same way impending is: it follows
    /// from how the spell was paid for, and the permanent records that.
    pub(super) fn is_bestowed_aura(permanent: &Permanent) -> bool {
        permanent.cast_alternative == Some(crate::card::AlternativeCastKindDef::Bestow)
            && permanent.attached_to.is_some()
    }

    pub(super) fn is_aura_permanent(&self, permanent: &Permanent) -> bool {
        if Self::is_bestowed_aura(permanent) {
            return true;
        }
        self.effective_subtypes(permanent).contains(&"Aura")
            && self.effective_rules(permanent).is_some_and(|rules| {
                (rules.enchant().is_some() && permanent.became_aura)
                    || rules.ability_clauses().iter().any(|ability| {
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
                | EffectDef::SimultaneousChoose(_)
                | EffectDef::ChooseCardName { .. }
                | EffectDef::BindMatching { .. }
                | EffectDef::SelectAtRandomFromZone { .. }
                | EffectDef::PayOr(_)
                | EffectDef::SplitIntoPiles(_)
                | EffectDef::PreventDamage { .. }
                | EffectDef::AddMana(_)
                | EffectDef::AddManaEqualTo { .. }
                | EffectDef::DealDamage { .. }
                | EffectDef::DealDamageFrom { .. }
                | EffectDef::DealDamageAndApply { .. }
                | EffectDef::DrainLife { .. }
                | EffectDef::GainLife { .. }
                | EffectDef::SetLifeTotal { .. }
                | EffectDef::AddPlayerCounters { .. }
                | EffectDef::DrawCards { .. }
                | EffectDef::Discard { .. }
                | EffectDef::DiscardCards { .. }
                | EffectDef::ShuffleLibrary { .. }
                | EffectDef::BuryGraveyard { .. }
                | EffectDef::EmptyManaPool { .. }
                | EffectDef::LoseLife { .. }
                | EffectDef::LoseTheGame { .. }
                | EffectDef::WinTheGame { .. }
                | EffectDef::Regenerate { .. }
                | EffectDef::Tap { .. }
                | EffectDef::RemoveFromCombat { .. }
                | EffectDef::SkipNextUntapSteps { .. }
                | EffectDef::DoubleCounters { .. }
                | EffectDef::RemoveAllCounters { .. }
                | EffectDef::Untap { .. }
                | EffectDef::Saddle { .. }
                | EffectDef::Destroy { .. }
                | EffectDef::Sacrifice { .. }
                | EffectDef::SacrificeOfChoice { .. }
                | EffectDef::ExileTopOfLibraryToPlay { .. }
                | EffectDef::Mill { .. }
                | EffectDef::SearchZonesAndExileRest { .. }
                | EffectDef::MillUntil { .. }
                | EffectDef::LookAtTopAndSelect { .. }
                | EffectDef::LookAtTopAndDistribute { .. }
                | EffectDef::LookAtHand { .. }
                | EffectDef::RevealHand { .. }
                | EffectDef::SearchZone { .. }
                | EffectDef::ChooseCards { .. }
                | EffectDef::ReplaceNextDrawThisTurn { .. }
                | EffectDef::CreateEmblem { .. }
                | EffectDef::CreateOngoingEffect(_)
                | EffectDef::Transform { .. }
                | EffectDef::Unattach { .. }
                | EffectDef::Attach { .. }
                | EffectDef::Counter { .. }
                | EffectDef::CopyStackObject(_)
                | EffectDef::AddCounters { .. }
                | EffectDef::ChooseCounterKind { .. }
                | EffectDef::ChooseEffect { .. }
                | EffectDef::ModifyCounters { .. }
                | EffectDef::RemoveCounters { .. }
                | EffectDef::ChangeTextBasicLandType { .. }
                | EffectDef::ChooseColor { .. }
                | EffectDef::BecomeCopyOf { .. }
                | EffectDef::May { .. }
                | EffectDef::ScheduleTurnPhases(_)
                | EffectDef::TakeExtraTurn { .. }
                | EffectDef::PutSourceOntoBattlefieldAttacking
                | EffectDef::BecomeMonarch { .. }
                | EffectDef::VoteForPermanentToExile { .. }
                | EffectDef::DamageCannotBePreventedThisTurn
                | EffectDef::GrantFlashToNextSorcery
                | EffectDef::ExileLinkedToSource { .. }
                | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
                | EffectDef::MayPlayWithoutPaying { .. }
                | EffectDef::ReturnLinkedExiles { .. }
                | EffectDef::Detain { .. }
                | EffectDef::GainControl { .. }
                | EffectDef::IfCondition { .. }
                | EffectDef::InstallTrigger(_)
                | EffectDef::CannotBeForcedToSacrifice
                | EffectDef::CannotBeForcedToDiscard
                | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
                | EffectDef::ReduceGenericCostBy(_)
                | EffectDef::ModifyCost(_)
                | EffectDef::LandwalkCanBeBlocked(_)
                | EffectDef::CannotAttackUnless(_)
                | EffectDef::PutIntoLibraryBeneathTop { .. }
                | EffectDef::MoveToZone { .. }
                | EffectDef::PhaseOut { .. }
                | EffectDef::CreateToken { .. }
                | EffectDef::CreateAttachedToken { .. }
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

    /// What an Aura spell targeted, read off its own spell clause. Usually a
    /// permanent; an Aura whose slot names a card in a zone -- "enchant
    /// creature card in a graveyard" -- comes back with that card instead.
    pub(super) fn aura_host_for(object: &StackObject) -> Option<GameObjectId> {
        let ability = object.ability.as_ref()?;
        let primary = match ability.resolver {
            StackAbilityResolver::Declarative(effect)
            | StackAbilityResolver::DeclarativeIgnoringTargetFizzle(effect)
            | StackAbilityResolver::DeclarativeWithCustomFollowup { effect, .. } => Some(effect),
            StackAbilityResolver::Custom(_)
            | StackAbilityResolver::CardOwned(_)
            | StackAbilityResolver::CastOffer(_) => None,
        };
        primary
            .into_iter()
            .chain(ability.mode_effects.iter().copied())
            .find_map(|scoped| {
                let target = Self::immediate_attachment_target(scoped.effect)?;
                Self::chosen_targets(object, scoped.target_slot(target)).find_map(|target| {
                    match target {
                        Target::Permanent(id) | Target::Card(id) => Some(id),
                        Target::Player(_) | Target::Spell(_) => None,
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
            // a static effect has no chosen target either, and the mixed
            // recipient belongs to a resolving damage clause.
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
            | EffectRecipientSetDef::Objects(
                ObjectSetDef::One(
                    ObjectRefDef::Binding(_)
                    | ObjectRefDef::ZoneChangeSuccessor(_)
                    | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                    | ObjectRefDef::ResolvingObject
                    | ObjectRefDef::AdditionalCostObject(_)
                    | ObjectRefDef::AbilityGrantSource
                    | ObjectRefDef::Target(_)
                    | ObjectRefDef::SourceOfTargetedStackObject(_)
                    | ObjectRefDef::TriggeringObject
                    | ObjectRefDef::DamagedObject,
                )
                | ObjectSetDef::Binding(_)
                | ObjectSetDef::MatchingBinding { .. }
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::PermanentsTargetedBy(_)
                | ObjectSetDef::LinkedExiles(_)
                | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                | ObjectSetDef::BottomOfGraveyard(_)
                | ObjectSetDef::SharingNameWith(_)
                | ObjectSetDef::SharingNameWithBinding { .. }
                | ObjectSetDef::TopOfGraveyardMatching { .. },
            )
            // A static clause names what it affects outright; nothing static
            // is scoped to what a creature happens to be attacking.
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::Players(_) => false,
        }
    }
}

include!("continuous_effects/aura_hosts.rs");
include!("continuous_effects/characteristics.rs");
include!("continuous_effects/graveyard_sources.rs");
include!("continuous_effects/static_predicates.rs");
include!("continuous_effects/player_auras.rs");
