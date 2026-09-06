//! Creatures that tap for mana. Mana abilities are never offered as
//! standalone actions, so what needs covering is that the planner reaches
//! one on a creature at all, and that a source offering a fixed set of
//! colours is held to that set: the three-colour Elf pays for a blue spell
//! and not for a red one, while the any-colour Tree pays for both.

use super::*;

/// `source` as the only permanent player one controls, with `spell` in hand
/// and no mana anywhere else.
fn staged(source: CardDefinitionId, spell: CardDefinitionId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    let held = card(44_000, spell, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    let mut permanent = creature(44_001, source, PlayerId::One);
    permanent.entered_controller_turn = 0;
    game.battlefield.push(permanent);
    (game, held_id)
}

fn can_cast(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

#[test]
fn the_elf_pays_for_a_colour_it_makes() {
    let (game, spell) = staged(cards::URBORG_ELF, cards::UNSUMMON);
    assert!(can_cast(&game, spell), "{{U}} is one of the Elf's three");
}

#[test]
fn the_elf_does_not_pay_for_a_colour_it_does_not_make() {
    let (game, spell) = staged(cards::URBORG_ELF, cards::LIGHTNING_BOLT);
    assert!(
        !can_cast(&game, spell),
        "the Elf makes black, green and blue, so red is out of reach"
    );
}

#[test]
fn the_tree_pays_for_either_colour() {
    for spell in [cards::UNSUMMON, cards::LIGHTNING_BOLT] {
        let (game, held) = staged(cards::UTOPIA_TREE, spell);
        assert!(can_cast(&game, held), "any colour covers a one-pip spell");
    }
}

#[test]
fn taking_the_cast_taps_the_creature_for_it() {
    let (mut game, spell) = staged(cards::UTOPIA_TREE, cards::LIGHTNING_BOLT);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the Tree covers {R}");
    game.apply(PlayerId::One, cast)
        .expect("the tap pays for it");

    assert!(
        game.battlefield.iter().all(|permanent| permanent.tapped),
        "the Tree was tapped to make the mana"
    );
    assert_eq!(game.stack.len(), 1, "and the Bolt is on the stack");
}
