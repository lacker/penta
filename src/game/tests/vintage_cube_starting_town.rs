//! Starting Town: a City of Brass for the turns that matter, and a tapped
//! land after them.

use super::*;

/// Player One holding a Starting Town, having taken `turns` turns.
fn staged(turns: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let town = game
        .build_zone(PlayerId::One, &[cards::STARTING_TOWN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let town_id = town.id;
    game.players[0].hand.push(town);
    game.turns_started = [turns, turns];
    game.turn = turns * 2;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[0].life = 20;
    game.players[0].lands_played_this_turn = 0;
    (game, town_id)
}

/// Plays the land and hands back the permanent it became.
fn play(game: &mut Game, town: GameObjectId) -> &Permanent {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == town))
        .expect("a land drop is available");
    game.apply(PlayerId::One, action).expect("it is played");
    drain_pending(game);
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::STARTING_TOWN)
        .expect("it is on the battlefield")
}

/// Your first three turns: it comes in ready to use.
#[test]
fn it_enters_untapped_early() {
    for turn in 1..=3 {
        let (mut game, town) = staged(turn);

        assert!(
            !play(&mut game, town).tapped,
            "turn {turn} of yours is one of the first three",
        );
    }
}

/// The fourth is one turn too many.
#[test]
fn it_enters_tapped_later() {
    let (mut game, town) = staged(4);

    assert!(play(&mut game, town).tapped, "and the fourth is not");
}

/// The turns counted are yours: on the draw, your third turn is the game's
/// sixth, and the Town does not care about the number on the turn.
#[test]
fn it_counts_your_turns_rather_than_the_games() {
    let (mut game, town) = staged(3);
    game.turn = 6;

    assert!(
        !play(&mut game, town).tapped,
        "three turns taken is three turns taken",
    );
}

/// Colourless for free, or any colour for a life.
#[test]
fn it_taps_for_colourless_or_anything_for_a_life() {
    let (mut game, town) = staged(1);
    let id = play(&mut game, town).card.id;

    let mana = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(mana.len(), 6, "colourless plus the five colours");

    let colored = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, color, .. }
                if *source == id && *color == ManaColor::Green)
        })
        .expect("green is on offer");
    game.apply(PlayerId::One, colored).expect("it activates");

    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.players[0].life, 19, "a life for the colour");
}

/// At zero life the coloured half is not payable, but the colourless half
/// costs nothing and stays.
#[test]
fn with_no_life_only_the_colourless_half_remains() {
    let (mut game, town) = staged(1);
    let id = play(&mut game, town).card.id;
    game.players[0].life = 0;

    let mana = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(mana, vec![ManaColor::Colorless], "only the free half");
}

/// A life you have is a life you may pay, and the Town does not ask whether
/// it was your last one: the colour comes, and the state-based actions that
/// follow end the game.
#[test]
fn the_last_life_is_still_a_price_you_may_pay() {
    let (mut game, town) = staged(1);
    let id = play(&mut game, town).card.id;
    game.players[0].life = 1;

    let colored = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, color, .. }
                if *source == id && *color == ManaColor::Blue)
        })
        .expect("one life is enough to pay one life");
    game.apply(PlayerId::One, colored).expect("it activates");

    assert_eq!(game.players[0].mana_pool.blue, 1, "the mana is yours");
    assert_eq!(game.players[0].life, 0);

    game.check_state_based_actions();
    assert!(
        matches!(
            game.result,
            Some(GameResult::Winner {
                winner: PlayerId::Two,
                ..
            })
        ),
        "and then you lose with the mana still in your pool",
    );
}

/// "Town is a land type with no special meaning." It is the only subtype the
/// land has, it is not one of the five that carry a mana ability, and every
/// drop of mana the Town makes comes from its own printed lines.
#[test]
fn the_town_type_grants_nothing_by_itself() {
    let (mut game, town) = staged(1);
    let id = play(&mut game, town).card.id;
    let forest = game
        .put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is there");
    let subtypes = game.effective_subtypes(permanent);
    assert!(
        subtypes.contains(crate::card::Subtype::named("Town")),
        "it is a Town"
    );
    assert!(
        !["Forest", "Island", "Swamp", "Mountain", "Plains"]
            .iter()
            .any(|basic| subtypes.contains(crate::card::Subtype::named(basic))),
        "and nothing else: a card hunting basic land types passes it by",
    );

    let colors = |source: GameObjectId, game: &Game| {
        let mut colors = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateManaAbility {
                    source: from,
                    color,
                    ..
                } if from == source => Some(color),
                _ => None,
            })
            .collect::<Vec<_>>();
        colors.sort_by_key(|color| format!("{color:?}"));
        colors
    };
    assert_eq!(
        colors(id, &game).len(),
        6,
        "the two printed abilities and no third one from the type",
    );
    assert_eq!(
        colors(forest, &game),
        vec![ManaColor::Green],
        "which is what a type that does mean something looks like",
    );
}
