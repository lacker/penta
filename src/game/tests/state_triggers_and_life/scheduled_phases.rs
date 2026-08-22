#[test]
fn scheduled_turn_phase_sequences_are_ordered_nested_and_resume_the_displaced_phase() {
    static COMBAT: [TurnPhaseDef; 1] = [TurnPhaseDef::Combat];
    static COMBAT_AND_MAIN: [TurnPhaseDef; 2] =
        [TurnPhaseDef::Combat, TurnPhaseDef::PostcombatMain];

    let mut game = ready_game();
    let source = spell(40_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.step = Step::PostcombatMain;

    // A later schedule after the same phase runs before an earlier complete
    // sequence, without reversing that earlier sequence internally.
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT_AND_MAIN)),
        &source,
        TriggerContext::empty(),
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT)),
        &source,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.turn_phase_queue.iter().copied().collect::<Vec<_>>(),
        vec![
            TurnPhaseDef::Combat,
            TurnPhaseDef::Combat,
            TurnPhaseDef::PostcombatMain,
        ]
    );

    game.players[0].mana_pool = ManaPool {
        red: 1,
        ..ManaPool::default()
    };
    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert_eq!(
        game.turn_phase_resume,
        Some(TurnPhaseResume::Step(Step::End)),
        "a phase inserted after the postcombat main must resume at the end step"
    );

    // Scheduling from inside that inserted combat prepends work after the
    // combat now in progress, ahead of the older continuation.
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT_AND_MAIN)),
        &source,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.turn_phase_queue.iter().copied().collect::<Vec<_>>(),
        vec![
            TurnPhaseDef::Combat,
            TurnPhaseDef::PostcombatMain,
            TurnPhaseDef::Combat,
            TurnPhaseDef::PostcombatMain,
        ]
    );

    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == lions)
        .expect("Lions are on the battlefield");
    permanent.attacking = true;
    permanent.blocked = true;
    permanent.attacked_this_turn = true;
    permanent.attacks_this_turn = 1;
    permanent.damage_sources.push(source.card.id);
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        ..ManaPool::default()
    };

    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions)
        .expect("Lions remain on the battlefield");
    assert!(!permanent.attacking && !permanent.blocked);
    assert!(permanent.attacked_this_turn);
    assert_eq!(permanent.attacks_this_turn, 1);
    assert_eq!(permanent.damage_sources, vec![source.card.id]);

    // Nested combat, nested main, older combat, older main, then the frozen
    // ordinary continuation from the original postcombat main.
    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::PostcombatMain);
    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::PostcombatMain);
    game.advance_step();
    assert_eq!(game.step, Step::End);
    assert!(game.turn_phase_queue.is_empty());
    assert_eq!(game.turn_phase_resume, None);
}

#[test]
fn scheduled_turn_phases_from_precombat_resume_the_ordinary_combat() {
    static COMBAT_AND_MAIN: [TurnPhaseDef; 2] =
        [TurnPhaseDef::Combat, TurnPhaseDef::PostcombatMain];

    let mut game = ready_game();
    let source = spell(40_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.step = Step::PrecombatMain;
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT_AND_MAIN)),
        &source,
        TriggerContext::empty(),
    );

    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::PostcombatMain);
    game.advance_step();
    assert_eq!(
        game.step,
        Step::BeginningOfCombat,
        "the ordinary combat displaced by the inserted sequence still happens"
    );
    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::PostcombatMain);
}

#[test]
fn ordinary_and_inserted_combats_publish_the_same_beginning_step_trigger() {
    static COMBAT: [TurnPhaseDef; 1] = [TurnPhaseDef::Combat];

    let mut game = ready_game();
    game.battlefield.clear();
    let demon = game
        .put_onto_battlefield(PlayerId::One, cards::DESECRATION_DEMON)
        .expect("cataloged");
    let source = spell(40_003, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.step = Step::PrecombatMain;
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT)),
        &source,
        TriggerContext::empty(),
    );

    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);

    assert_eq!(
        game.events
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    GameEvent::AbilityTriggered { source, .. } if *source == demon
                )
            })
            .count(),
        2,
        "the inserted combat and displaced ordinary combat share the normal step event"
    );
}

#[test]
fn standalone_combat_after_postcombat_main_enters_the_end_step_normally() {
    static COMBAT: [TurnPhaseDef; 1] = [TurnPhaseDef::Combat];

    let mut game = ready_game();
    let source = spell(40_004, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let doomed = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == doomed)
        .expect("Lions are on the battlefield")
        .destroy_at_end = true;
    game.step = Step::PostcombatMain;
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT)),
        &source,
        TriggerContext::empty(),
    );

    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == doomed),
        "the end-step instruction waits through the inserted combat"
    );

    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.step, Step::End);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != doomed),
        "resuming at End runs the same end-step entry processing as ordinary progression"
    );
}

#[test]
fn a_phase_scheduled_during_the_ending_phase_precedes_the_next_turn() {
    static COMBAT: [TurnPhaseDef; 1] = [TurnPhaseDef::Combat];

    let mut game = ready_game();
    let source = spell(40_002, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == lions)
        .expect("Lions are on the battlefield");
    permanent.attacked_this_turn = true;
    permanent.attacks_this_turn = 1;
    permanent.damage_sources.push(source.card.id);
    game.sorcery_flash_grants[PlayerId::One.index()] = 1;
    let play_rule = AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::NoncreatureSpell,
    )));
    game.resolved_play_restrictions
        .push(ResolvedPlayRestriction {
            definition: play_rule,
            source: AbilitySourceRef {
                object: source.card.id,
                ability: AbilityOrigin::Printed {
                    definition: cards::AURELIAS_FURY,
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                },
            },
            affected_player: PlayerId::One,
            timestamp: ContinuousEffectTimestamp(40_003),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            restriction: PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                ObjectPredicateDef::NoncreatureSpell,
            ),
        });
    game.step = Step::End;
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::ScheduleTurnPhases(&COMBAT)),
        &source,
        TriggerContext::empty(),
    );

    game.advance_step();
    assert_eq!(game.turn, 1);
    assert_eq!(game.step, Step::BeginningOfCombat);
    assert_eq!(game.sorcery_flash_grants[PlayerId::One.index()], 0);
    assert!(game.resolved_play_restrictions.is_empty());
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions)
        .expect("Lions remain on the battlefield");
    assert!(permanent.attacked_this_turn);
    assert_eq!(permanent.attacks_this_turn, 1);
    assert_eq!(permanent.damage_sources, vec![source.card.id]);

    game.step = Step::EndOfCombat;
    game.advance_step();
    assert_eq!(game.turn, 2);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.step, Step::Upkeep);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions)
        .expect("Lions remain on the battlefield");
    assert!(!permanent.attacked_this_turn);
    assert_eq!(permanent.attacks_this_turn, 0);
    assert!(permanent.damage_sources.is_empty());
}
