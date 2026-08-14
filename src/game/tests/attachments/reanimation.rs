use super::*;

#[test]
fn animate_dead_hands_off_the_graveyard_id_and_sacrifices_on_leave() {
    let mut game = ready_game();
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    let animate = hand_card(&mut game, 20_080, cards::ANIMATE_DEAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Black, 2);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        animate,
        false,
        Some(Target::Card(graveyard_angel)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let animate = battlefield_id(&game, cards::ANIMATE_DEAD);
    assert_eq!(
        battlefield_permanent(&game, animate).attached_to,
        Some(graveyard_angel),
        "the Aura first enchants the graveyard incarnation",
    );
    assert_eq!(game.stack.len(), 1, "the reanimation is an ETB trigger");
    pass_priority_pair(&mut game);

    let angel = battlefield_id(&game, cards::SERRA_ANGEL);
    assert_ne!(
        angel, graveyard_angel,
        "a true zone change creates a new ID"
    );
    let animate_permanent = battlefield_permanent(&game, animate);
    assert_eq!(animate_permanent.attached_to, Some(angel));
    assert_eq!(animate_permanent.reanimation_linked, Some(angel));
    assert_eq!(animate_permanent.attachment_form, None);
    let angel_permanent = battlefield_permanent(&game, angel);
    assert_eq!(angel_permanent.controller, PlayerId::One);
    assert_eq!(game.power(&angel_permanent), Some(3));

    game.return_permanent_to_hand(animate);
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1, "leaving creates the sacrifice trigger");
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel),
    );
    assert!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
    );
}

#[test]
fn reanimation_leave_trigger_survives_ability_loss_and_can_be_countered() {
    let mut game = ready_game();
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    let animate = hand_card(&mut game, 20_085, cards::ANIMATE_DEAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Black, 2);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        animate,
        false,
        Some(Target::Card(graveyard_angel)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    let animate = battlefield_id(&game, cards::ANIMATE_DEAD);
    pass_priority_pair(&mut game);
    let angel = battlefield_id(&game, cards::SERRA_ANGEL);

    remove_all_abilities(&mut game, animate, 20_086);
    game.finish_rules_procedure();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != animate),
        "an Aura with no enchant ability is put into its owner's graveyard",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "the delayed sacrifice waits on the stack",
    );
    let leave = game
        .observe(PlayerId::One)
        .stack
        .last()
        .expect("the frozen leave trigger is stacked")
        .clone();
    assert_eq!(leave.kind, StackObjectKind::TriggeredAbility);
    assert_eq!(leave.source, Some(animate));
    assert_eq!(leave.controller, PlayerId::One);
    assert!(leave.counterable);

    game.counter_spell(leave.id);
    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "countering the delayed trigger leaves the returned creature alone",
    );
}

#[test]
fn reanimation_leave_trigger_survives_copy_and_tracks_the_exact_returned_object() {
    let mut game = ready_game();
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    let animate = hand_card(&mut game, 20_087, cards::ANIMATE_DEAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Black, 2);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        animate,
        false,
        Some(Target::Card(graveyard_angel)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    let animate = battlefield_id(&game, cards::ANIMATE_DEAD);
    pass_priority_pair(&mut game);
    let old_angel = battlefield_id(&game, cards::SERRA_ANGEL);
    let lions = setup_permanent(&mut game, PlayerId::One, cards::SAVANNAH_LIONS);

    let mut copy = spell_with_targets(
        20_088,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(lions)],
        0,
    );
    copy.source = Some(animate);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::BecomeCopyOf {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            retain_source_ability: false,
        }),
        &copy,
        TriggerContext::empty(),
    );
    game.finish_rules_procedure();
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == animate),
        "copying away Animate Dead's Aura subtype does not remove the source",
    );
    assert_eq!(battlefield_permanent(&game, animate).attached_to, None);
    assert!(game.stack.is_empty());

    game.return_permanent_to_hand(animate);
    game.finish_rules_procedure();
    let leave = game
        .observe(PlayerId::One)
        .stack
        .last()
        .expect("copying the source did not erase its frozen leave trigger")
        .clone();
    assert_eq!(leave.kind, StackObjectKind::TriggeredAbility);
    assert_eq!(leave.source, Some(animate));

    game.return_permanent_to_hand(old_angel);
    game.finish_rules_procedure();
    let hand_index = game.players[PlayerId::Two.index()]
        .hand
        .iter()
        .position(|card| card.definition == cards::SERRA_ANGEL)
        .expect("the old returned creature moved to its owner's hand");
    let angel_card = game.players[PlayerId::Two.index()].hand.remove(hand_index);
    game.put_card_onto_battlefield_from(angel_card, ZoneKind::Hand, PlayerId::One, None);
    game.finish_rules_procedure();
    let new_angel = battlefield_id(&game, cards::SERRA_ANGEL);
    assert_ne!(new_angel, old_angel);

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == new_angel),
        "the delayed trigger cannot sacrifice the new battlefield incarnation",
    );
}

#[test]
fn necromancy_targets_on_its_etb_and_marks_only_an_off_timing_cast() {
    let mut game = ready_game();
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    let necromancy = hand_card(&mut game, 20_090, cards::NECROMANCY, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Black, 3);
    let cast = matching_cast(&game, PlayerId::One, necromancy, false, None);
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let necromancy = battlefield_id(&game, cards::NECROMANCY);
    let ordinary = battlefield_permanent(&game, necromancy);
    assert_eq!(ordinary.attachment_form, None);
    assert_eq!(ordinary.attached_to, None);
    assert!(game.scheduled_triggers.is_empty());
    choose_object(&mut game, PlayerId::One, graveyard_angel);
    pass_priority_pair(&mut game);
    let angel = battlefield_id(&game, cards::SERRA_ANGEL);
    let permanent = battlefield_permanent(&game, necromancy);
    assert_eq!(permanent.reanimation_linked, Some(angel));
    assert_eq!(permanent.attachment_form, None);
    assert!(game.effective_subtypes(&permanent).contains(&"Aura"));

    let mut off_timing = ready_game();
    off_timing.priority = PlayerId::Two;
    let necromancy = hand_card(&mut off_timing, 20_091, cards::NECROMANCY, PlayerId::Two);
    fund(&mut off_timing, PlayerId::Two, ManaColor::Black, 3);
    let cast = matching_cast(&off_timing, PlayerId::Two, necromancy, false, None);
    off_timing.apply(PlayerId::Two, cast).unwrap();
    pass_priority_pair(&mut off_timing);
    let necromancy = battlefield_id(&off_timing, cards::NECROMANCY);
    assert_eq!(off_timing.scheduled_triggers.len(), 1);
    assert_eq!(
        off_timing.scheduled_triggers[0]
            .capture
            .context
            .source_linked,
        Some(necromancy),
        "the cleanup trigger binds the exact resulting permanent",
    );
}

#[test]
fn necromancys_subtype_effect_does_not_force_an_incompatible_copy_to_be_an_aura() {
    let mut game = ready_game();
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    let necromancy = hand_card(&mut game, 20_092, cards::NECROMANCY, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Black, 3);
    let cast = matching_cast(&game, PlayerId::One, necromancy, false, None);
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    let necromancy = battlefield_id(&game, cards::NECROMANCY);
    choose_object(&mut game, PlayerId::One, graveyard_angel);
    pass_priority_pair(&mut game);
    let angel = battlefield_id(&game, cards::SERRA_ANGEL);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == necromancy)
        .expect("Necromancy is present")
        .copy_effect = Some(copied_characteristics(cards::SAVANNAH_LIONS));
    game.check_state_based_actions();

    let copied = battlefield_permanent(&game, necromancy);
    assert_eq!(copied.reanimation_linked, Some(angel));
    assert!(copied.reanimation_effect.is_some());
    assert!(!game.is_aura_permanent(&copied));
    assert!(!game.effective_subtypes(&copied).contains(&"Aura"));
    assert_eq!(copied.attached_to, None);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == necromancy),
        "the incompatible copy detaches instead of being forced into an Aura form",
    );
}

fn setup_off_timing_necromancy(
    game: &mut Game,
    card_id: u32,
) -> (GameObjectId, GameObjectId, GameObjectId) {
    let graveyard_angel = game
        .put_into_graveyard(PlayerId::One, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    game.priority = PlayerId::Two;
    let cast_source = hand_card(game, card_id, cards::NECROMANCY, PlayerId::Two);
    fund(game, PlayerId::Two, ManaColor::Black, 3);
    let cast = matching_cast(game, PlayerId::Two, cast_source, false, None);
    game.apply(PlayerId::Two, cast).unwrap();
    pass_priority_pair(game);

    let necromancy = battlefield_id(game, cards::NECROMANCY);
    assert_ne!(cast_source, necromancy);
    assert_eq!(game.scheduled_triggers.len(), 1);
    assert_eq!(
        game.scheduled_triggers[0].capture.source.object,
        cast_source,
    );
    assert_eq!(
        game.scheduled_triggers[0].capture.context.source_linked,
        Some(necromancy),
    );
    choose_object(game, PlayerId::Two, graveyard_angel);
    pass_priority_pair(game);
    let angel = battlefield_id(game, cards::SERRA_ANGEL);
    let permanent = battlefield_permanent(game, necromancy);
    assert_eq!(permanent.reanimation_linked, Some(angel));
    assert_eq!(permanent.attachment_form, None);
    (cast_source, necromancy, angel)
}

fn start_necromancy_cleanup_trigger(
    game: &mut Game,
    necromancy: GameObjectId,
    delayed_source: GameObjectId,
) -> (u32, GameObjectId) {
    let turn = game.turn;
    game.step = Step::End;
    game.priority = game.active_player;
    pass_priority_pair(game);

    assert_eq!(game.turn, turn);
    assert_eq!(game.step, Step::Cleanup);
    assert_eq!(game.priority, game.active_player);
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == necromancy),
        "creating the delayed trigger does not sacrifice Necromancy",
    );
    let observation = game.observe(game.active_player);
    let trigger = observation
        .stack
        .last()
        .expect("cleanup trigger is stacked");
    assert_eq!(trigger.kind, StackObjectKind::TriggeredAbility);
    assert_eq!(trigger.source, Some(delayed_source));
    assert_eq!(trigger.controller, PlayerId::Two);
    assert!(trigger.counterable);
    (turn, trigger.id)
}

#[test]
fn countered_off_timing_necromancy_keeps_an_unbound_cleanup_trigger() {
    let mut game = ready_game();
    game.priority = PlayerId::Two;
    let source = hand_card(&mut game, 20_095, cards::NECROMANCY, PlayerId::Two);
    fund(&mut game, PlayerId::Two, ManaColor::Black, 3);
    let cast = matching_cast(&game, PlayerId::Two, source, false, None);
    game.apply(PlayerId::Two, cast).unwrap();

    assert_eq!(game.scheduled_triggers.len(), 1);
    assert_eq!(game.scheduled_triggers[0].capture.source.object, source);
    assert_eq!(game.scheduled_triggers[0].capture.controller, PlayerId::Two);
    assert_eq!(
        game.scheduled_triggers[0].capture.context.source_linked,
        None,
    );
    let spell = game.stack.last().expect("Necromancy is on the stack").id;
    game.counter_spell(spell);
    assert!(game.stack.is_empty());

    game.step = Step::End;
    game.priority = game.active_player;
    game.consecutive_passes = 0;
    pass_priority_pair(&mut game);
    let trigger = game
        .observe(PlayerId::One)
        .stack
        .last()
        .expect("the cast-created cleanup trigger is stacked")
        .clone();
    assert_eq!(trigger.source, Some(source));
    assert_eq!(trigger.controller, PlayerId::Two);
    assert!(trigger.counterable);

    pass_priority_pair(&mut game);
    assert!(
        game.stack.is_empty(),
        "the unbound trigger resolves as a no-op"
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::NECROMANCY),
    );
}

#[test]
fn necromancy_cleanup_trigger_uses_priority_then_repeats_cleanup() {
    let mut game = ready_game();
    let (delayed_source, necromancy, angel) = setup_off_timing_necromancy(&mut game, 20_096);
    let (turn, cleanup_trigger) =
        start_necromancy_cleanup_trigger(&mut game, necromancy, delayed_source);

    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::PassPriority),
        "the active player receives priority in cleanup",
    );
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    assert_eq!(game.priority, PlayerId::Two);
    let bolt = hand_card(&mut game, 20_097, cards::LIGHTNING_BOLT, PlayerId::Two);
    fund(&mut game, PlayerId::Two, ManaColor::Red, 1);
    let response = matching_cast(
        &game,
        PlayerId::Two,
        bolt,
        false,
        Some(Target::Player(PlayerId::One)),
    );
    let life_before = game.players[PlayerId::One.index()].life;
    game.apply(PlayerId::Two, response).unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, life_before - 3);
    assert_eq!(
        game.stack.last().map(|object| object.id),
        Some(cleanup_trigger)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == necromancy)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != necromancy)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "Necromancy's cleanup trigger only sacrifices Necromancy",
    );
    assert_eq!(
        game.stack.len(),
        1,
        "leaving creates a separate linked trigger"
    );
    let leave_trigger = game.observe(PlayerId::One).stack.pop().unwrap();
    assert_eq!(leave_trigger.kind, StackObjectKind::TriggeredAbility);
    assert_eq!(leave_trigger.source, Some(necromancy));
    assert!(leave_trigger.counterable);

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel)
    );
    assert!(game.stack.is_empty());
    assert_eq!(game.turn, turn);
    assert_eq!(game.step, Step::Cleanup);

    pass_priority_pair(&mut game);
    assert_eq!(game.turn, turn + 1);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
}

#[test]
fn necromancy_cleanup_and_leave_triggers_can_be_countered_independently() {
    let mut cleanup_countered = ready_game();
    let (delayed_source, necromancy, angel) =
        setup_off_timing_necromancy(&mut cleanup_countered, 20_098);
    let (turn, cleanup_trigger) =
        start_necromancy_cleanup_trigger(&mut cleanup_countered, necromancy, delayed_source);
    cleanup_countered.counter_spell(cleanup_trigger);
    assert!(cleanup_countered.stack.is_empty());
    assert!(
        cleanup_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == necromancy)
    );
    assert!(
        cleanup_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );
    pass_priority_pair(&mut cleanup_countered);
    assert_eq!(cleanup_countered.turn, turn + 1);
    assert!(
        cleanup_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == necromancy)
    );
    assert!(
        cleanup_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );

    let mut leave_countered = ready_game();
    let (delayed_source, necromancy, angel) =
        setup_off_timing_necromancy(&mut leave_countered, 20_099);
    let (turn, _) =
        start_necromancy_cleanup_trigger(&mut leave_countered, necromancy, delayed_source);
    pass_priority_pair(&mut leave_countered);
    assert!(
        leave_countered
            .battlefield
            .iter()
            .all(|permanent| permanent.card.id != necromancy)
    );
    assert!(
        leave_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );
    let leave_trigger = leave_countered
        .stack
        .last()
        .expect("leave trigger is stacked")
        .id;
    assert!(
        leave_countered
            .observe(PlayerId::One)
            .stack
            .last()
            .unwrap()
            .counterable
    );
    leave_countered.counter_spell(leave_trigger);
    assert!(leave_countered.stack.is_empty());
    assert!(
        leave_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );
    pass_priority_pair(&mut leave_countered);
    assert_eq!(leave_countered.turn, turn + 1);
    assert!(
        leave_countered
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel)
    );
}

#[test]
fn necromancy_cleanup_trigger_does_not_follow_a_returned_object() {
    let mut game = ready_game();
    let (delayed_source, necromancy, angel) = setup_off_timing_necromancy(&mut game, 20_100);

    game.return_permanent_to_hand(necromancy);
    game.finish_rules_procedure();
    let hand_index = game.players[PlayerId::Two.index()]
        .hand
        .iter()
        .position(|card| card.definition == cards::NECROMANCY)
        .expect("bounced Necromancy is in hand");
    let bounced = game.players[PlayerId::Two.index()].hand.remove(hand_index);
    game.put_card_onto_battlefield_from(bounced, ZoneKind::Hand, PlayerId::Two, None);
    game.finish_rules_procedure();
    let returned = battlefield_id(&game, cards::NECROMANCY);
    assert_ne!(returned, necromancy);
    assert_eq!(
        game.scheduled_triggers[0].capture.context.source_linked,
        Some(necromancy),
        "the delayed trigger does not follow the card's new incarnation",
    );
    assert_eq!(game.stack.len(), 1, "the original leave trigger is stacked");

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel)
    );
    assert!(game.stack.is_empty());

    let turn = game.turn;
    game.step = Step::End;
    game.priority = game.active_player;
    pass_priority_pair(&mut game);
    assert_eq!(game.turn, turn);
    assert_eq!(game.step, Step::Cleanup);
    assert_eq!(game.stack.len(), 1);
    let cleanup_trigger = game.observe(PlayerId::One).stack.pop().unwrap();
    assert_eq!(cleanup_trigger.kind, StackObjectKind::TriggeredAbility);
    assert_eq!(cleanup_trigger.source, Some(delayed_source));
    assert_eq!(cleanup_trigger.controller, PlayerId::Two);
    assert!(cleanup_trigger.counterable);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::NECROMANCY)
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![returned],
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::NECROMANCY)
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![returned],
        "the delayed trigger cannot sacrifice Necromancy's new object",
    );
    assert!(game.stack.is_empty());
    assert_eq!(game.turn, turn);
    pass_priority_pair(&mut game);
    assert_eq!(game.turn, turn + 1);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == returned)
    );
}

fn reconfigure_reanimated_battery_and_resolve_linked_sacrifice(
    game: &mut Game,
    aura: GameObjectId,
    battery: GameObjectId,
    host: GameObjectId,
) {
    let attach = matching_activation(game, PlayerId::One, battery, Some(Target::Permanent(host)))
        .expect("the reanimated Rabbit Battery may reconfigure");
    game.apply(PlayerId::One, attach).unwrap();
    pass_priority_pair(game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura),
        "the dynamic Aura is illegal as soon as its host stops being a creature",
    );
    let battery_permanent = battlefield_permanent(game, battery);
    assert!(matches!(
        battery_permanent.attachment_form,
        Some(AttachmentForm::Reconfigured { .. })
    ));
    assert_eq!(battery_permanent.attached_to, Some(host));
    assert_eq!(
        game.stack.len(),
        1,
        "the Aura's linked leave trigger is waiting to sacrifice the host",
    );

    pass_priority_pair(game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != battery),
        "the leave trigger follows the reanimated object's new battlefield ID",
    );
}

#[test]
fn reanimation_auras_die_and_sacrifice_when_their_host_loses_creature() {
    let mut animate_game = ready_game();
    let host = GameObjectId(20_092);
    animate_game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let graveyard_battery = animate_game
        .put_into_graveyard(PlayerId::Two, cards::RABBIT_BATTERY)
        .expect("Rabbit Battery is cataloged");
    let animate = hand_card(
        &mut animate_game,
        20_093,
        cards::ANIMATE_DEAD,
        PlayerId::One,
    );
    fund(&mut animate_game, PlayerId::One, ManaColor::Black, 2);
    fund(&mut animate_game, PlayerId::One, ManaColor::Red, 1);
    let cast = matching_cast(
        &animate_game,
        PlayerId::One,
        animate,
        false,
        Some(Target::Card(graveyard_battery)),
    );
    animate_game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut animate_game);
    let animate = battlefield_id(&animate_game, cards::ANIMATE_DEAD);
    pass_priority_pair(&mut animate_game);
    let battery = battlefield_id(&animate_game, cards::RABBIT_BATTERY);
    assert_ne!(battery, graveyard_battery);
    reconfigure_reanimated_battery_and_resolve_linked_sacrifice(
        &mut animate_game,
        animate,
        battery,
        host,
    );

    let mut necromancy_game = ready_game();
    let host = GameObjectId(20_094);
    necromancy_game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let graveyard_battery = necromancy_game
        .put_into_graveyard(PlayerId::Two, cards::RABBIT_BATTERY)
        .expect("Rabbit Battery is cataloged");
    let necromancy = hand_card(
        &mut necromancy_game,
        20_095,
        cards::NECROMANCY,
        PlayerId::One,
    );
    fund(&mut necromancy_game, PlayerId::One, ManaColor::Black, 3);
    fund(&mut necromancy_game, PlayerId::One, ManaColor::Red, 1);
    let cast = matching_cast(&necromancy_game, PlayerId::One, necromancy, false, None);
    necromancy_game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut necromancy_game);
    let necromancy = battlefield_id(&necromancy_game, cards::NECROMANCY);
    choose_object(&mut necromancy_game, PlayerId::One, graveyard_battery);
    pass_priority_pair(&mut necromancy_game);
    let battery = battlefield_id(&necromancy_game, cards::RABBIT_BATTERY);
    assert_ne!(battery, graveyard_battery);
    reconfigure_reanimated_battery_and_resolve_linked_sacrifice(
        &mut necromancy_game,
        necromancy,
        battery,
        host,
    );
}
