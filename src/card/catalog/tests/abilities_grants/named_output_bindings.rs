#[test]
fn later_sequence_steps_may_read_explicitly_bound_effect_outputs() {
    let label = "produced_cards";
    let produced_label = Box::leak(Box::new(crate::card::EffectBindingLabelDef(label)));
    let produced_cards = Box::leak(Box::new(ObjectSetDef::NamedBinding(produced_label)));
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
            binding: crate::card::EffectOutputBindingDef::Objects(label),
        };
        let valid = Box::leak(Box::new([bound, count_bound]));
        super::validate_ability_targets(&[], EffectDef::Sequence(valid))
            .expect("a later sequence step may read the producer's output binding");

        let reversed = Box::leak(Box::new([count_bound, bound]));
        assert_eq!(
            super::validate_ability_targets(&[], EffectDef::Sequence(reversed)),
            Err(GrantedAbilityValidationError::NamedObjectSetBindingReferenceOutOfScope {
                label,
            }),
            "the binding is unavailable before its producer publishes it",
        );
    }

    let revealed_label = "revealed_card";
    let revealed_binding_label =
        Box::leak(Box::new(crate::card::EffectBindingLabelDef(revealed_label)));
    let consume_reveal = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::CountObjects(Box::leak(Box::new(ObjectSetDef::NamedBinding(
            revealed_binding_label,
        )))),
    };
    let reveal = EffectDef::BindOutput {
        effect: &EffectDef::RevealAtRandomFromHand {
            player: EffectRecipientDef::Controller,
        },
        binding: crate::card::EffectOutputBindingDef::Objects(revealed_label),
    };
    let valid = Box::leak(Box::new([reveal, consume_reveal]));
    super::validate_ability_targets(&[], EffectDef::Sequence(valid))
        .expect("a later sequence step may read the revealed-card binding");

    let reversed = Box::leak(Box::new([consume_reveal, reveal]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(reversed)),
        Err(
            GrantedAbilityValidationError::NamedObjectSetBindingReferenceOutOfScope {
                label: revealed_label,
            }
        ),
    );
}

#[test]
fn named_effect_output_bindings_are_lexical_and_set_valued() {
    let label = "cards";
    let mill = EffectDef::BindOutput {
        effect: &EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        binding: crate::card::EffectOutputBindingDef::Objects(label),
    };
    let duplicate = Box::leak(Box::new([mill, mill]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(duplicate)),
        Err(GrantedAbilityValidationError::NamedBindingAlreadyInScope { label }),
    );

    let reads_own_binding = EffectDef::BindOutput {
        effect: Box::leak(Box::new(EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::CountObjects(&ObjectSetDef::NamedBinding(
                &crate::card::EffectBindingLabelDef("cards"),
            )),
        })),
        binding: crate::card::EffectOutputBindingDef::Objects("cards"),
    };
    assert_eq!(
        super::validate_ability_targets(&[], reads_own_binding),
        Err(
            GrantedAbilityValidationError::NamedObjectSetBindingReferenceOutOfScope {
                label: "cards",
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
            ObjectSetDef::NamedBinding(&crate::card::EffectBindingLabelDef("cards")),
        ))),
    };
    let invalid_escape = Box::leak(Box::new([branch_local, count_cards]));
    assert_eq!(
        super::validate_ability_targets(&[], EffectDef::Sequence(invalid_escape)),
        Err(GrantedAbilityValidationError::NamedObjectSetBindingReferenceOutOfScope {
            label,
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
        binding: crate::card::EffectOutputBindingDef::Objects(label),
    };
    let valid_escape = Box::leak(Box::new([explicit_optional_output, count_cards]));
    super::validate_ability_targets(&[], EffectDef::Sequence(valid_escape))
        .expect("wrapping the branch declares an output visible to later siblings");
}
