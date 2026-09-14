use super::top_library::{mana, restored, staged};
use super::*;

fn shape(game: &Game, index: usize) -> (Option<i16>, Option<i16>, bool) {
    let permanent = &game.battlefield[index];
    (
        game.power(permanent),
        game.toughness(permanent),
        game.has_flying(permanent),
    )
}

#[test]
fn vampire_nocturnus_reveals_only_the_top_and_rechecks_its_color() {
    for prepared in [false, true] {
        let mut game = staged(
            &[
                cards::VAMPIRE_NOCTURNUS,
                cards::CHILD_OF_NIGHT,
                cards::GRIZZLY_BEARS,
            ],
            &[
                cards::SWAMP,
                cards::TERMINATE,
                cards::FOREST,
                cards::DARK_RITUAL,
                cards::LIGHTNING_BOLT,
            ],
        );
        game.set_prepared_engine_enabled(prepared);
        game.put_onto_battlefield(PlayerId::Two, cards::CHILD_OF_NIGHT)
            .unwrap();
        mana(&mut game);

        // A black card below the top is insufficient; multicolor black cards count,
        // while Swamps and an empty library do not.
        for active in [false, true, false, true, false, false] {
            assert_eq!(
                shape(&game, 0),
                if active {
                    (Some(5), Some(4), true)
                } else {
                    (Some(3), Some(3), false)
                }
            );
            assert_eq!(
                shape(&game, 1),
                if active {
                    (Some(4), Some(2), true)
                } else {
                    (Some(2), Some(1), false)
                }
            );
            assert_eq!(shape(&game, 2), (Some(2), Some(2), false));
            assert_eq!(shape(&game, 3), (Some(2), Some(1), false));
            for viewer in [PlayerId::One, PlayerId::Two] {
                let known = game.observe(viewer).known_cards;
                let expected = game.players[0]
                    .library
                    .last()
                    .map(|card| (card.id, card.definition, 0))
                    .into_iter()
                    .collect::<Vec<_>>();
                assert_eq!(
                    known
                        .iter()
                        .map(|card| (card.card, card.definition, card.position_from_top))
                        .collect::<Vec<_>>(),
                    expected
                );
            }
            assert!(!game.legal_actions(PlayerId::One).iter().any(|action| {
                matches!(action, Action::CastSpell { .. } | Action::PlayLand { .. })
            }));
            if active {
                let checkpoint = restored(&game);
                assert_eq!(shape(&checkpoint, 0), shape(&game, 0));
                assert_eq!(
                    checkpoint.observe(PlayerId::Two).known_cards,
                    game.observe(PlayerId::Two).known_cards
                );
            }
            game.players[0].library.pop();
        }
    }
}

#[test]
fn vampire_nocturnus_includes_itself_without_the_vampire_subtype() {
    for prepared in [false, true] {
        let mut game = staged(
            &[
                cards::VAMPIRE_NOCTURNUS,
                cards::CHILD_OF_NIGHT,
                cards::FOREST,
            ],
            &[cards::DARK_RITUAL],
        );
        game.set_prepared_engine_enabled(prepared);
        for index in [0, 1] {
            let id = game.battlefield[index].card.id;
            attach_constant_resolved_characteristics(
                &mut game,
                id,
                &[AppliedEffectDef::set_creature_types(
                    CreatureTypeSetDef::named(&["Frog"]),
                )],
                ContinuousEffectExpiration::Never,
            );
        }
        let land = game.battlefield[2].card.id;
        attach_constant_resolved_characteristics(
            &mut game,
            land,
            &[
                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Kindred)),
                AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Vampire"])),
            ],
            ContinuousEffectExpiration::Never,
        );
        assert_eq!(shape(&game, 0), (Some(5), Some(4), true));
        assert_eq!(shape(&game, 1), (Some(2), Some(1), false));
        assert!(
            !game.has_flying(&game.battlefield[2]),
            "a noncreature Vampire is excluded"
        );
    }
}

#[test]
fn vampire_nocturnus_follows_its_controller_and_loses_its_grants_with_its_abilities() {
    for prepared in [false, true] {
        let mut game = staged(
            &[cards::VAMPIRE_NOCTURNUS, cards::CHILD_OF_NIGHT],
            &[cards::DARK_RITUAL],
        );
        game.set_prepared_engine_enabled(prepared);
        game.players[1].library = game.build_zone(PlayerId::Two, &[cards::FOREST]).unwrap();
        game.put_onto_battlefield(PlayerId::Two, cards::CHILD_OF_NIGHT)
            .unwrap();
        game.battlefield[0].controller = PlayerId::Two;
        assert_eq!(shape(&game, 0), (Some(3), Some(3), false));
        assert_eq!(shape(&game, 1), (Some(2), Some(1), false));
        assert_eq!(shape(&game, 2), (Some(2), Some(1), false));
        for viewer in [PlayerId::One, PlayerId::Two] {
            let known = game.observe(viewer).known_cards;
            assert_eq!(known.len(), 1);
            assert_eq!(known[0].card, game.players[1].library[0].id);
        }

        game.players[1].library = game
            .build_zone(PlayerId::Two, &[cards::DARK_RITUAL])
            .unwrap();
        assert_eq!(shape(&game, 0), (Some(5), Some(4), true));
        assert_eq!(shape(&game, 1), (Some(2), Some(1), false));
        assert_eq!(shape(&game, 2), (Some(4), Some(2), true));
        let source = game.battlefield[0].card.id;
        attach_constant_resolved_characteristics(
            &mut game,
            source,
            &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
            ContinuousEffectExpiration::Never,
        );
        assert_eq!(shape(&game, 0), (Some(3), Some(3), false));
        assert_eq!(shape(&game, 2), (Some(2), Some(1), false));
        for viewer in [PlayerId::One, PlayerId::Two] {
            assert!(game.observe(viewer).known_cards.is_empty());
        }
    }
}
