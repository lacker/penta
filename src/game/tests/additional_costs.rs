//! Additional costs that spend an object.
//!
//! Distinct from a target: the object is chosen and spent as the spell is
//! cast, and never checked again. What these check is that the spell is not
//! offered with nothing to spend, that casting it really exiles the chosen
//! card, and that the choice is per-object rather than a single blanket
//! option.

use super::*;
use crate::ImplementationStatus;

/// Makeshift Mauler in hand, mana to cast it, and `fodder` creature cards in
/// the graveyard.
fn mauler_board(fodder: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let mauler = card(10_000, cards::MAKESHIFT_MAULER, PlayerId::One);
    let mauler_id = mauler.id;
    game.players[PlayerId::One.index()].hand.push(mauler);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    for index in 0..fodder {
        game.players[PlayerId::One.index()].graveyard.push(card(
            20_000 + u32::try_from(index).expect("small"),
            cards::SEDGE_TROLL,
            PlayerId::One,
        ));
    }
    (game, mauler_id)
}

fn cast_actions(game: &Game, card: GameObjectId) -> Vec<Vec<GameObjectId>> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card: actual,
                sacrifices,
                ..
            } if actual == card => Some(sacrifices),
            _ => None,
        })
        .collect()
}

#[test]
fn the_spell_is_not_castable_without_something_to_spend() {
    let (game, mauler) = mauler_board(0);
    assert!(
        cast_actions(&game, mauler).is_empty(),
        "an empty graveyard leaves no way to pay"
    );
}

/// One action per payable object, so the player picks which card leaves.
#[test]
fn each_payable_object_is_its_own_choice() {
    let (game, mauler) = mauler_board(2);
    let choices = cast_actions(&game, mauler);
    assert_eq!(choices.len(), 2, "two creature cards, two ways to pay");
    assert_ne!(
        choices[0], choices[1],
        "and they name different cards rather than repeating"
    );
}

#[test]
fn casting_exiles_the_chosen_card() {
    let (mut game, mauler) = mauler_board(1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == mauler))
        .expect("it can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::One.index()].graveyard.is_empty(),
        "the creature card left the graveyard"
    );
    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        1,
        "and went to exile rather than anywhere else"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MAKESHIFT_MAULER),
        "and the creature arrived"
    );
}

/// The predicate narrows what may be spent: a noncreature card in the
/// graveyard is not payment.
#[test]
fn only_matching_cards_can_be_spent() {
    let (mut game, mauler) = mauler_board(0);
    game.players[PlayerId::One.index()].graveyard.push(card(
        20_000,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    assert!(
        cast_actions(&game, mauler).is_empty(),
        "an instant is not a creature card"
    );
}

/// The zone decides what spending means. A creature on the battlefield is
/// sacrificed rather than exiled.
#[test]
fn a_battlefield_cost_sacrifices_rather_than_exiles() {
    let mut game = ready_game();
    let reap = card(10_000, cards::ALTARS_REAP, PlayerId::One);
    let reap_id = reap.id;
    game.players[PlayerId::One.index()].hand.push(reap);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let fodder = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let fodder_id = fodder.card.id;
    game.battlefield.push(fodder);
    let hand_before = game.players[PlayerId::One.index()].hand.len();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == reap_id))
        .expect("it can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fodder_id),
        "the creature was spent"
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SEDGE_TROLL),
        "and it went to the graveyard rather than exile"
    );
    // Two drawn, the Reap itself left hand, and it drew after resolving.
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        hand_before + 1
    );
}

/// A cost paid from hand discards, and never offers the spell itself as its
/// own payment.
#[test]
fn a_hand_cost_discards_something_other_than_the_spell() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let guess = card(10_000, cards::WILD_GUESS, PlayerId::One);
    let guess_id = guess.id;
    game.players[PlayerId::One.index()].hand.push(guess);
    let fodder = card(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let fodder_id = fodder.id;
    game.players[PlayerId::One.index()].hand.push(fodder);
    game.players[PlayerId::One.index()].mana_pool.red = 2;

    assert_eq!(
        cast_actions(&game, guess_id),
        vec![vec![fodder_id]],
        "the spell cannot pay for itself, so only the other card is payment"
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == guess_id))
        .expect("it can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SEDGE_TROLL),
        "the chosen card was discarded rather than exiled"
    );
}

#[test]
fn corpse_lunge_uses_the_exiled_cards_last_known_power() {
    let mut game = ready_game();
    let lunge = card(10_000, cards::CORPSE_LUNGE, PlayerId::One);
    let lunge_id = lunge.id;
    game.players[0].hand.push(lunge);
    let fodder = card(10_001, cards::SERRA_ANGEL, PlayerId::One);
    let fodder_id = fodder.id;
    game.players[0].graveyard.push(fodder);
    let victim = creature(10_002, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let cast = cast_action(
        lunge_id,
        vec![Target::Permanent(victim_id)],
        vec![fodder_id],
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != victim_id),
        "the exiled 4-power card deals lethal damage to a 4/4",
    );
}

#[test]
fn ichor_explosion_uses_the_sacrificed_creatures_effective_power() {
    let mut game = ready_game();
    let explosion = card(10_000, cards::ICHOR_EXPLOSION, PlayerId::One);
    let explosion_id = explosion.id;
    game.players[0].hand.push(explosion);
    let mut fodder = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    fodder.counters.set(CounterKind::PlusOnePlusOne, 1);
    let fodder_id = fodder.card.id;
    game.battlefield.push(fodder);
    let survivor = creature(10_002, cards::AIR_ELEMENTAL, PlayerId::Two);
    let survivor_id = survivor.card.id;
    game.battlefield.push(survivor);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);

    let cast = cast_action(explosion_id, Vec::new(), vec![fodder_id], 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    let survivor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == survivor_id)
        .expect("a 4/4 survives -3/-3");
    assert_eq!(game.power(survivor), Some(1));
    assert_eq!(game.toughness(survivor), Some(1));
}

#[test]
fn every_additional_cost_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::ALTARS_REAP,
        cards::WILD_GUESS,
        cards::MAKESHIFT_MAULER,
        cards::STITCHED_DRAKE,
        cards::HEADLESS_SKAAB,
        cards::RELENTLESS_SKAABS,
        cards::CORPSE_LUNGE,
        cards::ICHOR_EXPLOSION,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
