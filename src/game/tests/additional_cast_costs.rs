//! "As an additional cost to cast this spell, ..." -- a cost paid on the way
//! to the stack rather than on resolution. What has to be right is that the
//! spell is not castable at all when the cost cannot be paid, and that the
//! payment really happens: a cost the runtime quietly skips looks exactly
//! like a cheaper card.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[PlayerId::One.index()] = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn offered(game: &Game, held: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .collect()
}

fn resolve(game: &mut Game) {
    for _ in 0..12 {
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

fn battlefield_count(game: &Game, definition: CardDefinitionId) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Card(definition))
        .count()
}

/// The Skulltap is uncastable with an empty board and costs exactly one
/// creature with a stocked one.
#[test]
fn skulltap_is_uncastable_until_there_is_a_creature_to_spend() {
    let mut game = ready();
    let held = card(78_000, cards::SKULLTAP, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
    assert!(
        offered(&game, held_id).is_empty(),
        "two black mana is not enough on its own"
    );

    game.battlefield
        .push(creature(78_001, cards::GRIZZLY_BEARS, PlayerId::One));
    let cast = offered(&game, held_id)
        .into_iter()
        .next()
        .expect("a creature on the board makes it castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);

    assert_eq!(
        battlefield_count(&game, cards::GRIZZLY_BEARS),
        0,
        "the Bears paid for it"
    );
    assert_eq!(game.players[0].hand.len(), 2, "and two cards came back");
}

/// Two creatures means two, not one: the Tribute stays uncastable while only
/// half the price is on the board.
#[test]
fn phyrexian_tribute_charges_two_creatures() {
    let stage = |creatures: usize| {
        let mut game = ready();
        let held = card(78_100, cards::PHYREXIAN_TRIBUTE, PlayerId::One);
        let held_id = held.id;
        game.players[0].hand.push(held);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
        game.battlefield
            .push(creature(78_101, cards::ORNITHOPTER, PlayerId::Two));
        for index in 0..creatures {
            game.battlefield.push(creature(
                78_110 + u32::try_from(index).expect("a small fixture"),
                cards::GRIZZLY_BEARS,
                PlayerId::One,
            ));
        }
        (game, held_id)
    };

    let (game, held_id) = stage(1);
    assert!(
        offered(&game, held_id).is_empty(),
        "one creature does not pay a two-creature cost"
    );

    let (mut game, held_id) = stage(2);
    let cast = offered(&game, held_id)
        .into_iter()
        .next()
        .expect("two creatures pay it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    assert_eq!(
        battlefield_count(&game, cards::GRIZZLY_BEARS),
        0,
        "both Bears were spent, not just one"
    );
    assert_eq!(
        battlefield_count(&game, cards::ORNITHOPTER),
        0,
        "and the artifact it was aimed at is gone"
    );
}

/// A life cost is paid on the way to the stack too, and the Fumarole takes
/// two different permanents with one card.
#[test]
fn fumarole_pays_three_life_and_takes_a_creature_and_a_land() {
    let mut game = ready();
    let held = card(78_200, cards::FUMAROLE, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);
    game.battlefield
        .push(creature(78_201, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(78_202, cards::MOUNTAIN, PlayerId::Two));

    let before = game.players[0].life;
    let cast = offered(&game, held_id)
        .into_iter()
        .next()
        .expect("both a creature and a land are on the board");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);

    assert_eq!(
        game.players[0].life,
        before - 3,
        "the three life was actually charged"
    );
    assert_eq!(
        battlefield_count(&game, cards::GRIZZLY_BEARS),
        0,
        "the creature half resolved"
    );
    assert_eq!(
        battlefield_count(&game, cards::MOUNTAIN),
        0,
        "and so did the land half"
    );
}
