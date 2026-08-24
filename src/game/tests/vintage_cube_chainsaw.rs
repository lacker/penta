//! Chainsaw: it shoots something on the way in, and then revs for the rest
//! of the game every time the board clears.

use super::*;

/// The Chainsaw in hand, with `creatures` on the battlefield under player
/// Two and enough mana to cast and equip.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut ids = Vec::new();
    for definition in theirs {
        ids.push(
            game.put_onto_battlefield(PlayerId::Two, *definition)
                .expect("cataloged"),
        );
    }
    let saw = game
        .build_zone(PlayerId::One, &[cards::CHAINSAW])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let saw_id = saw.id;
    game.players[0].hand.push(saw);
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 8);
    (game, saw_id, ids)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
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

/// Casts it, pointing the enter trigger at `wanted` if there is one.
fn cast(game: &mut Game, saw: GameObjectId, wanted: Option<GameObjectId>) -> GameObjectId {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == saw))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it is cast");
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options: Vec<_> = decision
                .options
                .iter()
                .filter(|option| {
                    wanted.is_some_and(|wanted| {
                        option.card.is_some_and(|(object, _)| object == wanted)
                    })
                })
                .map(|option| option.id)
                .take(1)
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
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::CHAINSAW))
        .expect("the Equipment arrived")
        .card
        .id
}

fn rev_counters(game: &Game, saw: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == saw)
        .expect("it is on the battlefield")
        .counters(CounterKind::named("rev"))
}

/// Kills every creature player Two controls at once.
fn wipe(game: &mut Game) {
    let doomed = game
        .battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::Two
                && game
                    .permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Creature))
        })
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    game.move_permanents_to_graveyard(&doomed);
    settle(game);
}

/// It shoots on the way in, and three damage kills a 2/2.
#[test]
fn it_shoots_something_as_it_enters() {
    let (mut game, saw, theirs) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = theirs[0];

    cast(&mut game, saw, Some(bears));

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "three damage killed the 2/2",
    );
}

/// "Up to one": with nothing worth shooting it still arrives.
#[test]
fn it_arrives_with_nothing_to_shoot() {
    let (mut game, saw, _) = staged(&[]);

    let equipment = cast(&mut game, saw, None);

    assert_eq!(rev_counters(&game, equipment), 0, "and nothing has died");
}

/// The creature it kills on arrival is itself one or more creatures dying,
/// so the Chainsaw revs off its own shot.
#[test]
fn its_own_kill_revs_it() {
    let (mut game, saw, theirs) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = theirs[0];

    let equipment = cast(&mut game, saw, Some(bears));

    assert_eq!(rev_counters(&game, equipment), 1);
}

/// Three creatures dying at once is one counter, not three.
#[test]
fn a_board_wipe_is_one_counter() {
    let (mut game, saw, _) = staged(&[
        cards::GRIZZLY_BEARS,
        cards::SAVANNAH_LIONS,
        cards::SERRA_ANGEL,
    ]);
    let equipment = cast(&mut game, saw, None);
    assert_eq!(rev_counters(&game, equipment), 0);

    wipe(&mut game);

    assert_eq!(
        rev_counters(&game, equipment),
        1,
        "one trigger for the whole batch",
    );
}

/// Deaths at different times are different batches.
#[test]
fn separate_deaths_are_separate_counters() {
    let (mut game, saw, theirs) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let equipment = cast(&mut game, saw, None);

    game.move_permanents_to_graveyard(&[theirs[0]]);
    settle(&mut game);
    assert_eq!(rev_counters(&game, equipment), 1);

    game.move_permanents_to_graveyard(&[theirs[1]]);
    settle(&mut game);
    assert_eq!(rev_counters(&game, equipment), 2);
}

/// The counters are the equipped creature's power bonus, and they only
/// touch power.
#[test]
fn the_counters_are_the_equipped_creatures_power() {
    let (mut game, saw, _theirs) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let equipment = cast(&mut game, saw, None);
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    wipe(&mut game);
    assert_eq!(rev_counters(&game, equipment), 1);

    let equip = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == equipment
                        && targets
                            .iter()
                            .any(|slot| slot.targets().contains(&Target::Permanent(mine)))
            )
        })
        .expect("equip is activatable");
    game.apply(PlayerId::One, equip).expect("it equips");
    settle(&mut game);

    let host = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mine)
        .expect("the equipped creature is there");
    assert_eq!(game.power(host), Some(3), "2 plus one rev counter");
    assert_eq!(game.toughness(host), Some(2), "and toughness is untouched");
}
