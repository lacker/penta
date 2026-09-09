//! Chalice's counter count is a cast-event filter, not an intervening if.

use super::*;

fn chalice(game: &mut Game, charges: u16) -> GameObjectId {
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::CHALICE_OF_THE_VOID)
        .expect("Chalice enters without being cast");
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .unwrap();
    assert_eq!(permanent.counters(CounterKind::named("charge")), 0);
    permanent.set_counters(CounterKind::named("charge"), charges);
    id
}

fn cast(game: &mut Game, caster: PlayerId, definition: CardDefinitionId, x: u16) {
    let held = game.build_zone(caster, &[definition]).unwrap().remove(0);
    let held_id = held.id;
    game.players[caster.index()].hand.push(held);
    game.active_player = caster;
    game.priority = caster;
    let action = game
        .legal_actions(caster)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == held_id && choices.x() == x)
        })
        .expect("the chosen X is affordable");
    game.apply(caster, action).unwrap();
}

#[test]
fn chalice_pays_twice_x_and_enters_with_x_charge_counters_without_a_trigger() {
    for x in [0, 3] {
        let mut game = ready_game();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2 * x);
        cast(&mut game, PlayerId::One, cards::CHALICE_OF_THE_VOID, x);
        assert!(game.players[0].mana.is_empty(), "each X is paid");
        assert_eq!(game.stack.len(), 1);
        pass_priority_pair(&mut game);
        assert!(game.stack.is_empty(), "counters are an entry replacement");
        assert!(game.pending_triggers.is_empty());
        assert_eq!(
            game.battlefield[0].counters(CounterKind::named("charge")),
            x,
        );
    }
}

#[test]
fn chalice_compares_either_players_spell_to_its_current_charge_count_including_zero() {
    for caster in [PlayerId::One, PlayerId::Two] {
        for (charges, definition, matches) in [
            (0, cards::ORNITHOPTER, true),
            (1, cards::GRIZZLY_BEARS, false),
            (2, cards::GRIZZLY_BEARS, true),
            (3, cards::GRIZZLY_BEARS, false),
        ] {
            let mut game = ready_game();
            chalice(&mut game, charges);
            game.add_unrestricted_mana(caster, ManaColor::Green, 2);
            cast(&mut game, caster, definition, 0);
            assert_eq!(game.stack.len(), if matches { 2 } else { 1 });
            if matches {
                let trigger = game.stack.last().unwrap();
                assert_eq!(trigger.kind, StackObjectKind::TriggeredAbility);
                assert!(trigger.ability.as_ref().unwrap().target_defs.is_empty());
            }
            pass_priority_pair(&mut game);
            assert!(game.stack.is_empty());
            assert_eq!(
                game.players[caster.index()]
                    .graveyard
                    .iter()
                    .any(|card| card.definition == definition),
                matches,
            );
            assert_eq!(
                game.battlefield
                    .iter()
                    .any(|p| p.card.definition == definition),
                !matches,
            );
        }
    }
}

#[test]
fn chalice_does_not_recheck_the_counter_count_when_its_trigger_resolves() {
    for initial in [1, 2] {
        let mut game = ready_game();
        let source = chalice(&mut game, initial);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
        cast(&mut game, PlayerId::One, cards::GRIZZLY_BEARS, 0);
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
            .unwrap()
            .set_counters(CounterKind::named("charge"), 3 - initial);
        pass_priority_pair(&mut game);
        assert!(game.stack.is_empty());
        assert_eq!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::GRIZZLY_BEARS),
            initial == 2,
            "only the count at cast time matters",
        );
    }
}

#[test]
fn chalice_uses_the_spells_chosen_x_including_each_x_symbol() {
    for charges in [0, 2, 4] {
        let mut game = ready_game();
        chalice(&mut game, charges);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
        cast(&mut game, PlayerId::One, cards::WALKING_BALLISTA, 2);
        assert_eq!(game.stack.len(), if charges == 4 { 2 } else { 1 });
        pass_priority_pair(&mut game);
        assert!(game.stack.is_empty());
        assert_eq!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::WALKING_BALLISTA),
            charges == 4,
        );
    }
}

#[test]
fn chalice_still_counters_the_spell_after_leaving_the_battlefield() {
    let mut game = ready_game();
    let source = chalice(&mut game, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    cast(&mut game, PlayerId::One, cards::GRIZZLY_BEARS, 0);
    game.destroy_permanent(source);
    assert!(game.battlefield.is_empty());
    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn chalice_reads_mana_value_instead_of_the_taxed_casting_cost() {
    let mut game = ready_game();
    chalice(&mut game, 2);
    game.put_onto_battlefield(PlayerId::One, cards::THALIA_GUARDIAN_OF_THRABEN)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    cast(&mut game, PlayerId::One, cards::CHALICE_OF_THE_VOID, 1);
    assert!(
        game.players[0].mana.is_empty(),
        "two X symbols plus the tax"
    );
    assert_eq!(game.stack.len(), 2, "the spell's mana value is still two");
    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CHALICE_OF_THE_VOID)
    );
}

#[test]
fn chalice_reads_a_face_down_spells_zero_mana_value() {
    for charges in [0, 3, 9] {
        let mut game = ready_game();
        chalice(&mut game, charges);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        cast(&mut game, PlayerId::One, cards::KROSAN_COLOSSUS, 0);
        let spell = &game.stack[0];
        assert!(spell.face_down.is_some());
        assert_eq!(game.stack_spell_mana_value(spell), 0);
        let event = game.stack_trigger_event_object(spell).unwrap();
        assert_eq!(event.mana_value, 0);
        assert_eq!(event.colors, [false; 5]);
        assert_eq!(event.power, Some(2));
        assert!(event.subtypes.is_empty());
        assert_eq!(game.stack.len(), if charges == 0 { 2 } else { 1 });
        pass_priority_pair(&mut game);
        assert!(game.stack.is_empty());
        assert_eq!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::KROSAN_COLOSSUS),
            charges == 0,
            "neither the three mana paid nor the printed nine is its mana value",
        );
    }
}
