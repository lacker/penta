use super::*;

fn staged(drawn: [CardDefinitionId; 2]) -> Game {
    let mut game = ready_game();
    game.players[0].hand = vec![card(241_000, cards::ILL_TIMED_EXPLOSION, PlayerId::One)];
    game.players[0].library.extend([
        card(241_001, drawn[0], PlayerId::One),
        card(241_002, drawn[1], PlayerId::One),
    ]);
    game.battlefield.extend([
        creature(241_010, cards::GRIZZLY_BEARS, PlayerId::One),
        creature(241_011, cards::SHIVAN_DRAGON, PlayerId::Two),
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game
}

fn cast_explosion(game: &mut Game) -> GameObjectId {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == GameObjectId(241_000)))
        .expect("Ill-Timed Explosion is castable");
    game.apply(PlayerId::One, action).unwrap();
    let source = game.stack[0].id;
    pass_priority_pair(game);
    source
}

fn restore(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 241_100)
        .expect("the discard choice and reflexive trigger reconstruct")
}

fn damage(game: &Game, definition: CardDefinitionId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("the creature remains on the battlefield")
        .damage
}

#[test]
fn ill_timed_explosion_draws_then_discards_and_leaves_a_response_window() {
    for prepared in [false, true] {
        let mut game = staged([cards::JUGGERNAUT, cards::LIGHTNING_BOLT]);
        game.set_prepared_engine_enabled(prepared);
        game.players[0]
            .hand
            .push(card(241_003, cards::ISLAND, PlayerId::One));
        let source = cast_explosion(&mut game);
        assert_eq!(
            game.players[0].hand.len(),
            3,
            "draw before offering the discard"
        );
        choose_decision_by_label(&mut game, PlayerId::One, "Do it");
        game = restore(&game);
        game.set_prepared_engine_enabled(prepared);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!((decision.minimum, decision.maximum), (2, 2));
        let selected = decision
            .options
            .iter()
            .filter_map(|option| {
                let (_, card) = option.card?;
                (card.card_definition() != Some(cards::ISLAND)).then_some(option.id)
            })
            .collect::<Vec<_>>();
        assert_eq!(selected.len(), 2, "the newly drawn cards can be discarded");
        assert!(
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![selected[0]],
                }
            )
            .is_err(),
            "discarding only one card is illegal"
        );
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: selected,
            },
        )
        .unwrap();
        assert_eq!(game.stack.len(), 1);
        assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
        assert_eq!(game.stack[0].source, Some(source));
        assert!(
            game.installed_triggers.is_empty(),
            "the listener fired exactly once"
        );
        assert_eq!(damage(&game, cards::GRIZZLY_BEARS), 0);
        assert_eq!(damage(&game, cards::SHIVAN_DRAGON), 0);
        assert_eq!(
            game.players[0].graveyard.len(),
            3,
            "the spell has finished resolving"
        );

        // Neither moving the spell nor moving the discarded cards erases the
        // trigger's source or its snapshot of the selected hand objects.
        let graveyard = game.players[0]
            .graveyard
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>();
        game.exile_graveyard_cards(PlayerId::One, &graveyard);
        game = restore(&game);
        game.set_prepared_engine_enabled(prepared);
        game.players[0]
            .hand
            .push(card(241_004, cards::GIANT_GROWTH, PlayerId::One));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        let bears = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
            .unwrap()
            .card
            .id;
        let growth = game.legal_actions(PlayerId::One).into_iter().find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == GameObjectId(241_004) && choices.iter_targets().next() == Some(&Target::Permanent(bears)))
        }).expect("the controller can respond to the damage trigger");
        game.apply(PlayerId::One, growth).unwrap();
        pass_priority_pair(&mut game);
        assert_eq!(damage(&game, cards::GRIZZLY_BEARS), 0);
        pass_priority_pair(&mut game);
        assert_eq!(
            damage(&game, cards::GRIZZLY_BEARS),
            4,
            "maximum mana value, not the sum"
        );
        assert_eq!(
            damage(&game, cards::SHIVAN_DRAGON),
            4,
            "both players' creatures take damage"
        );
        assert_eq!([game.players[0].life, game.players[1].life], [20, 20]);
        assert!(game.stack.is_empty());
    }
}

#[test]
fn ill_timed_explosion_decline_keeps_the_draws_and_installs_no_listener() {
    let mut game = staged([cards::JUGGERNAUT, cards::LIGHTNING_BOLT]);
    cast_explosion(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(game.stack.is_empty());
    assert!(game.installed_triggers.is_empty());
    let hand = game.players[0]
        .hand
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    game.discard_cards_with_cause(
        PlayerId::One,
        &hand,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
    );
    game.finish_rules_procedure();
    assert!(
        game.stack.is_empty(),
        "a later discard cannot trigger the declined spell"
    );
    assert_eq!(damage(&game, cards::GRIZZLY_BEARS), 0);
}

#[test]
fn ill_timed_explosion_cannot_discard_a_short_hand_after_restricted_draws() {
    for already_drawn in [false, true] {
        let mut game = staged([cards::JUGGERNAUT, cards::LIGHTNING_BOLT]);
        game.put_onto_battlefield(PlayerId::Two, cards::NARSET_PARTER_OF_VEILS)
            .unwrap();
        if already_drawn {
            game.draw_cards(PlayerId::One, 1);
            game.players[0]
                .hand
                .retain(|card| card.definition == cards::ILL_TIMED_EXPLOSION);
        }
        cast_explosion(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(!already_drawn));
        assert!(game.pending_decisions.is_empty());
        assert!(game.stack.is_empty());
        assert!(game.installed_triggers.is_empty());
        assert_eq!(damage(&game, cards::GRIZZLY_BEARS), 0);
    }
}

#[test]
fn ill_timed_explosion_replaced_discards_still_trigger_once() {
    let mut game = staged([cards::JUGGERNAUT, cards::LIGHTNING_BOLT]);
    game.put_onto_battlefield(PlayerId::Two, cards::REST_IN_PEACE)
        .unwrap();
    super::delayed_triggers::drain_pending(&mut game);
    cast_explosion(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    assert_eq!(game.players[0].exile.len(), 3);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.stack.len(), 1);
    game = restore(&game);
    pass_priority_pair(&mut game);
    assert_eq!(damage(&game, cards::SHIVAN_DRAGON), 4);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn ill_timed_explosion_uses_zero_for_x_and_for_lands() {
    for (drawn, expected) in [
        ([cards::FIREBALL, cards::ISLAND], 1),
        ([cards::MOUNTAIN, cards::ISLAND], 0),
    ] {
        let mut game = staged(drawn);
        cast_explosion(&mut game);
        choose_decision_by_label(&mut game, PlayerId::One, "Do it");
        assert_eq!(
            game.stack.len(),
            1,
            "even zero damage has a reflexive trigger"
        );
        pass_priority_pair(&mut game);
        assert_eq!(damage(&game, cards::GRIZZLY_BEARS), expected);
        assert_eq!(damage(&game, cards::SHIVAN_DRAGON), expected);
    }
}
