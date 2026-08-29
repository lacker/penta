//! Urza's Saga: a land that costs nothing, taps for a turn, spends the next
//! two making Constructs, and fetches on its way out.

use super::*;

/// The Saga on the battlefield under Player One, with `library` buried at
/// the bottom of their library.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    // The named cards go to the bottom, since the Saga takes two more draw
    // steps to finish reading and anything on top would simply be drawn.
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    for index in 0..6 {
        game.players[0]
            .library
            .push(card(114_000 + index, cards::ISLAND, PlayerId::One));
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let saga = game
        .put_onto_battlefield(PlayerId::One, cards::URZA_S_SAGA)
        .expect("cataloged");
    settle(&mut game);
    (game, saga)
}

fn settle(game: &mut Game) {
    for _ in 0..40 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
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

/// Carries the turn round to Player One's next precombat main phase, which
/// is where the next lore counter goes on.
fn next_turn(game: &mut Game) {
    game.advance_step();
    settle(game);
    for _ in 0..64 {
        if game.step == Step::PrecombatMain && game.active_player == PlayerId::One {
            break;
        }
        game.advance_step();
        settle(game);
    }
}

fn lore(game: &Game, saga: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == saga)
        .map_or(0, |permanent| permanent.counters(CounterKind::Lore))
}

fn alive(game: &Game, saga: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == saga)
}

fn constructs(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| game.effective_subtypes(permanent).contains(&"Construct"))
        .collect()
}

/// It is a land and an enchantment at once, and it arrives reading its
/// first chapter.
#[test]
fn it_enters_as_a_land_that_taps_for_one() {
    let (mut game, saga) = staged(&[]);

    assert_eq!(lore(&game, saga), 1);
    let types = game
        .permanent_types(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == saga)
                .expect("it is there"),
        )
        .expect("it has card types");
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Enchantment));

    let add = Action::ActivateManaAbility {
        source: saga,
        ability: mana_ability_for(&game, saga, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, add)
        .expect("chapter I granted it");

    assert_eq!(game.players[0].mana_pool.colorless, 1);
}

/// The second chapter adds the Construct ability, and the token it makes
/// counts every artifact you control -- itself included.
#[test]
fn the_second_chapter_makes_constructs() {
    let (mut game, saga) = staged(&[]);
    next_turn(&mut game);
    assert_eq!(lore(&game, saga), 2, "the second chapter has been read");

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let make = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == saga))
        .expect("two mana and the tap makes a Construct");
    game.apply(PlayerId::One, make).expect("it activates");
    settle(&mut game);

    let constructed = constructs(&game);
    assert_eq!(constructed.len(), 1, "one Construct");
    assert_eq!(
        game.power(constructed[0]),
        Some(1),
        "a 0/0 that counts itself is a 1/1",
    );
    assert_eq!(game.toughness(constructed[0]), Some(1));
}

/// A second artifact makes every Construct bigger, live.
#[test]
fn the_constructs_grow_with_the_board() {
    let (mut game, saga) = staged(&[]);
    next_turn(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let make = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == saga))
        .expect("the Construct ability is offered");
    game.apply(PlayerId::One, make).expect("it activates");
    settle(&mut game);
    assert_eq!(game.power(constructs(&game)[0]), Some(1));

    game.put_onto_battlefield(PlayerId::One, cards::HOWLING_MINE)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        game.power(constructs(&game)[0]),
        Some(2),
        "the Mine counts too",
    );
}

/// The third chapter fetches a cheap artifact onto the battlefield, and the
/// Saga sacrifices itself afterwards.
#[test]
fn the_third_chapter_fetches_and_ends_it() {
    let (mut game, saga) = staged(&[cards::BLACK_LOTUS]);
    next_turn(&mut game);
    next_turn(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::BLACK_LOTUS),
        "the Lotus came out of the library",
    );
    assert!(
        !game.players[0]
            .library
            .iter()
            .any(|card| card.definition == cards::BLACK_LOTUS),
        "and left the library",
    );
    assert!(
        !alive(&game, saga),
        "and the Saga is gone after chapter III"
    );
}

/// "Mana cost {0} or {1}": a two-drop is not a legal choice.
#[test]
fn an_expensive_artifact_is_not_fetchable() {
    let (mut game, saga) = staged(&[cards::HOWLING_MINE]);
    next_turn(&mut game);
    next_turn(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::HOWLING_MINE),
        "a two-mana artifact is out of range",
    );
    assert!(
        game.players[0]
            .library
            .iter()
            .any(|card| card.definition == cards::HOWLING_MINE),
        "so it stayed put",
    );
    assert!(!alive(&game, saga), "the Saga still sacrifices itself");
}
