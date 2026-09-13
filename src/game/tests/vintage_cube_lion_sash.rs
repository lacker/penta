//! Lion Sash: graveyard hate that grows, and an Equipment that hands what it
//! ate to whatever it is strapped to.

use super::*;

/// Player One with a Sash on the battlefield and `graveyard` in Player Two's
/// graveyard.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1].graveyard.clear();
    for definition in graveyard {
        let card = game
            .build_zone(PlayerId::Two, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[1].graveyard.push(card);
    }
    let sash = game
        .put_onto_battlefield(PlayerId::One, cards::LION_SASH)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sash)
    {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    (game, sash)
}

fn resolve(game: &mut Game) {
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

/// Activates the exile ability at the graveyard card of `definition`.
fn eat(game: &mut Game, sash: GameObjectId, definition: CardDefinitionId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    let target = game.players[1]
        .graveyard
        .iter()
        .find(|card| card.definition == definition)
        .expect("it is in the graveyard")
        .id;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == sash
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Card(target)))
            }
            _ => false,
        })
        .expect("a card in a graveyard is a legal target");
    game.apply(PlayerId::One, action).expect("it activates");
    resolve(game);
}

fn counters(game: &Game, sash: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == sash)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::PlusOnePlusOne)
        })
}

/// A permanent card feeds it a counter.
#[test]
fn eating_a_permanent_card_grows_it() {
    let (mut game, sash) = staged(&[cards::GRIZZLY_BEARS]);
    eat(&mut game, sash, cards::GRIZZLY_BEARS);

    assert!(
        game.players[1].graveyard.is_empty(),
        "the card left the graveyard",
    );
    assert_eq!(counters(&game, sash), 1);
}

/// An instant does not: the card is still exiled, and the Sash stays a 1/1.
#[test]
fn eating_a_spell_card_exiles_it_without_growing() {
    let (mut game, sash) = staged(&[cards::LIGHTNING_BOLT]);
    eat(&mut game, sash, cards::LIGHTNING_BOLT);

    assert!(
        game.players[1].graveyard.is_empty(),
        "an instant is exiled all the same",
    );
    assert_eq!(counters(&game, sash), 0, "but pays no counter");
}

/// A land is a permanent card, which is the half of the rule that is easy to
/// get wrong.
#[test]
fn a_land_is_a_permanent_card() {
    let (mut game, sash) = staged(&[cards::MOUNTAIN]);
    eat(&mut game, sash, cards::MOUNTAIN);

    assert_eq!(counters(&game, sash), 1);
}

/// Reconfigured onto a creature, it hands over every counter it has eaten.
#[test]
fn the_equipped_creature_gets_the_counters() {
    let (mut game, sash) = staged(&[cards::GRIZZLY_BEARS, cards::SERRA_ANGEL]);
    eat(&mut game, sash, cards::GRIZZLY_BEARS);
    eat(&mut game, sash, cards::SERRA_ANGEL);
    assert_eq!(counters(&game, sash), 2);

    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let reconfigure = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == sash
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("two mana straps it on");
    game.apply(PlayerId::One, reconfigure)
        .expect("it activates");
    resolve(&mut game);

    let equipped = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("the Bears is there");
    assert_eq!(game.power(equipped), Some(4), "2/2 plus two counters");
    assert_eq!(game.toughness(equipped), Some(4));
}

/// The Sash itself, once it is on the battlefield.
fn sash_permanent(game: &Game, sash: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == sash)
        .expect("the Sash is there")
}

/// Reconfigure's ruling: "Attaching an Equipment with reconfigure to a
/// creature causes that Equipment to stop being a creature until it becomes
/// unattached. It also loses any creature subtypes it had." The Sash is a
/// Cat while it stands alone and an Equipment while it is worn.
#[test]
fn a_worn_sash_is_no_longer_a_cat() {
    let (mut game, sash) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    let subtypes = |game: &Game| {
        let mut subtypes = game
            .effective_subtypes(sash_permanent(game, sash))
            .iter()
            .map(crate::card::Subtype::name)
            .collect::<Vec<_>>();
        subtypes.sort_unstable();
        subtypes
    };
    let is_a_creature = |game: &Game| {
        game.permanent_types(sash_permanent(game, sash))
            .is_some_and(CardTypeSet::is_creature)
    };
    assert!(is_a_creature(&game), "on its own it is a creature");
    assert_eq!(
        subtypes(&game),
        vec!["Cat", "Equipment"],
        "an Equipment Cat while it stands alone",
    );

    let reconfigure = |game: &Game, host: Option<GameObjectId>| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    *source == sash
                        && host.is_none_or(|host| {
                            targets.iter().any(|selection| {
                                selection.targets().contains(&Target::Permanent(host))
                            })
                        })
                        && host.is_some()
                            == targets
                                .iter()
                                .any(|selection| !selection.targets().is_empty())
                }
                _ => false,
            })
    };
    let attach = reconfigure(&game, Some(bears)).expect("two mana straps it on");
    game.apply(PlayerId::One, attach).expect("it activates");
    resolve(&mut game);

    assert!(!is_a_creature(&game), "worn, it is not a creature");
    assert_eq!(
        subtypes(&game),
        vec!["Equipment"],
        "and the Cat went with the creature type",
    );

    let unattach = reconfigure(&game, None).expect("the other half of reconfigure");
    game.apply(PlayerId::One, unattach).expect("it activates");
    resolve(&mut game);

    assert!(is_a_creature(&game), "taken off, it is a creature again");
    assert_eq!(subtypes(&game), vec!["Cat", "Equipment"], "and a Cat again");
}

/// "Reconfigure {cost} means '[Cost]: Attach this permanent to another
/// target creature you control. Activate only as a sorcery.'" Three words
/// there do work nothing above tests: *another* (an Equipment creature can
/// never be attached to itself), *you control*, and the sorcery timing.
#[test]
fn reconfigure_names_another_creature_of_yours_at_sorcery_speed() {
    let (mut game, sash) = staged(&[]);
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let attaches_to = |game: &Game, wanted: GameObjectId| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    source == sash
                        && targets.iter().any(|selection| {
                            selection.targets().contains(&Target::Permanent(wanted))
                        })
                }
                _ => false,
            })
    };

    assert!(attaches_to(&game, mine), "your own bear may wear it");
    assert!(
        !attaches_to(&game, sash),
        "and it is never strapped to itself, Cat though it is",
    );
    assert!(
        !attaches_to(&game, theirs),
        "nor to an Angel across the table",
    );

    // "Activate only as a sorcery": their turn is not a window for it.
    game.active_player = PlayerId::Two;
    game.turns_started = [5, 6];
    assert!(
        !attaches_to(&game, mine),
        "and reconfigure waits for a turn of yours",
    );
}
