// What a triggered mana ability may do when it fires: add mana, and nothing
// that would need the stack. Kept beside the capture above rather than in it,
// because the refusal list is one arm per effect variant and says nothing
// about how triggers are captured.

impl Game {
    // Long because the effect vocabulary is wide, not because the function
    // does several things: every arm is one variant refused the same way.
    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_triggered_mana_effect(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: EffectDef,
        context: &EffectResolutionContext,
    ) {
        let mut choices = std::iter::empty();
        self.resolve_triggered_mana_effect_with_choices(
            source,
            controller,
            effect,
            context,
            &mut choices,
        );
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_triggered_mana_effect_with_choices(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: EffectDef,
        context: &EffectResolutionContext,
        choices: &mut impl Iterator<Item = crate::ManaSplit>,
    ) {
        match effect {
            EffectDef::BindOutput { effect, .. } => {
                self.resolve_triggered_mana_effect_with_choices(
                    source, controller, *effect, context, choices,
                );
            }
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.resolve_triggered_mana_effect_with_choices(
                        source,
                        controller,
                        *effect,
                        context,
                        choices,
                    );
                }
            }
            EffectDef::AddMana(effect) => {
                self.resolve_triggered_add_mana_effect(
                    source, controller, effect, context, choices,
                );
            }
            EffectDef::None | EffectDef::ContinueReplacedDraw | EffectDef::Randomized { .. } | EffectDef::Choose(_) | EffectDef::ChooseExact(_) |
EffectDef::ChooseCardsFromCollection(_) |
EffectDef::LookAtObjects(_) | EffectDef::ChooseObjectOrder(_) |
EffectDef::ClassifyObjects(_) | EffectDef::RevealAndClassifyCards(_) |
EffectDef::ChooseOneOfEach(_) |
EffectDef::CombineObjects(_) |
EffectDef::ChooseGroup(_) | EffectDef::BindObjects(_) |
EffectDef::IfNoObjects(_) |
EffectDef::PartitionGroup(_) | EffectDef::RandomizeObjectOrder(_) |
EffectDef::RevealObjects(_) | EffectDef::MoveObjects(_) |
EffectDef::PutObjectsOntoBattlefieldFaceDown(_) |
EffectDef::SimultaneousChoose(_) | EffectDef::ChooseCardName { .. } |
EffectDef::SelectAtRandomFromZone { .. } |
EffectDef::ForEachInBinding { .. } | EffectDef::PayOr(_) |
EffectDef::PreventDamage { .. } |
EffectDef::DealDamage { .. } | EffectDef::DealDamageSimultaneously(_) |
EffectDef::DealDamageFrom { .. } | EffectDef::DealDamageAndApply { .. } |
EffectDef::Fight { .. } | EffectDef::DrainLife { .. } |
EffectDef::GainLife { .. } | EffectDef::AddPlayerCounters { .. } |
EffectDef::DrawCards { .. } | EffectDef::Discard { .. } |
EffectDef::DiscardCards { .. } | EffectDef::ShuffleLibrary { .. } |
EffectDef::BuryGraveyard { .. } | EffectDef::EmptyManaPool { .. } |
EffectDef::LoseLife { .. } | EffectDef::LoseTheGame { .. } |
EffectDef::WinTheGame { .. } | EffectDef::AddManaEqualTo { .. } |
EffectDef::Regenerate { .. } | EffectDef::Tap { .. } |
EffectDef::RemoveFromCombat { .. } | EffectDef::SkipNextUntapSteps { .. } |
EffectDef::DoubleCounters { .. } | EffectDef::RemoveAllCounters { .. } |
EffectDef::Untap { .. } | EffectDef::Saddle { .. } | EffectDef::Destroy { .. }
| EffectDef::Sacrifice { .. } | EffectDef::SacrificeOfChoice { .. } |
EffectDef::ExileTopOfLibraryToPlay { .. } | EffectDef::ExileTopAndMayCast { ..
} | EffectDef::MayCastTargetWithoutPaying { .. } | EffectDef::Mill { .. } |
EffectDef::SearchZonesAndExileRest { .. } | EffectDef::MillUntil { .. } |
EffectDef::ExileFromTopUntil { .. } |
EffectDef::Cascade | EffectDef::Proliferate | EffectDef::Explore { .. } |
EffectDef::LookAtHand { .. } |
EffectDef::LookAtRandomCardInHand { .. } |
EffectDef::ExileOneFromEachZone(_) |
EffectDef::PermitCastFromGraveyardThisTurn { .. } |
EffectDef::MillWhileMatching(_) |
EffectDef::RevealAtRandomFromHand { .. } | EffectDef::RevealHand { .. } |
EffectDef::SearchZone { .. } | EffectDef::ChooseCards { .. } |
EffectDef::ReplaceNextDrawThisTurn { .. } | EffectDef::IfFormat { .. } |
EffectDef::Counter { .. } |
EffectDef::PutSpellIntoOwnersLibrary { .. } | EffectDef::CopyStackObject(_) | EffectDef::ChangeStackTargets(_) | EffectDef::AddCounters { .. } | EffectDef::ChooseCounterKind { .. } | EffectDef::ChooseEffect { .. } | EffectDef::ModifyCounters { .. } | EffectDef::RemoveCounters { .. } |
EffectDef::ChangeTextBasicLandType { .. } | EffectDef::ChooseColor { .. } |
EffectDef::BecomeCopyOf { .. } | EffectDef::May { .. } |
EffectDef::CannotBeForcedToSacrifice | EffectDef::CannotBeForcedToDiscard |
EffectDef::GainClassLevel { .. } |
EffectDef::SetLifeTotal { .. } |
EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. } |
EffectDef::CreateEmblem { .. } | EffectDef::CreateOngoingEffect(_) |
EffectDef::PutOntoBattlefieldThen { .. } | EffectDef::Transform { .. } |
EffectDef::ScheduleTurnPhases(_) | EffectDef::TakeExtraTurn { .. } |
EffectDef::PutSourceOntoBattlefieldAttacking | EffectDef::BecomeMonarch { .. }
| EffectDef::VoteForPermanentToExile { .. } |
EffectDef::DamageCannotBePreventedThisTurn |
EffectDef::ExileLinkedToSource { .. } |
EffectDef::PermitLookAtExiled { .. } |
EffectDef::MayPlayWithoutPaying { .. } | EffectDef::ExileGrantingOwnerPlay { .. } |
EffectDef::ExileGrantingControllerPlayThisTurn { .. } |
EffectDef::ReturnLinkedExiles { .. } | EffectDef::Detain { .. } |
EffectDef::GainControl { .. } | EffectDef::ExchangeControl { .. } |
EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. } |
EffectDef::InstallTrigger(_) |
EffectDef::ReduceGenericCostBy(_) | EffectDef::ModifyCost(_) |
EffectDef::LandwalkCanBeBlocked(_) | EffectDef::CannotAttackUnless(_) |
EffectDef::CannotAttackIf(_) | EffectDef::PutIntoLibraryBeneathTop { .. } |
EffectDef::MoveToZone { .. } |
EffectDef::WithBattlefieldArrival { .. } |
EffectDef::WithZoneMoveResult { .. } |
EffectDef::Attach { .. } | EffectDef::AttachToSource { .. } |
EffectDef::PairWithSource { .. } | EffectDef::Reconfigure { .. } |
EffectDef::Unattach { .. } | EffectDef::PhaseOut { .. } |
EffectDef::CreateToken { .. } | EffectDef::CreateAttachedToken { .. } |
EffectDef::Endure { .. } |
EffectDef::CreateMyriadTokens |
EffectDef::ConditionalStatic(_) |
EffectDef::StaticApply { .. } | EffectDef::Apply { .. } |
EffectDef::Special(_) => {
                // Choice-bearing and non-mana primitives need a dedicated
                // immediate procedure before a supported card can use them.
            }
        }
    }

    fn resolve_triggered_add_mana_effect(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: AddManaEffectDef,
        context: &EffectResolutionContext,
        choices: &mut impl Iterator<Item = crate::ManaSplit>,
    ) {
        let AddManaEffectDef {
            mana,
            also,
            amount,
            restrictions,
            spend_effects,
            damage_to_controller,
            recipient,
            amount_override,
            variable_amount: _,
            sacrifice_source_when_out_of: _,
        } = effect;
        // A mana trigger resolves without ever going on the stack, so it has
        // no resolving object to read a general player reference from. The
        // two a printed clause asks for are the ability's own controller and
        // the controller of whatever was tapped.
        let controller = match recipient {
            PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject) => context
                .trigger
                .object
                .and_then(|triggering| self.current_or_last_known_controller(triggering))
                .or(context.trigger.object_controller)
                .unwrap_or(controller),
            _ => controller,
        };
        let amount = amount_override
            .filter(|override_| {
                self.static_condition_holds(override_.condition, controller, source.object)
            })
            .map_or(amount, |override_| override_.amount);
        let mut split = match mana {
            ManaSelectionDef::One(kind) => {
                let Some(color) = self.mana_type_for_source(kind, source.object) else {
                    return;
                };
                let mut split = crate::ManaSplit::empty();
                split.add(color, amount);
                split
            }
            ManaSelectionDef::Choice(_) | ManaSelectionDef::Combination(_) => {
                let Some(split) = choices.next() else {
                    return;
                };
                split
            }
            ManaSelectionDef::ColorsOfLinkedExiles => return,
        };
        if let Some(color) = also {
            split.add(color, 1);
        }
        let source = ManaSource {
            object: source.object,
            ability: source.ability,
        };
        self.add_mana(
            controller,
            split.iter().flat_map(|(color, amount)| {
                std::iter::repeat_n(
                    Mana::from_ability(color, source, restrictions, spend_effects),
                    usize::from(amount),
                )
            }),
        );
        if damage_to_controller > 0 {
            self.damage_target_from(
                Some(source.object),
                Some(Target::Player(controller)),
                damage_to_controller,
            );
        }
    }
}
