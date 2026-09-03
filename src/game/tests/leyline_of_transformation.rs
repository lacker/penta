use super::*;

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is cataloged"))
}

fn enter_leyline_choosing(game: &mut Game, creature_type: &str) -> GameObjectId {
    let leyline = definition(game, "Leyline of Transformation");
    let id = game
        .put_onto_battlefield(PlayerId::One, leyline)
        .expect("Leyline is cataloged");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "the as-enters choice happens before Leyline is on the battlefield",
    );
    choose_decision_by_label(game, PlayerId::One, creature_type);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| permanent.chosen_creature_type.as_deref()),
        Some(creature_type),
    );
    id
}

#[test]
fn leyline_adds_its_choice_to_creatures_as_they_enter() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield.clear();
        enter_leyline_choosing(&mut game, "Warrior");

        let bear = definition(&game, "Grizzly Bears");
        let prospective = creature(29_000, bear, PlayerId::One);
        let subtypes = game.effective_subtypes_with_prospective(&prospective, &prospective);
        assert!(subtypes.contains(&"Bear"));
        assert!(subtypes.contains(&"Warrior"));
    }
}

#[test]
fn leyline_uses_controller_on_the_battlefield_and_owner_everywhere_else() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield.clear();
        game.players.iter_mut().for_each(|player| {
            player.hand.clear();
            player.library.clear();
            player.graveyard.clear();
            player.exile.clear();
        });
        enter_leyline_choosing(&mut game, "Warrior");

        let bear = definition(&game, "Grizzly Bears");
        let bolt = definition(&game, "Lightning Bolt");
        let controlled = creature(29_010, bear, PlayerId::One);
        let mut owned_but_not_controlled = creature(29_011, bear, PlayerId::Two);
        owned_but_not_controlled.card.owner = PlayerId::One;
        game.battlefield
            .extend([controlled, owned_but_not_controlled]);

        assert!(
            game.object_subtypes(GameObjectId(29_010))
                .contains(&"Warrior")
        );
        assert!(
            !game
                .object_subtypes(GameObjectId(29_011))
                .contains(&"Warrior"),
            "battlefield objects are related by controller, not owner",
        );

        let mut owned_cards = game
            .build_zone(PlayerId::One, &[bear, bear, bear, bear, bolt])
            .expect("fixture cards are cataloged");
        let hand_creature = owned_cards.remove(0);
        let library_creature = owned_cards.remove(0);
        let graveyard_creature = owned_cards.remove(0);
        let exile_creature = owned_cards.remove(0);
        let hand_noncreature = owned_cards.remove(0);
        let hand_creature_id = hand_creature.id;
        let ids = [
            hand_creature.id,
            library_creature.id,
            graveyard_creature.id,
            exile_creature.id,
        ];
        let noncreature_id = hand_noncreature.id;
        game.players[0]
            .hand
            .extend([hand_creature, hand_noncreature]);
        game.players[0].library.push(library_creature);
        game.players[0].graveyard.push(graveyard_creature);
        game.players[0].exile.push(exile_creature);

        for id in ids {
            let subtypes = game.object_subtypes(id);
            assert!(subtypes.contains(&"Bear"));
            assert!(subtypes.contains(&"Warrior"));
        }
        assert!(game.object_subtypes(noncreature_id).is_empty());

        let spell = game
            .printed_trigger_event_object(
                hand_creature_id,
                bear,
                PlayerId::One,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(CardPartId::PRIMARY),
                },
            )
            .expect("the creature has stack characteristics");
        assert!(spell.subtypes.contains(&"Bear"));
        assert!(spell.subtypes.contains(&"Warrior"));

        let opposing_card = game
            .build_zone(PlayerId::Two, &[bear])
            .expect("fixture card is cataloged")
            .pop()
            .expect("one card was built");
        let opposing_id = opposing_card.id;
        game.players[1].graveyard.push(opposing_card);
        assert!(
            !game.object_subtypes(opposing_id).contains(&"Warrior"),
            "cards outside the battlefield are related by owner",
        );
    }
}
