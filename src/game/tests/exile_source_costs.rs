use super::*;

#[test]
fn exile_source_and_life_costs_are_paid_before_the_ability_goes_on_the_stack() {
    static COSTS: [AbilityCostDef; 3] = [
        AbilityCostDef::PayLife(2),
        AbilityCostDef::TapSource,
        AbilityCostDef::ExileSource,
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Pay 2 life, tap and exile this artifact: You gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_096);
    let mut definition = CardDefinition::new(
        definition_id,
        "Exile source cost test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(&ABILITIES);
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
    let source = creature(10_000, definition_id, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateAbility {
        source: source_id,
        ability: activated_ability_for(&game, source_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    let event_start = game.events.len();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[PlayerId::One.index()].life, 18);
    assert!(game.events[event_start..].contains(&GameEvent::LifeLost {
        player: PlayerId::One,
        amount: 2,
    }));
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id)
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == definition_id && card.id != source_id)
    );
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 19);
}

#[test]
fn a_mana_ability_can_exile_its_source_and_pay_life() {
    static COSTS: [AbilityCostDef; 2] = [AbilityCostDef::PayLife(1), AbilityCostDef::ExileSource];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "Pay 1 life, exile this artifact: Add {C}.",
        &COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    )];
    let definition_id = CardDefinitionId::new(10_097);
    let mut definition = CardDefinition::new(
        definition_id,
        "Exile source mana cost test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(&ABILITIES);
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
    let source = creature(10_000, definition_id, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateManaAbility {
        source: source_id,
        ability: mana_ability_for(&game, source_id, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    let event_start = game.events.len();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[PlayerId::One.index()].life, 19);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 1);
    assert!(game.events[event_start..].contains(&GameEvent::LifeLost {
        player: PlayerId::One,
        amount: 1,
    }));
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == definition_id)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id)
    );
}

#[test]
fn source_leaving_mana_cannot_also_pay_an_exile_source_ability() {
    static MAIN_COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::Mana(ManaCost::new(1, 0)),
        AbilityCostDef::ExileSource,
    ];
    static MANA_COSTS: [AbilityCostDef; 1] = [AbilityCostDef::SacrificeSource];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "{1}, Exile this artifact: You gain 1 life.",
            &MAIN_COSTS,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "Sacrifice this artifact: Add {C}.",
            &MANA_COSTS,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ];
    let definition_id = CardDefinitionId::new(10_098);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mutually exclusive source costs test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(&ABILITIES);
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
    let source_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source_id.0, definition_id, PlayerId::One));

    assert!(game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == source_id)
    ));
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
    ));
}

#[test]
fn source_preserving_mana_can_pay_an_exile_source_ability() {
    static MAIN_COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::Mana(ManaCost::new(1, 0)),
        AbilityCostDef::ExileSource,
    ];
    static MANA_COSTS: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "{1}, Exile this artifact: You gain 1 life.",
            &MAIN_COSTS,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &MANA_COSTS,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ];
    let definition_id = CardDefinitionId::new(10_099);
    let mut definition = CardDefinition::new(
        definition_id,
        "Compatible source costs test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(&ABILITIES);
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
    let source_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source_id.0, definition_id, PlayerId::One));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
        })
        .expect("tapping the source for mana leaves it available to exile");

    game.apply(PlayerId::One, action).unwrap();
    assert!(game.battlefield.is_empty());
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == definition_id)
    );
    assert_eq!(game.stack.len(), 1);
}
