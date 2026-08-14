use super::*;

#[test]
fn living_weapon_attaches_before_sbas_and_a_missing_source_leaves_no_germ() {
    let mut game = ready_game();
    let dreadmask = setup_permanent(&mut game, PlayerId::One, cards::COLOSSAL_DREADMASK);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GERM_TOKEN_0_0_BLACK),
        "living weapon is a trigger, not an entry replacement",
    );
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);

    let germ = battlefield_id(&game, cards::GERM_TOKEN_0_0_BLACK);
    assert_eq!(
        battlefield_permanent(&game, dreadmask).attached_to,
        Some(germ),
    );
    let germ = battlefield_permanent(&game, germ);
    assert_eq!(game.power(&germ), Some(6));
    assert_eq!(game.toughness(&germ), Some(6));
    assert!(game.permanent_has_executable_keyword(&germ, KeywordAbility::Trample));

    let mut source_gone = ready_game();
    let dreadmask = setup_permanent(&mut source_gone, PlayerId::One, cards::COLOSSAL_DREADMASK);
    assert_eq!(source_gone.stack.len(), 1);
    source_gone.return_permanent_to_hand(dreadmask);
    source_gone.finish_rules_procedure();
    pass_priority_pair(&mut source_gone);
    assert!(
        source_gone
            .battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GERM_TOKEN_0_0_BLACK),
        "without the Equipment the unattached 0/0 Germ dies at the first SBA check",
    );
}

#[test]
fn rabbit_battery_reconfigures_with_the_right_timing_types_and_bonuses() {
    let mut game = ready_game();
    let battery = GameObjectId(20_060);
    let host = GameObjectId(20_061);
    let mut battery_permanent = super::creature(battery.0, cards::RABBIT_BATTERY, PlayerId::One);
    battery_permanent.entered_controller_turn = 0;
    game.battlefield.extend([
        battery_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::Red, 2);

    game.step = Step::Upkeep;
    assert!(
        matching_activation(&game, PlayerId::One, battery, Some(Target::Permanent(host)),)
            .is_none(),
        "reconfigure is sorcery speed",
    );
    game.step = Step::PrecombatMain;
    assert!(
        matching_activation(&game, PlayerId::One, battery, None).is_none(),
        "the unattach ability is unavailable while the Battery is unattached",
    );
    let attach = matching_activation(&game, PlayerId::One, battery, Some(Target::Permanent(host)))
        .expect("the attach half of reconfigure is legal");
    game.apply(PlayerId::One, attach).unwrap();
    pass_priority_pair(&mut game);

    let attached = battlefield_permanent(&game, battery);
    assert!(matches!(
        attached.attachment_form,
        Some(AttachmentForm::Reconfigured { .. })
    ));
    assert_eq!(attached.attached_to, Some(host));
    assert!(
        !game
            .permanent_types(&attached)
            .expect("Battery has types")
            .contains(CardType::Creature),
    );
    assert_eq!(game.effective_subtypes(&attached).as_ref(), &["Equipment"]);
    let equipped = battlefield_permanent(&game, host);
    assert_eq!(game.power(&equipped), Some(3));
    assert_eq!(game.toughness(&equipped), Some(2));
    assert!(game.permanent_has_executable_keyword(&equipped, KeywordAbility::Haste));

    let unattach = matching_activation(&game, PlayerId::One, battery, None)
        .expect("the unattach half of reconfigure is legal while attached");
    game.apply(PlayerId::One, unattach).unwrap();
    pass_priority_pair(&mut game);
    let restored = battlefield_permanent(&game, battery);
    assert_eq!(restored.attachment_form, None);
    assert_eq!(restored.attached_to, None);
    assert!(
        game.permanent_types(&restored)
            .expect("Battery has types")
            .contains(CardType::Creature),
    );
    assert_eq!(
        game.effective_subtypes(&restored).as_ref(),
        &["Equipment", "Rabbit"],
    );
    assert!(game.permanent_has_executable_keyword(&restored, KeywordAbility::Haste));
    let host = battlefield_permanent(&game, host);
    assert_eq!(game.power(&host), Some(2));
    assert!(!game.permanent_has_executable_keyword(&host, KeywordAbility::Haste));
}

#[test]
fn reconfigure_and_animation_type_changes_follow_timestamp_order() {
    let mut game = ready_game();
    let battery = GameObjectId(20_068);
    let host = GameObjectId(20_069);
    let mut battery_permanent = super::creature(battery.0, cards::RABBIT_BATTERY, PlayerId::One);
    let earlier = game.allocate_continuous_effect_timestamp();
    battery_permanent.animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_CREATURE_ANIMATION,
        timestamp: earlier,
    });
    game.battlefield.extend([
        battery_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);

    assert!(game.try_attach(battery, host));
    let reconfigured = battlefield_permanent(&game, battery);
    assert!(
        !game
            .permanent_types(&reconfigured)
            .expect("Battery has types")
            .contains(CardType::Creature),
        "the later reconfigure operation removes an earlier animation's creature type",
    );

    let later = game.allocate_continuous_effect_timestamp();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == battery)
        .expect("Battery is present")
        .animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_CREATURE_ANIMATION,
        timestamp: later,
    });
    assert!(
        game.permanent_types(&battlefield_permanent(&game, battery))
            .expect("Battery has types")
            .contains(CardType::Creature),
        "a later animation restores the creature type",
    );
    game.check_state_based_actions();
    let detached = battlefield_permanent(&game, battery);
    assert_eq!(detached.attached_to, None);
    assert_eq!(detached.attachment_form, None);
}

#[test]
fn reconfigured_host_swap_rechecks_current_reconfigure_ability() {
    let mut game = ready_game();
    let battery = GameObjectId(20_064);
    let first_host = GameObjectId(20_065);
    let second_host = GameObjectId(20_066);
    game.battlefield.extend([
        super::creature(battery.0, cards::RABBIT_BATTERY, PlayerId::One),
        super::creature(first_host.0, cards::SAVANNAH_LIONS, PlayerId::One),
        super::creature(second_host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    assert!(game.try_attach(battery, first_host));
    assert!(matches!(
        battlefield_permanent(&game, battery).attachment_form,
        Some(AttachmentForm::Reconfigured { .. })
    ));

    remove_all_abilities(&mut game, battery, 20_067);
    assert!(
        game.effective_abilities(&battlefield_permanent(&game, battery))
            .is_empty()
    );
    assert!(game.try_attach(battery, second_host));
    let moved = battlefield_permanent(&game, battery);
    assert_eq!(moved.attachment_form, None);
    assert_eq!(moved.attached_to, Some(second_host));
    assert!(
        game.permanent_types(&moved)
            .expect("moved Battery has types")
            .contains(CardType::Creature),
    );

    game.check_state_based_actions();
    assert_eq!(battlefield_permanent(&game, battery).attached_to, None);
    assert!(
        !game.try_attach(battery, first_host),
        "an unattached creature Equipment cannot attach after losing reconfigure",
    );
}

#[test]
fn reconfigured_equipment_ignores_lethal_creature_damage_until_it_unattaches() {
    let mut game = ready_game();
    let battery = GameObjectId(20_062);
    let host = GameObjectId(20_063);
    let mut damaged_battery = super::creature(battery.0, cards::RABBIT_BATTERY, PlayerId::One);
    damaged_battery.damage = 1;
    game.battlefield.extend([
        damaged_battery,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);

    assert!(game.try_attach(battery, host));
    game.check_state_based_actions();
    let attached = battlefield_permanent(&game, battery);
    assert!(matches!(
        attached.attachment_form,
        Some(AttachmentForm::Reconfigured { .. })
    ));
    assert_eq!(game.toughness(&attached), None);

    assert!(game.unattach(battery));
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != battery),
        "the marked damage is lethal again as soon as Battery is a creature",
    );
}

#[test]
fn darksteel_garrison_fortifies_and_its_land_tap_trigger_resolves() {
    let mut game = ready_game();
    let garrison = GameObjectId(20_070);
    let land = GameObjectId(20_071);
    let target = GameObjectId(20_072);
    game.battlefield.extend([
        super::creature(garrison.0, cards::DARKSTEEL_GARRISON, PlayerId::One),
        super::creature(land.0, cards::MOUNTAIN, PlayerId::One),
        super::creature(target.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::Colorless, 3);
    let fortify = matching_activation(
        &game,
        PlayerId::One,
        garrison,
        Some(Target::Permanent(land)),
    )
    .expect("fortify is legal in the main phase");
    game.apply(PlayerId::One, fortify).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        battlefield_permanent(&game, garrison).attached_to,
        Some(land),
    );
    let fortified = battlefield_permanent(&game, land);
    assert!(game.has_indestructible(&fortified));

    assert!(game.tap_permanent(land).is_some());
    game.finish_rules_procedure();
    choose_object(&mut game, PlayerId::One, target);
    pass_priority_pair(&mut game);
    let pumped = battlefield_permanent(&game, target);
    assert_eq!(game.power(&pumped), Some(3));
    assert_eq!(game.toughness(&pumped), Some(2));
}
