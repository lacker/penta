use super::*;

fn cast_target(
    game: &mut Game,
    definition: CardDefinitionId,
    target: Option<Target>,
) -> GameObjectId {
    let held = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 8);
    }
    game.priority = PlayerId::One;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::CastSpell { card, choices, .. } if *card == id && target.is_none_or(|target| choices.iter_targets().any(|chosen| *chosen == target)))
    }).expect("card has the requested legal cast");
    game.apply(PlayerId::One, action).unwrap();
    game.stack
        .iter()
        .find(|spell| {
            spell.card.definition.card_definition() == Some(definition)
                && spell.kind == StackObjectKind::Spell
        })
        .unwrap()
        .id
}

fn activate_target(game: &mut Game, source: GameObjectId, target: Target) {
    game.priority = PlayerId::One;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source: id, targets, .. } if *id == source && targets.iter().flat_map(TargetSelection::targets).any(|chosen| *chosen == target))
    }).expect("ability has the requested legal target");
    game.apply(PlayerId::One, action).unwrap();
    game.resolve_stack_top();
}

#[test]
fn color_layers_devoid_and_ghostfire_define_colors_in_every_zone() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        for definition in [
            cards::ELDRAZI_SKYSPAWNER,
            cards::FORERUNNER_OF_SLAUGHTER,
            cards::GHOSTFIRE,
        ] {
            let rules = &game.catalog.get(definition).unwrap().rules;
            assert!(
                !rules.printed_color_set().is_colorless(),
                "mana-cost colors remain available"
            );
            assert!(rules.color_set().is_colorless());
            for context in [
                CharacteristicContext::Library,
                CharacteristicContext::Hand,
                CharacteristicContext::Graveyard,
                CharacteristicContext::Exile,
                CharacteristicContext::Command,
                CharacteristicContext::Stack {
                    form: SpellForm::Part(CardPartId::PRIMARY),
                },
            ] {
                let view = game
                    .printed_trigger_event_object(
                        GameObjectId(950_000),
                        definition,
                        PlayerId::One,
                        &context,
                    )
                    .unwrap();
                assert_eq!(view.colors, [false; 5]);
            }
        }
        let thrall = &game.catalog.get(cards::CARRIER_THRALL).unwrap().rules;
        assert!(
            thrall.has_color(ManaColor::Black),
            "Carrier Thrall does not have devoid"
        );
    }
}

#[test]
fn color_layers_humility_removes_devoid_without_reversing_layer_five() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let eldrazi = game
            .put_onto_battlefield(PlayerId::One, cards::FORERUNNER_OF_SLAUGHTER)
            .unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::HUMILITY)
            .unwrap();
        let permanent = game
            .battlefield
            .iter()
            .find(|p| p.card.id == eldrazi)
            .unwrap();
        assert_eq!(game.permanent_colors(permanent), [false; 5]);
        assert_eq!(game.power(permanent), Some(1));
        assert_eq!(game.keyword_mask(permanent, None), 0);
        cast_target(
            &mut game,
            cards::THOUGHTLACE,
            Some(Target::Permanent(eldrazi)),
        );
        game.resolve_stack_top();
        assert_eq!(
            game.object_colors(eldrazi),
            [false, true, false, false, false]
        );
    }
}

#[test]
fn color_layers_gnomes_changes_spell_targets_and_carries_onto_the_permanent() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let gnomes = game
            .put_onto_battlefield(PlayerId::One, cards::ERSATZ_GNOMES)
            .unwrap();
        game.turns_started[0] += 1;
        let bears = cast_target(&mut game, cards::GRIZZLY_BEARS, None);
        activate_target(&mut game, gnomes, Target::Spell(bears));
        assert_eq!(
            game.stack_trigger_event_object(game.stack.iter().find(|s| s.id == bears).unwrap())
                .unwrap()
                .colors,
            [false; 5]
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        assert_eq!(rebuilt.object_colors(bears), [false; 5]);
        rebuilt.resolve_stack_top();
        let permanent = rebuilt
            .battlefield
            .iter()
            .find(|p| p.card.definition.card_definition() == Some(cards::GRIZZLY_BEARS))
            .unwrap();
        let permanent_id = permanent.card.id;
        assert_eq!(rebuilt.permanent_colors(permanent), [false; 5]);
        rebuilt.return_permanent_to_hand(permanent_id);
        let card = rebuilt.players[0].hand.last().unwrap();
        assert_eq!(
            rebuilt.object_colors(card.id),
            [false, false, false, false, true]
        );
    }
}

fn put_with_colors(
    game: &mut Game,
    definition: CardDefinitionId,
    indices: [usize; 2],
) -> GameObjectId {
    let id = game
        .put_onto_battlefield(PlayerId::One, definition)
        .unwrap();
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        (decision.minimum, decision.maximum, decision.options.len()),
        (2, 2, 5)
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: indices.map(|i| decision.options[i].id).to_vec(),
        },
    )
    .unwrap();
    id
}

fn activate_index(game: &mut Game, source: GameObjectId, index: u8, x: u16) {
    game.priority = PlayerId::One;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source: id, ability: AbilityOrigin::Printed { ability, .. }, x: chosen, .. }
            if *id == source && *ability == AbilityId(index) && *chosen == x)
    }).expect("requested activation is legal");
    game.apply(PlayerId::One, action).unwrap();
    game.resolve_stack_top();
}

#[test]
fn color_layers_ghostfire_blade_discounts_only_colorless_equip_targets() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let blade = game
            .put_onto_battlefield(PlayerId::One, cards::GHOSTFIRE_BLADE)
            .unwrap();
        let devoid = game
            .put_onto_battlefield(PlayerId::One, cards::FORERUNNER_OF_SLAUGHTER)
            .unwrap();
        let green = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let targets = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|a| match a {
                Action::ActivateAbility {
                    source, targets, ..
                } if source == blade => Some(targets),
                _ => None,
            })
            .flatten()
            .flat_map(|s| s.targets().to_vec())
            .collect::<Vec<_>>();
        assert!(targets.contains(&Target::Permanent(devoid)));
        assert!(!targets.contains(&Target::Permanent(green)));
        activate_target(&mut game, blade, Target::Permanent(devoid));
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
        let host = game
            .battlefield
            .iter()
            .find(|p| p.card.id == devoid)
            .unwrap();
        assert_eq!((game.power(host), game.toughness(host)), (Some(5), Some(4)));
    }
}

#[test]
fn color_layers_two_color_choices_survive_checkpoint_and_count_intersection() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let tablet = put_with_colors(&mut game, cards::TABLET_OF_THE_GUILDS, [2, 3]);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        assert_eq!(
            game.color_set_value(
                crate::card::ColorSetDef::ChosenBy(ObjectRefDef::Source),
                |_| Some(tablet)
            )
            .count(),
            2
        );
        let life = game.players[0].life;
        cast_target(&mut game, cards::FORERUNNER_OF_SLAUGHTER, None);
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, life, "devoid is not black or red");
        cast_target(
            &mut game,
            cards::LIGHTNING_BOLT,
            Some(Target::Player(PlayerId::Two)),
        );
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, life + 1);
        let target = game
            .battlefield
            .iter()
            .find(|p| p.card.definition.card_definition() == Some(cards::FORERUNNER_OF_SLAUGHTER))
            .unwrap()
            .card
            .id;
        cast_target(&mut game, cards::TERMINATE, Some(Target::Permanent(target)));
        drain_pending(&mut game);
        assert_eq!(
            game.players[0].life,
            life + 3,
            "each matching color contributes once"
        );
    }
}

#[test]
fn color_layers_seal_reduces_generic_cost_for_each_matching_color() {
    let mut game = ready_game();
    put_with_colors(&mut game, cards::SEAL_OF_THE_GUILDPACT, [1, 4]);
    let bears = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap()
        .remove(0);
    let id = bears.id;
    game.players[0].hand.push(bears);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn color_layers_spirit_dragon_x_preserves_devoid_and_pays_loyalty() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let ugin = game
            .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_SPIRIT_DRAGON)
            .unwrap();
        let devoid = game
            .put_onto_battlefield(PlayerId::One, cards::FORERUNNER_OF_SLAUGHTER)
            .unwrap();
        let bears = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        activate_index(&mut game, ugin, 1, 2);
        assert!(game.battlefield.iter().any(|p| p.card.id == devoid));
        assert!(!game.battlefield.iter().any(|p| p.card.id == bears));
        let pw = game.battlefield.iter().find(|p| p.card.id == ugin).unwrap();
        assert_eq!(pw.counters(CounterKind::Loyalty), 5);
        assert!(pw.activated_loyalty_this_turn);
        assert!(
            !game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == ugin))
        );
    }
}

#[test]
fn color_layers_ineffable_link_survives_source_and_ignores_unrelated_exits() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
            .unwrap();
        let ugin = game
            .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_INEFFABLE)
            .unwrap();
        let bears = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        activate_index(&mut game, ugin, 1, 0);
        let spirit = game
            .battlefield
            .iter()
            .find(|p| p.card.definition.is_token())
            .unwrap()
            .card
            .id;
        assert_eq!(game.players[0].exile.len(), 1);
        assert_eq!(game.observe(PlayerId::Two).face_down_exile_sizes[0], 1);
        game.destroy_permanent_without_regeneration(bears);
        game.destroy_permanent_without_regeneration(ugin);
        drain_pending(&mut game);
        assert_eq!(game.players[0].exile.len(), 1);
        assert_eq!(
            game.observe(PlayerId::One).exiles[0].len(),
            1,
            "the controller can still look"
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        assert_eq!(game.players[0].exile[0].definition, cards::LIGHTNING_BOLT);
        game.set_prepared_engine_enabled(prepared);
        game.return_permanent_to_hand(spirit);
        drain_pending(&mut game);
        assert!(game.players[0].exile.is_empty());
        assert!(
            game.players[0]
                .hand
                .iter()
                .any(|c| c.definition == cards::LIGHTNING_BOLT),
            "hand after return: {:?}",
            game.players[0].hand
        );
    }
}

#[test]
fn color_layers_ineffable_empty_library_still_creates_a_spirit() {
    let mut game = ready_game();
    game.players[0].library.clear();
    let ugin = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_INEFFABLE)
        .unwrap();
    activate_index(&mut game, ugin, 1, 0);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn color_layers_titans_presence_requires_a_reveal_and_reads_its_current_power() {
    let mut game = ready_game();
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let presence = game
        .build_zone(PlayerId::One, &[cards::TITANS_PRESENCE])
        .unwrap()
        .remove(0);
    let id = presence.id;
    game.players[0].hand.push(presence);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
    );
    let revealed = game
        .build_zone(PlayerId::One, &[cards::FORERUNNER_OF_SLAUGHTER])
        .unwrap()
        .remove(0);
    let reveal_id = revealed.id;
    game.players[0].hand.push(revealed);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.players[0].hand.iter().any(|c| c.id == reveal_id));
    assert!(
        game.observe(PlayerId::Two)
            .public_reveals
            .iter()
            .any(|(_, card, _)| *card == reveal_id)
    );
    game.resolve_stack_top();
    assert!(!game.battlefield.iter().any(|p| p.card.id == target));
}

#[test]
fn color_layers_humility_and_opalescence_keep_started_components() {
    for prepared in [false, true] {
        for order in [
            [cards::HUMILITY, cards::OPALESCENCE, cards::OPALESCENCE],
            [cards::OPALESCENCE, cards::HUMILITY, cards::OPALESCENCE],
            [cards::OPALESCENCE, cards::OPALESCENCE, cards::HUMILITY],
        ] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let ids = order.map(|card| game.put_onto_battlefield(PlayerId::One, card).unwrap());
            let expected = if order[2] == cards::HUMILITY {
                [1, 1, 1]
            } else if order[1] == cards::HUMILITY {
                [4, 4, 1]
            } else {
                [4, 4, 4]
            };
            for (id, power) in ids.into_iter().zip(expected) {
                let permanent = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
                assert_eq!(game.power(permanent), Some(power), "order {order:?}");
                assert!(game.collect_effective_abilities(permanent, None).is_empty());
            }
        }
    }
}

#[test]
fn color_layers_spell_copies_keep_copy_exceptions_but_not_gnomes_effects() {
    let mut game = ready_game();
    let gnomes = game
        .put_onto_battlefield(PlayerId::One, cards::ERSATZ_GNOMES)
        .unwrap();
    game.turns_started[0] += 1;
    let bears = cast_target(&mut game, cards::GRIZZLY_BEARS, None);
    activate_target(&mut game, gnomes, Target::Spell(bears));
    let spell = game.stack.iter().find(|s| s.id == bears).unwrap().clone();
    game.push_copy(spell, PlayerId::One, vec![]);
    let copied = game.stack.last().unwrap();
    assert_eq!(
        game.object_colors(copied.id),
        [false, false, false, false, true]
    );
    let copied = copied.clone();
    game.push_copy_with_colors(
        copied,
        PlayerId::One,
        vec![],
        Some(ColorSet::from_colors(&[ManaColor::Red])),
    );
    let red = game.stack.last().unwrap().clone();
    game.push_copy(red, PlayerId::One, vec![]);
    assert_eq!(
        game.object_colors(game.stack.last().unwrap().id),
        [false, false, false, true, false]
    );
}

#[test]
fn color_layers_cultivator_mana_covers_three_independent_permissions() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let drone = game
            .put_onto_battlefield(PlayerId::One, cards::CULTIVATOR_DRONE)
            .unwrap();
        let colorless = game
            .put_onto_battlefield(PlayerId::One, cards::FORERUNNER_OF_SLAUGHTER)
            .unwrap();
        let colored = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.turns_started[0] += 1;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::ActivateManaAbility { source, .. } if *source == drone))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        let mana = *game.players[0].mana.last().unwrap();
        let ability = |source| ManaPaymentPurpose::Ability {
            source,
            taps_source: false,
            leaves_source: false,
        };
        assert!(game.mana_can_pay_for_cost(mana, &ability(colorless), mana_cost!("{1}")));
        assert!(!game.mana_can_pay_for_cost(mana, &ability(colored), mana_cost!("{1}")));
        assert!(game.mana_can_pay_for_cost(mana, &ability(colored), mana_cost!("{1}{C}")));
        let hand_card = game
            .build_zone(PlayerId::One, &[cards::FORERUNNER_OF_SLAUGHTER])
            .unwrap()
            .remove(0);
        let hand_id = hand_card.id;
        game.players[0].hand.push(hand_card);
        assert!(!game.mana_can_pay_for_cost(mana, &ability(hand_id), mana_cost!("{1}")));
        assert!(game.mana_can_pay_for_cost(mana, &ManaPaymentPurpose::Other, mana_cost!("{C}")));
        assert!(!game.mana_can_pay_for_cost(mana, &ManaPaymentPurpose::Other, mana_cost!("{1}")));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        activate_target(&mut game, colorless, Target::Permanent(colorless));
        let host = game
            .battlefield
            .iter()
            .find(|p| p.card.id == colorless)
            .unwrap();
        assert!(game.permanent_has_executable_keyword(host, KeywordAbility::Haste));
    }
}

#[test]
fn color_layers_eye_and_ineffable_discount_devoid_spells() {
    for source in [cards::EYE_OF_UGIN, cards::UGIN_THE_INEFFABLE] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, source).unwrap();
        let card = game
            .build_zone(PlayerId::One, &[cards::ELDRAZI_SKYSPAWNER])
            .unwrap()
            .remove(0);
        let id = card.id;
        game.players[0].hand.push(card);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
            .unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn color_layers_consign_targets_devoid_and_rechecks_color_at_resolution() {
    for repaint in [false, true] {
        let mut game = ready_game();
        let spell = cast_target(&mut game, cards::FORERUNNER_OF_SLAUGHTER, None);
        cast_target(
            &mut game,
            cards::CONSIGN_TO_MEMORY,
            Some(Target::Spell(spell)),
        );
        if repaint {
            cast_target(&mut game, cards::THOUGHTLACE, Some(Target::Spell(spell)));
            game.resolve_stack_top();
        }
        drain_pending(&mut game);
        assert_eq!(
            game.battlefield.iter().any(
                |p| p.card.definition.card_definition() == Some(cards::FORERUNNER_OF_SLAUGHTER)
            ),
            repaint
        );
    }
}

static COLORLESS_EVERYWHERE: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Objects in every zone are colorless.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
            ObjectPredicateDef::Any,
            &[
                ZoneKind::Library,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                ZoneKind::Exile,
                ZoneKind::Command,
                ZoneKind::Stack,
                ZoneKind::Battlefield,
            ],
            PlayerRelation::Any,
        ))),
        effect: AppliedEffectDef::set_colors(ColorSet::empty()),
    },
)];

#[test]
fn color_layers_static_colors_apply_across_zones_even_after_humility_removes_the_ability() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield.push(token_permanent(
            970_000,
            crate::card::TokenCharacteristics::creature(&[], &[], 2, 2)
                .with_abilities(&COLORLESS_EVERYWHERE),
            PlayerId::One,
        ));
        game.put_onto_battlefield(PlayerId::One, cards::HUMILITY)
            .unwrap();
        let source = &game.battlefield[0];
        assert!(game.collect_effective_abilities(source, None).is_empty());
        for (zone, context) in [
            (ZoneKind::Hand, CharacteristicContext::Hand),
            (ZoneKind::Library, CharacteristicContext::Library),
            (ZoneKind::Graveyard, CharacteristicContext::Graveyard),
            (ZoneKind::Exile, CharacteristicContext::Exile),
            (ZoneKind::Command, CharacteristicContext::Command),
            (
                ZoneKind::Stack,
                CharacteristicContext::Stack {
                    form: SpellForm::Part(CardPartId::PRIMARY),
                },
            ),
        ] {
            let view = game
                .printed_trigger_event_object(
                    GameObjectId(970_001),
                    cards::GRIZZLY_BEARS,
                    PlayerId::One,
                    &context,
                )
                .unwrap();
            assert_eq!(view.colors, [false; 5], "{zone:?}");
        }
        let bears = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        assert_eq!(game.object_colors(bears), [false; 5]);
    }
}

#[test]
fn color_layers_titans_presence_retains_power_as_the_revealed_card_leaves_hand() {
    let mut game = ready_game();
    game.battlefield.push(token_permanent(
        970_002,
        crate::card::TokenCharacteristics::creature(&[], &[], 2, 2)
            .with_abilities(&COLORLESS_EVERYWHERE),
        PlayerId::One,
    ));
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::HILL_GIANT)
        .unwrap();
    game.players[0].hand = game
        .build_zone(
            PlayerId::One,
            &[
                cards::MARO,
                cards::FOREST,
                cards::FOREST,
                cards::TITANS_PRESENCE,
            ],
        )
        .unwrap();
    let maro = game.players[0].hand[0].id;
    let presence = game.players[0].hand[3].id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { card, choices, .. } if *card == presence && choices.iter_targets().any(|chosen| *chosen == Target::Permanent(target)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.current_or_last_known_power(maro), Some(3));
    game.discard_cards(PlayerId::One, &[maro]);
    let rest = game.players[0]
        .hand
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    game.discard_cards(PlayerId::One, &rest);
    assert_eq!(game.current_or_last_known_power(maro), Some(3));
    game.resolve_stack_top();
    assert!(!game.battlefield.iter().any(|p| p.card.id == target));
}

#[test]
fn color_layers_spirit_dragon_ultimate_puts_only_chosen_permanent_cards_into_play() {
    let mut game = ready_game();
    let ugin = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_SPIRIT_DRAGON)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ugin)
        .unwrap()
        .set_counters(CounterKind::Loyalty, 10);
    game.players[0].library = game
        .build_zone(
            PlayerId::One,
            &[
                cards::FOREST,
                cards::GRIZZLY_BEARS,
                cards::SOL_RING,
                cards::HUMILITY,
                cards::GHOSTFIRE,
                cards::LIGHTNING_BOLT,
                cards::CONSIGN_TO_MEMORY,
            ],
        )
        .unwrap();
    let life = game.players[0].life;
    activate_index(&mut game, ugin, 2, 0);
    assert_eq!(game.players[0].life, life + 7);
    let choice = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(choice.minimum, 0);
    assert_eq!(choice.options.len(), 4);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: choice.options.iter().map(|o| o.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.players[0].hand.len(), 3);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition.card_definition() == Some(cards::GRIZZLY_BEARS))
    );
}

#[test]
fn color_layers_eye_search_finds_devoid_creatures_and_has_no_mana_ability() {
    let mut game = ready_game();
    let eye = game
        .put_onto_battlefield(PlayerId::One, cards::EYE_OF_UGIN)
        .unwrap();
    game.players[0].library = game
        .build_zone(
            PlayerId::One,
            &[cards::FORERUNNER_OF_SLAUGHTER, cards::GRIZZLY_BEARS],
        )
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 7);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::ActivateManaAbility { source, .. } if *source == eye))
    );
    activate_index(&mut game, eye, 1, 0);
    let choice = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(choice.options.len(), 1);
    choose_decision_by_label(&mut game, PlayerId::One, "Forerunner of Slaughter");
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::FORERUNNER_OF_SLAUGHTER)
    );
}

#[test]
fn color_layers_eye_of_the_storms_triggers_for_devoid_casts() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::UGIN_EYE_OF_THE_STORMS)
        .unwrap();
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast_target(&mut game, cards::FORERUNNER_OF_SLAUGHTER, None);
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == bears));
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition.card_definition() == Some(cards::FORERUNNER_OF_SLAUGHTER))
    );
}

#[test]
fn color_layers_restricted_mana_uses_the_announced_face_down_spell() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let drone = game
            .put_onto_battlefield(PlayerId::One, cards::CULTIVATOR_DRONE)
            .unwrap();
        game.turns_started[0] += 1;
        let angel = game
            .build_zone(PlayerId::One, &[cards::EXALTED_ANGEL])
            .unwrap()
            .remove(0);
        let id = angel.id;
        game.players[0].hand.push(angel);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
            .expect("Cultivator Drone can fund the colorless face-down spell");
        assert!(
            game.mana_sources_for_action(PlayerId::One, &cast)
                .contains(&drone)
        );
        game.apply(PlayerId::One, cast).unwrap();
        let spell = game.stack.last().unwrap();
        assert!(spell.face_down.is_some());
        assert_eq!(game.object_colors(spell.id), [false; 5]);
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == drone)
                .unwrap()
                .tapped
        );
    }
}

#[test]
fn color_layers_humility_precedes_dependent_ability_grants_in_either_order() {
    for prepared in [false, true] {
        for humility_first in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.players[0].library = game
                .build_zone(PlayerId::One, &[cards::DARK_RITUAL])
                .unwrap();
            if humility_first {
                game.put_onto_battlefield(PlayerId::One, cards::HUMILITY)
                    .unwrap();
            }
            let vampire = game
                .put_onto_battlefield(PlayerId::One, cards::VAMPIRE_NOCTURNUS)
                .unwrap();
            if !humility_first {
                game.put_onto_battlefield(PlayerId::One, cards::HUMILITY)
                    .unwrap();
            }
            let permanent = game
                .battlefield
                .iter()
                .find(|p| p.card.id == vampire)
                .unwrap();
            assert_eq!(
                (game.power(permanent), game.toughness(permanent)),
                (Some(1), Some(1))
            );
            assert!(!game.has_flying(permanent));
        }
    }
}
