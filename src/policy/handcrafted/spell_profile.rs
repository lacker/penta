use super::{
    AbilityCostDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    BasicLandType, CardBehavior, CardDefinitionId, CardType, CardTypeSet, CastChoices,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, HandcraftedPolicy, ObjectPredicateDef,
    PlayerRelation, SpellForm, ValueDef, ZoneKind,
};
use crate::PlayerSetDef;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct DeclarativeSpellProfile {
    pub(super) damage: Option<u16>,
    pub(super) cards_drawn: Option<u16>,
    /// Cards every player draws. This is not ordinary card advantage for the
    /// caster, so the casting policy scores it separately.
    pub(super) cards_drawn_by_each_player: Option<u16>,
    pub(super) effect_kinds: u8,
    /// Permanent types swept by untargeted global destruction. This is the
    /// guaranteed subset of a matching predicate, so an `AnyOf` can record
    /// each type it names without guessing at narrower predicates.
    pub(super) global_destroy_types: CardTypeSet,
    /// Whether the activation taps its own source. A land that taps to pump
    /// is spending the mana it could have made.
    pub(super) taps_source: bool,
    pub(super) opponent_creature_sweep: bool,
    pub(super) opponent_spell_sweep: bool,
    /// Whether every effect this cast would resolve scales with the chosen X,
    /// so casting for X=0 resolves to nothing at all. `None` until an effect
    /// has been collected, which keeps a profile that recognized nothing from
    /// claiming that the spell is empty.
    pub(super) empty_without_x: Option<bool>,
}

impl DeclarativeSpellProfile {
    pub(super) const COUNTERS: u8 = 1 << 0;
    pub(super) const REMOVES: u8 = 1 << 1;
    pub(super) const TAPS: u8 = 1 << 2;
    pub(super) const APPLIES: u8 = 1 << 3;
    pub(super) const SWEEPS_CREATURES: u8 = 1 << 4;
    pub(super) const EXTRA_TURN: u8 = 1 << 5;

    pub(super) fn mark(&mut self, effect_kind: u8) {
        self.effect_kinds |= effect_kind;
    }

    pub(super) const fn has(self, effect_kind: u8) -> bool {
        self.effect_kinds & effect_kind != 0
    }
}

impl HandcraftedPolicy {
    pub(super) fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog
            .get(definition)
            .and_then(|card| card.rules.special_behavior())
    }

    /// Whether casting this spell for X=0 resolves to nothing at all, so the
    /// card is spent to draw, deal, or discard nothing. The policy only ever
    /// sees an X=0 cast when the bot has exactly the base cost and no more,
    /// which is precisely when it should wait, so a true answer here means
    /// hold the card. Restricted to spells that really do pay into an X.
    /// Fireball is named rather than inspected because its card-local damage
    /// selector leaves it no declarative effect to read.
    pub(super) fn is_empty_at_zero_x(
        &self,
        definition: CardDefinitionId,
        declarative: Option<DeclarativeSpellProfile>,
    ) -> bool {
        let Some(card) = self.catalog.get(definition) else {
            return false;
        };
        if !card.rules.mana_cost().is_some_and(|cost| cost.variable_x) {
            return false;
        }
        matches!(card.rules.special_behavior(), Some(CardBehavior::Fireball))
            || declarative.is_some_and(|profile| profile.empty_without_x == Some(true))
    }

    pub(super) fn is_mana_source(&self, definition: CardDefinitionId) -> bool {
        self.catalog.get(definition).is_some_and(|card| {
            (card.rules.has_type(CardType::Land)
                && BasicLandType::ALL
                    .into_iter()
                    .any(|land_type| card.rules.has_subtype(land_type.subtype())))
                || card.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::ActivatedMana(_))
                })
        })
    }

    pub(super) fn declarative_mana_value(&self, definition: CardDefinitionId) -> Option<i32> {
        let card = self.catalog.get(definition)?;
        if card.rules.has_type(CardType::Land) {
            return self.is_mana_source(definition).then_some(80);
        }
        card.rules
            .ability_clauses()
            .iter()
            .filter(|ability| ability.is_executable())
            .find_map(|ability| {
                let DeclarativeAbilityDef::ActivatedMana(definition) = ability.definition else {
                    return None;
                };
                let EffectDef::AddMana(effect) = ability.declarative_effect()? else {
                    return None;
                };
                Some(
                    if effect.amount >= 3
                        && definition.costs.contains(&AbilityCostDef::SacrificeSource)
                    {
                        100
                    } else {
                        90
                    },
                )
            })
    }

    pub(super) fn declarative_spell_profile(
        &self,
        definition: CardDefinitionId,
        choices: &CastChoices,
    ) -> Option<DeclarativeSpellProfile> {
        let card = self.catalog.get(definition)?;
        let option = card.play_option(choices.play_option())?;
        let SpellForm::Part(part) = option.form else {
            return None;
        };
        let rules = &card.part(part)?.rules;
        if let Some(ability) = choices.costs().alternative().and_then(|alternative| {
            rules.indexed_abilities().find_map(|attached| {
                (attached.definition.is_executable()
                    && attached.alternative_cost_id() == Some(alternative)
                    && matches!(
                        attached.definition.definition,
                        DeclarativeAbilityDef::AlternativeCast(alternative_cast)
                            if alternative_cast.kind == AlternativeCastKindDef::Overload
                    ))
                .then_some(attached.definition)
            })
        }) {
            if !choices.modes().is_empty() {
                return None;
            }
            let mut profile = DeclarativeSpellProfile::default();
            Self::collect_spell_effect(
                ability.declarative_effect()?,
                choices.x(),
                &[],
                &mut profile,
            );
            return Some(profile);
        }
        let ability = rules.ability_clauses().iter().find(|ability| {
            ability.is_executable() && matches!(ability.definition, DeclarativeAbilityDef::Spell(_))
        })?;
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            unreachable!("the selected ability is a spell ability")
        };
        let mut profile = DeclarativeSpellProfile::default();
        Self::collect_spell_effect(
            ability.declarative_effect()?,
            choices.x(),
            spell.targets(),
            &mut profile,
        );
        if spell.modal().is_none() {
            return choices.modes().is_empty().then_some(profile);
        }
        for selected in choices.modes() {
            let mode = spell.mode(*selected)?;
            if !mode.is_executable() {
                return None;
            }
            Self::collect_spell_effect(
                mode.declarative_effect()?,
                choices.x(),
                match mode.definition {
                    DeclarativeAbilityDef::Spell(spell) => spell.targets(),
                    _ => &[],
                },
                &mut profile,
            );
        }
        Some(profile)
    }

    /// Whether a recipient names every creature an opponent controls, which
    /// is what makes a damage or counter effect a one-sided sweep.
    pub(super) fn hits_every_opposing_creature(recipient: EffectRecipientDef) -> bool {
        recipient.object_query().is_some_and(|query| {
            query.object == ObjectPredicateDef::HasType(crate::CardType::Creature)
                && query.zones == [ZoneKind::Battlefield]
                && query.controller.is_none()
                && query.owner.is_none()
                && matches!(
                    query.related_player,
                    Some(PlayerSetDef::Related(
                        PlayerRelation::Opponent | PlayerRelation::NotYou
                    ))
                )
        })
    }

    /// A destroy is removal; a destroy aimed at every creature on the
    /// battlefield, whoever controls it, is a sweeper as well.
    pub(super) fn collect_destroy_profile(
        object: EffectRecipientDef,
        profile: &mut DeclarativeSpellProfile,
    ) {
        profile.mark(DeclarativeSpellProfile::REMOVES);
        if let Some(query) = object.object_query()
            && query.zones == [ZoneKind::Battlefield]
            && query.controller.is_none()
            && query.owner.is_none()
            && query.related_player == Some(PlayerSetDef::Related(PlayerRelation::Any))
        {
            let destroyed_types = Self::globally_destroyed_types(query.object);
            profile.global_destroy_types = profile.global_destroy_types.union(destroyed_types);
            if destroyed_types.contains(CardType::Creature) {
                profile.mark(DeclarativeSpellProfile::SWEEPS_CREATURES);
            }
        }
    }

    /// Types that a global predicate necessarily destroys in full. An
    /// `AnyOf` contributes the union of those guarantees; an `All`, `Not`, or
    /// narrower predicate contributes none because some permanents of the
    /// named type can fail it.
    fn globally_destroyed_types(object: ObjectPredicateDef) -> CardTypeSet {
        match object {
            ObjectPredicateDef::HasType(card_type) => CardTypeSet::single(card_type),
            ObjectPredicateDef::AnyOf(predicates) => predicates
                .iter()
                .copied()
                .fold(CardTypeSet::empty(), |types, predicate| {
                    types.union(Self::globally_destroyed_types(predicate))
                }),
            _ => CardTypeSet::empty(),
        }
    }

    fn collect_damage_profile(
        recipient: EffectRecipientDef,
        amount: ValueDef,
        x: u16,
        profile: &mut DeclarativeSpellProfile,
    ) {
        profile.damage = Self::policy_value(amount, x);
        profile.opponent_creature_sweep |= Self::hits_every_opposing_creature(recipient);
    }

    fn target_slot_is_on_battlefield(targets: &[AbilityTargetDef], index: usize) -> bool {
        targets.get(index).is_some_and(|definition| {
            matches!(
                definition.predicate,
                AbilityTargetPredicate::Object { zones, .. }
                    if zones.contains(&ZoneKind::Battlefield)
            )
        })
    }

    fn collect_draw_profile(
        recipient: EffectRecipientDef,
        amount: ValueDef,
        x: u16,
        profile: &mut DeclarativeSpellProfile,
    ) {
        if recipient == EffectRecipientDef::EachPlayer {
            profile.cards_drawn_by_each_player = Self::policy_value(amount, x);
        } else {
            profile.cards_drawn = Self::policy_value(amount, x);
        }
    }

    /// Collects one whole effect a cast would resolve. Every effect reaches
    /// the profile through here rather than through the recursive collector,
    /// so `empty_without_x` sees each top-level effect exactly once.
    fn collect_spell_effect(
        effect: EffectDef,
        x: u16,
        targets: &[AbilityTargetDef],
        profile: &mut DeclarativeSpellProfile,
    ) {
        Self::collect_spell_effect_profile(effect, x, targets, profile);
        let empty = Self::is_empty_without_x(effect);
        profile.empty_without_x = Some(profile.empty_without_x.unwrap_or(true) && empty);
    }

    /// Whether an effect does nothing whatsoever when the spell's X is zero,
    /// because every amount it resolves is that X. A spell built only from
    /// such effects — Braingeyser's draw, Earthquake's damage, Mind Twist's
    /// discard — is a wasted card at X=0. Detonate is not one: its destroy
    /// still kills a zero-cost artifact, so this reports false for it, as it
    /// does for any effect whose behavior at X=0 is not obviously nothing.
    fn is_empty_without_x(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => {
                !effects.is_empty()
                    && effects
                        .iter()
                        .all(|effect| Self::is_empty_without_x(*effect))
            }
            EffectDef::May { effect, .. } => Self::is_empty_without_x(*effect),
            EffectDef::DealDamage { amount, .. }
            | EffectDef::DealDamageFrom { amount, .. }
            | EffectDef::DealDamageAndApply { amount, .. }
            | EffectDef::DrainLife { amount, .. }
            | EffectDef::DrawCards { amount, .. }
            | EffectDef::Discard { amount, .. }
            | EffectDef::Mill { amount, .. }
            | EffectDef::GainLife { amount, .. }
            | EffectDef::LoseLife { amount, .. } => amount == ValueDef::ChosenX,
            _ => false,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn collect_spell_effect_profile(
        effect: EffectDef,
        x: u16,
        targets: &[AbilityTargetDef],
        profile: &mut DeclarativeSpellProfile,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    Self::collect_spell_effect_profile(*effect, x, targets, profile);
                }
            }
            EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            } => {
                Self::collect_spell_effect_profile(*on_success, x, targets, profile);
                Self::collect_spell_effect_profile(*on_failure, x, targets, profile);
            }
            EffectDef::Choose(choice) => {
                Self::collect_spell_effect_profile(*choice.then, x, targets, profile);
            }
            EffectDef::SimultaneousChoose(choice) => {
                Self::collect_spell_effect_profile(*choice.then, x, targets, profile);
            }
            EffectDef::ChooseCardName { then, .. }
            | EffectDef::SearchZone {
                then: Some(then), ..
            }
            | EffectDef::BindMatching { then, .. }
            | EffectDef::SelectAtRandomFromZone { then, .. } => {
                Self::collect_spell_effect_profile(*then, x, targets, profile);
            }
            EffectDef::PayOr(payment) => {
                for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                    Self::collect_spell_effect_profile(**effect, x, targets, profile);
                }
            }
            EffectDef::SplitIntoPiles(partition) => {
                Self::collect_spell_effect_profile(*partition.then, x, targets, profile);
            }
            // An optional effect is worth what it would do if taken. Iteration
            // has the same child profile; multiplicity is intentionally not a
            // separate policy weight here.
            EffectDef::May { effect, .. } | EffectDef::ForEachInBinding { effect, .. } => {
                Self::collect_spell_effect_profile(*effect, x, targets, profile);
            }
            EffectDef::DealDamage { recipient, amount }
            | EffectDef::DealDamageFrom {
                recipient, amount, ..
            }
            | EffectDef::DealDamageAndApply {
                recipient, amount, ..
            }
            | EffectDef::DrainLife { recipient, amount } => {
                Self::collect_damage_profile(recipient, amount, x, profile);
            }
            EffectDef::DrawCards { recipient, amount } => {
                Self::collect_draw_profile(recipient, amount, x, profile);
            }
            // Looting is card selection, not card advantage, so the discard
            // cancels out the draw the policy would otherwise reward.
            EffectDef::Discard { amount, .. } => {
                if let Some(drawn) = profile.cards_drawn {
                    profile.cards_drawn =
                        Some(drawn.saturating_sub(Self::policy_value(amount, x).unwrap_or(0)));
                }
            }
            // Returning a spell is not a counter, but it answers one the
            // same way, so the policy weighs it as one.
            EffectDef::MoveToZone {
                object,
                from: Some(ZoneKind::Stack),
                zone: ZoneKind::Hand,
                ..
            }
            | EffectDef::PutSpellIntoOwnersLibrary { object }
            | EffectDef::Counter { object, .. } => {
                profile.mark(DeclarativeSpellProfile::COUNTERS);
                profile.opponent_spell_sweep |= object.object_query().is_some_and(|query| {
                    query.object == ObjectPredicateDef::Spell
                        && query.zones == [ZoneKind::Stack]
                        && query.controller.is_none()
                        && query.owner.is_none()
                        && matches!(
                            query.related_player,
                            Some(PlayerSetDef::Related(
                                PlayerRelation::Opponent | PlayerRelation::NotYou
                            ))
                        )
                });
            }
            EffectDef::Destroy { object, then, .. } => {
                Self::collect_destroy_profile(object, profile);
                if let Some(follow_up) = then {
                    Self::collect_spell_effect_profile(*follow_up.effect, x, targets, profile);
                }
            }
            EffectDef::MoveToZone {
                object,
                from: None,
                zone: ZoneKind::Exile,
                ..
            } if object.legal_target().is_some_and(|target| {
                Self::target_slot_is_on_battlefield(targets, target.index())
            }) =>
            {
                profile.mark(DeclarativeSpellProfile::REMOVES);
            }
            EffectDef::Tap { .. }
            | EffectDef::RemoveFromCombat { .. }
            | EffectDef::SkipNextUntapSteps { .. }
            | EffectDef::DoubleCounters { .. }
            | EffectDef::RemoveAllCounters { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Saddle { .. }
            | EffectDef::PreventDamage { .. } => {
                profile.mark(DeclarativeSpellProfile::TAPS);
            }
            EffectDef::StaticApply { .. } | EffectDef::Apply { .. } => {
                profile.mark(DeclarativeSpellProfile::APPLIES);
            }
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            } => profile.mark(DeclarativeSpellProfile::EXTRA_TURN),
            // Nothing outranks winning, so it needs no profile of its own.
            EffectDef::LoseTheGame { .. }
            | EffectDef::WinTheGame { .. }
            | EffectDef::CopyResolvingSpell { .. }
            | EffectDef::CopyTargetSpell { .. }
            | EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::BuryGraveyard { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::AddPlayerCounters { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Regenerate { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::ExileTopOfLibraryToPlay { .. }
            | EffectDef::ExileTopAndMayCast { .. }
            | EffectDef::MayCastTargetWithoutPaying { .. }
            | EffectDef::Mill { .. }
            | EffectDef::SearchZonesAndExileRest { .. }
            | EffectDef::MillUntil { .. }
            | EffectDef::ExileFromTopUntil { .. }
            | EffectDef::ManifestDread { .. }
            | EffectDef::Cascade
            | EffectDef::Proliferate
            | EffectDef::Explore { .. }
            | EffectDef::LookAtTopAndSelect { .. }
            | EffectDef::LookAtTopAndDistribute { .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::LookAtRandomCardInHand { .. }
            | EffectDef::RevealAtRandomFromHand { .. }
            | EffectDef::RevealHand { .. }
            | EffectDef::SearchZone { .. }
            | EffectDef::ChooseCards { .. }
            | EffectDef::ReplaceNextDrawThisTurn { .. }
            | EffectDef::IfFormat { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::RemoveCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::ChooseColor { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::GainClassLevel { .. }
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
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::MayPlayWithoutPaying { .. }
            | EffectDef::ExileGrantingOwnerPlay { .. }
            | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::Detain { .. }
            | EffectDef::GainControl { .. }
            | EffectDef::ExchangeControl { .. }
            | EffectDef::InstallTrigger(_)
            | EffectDef::IfCondition { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::ModifyCost(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::CannotAttackIf(_)
            | EffectDef::PutIntoLibraryBeneathTop { .. }
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::AttachToSource { .. }
            | EffectDef::Reconfigure { .. }
            | EffectDef::Unattach { .. }
            | EffectDef::PairWithSource { .. }
            | EffectDef::PhaseOut { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::CreateAttachedToken { .. }
            | EffectDef::CreateTokenCopyOf { .. }
            | EffectDef::Endure { .. }
            | EffectDef::CreateMyriadTokens
            | EffectDef::Special(_) => {}
        }
    }

    pub(super) fn policy_value(value: ValueDef, x: u16) -> Option<u16> {
        match value {
            ValueDef::Constant(value) => u16::try_from(value).ok(),
            ValueDef::ChosenX => Some(x),
            // Board-dependent values are not knowable from the definition
            // alone, so the caller falls back to its own heuristics.
            ValueDef::SourceCastX
            | ValueDef::SourcePower
            | ValueDef::AffectedManaValue
            | ValueDef::AffectedColorCount
            | ValueDef::TotalPowerOfLinkedExiles
            | ValueDef::TotalToughnessOfLinkedExiles
            | ValueDef::ObjectPower(_)
            | ValueDef::ObjectManaValue(_)
            | ValueDef::TriggeringObjectPower
            | ValueDef::TriggeringObjectToughness
            | ValueDef::SourceToughness
            | ValueDef::TriggerEventAmount
            | ValueDef::CardsInHandAbove { .. }
            | ValueDef::DamageTakenThisTurn { .. }
            | ValueDef::CountMatchingObjects(_)
            | ValueDef::DistinctNamesAmong(_)
            | ValueDef::CountMatchingPlayerAttachments(_)
            | ValueDef::CountSpellsCastThisTurn(_)
            | ValueDef::GreatestPowerAmong(_)
            | ValueDef::AnyMatchingObject(_)
            | ValueDef::CountersOnSource(_)
            | ValueDef::CardsDrawnThisTurn(_)
            | ValueDef::LifeGainedThisTurn(_)
            | ValueDef::DevotionTo(_)
            | ValueDef::BasicLandTypesControlled(_)
            | ValueDef::LibrarySize(_)
            | ValueDef::SpellsCastThisGame(_)
            | ValueDef::ColorsOfManaSpent
            | ValueDef::PaidAmount
            | ValueDef::MatchedCount
            | ValueDef::MatchedCardTypes
            | ValueDef::MatchedManaValue
            | ValueDef::BoundObjectCount(_)
            | ValueDef::SpellsCastBeforeThisTurn
            | ValueDef::PlayerCounters { .. }
            | ValueDef::SacrificedManaValue
            | ValueDef::TimesAdditionalCostPaid
            | ValueDef::DividedAmongTargets
            | ValueDef::TargetPower(_)
            | ValueDef::TargetToughness(_)
            | ValueDef::TargetLibrarySize(_)
            | ValueDef::LifeTotal(_)
            | ValueDef::TargetManaValue(_)
            | ValueDef::IfCreatureDiedThisTurn(_)
            | ValueDef::IfControllerLifeAtMost(_)
            | ValueDef::IfTargetMatches(_)
            | ValueDef::IfMatchingObjectCount(_)
            | ValueDef::Negate(_)
            | ValueDef::Scaled(_)
            | ValueDef::Halved(_)
            | ValueDef::Sum(_)
            | ValueDef::CreaturesDiedThisTurn
            | ValueDef::OpponentsWhoLostLifeThisTurn
            | ValueDef::CardTypesAmongGraveyards(_)
            | ValueDef::IfCardTypesAmongGraveyards(_) => None,
        }
    }
}
