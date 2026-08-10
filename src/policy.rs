//! Bot policies and deterministic game-running utilities.

use std::error::Error;
use std::fmt;

use crate::card::{
    AbilityCostDef, AlternativeCastKindDef, BasicLandType, CardBehavior, CardCatalog,
    CardSupertype, CardType, CardTypeSet, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, SpellForm, ValueDef, ZoneKind,
};
use crate::game::{
    DecisionObservation, DecisionOption, DecisionPreference, DecisionZone, Game, GameResult,
    PlayerObservation, StackObjectKind, StackObservation, Step,
};
use crate::{
    AbilityOrigin, Action, ActionError, CardDefinitionId, CastChoices, GameObjectId, PlayerId,
    Target,
};

/// Chooses one of the actions in a player's current observation.
pub trait Policy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action>;
}

/// Selects uniformly from the non-concession legal actions using a seeded PRNG.
#[derive(Clone, Debug)]
pub struct RandomPolicy {
    state: u64,
}

impl RandomPolicy {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}

impl Policy for RandomPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            let mut options = decision
                .options
                .iter()
                .map(|option| option.id)
                .collect::<Vec<_>>();
            if options.len() < decision.minimum {
                return None;
            }
            for index in (1..options.len()).rev() {
                let index_u64 = u64::try_from(index + 1).unwrap_or(u64::MAX);
                let offset = usize::try_from(self.next_u64() % index_u64).unwrap_or(0);
                options.swap(index, offset);
            }
            let count = if decision.minimum == decision.maximum {
                decision.minimum
            } else {
                let span = decision.maximum - decision.minimum + 1;
                let offset =
                    usize::try_from(self.next_u64() % u64::try_from(span).unwrap_or(u64::MAX))
                        .unwrap_or(0);
                decision.minimum + offset
            };
            return Some(Action::ChooseDecision {
                decision: decision.id,
                options: options.into_iter().take(count).collect(),
            });
        }
        let choices: Vec<_> = observation
            .legal_actions
            .iter()
            .filter(|action| !matches!(action, Action::Concede))
            .collect();
        if choices.is_empty() {
            return observation.legal_actions.first().cloned();
        }
        let choice_count = u64::try_from(choices.len()).unwrap_or(u64::MAX);
        let unbiased_range = u64::MAX - u64::MAX % choice_count;
        loop {
            let value = self.next_u64();
            if value < unbiased_range {
                let index = usize::try_from(value % choice_count).unwrap_or(0);
                return Some(choices[index].clone());
            }
        }
    }
}

/// A deterministic baseline that applies simple card- and combat-aware rules.
#[derive(Clone, Debug)]
pub struct HandcraftedPolicy {
    catalog: CardCatalog,
    mulligans_taken: u8,
}

#[derive(Clone, Copy, Debug, Default)]
struct DeclarativeSpellProfile {
    damage: Option<u16>,
    cards_drawn: Option<u16>,
    effect_kinds: u8,
    /// Whether the activation taps its own source. A land that taps to pump
    /// is spending the mana it could have made.
    taps_source: bool,
    opponent_creature_sweep: bool,
    opponent_spell_sweep: bool,
}

impl DeclarativeSpellProfile {
    const COUNTERS: u8 = 1 << 0;
    const REMOVES: u8 = 1 << 1;
    const TAPS: u8 = 1 << 2;
    const APPLIES: u8 = 1 << 3;
    const SWEEPS_CREATURES: u8 = 1 << 4;

    fn mark(&mut self, effect_kind: u8) {
        self.effect_kinds |= effect_kind;
    }

    const fn has(self, effect_kind: u8) -> bool {
        self.effect_kinds & effect_kind != 0
    }
}

impl HandcraftedPolicy {
    #[must_use]
    pub fn new(catalog: CardCatalog) -> Self {
        Self {
            catalog,
            mulligans_taken: 0,
        }
    }

    fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog
            .get(definition)
            .and_then(|card| card.rules.special_behavior())
    }

    fn is_mana_source(&self, definition: CardDefinitionId) -> bool {
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

    fn declarative_mana_value(&self, definition: CardDefinitionId) -> Option<i32> {
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

    fn declarative_spell_profile(
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
                &mut profile,
            );
        }
        Some(profile)
    }

    fn collect_spell_effect_profile(
        effect: EffectDef,
        x: u16,
        profile: &mut DeclarativeSpellProfile,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    Self::collect_spell_effect_profile(*effect, x, profile);
                }
            }
            // An optional effect is worth what it would do if taken.
            EffectDef::May(inner) => Self::collect_spell_effect_profile(*inner, x, profile),
            EffectDef::DealDamage { recipient, amount } => {
                profile.damage = Self::policy_value(amount, x);
                profile.opponent_creature_sweep |= matches!(
                    recipient,
                    EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::HasType(crate::CardType::Creature),
                        zones: [ZoneKind::Battlefield],
                        controller: PlayerRelation::Opponent | PlayerRelation::NotYou,
                    }
                );
            }
            EffectDef::DrawCards { amount, .. } => {
                profile.cards_drawn = Self::policy_value(amount, x);
            }
            // Looting is card selection, not card advantage, so the discard
            // cancels out the draw the policy would otherwise reward.
            EffectDef::DiscardCards { amount, .. } => {
                if let Some(drawn) = profile.cards_drawn {
                    profile.cards_drawn =
                        Some(drawn.saturating_sub(Self::policy_value(amount, x).unwrap_or(0)));
                }
            }
            EffectDef::Counter { object } => {
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
            EffectDef::Destroy { object, .. } => {
                profile.mark(DeclarativeSpellProfile::REMOVES);
                if let EffectRecipientDef::MatchingObjects {
                    object,
                    zones,
                    controller,
                } = object
                    && object == ObjectPredicateDef::HasType(CardType::Creature)
                    && zones == [ZoneKind::Battlefield]
                    && controller == PlayerRelation::Any
                {
                    profile.mark(DeclarativeSpellProfile::SWEEPS_CREATURES);
                }
            }
            EffectDef::Tap { .. } | EffectDef::Untap { .. } => {
                profile.mark(DeclarativeSpellProfile::TAPS);
            }
            EffectDef::Apply { .. } => profile.mark(DeclarativeSpellProfile::APPLIES),
            EffectDef::None
            | EffectDef::AddMana(_)
            | EffectDef::GainLife { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
            | EffectDef::Mill { .. }
            | EffectDef::SearchLibrary { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalManaPayment { .. }
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::Transform { .. }
            | EffectDef::AdditionalCombatPhase
            | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::GainControlThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::Replacement(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => {}
        }
    }

    fn policy_value(value: ValueDef, x: u16) -> Option<u16> {
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
            | ValueDef::IfCreatureDiedThisTurn(_)
            | ValueDef::IfTargetMatches(_)
            | ValueDef::IfMatchingObjectCount(_)
            | ValueDef::Negate(_) => None,
        }
    }

    fn hand_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .hand
            .iter()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
    }

    fn graveyard_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .graveyards
            .iter()
            .flatten()
            .find_map(|(candidate, definition)| (*candidate == id).then_some(*definition))
    }

    fn permanent_definition(
        observation: &PlayerObservation,
        id: GameObjectId,
    ) -> Option<CardDefinitionId> {
        observation
            .battlefield
            .iter()
            .find_map(|permanent| (permanent.id == id).then_some(permanent.definition))
    }

    /// Spells whose whole purpose is to remove something the opponent
    /// controls. Pump and recursion also take a permanent target, so they are
    /// deliberately excluded — those want a friendly target.
    fn is_already_a_creature(observation: &PlayerObservation, id: GameObjectId) -> bool {
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == id && permanent.power.is_some())
    }

    /// Whether an ability's target is attacking or blocking right now, which
    /// is the only time a until-end-of-turn pump changes anything.
    fn source_is_attacking(observation: &PlayerObservation, source: GameObjectId) -> bool {
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == source && permanent.attacking)
    }

    fn target_is_fighting(observation: &PlayerObservation, target: Option<Target>) -> bool {
        let Some(Target::Permanent(id)) = target else {
            return false;
        };
        observation.battlefield.iter().any(|permanent| {
            permanent.id == id && (permanent.attacking || permanent.blocking.is_some())
        })
    }

    fn stack_spell_is_already_answered(
        observation: &PlayerObservation,
        spell: GameObjectId,
    ) -> bool {
        observation.stack.iter().any(|counter| {
            counter.controller == observation.viewer
                && counter.targets.contains(&Target::Spell(spell))
        })
    }

    fn is_effective_counter_target(
        observation: &PlayerObservation,
        object: &StackObservation,
    ) -> bool {
        object.kind == StackObjectKind::Spell
            && object.controller == observation.viewer.opponent()
            && object.counterable
            && !Self::stack_spell_is_already_answered(observation, object.id)
    }

    fn counter_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Spell(id) => observation
                .stack
                .iter()
                .find(|object| object.id == id)
                .map_or(-10_000, |object| {
                    if Self::is_effective_counter_target(observation, object) {
                        2_000
                    } else {
                        -10_000
                    }
                }),
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        1_000
                    }
                }),
            Target::Player(_) | Target::Card(_) => -10_000,
        }
    }

    /// A sweeper is worth casting in proportion to how far behind on board it
    /// leaves you -- which is to say, not at all when you are ahead. Without
    /// this the policy treats a wrath as an ordinary sorcery and fires it into
    /// its own creatures.
    fn sweeper_score(observation: &PlayerObservation) -> i32 {
        let count = |controller: PlayerId| {
            i32::try_from(
                observation
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        permanent.controller == controller && permanent.power.is_some()
                    })
                    .count(),
            )
            .unwrap_or(0)
        };
        let swing = count(observation.viewer.opponent()) - count(observation.viewer);
        if swing <= 0 {
            -10_000
        } else {
            6_500 + swing * 500
        }
    }

    fn is_hostile_removal(behavior: Option<CardBehavior>) -> bool {
        matches!(
            behavior,
            Some(
                CardBehavior::SwordsToPlowshares
                    | CardBehavior::DivineOffering
                    | CardBehavior::Terror
                    | CardBehavior::DustToDust
                    | CardBehavior::Detonate
            )
        )
    }

    /// Removal aimed at your own board is never worth its base score, so the
    /// penalty has to cancel that base outright rather than merely rank below
    /// a hostile target that may not exist.
    fn removal_target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-10_000, |permanent| {
                    if permanent.controller == observation.viewer {
                        -10_000
                    } else {
                        250 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    }
                }),
            Target::Player(_) | Target::Card(_) | Target::Spell(_) => -10_000,
        }
    }

    fn target_score(observation: &PlayerObservation, target: Target) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => 500,
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        -500
                    } else {
                        250
                    }
                }),
            Target::Card(_) | Target::Spell(_) => 100,
        }
    }

    fn damage_target_score(observation: &PlayerObservation, target: Target, amount: u16) -> i32 {
        match target {
            Target::Player(player) if player == observation.viewer.opponent() => {
                if observation.life_totals[player.index()]
                    <= i16::try_from(amount).unwrap_or(i16::MAX)
                {
                    10_000
                } else {
                    -2_000
                }
            }
            Target::Player(_) => -10_000,
            Target::Permanent(id) => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == id)
                .map_or(-500, |permanent| {
                    if permanent.controller == observation.viewer {
                        return -10_000;
                    }
                    let remaining = permanent
                        .toughness
                        .unwrap_or(0)
                        .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                    if i16::try_from(amount).unwrap_or(i16::MAX) >= remaining {
                        700 + i32::from(permanent.power.unwrap_or(0).max(0)) * 25
                    } else {
                        100
                    }
                }),
            Target::Card(_) | Target::Spell(_) => -500,
        }
    }

    fn card_value(&self, definition: CardDefinitionId) -> i32 {
        if let Some(value) = self.declarative_mana_value(definition) {
            return value;
        }
        match self.behavior(definition) {
            Some(CardBehavior::LightningBolt | CardBehavior::GoblinGrenade) => 75,
            Some(behavior) if behavior.types().is_creature() => 65,
            Some(_) => 55,
            None => self.catalog.get(definition).map_or(0, |card| {
                if card.rules.has_type(CardType::Creature) {
                    65
                } else {
                    55
                }
            }),
        }
    }

    fn cast_target_score(
        observation: &PlayerObservation,
        target: Target,
        cards_drawn: Option<u16>,
        counters: bool,
        removes: bool,
        damage: Option<u16>,
    ) -> i32 {
        if let Some(cards_drawn) = cards_drawn {
            return match target {
                Target::Player(player) if player == observation.viewer => {
                    if usize::from(cards_drawn) > observation.library_sizes[player.index()] {
                        -20_000
                    } else {
                        1_000 + i32::from(cards_drawn) * 100
                    }
                }
                Target::Player(player) => {
                    if usize::from(cards_drawn) > observation.library_sizes[player.index()] {
                        20_000
                    } else {
                        -10_000
                    }
                }
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => -10_000,
            };
        }
        if counters {
            return Self::counter_target_score(observation, target);
        }
        if removes {
            return Self::removal_target_score(observation, target);
        }
        damage.map_or_else(
            || Self::target_score(observation, target),
            |amount| Self::damage_target_score(observation, target, amount),
        )
    }

    fn score_cast(
        &self,
        observation: &PlayerObservation,
        card: GameObjectId,
        choices: &CastChoices,
    ) -> i32 {
        let definition = Self::hand_definition(observation, card)
            .or_else(|| Self::graveyard_definition(observation, card));
        let behavior = definition.and_then(|id| self.behavior(id));
        let declarative = definition.and_then(|id| self.declarative_spell_profile(id, choices));
        let kind = definition
            .and_then(|id| self.catalog.get(id))
            .map(|card| card.rules.types());
        let x = choices.x();
        let damage = match behavior {
            Some(CardBehavior::LightningBolt | CardBehavior::ChainLightning) => Some(3),
            Some(CardBehavior::PillarOfFlame) => Some(2),
            Some(CardBehavior::GoblinGrenade) => Some(5),
            Some(CardBehavior::Fireball) => Some(
                x.checked_div(u16::try_from(choices.iter_targets().count()).unwrap_or(u16::MAX))
                    .unwrap_or(0),
            ),
            _ => declarative.and_then(|profile| profile.damage),
        };
        let cards_drawn = declarative.and_then(|profile| profile.cards_drawn);
        let counters =
            declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::COUNTERS));
        let removes = declarative
            .is_some_and(|profile| profile.has(DeclarativeSpellProfile::REMOVES))
            || Self::is_hostile_removal(behavior);
        let sweeps_creatures = declarative
            .is_some_and(|profile| profile.has(DeclarativeSpellProfile::SWEEPS_CREATURES));
        let target_score: i32 = choices
            .iter_targets()
            .map(|target| {
                Self::cast_target_score(
                    observation,
                    *target,
                    cards_drawn,
                    counters,
                    removes,
                    damage,
                )
            })
            .sum();
        let opponent_creatures = i32::try_from(
            observation
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == observation.viewer.opponent()
                        && permanent.power.is_some()
                })
                .count(),
        )
        .unwrap_or(i32::MAX);
        let opponent_spells = i32::try_from(
            observation
                .stack
                .iter()
                .filter(|object| Self::is_effective_counter_target(observation, object))
                .count(),
        )
        .unwrap_or(i32::MAX);
        let base = match behavior {
            Some(CardBehavior::SwordsToPlowshares) => 8_400,
            Some(CardBehavior::TimeWalk) => 8_300,
            Some(CardBehavior::GoblinGrenade) => 8_500,
            Some(CardBehavior::LightningBolt | CardBehavior::ChainLightning) => 8_000,
            Some(CardBehavior::PillarOfFlame) => 7_800,
            Some(CardBehavior::Fireball) => 7_900 + i32::from(x) * 20,
            Some(CardBehavior::Detonate | CardBehavior::ChaosOrb) => 7_400,
            Some(CardBehavior::Fork) => 7_300,
            Some(CardBehavior::WheelOfFortune) => 6_600,
            Some(behavior) if behavior.types().is_permanent() => 6_800,
            _ if sweeps_creatures => Self::sweeper_score(observation),
            _ if declarative.is_some_and(|profile| profile.opponent_creature_sweep) => {
                if opponent_creatures == 0 {
                    -10_000
                } else {
                    7_500 + opponent_creatures * 900
                }
            }
            _ if declarative.is_some_and(|profile| profile.opponent_spell_sweep) => {
                if opponent_spells == 0 {
                    -10_000
                } else if opponent_spells == 1 {
                    6_000 + opponent_spells * 500
                } else {
                    8_900 + opponent_spells * 2_000
                }
            }
            _ if cards_drawn.is_some_and(|amount| amount >= 3) => 9_200,
            _ if counters => 8_900,
            _ if removes => 8_400,
            _ if damage.is_some() => 8_000,
            None if kind.is_some_and(CardTypeSet::is_permanent) => 6_800,
            Some(_) | None => 6_200,
        };
        base + target_score
    }

    fn activated_target_score(
        observation: &PlayerObservation,
        target: Target,
        declarative: Option<DeclarativeSpellProfile>,
    ) -> i32 {
        if let Some(amount) = declarative.and_then(|profile| profile.damage) {
            return Self::damage_target_score(observation, target, amount);
        }
        if declarative.is_some_and(|profile| {
            profile.has(DeclarativeSpellProfile::REMOVES | DeclarativeSpellProfile::TAPS)
        }) {
            return Self::removal_target_score(observation, target);
        }
        if declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::APPLIES)) {
            return match target {
                Target::Permanent(id) => observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == id)
                    .map_or(-10_000, |permanent| {
                        let useful_combat_window = observation.step == Step::DeclareBlockers
                            || (observation.step == Step::CombatDamage
                                && observation.regular_combat_damage_pending);
                        if permanent.controller == observation.viewer
                            && useful_combat_window
                            && (permanent.attacking || permanent.blocking.is_some())
                        {
                            1_500
                        } else {
                            -10_000
                        }
                    }),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => -10_000,
            };
        }
        Self::target_score(observation, target)
    }

    fn discard_source_cost(
        &self,
        source_definition: Option<CardDefinitionId>,
        origin: AbilityOrigin,
    ) -> i32 {
        let Some(source_definition) = source_definition else {
            return 0;
        };
        let AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } = origin
        else {
            return 0;
        };
        if definition != source_definition {
            return 0;
        }
        self.catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .and_then(|ability| match ability.definition {
                DeclarativeAbilityDef::Activated(definition)
                    if definition.costs.contains(&AbilityCostDef::DiscardSource) =>
                {
                    Some(self.card_value(source_definition))
                }
                _ => None,
            })
            .unwrap_or(0)
    }

    fn score_ability(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: &[crate::TargetSelection],
        sacrifice: Option<GameObjectId>,
        x: u16,
    ) -> i32 {
        let source_definition = Self::permanent_definition(observation, source)
            .or_else(|| Self::hand_definition(observation, source));
        let behavior = source_definition.and_then(|id| self.behavior(id));
        let declarative = source_definition
            .and_then(|definition| self.declarative_activated_profile(definition, ability));
        let target = targets
            .iter()
            .flat_map(crate::TargetSelection::targets)
            .next()
            .copied();
        let target_score = targets
            .iter()
            .flat_map(crate::TargetSelection::targets)
            .copied()
            .map(|value| Self::activated_target_score(observation, value, declarative))
            .sum::<i32>();
        let sacrifice_cost = sacrifice
            .filter(|card| *card != source)
            .and_then(|card| Self::permanent_definition(observation, card))
            .map_or(0, |definition| self.card_value(definition));
        let discard_source_cost = self.discard_source_cost(source_definition, ability);
        let score = match behavior {
            Some(CardBehavior::ChaosOrb) => 7_200 + target_score,
            // Animating a Factory that is already a creature does nothing but
            // spend mana, so only the +1/+1 mode stays repeatable.
            Some(CardBehavior::MishrasFactory)
                if target.is_none() && Self::is_already_a_creature(observation, source) =>
            {
                -100
            }
            // The +1/+1 costs the Factory its tap, so it only pays for itself
            // once the creature it feeds is in combat. Pumping earlier — most
            // often itself — spends the attack it was about to make.
            Some(CardBehavior::MishrasFactory)
                if target.is_some() && !Self::target_is_fighting(observation, target) =>
            {
                -100
            }
            Some(CardBehavior::MishrasFactory) => 5_800 + target_score,
            Some(CardBehavior::DragonWhelp) => 5_200,
            Some(CardBehavior::Atog) if self.atog_can_attack_for_lethal(observation, source) => {
                10_000
            }
            Some(CardBehavior::Atog) => -100,
            Some(_) => 4_500 + target_score,
            None if declarative.is_some_and(|profile| {
                profile.has(DeclarativeSpellProfile::REMOVES | DeclarativeSpellProfile::TAPS)
            }) =>
            {
                7_200 + target_score
            }
            None if declarative.is_some_and(|profile| profile.cards_drawn.is_some()) => 6_500,
            None if self.ability_spends_mana_on_nothing(
                observation,
                source,
                ability,
                source_definition,
                declarative,
                target,
            ) =>
            {
                -100
            }
            None if declarative.is_some_and(|profile| {
                profile.taps_source && profile.has(DeclarativeSpellProfile::APPLIES)
            }) =>
            {
                5_200 + target_score + i32::from(x) * 100
            }
            None if declarative.is_some_and(|profile| profile.damage.is_some()) => {
                7_200 + target_score
            }
            None if declarative
                .is_some_and(|profile| profile.has(DeclarativeSpellProfile::APPLIES)) =>
            {
                5_200 + target_score
            }
            None if declarative.is_some() => 4_500 + target_score,
            None => -10_000,
        };
        if sacrifice.is_some()
            && let Some(amount) = declarative.and_then(|profile| profile.damage)
            && matches!(target, Some(Target::Player(player)) if player == observation.viewer.opponent())
            && observation.life_totals[observation.viewer.opponent().index()]
                > i16::try_from(amount).unwrap_or(i16::MAX)
        {
            return -1_000;
        }
        score - sacrifice_cost - discard_source_cost
    }

    /// Every reason a greedy policy should decline an activated ability
    /// outright rather than pay for it. Each of these spends mana for a board
    /// that is no better, which is worse than passing.
    fn ability_spends_mana_on_nothing(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        ability: AbilityOrigin,
        source_definition: Option<CardDefinitionId>,
        declarative: Option<DeclarativeSpellProfile>,
        target: Option<Target>,
    ) -> bool {
        // A bonus that gives with one hand and takes with the other is only
        // worth mana in spots a greedy policy cannot see, and exiling your own
        // creature takes it off the board now for a return later.
        if source_definition.is_some_and(|definition| self.ability_is_a_wash(definition, ability)) {
            return true;
        }
        if source_definition
            .is_some_and(|definition| self.ability_only_buys_evasion(definition, ability))
            && !Self::source_is_attacking(observation, source)
        {
            return true;
        }
        // The same reasoning as Mishra's Factory, one step more general: an
        // ability that taps its source to pump spends whatever that source was
        // going to do, so it only pays for itself on a creature already in
        // combat.
        if declarative.is_some_and(|profile| {
            profile.taps_source && profile.has(DeclarativeSpellProfile::APPLIES)
        }) && !Self::target_is_fighting(observation, target)
        {
            return true;
        }
        // Animating a land turns a mana source into a creature that can be
        // killed, and the creature is worth nothing unless it can attack.
        if source_definition
            .is_some_and(|definition| self.ability_animates_the_source(definition, ability))
            && !Self::can_attack_this_combat(observation, source)
        {
            return true;
        }
        // An ability whose whole payoff is conditional on what it points at
        // does nothing when the condition fails.
        source_definition
            .is_some_and(|definition| self.ability_needs_a_matching_target(definition, ability))
            && !self.ability_target_matches_condition(
                observation,
                source_definition,
                ability,
                target,
            )
    }

    /// Whether the ability turns its own source into a creature.
    fn ability_animates_the_source(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> bool {
        let AbilityOrigin::Printed {
            definition: origin_definition,
            part,
            ability,
        } = origin
        else {
            return false;
        };
        if origin_definition != definition {
            return false;
        }
        self.catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .is_some_and(|ability| {
                matches!(
                    ability.declarative_effect(),
                    Some(EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: crate::card::AppliedEffectDef::Animate(_),
                        ..
                    })
                )
            })
    }

    /// Whether this permanent could still be declared as an attacker this
    /// turn. An untapped permanent in the viewer's own pre-attack combat
    /// steps is the window worth spending mana in.
    fn can_attack_this_combat(observation: &PlayerObservation, source: GameObjectId) -> bool {
        observation.active_player == observation.viewer
            && matches!(
                observation.step,
                Step::BeginningOfCombat | Step::DeclareAttackers
            )
            && observation
                .battlefield
                .iter()
                .any(|permanent| permanent.id == source && !permanent.tapped)
    }

    /// Whether the ability changes nothing a greedy policy can use: a
    /// power and toughness swap that nets zero, or exiling its own source.
    fn ability_is_a_wash(&self, definition: CardDefinitionId, origin: AbilityOrigin) -> bool {
        let AbilityOrigin::Printed {
            definition: origin_definition,
            part,
            ability,
        } = origin
        else {
            return false;
        };
        if origin_definition != definition {
            return false;
        }
        self.catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .and_then(|ability| ability.declarative_effect())
            .is_some_and(Self::effect_is_a_wash)
    }

    fn effect_is_a_wash(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects.iter().copied().any(Self::effect_is_a_wash),
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Source,
            } => true,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect:
                    crate::card::AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(power),
                        toughness: ValueDef::Constant(toughness),
                    },
                ..
            } => power + toughness == 0,
            _ => false,
        }
    }

    /// Whether the ability only buys evasion, which is worth nothing until
    /// the creature is actually attacking.
    fn ability_only_buys_evasion(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> bool {
        let AbilityOrigin::Printed {
            definition: origin_definition,
            part,
            ability,
        } = origin
        else {
            return false;
        };
        if origin_definition != definition {
            return false;
        }
        self.catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .is_some_and(|ability| {
                matches!(
                    ability.declarative_effect(),
                    Some(EffectDef::MakeUnblockableThisTurn {
                        object: EffectRecipientDef::Source
                    })
                )
            })
    }

    /// Whether every value in the ability's effect is conditional on the
    /// target, which is what makes a mismatched target worthless.
    fn ability_needs_a_matching_target(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> bool {
        self.activated_target_condition(definition, origin)
            .is_some()
    }

    /// The condition an ability's payoff hangs on, if it has exactly one.
    fn activated_target_condition(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> Option<&'static crate::card::TargetConditionDef> {
        let AbilityOrigin::Printed {
            definition: origin_definition,
            part,
            ability,
        } = origin
        else {
            return None;
        };
        if origin_definition != definition {
            return None;
        }
        let ability = self
            .catalog
            .get(definition)?
            .part(part)?
            .rules
            .ability(ability)?;
        ability
            .declarative_effect()
            .and_then(Self::target_condition_in)
    }

    /// The first target condition an effect hangs a value on.
    fn target_condition_in(effect: EffectDef) -> Option<&'static crate::card::TargetConditionDef> {
        match effect {
            EffectDef::Sequence(effects) => {
                effects.iter().copied().find_map(Self::target_condition_in)
            }
            EffectDef::OptionalManaPayment { effect, .. }
            | EffectDef::May(effect)
            | EffectDef::AtNextStep { effect, .. } => Self::target_condition_in(*effect),
            EffectDef::AddCounters { amount, .. } | EffectDef::GainLife { amount, .. } => {
                match amount {
                    ValueDef::IfTargetMatches(condition) => Some(condition),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Whether the chosen target satisfies that condition.
    fn ability_target_matches_condition(
        &self,
        observation: &PlayerObservation,
        definition: Option<CardDefinitionId>,
        origin: AbilityOrigin,
        target: Option<Target>,
    ) -> bool {
        let Some(condition) =
            definition.and_then(|definition| self.activated_target_condition(definition, origin))
        else {
            return true;
        };
        let ObjectPredicateDef::HasType(expected) = condition.object else {
            return true;
        };
        let Some(Target::Card(id)) = target else {
            return true;
        };
        observation
            .graveyards
            .iter()
            .flatten()
            .find(|(card, _)| *card == id)
            .and_then(|(_, definition)| self.catalog.get(*definition))
            .is_some_and(|card| card.rules.has_type(expected))
    }

    fn declarative_activated_profile(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> Option<DeclarativeSpellProfile> {
        let AbilityOrigin::Printed {
            definition: origin_definition,
            part,
            ability,
        } = origin
        else {
            return None;
        };
        if origin_definition != definition {
            return None;
        }
        let ability = self
            .catalog
            .get(definition)?
            .part(part)?
            .rules
            .ability(ability)?;
        if !ability.is_executable()
            || !matches!(ability.definition, DeclarativeAbilityDef::Activated(_))
        {
            return None;
        }
        let mut profile = DeclarativeSpellProfile::default();
        if let DeclarativeAbilityDef::Activated(definition) = ability.definition {
            profile.taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
        }
        Self::collect_spell_effect_profile(ability.declarative_effect()?, 0, &mut profile);
        Some(profile)
    }

    fn atog_can_attack_for_lethal(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
    ) -> bool {
        let Some(atog) = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == source)
        else {
            return false;
        };
        if !atog.attacking
            || observation
                .battlefield
                .iter()
                .any(|permanent| permanent.blocking == Some(source))
        {
            return false;
        }
        let artifacts = observation
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == observation.viewer
                    && self
                        .catalog
                        .get(permanent.definition)
                        .is_some_and(|card| card.rules.has_type(CardType::Artifact))
            })
            .count();
        let potential_power = atog
            .power
            .unwrap_or(0)
            .saturating_add(i16::try_from(artifacts.saturating_mul(2)).unwrap_or(i16::MAX));
        potential_power >= observation.life_totals[observation.viewer.opponent().index()]
    }

    fn score_attack(&self, observation: &PlayerObservation, attacker: GameObjectId) -> i32 {
        let Some(attacker) = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == attacker)
        else {
            return -1_000;
        };
        let attacker_power = attacker.power.unwrap_or(0).max(0);
        let attacker_toughness = attacker
            .toughness
            .unwrap_or(0)
            .saturating_sub(i16::try_from(attacker.damage).unwrap_or(i16::MAX));
        let already_attacking = observation
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == observation.viewer && permanent.attacking)
            .count();
        let blockers: Vec<_> = observation
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == observation.viewer.opponent()
                    && !permanent.tapped
                    && permanent.power.is_some()
                    && (!attacker.flying || permanent.flying)
                    && !(self.behavior(permanent.definition) == Some(CardBehavior::IronclawOrcs)
                        && attacker_power >= 2)
            })
            .collect();
        if already_attacking >= blockers.len() {
            return 7_000;
        }
        let gets_eaten = blockers.iter().any(|blocker| {
            let blocker_power = blocker.power.unwrap_or(0).max(0);
            let blocker_toughness = blocker
                .toughness
                .unwrap_or(0)
                .saturating_sub(i16::try_from(blocker.damage).unwrap_or(i16::MAX));
            blocker_power >= attacker_toughness && blocker_toughness > attacker_power
        });
        if gets_eaten { 500 } else { 7_000 }
    }

    fn score_block(
        observation: &PlayerObservation,
        blocker: GameObjectId,
        attacker: GameObjectId,
    ) -> i32 {
        let blocker = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == blocker);
        let attacker = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == attacker);
        let (Some(blocker), Some(attacker)) = (blocker, attacker) else {
            return 0;
        };
        let blocker_power = blocker.power.unwrap_or(0);
        let blocker_toughness =
            blocker.toughness.unwrap_or(0) - i16::try_from(blocker.damage).unwrap_or(i16::MAX);
        let attacker_power = attacker.power.unwrap_or(0);
        let attacker_toughness =
            attacker.toughness.unwrap_or(0) - i16::try_from(attacker.damage).unwrap_or(i16::MAX);
        let existing_power: i16 = observation
            .battlefield
            .iter()
            .filter(|permanent| permanent.blocking == Some(attacker.id))
            .filter_map(|permanent| permanent.power)
            .fold(0, i16::saturating_add);
        if existing_power >= attacker_toughness {
            return 0;
        }
        let kills = existing_power.saturating_add(blocker_power) >= attacker_toughness;
        let survives = blocker_toughness > attacker_power;
        match (kills, survives) {
            (true, true) => 7_000,
            (true, false) => 6_000,
            (false, true) => 4_000,
            (false, false) if attacker_power >= 4 => 2_000,
            (false, false) => 500,
        }
    }

    fn score_assignment(
        observation: &PlayerObservation,
        assignments: &[crate::CombatDamageAssignment],
    ) -> i32 {
        assignments
            .iter()
            .map(|assignment| match assignment.recipient {
                Target::Player(player) if player == observation.viewer.opponent() => {
                    i32::from(assignment.amount) * 200
                }
                Target::Permanent(id) => observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == id)
                    .map_or(0, |permanent| {
                        let remaining = permanent
                            .toughness
                            .unwrap_or(0)
                            .saturating_sub(i16::try_from(permanent.damage).unwrap_or(i16::MAX));
                        if i16::try_from(assignment.amount).unwrap_or(i16::MAX) >= remaining {
                            500
                        } else {
                            i32::from(assignment.amount)
                        }
                    }),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => 0,
            })
            .sum()
    }

    fn should_mulligan(&self, observation: &PlayerObservation) -> bool {
        if self.mulligans_taken >= 2 {
            return false;
        }
        let mana_sources = observation
            .hand
            .iter()
            .filter(|(_, definition)| self.is_mana_source(*definition))
            .count();
        !(2..=5).contains(&mana_sources)
    }

    fn mana_action_score(&self, observation: &PlayerObservation, source: GameObjectId) -> i32 {
        let needs_factory_mana = observation.active_player == observation.viewer
            && matches!(
                observation.step,
                Step::BeginningOfCombat | Step::DeclareAttackers
            )
            && observation.mana_pools[observation.viewer.index()].total() == 0
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && !permanent.tapped
                    && permanent.power.is_none()
                    && self.behavior(permanent.definition) == Some(CardBehavior::MishrasFactory)
            });
        if needs_factory_mana
            && Self::permanent_definition(observation, source)
                .and_then(|definition| self.behavior(definition))
                != Some(CardBehavior::MishrasFactory)
        {
            8_800
        } else {
            -100
        }
    }

    fn score_land(&self, observation: &PlayerObservation, card: GameObjectId) -> i32 {
        let definition = Self::hand_definition(observation, card);
        let behavior = definition.and_then(|id| self.behavior(id));
        // The legend rule bins a duplicate on arrival. Replacing a tapped copy
        // with a fresh one is fine; duplicating an untapped one wastes both
        // the card and the land drop.
        if definition
            .and_then(|id| self.catalog.get(id))
            .is_some_and(|card| card.rules.has_supertype(CardSupertype::Legendary))
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && Some(permanent.definition) == definition
                    && !permanent.tapped
            })
        {
            return 40;
        }
        let basic_land_type_count =
            definition
                .and_then(|id| self.catalog.get(id))
                .map_or(0, |card| {
                    BasicLandType::ALL
                        .into_iter()
                        .filter(|land_type| card.rules.has_subtype(land_type.subtype()))
                        .count()
                });
        if basic_land_type_count >= 2 {
            return 9_400;
        }
        if basic_land_type_count == 1 {
            return 9_300;
        }
        match behavior {
            Some(CardBehavior::MishrasFactory) => 9_200,
            Some(_) | None => 9_000,
        }
    }

    fn score_untap(&self, observation: &PlayerObservation, permanents: &[GameObjectId]) -> i32 {
        8_000
            + permanents
                .iter()
                .filter_map(|id| {
                    observation
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.id == *id)
                })
                .map(|permanent| {
                    let card = self.card_value(permanent.definition);
                    let power = i32::from(permanent.power.unwrap_or(0).max(0));
                    card + power * 10
                })
                .sum::<i32>()
    }

    fn score_action(&self, observation: &PlayerObservation, action: &Action) -> i32 {
        match action {
            Action::KeepHand => 10_000,
            Action::TakeMulligan if self.should_mulligan(observation) => 11_000,
            Action::TakeMulligan => -5_000,
            Action::BottomCards { cards } | Action::DiscardCards { cards } => {
                9_000
                    - cards
                        .iter()
                        .filter_map(|card| Self::hand_definition(observation, *card))
                        .map(|definition| self.card_value(definition))
                        .sum::<i32>()
            }
            Action::ChooseDecision { options, .. } => {
                let selected_value = observation.decision.as_ref().map_or(0, |decision| {
                    decision
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .filter_map(|option| option.card)
                        .map(|(_, definition)| self.card_value(definition))
                        .sum::<i32>()
                });
                match observation
                    .decision
                    .as_ref()
                    .map(|decision| decision.preference)
                {
                    Some(crate::DecisionPreference::HigherCardValue) => 8_000 + selected_value,
                    Some(crate::DecisionPreference::LowerCardValue) => 8_000 - selected_value,
                    Some(crate::DecisionPreference::PreferOption(preferred)) => {
                        8_000 + i32::from(options.contains(&preferred))
                    }
                    Some(
                        crate::DecisionPreference::LinkedExileTargets
                        | crate::DecisionPreference::Neutral,
                    )
                    | None => 8_000,
                }
            }
            Action::CancelDecision { .. } => -1_000,
            Action::ChooseUntap { permanents } => self.score_untap(observation, permanents),
            Action::PlayLand { card, .. } => self.score_land(observation, *card),
            Action::ActivateManaAbility { source, .. } => {
                self.mana_action_score(observation, *source)
            }
            Action::PayLifeForMana => 5,
            Action::CastSpell { card, choices, .. } => self.score_cast(observation, *card, choices),
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_object,
                x,
            } => self.score_ability(observation, *source, *ability, targets, *cost_object, *x),
            Action::DeclareAttacker { attacker } => self.score_attack(observation, *attacker),
            Action::DeclareBlocker { blocker, attacker } => {
                Self::score_block(observation, *blocker, *attacker)
            }
            Action::FinishDeclaringAttackers | Action::FinishDeclaringBlockers => 1_000,
            Action::AssignCombatDamage { assignments, .. } => {
                6_000 + Self::score_assignment(observation, assignments)
            }
            Action::PassPriority => 0,
            Action::Concede => i32::MIN,
        }
    }

    fn linked_exile_target_score(
        &self,
        observation: &PlayerObservation,
        decision: &DecisionObservation,
        option: &DecisionOption,
    ) -> i32 {
        let Some((object, definition)) = option.card else {
            return -10_000;
        };
        let value = self.card_value(definition).max(1);
        match option.zone {
            DecisionZone::Battlefield => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == object)
                .map_or(-value, |permanent| {
                    if permanent.controller == decision.player {
                        -value
                    } else {
                        value
                    }
                }),
            DecisionZone::Graveyard => {
                if observation.graveyards[decision.player.index()]
                    .iter()
                    .any(|(card, _)| *card == object)
                {
                    value
                } else {
                    -value
                }
            }
            DecisionZone::Hand
            | DecisionZone::Stack
            | DecisionZone::Library
            | DecisionZone::Exile
            | DecisionZone::Command
            | DecisionZone::DrawnThisStep
            | DecisionZone::None => -value,
        }
    }

    fn choose_decision(
        &self,
        observation: &PlayerObservation,
        decision: &DecisionObservation,
    ) -> Option<Action> {
        if decision.options.len() < decision.minimum {
            return None;
        }
        let mut options = decision.options.iter().collect::<Vec<_>>();
        options.sort_by_key(|option| {
            let value = option
                .card
                .map_or(0, |(_, definition)| self.card_value(definition));
            match decision.preference {
                DecisionPreference::HigherCardValue => -value,
                DecisionPreference::LowerCardValue => value,
                DecisionPreference::LinkedExileTargets => {
                    -self.linked_exile_target_score(observation, decision, option)
                }
                DecisionPreference::PreferOption(preferred) => i32::from(option.id != preferred),
                DecisionPreference::Neutral => 0,
            }
        });
        // How many to take, once they are in preference order. Taking the
        // minimum is right when a decision costs you something — discards and
        // sacrifices give up as little as the effect demands. `HigherCardValue`
        // marks the decisions that hand you cards, and there the minimum can be
        // zero: a search may always fail to find, and a bot that took the
        // minimum would tutor for nothing every time.
        let take = match decision.preference {
            DecisionPreference::HigherCardValue => decision.maximum.min(options.len()),
            DecisionPreference::LinkedExileTargets => options
                .iter()
                .filter(|option| self.linked_exile_target_score(observation, decision, option) > 0)
                .count()
                .max(decision.minimum)
                .min(decision.maximum)
                .min(options.len()),
            DecisionPreference::LowerCardValue
            | DecisionPreference::PreferOption(_)
            | DecisionPreference::Neutral => decision.minimum,
        };
        Some(Action::ChooseDecision {
            decision: decision.id,
            options: options
                .into_iter()
                .take(take)
                .map(|option| option.id)
                .collect(),
        })
    }
}

impl Policy for HandcraftedPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            return self.choose_decision(observation, decision);
        }
        let action = observation
            .legal_actions
            .iter()
            .max_by_key(|action| self.score_action(observation, action))
            .cloned();
        if matches!(action, Some(Action::TakeMulligan)) {
            self.mulligans_taken += 1;
        } else if matches!(action, Some(Action::KeepHand)) {
            self.mulligans_taken = 0;
        }
        action
    }
}

#[cfg(test)]
mod tests {
    use super::HandcraftedPolicy;
    use crate::TargetIndex;
    use crate::card::{
        EffectDef, EffectRecipientDef, ManaCost, ObjectPredicateDef, PlayerRelation,
        TargetConditionDef, TurnStepDef, ValueDef,
    };

    static TARGET_CONDITION: TargetConditionDef = TargetConditionDef {
        slot: TargetIndex::PRIMARY,
        object: ObjectPredicateDef::Any,
        then: ValueDef::Constant(1),
        otherwise: ValueDef::Constant(0),
    };
    static CONDITIONAL_EFFECT: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::IfTargetMatches(&TARGET_CONDITION),
    };

    #[test]
    fn target_condition_search_descends_decision_effects() {
        let may = EffectDef::May(&CONDITIONAL_EFFECT);
        let optional_payment = EffectDef::OptionalManaPayment {
            cost: ManaCost::new(1, 0),
            effect: &CONDITIONAL_EFFECT,
        };
        let delayed = EffectDef::AtNextStep {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
            effect: &CONDITIONAL_EFFECT,
        };

        assert_eq!(
            HandcraftedPolicy::target_condition_in(may),
            Some(&TARGET_CONDITION),
        );
        assert_eq!(
            HandcraftedPolicy::target_condition_in(optional_payment),
            Some(&TARGET_CONDITION),
        );
        assert_eq!(
            HandcraftedPolicy::target_condition_in(delayed),
            Some(&TARGET_CONDITION),
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlayError {
    PolicyReturnedNoAction(PlayerId),
    IllegalAction(Box<ActionError>),
    ActionLimitExceeded(usize),
}

impl fmt::Display for PlayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyReturnedNoAction(player) => {
                write!(formatter, "policy for {player} returned no action")
            }
            Self::IllegalAction(error) => {
                write!(formatter, "policy returned an illegal action: {error}")
            }
            Self::ActionLimitExceeded(limit) => {
                write!(formatter, "game exceeded its action limit of {limit}")
            }
        }
    }
}

impl Error for PlayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IllegalAction(error) => Some(error.as_ref()),
            Self::PolicyReturnedNoAction(_) | Self::ActionLimitExceeded(_) => None,
        }
    }
}

/// Plays a game to completion using one policy for each player.
///
/// # Errors
///
/// Returns [`PlayError`] if a policy fails to choose an action, chooses an
/// illegal action, or the game exceeds `action_limit`.
pub fn play_game(
    game: &mut Game,
    player_one: &mut dyn Policy,
    player_two: &mut dyn Policy,
    action_limit: usize,
) -> Result<GameResult, PlayError> {
    for _ in 0..action_limit {
        if let Some(result) = game.result() {
            return Ok(result);
        }
        let Some(player) = game.decision_player() else {
            return game
                .result()
                .ok_or(PlayError::ActionLimitExceeded(action_limit));
        };
        let observation = game.observe(player);
        let action = match player {
            PlayerId::One => player_one.choose_action(&observation),
            PlayerId::Two => player_two.choose_action(&observation),
        }
        .ok_or(PlayError::PolicyReturnedNoAction(player))?;
        game.apply(player, action)
            .map_err(|error| PlayError::IllegalAction(Box::new(error)))?;
    }
    game.result()
        .ok_or(PlayError::ActionLimitExceeded(action_limit))
}
