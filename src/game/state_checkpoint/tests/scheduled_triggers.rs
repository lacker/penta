use super::*;

#[test]
fn a_frozen_one_shot_trigger_reconstructs_with_its_nested_ability_and_link() {
    let mut game = crate::game::tests::ready_game();
    let linked =
        crate::game::tests::creature(10_010, crate::card::cards::SERRA_ANGEL, PlayerId::Two);
    let source = linked.card.id;
    game.battlefield.push(linked);
    let (origin, ability, presentation) = {
        let card = game
            .catalog
            .get(crate::card::cards::NECROMANCY)
            .expect("Necromancy is cataloged");
        let option = card.play_options.first().expect("Necromancy can be cast");
        let (origin, ability) = Game::play_option_cleanup_flash_trigger(card, option)
            .expect("Necromancy declares its delayed cleanup ability");
        (origin, ability, card.id)
    };
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        panic!("the nested cleanup ability is triggered");
    };
    let capture = TriggerCapture {
        source: AbilitySourceRef {
            object: source,
            ability: origin,
        },
        definition: presentation,
        owner: PlayerId::Two,
        controller: PlayerId::Two,
        text: ability.text,
        target_defs: triggered.targets,
        effect: ability.effect.definition,
        resolver: Game::ability_resolver(origin, &ability),
        context: TriggerContext {
            source_linked: Some(source),
            ..TriggerContext::empty()
        },
        condition: triggered.condition,
    };
    game.schedule_one_shot_event_trigger(triggered.event, &capture);

    let (viewer, wire) = checkpoint_wire(&game);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        1_012,
    )
    .expect("the one-shot trigger reconstructs");
    assert_eq!(rebuilt.scheduled_triggers, game.scheduled_triggers);

    rebuilt.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::Cleanup,
        player: PlayerId::One,
    });
    rebuilt.finish_rules_procedure();
    assert!(rebuilt.scheduled_triggers.is_empty());
    let stacked = rebuilt.stack.last().expect("the due trigger is stacked");
    assert_eq!(stacked.source, Some(source));
    assert_eq!(stacked.controller, PlayerId::Two);
    assert_eq!(
        stacked
            .ability
            .as_ref()
            .expect("trigger payload is frozen")
            .context
            .source_linked,
        Some(source),
    );
}

fn game_with_pending_necromancy_entry() -> (Game, GameObjectId, GameObjectId, u32) {
    let mut game = crate::game::tests::ready_game();
    let controller = PlayerId::Two;
    let hand_card = game.unbacked_object(
        crate::card::cards::NECROMANCY,
        controller,
        CharacteristicSource::Card(crate::card::cards::NECROMANCY),
    );
    let cast_source = hand_card.id;
    let (origin, ability, presentation) = {
        let card = game
            .catalog
            .get(crate::card::cards::NECROMANCY)
            .expect("Necromancy is cataloged");
        let option = card.play_options.first().expect("Necromancy can be cast");
        let (origin, ability) = Game::play_option_cleanup_flash_trigger(card, option)
            .expect("Necromancy declares its delayed cleanup ability");
        (origin, ability, card.id)
    };
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        panic!("the nested cleanup ability is triggered");
    };
    let cleanup_trigger = game.schedule_one_shot_event_trigger(
        triggered.event,
        &TriggerCapture {
            source: AbilitySourceRef {
                object: cast_source,
                ability: origin,
            },
            definition: presentation,
            owner: controller,
            controller,
            text: ability.text,
            target_defs: triggered.targets,
            effect: ability.effect.definition,
            resolver: Game::ability_resolver(origin, &ability),
            context: TriggerContext::empty(),
            condition: triggered.condition,
        },
    );

    let (stack_card, _) = game.zone_change_card(hand_card);
    let spell_id = stack_card.id;
    let signature = CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::default(),
    );
    let stack_object = StackObject {
        id: spell_id,
        kind: StackObjectKind::Spell,
        card: stack_card.clone(),
        source: None,
        ability: game.frozen_spell_payload(crate::card::cards::NECROMANCY, &signature),
        controller,
        signature: Some(signature),
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        text_changes: Vec::new(),
        colors: None,
        cast_via_flashback: false,
        schedule_on_entry: Some(cleanup_trigger),
        is_copy: false,
    };
    game.retire_stack_object(&stack_object);

    let mut entering = Permanent::entering(
        stack_card,
        CardPartId::PRIMARY,
        controller,
        game.turns_started[controller.index()],
    );
    game.initialize_battlefield_entry(&mut entering);
    game.pending_events.push_back(PendingEvent {
        event: ReplaceableEvent::BattlefieldEntry(PendingBattlefieldEntry {
            permanent: entering,
            from: ZoneKind::Stack,
            completion: EntryCompletion::SpellResolved {
                card: spell_id,
                definition: crate::card::cards::NECROMANCY,
            },
        }),
        applied: Vec::new(),
        effects: Vec::new(),
    });
    (game, cast_source, spell_id, cleanup_trigger)
}

#[test]
fn pending_necromancy_entry_reconstructs_and_binds_its_cleanup_trigger() {
    let (game, cast_source, spell_id, cleanup_trigger) = game_with_pending_necromancy_entry();
    let (viewer, wire) = checkpoint_wire(&game);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        1_014,
    )
    .expect("the pending Necromancy entry reconstructs");
    assert!(
        matches!(
            rebuilt.retired_objects.get(&spell_id),
            Some(RetiredObject::Stack(object))
                if object.schedule_on_entry == Some(cleanup_trigger)
        ),
        "the pending entry retains the retired spell that owns the binding",
    );

    let pending = rebuilt
        .pending_events
        .pop_front()
        .expect("Necromancy still has a pending battlefield entry");
    rebuilt.commit_pending_event(pending);
    let permanent = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::NECROMANCY)
        .expect("Necromancy entered after reconstruction")
        .card
        .id;
    assert_ne!(permanent, spell_id);
    assert_eq!(
        rebuilt.scheduled_triggers[0].capture.context.source_linked,
        Some(permanent),
        "entry completion binds the cleanup trigger to the resulting permanent",
    );

    rebuilt.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::Cleanup,
        player: PlayerId::One,
    });
    let captured = rebuilt
        .pending_triggers
        .iter()
        .find(|trigger| trigger.source.object == cast_source)
        .expect("the reconstructed cleanup listener triggers");
    assert_eq!(captured.context.source_linked, Some(permanent));
}
