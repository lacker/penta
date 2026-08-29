//! A tax paid in a colour rather than in generic mana.
//!
//! An increase is not a discount with the sign flipped: a discount may only
//! touch generic mana, while this adds a black pip that only black mana can
//! pay. It applies to its controller's black spells and to nothing else.

use super::*;

/// `spell` in player one's hand with `black` and `colorless` available, and
/// `derelors` copies of Derelor out under `controller`.
fn taxed(
    spell: CardDefinitionId,
    derelors: u32,
    controller: PlayerId,
    black: u16,
    colorless: u16,
) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    for index in 0..derelors {
        game.battlefield
            .push(creature(10_000 + index, cards::DERELOR, controller));
    }

    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);
    game.players[PlayerId::One.index()].mana_pool.black = black;
    game.players[PlayerId::One.index()].mana_pool.colorless = colorless;
    game.priority = PlayerId::One;
    (game, spell_id)
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

/// Dark Ritual is {B}. With one Derelor out it wants {B}{B}.
#[test]
fn a_black_spell_costs_one_more_black() {
    let (game, spell) = taxed(cards::DARK_RITUAL, 1, PlayerId::One, 1, 0);
    assert!(!castable(&game, spell), "one black no longer covers it");

    let (game, spell) = taxed(cards::DARK_RITUAL, 1, PlayerId::One, 2, 0);
    assert!(castable(&game, spell));
}

/// The control: no Derelor, no tax.
#[test]
fn without_it_the_spell_costs_what_it_prints() {
    let (game, spell) = taxed(cards::DARK_RITUAL, 0, PlayerId::One, 1, 0);
    assert!(castable(&game, spell));
}

/// The added pip is black specifically, so colorless mana cannot pay it.
/// This is what separates an increase from a negative discount.
#[test]
fn colorless_mana_cannot_pay_the_added_pip() {
    let (game, spell) = taxed(cards::DARK_RITUAL, 1, PlayerId::One, 1, 5);
    assert!(!castable(&game, spell), "five colorless is no help");
}

/// Two copies stack, which is why the tax is read off every permanent rather
/// than found once.
#[test]
fn two_copies_tax_twice() {
    let (game, spell) = taxed(cards::DARK_RITUAL, 2, PlayerId::One, 2, 0);
    assert!(!castable(&game, spell));

    let (game, spell) = taxed(cards::DARK_RITUAL, 2, PlayerId::One, 3, 0);
    assert!(castable(&game, spell));
}

/// "Black spells", so a red one is untouched.
#[test]
fn a_spell_of_another_colour_is_not_taxed() {
    let (mut game, spell) = taxed(cards::LIGHTNING_BOLT, 1, PlayerId::One, 0, 0);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(castable(&game, spell));
}

/// "You cast", so the other player's Derelor does not tax player one.
#[test]
fn an_opponents_copy_does_not_tax_you() {
    let (game, spell) = taxed(cards::DARK_RITUAL, 1, PlayerId::Two, 1, 0);
    assert!(castable(&game, spell));
}
