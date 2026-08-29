//! Boseiju, Who Endures: a Forest that answers the one artifact the deck
//! could not otherwise beat, and costs nothing to play when it does not have
//! to.

use super::*;

/// Boseiju in hand, `theirs` on the other side of the table, and `mine`
/// under Player One.
fn staged(
    theirs: &[CardDefinitionId],
    mine: &[CardDefinitionId],
) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].library.clear();
    let mut ids = Vec::new();
    for definition in theirs {
        ids.push(
            game.put_onto_battlefield(PlayerId::Two, *definition)
                .expect("cataloged"),
        );
    }
    for definition in mine {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    drain_pending(&mut game);
    let boseiju = game
        .build_zone(PlayerId::One, &[cards::BOSEIJU_WHO_ENDURES])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = boseiju.id;
    game.players[0].hand.push(boseiju);
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id, ids)
}

fn settle(game: &mut Game, search: bool) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // A "may" is a two-option question rather than an empty answer:
            // declining is a choice with a label on it. Every other decision
            // in the chain -- which land the search takes -- follows the
            // same flag.
            let wanted = if search { "Do it" } else { "Decline" };
            let options = match decision
                .options
                .iter()
                .find(|option| option.label == wanted)
            {
                Some(option) => vec![option.id],
                None => decision
                    .options
                    .iter()
                    .take(if search {
                        decision.minimum.max(1)
                    } else {
                        decision.minimum
                    })
                    .map(|option| option.id)
                    .collect(),
            };
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

fn channels(game: &Game, boseiju: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == boseiju),
        )
        .collect()
}

fn channel_at(game: &mut Game, boseiju: GameObjectId, target: GameObjectId, search: bool) {
    let action = channels(game, boseiju)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility { targets, .. } => targets
                .iter()
                .any(|selection| selection.targets().contains(&Target::Permanent(target))),
            _ => false,
        })
        .expect("the channel is offered at that permanent");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game, search);
}

fn channel_targets(game: &Game, boseiju: GameObjectId) -> Vec<GameObjectId> {
    channels(game, boseiju)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { targets, .. } => targets
                .into_iter()
                .flat_map(|selection| selection.targets().to_vec())
                .find_map(|target| match target {
                    Target::Permanent(id) => Some(id),
                    _ => None,
                }),
            _ => None,
        })
        .collect()
}

fn on_battlefield(game: &Game, permanent: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|candidate| candidate.card.id == permanent)
}

/// Played as a land it taps for green, which is most of what it ever does.
#[test]
fn it_taps_for_green() {
    let (mut game, _boseiju, _) = staged(&[], &[]);
    let land = game
        .put_onto_battlefield(PlayerId::One, cards::BOSEIJU_WHO_ENDURES)
        .expect("cataloged");

    let action = Action::ActivateManaAbility {
        source: land,
        ability: mana_ability_for(&game, land, ManaColor::Green),
        color: ManaColor::Green,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, action).expect("it taps");

    assert_eq!(game.players[0].mana_pool.green, 1);
}

/// Two mana and the card itself destroys an artifact across the table.
#[test]
fn channelling_it_destroys_an_artifact() {
    let (mut game, boseiju, theirs) = staged(&[cards::SOL_RING], &[]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    channel_at(&mut game, boseiju, theirs[0], false);

    assert!(!on_battlefield(&game, theirs[0]), "the Sol Ring is gone");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::BOSEIJU_WHO_ENDURES),
        "and Boseiju was discarded to pay for it",
    );
}

/// A basic land is not a legal target, and neither is a creature; a nonbasic
/// land is.
#[test]
fn it_names_artifacts_enchantments_and_nonbasic_lands() {
    let (mut game, boseiju, theirs) = staged(
        &[
            cards::MOUNTAIN,
            cards::GRIZZLY_BEARS,
            cards::MISHRA_S_FACTORY,
            cards::CONTROL_MAGIC,
            cards::SOL_RING,
        ],
        &[],
    );
    let (mountain, bears, factory) = (theirs[0], theirs[1], theirs[2]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let offered = channel_targets(&game, boseiju);
    assert!(!offered.is_empty(), "the channel is payable");

    assert!(!offered.contains(&mountain), "a basic is safe");
    assert!(!offered.contains(&bears), "and so is a creature");
    assert!(offered.contains(&factory), "a nonbasic land is not");
    assert!(offered.contains(&theirs[4]), "nor is an artifact");
}

/// It only ever answers the other player's permanents.
#[test]
fn it_never_names_your_own() {
    let (mut game, boseiju, _) = staged(&[cards::SOL_RING], &[cards::JAYEMDAE_TOME]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let mine = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::JAYEMDAE_TOME)
        .expect("mine is out")
        .card
        .id;

    assert!(
        !channel_targets(&game, boseiju).contains(&mine),
        "your own artifact is never a target",
    );
}

/// The compensation is theirs to take: a basic land out of their library and
/// onto the battlefield untapped.
#[test]
fn they_may_replace_it_with_a_basic() {
    let (mut game, boseiju, theirs) = staged(&[cards::MISHRA_S_FACTORY], &[]);
    for index in 0..3 {
        game.players[1].library.push(card(
            89_000 + index,
            if index == 0 {
                cards::FOREST
            } else {
                cards::LIGHTNING_BOLT
            },
            PlayerId::Two,
        ));
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    channel_at(&mut game, boseiju, theirs[0], true);

    assert!(!on_battlefield(&game, theirs[0]), "the Factory is gone");
    let replacement = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::FOREST)
        .expect("they found a Forest");
    assert_eq!(replacement.controller, PlayerId::Two, "it is theirs");
    assert!(!replacement.tapped, "and it arrives untapped");
}

/// Declining the search is the other half of "may": the permanent is still
/// destroyed and nothing comes out of the library.
#[test]
fn they_may_decline_the_search() {
    let (mut game, boseiju, theirs) = staged(&[cards::MISHRA_S_FACTORY], &[]);
    game.players[1]
        .library
        .push(card(89_100, cards::FOREST, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    channel_at(&mut game, boseiju, theirs[0], false);

    assert!(!on_battlefield(&game, theirs[0]));
    assert_eq!(game.players[1].library.len(), 1, "the Forest stayed put");
}

/// The legends discount: each one takes a generic off, and the {G} is what
/// cannot be reduced away.
#[test]
fn every_legend_takes_a_generic_off() {
    let (mut game, boseiju, theirs) = staged(&[cards::SOL_RING], &[cards::JACE_THE_MIND_SCULPTOR]);
    // One green alone does not pay {1}{G}.
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    assert!(
        channels(&game, boseiju).is_empty(),
        "a planeswalker is not a legendary creature",
    );

    game.put_onto_battlefield(PlayerId::One, cards::SHIVAN_DRAGON)
        .expect("cataloged");
    assert!(
        channels(&game, boseiju).is_empty(),
        "and a nonlegendary creature is not either",
    );

    let legend = game
        .put_onto_battlefield(PlayerId::One, cards::GRISELBRAND)
        .expect("cataloged");
    drain_pending(&mut game);
    assert!(
        !channels(&game, boseiju).is_empty(),
        "one legendary creature makes the green alone enough",
    );

    channel_at(&mut game, boseiju, theirs[0], false);
    assert!(!on_battlefield(&game, theirs[0]));
    let _ = legend;
}
