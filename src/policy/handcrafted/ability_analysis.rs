use crate::card::AppliedRuleDef;

use super::{
    AbilityCostDef, AbilityOrigin, AppliedEffectDef, CardDefinitionId, CharacteristicOperationDef,
    DeclarativeAbilityDef, DeclarativeSpellProfile, EffectDef, EffectRecipientDef, GameObjectId,
    HandcraftedPolicy, ObjectCharacteristics, ObjectPredicateDef, PlayerObservation,
    PlayerRelation, PowerToughnessOperationDef, SetOperationDef, Step, Target, ValueDef,
};

impl HandcraftedPolicy {
    /// Every reason a greedy policy should decline an activated ability
    /// outright rather than pay for it. Each of these spends mana for a board
    /// that is no better, which is worse than passing.
    pub(super) fn ability_spends_mana_on_nothing(
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
        // A pump paid for with a permanent is a wash unless it is lethal: the
        // creature shrinks back at cleanup and the permanent is gone for good.
        if source_definition.is_some_and(|definition| {
            self.sacrifice_pump(definition, ability).is_some()
                && !self.sacrifice_pump_wins_now(observation, source, definition, ability)
        }) {
            return true;
        }
        // Animating a land turns a mana source into a creature that can be
        // killed, and the creature is worth nothing unless it can attack.
        // Animating one that is already a creature buys nothing at all.
        if source_definition
            .is_some_and(|definition| self.ability_animates_the_source(definition, ability))
            && (!Self::can_attack_this_combat(observation, source)
                || Self::is_already_a_creature(observation, source)
                || Self::already_on_the_stack(observation, source, ability))
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
    pub(super) fn ability_animates_the_source(
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
            .and_then(|ability| ability.declarative_effect())
            .is_some_and(Self::effect_animates_source)
    }

    pub(super) fn effect_animates_source(effect: EffectDef) -> bool {
        let EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect,
            ..
        } = effect
        else {
            return false;
        };
        Self::applied_effect_adds_creature_type(effect)
    }

    fn applied_effect_adds_creature_type(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_adds_creature_type),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                SetOperationDef::Add(types) | SetOperationDef::Set(types),
            )) => types.contains(crate::card::CardType::Creature),
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
        }
    }

    /// Whether a battlefield permanent satisfies a cost's predicate. Only the
    /// shapes a sacrifice cost actually uses are recognised; anything else is
    /// treated as no match, so the policy declines rather than guesses.
    pub(super) fn permanent_matches_predicate(
        &self,
        permanent: &crate::game::PermanentObservation,
        predicate: ObjectPredicateDef,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Any => true,
            ObjectPredicateDef::HasType(expected) => {
                if permanent.types.is_empty() {
                    match permanent.characteristics {
                        super::ObjectCharacteristics::Card { definition, part } => self
                            .catalog
                            .get(definition)
                            .and_then(|card| card.part(part))
                            .is_some_and(|part| part.rules.has_type(expected)),
                        super::ObjectCharacteristics::Token { token, part } => token
                            .part(part)
                            .is_some_and(|part| part.rules.has_type(expected)),
                        super::ObjectCharacteristics::Emblem { .. } => false,
                        super::ObjectCharacteristics::FaceDown { face_down } => {
                            face_down.rules().has_type(expected)
                        }
                    }
                } else {
                    permanent.types.contains(expected)
                }
            }
            _ => false,
        }
    }

    /// The power an ability adds to its own source, and what its cost eats to
    /// do it. Atog's shape: sacrifice a permanent, get bigger until end of
    /// turn.
    pub(super) fn sacrifice_pump(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> Option<(i16, ObjectPredicateDef)> {
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
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))?;
        let DeclarativeAbilityDef::Activated(activated) = ability.definition else {
            return None;
        };
        let eaten = activated
            .costs
            .as_slice()
            .iter()
            .find_map(|cost| match cost {
                AbilityCostDef::SacrificePermanent {
                    object,
                    controller: PlayerRelation::You,
                } => Some(*object),
                _ => None,
            })?;
        let Some(EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect:
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::Modify {
                        power: ValueDef::Constant(power),
                        ..
                    },
                )),
            duration: crate::card::ResolvedEffectDurationDef::UntilEndOfTurn,
        }) = ability.declarative_effect()
        else {
            return None;
        };
        i16::try_from(power).ok().map(|power| (power, eaten))
    }

    /// Whether eating everything the cost can reach makes this attacker
    /// lethal right now. Spending permanents on a pump is only worth it when
    /// it ends the game: the creature is smaller again next turn and the
    /// permanents are gone for good.
    pub(super) fn sacrifice_pump_wins_now(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> bool {
        let Some((power, eaten)) = self.sacrifice_pump(definition, origin) else {
            return false;
        };
        let Some(attacker) = observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == source)
        else {
            return false;
        };
        if !attacker.attacking
            || observation
                .battlefield
                .iter()
                .any(|permanent| permanent.blocking.contains(&source))
        {
            return false;
        }
        let food = observation
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == observation.viewer
                    && permanent.id != source
                    && self.permanent_matches_predicate(permanent, eaten)
            })
            .count();
        let potential = attacker.power.unwrap_or(0).saturating_add(
            i16::try_from(food)
                .unwrap_or(i16::MAX)
                .saturating_mul(power),
        );
        potential >= observation.life_totals[observation.viewer.opponent().index()]
    }

    /// Whether this permanent could still be declared as an attacker this
    /// turn. An untapped permanent in the viewer's own pre-attack combat
    /// steps is the window worth spending mana in.
    pub(super) fn can_attack_this_combat(
        observation: &PlayerObservation,
        source: GameObjectId,
    ) -> bool {
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
    pub(super) fn ability_is_a_wash(
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
            .and_then(|ability| ability.declarative_effect())
            .is_some_and(Self::effect_is_a_wash)
    }

    pub(super) fn effect_is_a_wash(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects.iter().copied().any(Self::effect_is_a_wash),
            EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            } => Self::effect_is_a_wash(*on_success) || Self::effect_is_a_wash(*on_failure),
            EffectDef::Choose(choice) => Self::effect_is_a_wash(*choice.then),
            EffectDef::PayOr(payment) => payment
                .if_paid
                .iter()
                .chain(payment.otherwise.iter())
                .any(|effect| Self::effect_is_a_wash(**effect)),
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Source,
                then,
                ..
            } => then.is_none_or(|then| matches!(*then, EffectDef::InstallTrigger(_))),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect:
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                        PowerToughnessOperationDef::Modify {
                            power: ValueDef::Constant(power),
                            toughness: ValueDef::Constant(toughness),
                        },
                    )),
                ..
            } => power + toughness == 0,
            _ => false,
        }
    }

    /// Whether the ability only buys evasion, which is worth nothing until
    /// the creature is actually attacking.
    pub(super) fn ability_only_buys_evasion(
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
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        ..
                    })
                )
            })
    }

    /// Whether every value in the ability's effect is conditional on the
    /// target, which is what makes a mismatched target worthless.
    pub(super) fn ability_needs_a_matching_target(
        &self,
        definition: CardDefinitionId,
        origin: AbilityOrigin,
    ) -> bool {
        self.activated_target_condition(definition, origin)
            .is_some()
    }

    /// The condition an ability's payoff hangs on, if it has exactly one.
    pub(super) fn activated_target_condition(
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
    pub(in crate::policy) fn target_condition_in(
        effect: EffectDef,
    ) -> Option<&'static crate::card::TargetConditionDef> {
        match effect {
            EffectDef::Sequence(effects) => {
                effects.iter().copied().find_map(Self::target_condition_in)
            }
            EffectDef::Randomized {
                on_success,
                on_failure,
                ..
            } => Self::target_condition_in(*on_success)
                .or_else(|| Self::target_condition_in(*on_failure)),
            EffectDef::Choose(choice) => Self::target_condition_in_object_set(choice.candidates)
                .or_else(|| Self::target_condition_in(*choice.then)),
            EffectDef::ChooseExact(choice) => {
                Self::target_condition_in_object_set(choice.candidates)
                    .or_else(|| Self::target_condition_in_value(choice.amount))
                    .or_else(|| Self::target_condition_in(*choice.then))
            }
            EffectDef::PayOr(payment) => {
                let payment_condition = match payment.payment.cost {
                    crate::card::EffectPaymentCostDef::GenericMana(amount) => {
                        Self::target_condition_in_value(amount)
                    }
                    crate::card::EffectPaymentCostDef::Mana(_)
                    | crate::card::EffectPaymentCostDef::Life(_)
                    | crate::card::EffectPaymentCostDef::Energy(_)
                    | crate::card::EffectPaymentCostDef::Mill(_)
                    | crate::card::EffectPaymentCostDef::SacrificeCreaturesWithTotalPower(_)
                    | crate::card::EffectPaymentCostDef::Discard(_)
                    | crate::card::EffectPaymentCostDef::DiscardMatching(_)
                    | crate::card::EffectPaymentCostDef::ChosenGenericMana
                    | crate::card::EffectPaymentCostDef::ChosenEnergy
                    | crate::card::EffectPaymentCostDef::RemoveAnyNumberOfCounters { .. }
                    | crate::card::EffectPaymentCostDef::MovePermanentMatching { .. }
                    | crate::card::EffectPaymentCostDef::SacrificePermanentMatching(_)
                    | crate::card::EffectPaymentCostDef::ObjectManaCostReducedBy { .. }
                    | crate::card::EffectPaymentCostDef::ColoredMana { .. } => None,
                };
                payment_condition.or_else(|| {
                    payment
                        .if_paid
                        .iter()
                        .chain(payment.otherwise.iter())
                        .find_map(|effect| Self::target_condition_in(**effect))
                })
            }
            EffectDef::May { effect, .. } => Self::target_condition_in(*effect),
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                Self::target_condition_in(*conditional.then).or_else(|| {
                    conditional
                        .otherwise
                        .and_then(|otherwise| Self::target_condition_in(*otherwise))
                })
            }
            EffectDef::InstallTrigger(trigger) => trigger
                .ability
                .declarative_effect()
                .and_then(Self::target_condition_in),
            EffectDef::PreventDamage { prevention, .. } => {
                let capacity = match prevention.capacity {
                    crate::card::DamagePreventionCapacityDef::Amount(amount) => {
                        Self::target_condition_in_value(amount)
                    }
                    crate::card::DamagePreventionCapacityDef::Events(_)
                    | crate::card::DamagePreventionCapacityDef::Unlimited => None,
                };
                capacity.or_else(|| Self::target_condition_in_value(prevention.amount))
            }
            EffectDef::AddCounters { amount, .. } | EffectDef::GainLife { amount, .. } => {
                Self::target_condition_in_value(amount)
            }
            _ => None,
        }
    }

    fn target_condition_in_object_set(
        objects: crate::card::ObjectSetDef,
    ) -> Option<&'static crate::card::TargetConditionDef> {
        match objects {
            crate::card::ObjectSetDef::Query(query) => {
                Self::target_condition_in_object_predicate(query.object)
            }
            crate::card::ObjectSetDef::PlayerAttachments(query) => {
                Self::target_condition_in_object_predicate(query.object)
            }
            crate::card::ObjectSetDef::ExceptObject { objects, .. } => {
                Self::target_condition_in_object_set(*objects)
            }
            crate::card::ObjectSetDef::One(_)
            | crate::card::ObjectSetDef::Binding(_)
            | crate::card::ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
            | crate::card::ObjectSetDef::MatchingBinding { .. }
            | crate::card::ObjectSetDef::Matching { .. }
            | crate::card::ObjectSetDef::PermanentsTargetedBy(_)
            | crate::card::ObjectSetDef::LegalAttachmentHosts(_)
            | crate::card::ObjectSetDef::LinkedExiles
            | crate::card::ObjectSetDef::CardsDrawnThisTurnInHand(_)
            | crate::card::ObjectSetDef::PermanentsControlledBy(_)
            | crate::card::ObjectSetDef::TokensCreatedBy(_)
            | crate::card::ObjectSetDef::BottomOfGraveyard(_)
            | crate::card::ObjectSetDef::LegalTargets(_)
            | crate::card::ObjectSetDef::TopOfGraveyardMatching { .. } => None,
        }
    }

    fn target_condition_in_object_predicate(
        object: ObjectPredicateDef,
    ) -> Option<&'static crate::card::TargetConditionDef> {
        match object {
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates
                    .iter()
                    .copied()
                    .find_map(Self::target_condition_in_object_predicate)
            }
            ObjectPredicateDef::Not(predicate) => {
                Self::target_condition_in_object_predicate(*predicate)
            }
            ObjectPredicateDef::ManaValueEqualTo(value)
            | ObjectPredicateDef::ManaValueAtMostValue(value)
            | ObjectPredicateDef::ToughnessLessThan(value) => {
                Self::target_condition_in_value(value)
            }
            _ => None,
        }
    }

    fn target_condition_in_value(
        value: ValueDef,
    ) -> Option<&'static crate::card::TargetConditionDef> {
        match value {
            ValueDef::IfTargetMatches(condition) => Some(condition),
            ValueDef::Negate(value) => Self::target_condition_in_value(*value),
            ValueDef::Scaled(value) => Self::target_condition_in_value(value.value),
            ValueDef::IfCreatureDiedThisTurn(condition) => {
                Self::target_condition_in_value(condition.then)
                    .or_else(|| Self::target_condition_in_value(condition.otherwise))
            }
            ValueDef::IfCondition(condition) => Self::target_condition_in_value(condition.then)
                .or_else(|| Self::target_condition_in_value(condition.otherwise)),
            ValueDef::IfMatchingObjectCount(condition) => {
                Self::target_condition_in_object_predicate(condition.query.object)
                    .or_else(|| Self::target_condition_in_value(condition.then))
                    .or_else(|| Self::target_condition_in_value(condition.otherwise))
            }
            ValueDef::CountMatchingObjects(query) | ValueDef::AnyMatchingObject(query) => {
                Self::target_condition_in_object_predicate(query.object)
            }
            _ => None,
        }
    }

    /// Whether the chosen target satisfies that condition.
    pub(super) fn ability_target_matches_condition(
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

    pub(super) fn declarative_activated_profile(
        &self,
        characteristics: ObjectCharacteristics,
        origin: AbilityOrigin,
    ) -> Option<DeclarativeSpellProfile> {
        let ability = match (characteristics, origin) {
            (
                ObjectCharacteristics::Card {
                    definition,
                    part: presented,
                },
                AbilityOrigin::Printed {
                    definition: origin_definition,
                    part,
                    ability,
                },
            ) if definition == origin_definition && presented == part => *self
                .catalog
                .get(definition)?
                .part(part)?
                .rules
                .ability(ability)?,
            (
                ObjectCharacteristics::Token {
                    token,
                    part: presented,
                },
                AbilityOrigin::Token { part, ability },
            ) if presented == part => *token.part(part)?.rules.ability(ability)?,
            _ => return None,
        };
        if !matches!(ability.definition, DeclarativeAbilityDef::Activated(_)) {
            return None;
        }
        let mut profile = DeclarativeSpellProfile::default();
        let targets = if let DeclarativeAbilityDef::Activated(definition) = ability.definition {
            profile.taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
            definition.targets
        } else {
            &[]
        };
        Self::collect_spell_effect_profile(ability.declarative_effect()?, 0, targets, &mut profile);
        Some(profile)
    }
}
