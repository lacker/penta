//! Thundertrap Trainer: two mana to dig four cards deep, or six for two
//! bodies and two digs.

use super::*;

/// Player One holding the Trainer, with `library` stacked so the last entry
/// is on top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let trainer = game
        .build_zone(PlayerId::One, &[cards::THUNDERTRAP_TRAINER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = trainer.id;
    game.players[0].hand.push(trainer);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id)
}

/// Answers whatever is asked, taking the card whose definition is `wanted`
/// when it is offered and nothing otherwise.
fn settle_taking(game: &mut Game, wanted: Option<CardDefinitionId>) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options: Vec<_> = decision
                .options
                .iter()
                .filter(|option| match (wanted, option.card) {
                    (Some(wanted), Some((_, ObjectCharacteristics::Card { definition, .. }))) => {
                        definition == wanted
                    }
                    _ => false,
                })
                .map(|option| option.id)
                .take(1)
                .collect();
            let options = if options.len() < decision.minimum {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum)
                    .collect()
            } else {
                options
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

fn cast_for(game: &mut Game, card: GameObjectId, offspring: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } => *id == card && choices.costs().additional().is_empty() != offspring,
            _ => false,
        })
        .expect("that way of casting him is on offer");
    game.apply(PlayerId::One, action).expect("he is cast");
}

fn trainers(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| game.effective_subtypes(permanent).contains(&"Otter"))
        .collect()
}

/// Cast for two, he digs four deep and takes the noncreature nonland card.
#[test]
fn he_digs_four_for_a_spell() {
    let (mut game, trainer) = staged(&[
        cards::SERRA_ANGEL,
        cards::MOUNTAIN,
        cards::GRIZZLY_BEARS,
        cards::LIGHTNING_BOLT,
        cards::FOREST,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    cast_for(&mut game, trainer, false);
    settle_taking(&mut game, Some(cards::LIGHTNING_BOLT));

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "the instant among the four came to hand",
    );
    assert_eq!(
        game.players[0].library.len(),
        4,
        "the three he passed over went to the bottom, and one was never seen",
    );
    assert_eq!(trainers(&game).len(), 1, "and no token without offspring");
}

/// A creature among the four is not a legal choice.
#[test]
fn a_creature_is_not_what_he_looks_for() {
    let (mut game, trainer) = staged(&[
        cards::MOUNTAIN,
        cards::GRIZZLY_BEARS,
        cards::SERRA_ANGEL,
        cards::FOREST,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    cast_for(&mut game, trainer, false);
    settle_taking(&mut game, Some(cards::GRIZZLY_BEARS));

    assert!(
        game.players[0].hand.is_empty(),
        "nothing among them was a noncreature nonland card",
    );
}

/// Paid for with offspring, he brings a 1/1 copy of himself -- and the copy
/// digs too.
#[test]
fn offspring_makes_a_one_one_copy_that_digs() {
    let (mut game, trainer) = staged(&[
        cards::SERRA_ANGEL,
        cards::MOUNTAIN,
        cards::LIGHTNING_BOLT,
        cards::GRIZZLY_BEARS,
        cards::FOREST,
        cards::MOUNTAIN,
        cards::ISLAND,
        cards::SWAMP,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    cast_for(&mut game, trainer, true);
    settle_taking(&mut game, Some(cards::LIGHTNING_BOLT));

    let bodies = trainers(&game);
    assert_eq!(bodies.len(), 2, "the token copy arrived");
    let token = bodies
        .iter()
        .find(|permanent| game.power(permanent) == Some(1) && game.toughness(permanent) == Some(1))
        .expect("the copy is a 1/1");
    assert!(
        token.card.definition.is_token(),
        "and it is a token rather than the card",
    );
    assert!(
        bodies
            .iter()
            .any(|permanent| game.toughness(permanent) == Some(2)),
        "while the original is still a 1/2",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and the digs happened",
    );
}

/// Without paying it, nothing is copied even though the trigger is printed.
#[test]
fn no_offspring_no_token() {
    let (mut game, trainer) = staged(&[cards::MOUNTAIN, cards::FOREST, cards::ISLAND]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    cast_for(&mut game, trainer, false);
    settle_taking(&mut game, None);

    assert_eq!(trainers(&game).len(), 1);
}

/// "If the spell resolves but the creature with offspring leaves the
/// battlefield before the offspring ability resolves, you'll still create a
/// token copy of it."
#[test]
fn the_token_still_arrives_when_he_does_not_survive_his_own_trigger() {
    let (mut game, trainer) = staged(&[
        cards::MOUNTAIN,
        cards::FOREST,
        cards::ISLAND,
        cards::SWAMP,
        cards::MOUNTAIN,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    cast_for(&mut game, trainer, true);

    // Far enough for him to resolve onto the battlefield and no further:
    // the offspring trigger is what is still waiting.
    let body = loop {
        if let Some(permanent) = trainers(&game).first() {
            break permanent.card.id;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the spell is on the stack");
    };
    game.destroy_permanent(body);
    game.check_state_based_actions();
    settle_taking(&mut game, None);

    let bodies = trainers(&game);
    assert_eq!(bodies.len(), 1, "the token was made all the same");
    assert!(
        bodies[0].card.definition.is_token(),
        "and what is left is the copy rather than the card",
    );
    assert_eq!(
        (game.power(bodies[0]), game.toughness(bodies[0])),
        (Some(1), Some(1)),
        "a 1/1 of him",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::THUNDERTRAP_TRAINER),
        "while he himself is in the graveyard",
    );
}

/// "If the spell is countered, the offspring ability will not trigger, and
/// no token will be created." Paying the six buys nothing at all.
#[test]
fn a_countered_trainer_leaves_no_token_behind() {
    let (mut game, trainer) = staged(&[cards::MOUNTAIN, cards::FOREST, cards::ISLAND]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);
    game.players[1]
        .hand
        .push(card(93_900, cards::COUNTERSPELL, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    cast_for(&mut game, trainer, true);

    game.priority = PlayerId::Two;
    let counter = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(93_900))
        })
        .expect("two blue answers him");
    game.apply(PlayerId::Two, counter).expect("it is cast");
    settle_taking(&mut game, None);

    assert!(trainers(&game).is_empty(), "no body and no copy of one");
    assert!(
        game.players[0].hand.is_empty(),
        "and nothing was dug up either",
    );
}

/// "You can pay an offspring cost only once as you cast a spell with
/// offspring. You can't try to pay it multiple times to get more token
/// copies." A pool deep enough to pay it twice over still offers two prices
/// and no third.
#[test]
fn the_offspring_cost_is_paid_once_or_not_at_all() {
    let (mut game, trainer) = staged(&[cards::MOUNTAIN, cards::FOREST, cards::ISLAND]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 12);

    let mut prices = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } if id == trainer => Some(choices.costs().additional().len()),
            _ => None,
        })
        .collect::<Vec<_>>();
    prices.sort_unstable();
    prices.dedup();

    assert_eq!(
        prices,
        vec![0, 1],
        "offspring is paid exactly once or not at all"
    );
}

/// "The token copies exactly what was printed on the original creature and
/// nothing else, except it's a 1/1. It doesn't copy ... any counters on it."
/// A counter that lands while the offspring trigger waits stays behind.
#[test]
fn the_token_leaves_his_counters_behind() {
    let (mut game, trainer) = staged(&[
        cards::MOUNTAIN,
        cards::FOREST,
        cards::ISLAND,
        cards::SWAMP,
        cards::MOUNTAIN,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    cast_for(&mut game, trainer, true);

    // He is on the battlefield and the offspring trigger has not resolved.
    let body = loop {
        if let Some(permanent) = trainers(&game).first() {
            break permanent.card.id;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the spell is on the stack");
    };
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == body)
        .expect("he is here")
        .set_counters(CounterKind::PlusOnePlusOne, 2);
    settle_taking(&mut game, None);

    let token = trainers(&game)
        .into_iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the copy was made");
    assert_eq!(
        token.counters(CounterKind::PlusOnePlusOne),
        0,
        "the counters were not part of what it copied",
    );
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(1), Some(1)),
        "so it is the 1/1 the ability makes",
    );
    let original = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == body)
        .expect("he is still here");
    assert_eq!(
        (game.power(original), game.toughness(original)),
        (Some(3), Some(4)),
        "while he kept his own",
    );
}
