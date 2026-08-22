//! Joint life-budget and exact mana-source planning regressions.

use super::*;

fn plan_mana_sources(
    game: &Game,
    player: PlayerId,
    cost: ManaCost,
    x: u16,
    options: ManaPlanOptions,
    purpose: &ManaPaymentPurpose,
) -> Vec<GameObjectId> {
    crate::game::mana_planning::unique_payment_source_ids(
        game.plan_mana_activations_with_options_for(player, cost, x, options, purpose)
            .unwrap_or_default(),
    )
}

#[test]
fn free_mana_sources_are_used_before_channel() {
    let mut game = ready_game();
    resolve_channel(&mut game);
    let plains = game
        .put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .expect("Plains is cataloged");
    let cost = mana_cost!("{1}");

    assert_eq!(
        plan_mana_sources(
            &game,
            PlayerId::One,
            cost,
            0,
            ManaPlanOptions::default(),
            &ManaPaymentPurpose::Other,
        ),
        [plains],
    );
    game.activate_mana_for_cost(PlayerId::One, cost, 0);
    let _ = game.pay_player_cost(PlayerId::One, cost, 0);
    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == plains)
            .is_some_and(|permanent| permanent.tapped),
    );
}

#[test]
fn generic_allocation_avoids_a_source_before_choosing_its_color() {
    let mut game = ready_game();
    let plains = game
        .put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .expect("Plains is cataloged");
    let island = game
        .put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("Island is cataloged");

    assert_eq!(
        plan_mana_sources(
            &game,
            PlayerId::One,
            mana_cost!("{1}"),
            0,
            ManaPlanOptions {
                avoid: Some(plains),
                tap_cost_payer: None,
            },
            &ManaPaymentPurpose::Other,
        ),
        [island],
        "generic allocation chooses an unreserved color before its avoided source",
    );
}

#[test]
fn flexible_allocation_caps_each_color_by_the_affordable_sources() {
    static PAIN_COSTS: [AbilityCostDef; 2] =
        [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];
    static PAIN_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add {W}.",
        &PAIN_COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
    )];
    let pain_definition_id = CardDefinitionId::new(10_106);
    let mut pain_definition = CardDefinition::new(
        pain_definition_id,
        "White pain source test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    pain_definition.rules = CardRules::new_land(&[]).with_abilities(&PAIN_ABILITIES);
    synchronize_single_part_definition(&mut pain_definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(pain_definition);
    game.catalog = CardCatalog::new(definitions).expect("the fixture is valid");
    let forest = game
        .put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("Forest is cataloged");
    let plains = game
        .put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .expect("Plains is cataloged");
    let pain_source = game
        .put_onto_battlefield(PlayerId::One, pain_definition_id)
        .expect("the test source is cataloged");
    game.players[PlayerId::One.index()].life = 1;
    let purpose = ManaPaymentPurpose::Spell {
        object: GameObjectId(90_042),
        definition: cards::BARKSHELL_BLESSING,
        controller: PlayerId::One,
        form: SpellForm::Part(CardPartId::PRIMARY),
        reserved_life_payment: 1,
    };
    let cost = mana_cost!("{G/W}{G/W}");

    assert!(game.can_pay_cost_for(PlayerId::One, cost, 0, &purpose));
    assert_eq!(
        plan_mana_sources(
            &game,
            PlayerId::One,
            cost,
            0,
            ManaPlanOptions::default(),
            &purpose
        ),
        [plains, forest],
        "one free green and one free white pay instead of claiming white twice",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == pain_source)
            .is_some_and(|permanent| !permanent.tapped),
    );
}

#[test]
fn exact_mana_plan_preserves_cross_color_source_correlation() {
    static PAIN_COSTS: [AbilityCostDef; 2] =
        [AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)];
    static WHITE_ABILITY: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add {W}.",
        &PAIN_COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
    )];
    static BLUE_BLACK_ABILITY: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add {U}{B}.",
        &PAIN_COSTS,
        EffectDef::AddMana(AddManaEffectDef::one_of_each(
            ManaColor::Blue,
            ManaColor::Black,
        )),
    )];
    let white_id = CardDefinitionId::new(10_108);
    let blue_black_id = CardDefinitionId::new(10_109);
    let fixtures: [(CardDefinitionId, &str, &'static [AbilityDef]); 2] = [
        (white_id, "White pain source test", &WHITE_ABILITY),
        (
            blue_black_id,
            "Blue-black pain source test",
            &BLUE_BLACK_ABILITY,
        ),
    ];

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    for (id, name, abilities) in fixtures {
        let mut definition = CardDefinition::new(
            id,
            name,
            CardSet::Magic2014,
            false,
            CardBehavior::Unsupported,
        );
        definition.rules = CardRules::new_land(&[]).with_abilities(abilities);
        synchronize_single_part_definition(&mut definition);
        definitions.push(definition);
    }
    game.catalog = CardCatalog::new(definitions).expect("the fixtures are valid");
    let white = game
        .put_onto_battlefield(PlayerId::One, white_id)
        .expect("the white source is cataloged");
    let blue_black = game
        .put_onto_battlefield(PlayerId::One, blue_black_id)
        .expect("the blue-black source is cataloged");
    game.players[PlayerId::One.index()].life = 2;
    let purpose = ManaPaymentPurpose::Spell {
        object: GameObjectId(90_044),
        definition: cards::BARKSHELL_BLESSING,
        controller: PlayerId::One,
        form: SpellForm::Part(CardPartId::PRIMARY),
        reserved_life_payment: 1,
    };
    let cost = mana_cost!("{B}{W/U}");

    assert!(game.can_pay_cost_for(PlayerId::One, cost, 0, &purpose));
    assert_eq!(
        plan_mana_sources(
            &game,
            PlayerId::One,
            cost,
            0,
            ManaPlanOptions::default(),
            &purpose
        ),
        [blue_black],
        "one life funds the source whose single activation pays both symbols",
    );
    game.activate_mana_for_cost_avoiding_for(PlayerId::One, cost, 0, None, &purpose);
    let _ = game.pay_player_cost_for(PlayerId::One, cost, 0, &purpose);
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    for (source, tapped) in [(white, false), (blue_black, true)] {
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .map(|permanent| permanent.tapped),
            Some(tapped),
        );
    }
}
