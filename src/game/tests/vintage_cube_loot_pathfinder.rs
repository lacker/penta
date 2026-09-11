//! Loot, the Pathfinder: a hasty double striker whose three exhaust
//! abilities each fire once and never again.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..6 {
        game.players[0]
            .library
            .push(card(100_000 + index, cards::GRIZZLY_BEARS, PlayerId::One));
    }
    let loot = game
        .put_onto_battlefield(PlayerId::One, cards::LOOT_THE_PATHFINDER)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, loot)
}

/// The activations Loot is offering right now, by the mana each one costs.
fn offered(game: &Game, loot: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, .. } | Action::ActivateManaAbility { source, .. }
                    if *source == loot
            )
        })
        .collect()
}

fn resolve(game: &mut Game) {
    for _ in 0..12 {
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
}

/// The keywords are all three.
#[test]
fn it_has_its_three_keywords() {
    let (game, loot) = staged();
    let beast = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == loot)
        .expect("it is there");

    assert!(game.permanent_has_executable_keyword(beast, KeywordAbility::DoubleStrike));
    assert!(game.permanent_has_executable_keyword(beast, KeywordAbility::Vigilance));
    assert!(game.permanent_has_executable_keyword(beast, KeywordAbility::Haste));
}

/// The blue exhaust draws three, once. Untapping does not give it back.
#[test]
fn the_blue_exhaust_fires_once_and_never_again() {
    let (mut game, loot) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    let draw = offered(&game, loot)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("a blue mana buys the draw");
    game.apply(PlayerId::One, draw).expect("it activates");
    resolve(&mut game);
    assert_eq!(game.players[0].hand.len(), 3);

    // Untap it and try again: the ability is spent for good.
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == loot)
    {
        permanent.tapped = false;
    }
    game.cleanup();
    game.turns_started = [6, 6];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    assert!(
        !offered(&game, loot)
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { .. })),
        "exhaust is spent for as long as the creature is there",
    );
}

/// Spending one leaves the others: they are three abilities, not one.
#[test]
fn the_other_exhausts_are_untouched() {
    let (mut game, loot) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let draw = offered(&game, loot)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("a blue mana buys the draw");
    game.apply(PlayerId::One, draw).expect("it activates");
    resolve(&mut game);

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == loot)
    {
        permanent.tapped = false;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        offered(&game, loot)
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { .. })),
        "the red half was never activated",
    );
}

/// The green exhaust is a mana ability: three mana of one colour.
#[test]
fn the_green_exhaust_makes_three_mana() {
    let (mut game, loot) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let mana = offered(&game, loot)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateManaAbility { .. }))
        .expect("a green mana buys the mana ability");
    game.apply(PlayerId::One, mana).expect("it activates");
    resolve(&mut game);

    let pool = &game.players[0].mana_pool;
    assert_eq!(
        pool.white + pool.blue + pool.black + pool.red + pool.green + pool.colorless,
        3,
        "three mana of one colour, and the green that paid is spent",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == loot)
        .unwrap()
        .tapped = false;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    assert!(
        offered(&game, loot)
            .iter()
            .all(|action| !matches!(action, Action::ActivateManaAbility { .. })),
        "untapping cannot repay a once-per-object mana activation"
    );

    game.cleanup();
    game.turns_started = [6, 6];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        100_600,
    )
    .expect("per-object activation history reconstructs");
    assert!(
        offered(&rebuilt, loot)
            .iter()
            .all(|action| !matches!(action, Action::ActivateManaAbility { .. })),
        "turn changes and reconstruction preserve the spent mana ability"
    );
}

/// The red exhaust: three damage to any target, and spent afterwards like
/// the other two.
#[test]
fn the_red_exhaust_burns_once() {
    let (mut game, loot) = staged();
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.priority = PlayerId::One;

    let burn = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == loot
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bears))
            }
            _ => false,
        })
        .expect("a red mana buys the burn, and a creature is any target");
    game.apply(PlayerId::One, burn).expect("it activates");
    resolve(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "three damage is enough for a 2/2",
    );

    // Untapped and with the mana for it, the same half is still spent.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == loot)
        .expect("he is there")
        .tapped = false;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    assert!(
        !offered(&game, loot)
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { .. })),
        "each exhaust ability is activated only once",
    );
}

/// "If an exhaust ability of a permanent is activated, and then that
/// permanent leaves the battlefield and returns, it becomes a new object so
/// its exhaust ability can be activated again." An Ephemerate is exactly
/// that round trip.
#[test]
fn blinking_him_hands_the_exhausts_back() {
    let (mut game, loot) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let draw = offered(&game, loot)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("a blue mana buys the draw");
    game.apply(PlayerId::One, draw).expect("it activates");
    resolve(&mut game);
    let held = game.players[PlayerId::One.index()].hand.len();

    let ephemerate = card(100_500, cards::EPHEMERATE, PlayerId::One);
    let ephemerate_id = ephemerate.id;
    game.players[PlayerId::One.index()].hand.push(ephemerate);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.priority = PlayerId::One;
    let blink = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == ephemerate_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(loot))
            }
            _ => false,
        })
        .expect("a creature you control is what it names");
    game.apply(PlayerId::One, blink).expect("it is cast");
    resolve(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LOOT_THE_PATHFINDER)
        .expect("he came back")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let again = offered(&game, returned)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("a new object has spent nothing");
    game.apply(PlayerId::One, again).expect("it activates");
    resolve(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        held + 3,
        "three more cards, from an ability that had already been spent once",
    );
}
