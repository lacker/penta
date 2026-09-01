//! Zone-aware mana abilities and direct spell-cost contributions.

use super::*;

fn cast_for(game: &Game, spell: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

fn hand_card(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let card = card(id, definition, PlayerId::One);
    let object = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    object
}

#[test]
fn elvish_spirit_guide_is_a_hand_zone_mana_ability() {
    let mut game = ready_game();
    let guide = hand_card(&mut game, 91_000, cards::ELVISH_SPIRIT_GUIDE);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source,
                    color: ManaColor::Green,
                    ..
                } if *source == guide
            )
        })
        .expect("Spirit Guide offers its mana ability from hand");

    game.apply(PlayerId::One, action)
        .expect("the offered hand mana ability applies");

    assert!(game.players[PlayerId::One.index()].hand.is_empty());
    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 1);
}

#[test]
fn elvish_spirit_guide_can_be_planned_during_a_cast() {
    let mut game = ready_game();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let guide = hand_card(&mut game, 91_010, cards::ELVISH_SPIRIT_GUIDE);
    let bears = hand_card(&mut game, 91_011, cards::GRIZZLY_BEARS);
    let cast = cast_for(&game, bears).expect("one generic and Spirit Guide pay for the Bears");
    assert!(
        game.mana_sources_for_action(PlayerId::One, &cast)
            .contains(&guide)
    );

    game.apply(PlayerId::One, cast)
        .expect("the advertised Spirit Guide payment applies");

    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 1);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn cadaverous_bloom_repeats_once_per_exiled_hand_card() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::CADAVEROUS_BLOOM)
        .expect("Cadaverous Bloom is cataloged");
    for (id, definition) in [
        (91_020, cards::PLAINS),
        (91_021, cards::ISLAND),
        (91_022, cards::MOUNTAIN),
    ] {
        hand_card(&mut game, id, definition);
    }
    let assembler = hand_card(&mut game, 91_023, cards::FOUNDRY_ASSEMBLER);
    let cast = cast_for(&game, assembler)
        .expect("three Bloom activations produce enough mana for the five-mana spell");

    game.apply(PlayerId::One, cast)
        .expect("the repeated Bloom payment applies");

    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 3);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.total(), 1);
}

#[test]
fn cadaverous_bloom_cannot_exile_the_spell_being_cast() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::CADAVEROUS_BLOOM)
        .expect("Cadaverous Bloom is cataloged");
    hand_card(&mut game, 91_024, cards::PLAINS);
    hand_card(&mut game, 91_025, cards::ISLAND);
    let assembler = hand_card(&mut game, 91_026, cards::FOUNDRY_ASSEMBLER);

    assert!(
        cast_for(&game, assembler).is_none(),
        "the spell stays in hand until it is moved to the stack and cannot pay Bloom's cost",
    );
}

#[test]
fn cadaverous_bloom_counts_every_hand_card_for_variable_costs() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::CADAVEROUS_BLOOM)
        .expect("Cadaverous Bloom is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("Mountain is cataloged");
    for (id, definition) in [
        (91_027, cards::PLAINS),
        (91_028, cards::ISLAND),
        (91_029, cards::FOREST),
    ] {
        hand_card(&mut game, id, definition);
    }
    let fireball = hand_card(&mut game, 91_030, cards::FIREBALL);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == fireball && choices.x() == 6
            )
        })
        .expect("three Bloom activations and a Mountain make Fireball X=6 legal");

    game.apply(PlayerId::One, cast)
        .expect("the variable-cost Bloom payment applies");
    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 3);
}

#[test]
fn treasure_cruise_delves_only_for_generic_mana() {
    let mut game = ready_game();
    let cruise = hand_card(&mut game, 91_031, cards::TREASURE_CRUISE);
    for offset in 0..8 {
        let grave = card(91_040 + offset, cards::PLAINS, PlayerId::One);
        game.players[PlayerId::One.index()].graveyard.push(grave);
    }
    assert!(
        cast_for(&game, cruise).is_none(),
        "eight graveyard cards cannot pay Treasure Cruise's blue symbol",
    );

    let island = game
        .put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("Island is cataloged");
    let cast = cast_for(&game, cruise).expect("Island plus seven delve cards pay Treasure Cruise");
    let sources = game.mana_sources_for_action(PlayerId::One, &cast);
    assert_eq!(sources.len(), 8, "one Island and seven cards are selected");
    assert!(sources.contains(&island));

    game.apply(PlayerId::One, cast)
        .expect("the advertised delve payment applies");

    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 7);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == island)
            .is_some_and(|permanent| permanent.tapped),
    );
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn foundry_assembler_improvises_with_five_artifacts() {
    let mut game = ready_game();
    let assembler = hand_card(&mut game, 91_060, cards::FOUNDRY_ASSEMBLER);
    let artifacts = (0..5)
        .map(|offset| {
            game.put_onto_battlefield(PlayerId::One, cards::ORNITHOPTER)
                .unwrap_or_else(|_| panic!("artifact {offset} enters"))
        })
        .collect::<Vec<_>>();
    let cast = cast_for(&game, assembler).expect("five artifacts improvise the whole generic cost");

    game.apply(PlayerId::One, cast)
        .expect("the advertised improvise payment applies");

    assert!(artifacts.iter().all(|artifact| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == *artifact)
            .is_some_and(|permanent| permanent.tapped)
    }));
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn an_artifact_cannot_tap_for_mana_and_improvise() {
    let mut game = ready_game();
    let assembler = hand_card(&mut game, 91_070, cards::FOUNDRY_ASSEMBLER);
    game.put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("Mox Sapphire is cataloged");
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::One, cards::ORNITHOPTER)
            .expect("Ornithopter is cataloged");
    }

    assert!(
        cast_for(&game, assembler).is_none(),
        "the Mox contributes one unit by tapping for mana or improvise, never both",
    );
}

#[test]
fn improvise_cannot_pay_a_colored_symbol() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::keyword(
            "Improvise (Your artifacts can help cast this spell. Each artifact you tap after you're done activating mana abilities pays for {1}.)",
            KeywordAbility::Improvise,
        ),
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];
    let definition_id = CardDefinitionId::new(50_126);
    let mut definition = CardDefinition::new(
        definition_id,
        "Colored improvise test",
        CardSet::AetherRevolt,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the improvise fixture is valid");
    let spell = hand_card(&mut game, 91_080, definition_id);
    game.put_onto_battlefield(PlayerId::One, cards::ORNITHOPTER)
        .expect("Ornithopter is cataloged");

    assert!(
        cast_for(&game, spell).is_none(),
        "an artifact's generic-only contribution cannot satisfy blue",
    );
}

#[test]
fn channel_checkpoint_preserves_the_ongoing_mana_source() {
    let mut game = ready_game();
    resolve_channel(&mut game);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 4_242)
            .expect("Channel's ongoing mana ability round-trips");

    assert!(rebuilt.legal_actions(PlayerId::One).iter().any(|action| {
        matches!(
            action,
            Action::ActivateManaAbility {
                color: ManaColor::Colorless,
                ..
            }
        )
    }));
}
