use super::*;
use crate::card::BattlefieldEntryScalarChoiceDef;

#[test]
fn scalar_entry_choices_reject_mismatched_lists_and_destinations() {
    let choice = BattlefieldEntryScalarChoiceDef {
        list: ScalarChoiceListDef::CardNames(CardNameSetDef::AllCardNames),
        destination: BattlefieldEntryChoiceDestinationDef::CreatureType,
    };
    assert_eq!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)),
        ),
        Err(GrantedAbilityValidationError::InvalidScalarChoice {
            list: choice.list,
            destination: choice.destination,
        }),
    );
}

#[test]
fn card_name_entry_choices_require_public_name_sets_and_durable_bindings() {
    const PUBLIC_NAME_SETS: [CardNameSetDef; 2] = [
        CardNameSetDef::NonlandCardNames,
        CardNameSetDef::LandCardNames,
    ];
    const HIDDEN_OBJECTS: ObjectSetDef = ObjectSetDef::Query(ObjectQueryDef::new(
        ObjectPredicateDef::Any,
        &[ZoneKind::Hand],
    ));
    const PUBLIC: BattlefieldEntryScalarChoiceDef =
        BattlefieldEntryScalarChoiceDef::card_name(CardNameSetDef::Union(&PUBLIC_NAME_SETS));
    const PUBLIC_PRODUCER: ReplacementEffectDef =
        ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(PUBLIC));
    const HIDDEN: BattlefieldEntryScalarChoiceDef =
        BattlefieldEntryScalarChoiceDef::card_name(CardNameSetDef::NamesOf(&HIDDEN_OBJECTS));
    const HIDDEN_PRODUCER: ReplacementEffectDef =
        ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(HIDDEN));

    assert_eq!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::BindOutput {
                binding: Binding!("pithing_needle_name"),
                effect: &PUBLIC_PRODUCER,
            },
        ),
        Ok(()),
    );
    assert_eq!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::BindOutput {
                binding: Binding!("pithing_needle_name"),
                effect: &HIDDEN_PRODUCER,
            },
        ),
        Err(GrantedAbilityValidationError::InvalidScalarChoice {
            list: HIDDEN.list,
            destination: HIDDEN.destination,
        }),
    );
    assert_eq!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::BindOutput {
                binding: crate::ParentBinding,
                effect: &PUBLIC_PRODUCER,
            },
        ),
        Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "binding",
                operation: "BindOutput requires a durable labeled binding",
            }
        ),
    );
}

#[test]
fn creature_type_entry_bindings_validate_the_producer_and_mana_consumer() {
    use crate::card::{AddManaEffectDef, ManaRestrictionDef, SubtypeDef};
    const PRODUCER: ReplacementEffectDef = ReplacementEffectDef::Choose(
        ReplacementChoiceDef::Scalar(BattlefieldEntryScalarChoiceDef::CREATURE_TYPE),
    );
    assert_eq!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::BindOutput {
                binding: Binding!("cavern_creature_type"),
                effect: &PRODUCER,
            }
        ),
        Ok(())
    );
    assert!(
        validate_replacement_ability_targets(
            &[],
            ReplacementEffectDef::BindOutput {
                binding: crate::ParentBinding,
                effect: &PRODUCER,
            }
        )
        .is_err()
    );
    assert!(
        validate_ability_targets(
            &[],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::Subtype(SubtypeDef::Binding(
                    crate::ParentBinding
                ),)),
            ]),)
        )
        .is_err(),
        "mana restrictions must validate their subtype binding references"
    );
    assert!(
        validate_ability_targets(
            &[],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::AnyOf(&[ManaRestrictionDef::ActivateAbility(
                    ObjectPredicateDef::Subtype(SubtypeDef::Binding(crate::ParentBinding))
                ),])
            ]))
        )
        .is_err(),
        "alternative restrictions must validate nested bindings"
    );
}

#[test]
fn labeled_choices_require_unique_nonempty_options_and_a_durable_binding() {
    use crate::card::TokenChoiceDef;
    const GOOD: &[TokenChoiceDef] = &[
        TokenChoiceDef {
            label: "Sunrise",
            token: crate::card::TokenCharacteristics::creature(&["Citizen"], &[], 1, 1),
        },
        TokenChoiceDef {
            label: "Twilight",
            token: crate::card::TokenCharacteristics::creature(&["Citizen"], &[], 1, 1),
        },
    ];
    const DUPLICATE: &[TokenChoiceDef] = &[
        TokenChoiceDef {
            label: "same",
            token: crate::card::TokenCharacteristics::creature(&["Citizen"], &[], 1, 1),
        },
        TokenChoiceDef {
            label: "same",
            token: crate::card::TokenCharacteristics::creature(&["Citizen"], &[], 1, 1),
        },
    ];
    const BLANK: &[TokenChoiceDef] = &[TokenChoiceDef {
        label: " ",
        token: crate::card::TokenCharacteristics::creature(&["Citizen"], &[], 1, 1),
    }];
    for choices in [GOOD, DUPLICATE, BLANK, &[]] {
        let producer = Box::leak(Box::new(ReplacementEffectDef::Choose(
            ReplacementChoiceDef::Scalar(BattlefieldEntryScalarChoiceDef::tokens(choices)),
        )));
        let bound = ReplacementEffectDef::BindOutput {
            binding: Binding!("branch_output"),
            effect: producer,
        };
        assert_eq!(
            validate_replacement_ability_targets(&[], bound).is_ok(),
            choices == GOOD
        );
        assert!(
            validate_replacement_ability_targets(&[], *producer).is_err(),
            "label choices cannot discard their binding"
        );
        assert!(
            validate_replacement_ability_targets(
                &[],
                ReplacementEffectDef::BindOutput {
                    binding: crate::ParentBinding,
                    effect: producer,
                }
            )
            .is_err()
        );
    }
}

#[test]
fn labeled_choices_reject_nondurable_token_bindings() {
    assert!(
        validate_effect_target_shapes(
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(
                crate::card::TokenDef::Binding(crate::ParentBinding)
            )),
            &[],
            None,
        )
        .is_err()
    );
}
