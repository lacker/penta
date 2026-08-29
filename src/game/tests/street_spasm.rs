//! An overload cost that doubles X.
//!
//! Both halves read the same X, but the overload charges two mana for each
//! point of it and reaches every grounded creature instead of one. Fliers are
//! outside both halves, and so are the caster's own creatures.

use super::*;

/// Street Spasm in player one's hand with `red` mana, two grounded creatures
/// and one flier under player two, and one grounded creature of player one's.
fn spasm(
    red: u16,
) -> (
    Game,
    CardInstanceId,
    Vec<GameObjectId>,
    GameObjectId,
    GameObjectId,
) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    let mut theirs = Vec::new();
    for index in 0..2 {
        let creature = creature(10_000 + index, cards::GRIZZLY_BEARS, PlayerId::Two);
        theirs.push(creature.card.id);
        game.battlefield.push(creature);
    }
    let flier = creature(10_100, cards::AIR_ELEMENTAL, PlayerId::Two);
    let flier_id = flier.card.id;
    game.battlefield.push(flier);
    let mine = creature(10_200, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);

    let spell = card(20_000, cards::STREET_SPASM, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = red;
    game.priority = PlayerId::One;
    (game, spell_id, theirs, flier_id, mine_id)
}

/// Every cast on offer, as (X, target count) pairs.
fn offered_shapes(game: &Game, spell: CardInstanceId) -> Vec<(u16, usize)> {
    let mut shapes = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => Some((
                choices.x(),
                choices
                    .targets()
                    .iter()
                    .map(|slot| slot.targets().len())
                    .sum(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    shapes.sort_unstable();
    shapes.dedup();
    shapes
}

fn still_there(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// {X}{R} targeted, {X}{X}{R}{R} overloaded. Five red buys X=4 targeted but
/// only X=1 overloaded, so both appear among the offers.
#[test]
fn the_overload_costs_two_mana_for_each_point_of_x() {
    let (game, spell, _theirs, _flier, _mine) = spasm(5);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 4, "the targeted half stretches furthest");

    // The overload takes no targets, so its offers are the ones with none.
    let overloaded = offered_shapes(&game, spell)
        .into_iter()
        .filter(|(_, targets)| *targets == 0)
        .map(|(x, _)| x)
        .max()
        .expect("the overload is on offer");
    assert_eq!(overloaded, 1, "five red is {{R}}{{R}} plus one doubling");
}

/// The control: not enough for the overload's coloured pips at all.
#[test]
fn one_red_offers_only_the_targeted_half() {
    let (game, spell, _theirs, _flier, _mine) = spasm(1);
    assert!(
        offered_shapes(&game, spell)
            .iter()
            .all(|(_, targets)| *targets == 1),
        "every offer picks a target",
    );
}

#[test]
fn the_targeted_half_kills_one_grounded_creature() {
    let (mut game, spell, theirs, flier, mine) = spasm(3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.x() == 2
                    && choices.targets().iter().any(|slot| slot.targets() == [Target::Permanent(theirs[0])]))
        })
        .expect("two damage at one of their Bears");
    game.apply(PlayerId::One, action)
        .expect("three red covers it");
    drain_pending(&mut game);

    assert!(!still_there(&game, theirs[0]), "two killed a 2/2");
    assert!(still_there(&game, theirs[1]), "the other one is untouched");
    assert!(still_there(&game, flier));
    assert!(still_there(&game, mine));
}

/// Overloaded, it reaches every grounded creature they control and nothing
/// else.
#[test]
fn the_overload_sweeps_their_grounded_creatures_only() {
    let (mut game, spell, theirs, flier, mine) = spasm(6);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell && choices.x() == 2 && choices.targets().iter().all(|slot| slot.targets().is_empty()))
        })
        .expect("six red is {R}{R} plus two doublings");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(!still_there(&game, theirs[0]));
    assert!(!still_there(&game, theirs[1]));
    assert!(still_there(&game, flier), "a flier is outside both halves");
    assert!(still_there(&game, mine), "and so is my own creature");
}
