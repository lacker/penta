//! Seasoned Pyromancer: two cards for two cards, a body for each one that
//! was not a land, and the same again from the graveyard.

use super::*;

/// Player One with `hand` in hand and `library` stacked so the last entry is
/// on top.
fn staged(hand: &[CardDefinitionId], library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].library.clear();
    for definition in hand {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].hand.push(card);
    }
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers every pending decision, discarding the cards whose definitions
/// are listed rather than whatever comes first.
fn settle_discarding(game: &mut Game, wanted: &[CardDefinitionId]) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let mut remaining = wanted.to_vec();
            let mut options = Vec::new();
            for option in &decision.options {
                let Some((_, ObjectCharacteristics::Card { definition, .. })) = option.card else {
                    continue;
                };
                if let Some(index) = remaining.iter().position(|wanted| *wanted == definition) {
                    remaining.remove(index);
                    options.push(option.id);
                }
            }
            if options.len() < decision.minimum {
                options = decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum)
                    .collect();
            }
            options.truncate(decision.maximum);
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

fn elementals(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Elemental"))
        })
        .count()
}

/// Two nonland cards discarded is two Elementals, and the two cards drawn
/// come before them.
#[test]
fn two_nonland_discards_make_two_elementals() {
    let mut game = staged(
        &[cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS],
        &[cards::MOUNTAIN, cards::FOREST],
    );
    game.put_onto_battlefield(PlayerId::One, cards::SEASONED_PYROMANCER)
        .expect("cataloged");

    settle_discarding(&mut game, &[cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS]);

    assert_eq!(elementals(&game), 2);
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::FOREST, cards::MOUNTAIN],
        "the two cards drawn are what is left in hand",
    );
    assert_eq!(game.players[0].graveyard.len(), 2);
}

/// A land discarded pays nothing, which is what makes him a way to turn
/// flood into action rather than into bodies.
#[test]
fn a_discarded_land_makes_no_token() {
    let mut game = staged(
        &[cards::MOUNTAIN, cards::LIGHTNING_BOLT],
        &[cards::FOREST, cards::FOREST],
    );
    game.put_onto_battlefield(PlayerId::One, cards::SEASONED_PYROMANCER)
        .expect("cataloged");

    settle_discarding(&mut game, &[cards::MOUNTAIN, cards::LIGHTNING_BOLT]);

    assert_eq!(elementals(&game), 1, "only the Bolt paid");
}

/// The graveyard half exiles the card for two more bodies.
#[test]
fn the_graveyard_half_exiles_him_for_two_more() {
    let mut game = staged(&[], &[]);
    let card = game
        .build_zone(PlayerId::One, &[cards::SEASONED_PYROMANCER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[0].graveyard.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == id))
        .expect("five mana and the card in the graveyard pay for it");
    game.apply(PlayerId::One, activate).expect("it activates");
    settle_discarding(&mut game, &[]);

    assert_eq!(elementals(&game), 2);
    assert!(game.players[0].graveyard.is_empty(), "the card is exiled");
    assert_eq!(game.players[0].exile.len(), 1);
}

/// "If you have fewer than two cards in hand as the first ability resolves,
/// you'll discard your hand, then draw two cards regardless of how many you
/// discarded." One card in hand is one discard, one Elemental, and two
/// cards drawn all the same.
#[test]
fn one_card_in_hand_is_discarded_and_two_are_still_drawn() {
    let mut game = staged(&[cards::LIGHTNING_BOLT], &[cards::FOREST, cards::MOUNTAIN]);
    game.put_onto_battlefield(PlayerId::One, cards::SEASONED_PYROMANCER)
        .expect("cataloged");

    settle_discarding(&mut game, &[cards::LIGHTNING_BOLT]);

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::MOUNTAIN, cards::FOREST],
        "two cards were drawn off a single discard",
    );
    assert_eq!(elementals(&game), 1, "and the one nonland card paid once");
    assert_eq!(game.players[0].graveyard.len(), 1);
}

/// The same rule at nothing: an empty hand discards nothing, makes no
/// Elemental, and the two cards are drawn anyway.
#[test]
fn an_empty_hand_discards_nothing_and_still_draws_two() {
    let mut game = staged(&[], &[cards::FOREST, cards::MOUNTAIN]);
    game.put_onto_battlefield(PlayerId::One, cards::SEASONED_PYROMANCER)
        .expect("cataloged");

    settle_discarding(&mut game, &[]);

    assert_eq!(game.players[0].hand.len(), 2, "the draw is unconditional");
    assert_eq!(elementals(&game), 0, "nothing was discarded to pay for one");
    assert!(
        game.players[0].graveyard.is_empty(),
        "and nothing went to the graveyard",
    );
}

/// The second ability is printed to be activated from the graveyard, and
/// only from there: the Pyromancer standing on the battlefield with five
/// mana up is not offered it.
#[test]
fn the_graveyard_half_is_not_offered_from_the_battlefield() {
    let mut game = staged(&[], &[cards::FOREST, cards::MOUNTAIN]);
    let pyromancer = game
        .put_onto_battlefield(PlayerId::One, cards::SEASONED_PYROMANCER)
        .expect("cataloged");
    settle_discarding(&mut game, &[]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == pyromancer)
        ),
        "the exile cost wants the card in the graveyard, not the creature on the board",
    );
    assert_eq!(
        elementals(&game),
        0,
        "and no Elemental turned up while nobody was looking",
    );
}
