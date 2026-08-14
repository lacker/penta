use super::*;

static MUTAVAULT_TEST_ANIMATION: crate::card::AnimationDef =
    crate::card::AnimationDef::new(2, 2).with_all_creature_types();

#[test]
fn an_empty_library_draw_waits_for_state_based_actions_and_resolution_continues() {
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    ];

    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].life = 10;
    let source = spell(10_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&EFFECTS)),
        &source,
        TriggerContext::empty(),
    );

    assert_eq!(
        game.players[0].life, 13,
        "the rest of the resolving effect happened after the failed draw"
    );
    assert_eq!(
        game.result, None,
        "a failed draw does not end the game itself"
    );

    game.check_state_based_actions();
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
}

#[test]
fn simultaneous_player_loss_conditions_with_different_causes_make_a_draw() {
    let mut game = ready_game();
    game.players[0].library.clear();

    assert_eq!(game.draw_card(PlayerId::One), None);
    game.players[1].life = 0;
    assert_eq!(game.result, None);

    game.check_state_based_actions();

    assert_eq!(game.result, Some(GameResult::Draw));
}

#[test]
fn a_state_trigger_fires_when_its_condition_becomes_true_and_only_once() {
    let mut game = ready_game();
    game.battlefield.clear();
    let goblins = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLINS_OF_THE_FLARG)
        .expect("cataloged");
    game.check_state_based_actions();
    assert!(
        game.pending_triggers.is_empty(),
        "no Dwarf, so the condition is false and nothing triggers"
    );

    // No Dwarf is printed in the catalog yet, but an animated Mutavault is a
    // creature with every creature type, so it is one.
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::MUTAVAULT)
        .expect("cataloged");
    game.check_state_based_actions();
    assert!(
        game.pending_triggers.is_empty(),
        "an unanimated Mutavault is only a land"
    );
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == vault)
    {
        permanent.animation = Some(ResolvedAnimation {
            definition: &MUTAVAULT_TEST_ANIMATION,
            timestamp: permanent.timestamp,
        });
    }
    game.check_state_based_actions();
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "controlling a Dwarf makes the condition true"
    );

    // CR 603.8: it does not trigger again while it is already waiting.
    game.check_state_based_actions();
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "a state trigger already waiting does not stack up"
    );

    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == goblins),
        "the Goblins sacrificed themselves"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GOBLINS_OF_THE_FLARG),
        "and went to the graveyard"
    );
}

#[test]
fn disciple_of_bolas_pays_out_the_power_of_what_it_ate() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library = (0..5)
        .map(|index| card(12_000 + index, cards::PLAINS, PlayerId::One))
        .collect();
    game.players[0].hand.clear();
    // A 5/5 and a 2/1, so the choice is visible in the payout.
    game.put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let life_before = game.players[0].life;

    let disciple = game
        .put_onto_battlefield(PlayerId::One, cards::DISCIPLE_OF_BOLAS)
        .expect("cataloged");
    for _ in 0..12 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the sacrifice is a choice");
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.card.is_none_or(|(id, _)| id != disciple)),
        "\"another creature\" excludes the Disciple itself"
    );
    let angel = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, def)| def == cards::SERRA_ANGEL)
        })
        .expect("the Angel is a legal sacrifice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel.id],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    // Serra Angel is a 4/4, so four life and four cards.
    assert_eq!(game.players[0].life, life_before + 4);
    assert_eq!(game.players[0].hand.len(), 4);
    assert_eq!(game.players[0].library.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions),
        "only the chosen creature was sacrificed"
    );
}

#[test]
fn zealous_conscripts_borrows_a_permanent_and_gives_it_back_at_cleanup() {
    let mut game = ready_game();
    game.battlefield.clear();
    let stolen = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.tap_permanent(stolen);
    // It has been theirs all along, so only the granted haste lets it attack.
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == stolen)
    {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 2];

    game.put_onto_battlefield(PlayerId::One, cards::ZEALOUS_CONSCRIPTS)
        .expect("cataloged");
    for _ in 0..12 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    drain_pending(&mut game);

    let borrowed = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stolen)
        .expect("the permanent is still on the battlefield");
    assert_eq!(borrowed.controller, PlayerId::One, "control changed");
    assert!(!borrowed.tapped, "and it was untapped");
    assert!(
        game.permanent_has_executable_keyword(borrowed, KeywordAbility::Haste),
        "and it can attack this turn"
    );
    // Gaining control restarts summoning sickness, so the granted haste is
    // doing real work rather than restating what was already true.
    assert_eq!(
        borrowed.entered_controller_turn,
        game.turns_started[PlayerId::One.index()],
        "it counts as newly under its new controller's control"
    );
    let borrowed = borrowed.clone();
    assert!(
        game.can_attack(&borrowed),
        "with haste it can attack the turn it changes hands"
    );
    let mut without_haste = borrowed;
    without_haste.temporary_granted_abilities.retain(|grant| {
        !matches!(
            grant.ability.definition,
            DeclarativeAbilityDef::Keyword(KeywordAbility::Haste)
        )
    });
    assert!(
        !game.can_attack(&without_haste),
        "and without haste it could not"
    );

    game.cleanup();
    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stolen)
        .expect("still on the battlefield");
    assert_eq!(
        returned.controller,
        PlayerId::Two,
        "control reverts when the turn ends"
    );
    assert!(
        !game.permanent_has_executable_keyword(returned, KeywordAbility::Haste),
        "and the granted haste is gone with it"
    );
}

#[test]
fn desecration_demon_only_grows_when_an_opponent_feeds_it() {
    let feed = |accept: bool| {
        let mut game = ready_game();
        game.battlefield.clear();
        let demon = game
            .put_onto_battlefield(PlayerId::One, cards::DESECRATION_DEMON)
            .expect("cataloged");
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .expect("cataloged");
        game.turn = 2;
        game.step = Step::BeginningOfCombat;
        game.begin_step_triggers();
        for _ in 0..8 {
            if !game.pending_decisions.is_empty() {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let decision = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
            .expect("the opponent is offered the sacrifice");
        // Declining is a real answer, which is what makes it optional.
        assert_eq!(decision.player, PlayerId::Two);
        assert_eq!(decision.minimum, 0);
        let options = if accept {
            vec![decision.options[0].id]
        } else {
            Vec::new()
        };
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .unwrap();
        drain_pending(&mut game);
        (game, demon)
    };

    let (after_tribute, demon) = feed(true);
    let permanent = after_tribute
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == demon)
        .expect("the Demon is still there");
    assert!(permanent.tapped, "a fed Demon stays home");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 1);
    assert!(
        !after_tribute
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS),
        "and the tribute was paid"
    );

    let (starved, demon) = feed(false);
    let permanent = starved
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == demon)
        .expect("the Demon is still there");
    assert!(!permanent.tapped, "a refused Demon is free to attack");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 0);
    assert!(
        starved
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS),
        "and nothing was sacrificed"
    );
}

#[test]
fn rest_in_peace_exiles_everything_headed_for_a_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::REST_IN_PEACE)
        .expect("cataloged");
    drain_pending(&mut game);

    // A creature dying.
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.destroy_permanent(lions);
    drain_pending(&mut game);

    // A card discarded from hand.
    game.players[0].hand = vec![card(13_000, cards::PLAINS, PlayerId::One)];
    game.discard_cards(PlayerId::One, &[GameObjectId(13_000)]);

    // A card put into a graveyard by an effect, from the library.
    game.players[0].library = vec![card(13_001, cards::FOREST, PlayerId::One)];
    let milled = game.players[0].library.pop().expect("a card to bury");
    game.bury_cards(PlayerId::One, vec![milled]);

    assert!(
        game.players[0].graveyard.is_empty(),
        "no card reached the graveyard from any zone: {:?}",
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>()
    );
    for definition in [cards::SAVANNAH_LIONS, cards::PLAINS, cards::FOREST] {
        assert!(
            game.players[0]
                .exile
                .iter()
                .any(|card| card.definition == definition),
            "{definition:?} was exiled instead"
        );
    }

    // With the enchantment gone, the graveyard works again.
    let rest = game.battlefield[0].card.id;
    game.destroy_permanent(rest);
    drain_pending(&mut game);
    let ooze = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.destroy_permanent(ooze);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the replacement stopped when its source left"
    );
}

#[test]
fn sepulchral_primordial_reanimates_under_its_controller() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1]
        .graveyard
        .push(card(14_000, cards::SERRA_ANGEL, PlayerId::Two));
    // Your own graveyard is not a legal source, so this one stays put.
    game.players[0]
        .graveyard
        .push(card(14_001, cards::SAVANNAH_LIONS, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::SEPULCHRAL_PRIMORDIAL)
        .expect("cataloged");
    drain_pending(&mut game);

    let reanimated = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("the Angel was reanimated");
    assert_eq!(
        reanimated.controller,
        PlayerId::One,
        "under your control, not its owner's"
    );
    assert_eq!(
        reanimated.card.owner,
        PlayerId::Two,
        "ownership is unchanged, so it goes home if it dies"
    );
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "your own graveyard was never a legal target"
    );
}

#[test]
fn extort_drains_when_paid_with_either_half_of_its_hybrid() {
    let drain_with = |land: crate::CardDefinitionId| {
        let mut game = ready_game();
        game.battlefield.clear();
        game.put_onto_battlefield(PlayerId::One, cards::BLIND_OBEDIENCE)
            .expect("cataloged");
        drain_pending(&mut game);
        // One land for the spell, one for the extort payment.
        game.put_onto_battlefield(PlayerId::One, cards::SWAMP)
            .expect("cataloged");
        game.put_onto_battlefield(PlayerId::One, land)
            .expect("cataloged");
        game.players[0].hand = vec![card(15_000, cards::DARK_RITUAL, PlayerId::One)];
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { .. }))
            .expect("the spell is castable");
        game.apply(PlayerId::One, cast).unwrap();
        for _ in 0..12 {
            if !game.pending_decisions.is_empty() {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let decision = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
            .expect("extort offers its payment");
        let pay = decision
            .options
            .iter()
            .find(|option| option.label != "Decline")
            .expect("paying is an option")
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![pay],
            },
        )
        .unwrap();
        drain_pending(&mut game);
        (game.players[0].life, game.players[1].life)
    };

    // A {W/B} symbol takes either colour.
    assert_eq!(drain_with(cards::PLAINS), (21, 19));
    assert_eq!(drain_with(cards::SWAMP), (21, 19));
}

#[test]
fn a_loyalty_ability_costs_counters_and_runs_once_a_turn() {
    let mut game = ready_game();
    game.battlefield.clear();
    let jace = game
        .put_onto_battlefield(PlayerId::One, cards::JACE_MEMORY_ADEPT)
        .expect("cataloged");
    game.players[0].library = (0..30)
        .map(|index| card(16_000 + index, cards::PLAINS, PlayerId::One))
        .collect();
    game.players[1].library = (0..30)
        .map(|index| card(17_000 + index, cards::FOREST, PlayerId::Two))
        .collect();
    game.players[0].hand.clear();
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == jace)
            .map(|permanent| permanent.counters(CounterKind::Loyalty)),
        Some(4),
        "a planeswalker enters with its printed loyalty"
    );

    // The ultimate costs seven and Jace has four, so it is not offered.
    let offered = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == jace))
            .count()
    };
    assert!(offered(&game) > 0, "the affordable abilities are offered");
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { ability, .. }
                if matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(2)))
        }),
        "minus seven cannot be paid from four loyalty"
    );

    let plus_one = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == jace
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId::PRIMARY)
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .expect("plus one aimed at the opponent is offered");
    game.apply(PlayerId::One, plus_one).unwrap();
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == jace)
        .expect("Jace is still there");
    assert_eq!(
        permanent.counters(CounterKind::Loyalty),
        5,
        "the plus one added a counter"
    );
    assert_eq!(game.players[0].hand.len(), 1, "and drew a card");
    assert_eq!(game.players[1].graveyard.len(), 1, "and milled one");

    assert_eq!(
        offered(&game),
        0,
        "one loyalty ability per planeswalker per turn"
    );
}

#[test]
fn a_loyalty_ability_is_sorcery_speed_and_only_its_controller_may_use_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    let jace = game
        .put_onto_battlefield(PlayerId::One, cards::JACE_MEMORY_ADEPT)
        .expect("cataloged");
    game.players[1].library = (0..30)
        .map(|index| card(18_000 + index, cards::FOREST, PlayerId::Two))
        .collect();
    game.turn = 2;
    let offered = |game: &Game, player: PlayerId| {
        game.legal_actions(player)
            .into_iter()
            .filter(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == jace))
            .count()
    };

    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(offered(&game, PlayerId::One) > 0, "your own main phase");
    assert_eq!(
        offered(&game, PlayerId::Two),
        0,
        "an opponent may not use your planeswalker"
    );

    game.step = Step::DeclareBlockers;
    assert_eq!(offered(&game, PlayerId::One), 0, "not outside a main phase");

    // A main phase with something on the stack is still not sorcery speed.
    game.step = Step::PrecombatMain;
    game.players[0].hand = vec![card(18_500, cards::DARK_RITUAL, PlayerId::One)];
    game.put_onto_battlefield(PlayerId::One, cards::SWAMP)
        .expect("cataloged");
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { .. }))
        .expect("the spell is castable");
    game.apply(PlayerId::One, cast).unwrap();
    assert!(!game.stack.is_empty(), "the spell is waiting to resolve");
    assert_eq!(
        offered(&game, PlayerId::One),
        0,
        "not while anything is on the stack"
    );
}

#[test]
fn liliana_splits_a_board_and_the_victim_picks_the_pile() {
    let mut game = ready_game();
    game.battlefield.clear();
    let liliana = game
        .put_onto_battlefield(PlayerId::One, cards::LILIANA_OF_THE_VEIL)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liliana)
    {
        // Enough loyalty for the ultimate.
        permanent.set_counters(CounterKind::Loyalty, 6);
    }
    let lions = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::FOREST)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let ultimate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == liliana
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(2))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .expect("the ultimate is offered at six loyalty");
    game.apply(PlayerId::One, ultimate).unwrap();
    while game.pending_decisions.is_empty() && !game.stack.is_empty() {
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }

    // Liliana's controller makes the split: the two creatures in one pile.
    let split = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the split is offered to Liliana's controller");
    assert_eq!(split.player, PlayerId::One);
    let creatures = split
        .options
        .iter()
        .filter(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == lions || id == angel)
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(creatures.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: split.id,
            options: creatures,
        },
    )
    .unwrap();

    // The other player chooses which pile to give up, and takes the land.
    let choice = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the victim chooses a pile");
    assert_eq!(choice.player, PlayerId::Two);
    assert_eq!(choice.options.len(), 2);
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![choice.options[1].id],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions)
            && game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == angel),
        "the creatures were in the pile they kept"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::FOREST),
        "and the pile they chose was sacrificed"
    );
}

#[test]
fn aurelia_untaps_the_team_and_buys_exactly_one_extra_combat() {
    let mut game = ready_game();
    game.battlefield.clear();
    let aurelia = game
        .put_onto_battlefield(PlayerId::One, cards::AURELIA_THE_WARLEADER)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.turn = 2;
    game.tap_permanent(lions);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: aurelia,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == lions)
            .expect("the Lions are still there")
            .tapped,
        "the trigger untapped the rest of the team"
    );

    // Walk the rest of combat; the extra phase comes instead of second main.
    let mut seen_second_combat = false;
    for _ in 0..40 {
        if game.step == Step::PostcombatMain {
            break;
        }
        if game.step == Step::EndOfCombat {
            game.advance_step();
            if game.step == Step::BeginningOfCombat {
                seen_second_combat = true;
            }
            continue;
        }
        game.advance_step();
    }
    assert!(seen_second_combat, "an additional combat phase happened");
    assert_eq!(
        game.step,
        Step::PostcombatMain,
        "and the turn reached its second main afterwards"
    );
    assert_eq!(
        game.additional_combat_phases, 0,
        "the extra combat was spent rather than granted every time"
    );
}

#[test]
fn an_attack_trigger_for_the_first_time_each_turn_does_not_loop() {
    let mut game = ready_game();
    game.battlefield.clear();
    let aurelia = game
        .put_onto_battlefield(PlayerId::One, cards::AURELIA_THE_WARLEADER)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.turn = 2;

    let attack = |game: &mut Game| {
        game.step = Step::DeclareAttackers;
        game.attackers_declared = false;
        for permanent in &mut game.battlefield {
            permanent.attacking = false;
            permanent.tapped = false;
        }
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker: aurelia,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .unwrap();
        game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
            .unwrap();
        drain_pending(game);
    };

    attack(&mut game);
    assert_eq!(
        game.additional_combat_phases, 1,
        "the first attack this turn granted a combat phase"
    );

    // Attacking again in the extra combat is not the first time this turn,
    // so it grants nothing. Without that guard Aurelia never stops attacking.
    attack(&mut game);
    assert_eq!(
        game.additional_combat_phases, 1,
        "attacking again the same turn granted nothing further"
    );
}
