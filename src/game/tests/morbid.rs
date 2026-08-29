//! Morbid.
//!
//! An entry replacement that only applies if a creature died this turn. The
//! condition is read as the permanent enters rather than when its spell was
//! cast, which is what lets a creature dying in response turn it on.

use super::*;

fn cast_morbid_creature(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let creature = card(10_000, definition, PlayerId::One);
    let creature_id = creature.id;
    game.players[PlayerId::One.index()].hand.push(creature);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == creature_id))
        .expect("the creature is castable");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(game);

    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("the creature entered")
        .card
        .id
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("on the battlefield")
        .counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn without_a_death_the_morbid_creatures_enter_as_printed() {
    for (definition, printed) in [
        (cards::FESTERHIDE_BOAR, (3, 3)),
        (cards::SOMBERWALD_SPIDER, (2, 4)),
        (cards::GRAVETILLER_WURM, (4, 4)),
    ] {
        let mut game = ready_game();
        let creature = cast_morbid_creature(&mut game, definition);

        assert_eq!(counters(&game, creature), 0, "nothing has died this turn");
        let creature = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature)
            .expect("there");
        assert_eq!(
            (game.power(creature), game.toughness(creature)),
            (Some(printed.0), Some(printed.1)),
        );
    }
}

#[test]
fn a_death_this_turn_adds_each_printed_counter_bonus() {
    for (definition, bonus, expected) in [
        (cards::FESTERHIDE_BOAR, 2, (5, 5)),
        (cards::SOMBERWALD_SPIDER, 2, (4, 6)),
        (cards::GRAVETILLER_WURM, 4, (8, 8)),
    ] {
        let mut game = ready_game();
        let doomed = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
        let doomed_id = doomed.card.id;
        game.battlefield.push(doomed);
        game.destroy_permanent(doomed_id);
        drain_pending(&mut game);

        let creature = cast_morbid_creature(&mut game, definition);
        assert_eq!(counters(&game, creature), bonus, "morbid is on");
        let creature = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature)
            .expect("there");
        assert_eq!(
            (game.power(creature), game.toughness(creature)),
            (Some(expected.0), Some(expected.1)),
        );
    }
}
