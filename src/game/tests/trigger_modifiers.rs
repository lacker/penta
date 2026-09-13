//! Interactions between event suppression, extra occurrences and trigger observers.
use super::*;

fn put(game: &mut Game, player: PlayerId, definition: CardDefinitionId) -> GameObjectId {
    game.put_onto_battlefield(player, definition).unwrap()
}

fn counts(game: &Game, source: GameObjectId) -> usize {
    game.pending_triggers
        .iter()
        .filter(|trigger| trigger.source.object == source)
        .count()
}

#[test]
fn suppression_uses_the_entering_object_and_wins_over_additional_occurrences() {
    for suppressor in [
        cards::TORPOR_ORB,
        cards::HUSHWING_GRYFF_15,
        cards::DOORKEEPER_THRULL,
        cards::HUSHBRINGER_18,
    ] {
        let mut game = ready_game();
        put(&mut game, PlayerId::One, cards::TRAVELING_CHOCOBO);
        let warden = put(&mut game, PlayerId::One, cards::SOUL_WARDEN);
        game.pending_triggers.clear();
        put(&mut game, PlayerId::Two, suppressor);
        assert_eq!(
            counts(&game, warden),
            0,
            "the suppressor's own arrival: {suppressor:?}"
        );
        put(&mut game, PlayerId::One, cards::BIRDS_OF_PARADISE);
        assert_eq!(counts(&game, warden), 0);
        let courser = put(&mut game, PlayerId::One, cards::COURSER_OF_KRUPHIX);
        put(&mut game, PlayerId::One, cards::FOREST);
        assert_eq!(counts(&game, courser), 2, "ordinary lands still trigger");
    }
}

#[test]
fn elesh_norn_reads_the_ability_controller_independently_of_the_arrival_controller() {
    let mut game = ready_game();
    let ours = put(&mut game, PlayerId::One, cards::SOUL_WARDEN);
    let theirs = put(&mut game, PlayerId::Two, cards::SOUL_WARDEN);
    game.pending_triggers.clear();
    put(
        &mut game,
        PlayerId::One,
        cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
    );
    assert_eq!(counts(&game, ours), 2);
    assert_eq!(counts(&game, theirs), 0);
    game.pending_triggers.clear();
    put(&mut game, PlayerId::Two, cards::GRIZZLY_BEARS);
    assert_eq!(counts(&game, ours), 2);
    assert_eq!(counts(&game, theirs), 0);
    put(
        &mut game,
        PlayerId::Two,
        cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
    );
    game.pending_triggers.clear();
    put(&mut game, PlayerId::One, cards::GRIZZLY_BEARS);
    assert!(game.pending_triggers.is_empty());
}

#[test]
fn gandalf_uses_predeparture_sources_and_adds_once_for_artifact_legends() {
    let mut game = ready_game();
    let gandalf = put(&mut game, PlayerId::One, cards::GANDALF_THE_WHITE_305);
    let artist = put(&mut game, PlayerId::One, cards::BLOOD_ARTIST);
    let strix = put(&mut game, PlayerId::Two, cards::BALEFUL_STRIX);
    game.pending_triggers.clear();
    game.destroy_permanents(&[gandalf, artist, strix], false);
    assert_eq!(
        counts(&game, artist),
        5,
        "Gandalf and the artifact each add one, even as both sources die"
    );
}

#[test]
fn hushbringer_suppresses_a_simultaneous_death_but_does_not_suppress_exile() {
    let mut game = ready_game();
    let hush = put(&mut game, PlayerId::One, cards::HUSHBRINGER_18);
    let artist = put(&mut game, PlayerId::Two, cards::BLOOD_ARTIST);
    game.destroy_permanents(&[hush, artist], false);
    assert!(game.pending_triggers.is_empty());

    let mut game = ready_game();
    put(&mut game, PlayerId::One, cards::HUSHBRINGER_18);
    let sculler = put(&mut game, PlayerId::One, cards::TIDEHOLLOW_SCULLER);
    assert!(game.pending_triggers.is_empty());
    game.exile_permanent(sculler);
    assert_eq!(
        counts(&game, sculler),
        1,
        "leaving without dying still triggers"
    );
}

#[test]
fn entry_replacements_still_apply_with_suppression_and_multiplication() {
    let mut game = ready_game();
    put(&mut game, PlayerId::One, cards::TORPOR_ORB);
    put(
        &mut game,
        PlayerId::One,
        cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
    );
    let depths = put(&mut game, PlayerId::One, cards::DARK_DEPTHS);
    let permanent = game
        .battlefield
        .iter()
        .find(|p| p.card.id == depths)
        .unwrap();
    assert_eq!(permanent.counters(CounterKind::named("ice")), 10);
    let land = put(&mut game, PlayerId::One, cards::HEDGE_MAZE);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == land)
            .unwrap()
            .tapped
    );
}

#[test]
fn greenwarden_plays_graveyard_lands_and_stacks_additively_with_chocobo() {
    let mut game = ready_game();
    put(&mut game, PlayerId::One, cards::ANCIENT_GREENWARDEN);
    put(&mut game, PlayerId::One, cards::TRAVELING_CHOCOBO);
    let courser = put(&mut game, PlayerId::One, cards::COURSER_OF_KRUPHIX);
    let land = game
        .build_zone(PlayerId::One, &[cards::FOREST])
        .unwrap()
        .remove(0);
    let id = land.id;
    game.players[0].graveyard.push(land);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == id))
        .expect("a graveyard land has an ordinary land play");
    game.apply(PlayerId::One, action).unwrap();
    let triggered = game
        .events
        .iter()
        .filter(|event| {
            matches!(event,
        GameEvent::AbilityTriggered { source, .. } if *source == courser)
        })
        .count();
    assert_eq!(triggered, 3);
}

fn proctor_game(
    prepared: bool,
    controller: PlayerId,
    extras: bool,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    let proctor = put(&mut game, controller, cards::STRICT_PROCTOR);
    if extras {
        put(
            &mut game,
            PlayerId::Two,
            cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
        );
    }
    let strix = put(&mut game, PlayerId::Two, cards::BALEFUL_STRIX);
    (game, proctor, strix)
}

#[test]
fn proctor_observers_follow_both_apnap_groups_and_do_not_target() {
    for controller in [PlayerId::One, PlayerId::Two] {
        let (mut game, proctor, strix) = proctor_game(true, controller, true);
        assert_eq!(counts(&game, strix), 2);
        assert_eq!(counts(&game, proctor), 2, "observe each occurrence once");
        game.begin_trigger_placement();
        while let Some(decision) = game
            .pending_decisions
            .first()
            .map(|d| d.observation.clone())
        {
            assert_eq!(decision.kind, DecisionKind::TriggerOrder);
            game.choose_decision(
                decision.player,
                decision.id,
                &decision.options.iter().map(|o| o.id).collect::<Vec<_>>(),
            );
        }
        assert_eq!(game.stack.len(), 4);
        assert!(game.stack.iter().take(2).all(|o| o.source == Some(strix)));
        assert!(game.stack.iter().skip(2).all(|o| o.source == Some(proctor)));
        for observer in game.stack.iter().skip(2) {
            let ability = observer.ability.as_ref().unwrap();
            assert!(ability.targets.is_empty());
            assert!(
                game.stack
                    .iter()
                    .take(2)
                    .any(|base| Some(base.id) == ability.context.trigger.object)
            );
        }
    }
}

#[test]
fn proctor_counters_without_payment_and_prepared_matches_reference() {
    let run = |prepared| {
        let (mut game, _, _) = proctor_game(prepared, PlayerId::One, false);
        let hand = game.players[1].hand.len();
        drain_pending(&mut game);
        assert_eq!(
            game.players[1].hand.len(),
            hand,
            "the ETB draw was countered"
        );
        assert!(game.stack.is_empty());
        game
    };
    let prepared = run(true);
    let reference = run(false);
    assert_eq!(prepared.players, reference.players);
    assert_eq!(prepared.events, reference.events);
}

#[test]
fn suppressed_abilities_do_not_trigger_proctor() {
    let (mut game, proctor, _) = proctor_game(true, PlayerId::One, false);
    game.pending_triggers.clear();
    put(&mut game, PlayerId::One, cards::TORPOR_ORB);
    put(&mut game, PlayerId::Two, cards::BALEFUL_STRIX);
    assert_eq!(counts(&game, proctor), 0);
    assert!(game.pending_triggers.is_empty());
}

#[test]
fn simultaneous_entries_see_suppressors_and_late_arriving_type_effects() {
    let mut game = ready_game();
    let watcher = put(&mut game, PlayerId::One, cards::SOUL_WARDEN);
    game.entering_together(|game| {
        put(game, PlayerId::One, cards::GRIZZLY_BEARS);
        put(game, PlayerId::Two, cards::DOORKEEPER_THRULL);
    });
    assert_eq!(counts(&game, watcher), 0);

    let mut game = ready_game();
    put(&mut game, PlayerId::One, cards::ANCIENT_GREENWARDEN);
    let watcher = put(&mut game, PlayerId::One, cards::SOUL_WARDEN);
    game.pending_triggers.clear();
    game.entering_together(|game| {
        put(game, PlayerId::One, cards::GRIZZLY_BEARS);
        put(game, PlayerId::One, cards::ASHAYA_SOUL_OF_THE_WILD_179);
    });
    assert_eq!(
        counts(&game, watcher),
        4,
        "both creatures are lands when the batch triggers"
    );
}

#[test]
fn death_suppression_preserves_the_new_cards_own_from_anywhere_trigger() {
    for simultaneous in [false, true] {
        let mut game = ready_game();
        let hush = put(&mut game, PlayerId::One, cards::HUSHBRINGER_18);
        let wurm = put(&mut game, PlayerId::One, cards::WORLDSPINE_WURM);
        let deaths = if simultaneous {
            vec![hush, wurm]
        } else {
            vec![wurm]
        };
        game.destroy_permanents(&deaths, false);
        assert_eq!(
            game.pending_triggers.len(),
            1,
            "only the graveyard shuffle triggers"
        );
        assert_ne!(
            game.pending_triggers[0].source.object, wurm,
            "the new card supplies the trigger"
        );
        assert_eq!(
            counts(&game, wurm),
            0,
            "the death token trigger is suppressed"
        );
    }
}

#[test]
fn additional_occurrences_respect_per_turn_limits() {
    const ABILITIES: &[AbilityDef] = &[AbilityDef::triggered(
        "Whenever a creature enters, you gain 1 life. This ability triggers only once each turn.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )
    .triggering_at_most(1)];
    let mut game = ready_game();
    put(
        &mut game,
        PlayerId::One,
        cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
    );
    let watcher = game.create_token_from(
        PlayerId::One,
        crate::card::TokenCharacteristics::creature(&[], &[], 1, 1).with_abilities(ABILITIES),
        None,
    );
    assert_eq!(counts(&game, watcher), 1);
    put(&mut game, PlayerId::One, cards::GRIZZLY_BEARS);
    assert_eq!(counts(&game, watcher), 1);
}

#[test]
fn grouped_deaths_add_one_occurrence_per_modifier() {
    const ABILITIES: &[AbilityDef] = &[AbilityDef::triggered(
        "Whenever one or more creatures die, you gain 1 life.",
        TriggerEventDef::ObjectsDied {
            object: ObjectPredicateDef::HasType(CardType::Creature),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let mut game = ready_game();
    let gandalf = put(&mut game, PlayerId::One, cards::GANDALF_THE_WHITE_305);
    let watcher = game.create_token_from(
        PlayerId::One,
        crate::card::TokenCharacteristics::creature(&[], &[], 1, 1).with_abilities(ABILITIES),
        None,
    );
    let artifact = game.create_token_from(
        PlayerId::Two,
        crate::card::TokenCharacteristics::creature(&[], &[], 1, 1)
            .with_type(CardType::Artifact)
            .with_supertype(CardSupertype::Legendary),
        None,
    );
    game.destroy_permanents(&[gandalf, watcher, artifact], false);
    assert_eq!(
        counts(&game, watcher),
        2,
        "several qualifying deaths are one grouped cause"
    );
}

#[test]
fn proctor_payments_are_independent_and_belong_to_the_original_controller() {
    let (mut game, _, _) = proctor_game(true, PlayerId::One, true);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 2);
    let hand = game.players[1].hand.len();
    let mut payments = 0;
    for _ in 0..40 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = if matches!(pending.continuation, DecisionContinuation::PayOr { .. }) {
                assert_eq!(decision.player, PlayerId::Two);
                payments += 1;
                vec![
                    decision
                        .options
                        .iter()
                        .find(|o| o.label.starts_with("Pay "))
                        .expect("two mana can pay once")
                        .id,
                ]
            } else {
                decision
                    .options
                    .iter()
                    .take(decision.maximum)
                    .map(|o| o.id)
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        } else {
            let player = game.priority;
            game.apply(player, Action::PassPriority).unwrap();
        }
    }
    assert_eq!(
        payments, 1,
        "the second occurrence is countered when no mana remains"
    );
    assert_eq!(game.players[1].hand.len(), hand + 1);
}

#[test]
fn thrull_suppresses_artifact_entries_that_torpor_orb_allows() {
    for (suppressor, expected) in [(cards::TORPOR_ORB, 1), (cards::DOORKEEPER_THRULL, 0)] {
        let mut game = ready_game();
        put(&mut game, PlayerId::One, suppressor);
        let wellspring = put(&mut game, PlayerId::Two, cards::ICHOR_WELLSPRING);
        assert_eq!(counts(&game, wellspring), expected);
    }
}

#[test]
fn gandalf_grants_flash_to_legendary_and_artifact_spells() {
    let mut game = ready_game();
    put(&mut game, PlayerId::One, cards::GANDALF_THE_WHITE_305);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 10);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 10);
    for (definition, allowed) in [
        (cards::SOL_RING, true),
        (cards::THALIA_GUARDIAN_OF_THRABEN, true),
        (cards::GRIZZLY_BEARS, false),
    ] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .unwrap()
            .remove(0);
        let id = card.id;
        game.players[0].hand.push(card);
        assert_eq!(
            game.legal_actions(PlayerId::One)
                .iter()
                .any(|action| { matches!(action, Action::CastSpell { card, .. } if *card == id) }),
            allowed
        );
    }
}

#[test]
fn a_suppressed_delayed_trigger_waits_and_is_not_multiplied_as_a_permanent_ability() {
    let mut game = ready_game();
    put(
        &mut game,
        PlayerId::One,
        cards::ELESH_NORN_MOTHER_OF_MACHINES_416,
    );
    let warden = put(&mut game, PlayerId::One, cards::SOUL_WARDEN);
    let listener = game
        .battlefield_trigger_listeners()
        .into_iter()
        .find(|l| l.capture.source.object == warden)
        .unwrap();
    let id = game.next_installed_trigger_id;
    game.next_installed_trigger_id += 1;
    game.installed_triggers.push(InstalledTrigger {
        id,
        event: listener.event,
        capture: listener.capture,
        lifetime: InstalledTriggerLifetime::Once,
    });
    let orb = put(&mut game, PlayerId::One, cards::TORPOR_ORB);
    game.pending_triggers.clear();
    put(&mut game, PlayerId::One, cards::GRIZZLY_BEARS);
    assert!(game.pending_triggers.is_empty());
    assert_eq!(
        game.installed_triggers.len(),
        1,
        "suppression does not consume a delayed trigger"
    );
    game.exile_permanent(orb);
    put(&mut game, PlayerId::One, cards::GRIZZLY_BEARS);
    assert_eq!(
        counts(&game, warden),
        3,
        "two permanent occurrences and one delayed occurrence"
    );
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn proctor_observation_survives_its_source_leaving_before_placement() {
    let (mut game, proctor, _) = proctor_game(true, PlayerId::One, false);
    game.destroy_permanent(proctor);
    assert_eq!(counts(&game, proctor), 1);
    let hand = game.players[1].hand.len();
    drain_pending(&mut game);
    assert_eq!(game.players[1].hand.len(), hand);
}
