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
        .chosen_creature_type_binding = Some(crate::Binding!("cavern_creature_type"));
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
