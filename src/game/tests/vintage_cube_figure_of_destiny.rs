//! Figure of Destiny: a one-drop that keeps eating mana, and the two gates
//! that say the steps have to be climbed in order.

use super::*;

/// Player One with a Figure that has been out a turn and `mana` hybrid-payable
/// red available.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let figure = game
        .put_onto_battlefield(PlayerId::One, cards::FIGURE_OF_DESTINY)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, mana);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, figure)
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Activates the ability whose printed text starts with `prefix`, or returns
/// false when it is not offered.
fn activate(game: &mut Game, figure: GameObjectId, prefix: &str) -> bool {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, ability, ..
            } => *source == figure && ability_text(game, figure, *ability).starts_with(prefix),
            _ => false,
        });
    let Some(action) = action else {
        return false;
    };
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
    true
}

fn ability_text(game: &Game, source: GameObjectId, origin: AbilityOrigin) -> &'static str {
    let mut text = "";
    let _ = game.visit_effective_abilities(permanent(game, source), |effective| {
        if effective.origin == origin {
            text = effective.ability.text;
            return std::ops::ControlFlow::Break(());
        }
        std::ops::ControlFlow::Continue(())
    });
    text
}

fn subtypes(game: &Game, figure: GameObjectId) -> Vec<String> {
    game.effective_subtypes(permanent(game, figure))
        .iter()
        .map(|subtype| subtype.name().to_string())
        .collect()
}

/// One hybrid mana turns a 1/1 Kithkin into a 2/2 Kithkin Spirit, and it
/// stays that way: nothing about the step ends.
#[test]
fn the_first_step_makes_a_two_two_spirit_for_good() {
    let (mut game, figure) = staged(1);
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(1),
        "a 1/1 to start"
    );

    assert!(activate(&mut game, figure, "{R/W}:"), "one mana is enough");

    assert_eq!(game.power(permanent(&game, figure)), Some(2), "now a 2/2");
    assert_eq!(
        subtypes(&game, figure),
        vec!["Kithkin".to_string(), "Spirit".to_string()],
        "a Kithkin Spirit, which is what \"becomes\" repaints rather than adds",
    );

    game.finish_cleanup();
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(2),
        "and it is still a 2/2 once the turn it was made in is over",
    );
}

/// Hybrid mana is paid with either half: a mono-white board climbs the same
/// ladder a mono-red one does.
#[test]
fn white_mana_pays_the_hybrid_too() {
    let (mut game, figure) = staged(0);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);

    assert!(activate(&mut game, figure, "{R/W}:"), "white pays {{R/W}}");
    assert_eq!(game.power(permanent(&game, figure)), Some(2), "a 2/2");
}

/// The second step reads the board: a Figure that is still a plain Kithkin
/// pays three mana and gets nothing.
#[test]
fn the_second_step_does_nothing_to_a_figure_that_is_not_a_spirit() {
    let (mut game, figure) = staged(3);

    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}:"),
        "the ability is activatable whether or not it will do anything",
    );

    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(1),
        "still a 1/1: it was not a Spirit",
    );
    assert_eq!(
        subtypes(&game, figure),
        vec!["Kithkin".to_string()],
        "and still only a Kithkin",
    );
}

/// Climbed in order, each step lands.
#[test]
fn the_whole_ladder_ends_at_an_eight_eight_flier() {
    let (mut game, figure) = staged(10);

    assert!(activate(&mut game, figure, "{R/W}:"), "step one");
    assert!(activate(&mut game, figure, "{R/W}{R/W}{R/W}:"), "step two");
    assert_eq!(game.power(permanent(&game, figure)), Some(4), "a 4/4");
    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}:"),
        "step three",
    );

    let figure_permanent = permanent(&game, figure);
    assert_eq!(game.power(figure_permanent), Some(8), "an 8/8");
    assert!(
        game.permanent_has_executable_keyword(figure_permanent, KeywordAbility::Flying),
        "with flying",
    );
    assert!(
        game.permanent_has_executable_keyword(figure_permanent, KeywordAbility::FirstStrike),
        "and first strike",
    );
    assert_eq!(
        subtypes(&game, figure),
        vec![
            "Avatar".to_string(),
            "Kithkin".to_string(),
            "Spirit".to_string(),
            "Warrior".to_string(),
        ],
        "all four types, in canonical vocabulary order",
    );
}

/// The third step is gated on the second, not on the first: a Spirit that
/// never became a Warrior does not skip a rung.
#[test]
fn the_third_step_will_not_skip_the_second() {
    let (mut game, figure) = staged(7);
    assert!(activate(&mut game, figure, "{R/W}:"), "step one");

    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}:"),
        "six mana is payable",
    );

    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(2),
        "still the 2/2 Spirit: it was never a Warrior",
    );
    assert!(
        !game.permanent_has_executable_keyword(permanent(&game, figure), KeywordAbility::Flying),
        "and it does not fly",
    );
}

/// "If Figure of Destiny is an 8/8 Kithkin Spirit Warrior Avatar with flying
/// and first strike, and you activate its first ability, it will become a
/// 2/2 Kithkin Spirit that still has flying and first strike." The steps
/// repaint the types and the base power and toughness; nothing takes an
/// ability back once it has been handed over.
#[test]
fn stepping_back_down_keeps_the_wings() {
    let (mut game, figure) = staged(11);
    assert!(activate(&mut game, figure, "{R/W}:"), "step one");
    assert!(activate(&mut game, figure, "{R/W}{R/W}{R/W}:"), "step two");
    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}:"),
        "step three",
    );
    assert_eq!(game.power(permanent(&game, figure)), Some(8), "an 8/8");

    // One more hybrid mana, spent going backwards.
    assert!(activate(&mut game, figure, "{R/W}:"), "step one again");

    let figure_permanent = permanent(&game, figure);
    assert_eq!(game.power(figure_permanent), Some(2), "a 2/2 again");
    assert_eq!(game.toughness(figure_permanent), Some(2));
    assert_eq!(
        subtypes(&game, figure),
        vec!["Kithkin".to_string(), "Spirit".to_string()],
        "and a Kithkin Spirit again, the Warrior and Avatar painted over",
    );
    assert!(
        game.permanent_has_executable_keyword(figure_permanent, KeywordAbility::Flying),
        "but the flying stayed: the first step never mentions it",
    );
    assert!(
        game.permanent_has_executable_keyword(figure_permanent, KeywordAbility::FirstStrike),
        "and so did the first strike",
    );
}

/// "It will not overwrite effects that modify power or toughness (whether
/// from a static ability, counters, or a resolved spell or ability)." The
/// step sets the base; a +1/+1 counter is still added on top of it.
#[test]
fn a_counter_rides_on_top_of_the_new_base() {
    let (mut game, figure) = staged(1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == figure)
        .expect("it is on the battlefield")
        .add_counters(CounterKind::PlusOnePlusOne, 1);
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(2),
        "a 1/1 and one"
    );

    assert!(activate(&mut game, figure, "{R/W}:"), "step one");

    let figure_permanent = permanent(&game, figure);
    assert_eq!(
        game.power(figure_permanent),
        Some(3),
        "base two, and the counter still counts",
    );
    assert_eq!(game.toughness(figure_permanent), Some(3));
}

/// None of the steps taps it, so nothing about the turn it arrived is in the
/// way: a Figure played this turn climbs the first rung the same turn.
#[test]
fn a_figure_that_just_arrived_can_still_climb() {
    let (mut game, figure) = staged(1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == figure)
        .expect("it is there")
        .entered_controller_turn = game.turns_started[PlayerId::One.index()];

    assert!(
        activate(&mut game, figure, "{R/W}:"),
        "an untapping cost is no cost a fresh creature cannot pay",
    );
    settle(&mut game);

    let body = permanent(&game, figure);
    assert_eq!((game.power(body), game.toughness(body)), (Some(2), Some(2)));
    assert!(
        !body.tapped,
        "and it is still untapped, which is why it could",
    );
}

/// The steps are ordinary activated abilities with no timing restriction:
/// the ladder is climbed on their turn as readily as on yours, which is how
/// a Figure grows out of range of the removal they just cast.
#[test]
fn the_ladder_may_be_climbed_on_their_turn() {
    let (mut game, figure) = staged(4);
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.priority = PlayerId::One;

    assert!(
        activate(&mut game, figure, "{R/W}:"),
        "the first step is offered in their end step",
    );
    settle(&mut game);
    // Passing priority around the step emptied the pool and handed the seat
    // back, as it does in any game; the timing claim is what this is about.
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
    game.priority = PlayerId::One;
    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}:"),
        "and so is the second",
    );
    settle(&mut game);

    let body = permanent(&game, figure);
    assert_eq!(
        (game.power(body), game.toughness(body)),
        (Some(4), Some(4)),
        "a 4/4 before their turn is over",
    );
    assert!(subtypes(&game, figure).iter().any(|kind| kind == "Warrior"));
}

/// Each hybrid pip is settled on its own, so one activation may be paid with
/// both halves at once. Every test above spends a single colour; three pips
/// bought with two reds and a white is the thing hybrid actually allows.
#[test]
fn one_step_may_be_paid_with_both_halves() {
    let (mut game, figure) = staged(0);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    assert!(
        activate(&mut game, figure, "{R/W}:"),
        "white buys the first"
    );
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(2),
        "a 2/2 Spirit"
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);

    assert!(
        activate(&mut game, figure, "{R/W}{R/W}{R/W}:"),
        "two red and a white pay three hybrid pips between them",
    );
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(4),
        "and the Spirit is a 4/4 Warrior for it",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana.len(),
        0,
        "all three were spent, whichever colour each one was",
    );
}

/// A hybrid pip is one of two colours and never a generic one: a pool of
/// colorless climbs no rungs at all.
#[test]
fn colorless_mana_pays_no_hybrid_pip() {
    let (mut game, figure) = staged(0);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);

    assert!(
        !activate(&mut game, figure, "{R/W}:"),
        "six colorless is not one red or white",
    );
    assert_eq!(
        game.power(permanent(&game, figure)),
        Some(1),
        "so it is the 1/1 it was printed as",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    assert!(
        activate(&mut game, figure, "{R/W}:"),
        "and one red is what it wanted all along",
    );
}
