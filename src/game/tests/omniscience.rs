//! Omniscience's hand-only battlefield-granted alternative cost.

use super::*;

fn casts_of(game: &Game, player: PlayerId, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(player)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

#[test]
fn omniscience_offers_free_casts_from_its_controllers_hand() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(121_000, cards::OMNISCIENCE, PlayerId::One));
    let spell = card(121_001, cards::MIND_SCULPT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    let casts = casts_of(&game, PlayerId::One, spell_id);
    assert_eq!(casts.len(), 1);
    assert!(matches!(
        &casts[0],
        Action::CastSpell { choices, .. } if choices.costs().alternative().is_some()
    ));

    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let opposing_spell = card(121_002, cards::MIND_SCULPT, PlayerId::Two);
    let opposing_spell_id = opposing_spell.id;
    game.players[PlayerId::Two.index()]
        .hand
        .push(opposing_spell);
    assert!(
        casts_of(&game, PlayerId::Two, opposing_spell_id).is_empty(),
        "Omniscience does not waive an opponent's mana cost"
    );
}

#[test]
fn omniscience_does_not_waive_a_flashback_cost_from_the_graveyard() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(121_010, cards::OMNISCIENCE, PlayerId::One));
    let spell = card(121_011, cards::DIVINE_RECKONING, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].graveyard.push(spell);

    assert!(
        casts_of(&game, PlayerId::One, spell_id).is_empty(),
        "a spell outside the hand still owes its own graveyard cost"
    );

    game.players[PlayerId::One.index()].mana_pool.white = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    let casts = casts_of(&game, PlayerId::One, spell_id);
    assert_eq!(casts.len(), 1);
    assert!(matches!(
        &casts[0],
        Action::CastSpell { choices, .. }
            if choices.costs().alternative() == Some(AlternativeCostId(1))
    ));
}

#[test]
fn omniscience_casts_an_x_spell_only_with_x_zero() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(121_020, cards::OMNISCIENCE, PlayerId::One));
    let spell = card(121_021, cards::DEVILS_PLAY, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    let casts = casts_of(&game, PlayerId::One, spell_id);
    assert!(!casts.is_empty(), "the zero-cost alternative is payable");
    assert!(casts.iter().all(|action| {
        matches!(
            action,
            Action::CastSpell { choices, .. }
                if choices.costs().alternative().is_some() && choices.x() == 0
        )
    }));
}
