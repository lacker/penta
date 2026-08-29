//! "Spells you cast cost less to cast."
//!
//! Read off a permanent rather than the card in hand, so unlike a card
//! discounting itself the clause has to say which spells and whose. Several
//! stack, and none of them can take a cost below its coloured requirements.

use super::*;

/// What `spell` costs player one to cast, in total mana, with `sources` on
/// the battlefield under `controller`.
fn cost_of(
    spell: CardDefinitionId,
    sources: &[(CardDefinitionId, PlayerId)],
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, (definition, controller)) in sources.iter().enumerate() {
        let id = 9_000 + u32::try_from(index).expect("a short list fits");
        game.battlefield
            .push(creature(id, *definition, *controller));
    }
    let card = card(10_000, spell, PlayerId::One);
    let card_id = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    (game, card_id)
}

fn reduction(game: &Game, spell: CardDefinitionId, card: GameObjectId) -> u16 {
    game.spell_cost_reduction(spell, PlayerId::One, card, &[])
        .generic()
}

#[test]
fn nothing_on_the_battlefield_discounts_nothing() {
    let (game, card) = cost_of(cards::LIGHTNING_BOLT, &[]);
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 0);
}

#[test]
fn the_electromancer_discounts_an_instant() {
    let (game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::GOBLIN_ELECTROMANCER, PlayerId::One)],
    );
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 1);
}

/// It names instants and sorceries, so a creature spell pays full price.
#[test]
fn the_electromancer_leaves_a_creature_alone() {
    let (game, card) = cost_of(
        cards::SEDGE_TROLL,
        &[(cards::GOBLIN_ELECTROMANCER, PlayerId::One)],
    );
    assert_eq!(reduction(&game, cards::SEDGE_TROLL, card), 0);
}

/// The caster relation is what separates these two: the Electromancer helps
/// only its controller, and Arcane Melee helps everyone.
#[test]
fn an_opponents_electromancer_does_not_help() {
    let (game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::GOBLIN_ELECTROMANCER, PlayerId::Two)],
    );
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 0);
}

#[test]
fn an_opponents_arcane_melee_still_helps() {
    let (game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::ARCANE_MELEE, PlayerId::Two)],
    );
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 2);
}

/// Two of them add up.
#[test]
fn discounts_stack() {
    let (game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[
            (cards::GOBLIN_ELECTROMANCER, PlayerId::One),
            (cards::ARCANE_MELEE, PlayerId::One),
        ],
    );
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 3);
}

/// Planar Gate and Mana Matrix name different halves of the card pool, and
/// each ignores the other's.
#[test]
fn the_two_artifacts_name_different_spells() {
    let (game, card) = cost_of(cards::SEDGE_TROLL, &[(cards::PLANAR_GATE, PlayerId::One)]);
    assert_eq!(reduction(&game, cards::SEDGE_TROLL, card), 2);

    let (game, card) = cost_of(cards::SEDGE_TROLL, &[(cards::MANA_MATRIX, PlayerId::One)]);
    assert_eq!(reduction(&game, cards::SEDGE_TROLL, card), 0);

    let (game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::MANA_MATRIX, PlayerId::One)],
    );
    assert_eq!(reduction(&game, cards::LIGHTNING_BOLT, card), 2);
}

/// A discount larger than the generic part cannot eat the coloured pips: a
/// Bolt is still {R} however much is taken off.
#[test]
fn a_discount_cannot_reach_the_coloured_cost() {
    let (mut game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::ARCANE_MELEE, PlayerId::One)],
    );
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    let castable = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card: cast, .. } if cast == card));
    assert!(castable, "one red still casts it");

    let (mut game, card) = cost_of(
        cards::LIGHTNING_BOLT,
        &[(cards::ARCANE_MELEE, PlayerId::One)],
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    let castable = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card: cast, .. } if cast == card));
    assert!(!castable, "and five colourless does not");
}
