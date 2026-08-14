#[cfg(test)]
use super::{AbilityId, AbilityOrigin};
use super::{
    AbilityTargetPredicate, AppliedEffectDef, CardDefinitionId, CardRules, CardSet, CardTypeSet,
    CharacteristicSource, ColorSet, ControlFlow, DeclarativeAbilityDef, EffectDef,
    EffectDurationDef, EffectRecipientDef, EntryCompletion, Game, GameObjectId, GrantId,
    KeywordAbility, ManaColor, PendingBattlefieldEntry, Permanent, PlayerId, RetiredObject,
    StackAbilityResolver, StackObject, StaticAppliedEffect, StaticEffectTraversal, Target,
    TargetIndex, TriggerContext, ZoneKind,
};

impl Game {
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
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => {
                // Traverse the whole applied-effect structure even when this
                // recipient does not match. Grant IDs identify structural
                // grant sites, so later grants must not be renumbered by
                // which permanent happens to be queried.
                let include_effect = enabled
                    && matches!(
                        duration,
                        EffectDurationDef::WhileSourceRemainsInZone
                            | EffectDurationDef::UntilSourceLeavesZone
                    )
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
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::DoesNotUntapDuringUntapStep
            | AppliedEffectDef::MayChooseNotToUntap
            | AppliedEffectDef::CannotBeEnchanted
            | AppliedEffectDef::CannotBecomeEnchanted
            | AppliedEffectDef::CannotChangeController
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::PreventDamageFrom(_)
            | AppliedEffectDef::PreventCombatDamage
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::SetLandTypes(_)
            | AppliedEffectDef::Animate(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::GrantAbility(_)
            | AppliedEffectDef::RemoveAbilities(_)
            | AppliedEffectDef::Special(_) => {
                let grant = if matches!(effect, AppliedEffectDef::GrantAbility(_)) {
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
                        effect,
                    })
                } else {
                    ControlFlow::Continue(())
                }
            }
        }
    }

    /// Whether this permanent has the shared Aura attachment spell effect.
    pub(super) fn is_aura_permanent(&self, permanent: &Permanent) -> bool {
        self.effective_rules(permanent).is_some_and(|rules| {
            rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
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
            EffectDef::Attach {
                object: EffectRecipientDef::Target(target),
            } => Some(target),
            EffectDef::Sequence(effects) => effects
                .iter()
                .find_map(|effect| Self::immediate_attachment_target(*effect)),
            EffectDef::IfFormat {
                then, otherwise, ..
            } => Self::immediate_attachment_target(*then)
                .or_else(|| Self::immediate_attachment_target(*otherwise)),
            EffectDef::None
            | EffectDef::Randomized { .. }
            | EffectDef::ChoosePermanent { .. }
            | EffectDef::ChooseDamageSource { .. }
            | EffectDef::PreventNextDamageFromSource { .. }
            | EffectDef::AddMana(_)
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::DealDamage { .. }
            | EffectDef::DrainLife { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::Discard { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::LoseTheGame { .. }
            | EffectDef::Regenerate { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::PreventAllCombatDamageThisTurn
            | EffectDef::PreventNextDamage { .. }
            | EffectDef::PreventAllDamageThisTurn { .. }
            | EffectDef::PreventCombatDamageThisTurn { .. }
            | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
            | EffectDef::PreventDamageToPlayerAndControlledCreaturesThisTurn { .. }
            | EffectDef::PreventAllCombatDamageExceptSourceThisTurn { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::DestroyOfChoice { .. }
            | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
            | EffectDef::RevealAndSplitIntoPiles { .. }
            | EffectDef::Mill { .. }
            | EffectDef::LookAtTopAndMayTake { .. }
            | EffectDef::LookAtTopAndSelect { .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::SearchZone { .. }
            | EffectDef::ChooseCards { .. }
            | EffectDef::ReplaceNextDrawThisTurn { .. }
            | EffectDef::CreateEmblem { .. }
            | EffectDef::Transform { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalPayment { .. }
            | EffectDef::UnlessPaid { .. }
            | EffectDef::May { .. }
            | EffectDef::AdditionalCombatPhase
            | EffectDef::TakeExtraTurn { .. }
            | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::CannotRegenerateThisTurn { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::GainControlWhileSourceRemains { .. }
            | EffectDef::GainControlThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::IfCondition { .. }
            | EffectDef::TriggerUntilYourNextTurn { .. }
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::PlayersCantPlay(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::Replacement(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => None,
        }
    }

    /// Whether a static effect forbids Auras on this permanent. This is not a
    /// targeting restriction like hexproof: it also makes an Aura that somehow
    /// arrived anyway fall off.
    pub(super) fn cannot_be_enchanted(&self, permanent: &Permanent) -> bool {
        self.visit_static_applied_effects(permanent, |applied| {
            if Self::applied_effect_contains(applied.effect, AppliedEffectDef::CannotBeEnchanted) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
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
                                AppliedEffectDef::DoesNotUntapDuringUntapStep,
                            )
                        })
            })
            .is_some()
        {
            return true;
        }
        self.visit_static_applied_effects(permanent, |applied| {
            if applied.source != permanent.card.id
                && Self::applied_effect_contains(
                    applied.effect,
                    AppliedEffectDef::DoesNotUntapDuringUntapStep,
                )
            {
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
                                AppliedEffectDef::MayChooseNotToUntap,
                            )
                        })
            })
            .is_some()
        {
            return true;
        }
        self.visit_static_applied_effects(permanent, |applied| {
            if applied.source != permanent.card.id
                && Self::applied_effect_contains(
                    applied.effect,
                    AppliedEffectDef::MayChooseNotToUntap,
                )
            {
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect,
                duration:
                    EffectDurationDef::WhileSourceRemainsInZone
                    | EffectDurationDef::UntilSourceLeavesZone,
            } => Self::applied_effect_contains(effect, expected),
            _ => false,
        }
    }

    /// Whether a new Aura may attach to this permanent. The narrower
    /// Guardian Beast prohibition joins the general one here, while
    /// `cannot_be_enchanted` remains the state-based check for Auras that are
    /// already attached.
    pub(super) fn cannot_become_enchanted(&self, permanent: &Permanent) -> bool {
        self.visit_static_applied_effects(permanent, |applied| {
            if Self::applied_effect_contains(applied.effect, AppliedEffectDef::CannotBeEnchanted)
                || Self::applied_effect_contains(
                    applied.effect,
                    AppliedEffectDef::CannotBecomeEnchanted,
                )
            {
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
        self.visit_static_applied_effects(permanent, |applied| {
            if Self::applied_effect_contains(
                applied.effect,
                AppliedEffectDef::CannotChangeController,
            ) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
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
        let Some(rules) = self.effective_rules(aura) else {
            return false;
        };
        let aura_colors = rules.colors();
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
                        // makes an existing attachment illegal.
                        && !self.is_protected_from_colors(host, aura_colors)
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

    /// Whether a definition is a token rather than a printed card.
    pub(super) fn is_token(&self, definition: CardDefinitionId) -> bool {
        self.catalog
            .get(definition)
            .is_some_and(|card| card.debut_set == CardSet::Token)
    }

    /// Puts one token onto the battlefield under `controller`.
    ///
    /// A token is a real permanent built from a catalog definition that no
    /// format allows, so it can be looked up and rendered like any other card
    /// while never being deck-legal.
    pub(super) fn create_token(&mut self, controller: PlayerId, token: CardDefinitionId) {
        self.create_token_from(controller, token, None);
    }

    /// Puts one token onto the battlefield, remembering which permanent's
    /// ability made it. Only the cards that later refer to their own tokens
    /// pass a creator; for everything else the link is dead weight.
    pub(super) fn create_token_from(
        &mut self,
        controller: PlayerId,
        token: CardDefinitionId,
        creator: Option<GameObjectId>,
    ) {
        let Some(definition) = self.catalog.get(token) else {
            return;
        };
        let presented = definition.primary_part_id();
        // A token has no physical card behind it, which is exactly what an
        // unbacked object is.
        let card = self.unbacked_object(token, controller, CharacteristicSource::Card(token));
        let mut permanent = Permanent::entering(
            card,
            presented,
            controller,
            self.turns_started[controller.index()],
        );
        permanent.created_by = creator;
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::None,
        });
    }

    /// What an Aura is attached to, if it is on the battlefield and attached.
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
        match recipient {
            EffectRecipientDef::Source => source.card.id == affected.card.id,
            EffectRecipientDef::AttachedPermanent => source.attached_to == Some(affected.card.id),

            EffectRecipientDef::MatchingObjects {
                object,
                zones,
                controller,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                    && self.player_relation_matches(
                        affected.controller,
                        controller,
                        source.controller,
                        TriggerContext::empty(),
                    )
                    && self.trigger_object_matches(
                        object,
                        &prospective.map_or_else(
                            || self.trigger_event_object(affected),
                            |prospective| {
                                self.trigger_event_object_with_prospective(affected, prospective)
                            },
                        ),
                        source.card.id,
                        false,
                    )
            }
            // None of these name a permanent a static effect could apply to;
            // a static effect has no chosen target either.
            EffectRecipientDef::ChosenPermanent(_)
            | EffectRecipientDef::ControllerOfTarget(_)
            | EffectRecipientDef::ObjectsControlledByTarget { .. }
            | EffectRecipientDef::ObjectsOwnedByTarget { .. }
            | EffectRecipientDef::CardsOwnedByTarget { .. }
            | EffectRecipientDef::Controller
            | EffectRecipientDef::Opponent
            | EffectRecipientDef::EachPlayer
            | EffectRecipientDef::Target(_)
            | EffectRecipientDef::ObjectsSharingNameWithTarget(_)
            | EffectRecipientDef::TriggeringObject
            | EffectRecipientDef::ControllerOfTriggeringObject
            | EffectRecipientDef::EventPlayer => false,
        }
    }

    /// Resolves an ordinary activated clause by its printed order. Legacy
    /// aggregate definitions fall back to their historic primary identity;
    /// migrated multi-ability cards retain the exact clause chosen by the
    /// action (for example, Factory's animate and pump abilities).
    #[cfg(test)]
    pub(super) fn activated_ability_origin(
        &self,
        permanent: &Permanent,
        index: usize,
    ) -> AbilityOrigin {
        let mut activated_index = 0;
        self.find_effective_ability(permanent, |effective| {
            if !effective.ability.is_executable()
                || !matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Activated(_)
                )
            {
                return false;
            }
            let matches = activated_index == index;
            activated_index += 1;
            matches
        })
        .map_or(
            AbilityOrigin::Printed {
                definition: Self::effective_rules_source(permanent).0,
                part: Self::effective_rules_source(permanent).1,
                ability: AbilityId::PRIMARY,
            },
            |effective| effective.origin,
        )
    }

    pub(super) fn permanent_types(&self, permanent: &Permanent) -> Option<CardTypeSet> {
        let mut types = self.effective_rules(permanent)?.types();
        if let Some(copy) = &permanent.copy_effect {
            types = types.union(copy.added_types);
        }
        if let Some(animation) = permanent.animation {
            types = types.union(animation.types);
        }
        Some(types)
    }

    /// The colours a permanent actually is. An animation that repaints it
    /// replaces the printed colours rather than adding to them.
    pub(super) fn effective_colors(permanent: &Permanent, rules: &CardRules) -> [bool; 5] {
        permanent
            .animation
            .and_then(|animation| animation.colors)
            .map_or_else(|| rules.colors(), ColorSet::to_flags)
    }

    pub(super) fn is_protected_from_colors(
        &self,
        permanent: &Permanent,
        source_colors: [bool; 5],
    ) -> bool {
        [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ]
        .into_iter()
        .any(|color| {
            source_colors[color
                .color_index()
                .expect("the iteration contains only colors")]
                && self.permanent_has_executable_keyword(
                    permanent,
                    KeywordAbility::ProtectionFrom(color),
                )
        })
    }

    pub(super) fn permanent_can_be_targeted_by(
        &self,
        permanent: &Permanent,
        controller: PlayerId,
        source: GameObjectId,
    ) -> bool {
        !(self.is_protected_from_colors(permanent, self.object_colors(source))
            || self.permanent_has_executable_keyword(permanent, KeywordAbility::Shroud)
            || permanent.controller != controller
                && self.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof)
            || self.cannot_become_enchanted(permanent) && self.source_attaches_itself(source))
    }

    /// Whether the object doing the targeting is an Aura spell: one whose
    /// spell clause attaches the permanent the spell becomes. "Can't be
    /// enchanted" is a targeting restriction for those and nothing else, so a
    /// Shock aimed at the same permanent is unaffected.
    pub(super) fn source_attaches_itself(&self, source: GameObjectId) -> bool {
        let Some(definition) = self
            .object_definition(source)
            .and_then(|definition| self.catalog.get(definition))
        else {
            return false;
        };
        definition.parts.iter().any(|part| {
            part.rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Spell(_))
                    && ability
                        .declarative_effect()
                        .is_some_and(Self::effect_attaches)
            })
        })
    }

    pub(super) fn effect_attaches(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Attach { .. } => true,
            EffectDef::Sequence(effects) => effects.iter().copied().any(Self::effect_attaches),
            EffectDef::May { effect, .. } => Self::effect_attaches(*effect),
            _ => false,
        }
    }

    /// The card definition behind a game object, wherever it currently is.
    pub(super) fn object_definition(&self, object: GameObjectId) -> Option<CardDefinitionId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.card.definition)
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|stack| stack.id == object)
                    .map(|stack| stack.card.definition)
            })
            .or_else(|| {
                self.card_in_nonbattlefield_zone(object)
                    .map(|(_, card)| card.definition)
            })
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|player| player.outside_game.iter())
                    .find(|card| card.id == object)
                    .map(|card| card.definition)
            })
    }

    /// The expansion a game object's card was first printed in. A token has
    /// no printing, so it belongs to no expansion.
    pub(super) fn object_debut_set(&self, object: GameObjectId) -> Option<CardSet> {
        let definition = self.object_definition(object)?;
        self.catalog.get(definition).map(|card| card.debut_set)
    }

    pub(super) fn object_colors(&self, object: GameObjectId) -> [bool; 5] {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return self
                .effective_rules(permanent)
                .map_or([false; 5], CardRules::colors);
        }
        if let Some(stack) = self.stack.iter().find(|stack| stack.id == object) {
            return stack.colors.map_or_else(
                || {
                    self.stack_trigger_event_object(stack)
                        .map_or([false; 5], |event| event.colors)
                },
                ColorSet::to_flags,
            );
        }
        if let Some(retired) = self.retired_objects.get(&object) {
            return match retired {
                RetiredObject::Permanent { permanent, .. } => self
                    .effective_rules(permanent)
                    .map_or([false; 5], CardRules::colors),
                RetiredObject::Stack(stack) => self
                    .stack_trigger_event_object(stack)
                    .map_or([false; 5], |event| event.colors),
                RetiredObject::Card(card) => self
                    .catalog
                    .get(card.definition)
                    .map_or([false; 5], |definition| definition.rules.colors()),
            };
        }
        self.card_in_nonbattlefield_zone(object)
            .map(|(_, card)| card)
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|player| player.outside_game.iter())
                    .find(|card| card.id == object)
            })
            .and_then(|card| self.catalog.get(card.definition))
            .map_or([false; 5], |definition| definition.rules.colors())
    }
    pub(super) fn combat_is_protected(&self, blocker: &Permanent, attacker: &Permanent) -> bool {
        let blocker_colors = self
            .effective_rules(blocker)
            .map_or([false; 5], CardRules::colors);
        self.is_protected_from_colors(attacker, blocker_colors)
    }
}
