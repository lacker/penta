//! Morbid as an intervening if.
//!
//! With nothing dead the trigger is never created, which is not the same as a
//! trigger that resolves and does nothing. Ulvenwald Bear is where the
//! difference shows: an uncreated trigger never asks for a target, so nothing
//! goes on the stack pointing at a creature.

use super::*;

/// A board with a creature to point at and, if `a_death`, one creature
/// already dead this turn.
fn board(a_death: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    let bystander = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    if a_death {
        let victim = creature(10_500, cards::SEDGE_TROLL, PlayerId::Two);
        let victim_id = victim.card.id;
        game.battlefield.push(victim);
        game.destroy_permanent(victim_id);
        drain_pending(&mut game);
    }
    game.priority = PlayerId::One;
    (game, bystander_id)
}

/// Casts `spell` from hand, paying with a full pool.
fn cast(game: &mut Game, spell: CardDefinitionId) {
    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.black = 2;
    pool.green = 2;
    pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the pool covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

fn zombies(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            )
        })
        .count()
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn wakedancer_makes_a_zombie_when_something_died() {
    let (mut game, _bystander) = board(true);
    cast(&mut game, cards::WAKEDANCER);
    assert_eq!(zombies(&game), 1);
}

/// The control: a quiet turn, no token.
#[test]
fn wakedancer_makes_nothing_on_a_quiet_turn() {
    let (mut game, _bystander) = board(false);
    cast(&mut game, cards::WAKEDANCER);
    assert_eq!(zombies(&game), 0);
}

#[test]
fn ulvenwald_bear_adds_two_counters_when_something_died() {
    let (mut game, bystander) = board(true);
    cast(&mut game, cards::ULVENWALD_BEAR);
    assert_eq!(counters(&game, bystander), 2);
}

/// The sharper half of the control: with nothing dead the trigger is not
/// created, so no target is ever chosen and no counters land anywhere.
#[test]
fn ulvenwald_bear_asks_for_no_target_on_a_quiet_turn() {
    let (mut game, bystander) = board(false);
    cast(&mut game, cards::ULVENWALD_BEAR);

    assert_eq!(counters(&game, bystander), 0);
    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ULVENWALD_BEAR)
        .expect("the Bear resolved");
    assert_eq!(
        counters(&game, bear.card.id),
        0,
        "and it did not point at itself either",
    );
}

#[test]
fn morkrut_banshee_shrinks_a_creature_when_something_died() {
    let (mut game, bystander) = board(true);
    cast(&mut game, cards::MORKRUT_BANSHEE);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bystander),
        "-4/-4 killed the 2/2 it pointed at",
    );
}

/// The same suppression as the Bear, on a card whose effect would otherwise
/// be lethal to whatever it was forced to choose.
#[test]
fn morkrut_banshee_asks_for_no_target_on_a_quiet_turn() {
    let (mut game, bystander) = board(false);
    cast(&mut game, cards::MORKRUT_BANSHEE);

    let stats = |id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(stats(bystander), (Some(2), Some(2)), "untouched");
    let banshee = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MORKRUT_BANSHEE)
        .expect("the Banshee resolved")
        .card
        .id;
    assert_eq!(stats(banshee), (Some(4), Some(4)), "and so is the Banshee");
}

#[test]
fn hollowhenge_scavenger_gains_five_only_when_something_died() {
    let (mut game, _bystander) = board(true);
    cast(&mut game, cards::HOLLOWHENGE_SCAVENGER);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) + 5,
    );

    let (mut game, _bystander) = board(false);
    cast(&mut game, cards::HOLLOWHENGE_SCAVENGER);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
        "a quiet turn gains nothing",
    );
}
