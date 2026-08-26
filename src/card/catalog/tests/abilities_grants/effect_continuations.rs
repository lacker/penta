// Recursive continuations share the same catalog obligations as their roots.

fn continuation_effects(child: &'static EffectDef) -> [EffectDef; 5] {
    [
        EffectDef::Destroy {
            object: EffectRecipientDef::Source,
            can_regenerate: true,
            then: Some(crate::card::DestroyFollowUpDef {
                binding: ObjectSetBindingIndex::PRIMARY,
                effect: child,
            }),
        },
        EffectDef::Discard {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
            selection: crate::card::DiscardSelectionDef::RecipientChooses,
            then: Some(crate::card::DiscardFollowUpDef {
                counted: ObjectPredicateDef::Any,
                bound: None,
                effect: child,
            }),
        },
        EffectDef::Mill {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
            binding: None,
            then: Some(child),
        },
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            then: None,
            amount: crate::card::SacrificedAmountDef::Power,
            otherwise: Some(child),
            optional: true,
        },
        EffectDef::PutOntoBattlefieldThen {
            object: EffectRecipientDef::Source,
            binding: ObjectSetBindingIndex::PRIMARY,
            counters: None,
            arrival_effect: None,
            then: child,
        },
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
    static CREATE: EffectDef = EffectDef::CreateToken {
        token: INCOHERENT_TOKEN,
        copy: None,
        controller: None,
        count: ValueDef::Constant(1),
        tapped: false,
        attacking: false,
        counters: None,
        created: None,
    };
    for effect in continuation_effects(&GRANT) {
        let child = Box::leak(Box::new(AbilityDef::activated(
            "Resolve a continuation that grants an ability.",
            &[],
            effect,
        )));
        assert_eq!(
            error(definition_granting(child)),
            CatalogError::InvalidGrantedAbility {
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                grant_path: vec![GrantId::PRIMARY, GrantId::PRIMARY],
                problem: GrantedAbilityValidationError::EmptyText,
            },
        );
    }

    for effect in continuation_effects(&CREATE) {
        let mut creator = definition(1, "Token Creator", CardSet::Alpha);
        let rules = creator.rules.with_ability(AbilityDef::activated(
            "Resolve a continuation that creates a token.",
            &[],
            effect,
        ));
        set_primary_rules(&mut creator, &rules);
        assert_eq!(
            error(creator),
            CatalogError::IncoherentCardRules {
                definition: CardDefinitionId::new(1),
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
                definition: CardDefinitionId::new(1),
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
                context: "resolving",
                operation: "CannotBeForcedToDiscard",
            },
        );
    }
}
