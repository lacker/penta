//! Pool doubling makes fresh mana; existing units retain their provenance.
use super::*;

fn activate(game: &mut Game, source: GameObjectId, color: Option<ManaColor>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source: id, color: produced, .. }
            if *id == source && color.is_none_or(|color| color == *produced))
        })
        .expect("mana activation is offered");
    game.apply(PlayerId::One, action)
        .expect("mana ability resolves");
}

fn put(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let permanent = creature(id, definition, PlayerId::One);
    let id = permanent.card.id;
    game.battlefield.push(permanent);
    id
}

#[test]
fn mana_pool_doubling_keeps_lotus_restrictions_only_on_original_mana() {
    let mut game = ready_game();
    let lotus = put(&mut game, 10_000, cards::JEWELED_LOTUS);
    let cube = put(&mut game, 10_001, cards::DOUBLING_CUBE);
    activate(&mut game, lotus, Some(ManaColor::Blue));
    let original = game.players[0].mana.clone();
    assert_eq!(original.len(), 3);
    assert!(
        original
            .iter()
            .all(|mana| mana.restrictions == [ManaRestrictionDef::CastYourCommander])
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::JEWELED_LOTUS)
    );
    assert_eq!(
        game.eligible_mana_pool(PlayerId::One, &ManaPaymentPurpose::Other)
            .total(),
        0
    );
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == cube)
    ));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    activate(&mut game, cube, None);
    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].mana_pool.blue, 6);
    assert_eq!(game.players[0].mana_pool.colorless, 0);
    for mana in &original {
        assert!(game.players[0].mana.contains(mana));
    }
    let fresh = game.players[0]
        .mana
        .iter()
        .filter(|mana| mana.source.is_some_and(|s| s.object == cube))
        .collect::<Vec<_>>();
    assert_eq!(fresh.len(), 3);
    assert!(
        fresh
            .iter()
            .all(|mana| mana.restrictions.is_empty() && mana.spend_effects.is_empty())
    );
    let spent = game.pay_player_cost(PlayerId::One, mana_cost!("{U}{U}{U}"), 0);
    assert!(
        spent
            .iter()
            .all(|mana| mana.source.is_some_and(|s| s.object == cube))
    );
    assert_eq!(game.players[0].mana, original);
}

#[test]
fn mana_pool_doubling_counts_all_six_types_after_paying_costs() {
    let mut game = ready_game();
    let cube = put(&mut game, 10_000, cards::DOUBLING_CUBE);
    for color in ManaColor::ALL {
        game.add_unrestricted_mana(
            PlayerId::One,
            color,
            if color == ManaColor::Colorless { 4 } else { 1 },
        );
    }
    activate(&mut game, cube, None);
    for color in ManaColor::ALL {
        assert_eq!(game.players[0].mana_pool.amount(color), 2);
    }
}

#[test]
fn mana_pool_doubling_can_resolve_with_nothing_left_to_double() {
    let mut game = ready_game();
    let cube = put(&mut game, 10_000, cards::DOUBLING_CUBE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    activate(&mut game, cube, None);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == cube)
            .unwrap()
            .tapped
    );
}

#[test]
fn mana_pool_doubling_does_not_copy_cavern_spend_effects_or_snow_provenance() {
    let mut game = ready_game();
    let cube = put(&mut game, 10_000, cards::DOUBLING_CUBE);
    let cavern = put(&mut game, 10_001, cards::CAVERN_OF_SOULS);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == cavern)
        .unwrap()
        .chosen_creature_type = Some("Soldier".into());
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == cavern)
        .unwrap()
        .chosen_creature_type_binding = Some("cavern_creature_type".into());
    let snow = put(&mut game, 10_002, cards::SNOW_COVERED_ISLAND);
    activate(&mut game, cavern, Some(ManaColor::Blue));
    activate(&mut game, snow, Some(ManaColor::Blue));
    let original = game.players[0].mana.clone();
    assert!(original.iter().any(|mana| !mana.spend_effects.is_empty()));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    activate(&mut game, cube, None);
    assert_eq!(game.players[0].mana_pool.blue, 4);
    assert_eq!(&game.players[0].mana[..2], original.as_slice());
    assert!(game.players[0].mana[2..].iter().all(|mana| {
        mana.source.is_some_and(|s| s.object == cube)
            && mana.restrictions.is_empty()
            && mana.spend_effects.is_empty()
    }));
    let snow_payment = ManaPaymentPurpose::Payment {
        label: None,
        source: cube,
        snow: true,
    };
    assert_eq!(
        game.eligible_mana_pool(PlayerId::One, &snow_payment).blue,
        1
    );
}

#[test]
fn mana_pool_doubling_lotus_mana_cannot_cast_an_ordinary_spell_or_eligible_legend() {
    let mut game = ready_game();
    let lotus = put(&mut game, 10_000, cards::JEWELED_LOTUS);
    activate(&mut game, lotus, Some(ManaColor::Blue));
    for definition in [cards::ANCESTRAL_RECALL, cards::EMRY_LURKER_OF_THE_LOCH] {
        let spell = ManaPaymentPurpose::Spell {
            object: GameObjectId(11_000),
            commander_owner: None,
            definition,
            controller: PlayerId::One,
            form: SpellForm::Part(CardPartId::PRIMARY),
            reserved_life_payment: 0,
        };
        assert_eq!(game.eligible_mana_pool(PlayerId::One, &spell).total(), 0);
    }
}

#[test]
fn mana_pool_doubling_pays_channel_shortfall_before_counting() {
    let mut game = ready_game();
    resolve_channel(&mut game);
    let cube = put(&mut game, 10_000, cards::DOUBLING_CUBE);
    let lotus = put(&mut game, 10_001, cards::JEWELED_LOTUS);
    activate(&mut game, lotus, Some(ManaColor::Green));
    let life = game.players[0].life;
    activate(&mut game, cube, None);
    assert_eq!(game.players[0].life, life - 3);
    assert_eq!(game.players[0].mana_pool.green, 6);
    assert_eq!(game.players[0].mana_pool.colorless, 0);
}

fn commander_game(format: Format) -> Game {
    let deck = Deck {
        commanders: vec![cards::SQUEE_GOBLIN_NABOB, cards::THALIA_GUARDIAN_OF_THRABEN],
        main: vec![cards::MOUNTAIN; 99],
        sideboard: Vec::new(),
    };
    let mut game = Game::new_with_format(
        format,
        crate::card::catalog().unwrap(),
        [deck.clone(), deck],
        1,
    )
    .unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game
}

fn cast_action(game: &Game, object: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == object))
}

fn spell_payment(game: &Game, object: GameObjectId, controller: PlayerId) -> ManaPaymentPurpose {
    let (_, card) = game.card_in_nonbattlefield_zone(object).unwrap();
    ManaPaymentPurpose::Spell {
        object,
        commander_owner: game.commander_owner(object),
        definition: card.definition,
        controller,
        form: SpellForm::Part(CardPartId::PRIMARY),
        reserved_life_payment: 0,
    }
}

fn choose_commander_destination(game: &mut Game, option: u32) {
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
}

#[test]
fn jeweled_lotus_auto_pays_for_each_designated_commander_in_both_formats() {
    for format in [Format::Cedh, Format::DuelCommander] {
        for index in 0..2 {
            let mut game = commander_game(format);
            let commander = game.players[0].command[index].id;
            assert!(cast_action(&game, commander).is_none());
            let lotus = put(&mut game, 10_000, cards::JEWELED_LOTUS);
            // Control of the Lotus, not ownership of its physical card, decides
            // who gets the mana and which player's commander it can pay for.
            game.battlefield.last_mut().unwrap().card.owner = PlayerId::Two;
            let action = cast_action(&game, commander).expect("Lotus can fund the commander");
            assert_eq!(
                game.mana_sources_for_action(PlayerId::One, &action),
                vec![lotus]
            );
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.commanders[index].casts, 1);
            assert_eq!(game.commanders[1 - index].casts, 0);
            assert!(
                game.players[1]
                    .graveyard
                    .iter()
                    .any(|c| c.definition == cards::JEWELED_LOTUS)
            );
            assert_eq!(game.players[0].mana_pool.total(), u16::from(index != 0));
        }
    }
}

#[test]
fn jeweled_lotus_pays_commander_tax_after_a_real_command_zone_return() {
    let mut game = commander_game(Format::Cedh);
    let commander = game.players[0].command[0].id;
    put(&mut game, 10_000, cards::JEWELED_LOTUS);
    game.apply(PlayerId::One, cast_action(&game, commander).unwrap())
        .unwrap();
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|p| game.is_commander(p.card.id))
        .unwrap()
        .card
        .id;
    game.return_permanent_to_hand(permanent);
    choose_commander_destination(&mut game, 0);
    let returned = game.players[0]
        .command
        .iter()
        .find(|c| c.definition == cards::SQUEE_GOBLIN_NABOB)
        .unwrap()
        .id;
    assert_ne!(returned, commander);
    let lotus = put(&mut game, 10_001, cards::JEWELED_LOTUS);
    activate(&mut game, lotus, Some(ManaColor::Red));
    assert!(
        cast_action(&game, returned).is_none(),
        "three mana cannot cover the five-mana taxed cost"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(PlayerId::One, cast_action(&game, returned).unwrap())
        .unwrap();
    assert_eq!(game.commanders[0].casts, 2);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn jeweled_lotus_follows_physical_identity_into_hand_and_rejects_other_uses() {
    let mut game = commander_game(Format::Cedh);
    let commander = game.players[0].command[0].id;
    game.move_target_to_zone(
        Target::Card(commander),
        ZoneKind::Hand,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    choose_commander_destination(&mut game, 1);
    let held = game.players[0].hand[0].id;
    assert_ne!(commander, held);
    game.players[0]
        .hand
        .push(card(10_010, cards::SQUEE_GOBLIN_NABOB, PlayerId::One));
    let ordinary = game.players[0].hand[1].id;
    let enemy = game.players[1].command[0].id;
    let lotus = put(&mut game, 10_000, cards::JEWELED_LOTUS);
    activate(&mut game, lotus, Some(ManaColor::Red));
    let mana = game.players[0].mana[0];
    assert!(game.mana_can_pay_for(mana, &spell_payment(&game, held, PlayerId::One)));
    assert!(!game.mana_can_pay_for(mana, &spell_payment(&game, ordinary, PlayerId::One)));
    assert!(!game.mana_can_pay_for(mana, &spell_payment(&game, enemy, PlayerId::One)));
    assert!(!game.mana_can_pay_for(mana, &spell_payment(&game, held, PlayerId::Two)));
    assert!(!game.mana_can_pay_for(
        mana,
        &ManaPaymentPurpose::Ability {
            source: held,
            taps_source: false,
            leaves_source: false
        }
    ));
    assert!(!game.mana_can_pay_for(
        mana,
        &ManaPaymentPurpose::Payment {
            source: held,
            label: None,
            snow: false
        }
    ));
    assert!(cast_action(&game, ordinary).is_none());
    game.apply(PlayerId::One, cast_action(&game, held).unwrap())
        .unwrap();
    assert_eq!(
        game.commanders[0].casts, 0,
        "casting from hand does not increase tax"
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn doubling_cube_preserves_commander_only_mana_through_checkpoint_and_both_casts() {
    let mut game = commander_game(Format::Cedh);
    let commander = game.players[0].command[0].id;
    game.players[0]
        .hand
        .push(card(10_010, cards::SQUEE_GOBLIN_NABOB, PlayerId::One));
    let ordinary = game.players[0].hand[0].id;
    let lotus = put(&mut game, 10_000, cards::JEWELED_LOTUS);
    let cube = put(&mut game, 10_001, cards::DOUBLING_CUBE);
    activate(&mut game, lotus, Some(ManaColor::Red));
    let original = game.players[0].mana.clone();
    assert!(cast_action(&game, commander).is_some());
    assert!(cast_action(&game, ordinary).is_none());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    activate(&mut game, cube, None);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
            .unwrap();
    assert_eq!(game.players[0].mana, restored.players[0].mana);
    assert_eq!(
        game.legal_actions(PlayerId::One),
        restored.legal_actions(PlayerId::One)
    );
    restored
        .apply(PlayerId::One, cast_action(&restored, ordinary).unwrap())
        .unwrap();
    assert_eq!(
        restored.players[0].mana, original,
        "ordinary spell spends only Cube's fresh mana"
    );
    drain_pending(&mut restored);
    restored
        .apply(PlayerId::One, cast_action(&restored, commander).unwrap())
        .unwrap();
    assert_eq!(
        restored.players[0].mana_pool.total(),
        0,
        "the commander can spend the original Lotus mana"
    );
}
