//! Two Gatecrash cards whose audit lines had gone stale.
//!
//! Frilled Oculus wanted a once-per-turn activation ration, and Gridlock an
//! X-counted target slot. Both had been built for other cards, so what these
//! check is that each card really is using the primitive its text names.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// The pump runs once and then the ability is gone for the turn, however much
/// green is left.
#[test]
fn the_oculus_pumps_once_a_turn() {
    let mut game = ready();
    let oculus = creature(10_000, cards::FRILLED_OCULUS, PlayerId::One);
    let oculus_id = oculus.card.id;
    game.battlefield.push(oculus);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    assert_eq!(stats(&game, oculus_id), (Some(1), Some(3)));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == oculus_id))
        .expect("one green and one generic is enough");
    game.apply(PlayerId::One, action)
        .expect("the cost is payable");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert_eq!(stats(&game, oculus_id), (Some(3), Some(5)));
    assert!(
        !offers(&game, oculus_id),
        "the ration closed it for the rest of the turn",
    );
}

/// And it opens again next turn.
#[test]
fn the_ration_returns_with_the_turn() {
    let mut game = ready();
    let oculus = creature(10_000, cards::FRILLED_OCULUS, PlayerId::One);
    let oculus_id = oculus.card.id;
    game.battlefield.push(oculus);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == oculus_id))
        .expect("on offer");
    game.apply(PlayerId::One, action).expect("payable");
    drain_pending(&mut game);

    // Walked rather than jumped, so the cleanup that ends an
    // until-end-of-turn effect actually happens.
    for _ in 0..12 {
        if game.step == Step::Cleanup {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    assert!(offers(&game, oculus_id), "a new turn, a new activation");
    assert_eq!(
        stats(&game, oculus_id),
        (Some(1), Some(3)),
        "and last turn's pump has worn off",
    );
}

/// Gridlock in hand with `blue` available, over `permanents` nonland
/// permanents and one land.
fn gridlocked(
    permanents: u32,
    blue: u16,
) -> (Game, CardInstanceId, Vec<GameObjectId>, GameObjectId) {
    let mut game = ready();
    let mut ids = Vec::new();
    for index in 0..permanents {
        let permanent = creature(10_000 + index, cards::GRIZZLY_BEARS, PlayerId::Two);
        ids.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    let land = creature(10_500, cards::ISLAND, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let spell = card(20_000, cards::GRIDLOCK, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = blue;
    (game, spell_id, ids, land_id)
}

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

#[test]
fn gridlock_takes_exactly_as_many_targets_as_the_x_paid() {
    let (game, spell, _permanents, _land) = gridlocked(3, 4);
    let shapes = offered_shapes(&game, spell);
    assert!(!shapes.is_empty(), "the spell is castable");
    for (x, count) in shapes {
        assert_eq!(
            usize::from(u8::try_from(x).expect("small X")),
            count,
            "X={x} took {count}",
        );
    }
}

/// The land is not a legal target, so three nonland permanents is the
/// ceiling however much blue is spare.
#[test]
fn the_land_is_not_among_the_targets() {
    let (game, spell, _permanents, _land) = gridlocked(3, 8);
    let largest = offered_shapes(&game, spell)
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .expect("something is on offer");
    assert_eq!(largest, 3, "the land does not raise the ceiling");
}

#[test]
fn gridlock_taps_the_permanents_chosen() {
    let (mut game, spell, permanents, land) = gridlocked(3, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.x() == 2
                    && choices.targets().iter().any(|slot| slot.targets()
                        == [Target::Permanent(permanents[0]), Target::Permanent(permanents[1])]))
        })
        .expect("two of the three is a legal choice");
    game.apply(PlayerId::One, action)
        .expect("three blue covers {X=2}{U}");
    drain_pending(&mut game);

    let tapped = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .tapped
    };
    assert!(tapped(permanents[0]) && tapped(permanents[1]));
    assert!(!tapped(permanents[2]), "the untargeted one stayed up");
    assert!(!tapped(land), "and so did the land");
}
