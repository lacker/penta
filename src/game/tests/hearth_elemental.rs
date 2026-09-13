use super::*;

fn cast_with(game: &Game, id: GameObjectId, option: PlayOptionId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == id && choices.play_option() == option)
        })
}

fn reduction(game: &Game, source: GameObjectId, part: CardPartId) -> u16 {
    game.spell_cost_reduction(
        cards::HEARTH_ELEMENTAL,
        &SpellForm::Part(part),
        PlayerId::One,
        source,
        &[],
    )
    .generic()
}

#[test]
fn cost_counts_only_qualifying_cards_in_the_casters_graveyard() {
    let mut game = ready_game();
    let hearth = card(91_000, cards::HEARTH_ELEMENTAL, PlayerId::One);
    let id = hearth.id;
    game.players[0].hand.push(hearth);
    assert_eq!(reduction(&game, id, CardPartId::PRIMARY), 0);
    game.players[0].graveyard.extend([
        card(91_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(91_002, cards::DIVINATION, PlayerId::One),
        card(91_003, cards::GRABBY_GIANT, PlayerId::One),
        card(91_004, cards::GRIZZLY_BEARS, PlayerId::One),
        card(91_005, cards::MOUNTAIN, PlayerId::One),
    ]);
    game.players[1]
        .graveyard
        .push(card(91_006, cards::LIGHTNING_BOLT, PlayerId::Two));
    game.players[0]
        .exile
        .push(card(91_007, cards::GRABBY_GIANT, PlayerId::One));
    game.players[0]
        .hand
        .push(card(91_008, cards::DIVINATION, PlayerId::One));
    assert_eq!(reduction(&game, id, CardPartId::PRIMARY), 3);
    assert_eq!(reduction(&game, id, CardPartId(1)), 0);

    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 1;
    assert!(cast_with(&game, id, PlayOptionId::DEFAULT).is_none());
    game.players[0].mana_pool.colorless = 2;
    let cast =
        cast_with(&game, id, PlayOptionId::DEFAULT).expect("three matching cards make it {2}{R}");
    game.apply(PlayerId::One, cast).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.stack_spell_mana_value(game.stack.last().unwrap()), 6);
}

#[test]
fn reduction_preserves_red_and_never_discounts_the_adventure() {
    let mut game = ready_game();
    let hearth = card(91_020, cards::HEARTH_ELEMENTAL, PlayerId::One);
    let id = hearth.id;
    game.players[0].hand.push(hearth);
    for index in 0..7 {
        game.players[0]
            .graveyard
            .push(card(91_021 + index, cards::LIGHTNING_BOLT, PlayerId::One));
    }
    game.players[0].mana_pool.colorless = 6;
    assert!(cast_with(&game, id, PlayOptionId::DEFAULT).is_none());
    game.players[0].mana_pool.colorless = 0;
    game.players[0].mana_pool.red = 1;
    assert!(cast_with(&game, id, PlayOptionId::DEFAULT).is_some());
    assert!(cast_with(&game, id, PlayOptionId(1)).is_none());
    game.players[0].mana_pool.colorless = 1;
    let cast = cast_with(&game, id, PlayOptionId(1)).expect("Stoke Genius still costs {1}{R}");
    game.apply(PlayerId::One, cast).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    let spell = game.stack.last().unwrap();
    assert_eq!(game.stack_spell_mana_value(spell), 2);
    assert!(
        !game
            .stack_trigger_event_object(spell)
            .unwrap()
            .has_adventure
    );
}

#[test]
fn stoke_genius_discards_at_resolution_then_draws_and_enables_the_discounted_creature() {
    for (hand_size, prepared) in [(0_u16, false), (3, false), (0, true), (3, true)] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let hearth = card(91_040, cards::HEARTH_ELEMENTAL, PlayerId::One);
        let id = hearth.id;
        game.players[0].hand.push(hearth);
        game.players[0].mana_pool.red = 1;
        game.players[0].mana_pool.colorless = 1;
        let cast = cast_with(&game, id, PlayOptionId(1)).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        // These cards arrive after the spell is cast: discard is an effect,
        // and must read the hand when Stoke Genius resolves.
        for index in 0..hand_size {
            game.players[0].hand.push(card(
                91_041 + u32::from(index),
                cards::LIGHTNING_BOLT,
                PlayerId::One,
            ));
        }
        let library_size = game.players[0].library.len();
        drain_pending(&mut game);
        assert!(game.stack.is_empty());
        assert_eq!(game.players[0].graveyard.len(), usize::from(hand_size));
        assert_eq!(game.players[0].hand.len(), 2);
        assert_eq!(game.players[0].library.len(), library_size - 2);
        let exiled = &game.players[0].exile[0];
        assert_eq!(exiled.definition, cards::HEARTH_ELEMENTAL);
        let exiled_id = exiled.id;
        game.players[0].mana_pool.red = 1;
        game.players[0].mana_pool.colorless = 5 - hand_size;
        assert!(cast_with(&game, exiled_id, PlayOptionId(1)).is_none());
        let creature = cast_with(&game, exiled_id, PlayOptionId::DEFAULT).unwrap();
        game.apply(PlayerId::One, creature).unwrap();
        assert_eq!(game.players[0].mana_pool.total(), 0);
        assert!(
            game.stack_trigger_event_object(game.stack.last().unwrap())
                .unwrap()
                .has_adventure
        );
        drain_pending(&mut game);
        let permanent = game.battlefield.last().unwrap();
        assert_eq!(permanent.card.definition, cards::HEARTH_ELEMENTAL);
        assert_eq!(game.power(permanent), Some(4));
        assert_eq!(game.toughness(permanent), Some(5));
    }
}

#[test]
fn has_adventure_uses_the_presented_part_and_preserves_copied_snapshots() {
    let mut game = ready_game();
    let original = creature(91_060, cards::HEARTH_ELEMENTAL, PlayerId::One);
    for context in [
        CharacteristicContext::Hand,
        CharacteristicContext::Library,
        CharacteristicContext::Graveyard,
        CharacteristicContext::Exile,
        CharacteristicContext::Stack {
            form: SpellForm::Part(CardPartId::PRIMARY),
        },
    ] {
        let view = game
            .printed_trigger_event_object(
                original.card.id,
                cards::HEARTH_ELEMENTAL,
                PlayerId::One,
                &context,
            )
            .unwrap();
        assert!(game.trigger_object_matches_for_controller(
            ObjectPredicateDef::HasAdventure,
            &view,
            original.card.id,
            false,
            Some(PlayerId::One)
        ));
        assert!(!view.subtypes.contains(&"Adventure"));
    }
    let omen = CardStructure::AlternateSpell {
        main: CardPartId::PRIMARY,
        alternate: CardPartId(1),
        kind: crate::card::AlternateSpellKind::Omen,
    };
    assert!(!omen.part_has_adventure(CardPartId::PRIMARY));
    let mut copier = creature(91_061, cards::GRIZZLY_BEARS, PlayerId::One);
    copier.copy_effect = Some(copied_characteristics(cards::HEARTH_ELEMENTAL));
    let mut token_copy = token_permanent(91_062, tokens::food(), PlayerId::One);
    token_copy.copy_effect = Some(copied_characteristics(cards::HEARTH_ELEMENTAL));
    game.battlefield.extend([original, copier, token_copy]);
    assert!(
        game.trigger_event_object(&game.battlefield[2])
            .has_adventure
    );
    let copied = game.trigger_event_object(&game.battlefield[1]);
    assert!(copied.has_adventure);
    game.battlefield[1].copy_effect = None;
    assert!(
        !game
            .trigger_event_object(&game.battlefield[1])
            .has_adventure
    );
    assert!(game.trigger_object_matches_for_controller(
        ObjectPredicateDef::HasAdventure,
        &copied,
        copied.id,
        false,
        Some(PlayerId::One)
    ));
    game.battlefield[0].face_down = Some(crate::card::face_down::ordinary());
    assert!(
        !game
            .trigger_event_object(&game.battlefield[0])
            .has_adventure
    );
    assert!(!Game::face_down_exiled_event_object(copied.id, PlayerId::One).has_adventure);
}
