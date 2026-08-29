//! Two cards blocked on machinery that has since been built.
//!
//! Rootwalla's quota is per permanent and per turn, and Stab Wound's drain
//! follows the creature rather than the player who cast the Aura -- both are
//! the halves easiest to get wrong.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn pump_offered(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source))
        .expect("the quota still has room");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(game);
}

/// One pump a turn, and the quota belongs to the permanent rather than the
/// player -- a second Rootwalla still has its own.
#[test]
fn each_rootwalla_pumps_once_a_turn() {
    let mut game = ready();
    let first = creature(10_000, cards::ROOTWALLA, PlayerId::One);
    let first_id = first.card.id;
    game.battlefield.push(first);
    let second = creature(10_100, cards::ROOTWALLA, PlayerId::One);
    let second_id = second.card.id;
    game.battlefield.push(second);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    activate(&mut game, first_id);
    assert_eq!(stats(&game, first_id), (Some(4), Some(4)));
    assert!(
        !pump_offered(&game, first_id),
        "its own quota is spent for the turn",
    );
    assert!(
        pump_offered(&game, second_id),
        "but the other Rootwalla has not spent its own",
    );

    activate(&mut game, second_id);
    assert_eq!(stats(&game, second_id), (Some(4), Some(4)));
}

/// And the quota refills, so next turn it pumps again.
#[test]
fn the_rootwalla_quota_refills_next_turn() {
    let mut game = ready();
    let walla = creature(10_000, cards::ROOTWALLA, PlayerId::One);
    let walla_id = walla.card.id;
    game.battlefield.push(walla);
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    activate(&mut game, walla_id);
    assert!(!pump_offered(&game, walla_id));

    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.commit_next_turn(PlayerId::One, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[PlayerId::One.index()].mana_pool.green = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    assert!(
        pump_offered(&game, walla_id),
        "a new turn, a new activation"
    );
}

/// Stab Wound shrinks the creature and drains the creature's controller --
/// who is not the player who cast the Aura.
#[test]
fn stab_wound_drains_the_creatures_controller() {
    let mut game = ready();
    let victim = creature(10_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    let spell = card(20_000, cards::STAB_WOUND, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("three mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        stats(&game, victim_id),
        (Some(2), Some(2)),
        "a 4/4 with -2/-2",
    );

    let before = [game.players[0].life, game.players[1].life];
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(
        [game.players[0].life, game.players[1].life],
        [before[0], before[1] - 2],
        "the creature's controller pays, not the Aura's",
    );
}

/// On the Aura controller's own upkeep nothing happens, because the enchanted
/// creature is not theirs.
#[test]
fn stab_wound_stays_quiet_on_the_wrong_upkeep() {
    let mut game = ready();
    let victim = creature(10_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    let mut aura = creature(10_100, cards::STAB_WOUND, PlayerId::One);
    aura.attached_to = Some(victim_id);
    game.battlefield.push(aura);

    let before = [game.players[0].life, game.players[1].life];
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(
        [game.players[0].life, game.players[1].life],
        before,
        "the trigger watches the creature's controller's upkeep only",
    );
}
