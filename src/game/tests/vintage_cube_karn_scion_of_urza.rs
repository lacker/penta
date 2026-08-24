//! Karn, Scion of Urza: a card a turn that the other player picks, a pile of
//! leftovers he can cash in later, and a body made of your artifacts.

use super::*;

/// Karn on the battlefield with `library` stacked so the last entry is on
/// top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.players[0].exile.clear();
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let karn = game
        .put_onto_battlefield(PlayerId::One, cards::KARN_SCION_OF_URZA)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, karn)
}

fn activate(game: &mut Game, karn: GameObjectId, ability: u8) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability: id, .. },
                ..
            } => *source == karn && *id == AbilityId(ability),
            _ => false,
        })
        .unwrap_or_else(|| panic!("ability {ability} is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
}

/// Answers whatever is asked, naming the card whose definition is `wanted`
/// when it is on offer.
fn settle(game: &mut Game, wanted: Option<CardDefinitionId>) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let mut options: Vec<_> = decision
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
            if options.len() < decision.minimum {
                options = decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum)
                    .collect();
            }
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

/// The opponent picks which card Karn keeps; the other one waits in exile
/// under a silver counter.
#[test]
fn the_opponent_chooses_and_the_rest_is_banked() {
    let (mut game, karn) = staged(&[cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS]);

    activate(&mut game, karn, 0);
    settle(&mut game, Some(cards::GRIZZLY_BEARS));

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::GRIZZLY_BEARS],
        "the card the opponent named went to hand",
    );
    let exiled = game.players[0]
        .exile
        .first()
        .expect("the other card is exiled");
    assert_eq!(exiled.definition, cards::LIGHTNING_BOLT);
    assert_eq!(
        exiled.counters(CounterKind::named("silver")),
        1,
        "and it carries the silver counter that names it later",
    );
}

/// The minus cashes one of those cards in, and it is the counter that makes
/// the pile nameable: an unrelated exiled card is not on offer.
#[test]
fn the_minus_takes_back_a_silver_card() {
    let (mut game, karn) = staged(&[cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS]);
    let stray = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].exile.push(stray);

    activate(&mut game, karn, 0);
    settle(&mut game, Some(cards::GRIZZLY_BEARS));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == karn)
        .expect("he is there")
        .activated_loyalty_this_turn = false;

    activate(&mut game, karn, 1);
    settle(&mut game, None);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the banked card came back",
    );
    assert_eq!(
        game.players[0]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::MOUNTAIN],
        "the card with no silver counter was never a candidate",
    );
}

/// The Construct is as big as the artifacts you control, itself included.
#[test]
fn the_construct_counts_your_artifacts() {
    let (mut game, karn) = staged(&[]);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == karn)
        .expect("he is there")
        .set_counters(CounterKind::Loyalty, 5);

    activate(&mut game, karn, 2);
    settle(&mut game, None);

    let construct = game
        .battlefield
        .iter()
        .find(|permanent| game.effective_subtypes(permanent).contains(&"Construct"))
        .expect("the token arrived");
    assert_eq!(game.power(construct), Some(1), "it counts itself");

    game.put_onto_battlefield(PlayerId::One, cards::GUARDIAN_IDOL)
        .expect("cataloged");
    drain_pending(&mut game);

    let construct = game
        .battlefield
        .iter()
        .find(|permanent| game.effective_subtypes(permanent).contains(&"Construct"))
        .expect("still there");
    assert_eq!(game.power(construct), Some(2), "and every other artifact");
}
