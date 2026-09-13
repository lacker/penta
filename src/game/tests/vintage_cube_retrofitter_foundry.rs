//! Retrofitter Foundry: one mana on turn one, and a mana sink for the rest
//! of the game.

use super::*;

/// The Foundry on the battlefield since last turn, with `mana` colorless up.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let foundry = game
        .put_onto_battlefield(PlayerId::One, cards::RETROFITTER_FOUNDRY)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    drain_pending(&mut game);
    (game, foundry)
}

/// The Foundry's activations, by the ability each one names.
fn activations(game: &Game, foundry: GameObjectId) -> Vec<(u8, Action)> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match &action {
            Action::ActivateAbility {
                source,
                ability: crate::AbilityOrigin::Printed { ability, .. },
                ..
            } if *source == foundry => Some((ability.0, action)),
            _ => None,
        })
        .collect()
}

fn activate(game: &mut Game, foundry: GameObjectId, ability: u8) {
    let Some((_, action)) = activations(game, foundry)
        .into_iter()
        .find(|(id, _)| *id == ability)
    else {
        panic!("ability {ability} is offered");
    };
    game.apply(PlayerId::One, action).expect("it activates");
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn tokens(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect()
}

fn token_named<'a>(game: &'a Game, subtype: &str) -> Option<&'a Permanent> {
    tokens(game).into_iter().find(|permanent| {
        game.effective_subtypes(permanent)
            .contains(crate::card::Subtype::named(subtype))
    })
}

/// Two mana and a tap is a Servo.
#[test]
fn it_makes_a_servo() {
    let (mut game, foundry) = staged(2);

    activate(&mut game, foundry, 1);

    let servo = token_named(&game, "Servo").expect("a Servo arrived");
    assert_eq!(game.power(servo), Some(1));
    assert_eq!(game.toughness(servo), Some(1));
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == foundry)
            .expect("still there")
            .tapped,
        "and the Foundry is tapped",
    );
}

/// Untapping is what makes it more than one activation a turn.
#[test]
fn three_mana_untaps_it_for_another_activation() {
    let (mut game, foundry) = staged(7);

    activate(&mut game, foundry, 1);
    assert!(
        activations(&game, foundry)
            .iter()
            .all(|(ability, _)| *ability == 0),
        "tapped, only the untap is on offer",
    );

    activate(&mut game, foundry, 0);
    activate(&mut game, foundry, 1);

    assert_eq!(
        tokens(&game).len(),
        2,
        "two Servos out of seven mana and one card",
    );
}

/// A Servo becomes a flying Thopter, and a Thopter becomes a 4/4.
#[test]
fn the_servo_becomes_a_thopter_and_the_thopter_a_construct() {
    // Two mana for the Servo, three to untap, one for the Thopter, three to
    // untap again, and the Construct is free.
    let (mut game, foundry) = staged(9);

    activate(&mut game, foundry, 1);
    activate(&mut game, foundry, 0);
    activate(&mut game, foundry, 2);

    let thopter = token_named(&game, "Thopter").expect("a Thopter arrived");
    assert!(game.has_flying(thopter), "with flying");
    assert!(
        token_named(&game, "Servo").is_none(),
        "and the Servo is gone"
    );

    activate(&mut game, foundry, 0);
    activate(&mut game, foundry, 3);

    let construct = token_named(&game, "Construct").expect("a Construct arrived");
    assert_eq!(game.power(construct), Some(4));
    assert_eq!(game.toughness(construct), Some(4));
    assert!(
        token_named(&game, "Thopter").is_none(),
        "the Thopter paid for it",
    );
}

/// The sacrifice is part of the cost: without the creature the ability is
/// not offered, and a Thopter is not a Servo.
#[test]
fn the_upgrades_need_the_creature_they_eat() {
    let (mut game, foundry) = staged(6);

    assert!(
        !activations(&game, foundry)
            .iter()
            .any(|(ability, _)| *ability == 2 || *ability == 3),
        "no Servo and no Thopter, no upgrades",
    );

    activate(&mut game, foundry, 1);
    activate(&mut game, foundry, 0);

    assert!(
        activations(&game, foundry)
            .iter()
            .any(|(ability, _)| *ability == 2),
        "a Servo opens the Thopter ability",
    );
    assert!(
        !activations(&game, foundry)
            .iter()
            .any(|(ability, _)| *ability == 3),
        "and a Servo is not a Thopter",
    );
}

/// Nothing about it is sorcery speed: the whole line happens on their turn.
#[test]
fn it_works_on_their_turn_too() {
    let (mut game, foundry) = staged(2);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    activate(&mut game, foundry, 1);

    assert!(
        token_named(&game, "Servo").is_some(),
        "a Servo on their turn"
    );
}

/// "Sacrifice a Thopter" names a creature type, not the Foundry's own work:
/// a blue Thopter another artifact made is food for the free 4/4 just the
/// same.
#[test]
fn it_eats_somebody_elses_thopter() {
    let (mut game, foundry) = staged(1);
    let thopter_foundry = game
        .put_onto_battlefield(PlayerId::One, cards::THOPTER_FOUNDRY)
        .expect("cataloged");
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::HOWLING_MINE)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);

    let make_a_thopter = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                cost_objects,
                ..
            } => *source == thopter_foundry && cost_objects == &[mine],
            _ => false,
        })
        .expect("a mana and the Mine to melt down");
    game.apply(PlayerId::One, make_a_thopter)
        .expect("it activates");
    drain_pending(&mut game);

    let theirs = token_named(&game, "Thopter").expect("the other Foundry made one");
    let blue = ManaColor::Blue.color_index().expect("blue is a colour");
    assert!(
        game.permanent_colors(theirs)[blue],
        "a blue Thopter, which the Retrofitter never makes itself",
    );

    activate(&mut game, foundry, 3);

    let construct = token_named(&game, "Construct").expect("a Construct arrived");
    assert_eq!(game.power(construct), Some(4));
    assert!(
        token_named(&game, "Thopter").is_none(),
        "and the borrowed Thopter paid for it",
    );
}
