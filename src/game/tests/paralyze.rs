//! An Aura whose ransom is paid by the other player.
//!
//! Everything about the card points at the host's controller rather than the
//! Aura's: their upkeep is when the offer comes, their mana pays it, and it
//! is their creature that untaps. The prohibition is what makes the offer
//! worth anything, so both halves are checked together.

use super::*;

/// Player two's Troll under player one's Paralyze, with `mana` colorless
/// available to the named player.
fn paralyzed(payer: PlayerId, mana: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;

    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut aura = creature(10_001, cards::PARALYZE, PlayerId::One);
    aura.attached_to = Some(host_id);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);

    game.players[payer.index()].mana_pool.colorless = mana;
    (game, host_id, aura_id)
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

fn set_tapped(game: &mut Game, id: GameObjectId, value: bool) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped = value;
}

/// The enters trigger is what puts the creature down in the first place, so
/// this one casts the Aura rather than placing it.
#[test]
fn it_taps_the_creature_as_it_arrives() {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);
    assert!(!tapped(&game, host_id));

    let spell = card(20_000, cards::PARALYZE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("their Troll is a legal host");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(tapped(&game, host_id));
}

/// Left alone, the creature stays down through its controller's untap step.
#[test]
fn the_creature_does_not_untap_on_its_own() {
    let (mut game, host, _aura) = paralyzed(PlayerId::Two, 0);
    set_tapped(&mut game, host, true);

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_choosing(&mut game, 0);

    assert!(tapped(&game, host), "the prohibition held");
}

/// Four mana from the host's controller buys it back.
#[test]
fn paying_four_untaps_it() {
    let (mut game, host, _aura) = paralyzed(PlayerId::Two, 4);
    set_tapped(&mut game, host, true);

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_choosing(&mut game, usize::MAX);

    assert!(!tapped(&game, host), "paid, so it came back up");
    assert_eq!(game.players[PlayerId::Two.index()].mana_pool.colorless, 0);
}

/// The control: declining leaves it down and spends nothing.
#[test]
fn declining_leaves_it_tapped() {
    let (mut game, host, _aura) = paralyzed(PlayerId::Two, 4);
    set_tapped(&mut game, host, true);

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_choosing(&mut game, 0);

    assert!(tapped(&game, host));
    assert_eq!(
        game.players[PlayerId::Two.index()].mana_pool.colorless,
        4,
        "and nothing was spent",
    );
}

/// The Aura's own controller cannot pay the ransom, and their upkeep is not
/// when it is offered.
#[test]
fn the_aura_controllers_upkeep_offers_nothing() {
    let (mut game, host, _aura) = paralyzed(PlayerId::One, 8);
    set_tapped(&mut game, host, true);

    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_choosing(&mut game, usize::MAX);

    assert!(tapped(&game, host), "wrong upkeep, wrong player");
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        8,
        "their mana was never asked for",
    );
}
