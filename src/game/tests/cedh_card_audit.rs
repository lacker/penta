//! Regression coverage for declarations promoted by the Commander corpus audit.
use super::*;
mod collections;

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield.iter().find(|p| p.card.id == id).unwrap()
}

fn mana_action(game: &Game, source: GameObjectId) -> Action {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateManaAbility { source: id, .. } if *id == source)
    }).expect("the mana activation is offered")
}

fn can_cast(game: &Game, spell: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

#[test]
fn gran_gran_discount_tracks_lessons_and_only_noncreature_spells() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .push(creature(120_000, cards::GRAN_GRAN_54, PlayerId::One));
        let draw = card(120_001, cards::DIVINATION, PlayerId::One);
        let drake = card(120_002, cards::WIND_DRAKE, PlayerId::One);
        game.players[0].hand.extend([draw.clone(), drake.clone()]);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        for id in 120_010..120_012 {
            game.players[0]
                .graveyard
                .push(card(id, cards::OCTOPUS_FORM_66, PlayerId::One));
        }
        game.players[1]
            .graveyard
            .push(card(120_013, cards::OCTOPUS_FORM_66, PlayerId::Two));
        assert!(
            !can_cast(&game, draw.id),
            "an opponent's Lesson does not count"
        );
        game.players[0]
            .graveyard
            .push(card(120_014, cards::OCTOPUS_FORM_66, PlayerId::One));
        assert!(can_cast(&game, draw.id));
        assert!(!can_cast(&game, drake.id), "creatures receive no reduction");
        game.players[0].graveyard.pop();
        assert!(
            !can_cast(&game, draw.id),
            "the discount ends when the threshold is lost"
        );
    }
}

#[test]
fn boseiju_protects_only_instant_and_sorcery_spells_paid_with_its_mana() {
    for (definition, protected) in [(cards::DIVINATION, true), (cards::WIND_DRAKE, false)] {
        for prepared in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.battlefield.push(creature(
                120_030,
                cards::BOSEIJU_WHO_SHELTERS_ALL_273,
                PlayerId::One,
            ));
            let action = mana_action(&game, GameObjectId(120_030));
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].life, 18);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
            let spell = card(120_031, definition, PlayerId::One);
            game.players[0].hand.push(spell.clone());
            game.apply(PlayerId::One, cast_action(spell.id, vec![], vec![], 0))
                .unwrap();
            assert_eq!(
                game.can_be_countered(game.stack.last().unwrap()),
                !protected
            );
        }
    }
}

#[test]
fn purphoros_changes_type_when_devotion_changes_without_losing_its_other_rules() {
    let mut game = ready_game();
    let god = creature(
        120_040,
        cards::PURPHOROS_GOD_OF_THE_FORGE_135,
        PlayerId::One,
    );
    let id = god.card.id;
    game.battlefield.push(god);
    assert!(
        !game
            .permanent_types(permanent(&game, id))
            .unwrap()
            .contains(CardType::Creature)
    );
    assert!(
        game.permanent_has_executable_keyword(permanent(&game, id), KeywordAbility::Indestructible)
    );
    game.battlefield.extend([
        creature(120_041, cards::GOBLIN_KING, PlayerId::One),
        creature(120_042, cards::GOBLIN_KING, PlayerId::One),
    ]);
    assert!(
        game.permanent_types(permanent(&game, id))
            .unwrap()
            .contains(CardType::Creature)
    );
    assert_eq!(game.power(permanent(&game, id)), Some(6));
    game.battlefield
        .retain(|p| p.card.id != GameObjectId(120_042));
    assert!(
        !game
            .permanent_types(permanent(&game, id))
            .unwrap()
            .contains(CardType::Creature)
    );
}

#[test]
fn swift_reconfiguration_sets_artifact_type_and_crew_restores_creature_type() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let host = creature(120_050, cards::SERRA_ANGEL, PlayerId::One);
        let host_id = host.card.id;
        game.battlefield.push(host);
        let mut aura = creature(120_051, cards::SWIFT_RECONFIGURATION_45, PlayerId::Two);
        aura.attached_to = Some(host_id);
        game.battlefield.push(aura);
        let types = game.permanent_types(permanent(&game, host_id)).unwrap();
        assert!(types.contains(CardType::Artifact));
        assert!(!types.contains(CardType::Creature));
        assert!(game.object_subtypes(host_id).contains(&"Vehicle"));
        assert!(
            game.permanent_has_executable_keyword(
                permanent(&game, host_id),
                KeywordAbility::Flying
            )
        );
        game.battlefield
            .push(creature(120_052, cards::CRAW_WURM, PlayerId::One));
        let crew = game.legal_actions(PlayerId::One).into_iter().find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == host_id)
        }).expect("the granted crew ability can tap the Wurm");
        game.apply(PlayerId::One, crew).unwrap();
        drain_pending(&mut game);
        assert!(
            game.permanent_types(permanent(&game, host_id))
                .unwrap()
                .contains(CardType::Creature)
        );
        assert_eq!(game.power(permanent(&game, host_id)), Some(4));
    }
}

#[test]
fn animation_module_chooses_one_existing_counter_kind_on_players_and_permanents() {
    for target_player in [false, true] {
        for prepared in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let module = creature(120_060, cards::ANIMATION_MODULE_194, PlayerId::One);
            let module_id = module.card.id;
            game.battlefield.push(module);
            let kind = CounterKind::named("charge");
            let other = CounterKind::named("energy");
            let target = if target_player {
                game.players[1].counters.add(kind, 2);
                game.players[1].counters.add(other, 1);
                Target::Player(PlayerId::Two)
            } else {
                let mut host = creature(120_061, cards::SOL_RING, PlayerId::Two);
                host.add_counters(kind, 2);
                host.add_counters(other, 1);
                game.battlefield.push(host);
                Target::Permanent(GameObjectId(120_061))
            };
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
            let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if *source == module_id && targets.iter().flat_map(TargetSelection::targets).copied().eq([target]))
            }).expect("the mixed target is legal");
            game.apply(PlayerId::One, action).unwrap();
            pass_priority_pair(&mut game);
            let decision = game
                .observe(PlayerId::One)
                .decision
                .expect("choose a counter kind");
            let option = decision
                .options
                .iter()
                .find(|option| option.label.contains("charge"))
                .unwrap()
                .id;
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option],
                },
            )
            .unwrap();
            let counters = if target_player {
                &game.players[1].counters
            } else {
                &permanent(&game, GameObjectId(120_061)).counters
            };
            assert_eq!(counters.count(kind), 3);
            assert_eq!(counters.count(other), 1, "only the chosen kind is added");
        }
    }
}

#[test]
fn mines_of_moria_exiles_three_graveyard_cards_as_an_activation_cost() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let id = GameObjectId(120_080);
        game.battlefield
            .push(creature(id.0, cards::MINES_OF_MORIA_257, PlayerId::One));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        for n in 120_081..120_083 {
            game.players[0]
                .graveyard
                .push(card(n, cards::FOREST, PlayerId::One));
        }
        let activation = |game: &Game| {
            game.legal_actions(PlayerId::One).into_iter().find(
                |action| matches!(action, Action::ActivateAbility { source, .. } if *source == id),
            )
        };
        assert!(
            activation(&game).is_none(),
            "two graveyard cards cannot pay the cost"
        );
        game.players[0]
            .graveyard
            .push(card(120_083, cards::FOREST, PlayerId::One));
        game.apply(
            PlayerId::One,
            activation(&game).expect("three cards can pay"),
        )
        .unwrap();
        assert!(
            game.players[0].graveyard.is_empty(),
            "exile is paid before resolution"
        );
        assert_eq!(game.players[0].exile.len(), 3);
        assert!(permanent(&game, id).tapped);
        assert_eq!(game.battlefield.len(), 1, "the tokens wait for resolution");
        drain_pending(&mut game);
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| game.object_subtypes(p.card.id).contains(&"Treasure"))
                .count(),
            2
        );
    }
}

#[test]
fn essence_flux_returns_a_new_object_to_its_owner_and_checks_its_returned_type() {
    for prepared in [false, true] {
        for spirit in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let definition = if spirit {
                cards::STRANGLEROOT_GEIST
            } else {
                cards::SERRA_ANGEL
            };
            let mut host = creature(120_090, definition, PlayerId::Two);
            host.controller = PlayerId::One;
            host.add_counters(CounterKind::PlusOnePlusOne, 3);
            game.battlefield.push(host);
            let spell = card(120_091, cards::ESSENCE_FLUX_61, PlayerId::One);
            game.players[0].hand.push(spell.clone());
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
            let action = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .find(
                    |action| matches!(action, Action::CastSpell { card, .. } if *card == spell.id),
                )
                .expect("the owned or borrowed creature is a legal blink target");
            game.apply(PlayerId::One, action).unwrap();
            drain_pending(&mut game);
            let returned = game
                .battlefield
                .iter()
                .find(|p| p.card.definition == definition)
                .unwrap();
            assert_ne!(returned.card.id, GameObjectId(120_090));
            assert_eq!(returned.controller, PlayerId::Two);
            assert_eq!(
                returned.counters(CounterKind::PlusOnePlusOne),
                u16::from(spirit)
            );
            assert!(game.players[1].exile.is_empty());
        }
    }
}

#[test]
fn faerie_artisans_replaces_only_its_own_previous_tokens_even_after_control_changes() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let first = game
            .put_onto_battlefield(PlayerId::One, cards::FAERIE_ARTISANS_8)
            .unwrap();
        let second = game
            .put_onto_battlefield(PlayerId::One, cards::FAERIE_ARTISANS_8)
            .unwrap();
        game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
            .unwrap();
        drain_pending(&mut game);
        let old: Vec<_> = game
            .battlefield
            .iter()
            .filter(|p| p.created_by == Some(first) || p.created_by == Some(second))
            .map(|p| p.card.id)
            .collect();
        assert_eq!(old.len(), 2);
        for token in game
            .battlefield
            .iter_mut()
            .filter(|p| old.contains(&p.card.id))
        {
            token.controller = PlayerId::Two;
        }
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        drain_pending(&mut game);
        assert!(game.battlefield.iter().all(|p| !old.contains(&p.card.id)));
        for source in [first, second] {
            let current: Vec<_> = game
                .battlefield
                .iter()
                .filter(|p| p.created_by == Some(source))
                .collect();
            assert_eq!(current.len(), 1);
            assert_eq!(current[0].controller, PlayerId::One);
            assert_eq!(game.power(current[0]), Some(2));
            assert!(
                game.permanent_types(current[0])
                    .unwrap()
                    .contains(CardType::Artifact)
            );
        }
    }
}

#[test]
fn pollywog_prodigy_compares_mana_value_strictly_at_trigger_time() {
    for prepared in [false, true] {
        for (definition, caster, should_draw) in [
            (cards::LIGHTNING_BOLT, PlayerId::Two, true),
            (cards::COUNTERSPELL, PlayerId::Two, false),
            (cards::LIGHTNING_BOLT, PlayerId::One, false),
            (cards::SAVANNAH_LIONS, PlayerId::Two, false),
        ] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let mut prodigy = creature(120_110, cards::POLLYWOG_PRODIGY_50, PlayerId::One);
            prodigy.add_counters(CounterKind::PlusOnePlusOne, 1);
            game.battlefield.push(prodigy);
            let cast = spell(120_111, definition, caster, 0);
            let event = game.stack_trigger_event_object(&cast).unwrap();
            let before = game.players[0].hand.len();
            game.capture_battlefield_triggers(&CommittedTriggerEvent::StackObject {
                object: event,
                kind: StackObjectKind::Spell,
                event: CommittedStackObjectEvent::Cast {
                    from: CastSourceZone::Hand,
                },
            });
            // The event predicate is not an intervening-if condition.
            game.battlefield
                .iter_mut()
                .find(|p| p.card.id == GameObjectId(120_110))
                .unwrap()
                .counters = Counters::default();
            drain_pending(&mut game);
            assert_eq!(
                game.players[0].hand.len(),
                before + usize::from(should_draw)
            );
        }
    }
}

#[test]
fn ashaya_counts_nontoken_creatures_as_forests_and_preserves_other_creatures() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let ashaya = creature(121_000, cards::ASHAYA_SOUL_OF_THE_WILD_179, PlayerId::One);
        let id = ashaya.card.id;
        game.battlefield.push(ashaya);
        game.battlefield
            .push(creature(121_001, cards::SERRA_ANGEL, PlayerId::One));
        game.battlefield
            .push(creature(121_002, cards::SERRA_ANGEL, PlayerId::Two));
        game.create_token(
            PlayerId::One,
            crate::card::TokenCharacteristics::creature(&["Bear"], &[ManaColor::Green], 2, 2),
        );
        let token = game.battlefield.last().unwrap().card.id;
        assert!(
            !game
                .permanent_types(permanent(&game, token))
                .unwrap()
                .contains(CardType::Land)
        );
        assert!(
            game.permanent_types(permanent(&game, id))
                .unwrap()
                .contains(CardType::Land)
        );
        assert!(game.object_subtypes(id).contains(&"Forest"));
        assert_eq!(game.power(permanent(&game, id)), Some(2));
        assert!(
            !game
                .permanent_types(permanent(&game, GameObjectId(121_002)))
                .unwrap()
                .contains(CardType::Land)
        );
        assert!(game.legal_actions(PlayerId::One).iter().any(|a| matches!(a, Action::ActivateManaAbility { source, .. } if *source == GameObjectId(121_001))));
        game.battlefield.retain(|p| p.card.id != id);
        assert!(
            !game
                .permanent_types(permanent(&game, GameObjectId(121_001)))
                .unwrap()
                .contains(CardType::Land)
        );
    }
}

fn answer_named(game: &mut Game, name: &str) {
    let decision = game.pending_decisions[0].observation.clone();
    let selected = decision
        .options
        .iter()
        .find(|option| option.label.contains(name))
        .unwrap_or_else(|| panic!("missing {name} in {decision:?}"));
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![selected.id],
        },
    )
    .unwrap();
}

fn resolve_to_decision(game: &mut Game) {
    for _ in 0..32 {
        if !game.pending_decisions.is_empty() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    panic!("resolution did not settle");
}

#[test]
fn finale_search_keeps_chosen_x_for_library_and_graveyard() {
    for prepared in [false, true] {
        for zone in ["library", "graveyard"] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let search = card(122_000, cards::FINALE_OF_DEVASTATION_160, PlayerId::One);
            game.players[0].hand.push(search.clone());
            let candidates = vec![
                card(122_001, cards::GRIZZLY_BEARS, PlayerId::One),
                card(122_002, cards::SERRA_ANGEL, PlayerId::One),
            ];
            if zone == "library" {
                game.players[0].library = candidates;
            } else {
                game.players[0].graveyard = candidates;
            }
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
            game.apply(PlayerId::One, cast_action(search.id, vec![], vec![], 2))
                .unwrap();
            resolve_to_decision(&mut game);
            answer_named(&mut game, &format!("Search your {zone}."));
            resolve_to_decision(&mut game);
            let decision = &game.pending_decisions[0].observation;
            assert_eq!(decision.options.len(), 1, "X=2 admits only the Bears");
            answer_named(&mut game, "Grizzly Bears");
            drain_pending(&mut game);
            assert!(
                game.battlefield
                    .iter()
                    .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
            );
        }
    }
}

fn commander_fixture(commanders: Vec<CardDefinitionId>) -> Game {
    let deck = crate::Deck {
        commanders,
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
    game
}

#[test]
fn stinging_study_chooses_one_owned_commander_even_under_opposing_control() {
    for prepared in [false, true] {
        let mut game = commander_fixture(vec![cards::GRIZZLY_BEARS, cards::SERRA_ANGEL]);
        game.set_prepared_engine_enabled(prepared);
        let angel = game.players[0].command[1].id;
        game.move_target_to_zone(
            Target::Card(angel),
            ZoneKind::Battlefield,
            ZoneMoveCause::Rules,
            Some(BattlefieldArrival::under(PlayerId::Two)),
            ZonePlacement::Top,
        );
        drain_pending(&mut game);
        let study = card(122_010, cards::STINGING_STUDY_44, PlayerId::One);
        game.players[0].hand.push(study.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
        game.apply(PlayerId::One, cast_action(study.id, vec![], vec![], 0))
            .unwrap();
        let before = game.players[0].hand.len();
        resolve_to_decision(&mut game);
        assert_eq!(game.pending_decisions[0].observation.options.len(), 2);
        answer_named(&mut game, "Serra Angel");
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), before + 5);
        assert_eq!(game.players[0].life, 35);
    }
}

#[test]
fn reality_shift_uses_the_exiled_creatures_controller_and_allows_manifest_turn_up() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let mut victim = creature(123_000, cards::GRIZZLY_BEARS, PlayerId::One);
        victim.controller = PlayerId::Two;
        game.battlefield.push(victim);
        game.players[1].library = vec![card(123_001, cards::SERRA_ANGEL, PlayerId::Two)];
        let spell = card(123_002, cards::REALITY_SHIFT_46, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        game.apply(
            PlayerId::One,
            cast_action(
                spell.id,
                vec![Target::Permanent(GameObjectId(123_000))],
                vec![],
                0,
            ),
        )
        .unwrap();
        drain_pending(&mut game);
        assert!(
            game.players[0]
                .exile
                .iter()
                .any(|c| c.definition == cards::GRIZZLY_BEARS)
        );
        let manifested = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::SERRA_ANGEL)
            .unwrap();
        let id = manifested.card.id;
        assert_eq!(manifested.controller, PlayerId::Two);
        assert_eq!(game.power(manifested), Some(2));
        game.priority = PlayerId::Two;
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::White, 5);
        let action = game
            .legal_actions(PlayerId::Two)
            .into_iter()
            .find(|a| matches!(a, Action::TurnFaceUp { permanent } if *permanent == id))
            .expect("a manifested creature can turn face up for its mana cost");
        game.apply(PlayerId::Two, action).unwrap();
        assert_eq!(game.power(permanent(&game, id)), Some(4));
    }
}

#[test]
fn vindictive_flamestoker_reduces_activation_before_sacrifice_and_discards_whole_hand() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let mut source = creature(123_010, cards::VINDICTIVE_FLAMESTOKER_388, PlayerId::One);
        source.add_counters(CounterKind::named("oil"), 6);
        game.battlefield.push(source);
        game.players[0].hand = vec![
            card(123_011, cards::PLAINS, PlayerId::One),
            card(123_012, cards::ISLAND, PlayerId::One),
        ];
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == GameObjectId(123_010))).expect("six oil counters reduce the cost to R");
        game.apply(PlayerId::One, action).unwrap();
        assert!(
            game.battlefield
                .iter()
                .all(|p| p.card.id != GameObjectId(123_010))
        );
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), 4);
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::PLAINS)
        );
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::ISLAND)
        );
    }
}

#[test]
fn tarrasque_conditional_haste_depends_on_cast_provenance() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .push(creature(123_020, cards::THE_TARRASQUE_207, PlayerId::One));
        assert!(!game.permanent_has_executable_keyword(
            permanent(&game, GameObjectId(123_020)),
            KeywordAbility::Haste
        ));
        let spell = card(123_021, cards::THE_TARRASQUE_207, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 9);
        game.apply(PlayerId::One, cast_action(spell.id, vec![], vec![], 0))
            .unwrap();
        // Remove the fixture before the legend rule requires choosing one.
        game.battlefield.clear();
        drain_pending(&mut game);
        let cast = game.battlefield.last().unwrap();
        assert!(game.permanent_has_executable_keyword(cast, KeywordAbility::Haste));
    }
}

#[test]
fn culling_ritual_counts_only_destroyed_permanents_and_can_make_mixed_mana() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        for (id, definition, owner) in [
            (124_000, cards::GRIZZLY_BEARS, PlayerId::One),
            (124_001, cards::SAVANNAH_LIONS, PlayerId::Two),
            (124_002, cards::FOREST, PlayerId::One),
        ] {
            game.battlefield.push(creature(id, definition, owner));
        }
        // This zero-mana indestructible artifact is selected but not destroyed.
        game.put_onto_battlefield(PlayerId::Two, cards::DARKSTEEL_RELIC)
            .unwrap();
        drain_pending(&mut game);
        let ritual = card(124_003, cards::CULLING_RITUAL_337, PlayerId::One);
        game.players[0].hand.push(ritual.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
        game.apply(PlayerId::One, cast_action(ritual.id, vec![], vec![], 0))
            .unwrap();
        resolve_to_decision(&mut game);
        while !game.pending_decisions.is_empty() {
            let d = game.pending_decisions[0].observation.clone();
            let color = if game.players[0].mana_pool.black == 0 {
                "black"
            } else {
                "green"
            };
            let option = d
                .options
                .iter()
                .find(|o| o.label.to_lowercase().contains(color))
                .expect("the requested mana color is offered")
                .id;
            game.apply(
                d.player,
                Action::ChooseDecision {
                    decision: d.id,
                    options: vec![option],
                },
            )
            .unwrap();
            resolve_to_decision(&mut game);
        }
        drain_pending(&mut game);
        assert_eq!(game.players[0].mana_pool.total(), 2);
        assert_eq!(game.players[0].mana_pool.black, 1);
        assert_eq!(game.players[0].mana_pool.green, 1);
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::FOREST)
        );
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::DARKSTEEL_RELIC)
        );
    }
}

#[test]
fn flare_of_fortitude_freezes_life_and_protects_existing_permanents() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .push(creature(124_010, cards::GRIZZLY_BEARS, PlayerId::One));
        let flare = card(124_011, cards::FLARE_OF_FORTITUDE_26, PlayerId::One);
        game.players[0].hand.push(flare.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
        game.apply(PlayerId::One, cast_action(flare.id, vec![], vec![], 0))
            .unwrap();
        drain_pending(&mut game);
        let host = permanent(&game, GameObjectId(124_010));
        assert!(game.permanent_has_executable_keyword(host, KeywordAbility::Indestructible));
        assert!(game.permanent_has_executable_keyword(host, KeywordAbility::Hexproof));
        game.deal_combat_damage_to_player(GameObjectId(124_010), PlayerId::One, 3);
        assert_eq!(game.players[0].life, 20);
        game.battlefield
            .push(creature(124_012, cards::GRIZZLY_BEARS, PlayerId::One));
        assert!(!game.permanent_has_executable_keyword(
            permanent(&game, GameObjectId(124_012)),
            KeywordAbility::Indestructible
        ));
    }
}

#[test]
fn kozilek_manifest_uses_both_players_hands_and_draws_for_actual_arrivals() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.players[0].hand = vec![
            card(124_020, cards::FOREST, PlayerId::One),
            card(124_021, cards::ISLAND, PlayerId::One),
        ];
        game.players[1].hand = vec![card(124_022, cards::SERRA_ANGEL, PlayerId::Two)];
        let kozilek = card(124_023, cards::KOZILEK_THE_BROKEN_REALITY_10, PlayerId::One);
        game.players[0].hand.push(kozilek.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 9);
        game.apply(PlayerId::One, cast_action(kozilek.id, vec![], vec![], 0))
            .unwrap();
        resolve_to_decision(&mut game);
        while !game.pending_decisions.is_empty() {
            let d = game.pending_decisions[0].observation.clone();
            let options = d.options.iter().take(d.maximum).map(|o| o.id).collect();
            game.apply(
                d.player,
                Action::ChooseDecision {
                    decision: d.id,
                    options,
                },
            )
            .unwrap();
            resolve_to_decision(&mut game);
        }
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), 3);
        assert!(game.players[1].hand.is_empty());
        let manifested = game
            .battlefield
            .iter()
            .filter(|p| p.face_down.is_some())
            .collect::<Vec<_>>();
        assert_eq!(manifested.len(), 3);
        assert_eq!(
            manifested
                .iter()
                .filter(|p| p.controller == PlayerId::Two)
                .count(),
            1
        );
    }
}
