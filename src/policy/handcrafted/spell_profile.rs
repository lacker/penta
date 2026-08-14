use super::{
    AbilityCostDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    BasicLandType, CardBehavior, CardDefinitionId, CardType, CardTypeSet, CastChoices,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, HandcraftedPolicy, ObjectPredicateDef,
    PlayerRelation, SpellForm, ValueDef, ZoneKind,
};

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
            Self::collect_spell_effect_profile(
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
        Self::collect_spell_effect_profile(
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
            Self::collect_spell_effect_profile(
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
        matches!(
            recipient,
            EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(crate::CardType::Creature),
                zones: [ZoneKind::Battlefield],
                controller: PlayerRelation::Opponent | PlayerRelation::NotYou,
            }
        )
    }

    /// A destroy is removal; a destroy aimed at every creature on the
    /// battlefield, whoever controls it, is a sweeper as well.
    pub(super) fn collect_destroy_profile(
        object: EffectRecipientDef,
        profile: &mut DeclarativeSpellProfile,
    ) {
        profile.mark(DeclarativeSpellProfile::REMOVES);
        if let EffectRecipientDef::MatchingObjects {
            object,
            zones,
            controller,
        } = object
            && zones == [ZoneKind::Battlefield]
            && controller == PlayerRelation::Any
        {
            let destroyed_types = Self::globally_destroyed_types(object);
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
            EffectDef::ChooseDamageSource { then, .. }
            | EffectDef::ChoosePermanent { then, .. } => {
                Self::collect_spell_effect_profile(*then, x, targets, profile);
            }
            // An optional effect is worth what it would do if taken.
            EffectDef::May { effect, .. } => {
                Self::collect_spell_effect_profile(*effect, x, targets, profile);
            }
            EffectDef::DealDamage { recipient, amount }
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
            EffectDef::Counter { object, .. } => {
                profile.mark(DeclarativeSpellProfile::COUNTERS);
                profile.opponent_spell_sweep |= matches!(
                    object,
                    EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::Spell,
                        zones: [ZoneKind::Stack],
                        controller: PlayerRelation::Opponent | PlayerRelation::NotYou,
                    }
                );
            }
            EffectDef::CounterUnlessPaid { .. } => {
                profile.mark(DeclarativeSpellProfile::COUNTERS);
            }
            EffectDef::Destroy { object, .. } => Self::collect_destroy_profile(object, profile),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(target),
                zone: ZoneKind::Exile,
                ..
            } if Self::target_slot_is_on_battlefield(targets, target.index()) => {
                profile.mark(DeclarativeSpellProfile::REMOVES);
            }
            EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::PreventAllCombatDamageThisTurn
            | EffectDef::PreventNextDamage { .. }
            | EffectDef::PreventAllDamageThisTurn { .. }
            | EffectDef::PreventNextDamageFromSource { .. }
            | EffectDef::PreventCombatDamageThisTurn { .. }
            | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
            | EffectDef::PreventDamageToPlayerAndControlledCreaturesThisTurn { .. }
            | EffectDef::PreventAllCombatDamageExceptSourceThisTurn { .. } => {
                profile.mark(DeclarativeSpellProfile::TAPS);
            }
            EffectDef::Apply { .. } => profile.mark(DeclarativeSpellProfile::APPLIES),
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            } => profile.mark(DeclarativeSpellProfile::EXTRA_TURN),
            // Nothing outranks winning, so it needs no profile of its own.
            EffectDef::LoseTheGame { .. }
            | EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Regenerate { .. }
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
            | EffectDef::IfFormat { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalPayment { .. }
            | EffectDef::UnlessPaid { .. }
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
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => {}
        }
    }

    pub(super) fn policy_value(value: ValueDef, x: u16) -> Option<u16> {
        match value {
            ValueDef::Constant(value) => u16::try_from(value).ok(),
            ValueDef::ChosenX => Some(x),
            // Board-dependent values are not knowable from the definition
            // alone, so the caller falls back to its own heuristics.
            ValueDef::SourcePower
            | ValueDef::SourceToughness
            | ValueDef::TriggerEventAmount
            | ValueDef::CardsInHandAbove { .. }
            | ValueDef::CountMatchingObjects(_)
            | ValueDef::AnyMatchingObject(_)
            | ValueDef::CountersOnSource(_)
            | ValueDef::DividedAmongTargets
            | ValueDef::TargetPower(_)
            | ValueDef::TargetManaValue(_)
            | ValueDef::IfCreatureDiedThisTurn(_)
            | ValueDef::IfTargetMatches(_)
            | ValueDef::IfMatchingObjectCount(_)
            | ValueDef::Negate(_)
            | ValueDef::Scaled(_) => None,
        }
    }
}
