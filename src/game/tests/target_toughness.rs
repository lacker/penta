//! Life gained from what a target slot points at.
//!
//! A target's power was readable and its toughness was not, which is the only
//! thing that stood between these three cards and the catalog. All of them
//! are aimed at a Wall of Stone, because 0/8 is the only creature that can
//! tell one reading from the other -- and for Predator's Rapport, that also
//! tells a sum apart from either half of it.

use super::*;

/// The spell in hand with mana to cast it, and a Wall of Stone under
/// `wall_controller`.
fn board(spell: CardDefinitionId, wall_controller: PlayerId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    game.battlefield
        .push(creature(10_000, cards::WALL_OF_STONE, wall_controller));
    let card = card(10_001, spell, PlayerId::One);
    let card_id = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    (game, card_id)
}

/// Casts the spell at its only legal target and resolves everything after.
fn cast_and_resolve(game: &mut Game, spell: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the spell has a legal target");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(game);
}

/// Zero power plus eight toughness is eight, which no single characteristic
/// and no doubling of one could produce.
#[test]
fn predators_rapport_adds_the_two_characteristics() {
    let (mut game, spell) = board(cards::PREDATORS_RAPPORT, PlayerId::One);
    cast_and_resolve(&mut game, spell);

    assert_eq!(game.players[0].life, 28, "zero power plus eight toughness");
}

/// Sheltering Word reads only the toughness, and grants the hexproof it
/// reads it through.
#[test]
fn sheltering_word_pays_the_toughness_and_grants_hexproof() {
    let (mut game, spell) = board(cards::SHELTERING_WORD, PlayerId::One);
    cast_and_resolve(&mut game, spell);

    assert_eq!(game.players[0].life, 28, "eight toughness, not zero power");
    let wall = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::WALL_OF_STONE)
        .expect("still there");
    assert!(
        game.permanent_has_executable_keyword(wall, KeywordAbility::Hexproof),
        "and the shelter is real",
    );
}

/// The opponent loses the creature; the caster gains its toughness.
#[test]
fn tribute_to_hunger_pays_the_caster_not_the_victim() {
    let (mut game, spell) = board(cards::TRIBUTE_TO_HUNGER, PlayerId::Two);
    cast_and_resolve(&mut game, spell);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::WALL_OF_STONE),
        "the Wall was eaten",
    );
    assert_eq!(game.players[0].life, 28, "and the caster gained eight");
    assert_eq!(game.players[1].life, 20, "while the victim gained nothing");
}

/// Both green spells name a creature you control, so an opponent's Wall is
/// not a legal target for either.
#[test]
fn the_green_spells_will_not_aim_at_an_opponents_creature() {
    for definition in [cards::PREDATORS_RAPPORT, cards::SHELTERING_WORD] {
        let (game, spell) = board(definition, PlayerId::Two);
        assert!(
            !game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell)),
            "{definition:?} should have nothing to aim at",
        );
    }
}
