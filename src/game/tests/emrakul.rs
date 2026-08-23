//! Emrakul, the Aeons Torn and the shared annihilator trigger.

use super::*;

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

#[test]
fn casting_emrakul_queues_an_extra_turn_and_the_spell_cannot_be_countered() {
    let mut game = ready_game();
    let emrakul = card(21_000, cards::EMRAKUL_THE_AEONS_TORN, PlayerId::One);
    game.players[0].hand.push(emrakul.clone());
    game.players[0].mana_pool.colorless = 15;

    game.apply(
        PlayerId::One,
        cast_action(emrakul.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Emrakul is castable for fifteen mana");

    let spell = game
        .stack
        .iter()
        .find(|object| object.kind == StackObjectKind::Spell)
        .expect("Emrakul is on the stack beneath its cast trigger");
    assert_eq!(spell.card.definition, cards::EMRAKUL_THE_AEONS_TORN);
    assert!(!game.can_be_countered(spell));

    drain_pending(&mut game);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
    let emrakul = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::EMRAKUL_THE_AEONS_TORN)
        .expect("the spell resolved after its cast trigger");
    assert_eq!(game.power(emrakul), Some(15));
    assert_eq!(game.toughness(emrakul), Some(15));
    assert!(game.permanent_has_executable_keyword(emrakul, KeywordAbility::Flying));
}

#[test]
fn emrakuls_protection_matches_colored_spells_not_colored_permanent_sources() {
    let mut game = ready_game();
    let emrakul_id = game
        .put_onto_battlefield(PlayerId::One, cards::EMRAKUL_THE_AEONS_TORN)
        .expect("Emrakul is cataloged");
    let colored_spell = card(21_100, cards::LIGHTNING_BOLT, PlayerId::Two);
    let colored_spell_id = colored_spell.id;
    game.players[1].hand.push(colored_spell);
    let colorless_spell = card(21_101, cards::ORNITHOPTER, PlayerId::Two);
    let colorless_spell_id = colorless_spell.id;
    game.players[1].hand.push(colorless_spell);
    let colored_permanent = creature(21_102, cards::SAVANNAH_LIONS, PlayerId::Two);
    let colored_permanent_id = colored_permanent.card.id;
    game.battlefield.push(colored_permanent);

    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, emrakul_id),
        PlayerId::Two,
        colored_spell_id,
        true,
    ));
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, emrakul_id),
        PlayerId::Two,
        colorless_spell_id,
        true,
    ));
    assert!(
        game.permanent_can_be_targeted_by(
            permanent(&game, emrakul_id),
            PlayerId::Two,
            colored_permanent_id,
            false,
        ),
        "a colored permanent's activated ability is not a colored spell",
    );

    let colored_stack_spell = spell(21_103, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let colored_stack_spell_id = colored_stack_spell.id;
    game.stack.push(colored_stack_spell);
    game.damage_target_from(
        Some(colored_stack_spell_id),
        Some(Target::Permanent(emrakul_id)),
        3,
    );
    assert_eq!(permanent(&game, emrakul_id).damage, 0);

    let colorless_stack_spell = spell(21_104, cards::ORNITHOPTER, PlayerId::Two, 0);
    let colorless_stack_spell_id = colorless_stack_spell.id;
    game.stack.push(colorless_stack_spell);
    game.damage_target_from(
        Some(colorless_stack_spell_id),
        Some(Target::Permanent(emrakul_id)),
        1,
    );
    assert_eq!(permanent(&game, emrakul_id).damage, 1);
}

#[test]
fn annihilator_six_makes_the_defending_player_choose_six_permanents() {
    let mut game = ready_game();
    let definition = game
        .catalog
        .get(cards::EMRAKUL_THE_AEONS_TORN)
        .expect("Emrakul is cataloged");
    assert_eq!(definition.rules.ability_clauses()[4].text, "Annihilator 6");
    game.battlefield.clear();
    let emrakul_id = game
        .put_onto_battlefield(PlayerId::One, cards::EMRAKUL_THE_AEONS_TORN)
        .expect("Emrakul is cataloged");
    let planeswalker_id = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_MEMORY_ADEPT)
        .expect("Jace is cataloged");
    for index in 0..6 {
        game.battlefield
            .push(creature(21_201 + index, cards::MOUNTAIN, PlayerId::Two));
    }
    game.turn = 5;
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareAttackers;

    game.declare_attacker(emrakul_id, AttackDefender::Planeswalker(planeswalker_id));
    game.finish_declaring_attackers();
    let decision = advance_to_prompt(&mut game, PlayerId::Two, "Choose permanents to sacrifice");
    assert_eq!((decision.minimum, decision.maximum), (6, 6));
    assert_eq!(decision.options.len(), 7);
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(6)
                .map(|option| option.id)
                .collect(),
        },
    )
    .expect("six of the seven permanents may be chosen");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .count(),
        1,
    );
    assert_eq!(game.players[1].graveyard.len(), 6);
    assert_eq!(permanent(&game, emrakul_id).controller, PlayerId::One);
}

#[test]
fn emrakul_put_into_a_graveyard_shuffles_its_owners_whole_graveyard() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0]
        .graveyard
        .push(card(21_300, cards::SAVANNAH_LIONS, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(21_301, cards::GRIZZLY_BEARS, PlayerId::Two));
    let emrakul = card(21_302, cards::EMRAKUL_THE_AEONS_TORN, PlayerId::One);
    let emrakul_id = emrakul.id;
    game.players[0].hand.push(emrakul);

    game.discard_cards(PlayerId::One, &[emrakul_id]);
    drain_pending(&mut game);

    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].library.len(), 2);
    assert!(
        game.players[0]
            .library
            .iter()
            .any(|card| card.definition == cards::EMRAKUL_THE_AEONS_TORN),
    );
    assert_eq!(
        game.players[1].graveyard.len(),
        1,
        "only Emrakul's owner's graveyard is shuffled",
    );
}

#[test]
fn a_stolen_emrakul_still_shuffles_its_owners_graveyard() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0]
        .graveyard
        .push(card(21_400, cards::SAVANNAH_LIONS, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(21_401, cards::GRIZZLY_BEARS, PlayerId::Two));
    let mut emrakul = creature(21_402, cards::EMRAKUL_THE_AEONS_TORN, PlayerId::One);
    emrakul.controller = PlayerId::Two;
    let emrakul_id = emrakul.card.id;
    game.battlefield.push(emrakul);

    game.move_permanents_to_graveyard(&[emrakul_id]);
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "Emrakul's death trigger is captured"
    );
    drain_pending(&mut game);

    assert!(
        game.players[0].graveyard.is_empty(),
        "owner library {}, owner graveyard {}, controller library {}, controller graveyard {}",
        game.players[0].library.len(),
        game.players[0].graveyard.len(),
        game.players[1].library.len(),
        game.players[1].graveyard.len(),
    );
    assert_eq!(game.players[0].library.len(), 2);
    assert_eq!(game.players[1].graveyard.len(), 1);
}
