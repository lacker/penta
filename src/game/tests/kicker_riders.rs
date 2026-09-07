//! Kicker riders that are read after the spell has already resolved. The
//! payment is recorded on the permanent or the spell, and three different
//! places have to be able to ask about it: a battlefield-entry replacement,
//! a static ability on the permanent that arrived, and a resolving effect
//! choosing between two branches. The middle one is the interesting case --
//! nothing else in the catalog asks a static walk what a spell was cast for.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
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

/// Casts `definition` from hand with plenty of mana, kicked or not.
fn cast(game: &mut Game, id: u32, definition: CardDefinitionId, kicked: bool) {
    let held = card(id, definition, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 6);
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
        .expect("both the kicked and the plain cast are offered");
    game.apply(PlayerId::One, chosen).expect("it is cast");
    resolve(game);
}

fn creature_on_board(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(definition))
        .expect("the creature resolved")
}

/// The counters come from an entry replacement and the trample from a static
/// that has to ask the same question afterwards. Both halves are checked,
/// because the counters arriving is no evidence the keyword did.
#[test]
fn kavu_titan_is_a_bear_unkicked_and_a_trampler_kicked() {
    let mut plain = ready();
    cast(&mut plain, 80_000, cards::KAVU_TITAN, false);
    let titan = creature_on_board(&plain, cards::KAVU_TITAN);
    let stats = plain.creature_stats(titan).expect("a creature");
    assert_eq!((stats.power, stats.toughness), (2, 2), "the printed body");
    assert!(
        !plain.has_trample(titan),
        "and no trample without the kicker"
    );

    let mut kicked = ready();
    cast(&mut kicked, 80_001, cards::KAVU_TITAN, true);
    let titan = creature_on_board(&kicked, cards::KAVU_TITAN);
    let stats = kicked.creature_stats(titan).expect("a creature");
    assert_eq!(
        (stats.power, stats.toughness),
        (5, 5),
        "three +1/+1 counters on a 2/2"
    );
    assert!(
        kicked.has_trample(titan),
        "and the static rider found the kicker payment"
    );
}

/// Two branches of one resolving effect rather than two effects, so what is
/// checked is that the unkicked half is not silently the kicked one.
#[test]
fn explosive_growth_picks_its_branch_from_the_kicker() {
    for (kicked, expected) in [(false, (4, 4)), (true, (7, 7))] {
        let mut game = ready();
        game.battlefield
            .push(creature(80_100, cards::GRIZZLY_BEARS, PlayerId::One));
        cast(&mut game, 80_101, cards::EXPLOSIVE_GROWTH, kicked);
        let bears = creature_on_board(&game, cards::GRIZZLY_BEARS);
        let stats = game.creature_stats(bears).expect("a creature");
        assert_eq!(
            (stats.power, stats.toughness),
            expected,
            "a 2/2 pumped by the {} half",
            if kicked { "kicked" } else { "plain" }
        );
    }
}

/// An intervening-if on the enters trigger: unkicked, the trigger must not
/// even go on the stack, so the land it would have taken is still there.
#[test]
fn the_benalish_emissary_only_takes_a_land_when_it_was_kicked() {
    for (kicked, remaining) in [(false, 1), (true, 0)] {
        let mut game = ready();
        game.battlefield
            .push(creature(80_200, cards::MOUNTAIN, PlayerId::Two));
        cast(&mut game, 80_201, cards::BENALISH_EMISSARY, kicked);
        let lands = game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == ObjectKind::Card(cards::MOUNTAIN))
            .count();
        assert_eq!(
            lands, remaining,
            "the Mountain survives exactly when the Emissary was not kicked"
        );
    }
}
