//! Burst Lightning: one mana to answer what a one-drop deck leads with, and
//! five to point the same card at anything later.

use super::*;

/// Burst Lightning in hand with `mana` colorless beside one red, and a
/// Grizzly Bears across the table.
fn staged(extra: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bolt = game
        .build_zone(PlayerId::One, &[cards::BURST_LIGHTNING])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, extra);
    game.players[1].life = 20;
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id, bears)
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
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

/// Every way of casting it at `target`, cheapest first.
fn casts_at(game: &Game, bolt: GameObjectId, target: Target) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| match action {
            Action::CastSpell {
                card: cast,
                choices,
                ..
            } => *cast == bolt && choices.iter_targets().any(|chosen| *chosen == target),
            _ => false,
        })
        .collect()
}

fn cast(game: &mut Game, bolt: GameObjectId, target: Target, kicked: bool) {
    let action = casts_at(game, bolt, target)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { choices, .. }
                if choices.costs().alternative().is_none()
                    && choices.costs().additional().is_empty() != kicked)
        })
        .unwrap_or_else(|| panic!("it is castable (kicked: {kicked})"));
    game.apply(PlayerId::One, action).expect("it is cast");
    settle(game);
}

/// One red deals two, which kills the two-drop it is usually pointed at.
#[test]
fn unkicked_it_deals_two() {
    let (mut game, bolt, bears) = staged(0);

    cast(&mut game, bolt, Target::Permanent(bears), false);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GRIZZLY_BEARS),
        "two is enough for a 2/2",
    );
}

/// Kicked it deals four, which is the whole reason to hold it.
#[test]
fn kicked_it_deals_four() {
    let (mut game, bolt, _) = staged(4);

    cast(&mut game, bolt, Target::Player(PlayerId::Two), true);

    assert_eq!(game.players[1].life, 16, "four damage");
}

/// Unkicked at a player is two, so the two casts are really two sizes.
#[test]
fn the_small_cast_is_still_two_at_a_player() {
    let (mut game, bolt, _) = staged(4);

    cast(&mut game, bolt, Target::Player(PlayerId::Two), false);

    assert_eq!(game.players[1].life, 18, "the cheap one deals two");
}

/// Without the extra four there is only one way to cast it.
#[test]
fn one_mana_offers_only_the_small_cast() {
    let (game, bolt, _) = staged(0);

    let offered = casts_at(&game, bolt, Target::Player(PlayerId::Two));
    assert_eq!(offered.len(), 1, "one red buys one cast: {offered:?}");
    assert!(
        offered.iter().all(|action| matches!(action,
            Action::CastSpell { choices, .. }
                if choices.costs().alternative().is_none()
                    && choices.costs().additional().is_empty())),
        "and it is the unkicked one",
    );
}

/// With five available both are offered: the kicker is a choice rather than
/// a discount.
#[test]
fn five_mana_offers_both() {
    let (game, bolt, _) = staged(4);

    let offered = casts_at(&game, bolt, Target::Player(PlayerId::Two));
    assert_eq!(offered.len(), 2, "small and kicked: {offered:?}");
}
