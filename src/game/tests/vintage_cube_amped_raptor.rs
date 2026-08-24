//! Amped Raptor: two energy, a dig off the top, and a card you pay for in
//! energy instead of mana.

use super::*;

/// Player One with a library of two Mountains under a Lightning Bolt, so the
/// dig walks past lands and stops on the first thing that is not one.
fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    for definition in [cards::LIGHTNING_BOLT, cards::MOUNTAIN, cards::MOUNTAIN] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    game
}

/// Puts a Raptor into Player One's hand and casts it, then resolves.
fn cast_the_raptor(game: &mut Game) {
    let raptor = game
        .build_zone(PlayerId::One, &[cards::AMPED_RAPTOR])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let raptor_id = raptor.id;
    game.players[0].hand.push(raptor);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == raptor_id))
        .expect("two mana buys a Raptor");
    game.apply(PlayerId::One, cast)
        .expect("the Raptor is castable");
    settle(game);
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
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn exiled(game: &Game, definition: CardDefinitionId) -> usize {
    game.players[0]
        .exile
        .iter()
        .filter(|card| card.definition == definition)
        .count()
}

/// Cast from hand: two energy, the lands exiled on the way, and the nonland
/// card exiled with them.
#[test]
fn casting_it_digs_past_the_lands() {
    let mut game = staged();
    cast_the_raptor(&mut game);

    assert_eq!(
        game.players[0].counters.count(CounterKind::Energy),
        2,
        "two energy counters"
    );
    assert_eq!(exiled(&game, cards::MOUNTAIN), 2, "walked past both lands");
    assert_eq!(exiled(&game, cards::LIGHTNING_BOLT), 1, "and stopped here");
    assert!(game.players[0].library.is_empty());
}

/// The exiled card is castable for energy equal to its mana value, and the
/// energy actually goes.
#[test]
fn the_exiled_card_is_bought_with_energy() {
    let mut game = staged();
    cast_the_raptor(&mut game);

    let bolt = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::LIGHTNING_BOLT)
        .expect("the Bolt is in exile")
        .id;
    // Nothing left to pay mana with, so a cast that shows up is paid for in
    // energy or not at all.
    assert_eq!(game.players[0].mana_pool.red, 0);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt))
        .expect("one energy per mana value, and the Bolt costs one");
    game.apply(PlayerId::One, cast).expect("energy pays for it");

    assert_eq!(
        game.players[0].counters.count(CounterKind::Energy),
        1,
        "one energy for a mana value of 1"
    );
    assert_eq!(game.stack.len(), 1, "and the Bolt is on the stack");
}

/// Without the energy for it, the card in exile is not castable at all.
#[test]
fn too_little_energy_is_no_cast() {
    let mut game = staged();
    cast_the_raptor(&mut game);
    game.players[0].counters.set(CounterKind::Energy, 0);

    let bolt = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::LIGHTNING_BOLT)
        .expect("the Bolt is in exile")
        .id;
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == bolt)),
        "energy replaces the mana cost rather than joining it",
    );
}

/// A Raptor that was never cast gets the energy and nothing else.
#[test]
fn one_put_onto_the_battlefield_only_gets_the_energy() {
    let mut game = staged();
    game.put_onto_battlefield(PlayerId::One, cards::AMPED_RAPTOR)
        .expect("cataloged");
    settle(&mut game);

    assert_eq!(
        game.players[0].counters.count(CounterKind::Energy),
        2,
        "the energy is unconditional"
    );
    assert_eq!(
        game.players[0].library.len(),
        3,
        "but the dig is not: it was never cast from a hand",
    );
    assert!(game.players[0].exile.is_empty());
}
