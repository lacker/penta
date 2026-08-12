use super::runtime_support::*;
use super::*;
use crate::{CostDef, PaymentDef};

#[test]
fn activated_cost_boundary_is_specific_to_the_source_zone() {
    let mana = AbilityCostDef::Mana(ManaCost::colored(0, 0, 0, 0, 1, 1));
    assert!(shared_activated_costs(
        &[ZoneKind::Hand],
        &[mana, AbilityCostDef::DiscardSource],
    ));
    assert!(!shared_activated_costs(
        &[ZoneKind::Hand],
        &[AbilityCostDef::PayLife(1)],
    ));
    assert!(shared_activated_costs(
        &[ZoneKind::Battlefield],
        &[mana, AbilityCostDef::TapSource],
    ));
    assert!(!shared_activated_costs(
        &[ZoneKind::Battlefield],
        &[AbilityCostDef::DiscardSource],
    ));
}

#[test]
fn triggered_mana_conditions_stay_outside_the_shared_runtime_boundary() {
    static CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        },
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    };
    let ordinary = AbilityDef::triggered_mana(
        "Whenever this becomes tapped, add {C}.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    );
    let DeclarativeAbilityDef::TriggeredMana(definition) = ordinary.definition else {
        unreachable!("triggered_mana must construct a triggered mana definition");
    };
    let conditional = AbilityDef::defined(
        "Whenever this becomes tapped, if you control a permanent, add {C}.",
        DeclarativeAbilityDef::TriggeredMana(definition.with_condition(&CONDITION)),
        ordinary.effect.definition,
    );

    assert!(shared_definition_ability(&ordinary));
    assert!(!shared_definition_ability(&conditional));
}

#[test]
fn decision_effects_stay_at_the_stack_effect_root() {
    static TAP: EffectDef = EffectDef::Tap {
        object: EffectRecipientDef::Source,
    };
    static UNTAP: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Source,
    };
    static PLAIN_SEQUENCE_COMPONENTS: [EffectDef; 2] = [TAP, UNTAP];
    static PLAIN_SEQUENCE: EffectDef = EffectDef::Sequence(&PLAIN_SEQUENCE_COMPONENTS);
    static MAY_TAP: EffectDef = EffectDef::May(&TAP);
    static OPTIONAL_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(ManaCost::new(1, 0))]),
        if_paid: &TAP,
    };
    static ANY_PAYER_OPTIONAL_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::Any, &[CostDef::Mana(ManaCost::new(1, 0))]),
        if_paid: &TAP,
    };
    static CHOSEN_PAYER_OPTIONAL_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(
            PlayerRelation::ChosenPlayer,
            &[CostDef::Mana(ManaCost::new(1, 0))],
        ),
        if_paid: &TAP,
    };
    static EVENT_PAYER_OPTIONAL_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(
            PlayerRelation::EventPlayer,
            &[CostDef::Mana(ManaCost::new(1, 0))],
        ),
        if_paid: &TAP,
    };
    static LIFE_PAYMENT_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &[CostDef::PayLife(1)]),
        if_paid: &TAP,
    };
    static MULTIPLE_MANA_PAYMENTS_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(
            PlayerRelation::You,
            &[
                CostDef::Mana(ManaCost::new(1, 0)),
                CostDef::Mana(ManaCost::new(1, 0)),
            ],
        ),
        if_paid: &TAP,
    };
    static DELAYED_MAY: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::You,
        effect: &MAY_TAP,
    };
    static SEQUENCE_WITH_MAY: [EffectDef; 2] = [MAY_TAP, UNTAP];
    static SEQUENCE_WITH_PAYMENT: [EffectDef; 2] = [OPTIONAL_TAP, UNTAP];
    static SEQUENCE_WITH_DELAYED_MAY: [EffectDef; 2] = [DELAYED_MAY, UNTAP];

    assert!(shared_stack_effect(MAY_TAP));
    assert!(shared_stack_effect(EffectDef::May(&PLAIN_SEQUENCE)));
    assert!(shared_stack_effect(OPTIONAL_TAP));
    assert!(!shared_stack_effect(ANY_PAYER_OPTIONAL_TAP));
    assert!(!shared_stack_effect(CHOSEN_PAYER_OPTIONAL_TAP));
    assert!(!shared_stack_effect(EVENT_PAYER_OPTIONAL_TAP));
    assert!(!shared_stack_effect(LIFE_PAYMENT_TAP));
    assert!(!shared_stack_effect(MULTIPLE_MANA_PAYMENTS_TAP));
    assert!(!shared_stack_effect(EffectDef::Sequence(
        &SEQUENCE_WITH_MAY,
    )));
    assert!(!shared_stack_effect(EffectDef::Sequence(
        &SEQUENCE_WITH_PAYMENT,
    )));
    assert!(shared_stack_effect(EffectDef::Sequence(
        &SEQUENCE_WITH_DELAYED_MAY,
    )));
}

#[test]
fn composite_uncounterability_stays_within_the_shared_runtime_boundary() {
    static CANNOT_BE_COUNTERED: [AppliedEffectDef; 1] = [AppliedEffectDef::CannotBeCountered];
    static MIXED: [AppliedEffectDef; 2] = [
        AppliedEffectDef::CannotBeCountered,
        AppliedEffectDef::Special("unsupported"),
    ];
    static RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::Composite(&CANNOT_BE_COUNTERED),
    )];
    static MIXED_RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
        AppliedEffectDef::Composite(&MIXED),
    )];

    let stack_effect = |effect| EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect,
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    };
    assert!(shared_static_effect(
        &[ZoneKind::Stack],
        stack_effect(AppliedEffectDef::Composite(&CANNOT_BE_COUNTERED)),
    ));
    assert!(!shared_static_effect(
        &[ZoneKind::Stack],
        stack_effect(AppliedEffectDef::Composite(&MIXED)),
    ));
    assert!(!shared_static_effect(
        &[ZoneKind::Stack],
        stack_effect(AppliedEffectDef::Composite(&[])),
    ));

    assert!(shared_mana_effect(
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_spend_effects(&RIDERS),),
        false,
    ));
    assert!(!shared_mana_effect(
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless).with_spend_effects(&MIXED_RIDERS),
        ),
        false,
    ));
}

/// A clause that declares card-owned execution has to have a binding, and
/// a binding has to be declared on its clause. Either half alone lets the
/// two drift: an undeclared binding makes the clause read as a no-op, and
/// an unbacked declaration is an ability that silently does nothing.
#[test]
fn card_owned_clauses_and_their_bindings_agree() {
    let mut declared = Vec::new();
    let mut bound = Vec::new();
    for record in SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
    {
        let definition = record.definition();
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                if attached.definition.effect.execution == EffectExecutionDef::CardOwned {
                    declared.push((definition.name.clone(), part.id, attached.id));
                }
            }
        }
        for binding in record.ability_bindings {
            bound.push((definition.name.clone(), binding.part, binding.ability));
            assert_eq!(
                binding.expected.effect.execution,
                EffectExecutionDef::CardOwned,
                "{} {:?} ability {:?} has a card-owned binding but its clause does not say so",
                definition.name,
                binding.part,
                binding.ability,
            );
        }
    }
    declared.sort();
    bound.sort();
    assert_eq!(
        declared, bound,
        "every card-owned clause needs a binding and every binding needs its clause to declare it"
    );
    assert!(
        !declared.is_empty(),
        "the scan found no card-owned clauses, so it is measuring the wrong thing"
    );
}

#[test]
fn fully_declarative_clauses_stay_within_the_shared_runtime_boundary() {
    for record in SET_MODULES
        .iter()
        .flat_map(|module| module.cards.iter().copied())
    {
        let definition = record.definition();
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let ability_id = attached.id;
                let ability = attached.definition;
                assert!(
                    !matches!(ability.definition, DeclarativeAbilityDef::Legacy)
                        || !ability.is_executable()
                        || ability.custom_behavior().is_some(),
                    "{} {:?} ability {:?} is legacy text claiming full implementation without an executable behavior: {ability:?}",
                    definition.name,
                    part.id,
                    ability_id,
                );
                if ability.declarative_effect().is_some() {
                    assert!(
                        shared_definition_ability(&ability),
                        "{} {:?} ability {:?} claims shared declarative execution outside the shared runtime boundary: {ability:?}",
                        definition.name,
                        part.id,
                        ability_id,
                    );
                }
                assert_nested_definition_abilities(&definition.name, ability.effect.definition);
                if let DeclarativeAbilityDef::Spell(spell) = ability.definition
                    && let Some(modal) = spell.modal()
                {
                    for mode in modal.modes {
                        if mode.declarative_effect().is_some() {
                            assert!(
                                shared_definition_ability(mode),
                                "{} {:?} ability {:?} contains a shared declarative modal branch outside the shared runtime boundary: {mode:?}",
                                definition.name,
                                part.id,
                                ability_id,
                            );
                        }
                        assert_nested_definition_abilities(
                            &definition.name,
                            mode.effect.definition,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn long_lived_composite_ability_changes_accept_shared_activated_grants() {
    static ACTIVATED: AbilityDef = AbilityDef::activated(
        "Draw a card.",
        &[],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    );
    static CHANGES: [AppliedEffectDef; 2] = [
        AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Any),
        AppliedEffectDef::GrantAbility(&ACTIVATED),
    ];
    let recipient = EffectRecipientDef::MatchingObjects {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::Any,
    };

    assert!(shared_resolving_apply(
        recipient,
        AppliedEffectDef::Composite(&CHANGES),
        EffectDurationDef::Permanent,
    ));
}
