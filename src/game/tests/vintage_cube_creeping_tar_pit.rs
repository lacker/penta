//! Creeping Tar Pit: a land that pays for itself and then walks past
//! everything, and the two rulings a land that becomes a creature carries.

use super::*;

/// The Tar Pit on the battlefield, untapped and there since last turn
/// unless `fresh`, with the mana to animate it.
fn staged(fresh: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let pit = game
        .put_onto_battlefield(PlayerId::One, cards::CREEPING_TAR_PIT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == pit)
    {
        permanent.tapped = false;
        permanent.entered_controller_turn = if fresh { 5 } else { 0 };
    }
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, pit)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Pays the three and lets the animation resolve.
fn animate(game: &mut Game, pit: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == pit))
        .expect("three mana animates it");
    game.apply(PlayerId::One, action).expect("it activates");
    drain_pending(game);
}

/// It arrives tapped, which is the price of the two colours.
#[test]
fn it_enters_tapped() {
    let mut game = ready_game();
    game.battlefield.clear();

    let pit = game
        .put_onto_battlefield(PlayerId::One, cards::CREEPING_TAR_PIT)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(permanent(&game, pit).tapped, "tapped on arrival");
}

/// Blue or black, and nothing else.
#[test]
fn it_taps_for_its_two_colors() {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        let (mut game, pit) = staged(false);
        let offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| match action {
                Action::ActivateManaAbility {
                    source,
                    color: made,
                    ..
                } => source == pit && made == color,
                _ => false,
            });
        assert_eq!(
            offered,
            matches!(color, ManaColor::Blue | ManaColor::Black),
            "{color:?} off a Tar Pit",
        );
        let _ = &mut game;
    }
}

/// Animated it is a 3/2 blue and black Elemental, still a land, and nothing
/// may block it.
#[test]
fn animating_it_makes_an_unblockable_three_two() {
    let (mut game, pit) = staged(false);

    animate(&mut game, pit);

    let body = permanent(&game, pit);
    assert_eq!((game.power(body), game.toughness(body)), (Some(3), Some(2)));
    let types = game.permanent_types(body).expect("it has types");
    assert!(types.contains(CardType::Creature), "a creature now");
    assert!(types.contains(CardType::Land), "and a land still");
    assert!(
        game.effective_subtypes(body)
            .contains(crate::card::Subtype::named("Elemental")),
        "an Elemental at that",
    );
    let colors = game.permanent_colors(body);
    for color in [ManaColor::Blue, ManaColor::Black] {
        assert!(
            colors[color.color_index().expect("a colour")],
            "{color:?} is one of its two",
        );
    }

    let blocker = creature(63_900, cards::SERRA_ANGEL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(pit, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareBlocker { blocker, .. } if *blocker == blocker_id)
        }),
        "nothing blocks it this turn",
    );
}

/// "Until end of turn": the turn after, it is a land again.
#[test]
fn the_animation_wears_off() {
    let (mut game, pit) = staged(false);
    animate(&mut game, pit);
    assert!(
        game.permanent_types(permanent(&game, pit))
            .is_some_and(|types| types.contains(CardType::Creature)),
        "a creature while the turn lasts",
    );

    // Walking the steps rather than jumping the turn: the cleanup step is
    // where an until-end-of-turn effect is let go of.
    let turn = game.turn;
    for _ in 0..60 {
        if game.turn > turn {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }

    assert!(
        !game
            .permanent_types(permanent(&game, pit))
            .is_some_and(|types| types.contains(CardType::Creature)),
        "and a plain land afterwards",
    );
}

/// "Summoning sickness cares about when that permanent came under your
/// control, not when it became a creature." A Tar Pit played this turn is
/// animated and still cannot attack.
#[test]
fn a_tar_pit_played_this_turn_cannot_attack() {
    let (mut game, pit) = staged(true);
    animate(&mut game, pit);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == pit)
        ),
        "it has been a creature for a moment and a permanent for no turns",
    );
}

/// "When a land becomes a creature, that doesn't count as having a creature
/// enter." A Midnight Guard watching the battlefield is not woken by it.
#[test]
fn becoming_a_creature_is_not_a_creature_entering() {
    let (mut game, pit) = staged(false);
    let guard = game
        .put_onto_battlefield(PlayerId::One, cards::MIDNIGHT_GUARD)
        .expect("cataloged");
    drain_pending(&mut game);
    game.tap_permanent(guard);
    game.priority = PlayerId::One;
    assert!(permanent(&game, guard).tapped, "the Guard is asleep");

    animate(&mut game, pit);

    assert!(
        permanent(&game, guard).tapped,
        "the permanent was already there and only changed its types",
    );
}
