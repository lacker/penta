use super::*;

const BLADE: GameObjectId = GameObjectId(240_000);
const HOST: GameObjectId = GameObjectId(240_001);

static COLORS: [ManaColor; 5] = ManaColor::COLORS;

fn board(colors: &'static [ManaColor], prepared: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.battlefield
        .push(creature(BLADE.0, cards::DRAGONFIRE_BLADE, PlayerId::One));
    game.battlefield.push(token_permanent(
        HOST.0,
        crate::card::TokenCharacteristics::creature(&[], colors, 2, 2),
        PlayerId::One,
    ));
    game
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield.iter().find(|p| p.card.id == id).unwrap()
}

fn equip(game: &Game, target: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == BLADE && targets.iter().flat_map(TargetSelection::targets).any(|t| *t == Target::Permanent(target)))
    })
}

#[test]
fn dragonfire_blade_prices_and_pays_each_targets_color_count() {
    for prepared in [false, true] {
        for count in 0..=5 {
            let mut game = board(&COLORS[..count], prepared);
            let cost = 4_u16.saturating_sub(u16::try_from(count).unwrap());
            if cost > 0 {
                game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, cost - 1);
                assert!(
                    equip(&game, HOST).is_none(),
                    "{count} colors: insufficient mana"
                );
                game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
            }
            let action = equip(&game, HOST).expect("the discounted cost is affordable");
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].mana_pool, ManaPool::default());
            assert_eq!(
                permanent(&game, BLADE).attached_to,
                None,
                "equip uses the stack"
            );
            drain_pending(&mut game);
            assert_eq!(permanent(&game, BLADE).attached_to, Some(HOST));
            assert_eq!(game.power(permanent(&game, HOST)), Some(4));
            assert_eq!(game.toughness(permanent(&game, HOST)), Some(4));
        }
    }
}

#[test]
fn dragonfire_blade_reprices_live_colors_and_keeps_equip_timing_and_control() {
    let mut game = board(&[ManaColor::Green], true);
    game.battlefield[1] = creature(HOST.0, cards::GRIZZLY_BEARS, PlayerId::One);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    assert!(equip(&game, HOST).is_none());
    attach_constant_resolved_characteristics(
        &mut game,
        HOST,
        &[AppliedEffectDef::set_colors(ColorSet::from_colors(&[
            ManaColor::Red,
            ManaColor::Blue,
        ]))],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert!(
        equip(&game, HOST).is_some(),
        "use current colors, not the token's initial color"
    );
    game.step = Step::Upkeep;
    assert!(equip(&game, HOST).is_none());
    game.step = Step::PrecombatMain;
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == HOST)
        .unwrap()
        .controller = PlayerId::Two;
    assert!(equip(&game, HOST).is_none());
}

#[test]
fn dragonfire_blade_hexproof_filters_opposing_source_colors_and_follows_attachment() {
    for prepared in [false, true] {
        let mut game = board(&COLORS, prepared);
        let action = equip(&game, HOST).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        for (offset, definition, allowed) in [
            (2, cards::LIGHTNING_BOLT, false),
            (3, cards::TERMINATE, true),
            (4, cards::DISMEMBER, false),
            (5, cards::SOL_RING, true),
        ] {
            let source = card(BLADE.0 + offset, definition, PlayerId::Two);
            let id = source.id;
            game.players[1].hand.push(source);
            assert_eq!(
                game.permanent_can_be_targeted_by(permanent(&game, HOST), PlayerId::Two, id, true),
                allowed
            );
            assert!(
                game.permanent_can_be_targeted_by(permanent(&game, HOST), PlayerId::One, id, true),
                "its controller may still target it"
            );
        }
        let pinger = creature(240_010, cards::PRODIGAL_SORCERER, PlayerId::Two);
        let pinger_id = pinger.card.id;
        game.battlefield.push(pinger);
        assert!(!game.permanent_can_be_targeted_by(
            permanent(&game, HOST),
            PlayerId::Two,
            pinger_id,
            false
        ));
        game.damage_target_from(Some(pinger_id), Some(Target::Permanent(HOST)), 1);
        assert_eq!(
            permanent(&game, HOST).damage,
            1,
            "hexproof does not prevent damage"
        );
        game.battlefield
            .push(creature(240_011, cards::GRIZZLY_BEARS, PlayerId::One));
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == BLADE)
            .unwrap()
            .attached_to = Some(GameObjectId(240_011));
        assert!(game.permanent_can_be_targeted_by(
            permanent(&game, HOST),
            PlayerId::Two,
            pinger_id,
            false
        ));
        assert_eq!(game.power(permanent(&game, HOST)), Some(2));
        assert!(!game.permanent_can_be_targeted_by(
            permanent(&game, GameObjectId(240_011)),
            PlayerId::Two,
            pinger_id,
            false
        ));
        game.battlefield.retain(|p| p.card.id != BLADE);
        assert!(game.permanent_can_be_targeted_by(
            permanent(&game, GameObjectId(240_011)),
            PlayerId::Two,
            pinger_id,
            false
        ));
    }
}

#[test]
fn dragonfire_blade_pending_equip_and_attached_grant_survive_reconstruction() {
    let mut game = board(&[ManaColor::Green], true);
    game.battlefield[1] = creature(HOST.0, cards::GRIZZLY_BEARS, PlayerId::One);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = equip(&game, HOST).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    drain_pending(&mut rebuilt);
    assert_eq!(permanent(&rebuilt, BLADE).attached_to, Some(HOST));
    let (wire, hidden) = checkpoint_fixture(&rebuilt, PlayerId::One);
    let mut rebuilt = Game::from_observation_checkpoint(
        rebuilt.catalog.clone(),
        rebuilt.format,
        &wire,
        &hidden,
        0,
    )
    .unwrap();
    let bolt = rebuilt
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .unwrap()
        .pop()
        .unwrap();
    let bolt_id = bolt.id;
    rebuilt.players[1].hand.push(bolt);
    assert_eq!(rebuilt.power(permanent(&rebuilt, HOST)), Some(4));
    assert!(!rebuilt.permanent_can_be_targeted_by(
        permanent(&rebuilt, HOST),
        PlayerId::Two,
        bolt_id,
        true
    ));
}

#[test]
fn dragonfire_blade_mana_preview_uses_the_chosen_targets_discount() {
    let mut game = board(&[ManaColor::Red, ManaColor::Blue, ManaColor::Green], true);
    let mountain = creature(240_030, cards::MOUNTAIN, PlayerId::One);
    let land_id = mountain.card.id;
    game.battlefield.push(mountain);
    game.battlefield.push(token_permanent(
        240_031,
        crate::card::TokenCharacteristics::creature(&[], &[], 2, 2),
        PlayerId::One,
    ));
    assert!(equip(&game, GameObjectId(240_031)).is_none());
    let action = equip(&game, HOST).unwrap();
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![land_id]
    );
    assert!(!permanent(&game, land_id).tapped);
    game.apply(PlayerId::One, action).unwrap();
    assert!(permanent(&game, land_id).tapped);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn dragonfire_blade_reduces_the_total_after_cost_increases() {
    let mut game = board(&COLORS, true);
    attach_constant_resolved_characteristics(
        &mut game,
        BLADE,
        &[
            AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Enchantment)),
        ],
        ContinuousEffectExpiration::EndOfTurn,
    );
    game.battlefield
        .push(creature(240_040, cards::GLOOM, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(
        equip(&game, HOST).is_none(),
        "four plus three minus five costs two"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let action = equip(&game, HOST).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    drain_pending(&mut game);
    assert_eq!(permanent(&game, BLADE).attached_to, Some(HOST));
}

#[test]
fn dragonfire_blade_hexproof_checks_live_ability_source_colors() {
    let mut game = board(&COLORS, true);
    let action = equip(&game, HOST).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    let source = creature(240_050, cards::PRODIGAL_SORCERER, PlayerId::Two);
    let source_id = source.card.id;
    game.battlefield.push(source);
    assert!(!game.permanent_can_be_targeted_by(
        permanent(&game, HOST),
        PlayerId::Two,
        source_id,
        false
    ));
    attach_constant_resolved_characteristics(
        &mut game,
        source_id,
        &[AppliedEffectDef::add_colors(ColorSet::from_colors(&[
            ManaColor::Red,
        ]))],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert!(game.permanent_can_be_targeted_by(
        permanent(&game, HOST),
        PlayerId::Two,
        source_id,
        false
    ));
}

#[test]
fn target_color_discount_locks_combined_mana_before_tapping_changes_colors() {
    const ABILITIES: &[AbilityDef] = &[
        AbilityDef::activated_with_targets(
            "Tap, pay three: draw a card. Discount for the target's colors.",
            &[
                CostDef::TapSource,
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::Mana(mana_cost!("{2}")),
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Source,
            )],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_cost_reduction(
            ValueDef::ColorCount(ObjectRefDef::Target(TargetIndex::PRIMARY)),
            0,
        ),
        AbilityDef::static_ability(
            "This creature is colorless while tapped.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceIsTapped,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_colors(ColorSet::empty()),
                },
            },
        ),
    ];
    let (mut game, source) = cost_lists::game_with_cost_rules(
        &CardRules::new_creature(mana_cost!("{U}"), &[], 2, 2).with_abilities(ABILITIES),
    );
    game.prepared_engine = crate::prepared_engine::PreparedEngine::compile(&game.catalog);
    game.turns_started = [3, 3];
    let source_card = game.players[0].hand.pop().unwrap();
    game.battlefield.push(Permanent::entering(
        source_card,
        CardPartId::PRIMARY,
        PlayerId::One,
        0,
        0,
    ));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(permanent(&game, source).tapped);
    assert_eq!(game.object_color_count(source), 0);
    assert_eq!(
        game.players[0].mana_pool.colorless, 1,
        "the full three mana gets one discount, locked while the target was blue"
    );
}

#[test]
fn color_count_composes_with_target_source_attachment_and_bound_references() {
    for prepared in [false, true] {
        let mut game = board(&[], prepared);
        game.battlefield[1] = creature(HOST.0, cards::LOXODON_SMITER, PlayerId::One);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        game.apply(PlayerId::One, equip(&game, HOST).unwrap())
            .unwrap();
        let resolving = game.stack.last().unwrap().clone();
        let binding = Binding!("counted_object");
        let mut context = EffectResolutionContext::empty();
        context.bind_single_object(binding, Some(Target::Permanent(HOST)));
        let count = |game: &Game, reference| {
            game.effect_value(
                ValueDef::ColorCount(reference),
                &resolving,
                &context,
                ScopedEffect::primary(EffectDef::None),
            )
        };
        assert_eq!(count(&game, ObjectRefDef::Target(TargetIndex::PRIMARY)), 2);
        assert_eq!(count(&game, ObjectRefDef::Binding(binding)), 2);
        assert_eq!(count(&game, ObjectRefDef::Source), 0);
        assert_eq!(count(&game, ObjectRefDef::AttachedToSource), 0);
        assert_eq!(count(&game, ObjectRefDef::Binding(Binding!("missing"))), 0);

        drain_pending(&mut game);
        assert_eq!(count(&game, ObjectRefDef::AttachedToSource), 2);
        let source_count = ValueDef::ColorCount(ObjectRefDef::Source);
        let attached_count = ValueDef::ColorCount(ObjectRefDef::AttachedToSource);
        assert_eq!(game.static_stat_value(source_count, HOST, PlayerId::One), 2);
        assert_eq!(
            game.static_stat_value(attached_count, BLADE, PlayerId::One),
            2
        );
        assert_eq!(
            game.cost_reduction_value(source_count, PlayerId::One, HOST),
            2
        );
        assert_eq!(
            game.cost_reduction_value(attached_count, PlayerId::One, BLADE),
            2
        );

        attach_constant_resolved_characteristics(
            &mut game,
            HOST,
            &[AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                ManaColor::Red,
            ]))],
            ContinuousEffectExpiration::EndOfTurn,
        );
        assert_eq!(count(&game, ObjectRefDef::Binding(binding)), 1);
        assert_eq!(count(&game, ObjectRefDef::AttachedToSource), 1);
        assert_eq!(game.static_stat_value(source_count, HOST, PlayerId::One), 1);
        assert_eq!(
            game.cost_reduction_value(attached_count, PlayerId::One, BLADE),
            1
        );

        game.move_target_to_zone(
            Target::Permanent(HOST),
            ZoneKind::Graveyard,
            ZoneMoveCause::Effect {
                controller: PlayerId::One,
            },
            None,
            ZonePlacement::Top,
        );
        assert_eq!(
            count(&game, ObjectRefDef::Binding(binding)),
            1,
            "a bound reference retains the old object's last-known colors"
        );
        let card = game.players[0].graveyard.last().unwrap();
        assert_eq!(
            game.object_color_count(card.id),
            2,
            "the new graveyard object has its printed colors"
        );
    }
}
