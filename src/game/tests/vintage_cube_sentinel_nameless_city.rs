//! Sentinel of the Nameless City: a 3/4 that hands you a Map for arriving
//! and another for every attack.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].library.push(card(
        95_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    let sentinel = game
        .put_onto_battlefield(PlayerId::One, cards::SENTINEL_OF_THE_NAMELESS_CITY)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, sentinel)
}

fn maps(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
                && permanent.controller == PlayerId::One
        })
        .count()
}

/// Arriving makes one.
#[test]
fn arriving_makes_a_map() {
    let (game, sentinel) = staged();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sentinel)
        .expect("it is there");

    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(3), Some(4))
    );
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance));
    assert_eq!(maps(&game), 1);
}

/// Attacking makes another, and vigilance means it is still untapped
/// afterwards.
#[test]
fn attacking_makes_another_and_leaves_it_untapped() {
    let (mut game, sentinel) = staged();

    game.step = Step::DeclareAttackers;
    game.declare_attacker(sentinel, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    assert_eq!(maps(&game), 2);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == sentinel)
            .expect("it is there")
            .tapped,
        "vigilance keeps it back",
    );
}

/// The Map it makes is the real one: a mana, a tap, and itself for an
/// explore.
#[test]
fn the_map_explores() {
    let (mut game, sentinel) = staged();
    let map = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
        })
        .expect("the Map is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.step = Step::PrecombatMain;

    let explore = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(sentinel))
            }
            _ => false,
        })
        .expect("one mana and the Map itself buys an explore");
    game.apply(PlayerId::One, explore).expect("it activates");
    drain_pending(&mut game);

    let explored = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sentinel)
        .expect("it is there");
    assert_eq!(
        explored.counters(CounterKind::PlusOnePlusOne),
        1,
        "a nonland on top means a counter",
    );
    assert_eq!(maps(&game), 0, "the Map is spent");
}

/// Activates the Map at `creature` and answers whatever the explore asks,
/// burying the revealed card when `bury` is set.
fn explore_with_map(game: &mut Game, creature: GameObjectId, bury: bool) {
    let map = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
        })
        .expect("the Map is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let explore = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(creature))
            }
            _ => false,
        })
        .expect("one mana and the Map itself buys an explore");
    game.apply(PlayerId::One, explore).expect("it activates");
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let wanted = if bury { "Graveyard" } else { "Top of library" };
            let option = decision
                .options
                .iter()
                .find(|option| option.label == wanted)
                .expect("the explore offers both halves")
                .id;
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option],
                },
            )
            .expect("the choice is legal");
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

/// The other half of explore: a land on top goes to your hand, and nothing
/// grows.
#[test]
fn a_land_on_top_goes_to_hand_instead() {
    let (mut game, sentinel) = staged();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()]
        .library
        .push(card(95_100, cards::MOUNTAIN, PlayerId::One));
    game.players[PlayerId::One.index()].hand.clear();

    explore_with_map(&mut game, sentinel, false);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::MOUNTAIN],
        "the land went to hand",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == sentinel)
            .expect("it is there")
            .counters(CounterKind::PlusOnePlusOne),
        0,
        "and the land branch grows nothing",
    );
}

/// The nonland branch ends in a choice, and burying is the other answer.
#[test]
fn a_revealed_nonland_may_be_buried() {
    let (mut game, sentinel) = staged();
    game.players[PlayerId::One.index()].graveyard.clear();

    explore_with_map(&mut game, sentinel, true);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::GRIZZLY_BEARS],
        "the Bears were binned rather than left on top",
    );
    assert!(game.players[PlayerId::One.index()].library.is_empty());
}

/// "If no card is revealed, most likely because that player's library is
/// empty, the exploring creature receives a +1/+1 counter."
#[test]
fn an_empty_library_still_grows_it() {
    let (mut game, sentinel) = staged();
    game.players[PlayerId::One.index()].library.clear();

    explore_with_map(&mut game, sentinel, false);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == sentinel)
            .expect("it is there")
            .counters(CounterKind::PlusOnePlusOne),
        1,
        "nothing to reveal is still a counter",
    );
}

/// "If you reveal a nonland card when a creature explores and leave it on
/// top of your library, then the creature explores again immediately
/// afterwards, you'll reveal the same card again." Two Maps, one card, two
/// counters.
#[test]
fn leaving_the_card_on_top_shows_it_to_the_next_explore() {
    let (mut game, sentinel) = staged();
    // A second Map, so the same card can be looked at twice.
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(sentinel, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);
    assert_eq!(maps(&game), 2, "one for arriving and one for attacking");
    let library = game.players[PlayerId::One.index()].library.len();

    explore_with_map(&mut game, sentinel, false);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library,
        "the Bears stayed where they were",
    );

    explore_with_map(&mut game, sentinel, false);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == sentinel)
            .expect("the Sentinel is there")
            .counters(CounterKind::PlusOnePlusOne),
        2,
        "the same nonland was revealed twice and paid twice",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library,
        "and it is still on top after both looks",
    );
}

/// "Once an ability that causes a creature to explore begins to resolve, no
/// player may take any other actions until it's done. Notably, opponents
/// can't try to remove the exploring creature after you reveal a nonland
/// card but before it receives a counter."
#[test]
fn nobody_acts_between_the_reveal_and_the_counter() {
    let (mut game, sentinel) = staged();
    let map = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
        })
        .expect("the Map is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let explore = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(sentinel))
            }
            _ => false,
        })
        .expect("one mana and the Map itself buys an explore");
    game.apply(PlayerId::One, explore).expect("it activates");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        !game.pending_decisions.is_empty(),
        "the explore is mid-resolution, asking where the card goes",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == sentinel)
            .expect("the Sentinel is there")
            .counters(CounterKind::PlusOnePlusOne),
        1,
        "the counter is already on, which is what they would want to answer",
    );
    // Conceding is always available and is not an action taken in the game
    // (CR 104.3a); everything else would be a response.
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .all(|action| matches!(action, Action::Concede)),
        "and they have no window at all: {:?}",
        game.legal_actions(PlayerId::Two),
    );
}

/// "Activate only as a sorcery": the Map waits for your own main phase with
/// an empty stack, which is what keeps it from being a combat trick.
#[test]
fn the_map_is_a_sorcery_speed_ability() {
    let (mut game, sentinel) = staged();
    let map = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
        })
        .expect("the Map is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let offered = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == map),
        )
    };

    game.step = Step::DeclareBlockers;
    assert!(!offered(&game), "not in the middle of your own combat");

    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(!offered(&game), "and not on their main phase either");

    game.active_player = PlayerId::One;
    assert!(
        offered(&game),
        "your own main phase with an empty stack is where it lives",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == sentinel),
        "with the creature it would point at still standing",
    );
}

/// "Target creature you control": their creature is no target for your Map,
/// however much you would like to grow it.
#[test]
fn the_map_explores_only_your_own_creatures() {
    let (mut game, sentinel) = staged();
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let map = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Map"))
        })
        .expect("the Map is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let named = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == map => Some(
                targets
                    .iter()
                    .flat_map(crate::casting::TargetSelection::targets)
                    .filter_map(|target| match target {
                        Target::Permanent(id) => Some(*id),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();

    assert_eq!(
        named,
        vec![sentinel],
        "the Sentinel is the only creature you control",
    );
    assert!(
        !named.contains(&theirs),
        "and theirs is not on the list at all",
    );
}
