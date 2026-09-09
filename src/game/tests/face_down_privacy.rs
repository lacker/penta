use super::*;

#[test]
fn face_down_privacy_subtlety_hides_target_identity_in_public_decisions() {
    let mut game = ready_game();
    let held = game
        .build_zone(PlayerId::One, &[cards::KROSAN_COLOSSUS])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let subtlety = game
        .build_zone(PlayerId::Two, &[cards::SUBTLETY])
        .unwrap()
        .remove(0);
    let subtlety_id = subtlety.id;
    game.players[1].hand.push(subtlety);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 4);
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == subtlety_id && choices.costs().alternative().is_none())
        })
        .unwrap();
    game.apply(PlayerId::Two, cast).unwrap();
    pass_priority_pair(&mut game);
    let observed = game.observe(PlayerId::Two);
    let decision = observed.decision.unwrap();
    assert_eq!(decision.minimum, 0);
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some())
        .unwrap();
    assert_eq!(option.label, "Face-down creature");
    assert!(option.card.unwrap().1.card_definition().is_none());
    let owner_view = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(owner_view.options, decision.options);
    let target = Target::Spell(
        game.stack
            .iter()
            .find(|object| object.face_down.is_some())
            .unwrap()
            .id,
    );
    let effect_option = game.effect_target_option(0, target);
    assert_eq!(effect_option.label, option.label);
    assert_eq!(effect_option.card, option.card);
    assert_eq!(game.effect_target_card(target), option.card);
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option.id],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let choice = game.observe(PlayerId::Two).decision.unwrap();
    assert_eq!(
        choice.prompt,
        "Put Face-down creature on the top or bottom of your library"
    );
}

#[test]
fn face_down_privacy_checkpoint_withholds_live_and_retired_morph_identity() {
    let mut game = ready_game();
    let held = game
        .build_zone(PlayerId::One, &[cards::KROSAN_COLOSSUS])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    let observation = game.observe(PlayerId::Two);
    assert!(
        observation.stack[0]
            .characteristics
            .card_definition()
            .is_none()
    );
    assert!(observation.stack[0].signature.is_none());
    assert_eq!(
        observation.checkpoint["unavailableReason"],
        "hiddenFaceDownObjects"
    );
    assert!(observation.checkpoint.get("stack").is_none());
    assert_eq!(observation.checkpoint.as_object().unwrap().len(), 5);
    pass_priority_pair(&mut game);
    let observation = game.observe(PlayerId::Two);
    assert!(
        observation.battlefield[0]
            .characteristics
            .card_definition()
            .is_none()
    );
    assert_eq!(
        observation.checkpoint["unavailableReason"],
        "hiddenFaceDownObjects"
    );
    assert!(observation.checkpoint.get("battlefield").is_none());
    assert!(observation.checkpoint.get("retiredObjects").is_none());
    // Retired spell history still contains the hidden face after the live
    // object turns up; it must never bypass checkpoint privacy.
    game.battlefield[0].face_down = None;
    assert_eq!(
        game.observe(PlayerId::Two).checkpoint["unavailableReason"],
        "hiddenFaceDownObjects"
    );
    game.spell_cast_history_this_turn.clear();
    assert!(
        game.observe(PlayerId::Two)
            .checkpoint
            .get("unavailableReason")
            .is_none(),
        "unreferenced retired objects need not block later rollouts"
    );
}

#[test]
fn face_down_privacy_transforming_permanent_has_zero_mana_value() {
    let mut game = ready_game();
    let mut masked = creature(870_101, cards::HUNTMASTER_OF_THE_FELLS, PlayerId::One);
    masked.face_down = Some(crate::card::face_down::manifest());
    masked.turn_up_for_mana_cost = true;
    game.battlefield.push(masked);
    assert_eq!(game.permanent_mana_value(&game.battlefield[0]), 0);
    assert_eq!(
        game.effective_rules(&game.battlefield[0])
            .unwrap()
            .printed_mana_cost()
            .mana_value(),
        0
    );
    let decay = card(870_102, cards::ABRUPT_DECAY, PlayerId::Two);
    let decay_id = decay.id;
    game.players[1].hand.push(decay);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Green, 1);
    game.priority = PlayerId::Two;
    assert!(game.legal_actions(PlayerId::Two).iter().any(|action| {
        matches!(action, Action::CastSpell { card, choices, .. }
            if *card == decay_id && choices.iter_targets().any(|target|
                *target == Target::Permanent(CardInstanceId(870_101))))
    }));
}

#[test]
fn face_down_privacy_printed_uncounterability_is_masked() {
    let mut game = ready_game();
    let mut masked = spell(870_201, cards::THRUN_THE_LAST_TROLL, PlayerId::One, 0);
    masked.face_down = Some(crate::card::face_down::ordinary());
    game.stack.push(masked);
    assert!(game.can_be_countered(&game.stack[0]));
    assert!(!game.object_has_ability(game.stack[0].id, crate::card::AbilityPredicateDef::Any));
    let observation = game.observe(PlayerId::Two);
    assert!(
        observation.stack[0]
            .characteristics
            .card_definition()
            .is_none()
    );
    assert!(observation.stack[0].counterable);
    game.stack[0].applied_effects.push(AppliedStackEffect {
        source: None,
        granting: None,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
    });
    assert!(
        !game.can_be_countered(&game.stack[0]),
        "external effects still apply"
    );
}

#[test]
fn face_down_privacy_exiled_card_has_no_printed_characteristics() {
    let mut game = ready_game();
    let held = game
        .build_zone(PlayerId::One, &[cards::DELAYED_BLAST_FIREBALL])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(PlayerId::One, Action::Foretell { card: id })
        .unwrap();
    let exiled = &game.players[0].exile[0];
    assert!(game.exiled_card_is_face_down(exiled.id));
    assert!(game.observe(PlayerId::Two).exiles[0].is_empty());
    assert!(!game.card_object_matches(
        ObjectPredicateDef::HasType(CardType::Instant),
        exiled,
        ZoneKind::Exile,
        CardInstanceId(870_302)
    ));
    for predicate in [
        ObjectPredicateDef::GenericManaCostAtMost(0),
        ObjectPredicateDef::NameEquals(crate::card::CardNameDef::Literal("Delayed Blast Fireball")),
    ] {
        assert!(!game.card_object_matches(predicate, exiled, ZoneKind::Exile, exiled.id));
    }
    assert!(game.card_object_matches(ObjectPredicateDef::Any, exiled, ZoneKind::Exile, exiled.id));
    assert!(game.card_object_matches(
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        exiled,
        ZoneKind::Exile,
        exiled.id
    ));
    assert_eq!(game.current_or_last_known_mana_value(exiled.id), Some(0));
    assert!(game.object_card_name(exiled.id).is_none());
    assert_eq!(
        game.target_label(PlayerId::One, Target::Card(exiled.id)),
        "Face-down card"
    );
    assert!(game.target_card(Target::Card(exiled.id)).is_none());
    let option = game.effect_target_option(0, Target::Card(exiled.id));
    assert_eq!(option.label, "Face-down card");
    assert!(option.card.is_none());
}

#[test]
fn face_down_privacy_public_observations_do_not_depend_on_morph_identity() {
    let mut observations = Vec::new();
    for definition in [cards::KROSAN_COLOSSUS, cards::EXALTED_ANGEL] {
        let mut game = ready_game();
        let held = game
            .build_zone(PlayerId::One, &[definition])
            .unwrap()
            .remove(0);
        let id = held.id;
        game.players[0].hand.push(held);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
            .unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        for stage in 0..2 {
            if stage == 1 {
                pass_priority_pair(&mut game);
            }
            observations.push(crate::protocol::observation_json_for_format(
                &game.catalog,
                game.format,
                &game.observe(PlayerId::Two),
                false,
                &game.legal_actions(PlayerId::Two),
            ));
        }
    }
    assert_eq!(observations[0], observations[2], "stack observations");
    assert_eq!(observations[1], observations[3], "battlefield observations");
}

#[test]
fn face_down_privacy_masks_printed_spell_types_names_and_cast_triggers() {
    let mut game = ready_game();
    let mut masked = spell(870_400, cards::BRAIN_FREEZE, PlayerId::One, 0);
    masked.face_down = Some(crate::card::face_down::ordinary());
    masked.cast = Some(CastContext::for_cast(
        CastSourceZone::Hand,
        None,
        false,
        &game.catalog.get(cards::BRAIN_FREEZE).unwrap().play_options[0],
        masked.signature.as_ref().unwrap(),
        false,
    ));
    let id = masked.id;
    game.stack.push(masked);
    assert_eq!(
        game.stack_spell_types(&game.stack[0]),
        Some(CardTypeSet::single(CardType::Creature))
    );
    assert!(game.object_card_name(id).is_none());
    game.capture_own_cast_triggers(id);
    assert!(
        game.pending_triggers.is_empty(),
        "printed storm is absent while face down"
    );
}
