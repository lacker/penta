//! Two shapes that read the board rather than an event: a static that
//! switches on whether you control a creature of some type, and an
//! activation paid by tapping another permanent you control. Both are
//! silent when they are wrong -- the creature is simply the printed size,
//! or the ability is simply never offered.

use super::*;

/// Cloudreach Cavalry under player one, with a Bird under `bird_owner` if
/// there is one at all.
fn cavalry(bird_owner: Option<PlayerId>) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut cavalry = creature(74_000, cards::CLOUDREACH_CAVALRY, PlayerId::One);
    cavalry.entered_controller_turn = 0;
    let cavalry_id = cavalry.card.id;
    game.battlefield.push(cavalry);
    if let Some(owner) = bird_owner {
        let mut bird = creature(74_001, cards::SOULCATCHER, owner);
        bird.entered_controller_turn = 0;
        game.battlefield.push(bird);
    }
    (game, cavalry_id)
}

fn shape(game: &Game, id: GameObjectId) -> (i16, bool) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Cavalry is on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.has_flying(permanent),
    )
}

#[test]
fn the_cavalry_is_a_bear_with_no_bird() {
    let (game, cavalry) = cavalry(None);
    assert_eq!(shape(&game, cavalry), (1, false), "the printed body");
}

#[test]
fn a_bird_you_control_switches_it_on() {
    let (game, cavalry) = cavalry(Some(PlayerId::One));
    assert_eq!(
        shape(&game, cavalry),
        (3, true),
        "three power and flying while the Bird is there"
    );
}

#[test]
fn the_opponents_bird_does_nothing() {
    let (game, cavalry) = cavalry(Some(PlayerId::Two));
    assert_eq!(
        shape(&game, cavalry),
        (1, false),
        "the clause says you control one"
    );
}

/// Opposition under player one with `creatures` untapped creatures, and one
/// untapped Forest of the opponent's to aim at.
fn opposition(creatures: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut enchantment = creature(74_100, cards::OPPOSITION, PlayerId::One);
    enchantment.entered_controller_turn = 0;
    game.battlefield.push(enchantment);
    for index in 0..creatures {
        let mut bear = creature(
            74_110 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        );
        bear.entered_controller_turn = 0;
        game.battlefield.push(bear);
    }
    let mut forest = creature(74_200, cards::FOREST, PlayerId::Two);
    forest.entered_controller_turn = 0;
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    (game, forest_id)
}

fn tap_action(game: &Game, target: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == GameObjectId(74_100)
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(target))
            }
            _ => false,
        })
}

#[test]
fn opposition_needs_an_untapped_creature_to_pay_with() {
    let (game, forest) = opposition(0);
    assert!(
        tap_action(&game, forest).is_none(),
        "with no creature there is nothing to tap"
    );
}

#[test]
fn opposition_taps_a_land_by_tapping_a_bear() {
    let (mut game, forest) = opposition(1);
    let activation = tap_action(&game, forest).expect("the Bears pay for it");
    game.apply(PlayerId::One, activation)
        .expect("the cost is payable");
    for _ in 0..8 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == forest && permanent.tapped),
        "the opponent's Forest is tapped"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(74_110) && permanent.tapped),
        "and the Bears paid for it"
    );
}
