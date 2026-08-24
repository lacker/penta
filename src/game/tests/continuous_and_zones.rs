use super::*;

#[test]
fn sigarda_stops_an_opponents_edict() {
    for sigarda_out in [false, true] {
        let mut game = ready_game();
        let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
        attacker.attacking = true;
        game.battlefield.push(attacker);
        if sigarda_out {
            game.battlefield.push(creature(
                10_003,
                cards::SIGARDA_HOST_OF_HERONS,
                PlayerId::Two,
            ));
        }
        let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
        game.players[0].hand.push(flare.clone());
        game.players[0].mana_pool.white = 2;

        game.apply(
            PlayerId::One,
            cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let attacker_survived = game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(10_000));
        assert_eq!(attacker_survived, sigarda_out, "Sigarda out: {sigarda_out}");
    }
}

#[test]
fn kessig_wolf_run_offers_only_the_x_it_can_actually_pay() {
    let mut game = ready_game();
    // Only the floating mana pays, so the affordable range is exact. The Run
    // itself taps for the ability, so its own colorless is not available.
    game.battlefield.clear();
    let run = game
        .put_onto_battlefield(PlayerId::One, cards::KESSIG_WOLF_RUN)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;

    let mut offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { source, x, .. } if source == run => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();
    assert_eq!(offered, vec![0, 1, 2, 3], "five mana, less the two colored");

    let pump = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, x, .. } if *source == run && *x == 3))
        .expect("X of three is affordable");
    game.apply(PlayerId::One, pump).unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions)
        .expect("still there");
    assert_eq!(game.power(lions), Some(5), "2/1 plus three");
    assert_eq!(game.toughness(lions), Some(1), "toughness is untouched");
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));
}

#[test]
fn kessig_wolf_run_scales_across_many_chromatic_lantern_lands() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.clear();
    let run = game
        .put_onto_battlefield(PlayerId::One, cards::KESSIG_WOLF_RUN)
        .expect("Kessig Wolf Run is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::CHROMATIC_LANTERN)
        .expect("Chromatic Lantern is cataloged");
    for _ in 0..16 {
        game.put_onto_battlefield(PlayerId::One, cards::FOREST)
            .expect("Forest is cataloged");
    }
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("the target creature is cataloged");

    let mut offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { source, x, .. } if source == run => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();
    assert_eq!(
        offered,
        (0..=15).collect::<Vec<_>>(),
        "seventeen usable sources pay red, green, and X while the Run taps"
    );
}

#[test]
fn gaze_of_granite_sweeps_up_to_the_x_it_was_cast_for() {
    let mut game = ready_game();
    game.battlefield.clear();
    // One, five, and a land: X of two takes the first and spares the rest.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");
    let sweeper = card(10_000, cards::GAZE_OF_GRANITE, PlayerId::One);
    game.players[0].hand.push(sweeper.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(sweeper.id, Vec::new(), Vec::new(), 2),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL, cards::MOUNTAIN],
        "the two-or-less nonland permanent is the only one destroyed"
    );
}

#[test]
fn blasphemous_act_gets_cheaper_as_the_board_fills_up() {
    let mut game = ready_game();
    game.battlefield.clear();
    let act = card(10_000, cards::BLASPHEMOUS_ACT, PlayerId::One);
    game.players[0].hand.push(act.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 2;

    let castable = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == act.id))
    };
    assert!(!castable(&game), "nine mana is out of reach on three");

    // Six creatures take it to {2}{R}, which the pool covers. Both sides
    // count: the reduction is not about who controls them.
    for (index, owner) in [PlayerId::One, PlayerId::Two]
        .into_iter()
        .cycle()
        .take(6)
        .enumerate()
    {
        game.battlefield.push(creature(
            10_010 + u32::try_from(index).unwrap(),
            cards::SAVANNAH_LIONS,
            owner,
        ));
    }
    assert!(castable(&game), "six creatures pay for six of the eight");

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == act.id))
        .expect("castable");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "thirteen damage to each creature"
    );
}

#[test]
fn scavenging_ooze_only_grows_on_a_creature_card() {
    for (definition, expect_growth) in [(cards::SAVANNAH_LIONS, true), (cards::MOUNTAIN, false)] {
        let mut game = ready_game();
        let ooze = game
            .put_onto_battlefield(PlayerId::One, cards::SCAVENGING_OOZE)
            .expect("cataloged");
        let food = card(10_000, definition, PlayerId::Two);
        game.players[1].graveyard.push(food.clone());
        game.players[0].mana_pool.green = 1;
        let life_before = game.players[0].life;

        let eat = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == ooze))
            .expect("the graveyard card is a legal target");
        game.apply(PlayerId::One, eat).unwrap();
        pass_priority_pair(&mut game);

        assert!(game.players[1].graveyard.is_empty(), "it was exiled");
        assert_eq!(game.players[1].exile.len(), 1);
        let ooze = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == ooze)
            .expect("still there");
        let counters = ooze.counters.count(CounterKind::PlusOnePlusOne);
        assert_eq!(counters, u16::from(expect_growth), "{definition:?}");
        assert_eq!(
            game.players[0].life - life_before,
            i16::from(expect_growth),
            "{definition:?}"
        );
    }
}

#[test]
fn demonic_rising_only_pays_off_with_exactly_one_creature() {
    for (creatures, expect_demon) in [(0, false), (1, true), (2, false)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.put_onto_battlefield(PlayerId::One, cards::DEMONIC_RISING)
            .expect("cataloged");
        for _ in 0..creatures {
            game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
                .expect("cataloged");
        }
        // The opponent's creatures are not yours, whatever the count.
        game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
            .expect("cataloged");

        game.step = Step::PostcombatMain;
        game.advance_step();
        for _ in 0..8 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let demons = game
            .battlefield
            .iter()
            .filter(|permanent| {
                is_token_with(
                    permanent,
                    token_with_flying(tokens::creature(&["Demon"], &[ManaColor::Black], 5, 5)),
                )
            })
            .count();
        assert_eq!(demons, usize::from(expect_demon), "{creatures} creatures");
    }
}

#[test]
fn izzet_staticaster_hits_every_copy_of_the_creature_it_names() {
    let mut game = ready_game();
    game.battlefield.clear();
    let caster = game
        .put_onto_battlefield(PlayerId::One, cards::IZZET_STATICASTER)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == caster)
        .expect("just entered")
        .entered_controller_turn = game.turns_started[0] - 1;
    // Two Lions on one side, one on the other, and an unrelated creature.
    let mut lions = Vec::new();
    for owner in [PlayerId::Two, PlayerId::Two, PlayerId::One] {
        lions.push(
            game.put_onto_battlefield(owner, cards::SAVANNAH_LIONS)
                .expect("cataloged"),
        );
    }
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");

    let zap = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == caster
                && targets.iter().flat_map(TargetSelection::targets).any(|target| {
                    *target == Target::Permanent(lions[0])
                }))
        })
        .expect("a Lion is a legal target");
    game.apply(PlayerId::One, zap).unwrap();
    pass_priority_pair(&mut game);

    // A 2/1 dies to one damage, whoever controls it.
    assert!(
        lions
            .iter()
            .all(|lion| !game.battlefield.iter().any(|p| p.card.id == *lion)),
        "every Savannah Lions was named"
    );
    assert!(
        game.battlefield.iter().any(|p| p.card.id == angel),
        "the Angel shares no name"
    );
}

#[test]
fn izzet_staticaster_reads_the_name_copied_by_thespians_stage() {
    let mut game = ready_game();
    game.battlefield.clear();
    let stage = game
        .put_onto_battlefield(PlayerId::One, cards::THESPIANS_STAGE)
        .expect("cataloged");
    let arbor = game
        .put_onto_battlefield(PlayerId::One, cards::DRYAD_ARBOR)
        .expect("cataloged");
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage,
            ability: activated_ability_for(&game, stage, 0),
            targets: activated_targets(Target::Permanent(arbor)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let caster = game
        .put_onto_battlefield(PlayerId::Two, cards::IZZET_STATICASTER)
        .expect("cataloged");
    game.priority = PlayerId::Two;
    game.consecutive_passes = 0;
    let zap = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == caster
                && targets.iter().flat_map(TargetSelection::targets).any(|target| {
                    *target == Target::Permanent(stage)
                }))
        })
        .expect("the Stage presenting Dryad Arbor is a legal target");
    game.apply(PlayerId::Two, zap).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        [stage, arbor].iter().all(|id| !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == *id)),
        "the copied and physical Dryad Arbors share a copiable name",
    );
}

#[test]
fn oblivion_ring_gives_back_exactly_what_it_took() {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    // A second creature the Ring never touched, to prove the link is
    // specific rather than a sweep of the exile zone.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[1]
        .exile
        .push(card(10_050, cards::MOUNTAIN, PlayerId::Two));

    let ring = game
        .put_onto_battlefield(PlayerId::One, cards::OBLIVION_RING)
        .expect("cataloged");
    let drain = |game: &mut Game| {
        for _ in 0..10 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .filter(|option| {
                        option.card
                            == Some((
                                angel,
                                ObjectCharacteristics::card(
                                    cards::SERRA_ANGEL,
                                    CardPartId::PRIMARY,
                                ),
                            ))
                    })
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);

    assert!(
        !game.battlefield.iter().any(|p| p.card.id == angel),
        "the Angel was exiled"
    );

    game.destroy_permanent(ring);
    drain(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
        "the Angel came back and the unrelated exiled Mountain stayed put"
    );
    assert_eq!(game.players[1].exile.len(), 1, "the Mountain is untouched");
}

#[test]
fn detention_sphere_takes_every_copy_and_gives_them_all_back() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = [
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .expect("cataloged"),
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .expect("cataloged"),
    ];
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    let sphere = game
        .put_onto_battlefield(PlayerId::One, cards::DETENTION_SPHERE)
        .expect("cataloged");

    let drain = |game: &mut Game| {
        for _ in 0..12 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                // Take the optional exile, and name a Lion when asked.
                let options = decision
                    .options
                    .iter()
                    .filter(|option| {
                        option.label == "Do it"
                            || option.card
                                == Some((
                                    lions[0],
                                    ObjectCharacteristics::card(
                                        cards::SAVANNAH_LIONS,
                                        CardPartId::PRIMARY,
                                    ),
                                ))
                    })
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };

    // The Sphere's trigger needs a target chosen when it is put on the stack.
    let target = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ChooseDecision { .. }));
    if let Some(action) = target {
        let _ = game.apply(PlayerId::One, action);
    }
    drain(&mut game);

    let on_field = |game: &Game, id: GameObjectId| game.battlefield.iter().any(|p| p.card.id == id);
    assert!(
        !on_field(&game, lions[0]) && !on_field(&game, lions[1]),
        "both Lions left"
    );
    assert!(on_field(&game, angel), "the Angel shares no name");

    game.destroy_permanent(sphere);
    drain(&mut game);

    let names = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.definition)
        .collect::<Vec<_>>();
    assert_eq!(
        names
            .iter()
            .filter(|d| **d == cards::SAVANNAH_LIONS)
            .count(),
        2,
        "both Lions came back"
    );
}

#[test]
fn angel_of_serenity_takes_from_both_zones_and_returns_to_hand() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[1]
        .graveyard
        .push(card(10_050, cards::SERRA_ANGEL, PlayerId::Two));
    let hand_before = game.players[1].hand.len();

    let angel = game
        .put_onto_battlefield(PlayerId::One, cards::ANGEL_OF_SERENITY)
        .expect("cataloged");
    // Take every offered target, and accept the optional exile.
    let drain = |game: &mut Game| {
        for _ in 0..12 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let cards = decision
                    .options
                    .iter()
                    .filter(|option| option.card.is_some())
                    .map(|option| option.id)
                    .take(decision.maximum)
                    .collect::<Vec<_>>();
                let options = if cards.is_empty() {
                    decision
                        .options
                        .iter()
                        .filter(|option| option.label == "Do it")
                        .map(|option| option.id)
                        .chain(decision.options.iter().map(|option| option.id))
                        .take(decision.minimum.max(1))
                        .collect::<Vec<_>>()
                } else {
                    cards
                };
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions),
        "the creature on the battlefield was taken"
    );
    assert!(
        game.players[1].graveyard.is_empty(),
        "so was the creature card in the graveyard"
    );
    assert_eq!(game.players[1].exile.len(), 2);

    game.destroy_permanent(angel);
    drain(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        hand_before + 2,
        "both came back to hand rather than to the battlefield"
    );
    assert!(game.players[1].exile.is_empty());
}

#[test]
fn quicken_lets_one_sorcery_be_cast_at_instant_speed() {
    let mut game = ready_game();
    let quicken = card(10_000, cards::QUICKEN, PlayerId::One);
    game.players[0].hand.push(quicken.clone());
    let sorceries = [
        card(10_001, cards::MIND_TWIST, PlayerId::One),
        card(10_002, cards::MIND_TWIST, PlayerId::One),
    ];
    for sorcery in &sorceries {
        game.players[0].hand.push(sorcery.clone());
    }
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.black = 4;
    // The opponent's turn, where a sorcery is never castable.
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;

    let castable = |game: &Game, id: CardInstanceId| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    };
    assert!(!castable(&game, sorceries[0].id), "not on their turn");

    game.apply(
        PlayerId::One,
        cast_action(quicken.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // Quicken resolving handed priority back to the active player.
    game.priority = PlayerId::One;
    assert!(castable(&game, sorceries[0].id), "the grant covers it");
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[0].id))
        .expect("castable");
    game.apply(PlayerId::One, cast).unwrap();
    game.priority = PlayerId::One;

    assert!(
        !castable(&game, sorceries[1].id),
        "the grant covered the next sorcery, not every one"
    );
}

#[test]
fn obzedat_blinks_itself_and_comes_back_hasty_next_upkeep() {
    let mut game = ready_game();
    game.battlefield.clear();
    let obzedat = game
        .put_onto_battlefield(PlayerId::One, cards::OBZEDAT_GHOST_COUNCIL)
        .expect("cataloged");
    // Its entry trigger fires too; take every decision as it comes.
    let drain = |game: &mut Game| {
        for _ in 0..14 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .filter(|option| option.label == "Do it")
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);
    assert!(
        game.battlefield.iter().any(|p| p.card.id == obzedat),
        "it starts on the battlefield"
    );

    game.step = Step::PostcombatMain;
    game.advance_step();
    drain(&mut game);
    assert!(
        !game.battlefield.iter().any(|p| p.card.id == obzedat),
        "the end step exiled it"
    );
    assert_eq!(game.players[0].exile.len(), 1);

    // Their turn, then back to ours: it returns at our upkeep, not theirs.
    game.start_next_turn();
    drain(&mut game);
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "not on the opponent's upkeep"
    );

    game.start_next_turn();
    drain(&mut game);
    let back = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::OBZEDAT_GHOST_COUNCIL)
        .expect("it came back");
    assert!(
        game.permanent_has_executable_keyword(back, KeywordAbility::Haste),
        "and it can attack straight away"
    );
    assert!(game.players[0].exile.is_empty());
}

#[test]
fn aetherling_dodges_a_blocker_and_comes_back_at_the_end_step() {
    let mut game = ready_game();
    game.battlefield.clear();
    let aetherling = game
        .put_onto_battlefield(PlayerId::One, cards::AETHERLING)
        .expect("cataloged");
    let wall = game
        .put_onto_battlefield(PlayerId::Two, cards::WALL_OF_STONE)
        .expect("cataloged");
    game.players[0].mana_pool.blue = 2;

    let activate = |game: &mut Game, index: usize| {
        let mut printed = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    ability: AbilityOrigin::Printed { ability, .. },
                    source,
                    ..
                } if source == aetherling => Some(ability),
                _ => None,
            })
            .collect::<Vec<_>>();
        printed.sort_unstable();
        printed.dedup();
        let ability = printed[index];
        game.apply(
            PlayerId::One,
            Action::ActivateAbility {
                source: aetherling,
                ability: AbilityOrigin::Printed {
                    definition: cards::AETHERLING,
                    part: CardPartId::PRIMARY,
                    ability,
                },
                targets: Vec::new(),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
            },
        )
        .unwrap();
        pass_priority_pair(game);
    };

    // The unblockable ability is the second printed clause.
    activate(&mut game, 1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == aetherling)
        .expect("still there")
        .attacking = true;
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::DeclareBlocker { blocker, .. } if *blocker == wall)
        ),
        "nothing can block it this turn"
    );

    // The first clause blinks it until the end step.
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    activate(&mut game, 0);
    assert!(
        !game.battlefield.iter().any(|p| p.card.id == aetherling),
        "it left for exile"
    );

    game.advance_step();
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::AETHERLING),
        "and returned at the end step"
    );
}

#[test]
fn restoration_angel_blinks_a_creature_within_one_resolution() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    // A creature the Angel may not target, so the choice is not vacuous.
    let serra = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.tap_permanent(lions);

    game.put_onto_battlefield(PlayerId::One, cards::RESTORATION_ANGEL)
        .expect("cataloged");
    for _ in 0..12 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let cards = decision
                .options
                .iter()
                .filter(|option| option.card.is_some())
                .map(|option| option.id)
                .take(decision.maximum)
                .collect::<Vec<_>>();
            let options = if cards.is_empty() {
                decision
                    .options
                    .iter()
                    .filter(|option| option.label == "Do it")
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>()
            } else {
                cards
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.players[0].exile.is_empty(),
        "the blink returned the card rather than leaving it exiled"
    );
    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the Lions came back to the battlefield");
    assert_ne!(
        returned.card.id, lions,
        "a blinked permanent returns as a new object"
    );
    assert!(!returned.tapped, "the new object is untapped");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == serra),
        "the untargetable Angel stayed put"
    );
}
