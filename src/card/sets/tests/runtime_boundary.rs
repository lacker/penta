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
    assert!(shared_activated_costs(
        &[ZoneKind::Battlefield],
        &[AbilityCostDef::ExileSource],
    ));
    assert!(!shared_activated_costs(
        &[ZoneKind::Hand],
        &[AbilityCostDef::ExileSource],
    ));
    assert!(!shared_activated_costs(
        &[ZoneKind::Battlefield],
        &[AbilityCostDef::SacrificeSource, AbilityCostDef::ExileSource,],
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

fn assert_stack_effect_support(effects: &[EffectDef], expected: bool) {
    for effect in effects {
        assert_eq!(shared_stack_effect(*effect), expected, "{effect:?}");
    }
}

fn assert_unsupported_optional_payments(tap: &'static EffectDef) {
    static ONE_MANA: [CostDef; 1] = [CostDef::Mana(ManaCost::new(1, 0))];
    static ONE_LIFE: [CostDef; 1] = [CostDef::PayLife(1)];
    static TWO_MANA_PAYMENTS: [CostDef; 2] = [
        CostDef::Mana(ManaCost::new(1, 0)),
        CostDef::Mana(ManaCost::new(1, 0)),
    ];

    let any_payer = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::Any, &ONE_MANA),
        if_paid: tap,
    };
    let chosen_payer = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::ChosenPlayer, &ONE_MANA),
        if_paid: tap,
    };
    let event_payer = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::EventPlayer, &ONE_MANA),
        if_paid: tap,
    };
    let life_payment = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &ONE_LIFE),
        if_paid: tap,
    };
    let multiple_mana_payments = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &TWO_MANA_PAYMENTS),
        if_paid: tap,
    };

    assert_stack_effect_support(
        &[
            any_payer,
            chosen_payer,
            event_payer,
            life_payment,
            multiple_mana_payments,
        ],
        false,
    );
}

#[test]
fn decision_effects_suspend_inside_shared_stack_sequences() {
    static TAP: EffectDef = EffectDef::Tap {
        object: EffectRecipientDef::Source,
    };
    static UNTAP: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Source,
    };
    static PLAIN_SEQUENCE_COMPONENTS: [EffectDef; 2] = [TAP, UNTAP];
    static PLAIN_SEQUENCE: EffectDef = EffectDef::Sequence(&PLAIN_SEQUENCE_COMPONENTS);
    static MAY_TAP: EffectDef = EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &TAP,
    };
    static OPTIONAL_TAP: EffectDef = EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(ManaCost::new(1, 0))]),
        if_paid: &TAP,
    };
    static SOURCE_PRESENT: TriggerConditionDef = TriggerConditionDef::SourceOnBattlefield;
    static CONDITIONAL_MAY: EffectDef = EffectDef::IfCondition {
        condition: &SOURCE_PRESENT,
        then: &MAY_TAP,
    };
    static DELAYED_MAY: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::You,
        effect: &MAY_TAP,
    };
    static SEQUENCE_WITH_MAY: [EffectDef; 2] = [MAY_TAP, UNTAP];
    static SEQUENCE_WITH_CONDITIONAL_MAY: [EffectDef; 2] = [CONDITIONAL_MAY, UNTAP];
    static SEQUENCE_WITH_PAYMENT: [EffectDef; 2] = [OPTIONAL_TAP, UNTAP];
    static SEQUENCE_WITH_DELAYED_MAY: [EffectDef; 2] = [DELAYED_MAY, UNTAP];
    static SEARCH: EffectDef = EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source: ZoneKind::Library,
        object: ObjectPredicateDef::Any,
        minimum: 1,
        maximum: 1,
        reveal: false,
        destination: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        shuffle: true,
    };
    static SEQUENCE_WITH_EARLY_SEARCH: [EffectDef; 2] = [SEARCH, UNTAP];
    static SEQUENCE_WITH_TERMINAL_SEARCH: [EffectDef; 2] = [TAP, SEARCH];

    assert_stack_effect_support(
        &[
            MAY_TAP,
            CONDITIONAL_MAY,
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &PLAIN_SEQUENCE,
            },
            OPTIONAL_TAP,
            EffectDef::Sequence(&SEQUENCE_WITH_MAY),
            EffectDef::Sequence(&SEQUENCE_WITH_CONDITIONAL_MAY),
            EffectDef::Sequence(&SEQUENCE_WITH_PAYMENT),
            EffectDef::Sequence(&SEQUENCE_WITH_DELAYED_MAY),
            EffectDef::Sequence(&SEQUENCE_WITH_EARLY_SEARCH),
            EffectDef::Sequence(&SEQUENCE_WITH_TERMINAL_SEARCH),
        ],
        true,
    );
    assert_unsupported_optional_payments(&TAP);
}

#[test]
fn zone_search_boundary_rejects_ambiguous_or_incoherent_shapes() {
    let search = |source, destination, maximum, shuffle| EffectDef::SearchZone {
        player: EffectRecipientDef::Controller,
        source,
        object: ObjectPredicateDef::Any,
        minimum: 0,
        maximum,
        reveal: false,
        destination,
        placement: ZonePlacement::Top,
        shuffle,
    };

    assert!(shared_stack_effect(search(
        ZoneKind::Library,
        ZoneKind::Hand,
        2,
        true,
    )));
    assert!(shared_stack_effect(search(
        ZoneKind::Library,
        ZoneKind::Battlefield,
        1,
        true,
    )));
    assert!(!shared_stack_effect(search(
        ZoneKind::Library,
        ZoneKind::Battlefield,
        2,
        true,
    )));
    assert!(!shared_stack_effect(search(
        ZoneKind::Library,
        ZoneKind::Library,
        2,
        true,
    )));
    assert!(!shared_stack_effect(search(
        ZoneKind::Graveyard,
        ZoneKind::Hand,
        1,
        true,
    )));
}

#[test]
fn static_conditions_require_only_source_battlefield_state() {
    static APPLIED: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::CannotBeEnchanted,
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    };
    static SOURCE_UNTAPPED: TriggerConditionDef = TriggerConditionDef::SourceUntapped;
    static TARGET_MATCHES: TriggerConditionDef = TriggerConditionDef::TargetMatches {
        slot: crate::TargetIndex::PRIMARY,
        object: ObjectPredicateDef::Any,
    };

    assert!(shared_static_effect(
        &[ZoneKind::Battlefield],
        EffectDef::IfCondition {
            condition: &SOURCE_UNTAPPED,
            then: &APPLIED,
        },
    ));
    assert!(!shared_static_effect(
        &[ZoneKind::Battlefield],
        EffectDef::IfCondition {
            condition: &TARGET_MATCHES,
            then: &APPLIED,
        },
    ));
}

#[test]
fn replacement_perform_stays_coupled_to_its_prospective_event() {
    static UNTAP_SOURCE: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Source,
    };
    static TAKE_EXTRA_TURN: EffectDef = EffectDef::TakeExtraTurn {
        player: EffectRecipientDef::Controller,
    };

    let untap = ReplacementEffectDef::Perform(&UNTAP_SOURCE);
    assert!(shared_begin_turn_replacement_effect(untap));
    assert!(!shared_battlefield_exit_replacement_effect(untap));

    let extra_turn = ReplacementEffectDef::Perform(&TAKE_EXTRA_TURN);
    assert!(!shared_begin_turn_replacement_effect(extra_turn));
    assert!(shared_battlefield_exit_replacement_effect(extra_turn));
}

#[test]
#[should_panic(expected = "nested shared declarative ability outside the shared runtime boundary")]
fn nested_definition_assertions_descend_replacement_programs() {
    static UNSUPPORTED: AbilityDef = AbilityDef::static_ability(
        "This nested ability is intentionally outside the boundary.",
        EffectDef::Special("unsupported nested effect"),
    );
    static GRANT: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::GrantAbility(&UNSUPPORTED),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    };
    static PROGRAM: [ReplacementEffectDef; 1] = [ReplacementEffectDef::Perform(&GRANT)];

    assert_nested_definition_abilities(
        "Replacement fixture",
        EffectDef::Replacement(ReplacementEffectDef::Sequence(&PROGRAM)),
    );
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
