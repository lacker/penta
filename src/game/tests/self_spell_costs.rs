use super::*;
use crate::card::{CostAdjustmentDef, CostAmountDef, abilities, sets};
use crate::prepared_engine::PreparedEngine;

fn cast(game: &Game, source: GameObjectId, option: PlayOptionId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == source && choices.play_option() == option)
        })
}

fn resolve_yawgmoths_will(game: &mut Game) {
    game.players[0]
        .hand
        .push(card(90_000, cards::YAWGMOTH_S_WILL, PlayerId::One));
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 2;
    let action = cast(game, GameObjectId(90_000), PlayOptionId::DEFAULT).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(game);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn crab_discount_combines_with_external_modifiers_from_graveyard_and_exile() {
    for prepared in [false, true] {
        for zone in [ZoneKind::Graveyard, ZoneKind::Exile] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let crab = card(20_000, cards::EDDYMURK_CRAB, PlayerId::One);
            match zone {
                ZoneKind::Graveyard => game.players[0].graveyard.push(crab),
                ZoneKind::Exile => game.players[0].exile.push(crab),
                _ => unreachable!(),
            }
            game.players[0].mana_pool.blue = 2;
            game.players[0].mana_pool.colorless = 5;
            assert!(cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT).is_none());
            game.players[0].mana_pool = ManaPool::default();
            if zone == ZoneKind::Graveyard {
                resolve_yawgmoths_will(&mut game);
            } else {
                game.permit_cast_this_turn(GameObjectId(20_000), PlayerId::One);
            }
            game.players[0].graveyard.extend([
                card(30_000, cards::LIGHTNING_BOLT, PlayerId::One),
                card(30_001, cards::DIVINATION, PlayerId::One),
            ]);
            game.battlefield
                .push(creature(40_000, cards::HELM_OF_AWAKENING, PlayerId::One));
            game.players[0].mana_pool.blue = 2;
            game.players[0].mana_pool.colorless = 1;
            assert!(cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT).is_none());
            game.players[0].mana_pool.colorless = 2;
            let action = cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT)
                .expect("the two graveyard spells and Helm reduce the cost to {2}{U}{U}");
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].mana_pool, ManaPool::default());
            pass_priority_pair(&mut game);
            assert!(
                game.battlefield
                    .iter()
                    .any(|p| p.card.definition == cards::EDDYMURK_CRAB)
            );
        }
    }
}

#[test]
fn a_graveyard_creature_discount_does_not_count_the_announced_spell() {
    let mut game = ready_game();
    resolve_yawgmoths_will(&mut game);
    game.players[0].graveyard.extend([
        card(20_000, cards::GHOULTREE, PlayerId::One),
        card(30_000, cards::GRIZZLY_BEARS, PlayerId::One),
        card(30_001, cards::SAVANNAH_LIONS, PlayerId::One),
        card(30_002, cards::SEDGE_TROLL, PlayerId::One),
    ]);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;
    assert!(
        cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT).is_none(),
        "Ghoultree has left the graveyard by the time its cost is determined"
    );
    game.players[0].mana_pool.colorless = 4;
    let action = cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn a_self_modifier_works_from_exile_and_does_not_discount_another_copy() {
    let mut game = ready_game();
    game.players[0]
        .exile
        .push(card(20_000, cards::CAVERN_HOARD_DRAGON_114, PlayerId::One));
    game.permit_cast_this_turn(GameObjectId(20_000), PlayerId::One);
    game.battlefield.extend([
        creature(30_000, cards::SOL_RING, PlayerId::Two),
        creature(30_001, cards::BLACK_VISE, PlayerId::Two),
        creature(30_002, cards::CAVERN_HOARD_DRAGON_114, PlayerId::One),
    ]);
    game.players[0].mana_pool.red = 2;
    game.players[0].mana_pool.colorless = 4;
    assert!(cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT).is_none());
    game.players[0].mana_pool.colorless = 5;
    let action = cast(&game, GameObjectId(20_000), PlayOptionId::DEFAULT)
        .expect("the announced Dragon's own ability removes two generic mana");
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn only_the_announced_parts_supply_self_discounts_and_taxes() {
    static HALVES: [(&str, CardRules); 2] = [
        (
            "Discount",
            CardRules::new_sorcery(ManaCost::new(3, 0)).with_abilities(&[
                abilities::this_spell_cost_reduction(
                    "This spell costs {2} less to cast.",
                    ValueDef::Constant(2),
                ),
            ]),
        ),
        (
            "Tax",
            CardRules::new_sorcery(ManaCost::new(1, 0)).with_abilities(&[
                abilities::this_spell_cost_adjustment(
                    "This spell costs {2} more to cast.",
                    CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::Constant(2))),
                ),
            ]),
        ),
    ];
    let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000003010");
    let mut definition =
        CardDefinition::new(id, "Discount // Tax", sets::bloomburrow::SET, HALVES[0].1);
    let composition = CardComposition::split(&HALVES, Some(ManaCost::new(4, 0)));
    definition.parts = composition.parts;
    definition.structure = composition.structure;
    definition.play_options = composition.play_options;
    for (option, total) in [
        (PlayOptionId(0), 1),
        (PlayOptionId(1), 3),
        (PlayOptionId(2), 4),
    ] {
        let mut game = ready_game();
        game.catalog = CardCatalog::new([definition.clone()]).unwrap();
        game.prepared_engine = PreparedEngine::compile(&game.catalog);
        game.players[0].hand.push(card(20_000, id, PlayerId::One));
        game.players[0].mana_pool.colorless = total - 1;
        assert!(cast(&game, GameObjectId(20_000), option).is_none());
        game.players[0].mana_pool.colorless = total;
        let action = cast(&game, GameObjectId(20_000), option)
            .expect("only announced halves modify this cast");
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}
