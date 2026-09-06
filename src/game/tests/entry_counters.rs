//! Permanents that arrive already grown. The counters come from a
//! battlefield-entry replacement rather than a trigger, so the size is
//! decided before the creature is ever on the battlefield -- and each of
//! these three reads its amount from somewhere different: a kicker that was
//! paid, an X that was announced, and a board that was counted.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn resolve(game: &mut Game) {
    for _ in 0..8 {
        drain_pending(game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// The power of the permanent `definition` became after resolving.
fn power_on_board(game: &Game, definition: CardDefinitionId) -> i16 {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(definition))
        .expect("the creature resolved onto the battlefield");
    game.power(permanent).expect("power")
}

/// Casts the Elite, kicked or not, and returns the resulting game.
fn cast_elite(kicked: bool) -> Game {
    let mut game = ready();
    let held = card(73_000, cards::LLANOWAR_ELITE, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    if kicked {
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 8);
    }
    let chosen = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == held_id && choices.costs().alternative().is_some() == kicked
            }
            _ => false,
        })
        .expect("a cast was offered");
    game.apply(PlayerId::One, chosen).expect("it is cast");
    resolve(&mut game);
    game
}

#[test]
fn the_elite_is_a_one_drop_unkicked() {
    let game = cast_elite(false);
    assert_eq!(
        power_on_board(&game, cards::LLANOWAR_ELITE),
        1,
        "one green buys the printed body and nothing else"
    );
}

#[test]
fn kicking_the_elite_brings_five_counters() {
    let game = cast_elite(true);
    assert_eq!(
        power_on_board(&game, cards::LLANOWAR_ELITE),
        6,
        "a 1/1 that arrived with five +1/+1 counters"
    );
}

#[test]
fn the_ivy_elemental_is_the_size_of_its_x() {
    let mut game = ready();
    let held = card(73_100, cards::IVY_ELEMENTAL, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    game.apply(
        PlayerId::One,
        cast_action(held_id, Vec::new(), Vec::new(), 3),
    )
    .expect("X of three is payable");
    resolve(&mut game);

    assert_eq!(
        power_on_board(&game, cards::IVY_ELEMENTAL),
        3,
        "the counters follow the X that was announced"
    );
}

/// The Beetle cast onto a board holding `others` creatures, half of them the
/// opponent's.
fn cast_beetle(others: usize) -> Game {
    let mut game = ready();
    for index in 0..others {
        let owner = if index % 2 == 0 {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        let mut bystander = creature(
            73_200 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            owner,
        );
        bystander.entered_controller_turn = 0;
        game.battlefield.push(bystander);
    }
    let held = card(73_300, cards::STAG_BEETLE, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held_id))
        .expect("the Beetle is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    game
}

#[test]
fn the_beetle_counts_every_other_creature() {
    let game = cast_beetle(0);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::STAG_BEETLE)),
        "an empty board leaves it a 0/0, which dies before anybody can look"
    );

    let game = cast_beetle(4);
    assert_eq!(
        power_on_board(&game, cards::STAG_BEETLE),
        4,
        "both sides count, and it does not count itself"
    );
}
