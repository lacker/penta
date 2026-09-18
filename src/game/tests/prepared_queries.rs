use super::*;

/// Exercise the same lower-layer contexts that recursive characteristic
/// readers install, restoring them even if a differential assertion fails.
struct QueryLayerPasses([bool; 3]);

impl QueryLayerPasses {
    fn enter(bits: u8) -> Self {
        Self([
            crate::game::ability_layers::STATIC_ABILITY_LAYER_PASS
                .with(|pass| pass.replace(bits & 1 != 0)),
            crate::game::continuous_effects::STATIC_SET_CHARACTERISTIC_LAYER_PASS
                .with(|pass| pass.replace(bits & 2 != 0)),
            crate::game::creature_characteristics::STATIC_POWER_TOUGHNESS_LAYER_PASS
                .with(|pass| pass.replace(bits & 4 != 0)),
        ])
    }
}

impl Drop for QueryLayerPasses {
    fn drop(&mut self) {
        crate::game::ability_layers::STATIC_ABILITY_LAYER_PASS.with(|pass| pass.set(self.0[0]));
        crate::game::continuous_effects::STATIC_SET_CHARACTERISTIC_LAYER_PASS
            .with(|pass| pass.set(self.0[1]));
        crate::game::creature_characteristics::STATIC_POWER_TOUGHNESS_LAYER_PASS
            .with(|pass| pass.set(self.0[2]));
    }
}

#[test]
fn ability_read_memo_matches_uncached_layers_and_excludes_same_id_snapshots() {
    let mut game = query_board();
    game.battlefield.extend([
        creature(98_450, cards::BLOOD_MOON, PlayerId::Two),
        creature(98_451, cards::HUMILITY, PlayerId::Two),
    ]);
    for stage in 0..3 {
        if stage == 1 {
            game.battlefield
                .retain(|p| p.card.definition != cards::HUMILITY);
        } else if stage == 2 {
            game.battlefield[1].face_down = Some(crate::card::face_down::disguise());
            game.battlefield[2].copy_effect =
                Some(Game::copiable_characteristics(&game.battlefield[1]));
        }
        let mut snapshot = game.battlefield[1].clone();
        snapshot.face_down = Some(crate::card::face_down::ordinary());
        let collect = |bits, x| {
            let _passes = QueryLayerPasses::enter(bits);
            game.prospective_x.set(x);
            game.battlefield
                .iter()
                .map(|permanent| game.collect_effective_abilities(permanent, None))
                .chain([
                    game.collect_effective_abilities(&snapshot, None),
                    game.collect_effective_abilities(&game.battlefield[1], Some(&snapshot)),
                ])
                .collect::<Vec<_>>()
        };
        let contexts = (0..8)
            .flat_map(|bits| [None, Some(0), Some(3)].map(|x| (bits, x)))
            .collect::<Vec<_>>();
        let expected = contexts
            .iter()
            .map(|&(bits, x)| collect(bits, x))
            .collect::<Vec<_>>();
        let read = game.hold_board_read_memo();
        assert!(game.ability_read_key(&snapshot, None).is_none());
        for index in (0..contexts.len()).chain((0..contexts.len()).rev()) {
            let (bits, x) = contexts[index];
            assert_eq!(
                collect(bits, x),
                expected[index],
                "stage {stage}, passes {bits}, X {x:?}"
            );
        }
        drop(read);
        game.prospective_x.set(None);
    }
}

#[test]
fn inline_read_memo_excludes_temporary_views_and_expires_before_mutation() {
    let mut game = query_board();
    game.battlefield[1].face_down = Some(crate::card::face_down::disguise());
    let expected = *game.effective_rules(&game.battlefield[1]).unwrap();
    let read = game.hold_board_read_memo();
    assert_eq!(
        *game.effective_rules(&game.battlefield[1]).unwrap(),
        expected
    );
    let mut snapshot = game.battlefield[1].clone();
    snapshot.face_down = Some(crate::card::face_down::ordinary());
    assert_ne!(*game.effective_rules(&snapshot).unwrap(), expected);
    snapshot.face_down = Some(crate::card::face_down::disguise());
    assert_eq!(*game.effective_rules(&snapshot).unwrap(), expected);
    drop(read);
    game.battlefield[1].face_down = None;
    let _read = game.hold_board_read_memo();
    assert_eq!(
        game.effective_rules(&game.battlefield[1])
            .unwrap()
            .creature_stats(),
        Some(crate::CreatureStats {
            power: 4,
            toughness: 4
        })
    );
}

fn query_board() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.extend([
        creature(98_400, cards::TAIGA, PlayerId::One),
        creature(98_401, cards::SERRA_ANGEL, PlayerId::One),
        creature(98_402, cards::KIRD_APE, PlayerId::Two),
    ]);
    game
}

#[test]
fn prepared_read_memo_preserves_prospective_order_and_nested_game_lifetime() {
    let mut game = query_board();
    let moon = creature(98_403, cards::BLOOD_MOON, PlayerId::Two);
    let source_ids = |game: &Game, prospective: Option<&Permanent>| {
        game.land_type_effect_sources(prospective)
            .into_iter()
            .map(|(source, timestamp)| (source.card.id, timestamp))
            .collect::<Vec<_>>()
    };
    let outer = game.hold_land_type_query_memo();
    assert!(source_ids(&game, None).is_empty());
    assert!(game.prepared_land_type_sources().is_some());
    let prospective = source_ids(&game, Some(&moon));
    assert_eq!(prospective.len(), 1);
    assert_eq!(prospective[0].0, moon.card.id);
    assert!(source_ids(&game, None).is_empty());
    // A nested read of another game cannot borrow this game's source index.
    let mut other = game.clone();
    other.battlefield.push(moon.clone());
    let inner = other.hold_land_type_query_memo();
    assert!(other.prepared_land_type_sources().is_some());
    assert!(game.prepared_land_type_sources().is_none());
    assert_eq!(source_ids(&other, None)[0].0, moon.card.id);
    drop(inner);
    assert!(game.prepared_land_type_sources().is_some());
    assert!(source_ids(&game, None).is_empty());
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _inner = other.hold_land_type_query_memo();
        assert_eq!(source_ids(&other, None)[0].0, moon.card.id);
        panic!("exercise nested land-type read unwinding");
    }));
    assert!(failed.is_err());
    assert!(source_ids(&game, None).is_empty());
    assert!(game.prepared_land_type_sources().is_some());
    drop(outer);
    game.battlefield.push(moon.clone());
    let expected = source_ids(&game, None);
    {
        let _read = game.hold_land_type_query_memo();
        assert_eq!(
            source_ids(&game, Some(&moon)),
            expected,
            "same-id prospective source does not duplicate the current source"
        );
    }
    game.set_prepared_engine_enabled(false);
    assert_eq!(source_ids(&game, None), expected);
}

#[test]
fn prepared_predicate_queries_match_reference_across_live_characteristics() {
    static COMPOSITE: ObjectPredicateDef = ObjectPredicateDef::All(&[
        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Color(ManaColor::White),
        ]),
    ]);
    let mut game = query_board();
    game.players[0]
        .graveyard
        .push(card(98_410, cards::FOREST, PlayerId::One));
    let source = game.battlefield[0].card.id;
    for stage in 0..4 {
        match stage {
            1 => game
                .battlefield
                .push(creature(98_411, cards::BLOOD_MOON, PlayerId::Two)),
            2 => game.battlefield[1].face_down = Some(crate::card::face_down::ordinary()),
            3 => {
                game.battlefield[2].copy_effect =
                    Some(Game::copiable_characteristics(&game.battlefield[0]));
            }
            _ => {}
        }
        for predicate in [
            COMPOSITE,
            ObjectPredicateDef::Any,
            ObjectPredicateDef::Source,
            ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Forest,
                BasicLandType::Mountain,
            ]),
            ObjectPredicateDef::Subtype(crate::SubtypeDef::from_name("Forest")),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::ColorCount(1),
            ObjectPredicateDef::ManaValueAtMost(3),
            ObjectPredicateDef::Tapped,
            // Unsupported roots keep the snapshot-based matcher.
            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            ObjectPredicateDef::PowerExactly(2),
        ] {
            let query = ObjectQueryDef::matching(
                predicate,
                &[ZoneKind::Battlefield, ZoneKind::Graveyard],
                PlayerRelation::You,
            );
            game.set_prepared_engine_enabled(false);
            let reference =
                game.objects_matching_query(query, PlayerId::One, source, TriggerContext::empty());
            game.set_prepared_engine_enabled(true);
            let prepared =
                game.objects_matching_query(query, PlayerId::One, source, TriggerContext::empty());
            assert_eq!(
                prepared, reference,
                "stage {stage}, predicate {predicate:?}"
            );
            let prospective = creature(source.0, cards::BLOOD_MOON, PlayerId::One);
            let prepared = game.objects_matching_query_with_prospective(
                query,
                PlayerId::One,
                source,
                TriggerContext::empty(),
                Some(&prospective),
            );
            game.set_prepared_engine_enabled(false);
            let reference = game.objects_matching_query_with_prospective(
                query,
                PlayerId::One,
                source,
                TriggerContext::empty(),
                Some(&prospective),
            );
            assert_eq!(prepared, reference, "prospective query");
        }
    }
}

#[test]
fn prepared_ability_tables_and_keywords_preserve_dynamic_overlays() {
    let mut game = query_board();
    assert!(game.prepared_keyword_mask(&game.battlefield[1]).is_some());
    for stage in 0..5 {
        match stage {
            1 => game.battlefield[1]
                .temporary_keywords
                .push(KeywordAbility::Haste),
            2 => game.battlefield[1].face_down = Some(crate::card::face_down::ordinary()),
            3 => {
                game.battlefield[1].face_down = None;
                game.battlefield[1].copy_effect =
                    Some(Game::copiable_characteristics(&game.battlefield[2]));
            }
            4 => game
                .battlefield
                .push(creature(98_420, cards::HUMILITY, PlayerId::Two)),
            _ => {}
        }
        for permanent in game.battlefield.clone() {
            game.set_prepared_engine_enabled(false);
            let abilities = game.effective_abilities(&permanent);
            let keywords = game.keyword_mask(&permanent, None);
            game.set_prepared_engine_enabled(true);
            assert_eq!(
                game.effective_abilities(&permanent),
                abilities,
                "stage {stage}"
            );
            assert_eq!(
                game.keyword_mask(&permanent, None),
                keywords,
                "stage {stage}"
            );
        }
    }
}

#[test]
fn prepared_seeded_games_match_every_observation_action_and_event() {
    use crate::{Policy, RandomPolicy};
    let names = crate::protocol::deck_names();
    for seed in 1..=8 {
        let index = usize::try_from(seed).unwrap();
        let decks = [
            crate::protocol::deck_by_name(names[index % names.len()]).unwrap(),
            crate::protocol::deck_by_name(names[(index * 7 + 3) % names.len()]).unwrap(),
        ];
        let mut prepared = Game::new(crate::card::catalog().unwrap(), decks, seed).unwrap();
        let mut reference = prepared.clone();
        reference.set_prepared_engine_enabled(false);
        let mut policy = RandomPolicy::new(seed ^ 0x517c_c1b7_2722_0a95);
        for step in 0..10_000 {
            let Some(player) = prepared.decision_player() else {
                break;
            };
            let observation = prepared.observe(player);
            let expected = reference.observe(player);
            assert_eq!(observation, expected, "seed {seed}, step {step}");
            let Some(action) = policy.choose_action(&observation) else {
                break;
            };
            prepared
                .apply_observed_action(&observation, action.clone())
                .unwrap();
            reference.apply_observed_action(&expected, action).unwrap();
            assert_eq!(
                prepared.events, reference.events,
                "seed {seed}, step {step}"
            );
            assert_eq!(prepared.pending_events, reference.pending_events);
            if prepared.result().is_some() {
                break;
            }
        }
        assert!(prepared.result().is_some(), "seed {seed} must finish");
        assert_eq!(prepared.result(), reference.result());
        assert_eq!(prepared.players, reference.players);
        assert_eq!(prepared.battlefield, reference.battlefield);
    }
}

#[test]
fn prepared_player_rule_filters_preserve_permissions_and_restriction_order() {
    let mut game = query_board();
    game.battlefield.extend([
        creature(98_430, cards::STEEL_GOLEM, PlayerId::One),
        creature(98_431, cards::SHIMMER_MYR, PlayerId::One),
    ]);
    let artifact = card(98_432, cards::SOL_RING, PlayerId::One);
    let option = game.catalog.get(cards::SOL_RING).unwrap().play_options[0].clone();
    for stage in 0..3 {
        if stage == 1 {
            game.battlefield[3].controller = PlayerId::Two;
        }
        if stage == 2 {
            game.battlefield.remove(4);
        }
        let collect = |game: &Game, player| {
            let mut restrictions = Vec::new();
            let result = game.visit_play_restrictions(player, |restriction| {
                restrictions.push(restriction);
                ControlFlow::Continue(())
            });
            assert!(result.is_continue());
            restrictions
        };
        for player in [PlayerId::One, PlayerId::Two] {
            game.set_prepared_engine_enabled(false);
            let restrictions = collect(&game, player);
            let permission = game.cast_as_though_it_had_flash(&artifact, player, &option);
            game.set_prepared_engine_enabled(true);
            assert_eq!(collect(&game, player), restrictions);
            assert_eq!(
                game.cast_as_though_it_had_flash(&artifact, player, &option),
                permission
            );
            if stage == 0 && player == PlayerId::One {
                assert_eq!(restrictions.len(), 1);
                assert!(permission);
            }
        }
    }
}

#[test]
fn prepared_queries_preserve_changed_color_and_land_type_words() {
    let mut game = ready_game();
    let mut source = creature(98_950, cards::BOG_WRAITH, PlayerId::One);
    let source_id = source.card.id;
    for word in [
        TextWordChange::Color {
            from: ManaColor::Black,
            to: ManaColor::Red,
        },
        TextWordChange::BasicLandType {
            from: BasicLandType::Swamp,
            to: BasicLandType::Island,
        },
    ] {
        source.text_changes.push(TextChange {
            word,
            expiration: ContinuousEffectExpiration::Never,
        });
    }
    game.battlefield = vec![
        source,
        creature(98_951, cards::HILL_GIANT, PlayerId::One),
        creature(98_952, cards::ISLAND, PlayerId::One),
        creature(98_953, cards::SWAMP, PlayerId::One),
    ];
    for (predicate, expected) in [
        (
            ObjectPredicateDef::Color(ManaColor::Black),
            GameObjectId(98_951),
        ),
        (
            ObjectPredicateDef::Subtype(crate::SubtypeDef::from_name("Swamp")),
            GameObjectId(98_952),
        ),
        (
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
            GameObjectId(98_952),
        ),
    ] {
        let query =
            ObjectQueryDef::matching(predicate, &[ZoneKind::Battlefield], PlayerRelation::Any);
        for enabled in [false, true] {
            game.set_prepared_engine_enabled(enabled);
            assert_eq!(
                game.objects_matching_query(
                    query,
                    PlayerId::One,
                    source_id,
                    TriggerContext::empty()
                ),
                vec![Target::Permanent(expected)]
            );
        }
    }
}
