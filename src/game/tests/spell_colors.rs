//! Triggers that read the color of a spell being cast.
//!
//! Sol'kanar's audit line said trigger capture could not see a spell's color.
//! It can, and has been able to since the cast event started carrying locked
//! characteristics. What these check is that the color is read off the spell
//! rather than off anything else, and that either player casting it counts.

use super::*;

/// Player one has Sol'kanar out; `caster` casts `spell` from hand with enough
/// mana in the pool to pay for it.
fn cast(spell: CardDefinitionId, caster: PlayerId) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let king = creature(10_000, cards::SOLKANAR_THE_SWAMP_KING, PlayerId::One);
    game.battlefield.push(king);
    // Something for a red spell to point at that costs nobody any life, so
    // the control measures the trigger rather than the spell.
    game.battlefield
        .push(creature(10_001, cards::MOX_JET, PlayerId::Two));

    let card = card(20_000, spell, caster);
    let card_id = card.id;
    game.players[caster.index()].hand.push(card);
    game.players[caster.index()].mana_pool.black = 6;
    game.players[caster.index()].mana_pool.red = 6;
    game.players[caster.index()].mana_pool.colorless = 6;
    game.priority = caster;

    let action = game
        .legal_actions(caster)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == card_id))
        .expect("the spell is castable");
    game.apply(caster, action).expect("the cast is legal");
    drain_pending(&mut game);
    game
}

fn life(game: &Game, player: PlayerId) -> i16 {
    game.players[player.index()].life
}

#[test]
fn a_black_spell_gains_the_king_a_life() {
    let game = cast(cards::DARK_RITUAL, PlayerId::One);

    assert_eq!(
        life(&game, PlayerId::One),
        i16::from(rules::STARTING_LIFE) + 1,
    );
}

/// The control: the trigger names a color, so a spell of another one does
/// nothing.
#[test]
fn a_red_spell_gains_nothing() {
    let game = cast(cards::SHATTER, PlayerId::One);

    assert_eq!(life(&game, PlayerId::One), i16::from(rules::STARTING_LIFE));
}

/// "Whenever a player casts" is either player, and the life still goes to the
/// King's controller.
#[test]
fn an_opponents_black_spell_gains_the_king_a_life() {
    let game = cast(cards::DARK_RITUAL, PlayerId::Two);

    assert_eq!(
        life(&game, PlayerId::One),
        i16::from(rules::STARTING_LIFE) + 1,
        "the King's controller gains it",
    );
    assert_eq!(
        life(&game, PlayerId::Two),
        i16::from(rules::STARTING_LIFE),
        "and the caster gains nothing",
    );
}
