use super::{
    AppliedEffectDef, AppliedRuleDef, CharacteristicContext, CounteredSpellZone,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, Game, GameObjectId, StackObject,
    StackObjectKind, Target, ZoneKind, applicable_part_ids,
};
use crate::card::ChooseDef;
use crate::card::ZonePlacement;

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
            && let Some(card_definition) = object.card.definition.card_definition()
            && let Some(definition) = self.catalog.get(card_definition)
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

    // Most of the length is the exhaustive list of effects that apply nothing
    // to their own source. Listing them is the point: a new effect has to be
    // classified here rather than silently answering "no".
    #[allow(clippy::too_many_lines)]
    pub(super) fn effect_applies_to_source(effect: EffectDef, expected: AppliedEffectDef) -> bool {
        match effect {
            EffectDef::BindOutput { effect, .. }
            | EffectDef::WithBattlefieldArrival { effect, .. } => {
                Self::effect_applies_to_source(*effect, expected)
            }
            EffectDef::WithZoneMoveResult { effect, then, .. } => {
                Self::effect_applies_to_source(*effect, expected)
                    || Self::effect_applies_to_source(*then, expected)
            }
            EffectDef::Sequence(effects) => effects
                .iter()
                .any(|effect| Self::effect_applies_to_source(*effect, expected)),
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect,
            } => Self::applied_effect_contains(effect, expected),
            EffectDef::IfFormat {
                then, otherwise, ..
            } => {
                Self::effect_applies_to_source(*then, expected)
                    || Self::effect_applies_to_source(*otherwise, expected)
            }
            EffectDef::Choose(ChooseDef { then, .. })
            | EffectDef::ChooseCardName { then, .. }
            | EffectDef::ForEachInBinding { effect: then, .. }
            | EffectDef::SearchZone {
                then: Some(then), ..
            }
            | EffectDef::PermitLookAtExiled { then, .. } => {
                Self::effect_applies_to_source(*then, expected)
            }
            EffectDef::ChooseCardsFromCollection(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::LookAtObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::ChooseObjectOrder(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::ClassifyObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::RevealAndClassifyCards(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::CombineObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::ChooseOneOfEach(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::ChooseGroup(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::BindObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::IfNoObjects(definition) => {
                Self::effect_applies_to_source(*definition.if_empty, expected)
                    || Self::effect_applies_to_source(*definition.otherwise, expected)
            }
            EffectDef::PartitionGroup(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::RandomizeObjectOrder(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::RevealObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::MoveObjects(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
                Self::effect_applies_to_source(*definition.then, expected)
            }
            EffectDef::SimultaneousChoose(choice) => {
                Self::effect_applies_to_source(*choice.then, expected)
            }
            EffectDef::Destroy {
                then: Some(follow_up),
                ..
            } => Self::effect_applies_to_source(*follow_up.effect, expected),
            EffectDef::PayOr(payment) => payment
                .if_paid
                .iter()
                .chain(payment.otherwise.iter())
                .any(|effect| Self::effect_applies_to_source(**effect, expected)),
            EffectDef::None
            | EffectDef::Randomized { .. }
            | EffectDef::PreventDamage { .. }
            | EffectDef::AddMana(_)
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::DealDamage { .. }
            | EffectDef::DealDamageSimultaneously(_)
            | EffectDef::DealDamageFrom { .. }
            | EffectDef::DealDamageAndApply { .. }
            | EffectDef::Fight { .. }
            | EffectDef::DrainLife { .. }
            | EffectDef::GainLife { .. }
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
            | EffectDef::Attach { .. }
            | EffectDef::AttachToSource { .. }
            | EffectDef::Reconfigure { .. }
            | EffectDef::Unattach { .. }
            | EffectDef::PairWithSource { .. }
            | EffectDef::PhaseOut { .. }
            | EffectDef::Destroy { then: None, .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::ExileTopOfLibraryToPlay { .. }
            | EffectDef::ExileTopAndMayCast { .. }
            | EffectDef::MayCastTargetWithoutPaying { .. }
            | EffectDef::Mill { .. }
            | EffectDef::SelectAtRandomFromZone { .. }
            | EffectDef::SearchZonesAndExileRest { .. }
            | EffectDef::MillUntil { .. }
            | EffectDef::ExileFromTopUntil { .. }
            | EffectDef::Cascade
            | EffectDef::Proliferate
            | EffectDef::Explore { .. }
            | EffectDef::SearchZone { then: None, .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::ExileOneFromEachZone(_)
            | EffectDef::PermitCastFromGraveyardThisTurn { .. }
            | EffectDef::MillWhileMatching(_)
            | EffectDef::LookAtRandomCardInHand { .. }
            | EffectDef::RevealAtRandomFromHand { .. }
            | EffectDef::RevealHand { .. }
            | EffectDef::ChooseCards { .. }
            | EffectDef::ReplaceNextDrawThisTurn { .. }
            | EffectDef::Counter { .. }
            | EffectDef::PutSpellIntoOwnersLibrary { .. }
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
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::GainClassLevel { .. }
            | EffectDef::SetLifeTotal { .. }
            | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
            | EffectDef::CreateEmblem { .. }
            | EffectDef::CreateOngoingEffect(_)
            | EffectDef::PutOntoBattlefieldThen { .. }
            | EffectDef::Transform { .. }
            | EffectDef::ScheduleTurnPhases(_)
            | EffectDef::TakeExtraTurn { .. }
            | EffectDef::PutSourceOntoBattlefieldAttacking
            | EffectDef::BecomeMonarch { .. }
            | EffectDef::VoteForPermanentToExile { .. }
            | EffectDef::DamageCannotBePreventedThisTurn
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::MayPlayWithoutPaying { .. }
            | EffectDef::ExileGrantingOwnerPlay { .. }
            | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::Detain { .. }
            | EffectDef::GainControl { .. }
            | EffectDef::ExchangeControl { .. }
            | EffectDef::IfCondition { .. }
            | EffectDef::IfElseCondition { .. }
            | EffectDef::InstallTrigger(_)
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::ModifyCost(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::CannotAttackIf(_)
            | EffectDef::PutIntoLibraryBeneathTop { .. }
            | EffectDef::MoveToZone { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::CreateAttachedToken { .. }
            | EffectDef::Endure { .. }
            | EffectDef::CreateMyriadTokens
            | EffectDef::StaticApply { .. }
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
        let Some(card_definition) = object.card.definition.card_definition() else {
            return false;
        };
        let Some(definition) = self.catalog.get(card_definition) else {
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
                        && ability
                            .declarative_effect()
                            .is_some_and(|effect| Self::effect_applies_to_source(effect, expected))
                })
            })
        })
    }

    /// Whether a spell on the stack can be countered at all. Printed static
    /// abilities and effects carried by mana converge here; neither changes
    /// whether the spell is a legal target.
    pub(super) fn can_be_countered(&self, object: &StackObject) -> bool {
        !self.stack_spell_has_static_effect(
            object,
            AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
        ) && !object.applied_effects.iter().any(|applied| {
            Self::applied_effect_contains(
                applied.effect,
                AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            )
        })
    }

    #[cfg(test)]
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
        self.remove_spell_from_stack(index, zone);
    }

    /// Returns a spell from the stack to its owner's hand.
    ///
    /// Not a counter. The spell is never countered, so "can't be countered"
    /// does not stop this and nothing watching for a countered spell sees
    /// one -- which is the whole reason Reprieve is played over a counter.
    /// "Its owner puts it on their choice of the top or bottom of their
    /// library." Not a counter: a spell that cannot be countered goes there
    /// all the same.
    pub(super) fn put_spell_into_library(&mut self, id: GameObjectId, placement: ZonePlacement) {
        let Some(index) = self.stack.iter().position(|object| object.id == id) else {
            return;
        };
        self.remove_spell_from_stack(index, CounteredSpellZone::Library(placement));
    }

    pub(super) fn return_spell_to_hand(&mut self, id: GameObjectId) {
        let Some(index) = self.stack.iter().position(|object| object.id == id) else {
            return;
        };
        self.remove_spell_from_stack(index, CounteredSpellZone::Hand);
    }

    /// Takes the stack object at `index` off the stack and puts its card
    /// where it is going. A copy has no card and simply ceases to exist
    /// (CR 707.10), and a spell cast via flashback is exiled wherever else
    /// it would have gone (CR 702.34a).
    fn remove_spell_from_stack(&mut self, index: usize, zone: CounteredSpellZone) {
        let object = self.stack.remove(index);
        self.retire_stack_object(&object);
        if object.kind == StackObjectKind::Spell && !object.is_copy {
            let owner = object.card.owner;
            let (card, _zone_change) = self.zone_change_card(
                object
                    .card
                    .into_card()
                    .expect("a nontoken spell is backed by a card"),
            );
            match if object.cast_via_flashback {
                CounteredSpellZone::Exile
            } else {
                zone
            } {
                CounteredSpellZone::Graveyard => self.put_card_into_graveyard(owner, card),
                CounteredSpellZone::Exile => self.players[owner.index()].exile.push(card),
                CounteredSpellZone::Hand => self.players[owner.index()].hand.push(card),
                CounteredSpellZone::Library(placement) => {
                    let library = &mut self.players[owner.index()].library;
                    let index = placement.library_index(library.len());
                    library.insert(index, card);
                }
            }
        }
    }
}
