// Recursive continuations share the same catalog obligations as their roots.

use crate::ZonePlacement;

fn continuation_effects(child: &'static EffectDef) -> [EffectDef; 4] {
    let after = |effect| EffectDef::Sequence(Box::leak(Box::new([effect, *child])));
    [
        after(EffectDef::Destroy {
            object: EffectRecipientDef::Source,
            then: None,
        }),
        after(EffectDef::Discard {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
            selection: crate::card::DiscardSelectionDef::RecipientChooses,
            then: None,
        }),
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            amount: crate::card::SacrificedAmountDef::Power,
            then: None,
            otherwise: Some(child),
            optional: true,
        },
        after(EffectDef::move_to_zone(
            EffectRecipientDef::Source,
            ZoneKind::Battlefield,
            ZonePlacement::Top,
        )),
    ]
}

#[test]
fn catalog_validation_follows_nested_token_and_grant_continuations() {
    static INVALID: AbilityDef = AbilityDef::spell("", EffectDef::None);
    static GRANT: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&INVALID),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    };
    static INCOHERENT_TOKEN: TokenCharacteristics = TokenCharacteristics::new(
        crate::CardTypeSet::single(CardType::Land),
        &[],
        &[],
        Some(crate::CreatureStats {
            power: 1,
            toughness: 1,
        }),
    )
    .with_name("Broken Land");
    static CREATE: EffectDef = EffectDef::CreateToken(crate::card::CreateTokenDef::new(
        crate::card::TokenDef::Literal(INCOHERENT_TOKEN),
    ));
    static CHOOSE: AbilityDef = AbilityDef::replacement(
        "As this enters, choose a token declaration.",
        crate::card::ReplacementEffectDef::BindOutput {
            binding: crate::Binding!("branch_output"),
            effect: &crate::card::ReplacementEffectDef::Choose(crate::card::ReplacementChoiceDef::Scalar(
                crate::card::BattlefieldEntryScalarChoiceDef::tokens(&[crate::card::TokenChoiceDef {
                    label: "Broken", token: INCOHERENT_TOKEN,
                }]),
            )),
        },
    );
    let mut chooser = definition(1, "Token Chooser", sets::alpha::SET);
    let rules = chooser.rules.with_ability(CHOOSE);
    set_primary_rules(&mut chooser, &rules);
    assert_eq!(error(chooser), CatalogError::IncoherentCardRules {
        definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        part: CardPartId::PRIMARY,
        explanation: "a noncreature cannot have creature power and toughness",
    });

    for effect in continuation_effects(&GRANT) {
        let child = Box::leak(Box::new(AbilityDef::activated(
            "Resolve a continuation that grants an ability.",
            &[],
            effect,
        )));
        assert_eq!(
            error(definition_granting(child)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
                problem: Box::new(GrantedAbilityValidationError::EmptyText),
            },
        );
    }

    for effect in continuation_effects(&CREATE) {
        let mut creator = definition(1, "Token Creator", sets::alpha::SET);
        let rules = creator.rules.with_ability(AbilityDef::activated(
            "Resolve a continuation that creates a token.",
            &[],
            effect,
        ));
        set_primary_rules(&mut creator, &rules);
        assert_eq!(
            error(creator),
            CatalogError::IncoherentCardRules {
                definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
                part: CardPartId::PRIMARY,
                explanation: "a noncreature cannot have creature power and toughness",
            },
        );
    }
}

#[test]
fn target_validation_follows_every_recursive_continuation() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )];
    static OUT_OF_RANGE: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex(1)),
    };
    static WRONG_KIND: EffectDef = EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    };
    for effect in continuation_effects(&OUT_OF_RANGE) {
        assert_eq!(
            super::validate_ability_targets(&TARGETS, effect),
            Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
                target: TargetIndex(1),
                target_count: 1,
            }),
        );
    }

    for effect in continuation_effects(&WRONG_KIND) {
        assert_eq!(
            super::validate_ability_targets(&TARGETS, effect),
            Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
                target: TargetIndex::PRIMARY,
                predicate: AbilityTargetPredicate::Player(PlayerRelation::Any),
                expected: crate::EffectSubjectKind::Object,
            }),
        );
    }
}

#[test]
fn resolving_program_context_follows_every_recursive_continuation() {
    static FORBIDDEN: EffectDef = EffectDef::CannotBeForcedToDiscard;
    for effect in continuation_effects(&FORBIDDEN) {
        let ability = AbilityDef::activated(
            "Resolve a continuation with an unsupported static clause.",
            &[],
            effect,
        );
        assert_eq!(
            error(definition_with_ability(ability)),
            CatalogError::UnsupportedAbilityEffectProgramContext {
                definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                context: "resolving",
                operation: "CannotBeForcedToDiscard",
            },
        );
    }
}

#[test]
fn continuing_a_replaced_draw_is_rejected_outside_a_replacement_program() {
    let ability = AbilityDef::spell(
        "Continue a draw that was not replaced.",
        EffectDef::ContinueReplacedDraw,
    );
    assert_eq!(
        error(definition_with_ability(ability)),
        CatalogError::UnsupportedAbilityEffectProgramContext {
            definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
            context: "resolving",
            operation: "ContinueReplacedDraw",
        },
    );
}

#[test]
fn token_copy_entry_modifiers_are_rejected_instead_of_ignored() {
    use crate::card::{CreateTokenDef, TokenCopyDef, TokenCountersDef, TokenDef};
    static COPY: TokenCopyDef = TokenCopyDef {
        object: &EffectRecipientDef::Source,
        exceptions: crate::card::CopyExceptionsDef::NONE,
    };
    let create = CreateTokenDef::new(TokenDef::Copy(&COPY));
    super::validate_ability_targets(&[], EffectDef::CreateToken(create))
        .expect("ordinary token copies are supported");
    for unsupported in [
        create.entering_tapped(),
        create.entering_attacking(),
        create.with_counters(TokenCountersDef {
            kind: crate::card::CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        }),
    ] {
        assert_eq!(
            super::validate_ability_targets(&[], EffectDef::CreateToken(unsupported)),
            Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "resolving",
                    operation: "token copies with entry modifiers",
                }
            ),
        );
    }
}
