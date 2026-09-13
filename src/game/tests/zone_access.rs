//! Zone selection, current knowledge, and permission restrictions compose independently.
use super::top_library::{casts, mana, restored, settle, staged, top};
use super::*;
use crate::card::{
    PlayActionMatcherDef, PlayCostDef, PlayPermissionDef, PlayRestrictionDef, ZonePositionDef,
};

fn with_access(game: &mut Game, abilities: &'static [AbilityDef]) {
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let source = definitions
        .iter_mut()
        .find(|definition| definition.id == cards::FUTURE_SIGHT)
        .unwrap();
    source.rules = CardRules::new_enchantment(ManaCost::default()).with_abilities(abilities);
    synchronize_single_part_definition(source);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.prepared_engine = crate::prepared_engine::PreparedEngine::compile(&game.catalog);
}

#[test]
fn a_zone_position_is_selected_before_card_characteristics_are_tested() {
    let game = staged(&[], &[cards::GRIZZLY_BEARS, cards::FOREST]);
    let query = ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Library],
        PlayerRelation::You,
    )
    .at(ZonePositionDef::FromTop(0));
    assert!(
        game.objects_matching_query(
            query,
            PlayerId::One,
            GameObjectId(0),
            TriggerContext::empty()
        )
        .is_empty()
    );
    assert_eq!(
        game.objects_matching_query(
            query.at(ZonePositionDef::FromTop(1)),
            PlayerId::One,
            GameObjectId(0),
            TriggerContext::empty()
        ),
        vec![Target::Card(game.players[0].library[0].id)]
    );
}

#[test]
fn knowledge_and_casting_can_select_a_different_position_in_the_same_zone() {
    static ACCESS: [AbilityDef; 2] = [
        abilities::cards_known_to(
            "The second card is known to you.",
            ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Library],
                PlayerRelation::You,
            )
            .at(ZonePositionDef::FromTop(1)),
            PlayerSetDef::Related(PlayerRelation::You),
        ),
        abilities::play_from_zone(
            ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Library],
                PlayerRelation::You,
            )
            .at(ZonePositionDef::FromTop(1)),
            "You may cast the second card.",
            PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Any),
        ),
    ];
    for prepared in [false, true] {
        let mut game = staged(
            &[cards::FUTURE_SIGHT],
            &[cards::GRIZZLY_BEARS, cards::FOREST],
        );
        with_access(&mut game, &ACCESS);
        game.set_prepared_engine_enabled(prepared);
        mana(&mut game);
        let card = game.players[0].library[0].id;
        let shown = game.observe(PlayerId::One).known_cards;
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].card, card);
        assert_eq!(shown[0].position_from_top, 1);
        assert_eq!(game.observe(PlayerId::One).revealed_library_top, None);
        assert!(game.observe(PlayerId::Two).known_cards.is_empty());
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action,
            Action::CastSpell { card: candidate, .. } if *candidate == card)
            })
            .unwrap();
        let mut game = restored(&game);
        assert_eq!(game.observe(PlayerId::One).known_cards, shown);
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].library.len(), 1);
        assert_eq!(game.players[0].library[0].definition, cards::FOREST);
    }
}

#[test]
fn a_graveyard_permission_uses_the_same_position_cost_and_selection() {
    static ACCESS: [AbilityDef; 1] = [AbilityDef::static_ability(
        "You may cast the first graveyard card by paying its mana value in life.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlay(
                PlayPermissionDef::new(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )
                    .at(ZonePositionDef::FromTop(0)),
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::Any,
                    ),
                )
                .with_cost(PlayCostDef::LifeEqualToManaValue)
                .with_limit(1, false),
            )),
        },
    )];
    let mut game = staged(&[cards::FUTURE_SIGHT], &[cards::FOREST]);
    with_access(&mut game, &ACCESS);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::SOL_RING, cards::GRIZZLY_BEARS])
        .unwrap();
    let first = game.players[0].graveyard[1].id;
    let older = game.players[0].graveyard[0].id;
    let actions = game.legal_actions(PlayerId::One);
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a, Action::CastSpell { card, .. } if *card == older))
    );
    let cast = actions
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == first))
        .unwrap();
    let life = game.players[0].life;
    game.apply(PlayerId::One, cast).unwrap();
    assert_eq!(game.players[0].life, life - 2);
    settle(&mut game);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
}

#[test]
fn continuous_knowledge_can_reach_an_opponents_hand_without_granting_a_play() {
    static ACCESS: [AbilityDef; 1] = [abilities::cards_known_to(
        "You know the opponent's hand.",
        ObjectQueryDef::matching(
            ObjectPredicateDef::Any,
            &[ZoneKind::Hand],
            PlayerRelation::Opponent,
        ),
        PlayerSetDef::Related(PlayerRelation::You),
    )];
    let mut game = staged(&[cards::FUTURE_SIGHT], &[cards::FOREST]);
    with_access(&mut game, &ACCESS);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .unwrap();
    let shown = game.observe(PlayerId::One).known_cards;
    assert_eq!(shown.len(), 1);
    assert_eq!(shown[0].definition, cards::LIGHTNING_BOLT);
    assert_eq!(shown[0].zone, ZoneKind::Hand);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
    assert_eq!(restored(&game).observe(PlayerId::One).known_cards, shown);
    game.battlefield.clear();
    assert!(game.observe(PlayerId::One).known_cards.is_empty());
}

#[test]
fn mystic_forge_accepts_colored_artifacts_and_colorless_spells_but_never_lands() {
    for (card, accepted) in [
        (cards::ETHERIUM_SCULPTOR, true),
        (cards::ALL_IS_DUST, true),
        (cards::GRIZZLY_BEARS, false),
        (cards::FOREST, false),
    ] {
        let mut game = staged(&[cards::MYSTIC_FORGE], &[card]);
        mana(&mut game);
        assert_eq!(!casts(&game).is_empty(), accepted, "{card:?}");
        assert!(
            !game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|a| matches!(a, Action::PlayLand { .. }))
        );
        assert_eq!(game.observe(PlayerId::One).known_cards.len(), 1);
        assert!(game.observe(PlayerId::Two).known_cards.is_empty());
    }
}

#[test]
fn mystic_forge_exiles_the_current_top_at_resolution_and_pays_life_up_front() {
    let mut game = staged(
        &[cards::MYSTIC_FORGE],
        &[cards::FOREST, cards::GRIZZLY_BEARS],
    );
    let life = game.players[0].life;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].life, life - 1);
    assert!(game.battlefield[0].tapped);
    game.draw_cards(PlayerId::One, 1);
    let exiled = top(&game);
    let mut game = restored(&game);
    settle(&mut game);
    assert!(game.players[0].library.is_empty());
    assert_eq!(game.players[0].exile[0].definition, cards::FOREST);
    assert_ne!(
        game.players[0].exile[0].id, exiled,
        "zone moves mint a successor"
    );
}

#[test]
fn a_zone_plot_grant_uses_its_declared_cost() {
    static ACCESS: [AbilityDef; 1] = [AbilityDef::static_ability(
        "You may plot the first card of your graveyard for one blue mana.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlot {
                cards: ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )
                .at(ZonePositionDef::FromTop(0)),
                ability: &abilities::plot(&[CostDef::Mana(mana_cost!("{U}"))]),
            }),
        },
    )];
    let mut game = staged(&[cards::FUTURE_SIGHT], &[cards::FOREST]);
    with_access(&mut game, &ACCESS);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let card = game.players[0].graveyard[0].id;
    let action = Action::Plot { card };
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.players[0].graveyard.is_empty());
    let exiled = &game.players[0].exile[0];
    assert_eq!(exiled.definition, cards::GRIZZLY_BEARS);
    assert!(game.plotted_cards.contains_key(&exiled.id));
    assert_ne!(card, exiled.id);
}
