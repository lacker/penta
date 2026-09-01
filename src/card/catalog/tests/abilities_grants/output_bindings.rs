#[test]
fn later_sequence_steps_may_read_explicitly_bound_effect_outputs() {
    let produced_cards = Box::leak(Box::new(ObjectSetDef::Binding(Binding!("produced_cards"))));
    let count_bound = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::CountObjects(produced_cards),
    };
    let mill_until = Box::leak(Box::new(crate::card::MillUntilDef {
        player: EffectRecipientDef::Controller,
        object: ObjectPredicateDef::HasType(CardType::Land),
        matched_zone: ZoneKind::Graveyard,
    }));
    let producers = [
        EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
        EffectDef::MillUntil(mill_until),
        EffectDef::SelectAtRandomFromZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Graveyard,
            object: ObjectPredicateDef::Any,
            amount: ValueDef::Constant(1),
        },
    ];

    for producer in producers {
        let producer = Box::leak(Box::new(producer));
        let bound = EffectDef::BindOutput {
            effect: producer,
            binding: Binding!("produced_cards"),
        };
        let valid = Box::leak(Box::new([bound, count_bound]));
        super::validate_ability_targets(&[], EffectDef::Sequence(valid))
            .expect("a later sequence step may read the producer's output binding");

        let reversed = Box::leak(Box::new([count_bound, bound]));
        assert_eq!(
            super::validate_ability_targets(&[], EffectDef::Sequence(reversed)),
            Err(GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                binding: Binding!("produced_cards"),
            }),
            "the binding is unavailable before its producer publishes it",
        );
    }

    let consume_reveal = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::CountObjects(Box::leak(Box::new(ObjectSetDef::Binding(
            Binding!("revealed_card"),
        )))),
    };
    let reveal = EffectDef::BindOutput {
        effect: &EffectDef::RevealAtRandomFromHand {
            player: EffectRecipientDef::Controller,
        },
        binding: Binding!("revealed_card"),
    };
    let valid = Box::leak(Box::new([reveal, consume_reveal]));
    super::validate_ability_targets(&[], EffectDef::Sequence(valid))
        .expect("a later sequence step may read the revealed-card binding");

    let reversed = Box::leak(Box::new([consume_reveal, reveal]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(reversed)),
        Err(
            GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                binding: Binding!("revealed_card"),
            }
        ),
    );
}

#[test]
fn effect_output_bindings_are_lexical_and_set_valued() {
    let mill = EffectDef::BindOutput {
        effect: &EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        binding: Binding!("cards"),
    };
    let duplicate = Box::leak(Box::new([mill, mill]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(duplicate)),
        Err(GrantedAbilityValidationError::BindingAlreadyDeclared {
            binding: Binding!("cards"),
        }),
    );

    let reads_own_binding = EffectDef::BindOutput {
        effect: Box::leak(Box::new(EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::CountObjects(Box::leak(Box::new(ObjectSetDef::Binding(
                Binding!("cards"),
            )))),
        })),
        binding: Binding!("cards"),
    };
    assert_eq!(
        super::validate_ability_targets(&[], reads_own_binding),
        Err(
            GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                binding: Binding!("cards"),
            }
        ),
    );

    let branch_local = EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceOnBattlefield,
        then: Box::leak(Box::new(mill)),
    };
    let count_cards = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::CountObjects(Box::leak(Box::new(
            ObjectSetDef::Binding(Binding!("cards")),
        ))),
    };
    let invalid_escape = Box::leak(Box::new([branch_local, count_cards]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(invalid_escape)),
        Err(GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
            binding: Binding!("cards"),
        }),
        "a binding declared only inside a branch does not escape that branch",
    );

    let conditional_mill = Box::leak(Box::new(EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceOnBattlefield,
        then: &EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    }));
    let explicit_optional_output = EffectDef::BindOutput {
        effect: conditional_mill,
        binding: Binding!("cards"),
    };
    let valid_escape = Box::leak(Box::new([explicit_optional_output, count_cards]));
    super::validate_ability_targets(&[], EffectDef::Sequence(valid_escape))
        .expect("wrapping the branch declares an output visible to later siblings");
}

#[test]
fn producer_continuations_require_and_expose_parent_binding() {
    let consume_parent = Box::leak(Box::new(EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::BoundObjectCount(ParentBinding),
    }));
    let valid = EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::new(
            ObjectPredicateDef::Any,
            &[ZoneKind::Battlefield],
        )),
        exclude: None,
        minimum: 0,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: consume_parent,
    });
    super::validate_ability_targets(&[], valid)
        .expect("a direct continuation may consume its producer through ParentBinding");

    let independent = EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
        then: &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        ..match valid {
            EffectDef::Choose(definition) => definition,
            _ => unreachable!(),
        }
    });
    assert!(matches!(
        super::validate_ability_targets(&[], independent),
        Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "then continuation does not consume its declared binding; use Sequence",
            ..
        })
    ));
}

#[test]
fn binding_labels_are_unique_across_sibling_branches() {
    let output = EffectDef::BindOutput {
        effect: &EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        binding: Binding!("branch_output"),
    };
    let branches = EffectDef::IfElseCondition {
        condition: &TriggerConditionDef::SourceOnBattlefield,
        then: Box::leak(Box::new(output)),
        otherwise: Box::leak(Box::new(output)),
    };
    assert_eq!(
        super::validate_ability_targets(&[], branches),
        Err(GrantedAbilityValidationError::BindingAlreadyDeclared {
            binding: Binding!("branch_output"),
        }),
    );
}
