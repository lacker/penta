//! A land that turns itself into a creature. Two things separate a working
//! animation from a definition that merely type-checks: the land has to be
//! declarable as an attacker afterwards, and it has to stay a land, so its
//! mana ability is still there to pay for the next spell.

use super::*;

/// Treetop Village under player one, animated when `animate` is set.
fn village(animate: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let mut village = creature(95_000, cards::TREETOP_VILLAGE, PlayerId::One);
    village.entered_controller_turn = 0;
    let village_id = village.card.id;
    game.battlefield.push(village);
    if animate {
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let activation = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(
                |action| matches!(action, Action::ActivateAbility { source, .. } if *source == village_id),
            )
            .expect("the animation is offered");
        game.apply(PlayerId::One, activation)
            .expect("two mana pays for it");
        pass_priority_pair(&mut game);
    }
    (game, village_id)
}

fn can_attack(game: &mut Game, attacker: GameObjectId) -> bool {
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: a, .. } if *a == attacker),
    )
}

#[test]
fn an_unanimated_village_is_only_a_land() {
    let (mut game, village) = village(false);
    assert!(
        !can_attack(&mut game, village),
        "a land cannot be declared as an attacker"
    );
}

#[test]
fn animating_it_makes_a_three_by_three_that_attacks() {
    let (mut game, village) = village(true);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == village)
        .expect("the Village is on the battlefield");
    assert_eq!(
        (
            game.power(permanent).expect("power"),
            game.toughness(permanent).expect("toughness")
        ),
        (3, 3),
        "the printed body of the animation"
    );
    assert!(
        can_attack(&mut game, village),
        "and it may now be declared as an attacker"
    );
}

#[test]
fn the_animated_village_is_still_a_land() {
    let (mut game, _) = village(true);
    game.players[0].mana_pool = ManaPool::default();
    let spell = card(95_100, cards::GIANT_GROWTH, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    let mut bear = creature(95_101, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    game.battlefield.push(bear);

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell_id)),
        "the Village still taps for the green mana the Growth needs"
    );
}
