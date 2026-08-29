//! Fabled Passage: Evolving Wilds that stops costing you a turn once you
//! have lands enough for the fourth one to matter.

use super::*;

/// The Passage on the battlefield with `others` further lands beside it, and
/// a Mountain waiting in the library.
fn staged(others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.players[0]
        .library
        .push(card(98_000, cards::MOUNTAIN, PlayerId::One));
    let passage = game
        .put_onto_battlefield(PlayerId::One, cards::FABLED_PASSAGE)
        .expect("cataloged");
    for _ in 0..others {
        game.put_onto_battlefield(PlayerId::One, cards::FOREST)
            .expect("cataloged");
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, passage)
}

/// Cracks the Passage and answers the search.
fn crack(game: &mut Game, passage: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == passage),
        )
        .expect("the Passage can be cracked");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
    game.check_state_based_actions();
}

fn fetched(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .expect("a Mountain arrived")
}

/// Behind on lands it is an ordinary Evolving Wilds: the basic arrives
/// tapped and stays that way.
#[test]
fn a_third_land_leaves_it_tapped() {
    let (mut game, passage) = staged(2);

    crack(&mut game, passage);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == passage),
        "the Passage sacrificed itself",
    );
    assert!(fetched(&game).tapped, "two lands and the new one is three");
}

/// The land it just found is the fourth, so it untaps again.
#[test]
fn the_fourth_land_untaps_it() {
    let (mut game, passage) = staged(3);

    crack(&mut game, passage);

    assert!(
        !fetched(&game).tapped,
        "three lands beside it and the arrival makes four",
    );
}

/// The Passage is not one of the four: it paid for the search with itself,
/// so a board of three lands counting it comes up short.
#[test]
fn the_passage_does_not_count_itself() {
    let (mut game, passage) = staged(2);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition != cards::MOUNTAIN)
            .count(),
        3,
        "three lands on the battlefield before it is cracked",
    );

    crack(&mut game, passage);

    assert!(fetched(&game).tapped, "and only three afterwards");
}

/// "Untap that land", not "untap a land": the one this search found.
#[test]
fn it_untaps_only_what_it_fetched() {
    let (mut game, passage) = staged(3);
    for permanent in &mut game.battlefield {
        permanent.tapped = permanent.card.definition == cards::FOREST;
    }

    crack(&mut game, passage);

    assert!(!fetched(&game).tapped, "the Mountain came back up");
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::FOREST)
            .all(|permanent| permanent.tapped),
        "and the Forests it did not find stayed down",
    );
}

/// A library with no basic in it: the search finds nothing, and the clause
/// that would untap it has nothing to name.
#[test]
fn an_empty_search_does_nothing() {
    let (mut game, passage) = staged(3);
    game.players[0].library.clear();
    let lands = game.battlefield.len();

    crack(&mut game, passage);

    assert_eq!(
        game.battlefield.len(),
        lands - 1,
        "only the Passage left the battlefield",
    );
}

/// "A basic land card": a Bayou is a Swamp and a Forest and still not basic,
/// so the search passes it over.
#[test]
fn only_a_basic_is_worth_finding() {
    let (mut game, passage) = staged(3);
    game.players[0]
        .library
        .push(card(98_500, cards::BAYOU, PlayerId::One));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == passage),
        )
        .expect("the Passage can be cracked");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let offered = game
        .pending_decisions
        .first()
        .expect("the search asks")
        .observation
        .options
        .iter()
        .filter_map(|option| match option.card {
            Some((_, ObjectCharacteristics::Card { definition, .. })) => Some(definition),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![cards::MOUNTAIN],
        "the Mountain is basic and the Bayou only has the types",
    );
}

/// The untap is not bookkeeping: the land it found pays for something the
/// same turn, which is the whole difference between this and an Evolving
/// Wilds.
#[test]
fn the_land_it_untapped_pays_at_once() {
    let (mut game, passage) = staged(3);
    game.empty_mana_pools();

    crack(&mut game, passage);

    let mountain = fetched(&game).card.id;
    let red = Action::ActivateManaAbility {
        source: mountain,
        ability: mana_ability_for(&game, mountain, ManaColor::Red),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, red)
        .expect("an untapped Mountain taps for red");

    assert_eq!(game.players[0].mana_pool.red, 1);
}
