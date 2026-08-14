use super::{
    AppliedEffectDef, CharacteristicContext, CounteredSpellZone, DeclarativeAbilityDef, EffectDef,
    EffectDurationDef, EffectRecipientDef, Game, GameObjectId, StackObject, StackObjectKind,
    Target, ZoneKind, applicable_part_ids,
};

impl Game {
    /// True when a spell had targets and every one of them is now illegal.
    pub(super) fn spell_fizzles(&self, object: &StackObject) -> bool {
        if object.target_count() == 0 {
            return false;
        }
        if object.ability.is_some() {
            return self.stack_ability_fizzles(object);
        }
        if let Some(signature) = &object.signature
            && let Some(definition) = self.catalog.get(object.card.definition)
            && let Some(option) = definition.play_option(signature.play_option())
        {
            let slots = Self::target_slots_for(option, signature.modes());
            if !slots.is_empty() || option.modes.is_some() || !option.targets.is_empty() {
                return signature
                    .targets()
                    .iter()
                    .zip(slots)
                    .flat_map(|(selection, slot)| {
                        selection
                            .targets()
                            .iter()
                            .map(move |target| (slot.predicate, *target))
                    })
                    .all(|(predicate, target)| !self.target_matches(predicate, target));
            }
        }
        object.iter_targets().all(|target| match target {
            Target::Player(_) => false,
            Target::Card(id) => self.card_in_nonbattlefield_zone(*id).is_none(),
            Target::Permanent(id) => !self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == *id),
            Target::Spell(id) => !self.stack.iter().any(|candidate| candidate.id == *id),
        })
    }

    pub(super) fn effect_applies_to_source(
        effect: EffectDef,
        expected: AppliedEffectDef,
        duration: EffectDurationDef,
    ) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .any(|effect| Self::effect_applies_to_source(*effect, expected, duration)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect,
                duration: actual_duration,
            } => Self::applied_effect_contains(effect, expected) && actual_duration == duration,
            EffectDef::IfFormat {
                then, otherwise, ..
            } => {
                Self::effect_applies_to_source(*then, expected, duration)
                    || Self::effect_applies_to_source(*otherwise, expected, duration)
            }
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
            | EffectDef::Attach { .. }
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
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalPayment { .. }
            | EffectDef::UnlessPaid { .. }
            | EffectDef::May { .. }
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CreateEmblem { .. }
            | EffectDef::Transform { .. }
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
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::PlayersCantPlay(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::Replacement(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => false,
        }
    }

    pub(super) fn applied_effect_contains(
        effect: AppliedEffectDef,
        expected: AppliedEffectDef,
    ) -> bool {
        effect == expected
            || matches!(
                effect,
                AppliedEffectDef::Composite(effects)
                    if effects
                        .iter()
                        .any(|effect| Self::applied_effect_contains(*effect, expected))
            )
    }

    pub(super) fn stack_spell_has_static_effect(
        &self,
        object: &StackObject,
        expected: AppliedEffectDef,
    ) -> bool {
        let Some(signature) = &object.signature else {
            return false;
        };
        let Some(definition) = self.catalog.get(object.card.definition) else {
            return false;
        };
        let Ok(parts) = applicable_part_ids(
            definition,
            &CharacteristicContext::Stack {
                form: signature.form().clone(),
            },
        ) else {
            return false;
        };
        parts.into_iter().any(|part| {
            definition.part(part).is_some_and(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(
                            ability.definition,
                            DeclarativeAbilityDef::Static(definition)
                                if definition.source_zones.contains(&ZoneKind::Stack)
                        )
                        && ability.declarative_effect().is_some_and(|effect| {
                            Self::effect_applies_to_source(
                                effect,
                                expected,
                                EffectDurationDef::WhileSourceRemainsInZone,
                            )
                        })
                })
            })
        })
    }

    /// Whether a spell on the stack can be countered at all. Printed static
    /// abilities and effects carried by mana converge here; neither changes
    /// whether the spell is a legal target.
    pub(super) fn can_be_countered(&self, object: &StackObject) -> bool {
        !self.stack_spell_has_static_effect(object, AppliedEffectDef::CannotBeCountered)
            && !object.applied_effects.iter().any(|applied| {
                Self::applied_effect_contains(applied.effect, AppliedEffectDef::CannotBeCountered)
            })
    }

    pub(super) fn counter_spell(&mut self, id: GameObjectId) {
        self.counter_spell_into(id, CounteredSpellZone::Graveyard);
    }

    /// A countered spell normally goes to its owner's graveyard, but several
    /// cards exile it instead so it cannot be rebought.
    pub(super) fn counter_spell_into(&mut self, id: GameObjectId, zone: CounteredSpellZone) {
        let Some(index) = self.stack.iter().position(|object| object.id == id) else {
            return;
        };
        // "Can't be countered" is not "can't be targeted": a Counterspell may
        // legally target Supreme Verdict, resolve, and accomplish nothing. So
        // this is the only place that checks, and the target lists do not.
        if !self.can_be_countered(&self.stack[index]) {
            return;
        }
        let object = self.stack.remove(index);
        self.retire_stack_object(&object);
        if object.kind == StackObjectKind::Spell && !object.is_copy {
            let owner = object.card.owner;
            let (card, _zone_change) = self.zone_change_card(object.card);
            match if object.cast_via_flashback {
                CounteredSpellZone::Exile
            } else {
                zone
            } {
                CounteredSpellZone::Graveyard => self.put_card_into_graveyard(owner, card),
                CounteredSpellZone::Exile => self.players[owner.index()].exile.push(card),
            }
        }
    }
}
