//! Grave Titan: ten power over three bodies for six mana, and killing it
//! still leaves four of it behind.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn zombies(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect()
}

/// Landing makes two of them.
#[test]
fn it_makes_two_zombies_as_it_enters() {
    let mut game = staged();

    let titan = game
        .put_onto_battlefield(PlayerId::One, cards::GRAVE_TITAN)
        .expect("cataloged");
    drain_pending(&mut game);

    let made = zombies(&game);
    assert_eq!(made.len(), 2, "two Zombies");
    assert_eq!(game.power(made[0]), Some(2), "each a 2/2");
    assert_eq!(game.toughness(made[0]), Some(2));
    assert!(
        made.iter().all(|zombie| game
            .effective_subtypes(zombie)
            .contains(crate::card::Subtype::named("Zombie"))),
        "and each a Zombie",
    );
    assert_eq!(
        made.iter()
            .filter(|zombie| zombie.controller == PlayerId::One)
            .count(),
        2,
        "under the Titan's controller",
    );
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == titan)
                .expect("the Titan is there"),
            KeywordAbility::Deathtouch,
        ),
        "and the Titan has deathtouch",
    );
}

/// Attacking makes two more: one printed ability with two ways in, and a
/// Titan that lands and then attacks makes four.
#[test]
fn attacking_makes_two_more() {
    let mut game = staged();
    let titan = game
        .put_onto_battlefield(PlayerId::One, cards::GRAVE_TITAN)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(zombies(&game).len(), 2);
    // Here since last turn, so it can attack.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == titan)
        .expect("the Titan is there")
        .entered_controller_turn = 0;

    game.step = Step::DeclareAttackers;
    game.declare_attacker(titan, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    assert_eq!(zombies(&game).len(), 4, "two more for the attack");
}

/// The Zombies are ordinary tokens: they are not the Titan, so killing the
/// Titan leaves them where they are.
#[test]
fn the_zombies_outlive_the_titan() {
    let mut game = staged();
    let titan = game
        .put_onto_battlefield(PlayerId::One, cards::GRAVE_TITAN)
        .expect("cataloged");
    drain_pending(&mut game);

    game.destroy_permanent(titan);
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert_eq!(zombies(&game).len(), 2, "the Zombies stay");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == titan),
        "and the Titan does not",
    );
}

/// "Two 2/2 black Zombie creature tokens": black is part of what they are,
/// and what the Titan is beyond that is not theirs to inherit.
#[test]
fn the_zombies_are_black_and_carry_nothing_of_the_titan() {
    let mut game = staged();
    game.put_onto_battlefield(PlayerId::One, cards::GRAVE_TITAN)
        .expect("cataloged");
    drain_pending(&mut game);

    let black = ManaColor::Black.color_index().expect("black is a colour");
    let green = ManaColor::Green.color_index().expect("green is a colour");
    for zombie in zombies(&game) {
        let colors = game.permanent_colors(zombie);
        assert!(colors[black], "a black Zombie");
        assert!(!colors[green], "and black alone");
        assert!(
            !game.permanent_has_executable_keyword(zombie, KeywordAbility::Deathtouch),
            "the Titan's deathtouch is the Titan's",
        );
        assert!(
            !game
                .effective_subtypes(zombie)
                .contains(crate::card::Subtype::named("Giant")),
            "and so is being a Giant",
        );
    }
}

/// The Titan's own deathtouch, which until now was only ever checked by its
/// absence on the Zombies. Six damage would leave a Colossus of Sardia
/// standing on toughness alone; any of it from a deathtoucher is lethal.
#[test]
fn its_deathtouch_kills_what_six_damage_would_not() {
    let mut game = staged();
    let titan = game
        .put_onto_battlefield(PlayerId::One, cards::GRAVE_TITAN)
        .expect("cataloged");
    let colossus = game
        .put_onto_battlefield(PlayerId::Two, cards::COLOSSUS_OF_SARDIA)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }

    let blocker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == colossus)
        .expect("it is there");
    assert!(
        game.toughness(blocker)
            .is_some_and(|toughness| toughness > 6),
        "it is bigger than the damage it is about to take",
    );
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == titan)
                .expect("it is there"),
            KeywordAbility::Deathtouch
        ),
        "and the Titan has what makes the size beside the point",
    );

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(titan, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);
    game.step = Step::DeclareBlockers;
    game.declare_blocker(colossus, titan);
    game.deal_combat_damage();
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == colossus),
        "a 9/9 that took six from a deathtoucher is a dead 9/9",
    );
    assert_eq!(
        game.players[1].life, 20,
        "and nothing got past it to the player",
    );
}
