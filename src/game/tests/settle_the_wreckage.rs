use super::*;

fn setup(prepared: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.players[1].library = (0..5)
        .map(|i| card(210_000 + i, cards::FOREST, PlayerId::Two))
        .chain([
            card(210_005, cards::TAIGA, PlayerId::Two),
            card(210_006, cards::LIGHTNING_BOLT, PlayerId::Two),
        ])
        .collect();
    game
}

fn attack(game: &mut Game, player: PlayerId, attackers: &[GameObjectId]) {
    game.turns_started = [3, 3];
    game.active_player = player;
    game.step = Step::DeclareAttackers;
    game.priority = player;
    game.attackers_declared = false;
    for attacker in attackers {
        game.apply(
            player,
            Action::DeclareAttacker {
                attacker: *attacker,
                defender: AttackDefender::Player(player.opponent()),
            },
        )
        .unwrap();
    }
    game.apply(player, Action::FinishDeclaringAttackers)
        .unwrap();
    game.priority = PlayerId::One;
}

fn cast(game: &mut Game, target: PlayerId) {
    let settle = card(210_020, cards::SETTLE_THE_WRECKAGE, PlayerId::One);
    game.players[0].hand.push(settle.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == settle.id && choices.iter_targets().copied().collect::<Vec<_>>()
                == vec![Target::Player(target)])
        })
        .expect("Settle targets exactly the chosen player");
    game.apply(PlayerId::One, action).unwrap();
}

fn choose_search(game: &mut Game, player: PlayerId, accept: bool) {
    let decision = game.observe(player).decision.expect("optional search");
    let option = decision
        .options
        .iter()
        .find(|option| (option.label == "Decline") != accept)
        .expect("accept or decline is offered");
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option.id],
        },
    )
    .unwrap();
}

fn grant_replacement(game: &mut Game, source: GameObjectId, ability: &'static AbilityDef) {
    let mut object = spell(210_030, cards::SETTLE_THE_WRECKAGE, PlayerId::One, 0);
    object.source = Some(source);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(ability),
            duration: ResolvedEffectDurationDef::Permanent,
        }),
        &object,
        TriggerContext::empty(),
    );
}

#[test]
fn settle_exiles_only_the_targets_attackers_and_searches_for_fewer_basics() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let attacker = creature(210_010, cards::INVISIBLE_STALKER, PlayerId::Two);
        let mut stolen = creature(210_011, cards::GRIZZLY_BEARS, PlayerId::One);
        stolen.controller = PlayerId::Two;
        let idle = creature(210_012, cards::SAVANNAH_LIONS, PlayerId::Two);
        let defender = creature(210_013, cards::GRIZZLY_BEARS, PlayerId::One);
        game.battlefield.extend([attacker, stolen, idle, defender]);
        attack(
            &mut game,
            PlayerId::Two,
            &[GameObjectId(210_010), GameObjectId(210_011)],
        );
        cast(&mut game, PlayerId::Two);
        pass_until_decision(&mut game);
        assert_eq!(game.battlefield.len(), 2);
        assert_eq!(game.players[0].exile[0].definition, cards::GRIZZLY_BEARS);
        assert_eq!(
            game.players[1].exile[0].definition,
            cards::INVISIBLE_STALKER
        );
        choose_search(&mut game, PlayerId::Two, true);
        let search = game.observe(PlayerId::Two).decision.unwrap();
        assert_eq!((search.minimum, search.maximum), (0, 2));
        assert_eq!(search.visibility, DecisionVisibility::Private);
        assert_eq!(search.options.len(), 5, "a dual land is not basic");
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: search.id,
                options: vec![search.options[0].id],
            },
        )
        .unwrap();
        let land = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::FOREST)
            .unwrap();
        assert!(land.tapped);
        assert_eq!(land.controller, PlayerId::Two);
        assert_eq!(game.players[1].library.len(), 6);
        assert!(game.pending_decisions.is_empty());
    }
}

#[test]
fn settle_counts_tokens_and_animated_lands_when_targeting_yourself() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.battlefield.push(token_permanent(
            210_010,
            TokenCharacteristics::creature(&["Bear"], &[ManaColor::Green], 2, 2),
            PlayerId::One,
        ));
        game.battlefield
            .push(creature(210_011, cards::MISHRA_S_FACTORY, PlayerId::One));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let activation = game.legal_actions(PlayerId::One).into_iter().find(|a|
            matches!(a, Action::ActivateAbility { source, .. } if *source == GameObjectId(210_011))
        ).unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        pass_priority_pair(&mut game);
        game.players[0].library = (0..3)
            .map(|i| card(210_040 + i, cards::PLAINS, PlayerId::One))
            .collect();
        attack(
            &mut game,
            PlayerId::One,
            &[GameObjectId(210_010), GameObjectId(210_011)],
        );
        cast(&mut game, PlayerId::One);
        pass_until_decision(&mut game);
        choose_search(&mut game, PlayerId::One, true);
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[0].exile.len(), 1);
        assert_eq!(game.players[0].exile[0].definition, cards::MISHRA_S_FACTORY);
        assert_eq!(game.observe(PlayerId::One).decision.unwrap().maximum, 2);
    }
}

#[test]
fn settle_declining_preserves_library_but_failing_to_find_shuffles_even_without_attackers() {
    for prepared in [false, true] {
        for attackers in [false, true] {
            for accept in [false, true] {
                let mut game = setup(prepared);
                if attackers {
                    game.battlefield
                        .push(creature(210_010, cards::GRIZZLY_BEARS, PlayerId::Two));
                    attack(&mut game, PlayerId::Two, &[GameObjectId(210_010)]);
                }
                cast(&mut game, PlayerId::Two);
                pass_until_decision(&mut game);
                let mut expected_rng = game.rng.clone();
                let mut expected_library = game.players[1].library.clone();
                if accept {
                    expected_rng.shuffle(&mut expected_library);
                }
                choose_search(&mut game, PlayerId::Two, accept);
                if accept && attackers {
                    let search = game.observe(PlayerId::Two).decision.unwrap();
                    game.apply(
                        PlayerId::Two,
                        Action::ChooseDecision {
                            decision: search.id,
                            options: vec![],
                        },
                    )
                    .unwrap();
                }
                assert_eq!(game.players[1].library, expected_library);
                assert_eq!(game.rng.clone().next_u64(), expected_rng.next_u64());
                assert!(game.pending_decisions.is_empty());
            }
        }
    }
}

#[test]
fn settle_reevaluates_attackers_at_resolution_and_fizzles_for_an_illegal_player() {
    for protected in [false, true] {
        let mut game = setup(false);
        game.battlefield
            .push(creature(210_010, cards::GRIZZLY_BEARS, PlayerId::Two));
        attack(&mut game, PlayerId::Two, &[GameObjectId(210_010)]);
        cast(&mut game, PlayerId::Two);
        if protected {
            game.put_onto_battlefield(PlayerId::Two, cards::IVORY_MASK)
                .unwrap();
        } else {
            game.remove_permanent_from_combat(GameObjectId(210_010));
        }
        pass_until_decision(&mut game);
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.id == GameObjectId(210_010))
        );
        assert!(game.players[1].exile.is_empty());
        if protected {
            assert!(game.pending_decisions.is_empty());
        } else {
            choose_search(&mut game, PlayerId::Two, true);
            assert!(
                game.pending_decisions.is_empty(),
                "the search maximum is now zero"
            );
        }
    }
}

#[test]
fn settle_waits_for_replacements_and_counts_redirected_but_not_canceled_moves() {
    static CANCEL: AbilityDef = AbilityDef::replacement_for(
        "If this creature would be exiled, instead it remains on the battlefield.",
        ReplacementEventDef::WouldMove {
            from: Some(ZoneKind::Battlefield),
            to: ZoneKind::Exile,
            cause: ZoneMoveCauseDef::Any,
        },
        ReplacementEffectDef::ReplaceEventWithNothing,
    );
    static REDIRECT: AbilityDef = AbilityDef::replacement_for(
        "If this creature would be exiled, put it into its owner's hand instead.",
        ReplacementEventDef::WouldMove {
            from: Some(ZoneKind::Battlefield),
            to: ZoneKind::Exile,
            cause: ZoneMoveCauseDef::Any,
        },
        ReplacementEffectDef::MoveToZone(ZoneKind::Hand),
    );
    for prepared in [false, true] {
        for cancel in [false, true] {
            let mut game = setup(prepared);
            for id in 210_010..210_013 {
                game.battlefield
                    .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::Two));
            }
            grant_replacement(&mut game, GameObjectId(210_010), &CANCEL);
            grant_replacement(&mut game, GameObjectId(210_011), &CANCEL);
            grant_replacement(&mut game, GameObjectId(210_011), &REDIRECT);
            attack(
                &mut game,
                PlayerId::Two,
                &[
                    GameObjectId(210_010),
                    GameObjectId(210_011),
                    GameObjectId(210_012),
                ],
            );
            cast(&mut game, PlayerId::Two);
            pass_until_decision(&mut game);
            let decision = game.observe(PlayerId::Two).decision.unwrap();
            assert!(decision.prompt.starts_with("Choose a replacement effect"));
            assert_eq!(game.battlefield.len(), 3, "the whole exit batch waits");
            let option = decision
                .options
                .iter()
                .find(|option| {
                    option
                        .label
                        .contains(if cancel { "remains" } else { "hand" })
                })
                .unwrap();
            game.apply(
                PlayerId::Two,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option.id],
                },
            )
            .unwrap();
            choose_search(&mut game, PlayerId::Two, true);
            assert_eq!(
                game.observe(PlayerId::Two).decision.unwrap().maximum,
                if cancel { 1 } else { 2 }
            );
            assert_eq!(game.battlefield.len(), if cancel { 2 } else { 1 });
            assert_eq!(game.players[1].exile.len(), 1);
            assert_eq!(game.players[1].hand.len(), usize::from(!cancel));
        }
    }
}

#[test]
fn settle_search_count_survives_checkpoint_reconstruction() {
    let mut game = setup(true);
    game.battlefield
        .push(creature(210_010, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield.push(token_permanent(
        210_011,
        TokenCharacteristics::creature(&["Bear"], &[ManaColor::Green], 2, 2),
        PlayerId::Two,
    ));
    attack(
        &mut game,
        PlayerId::Two,
        &[GameObjectId(210_010), GameObjectId(210_011)],
    );
    cast(&mut game, PlayerId::Two);
    pass_until_decision(&mut game);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::Two);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 7)
            .expect("the optional search retains the completed move binding");
    choose_search(&mut restored, PlayerId::Two, true);
    let search = restored.observe(PlayerId::Two).decision.unwrap();
    assert_eq!(search.maximum, 2);
    let (wire, hidden) = checkpoint_fixture(&restored, PlayerId::Two);
    let mut restored = Game::from_observation_checkpoint(
        restored.catalog.clone(),
        restored.format,
        &wire,
        &hidden,
        7,
    )
    .expect("the land choice is reconstructible");
    let search = restored.observe(PlayerId::Two).decision.unwrap();
    restored
        .apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: search.id,
                options: search
                    .options
                    .iter()
                    .take(2)
                    .map(|option| option.id)
                    .collect(),
            },
        )
        .unwrap();
    assert_eq!(
        restored
            .battlefield
            .iter()
            .filter(|p| p.controller == PlayerId::Two
                && p.tapped
                && p.card.definition == cards::FOREST)
            .count(),
        2
    );
}

#[test]
fn settle_counts_an_exiled_commander_before_its_return_choice() {
    let deck = crate::Deck {
        commanders: vec![cards::GRIZZLY_BEARS],
        main: vec![cards::FOREST; 99],
        sideboard: vec![],
    };
    let mut game = Game::new_with_format(
        crate::Format::Cedh,
        crate::card::catalog().unwrap(),
        [deck.clone(), deck],
        1,
    )
    .unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    for player in &mut game.players {
        player.hand.clear();
    }
    let commander = game.players[0].command[0].id;
    game.move_target_to_zone(
        Target::Card(commander),
        ZoneKind::Battlefield,
        ZoneMoveCause::Rules,
        Some(BattlefieldArrival::under(PlayerId::One)),
        ZonePlacement::Top,
    );
    drain_pending(&mut game);
    let commander = game.battlefield[0].card.id;
    game.battlefield[0].entered_controller_turn = 0;
    attack(&mut game, PlayerId::One, &[commander]);
    cast(&mut game, PlayerId::One);
    pass_until_decision(&mut game);
    choose_search(&mut game, PlayerId::One, true);
    assert_eq!(game.observe(PlayerId::One).decision.unwrap().maximum, 1);
}
