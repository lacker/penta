//! Two taxes on one card: one on casting, one on activating.
//!
//! Neither says "you", so both seats pay, the Gloom player included. The
//! second clause names white *enchantments* rather than white permanents, and
//! the offer and the payment have to agree about the price.

use super::*;

/// A board with `glooms` copies of Gloom under player two.
fn under_gloom(glooms: u32) -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    for index in 0..glooms {
        game.battlefield
            .push(creature(10_000 + index, cards::GLOOM, PlayerId::Two));
    }
    game.priority = PlayerId::One;
    game
}

fn holding(game: &mut Game, spell: CardDefinitionId, white: u16) -> CardInstanceId {
    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);
    game.players[PlayerId::One.index()].mana_pool.white = white;
    spell_id
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

/// Disenchant is {1}{W}; under Gloom it wants {4}{W}.
#[test]
fn a_white_spell_costs_three_more() {
    let mut game = under_gloom(1);
    let spell = holding(&mut game, cards::DISENCHANT, 4);
    assert!(!castable(&game, spell), "four is one short");

    let mut game = under_gloom(1);
    let spell = holding(&mut game, cards::DISENCHANT, 5);
    assert!(castable(&game, spell));
}

/// The control: no Gloom, printed price. Something has to be on the
/// battlefield for Disenchant to point at, which the Gloom itself supplies in
/// every other case here.
#[test]
fn without_gloom_the_spell_costs_what_it_prints() {
    let mut game = under_gloom(0);
    game.battlefield
        .push(creature(11_000, cards::MOX_JET, PlayerId::Two));
    let spell = holding(&mut game, cards::DISENCHANT, 2);
    assert!(castable(&game, spell));
}

/// "White spells", not "white spells you cast", so a Gloom on the other side
/// still taxes you.
#[test]
fn an_opponents_gloom_taxes_you() {
    let mut game = under_gloom(1);
    let spell = holding(&mut game, cards::DISENCHANT, 2);
    assert!(!castable(&game, spell));
}

/// A black spell is untouched.
#[test]
fn a_spell_of_another_colour_is_not_taxed() {
    let mut game = under_gloom(1);
    let spell = holding(&mut game, cards::DARK_RITUAL, 0);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    assert!(castable(&game, spell));
}

/// The second clause: a Circle of Protection's {1} becomes {4}.
#[test]
fn a_white_enchantments_ability_costs_three_more() {
    let mut game = under_gloom(1);
    let circle = creature(11_000, cards::CIRCLE_OF_PROTECTION_RED, PlayerId::One);
    let circle_id = circle.card.id;
    game.battlefield.push(circle);
    game.players[PlayerId::One.index()].mana_pool.white = 3;
    assert!(!offers(&game, circle_id), "three is one short of {{4}}");

    game.players[PlayerId::One.index()].mana_pool.white = 4;
    assert!(offers(&game, circle_id));
}

/// The control for that clause: without Gloom the printed {1} is enough.
#[test]
fn without_gloom_the_ability_costs_what_it_prints() {
    let mut game = under_gloom(0);
    let circle = creature(11_000, cards::CIRCLE_OF_PROTECTION_RED, PlayerId::One);
    let circle_id = circle.card.id;
    game.battlefield.push(circle);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    assert!(offers(&game, circle_id));
}

/// White *enchantments*, so a white creature's ability is untouched.
#[test]
fn a_white_creatures_ability_is_not_taxed() {
    let mut game = under_gloom(1);
    let healer = creature(11_000, cards::SAMITE_HEALER, PlayerId::One);
    let healer_id = healer.card.id;
    game.battlefield.push(healer);
    game.battlefield
        .push(creature(11_001, cards::SEDGE_TROLL, PlayerId::Two));
    game.players[PlayerId::One.index()].mana_pool.white = 1;

    assert!(
        offers(&game, healer_id),
        "a creature is not an enchantment, whatever its colour",
    );
}

/// Paying goes through the same price the offer was made at, so activating
/// actually spends four.
#[test]
fn activating_spends_the_increased_cost() {
    let mut game = under_gloom(1);
    let circle = creature(11_000, cards::CIRCLE_OF_PROTECTION_RED, PlayerId::One);
    let circle_id = circle.card.id;
    game.battlefield.push(circle);
    game.players[PlayerId::One.index()].mana_pool.white = 5;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == circle_id))
        .expect("five white covers the taxed cost");
    game.apply(PlayerId::One, action)
        .expect("the cost is payable");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.white,
        1,
        "four spent, not one",
    );
}
