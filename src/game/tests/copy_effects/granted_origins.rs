// Where a granted ability's structural origin comes from, and what it
// freezes.
//
// Split from the copy tests next door for the source-size budget, and split
// here because these ask a different question: not what a copy effect copies,
// but which grant site an activation names and what that name still means
// once the granting permanent is gone. Included textually, so the imports
// here are that module's.

#[test]
fn granted_activation_freezes_payload_before_sacrificing_grant_source() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
        "Sacrifice an artifact: This creature deals 2 damage to any target.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            controller: PlayerRelation::You,
        }],
        &TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    );
    static GRANTED_TO: EffectRecipientDef = EffectRecipientDef::matching_objects(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    );
    static GRANTOR_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures you control have the test ability.",
        EffectDef::StaticApply {
            recipient: GRANTED_TO,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
    )];
    let grantor_definition_id = CardDefinitionId::new(10_062);
    let mut grantor_definition = CardDefinition::new(
        grantor_definition_id,
        "Activated snapshot test grantor",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    grantor_definition.rules =
        CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&GRANTOR_ABILITIES);
    synchronize_single_part_definition(&mut grantor_definition);

    let mut game = ready_game();
    let mut definitions: Vec<CardDefinition> =
        game.catalog.definitions().into_iter().cloned().collect();
    definitions.push(grantor_definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let grantor = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(grantor.0, grantor_definition_id, PlayerId::One),
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);
    let origin = AbilityOrigin::Granted {
        source: grantor,
        source_definition: grantor_definition_id,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    };
    let activation = Action::ActivateAbility {
        source: receiver,
        ability: origin,
        targets: activated_targets(Target::Player(PlayerId::Two)),
        cost_objects: vec![grantor],
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != grantor),
        "the continuous-effect source was sacrificed as the activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen stack payload");
    assert_eq!(payload.origin, origin);
    assert_eq!(payload.target_defs, &TARGETS);
    assert_eq!(
        payload.targets,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
    );
    assert!(matches!(
        payload.resolver,
        StackAbilityResolver::Declarative(ScopedEffect {
            effect: EffectDef::DealDamage { .. },
            ..
        })
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 18,
        "resolution must use the definition frozen before the grant disappeared",
    );
}

#[test]
fn separate_grant_sites_receive_distinct_structural_origins() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has flying.\nThis permanent has flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId::new(10_063);
    let mut definition = CardDefinition::new(
        definition_id,
        "Grant identity test card",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));

    let granted = game
        .effective_abilities(&game.battlefield[0])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. }
            | AbilityOrigin::Token { .. }
            | AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. }
            | AbilityOrigin::FaceDownGranted { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![
            AbilityOrigin::Granted {
                source,
                source_definition: definition_id,
                source_part: CardPartId::PRIMARY,
                source_ability: AbilityId::PRIMARY,
                grant: GrantId::PRIMARY,
            },
            AbilityOrigin::Granted {
                source,
                source_definition: definition_id,
                source_part: CardPartId::PRIMARY,
                source_ability: AbilityId::PRIMARY,
                grant: GrantId(1),
            },
        ]
    );
}

#[test]
fn a_nonmatching_grant_site_still_advances_the_structural_origin() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has flying. Creatures you control have flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId::new(10_080);
    let mut definition = CardDefinition::new(
        definition_id,
        "Nonmatching grant identity test card",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);

    let granted = game
        .effective_abilities(&game.battlefield[1])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. }
            | AbilityOrigin::Token { .. }
            | AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. }
            | AbilityOrigin::FaceDownGranted { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![AbilityOrigin::Granted {
            source,
            source_definition: definition_id,
            source_part: CardPartId::PRIMARY,
            source_ability: AbilityId::PRIMARY,
            grant: GrantId(1),
        }]
    );
}

#[test]
fn nonmatching_composite_grant_sites_still_advance_structural_origins() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static MISSED_COMPONENTS: [AppliedEffectDef; 1] =
        [AppliedEffectDef::add_ability(&GRANTED_ABILITY)];
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&MISSED_COMPONENTS),
        },
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "The attached permanent has flying.\nThis permanent has flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId::new(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Conditional composite grant identity test card",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_001);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));

    let granted = game
        .effective_abilities(&game.battlefield[0])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. }
            | AbilityOrigin::Token { .. }
            | AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. }
            | AbilityOrigin::FaceDownGranted { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![AbilityOrigin::Granted {
            source,
            source_definition: definition_id,
            source_part: CardPartId::PRIMARY,
            source_ability: AbilityId::PRIMARY,
            grant: GrantId(1),
        }]
    );
}

static COPY_GRANT_A: AbilityDef = AbilityDef::activated(
    "Gain 1 life.",
    &[],
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);
static COPY_GRANT_B: AbilityDef = AbilityDef::activated(
    "Lose 1 life.",
    &[],
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);
static COPY_GRANT_SOURCE_A_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Creatures you control have the first test ability.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&COPY_GRANT_A),
    },
)];
static COPY_GRANT_SOURCE_B_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Creatures you control have the second test ability.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&COPY_GRANT_B),
    },
)];

fn copy_grant_source_definition(
    id: CardDefinitionId,
    name: &'static str,
    abilities: &'static [AbilityDef],
) -> CardDefinition {
    let mut definition = CardDefinition::new(
        id,
        name,
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    definition
}

pub(super) fn copied_grant_source_game() -> (
    Game,
    CardInstanceId,
    CardInstanceId,
    CardDefinitionId,
    CardDefinitionId,
) {
    let definition_a = CardDefinitionId::new(10_064);
    let definition_b = CardDefinitionId::new(10_065);
    let source_a = copy_grant_source_definition(
        definition_a,
        "First grant source",
        &COPY_GRANT_SOURCE_A_ABILITIES,
    );
    let source_b = copy_grant_source_definition(
        definition_b,
        "Second grant source",
        &COPY_GRANT_SOURCE_B_ABILITIES,
    );
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([source_a, source_b]);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let grantor = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    let mut copied_source = creature(grantor.0, cards::COPY_ARTIFACT, PlayerId::One);
    copied_source.copy_effect = Some(copied_characteristics(definition_a));
    game.battlefield.extend([
        copied_source,
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);
    (game, grantor, receiver, definition_a, definition_b)
}

pub(super) fn sole_granted_origin(game: &Game, receiver: CardInstanceId) -> AbilityOrigin {
    let receiver = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == receiver)
        .expect("the granted-ability receiver is on the battlefield");
    game.effective_abilities(receiver)
        .into_iter()
        .find_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. }
            | AbilityOrigin::Token { .. }
            | AbilityOrigin::Emblem { .. }
            | AbilityOrigin::FaceDown { .. }
            | AbilityOrigin::TokenGranted { .. }
            | AbilityOrigin::EmblemGranted { .. }
            | AbilityOrigin::FaceDownGranted { .. }
            | AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_) => None,
        })
        .expect("the copied source grants an ability")
}

pub(super) const fn copied_grant_origin(
    grantor: CardInstanceId,
    definition: CardDefinitionId,
) -> AbilityOrigin {
    AbilityOrigin::Granted {
        source: grantor,
        source_definition: definition,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    }
}
