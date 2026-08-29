//! Switching power and toughness, CR 613.4e.
//!
//! The switch is applied after every other power-and-toughness layer, and two
//! switches in effect at once cancel -- so the effect carries no values and
//! only the parity of how many apply matters. Every test uses a lopsided
//! creature, because a switch is invisible on an evenly-statted one.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
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

/// A 0/8 Wall becomes an 8/0 -- which is also why it then dies.
#[test]
fn transmutation_turns_a_wall_inside_out() {
    let mut game = ready();
    let wall = creature(10_000, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    let spell = card(10_001, cards::TRANSMUTATION, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    assert_eq!(stats(&game, wall_id), (Some(0), Some(8)));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the spell has a target");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == wall_id),
        "an 8/0 has zero toughness, so it is put into a graveyard",
    );
}

/// The switch is applied after a pump, not before it: +2/+0 on a 1/5 gives a
/// 3/5, which switches to 5/3 rather than to 5/1 plus two.
#[test]
fn the_switch_comes_after_every_other_layer() {
    let mut game = ready();
    let charger = creature(10_000, cards::FLUXCHARGER, PlayerId::One);
    let charger_id = charger.card.id;
    game.battlefield.push(charger);
    assert_eq!(stats(&game, charger_id), (Some(1), Some(5)));

    attach_constant_resolved_characteristics(
        &mut game,
        charger_id,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(2),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert_eq!(stats(&game, charger_id), (Some(3), Some(5)), "3/5 first");

    attach_constant_resolved_characteristics(
        &mut game,
        charger_id,
        &[AppliedEffectDef::switch_power_toughness()],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert_eq!(
        stats(&game, charger_id),
        (Some(5), Some(3)),
        "and the switch is applied to that, not to the printed 1/5",
    );
}

/// Two switches cancel, which is what makes the effect carry no values.
#[test]
fn two_switches_cancel() {
    let mut game = ready();
    let charger = creature(10_000, cards::FLUXCHARGER, PlayerId::One);
    let charger_id = charger.card.id;
    game.battlefield.push(charger);

    attach_constant_resolved_characteristics(
        &mut game,
        charger_id,
        &[AppliedEffectDef::switch_power_toughness()],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert_eq!(stats(&game, charger_id), (Some(5), Some(1)), "once");

    attach_constant_resolved_characteristics(
        &mut game,
        charger_id,
        &[AppliedEffectDef::switch_power_toughness()],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert_eq!(stats(&game, charger_id), (Some(1), Some(5)), "and back");
}

/// The Charger's own trigger fires on an instant and is optional.
#[test]
fn the_charger_switches_when_you_cast_an_instant() {
    let mut game = ready();
    let charger = creature(10_000, cards::FLUXCHARGER, PlayerId::One);
    let charger_id = charger.card.id;
    game.battlefield.push(charger);
    let spell = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the Bolt has a target");
    game.apply(PlayerId::One, action).expect("it is cast");
    // The trigger is a "you may", so its offer has to be taken rather than
    // drained at its minimum.
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert_eq!(
        stats(&game, charger_id),
        (Some(5), Some(1)),
        "the trigger took the offer",
    );
}
