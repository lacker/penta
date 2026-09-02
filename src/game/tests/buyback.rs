//! Buyback as an optional additional cost, including its interaction with
//! object payments, counters, and a separately granted flashback cost.

use super::*;
use crate::card::{BasicLandType, CostQuantityDef, SpellAdditionalCostDef};

fn constant_mists_casts(game: &Game, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

fn sprout_swarm_casts(game: &Game, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

fn is_bought_back(action: &Action) -> bool {
    matches!(
        action,
        Action::CastSpell { choices, .. } if !choices.costs().additional().is_empty()
    )
}

fn give_constant_mists_mana(game: &mut Game) {
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
}

#[test]
fn ordinary_constant_mists_needs_no_land_and_goes_to_graveyard() {
    let mut game = ready_game();
    let mists = card(95_000, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].hand.push(mists);
    give_constant_mists_mana(&mut game);

    let actions = constant_mists_casts(&game, mists_id);
    assert_eq!(
        actions.len(),
        1,
        "the ordinary cast needs no land to sacrifice"
    );
    assert!(
        !is_bought_back(&actions[0]),
        "no controlled land means no Buyback action",
    );
    let Action::CastSpell { sacrifices, .. } = &actions[0] else {
        unreachable!("the helper returns only spell casts")
    };
    assert!(
        sacrifices.is_empty(),
        "the ordinary cast pays no object cost"
    );

    game.apply(PlayerId::One, actions.into_iter().next().unwrap())
        .expect("the ordinary cast is legal");
    pass_priority_pair(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CONSTANT_MISTS),
        "an ordinary Constant Mists goes to its owner's graveyard",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .all(|card| card.definition != cards::CONSTANT_MISTS),
    );
}

#[test]
fn sacrifice_for_mana_land_cannot_also_pay_constant_mists_buyback() {
    let mut game = ready_game();
    let mists = card(95_050, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].hand.push(mists);
    let battleground = creature(95_051, cards::HAVENWOOD_BATTLEGROUND, PlayerId::One);
    game.battlefield.push(battleground);

    let actions = constant_mists_casts(&game, mists_id);
    assert!(
        actions.iter().any(|action| !is_bought_back(action)),
        "sacrificing Havenwood Battleground for {{G}}{{G}} pays the ordinary {{1}}{{G}} cost",
    );
    assert!(
        actions.iter().all(|action| !is_bought_back(action)),
        "the Battleground cannot also be sacrificed a second time to pay Buyback",
    );
}

#[test]
fn buyback_sacrifices_exactly_one_controlled_land_and_returns_mists() {
    let mut game = ready_game();
    let mists = card(95_100, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].hand.push(mists);
    give_constant_mists_mana(&mut game);

    let first_land = creature(95_101, cards::FOREST, PlayerId::One);
    let first_land_id = first_land.card.id;
    let second_land = creature(95_102, cards::ISLAND, PlayerId::One);
    let second_land_id = second_land.card.id;
    let opposing_land = creature(95_103, cards::MOUNTAIN, PlayerId::Two);
    let opposing_land_id = opposing_land.card.id;
    game.battlefield
        .extend([first_land, second_land, opposing_land]);

    let bought_back = constant_mists_casts(&game, mists_id)
        .into_iter()
        .filter(is_bought_back)
        .collect::<Vec<_>>();
    assert_eq!(
        bought_back.len(),
        2,
        "each controlled land is one distinct way to pay Buyback",
    );
    for action in &bought_back {
        let Action::CastSpell { sacrifices, .. } = action else {
            unreachable!("the helper returns only spell casts")
        };
        assert_eq!(sacrifices.len(), 1, "Buyback sacrifices exactly one land");
        assert!(
            matches!(sacrifices.as_slice(), [id] if *id == first_land_id || *id == second_land_id),
            "only a land controlled by the caster may pay Buyback",
        );
    }

    let action = bought_back
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices == &[first_land_id])
        })
        .expect("the first controlled land can pay Buyback");
    game.apply(PlayerId::One, action)
        .expect("the bought-back cast is legal");

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_land_id),
        "the selected land is sacrificed while the spell is cast",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_land_id),
        "an unselected controlled land remains",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == opposing_land_id),
        "the opponent's land remains",
    );
    pass_priority_pair(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::CONSTANT_MISTS),
        "successful Buyback returns Constant Mists to hand",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FOREST),
        "the land remains paid even though the spell returned",
    );
}

#[test]
fn countered_bought_back_constant_mists_does_not_return() {
    let mut game = ready_game();
    let mists = card(95_200, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].hand.push(mists);
    give_constant_mists_mana(&mut game);
    let land = creature(95_201, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let counterspell = card(95_202, cards::COUNTERSPELL, PlayerId::Two);
    let counterspell_id = counterspell.id;
    game.players[PlayerId::Two.index()].hand.push(counterspell);
    game.players[PlayerId::Two.index()].mana_pool.blue = 2;

    let buyback = constant_mists_casts(&game, mists_id)
        .into_iter()
        .find(|action| {
            is_bought_back(action)
                && matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices == &[land_id])
        })
        .expect("the land can pay Buyback");
    game.apply(PlayerId::One, buyback)
        .expect("the bought-back cast is legal");
    acceptance_attempt_counterspell(&mut game, counterspell_id);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .all(|card| card.definition != cards::CONSTANT_MISTS),
        "Buyback returns only a spell that resolves",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CONSTANT_MISTS),
        "the countered spell goes to its owner's graveyard",
    );
}

#[test]
fn granted_card_cost_flashback_combines_with_buyback_and_exiles_mists() {
    let mut game = ready_game();
    let mists = card(95_300, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].graveyard.push(mists);
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object: mists_id,
            ability: CARD_COST_FLASHBACK,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        });
    give_constant_mists_mana(&mut game);
    let land = creature(95_301, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let actions = constant_mists_casts(&game, mists_id);
    assert_eq!(
        actions.len(),
        2,
        "granted Flashback offers casts with and without Buyback",
    );
    let buyback_flashback = actions
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    choices,
                    sacrifices,
                    ..
                } if choices.costs().alternative().is_some()
                    && !choices.costs().additional().is_empty()
                    && sacrifices == &[land_id]
            )
        })
        .expect("Flashback and Buyback can be selected together");
    game.apply(PlayerId::One, buyback_flashback)
        .expect("the combined cast is legal");

    let signature = game
        .stack
        .last()
        .and_then(|object| object.signature.as_ref())
        .expect("the spell is on the stack with its cast signature");
    assert!(signature.costs().alternative().is_some());
    assert!(!signature.costs().additional().is_empty());
    assert!(
        game.stack
            .last()
            .unwrap()
            .cast
            .as_ref()
            .is_some_and(|cast| cast.via_flashback)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != land_id),
        "the Buyback land is paid before resolution",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FOREST),
    );

    pass_priority_pair(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::CONSTANT_MISTS),
        "Flashback exiles the resolving card even though Buyback was paid",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .all(|card| card.definition != cards::CONSTANT_MISTS),
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::CONSTANT_MISTS),
    );
}

#[test]
fn granted_card_cost_flashback_adds_sprout_swarm_mana_buyback_and_exiles() {
    let mut game = ready_game();
    let sprout = card(95_400, cards::SPROUT_SWARM, PlayerId::One);
    let sprout_id = sprout.id;
    game.players[PlayerId::One.index()].graveyard.push(sprout);
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object: sprout_id,
            ability: CARD_COST_FLASHBACK,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        });
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let short_on_buyback = sprout_swarm_casts(&game, sprout_id);
    assert!(
        short_on_buyback.iter().any(|action| matches!(
            action,
            Action::CastSpell { choices, .. }
                if choices.costs().alternative().is_some() && !is_bought_back(action)
        )),
        "{{1}}{{G}} is enough for the granted card-cost Flashback cast",
    );
    assert!(
        short_on_buyback
            .iter()
            .all(|action| !is_bought_back(action)),
        "{{3}}{{G}} is one generic mana short of Sprout Swarm plus Buyback",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    let buyback_flashback = sprout_swarm_casts(&game, sprout_id)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { choices, .. }
                    if choices.costs().alternative().is_some() && is_bought_back(action)
            )
        })
        .expect("{4}{G} pays the printed card cost and the {3} Buyback together");
    game.apply(PlayerId::One, buyback_flashback)
        .expect("the combined Flashback and Buyback cast is legal");
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.total(),
        0,
        "the combined cast spends all five mana",
    );
    assert!(
        game.stack
            .last()
            .unwrap()
            .cast
            .as_ref()
            .is_some_and(|cast| cast.via_flashback)
    );

    pass_priority_pair(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::SPROUT_SWARM),
        "Flashback exiles Sprout Swarm even when its mana Buyback was paid",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .all(|card| card.definition != cards::SPROUT_SWARM),
    );
}

#[test]
fn free_exile_cast_replaces_sprout_swarm_base_cost_but_not_mana_buyback() {
    let mut game = ready_game();
    let sprout = card(95_500, cards::SPROUT_SWARM, PlayerId::One);
    let sprout_id = sprout.id;
    game.players[PlayerId::One.index()].exile.push(sprout);
    game.permit_free_play_this_turn(sprout_id, PlayerId::One);

    let free_casts = sprout_swarm_casts(&game, sprout_id);
    assert!(
        free_casts.iter().any(|action| !is_bought_back(action)),
        "the exile permission waives Sprout Swarm's printed {{1}}{{G}}",
    );
    assert!(
        free_casts.iter().all(|action| !is_bought_back(action)),
        "Buyback is not free with the spell's base mana cost",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    assert!(
        sprout_swarm_casts(&game, sprout_id)
            .iter()
            .all(|action| !is_bought_back(action)),
        "two mana cannot pay Buyback {{3}}",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    let bought_back = sprout_swarm_casts(&game, sprout_id)
        .into_iter()
        .find(is_bought_back)
        .expect("three mana pays Buyback on the otherwise-free cast");
    game.apply(PlayerId::One, bought_back)
        .expect("the free exile cast with paid Buyback is legal");
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.total(),
        0,
        "only the {{3}} Buyback is paid",
    );
    pass_priority_pair(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SPROUT_SWARM),
        "the non-Flashback exile cast returns to hand when Buyback was paid",
    );
}

#[test]
fn checkpoint_round_trip_preserves_bought_back_spell_hand_destination() {
    let mut game = ready_game();
    for player in &mut game.players {
        player.library.clear();
    }
    let mists = card(95_600, cards::CONSTANT_MISTS, PlayerId::One);
    let mists_id = mists.id;
    game.players[PlayerId::One.index()].hand.push(mists);
    give_constant_mists_mana(&mut game);
    let land = creature(95_601, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);

    let bought_back = constant_mists_casts(&game, mists_id)
        .into_iter()
        .find(|action| {
            is_bought_back(action)
                && matches!(action, Action::CastSpell { sacrifices, .. } if sacrifices == &[land_id])
        })
        .expect("the land pays Buyback");
    game.apply(PlayerId::One, bought_back)
        .expect("Constant Mists is cast with Buyback");

    let viewer = game.priority;
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &actions,
    );
    let hidden = serde_json::json!({
        "hands": {"p1": [], "p2": []},
        "libraries": {"p1": [], "p2": []},
        "outsideGame": {"p1": [], "p2": []},
    });
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        95_602,
    )
    .expect("the bought-back stack object reconstructs");
    assert!(
        rebuilt.stack.last().is_some_and(|object| {
            object
                .signature
                .as_ref()
                .is_some_and(|signature| !signature.costs().additional().is_empty())
        }),
        "the restored cast signature retains the selected Buyback cost",
    );

    pass_priority_pair(&mut rebuilt);

    assert!(
        rebuilt.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::CONSTANT_MISTS),
        "the restored bought-back spell resolves to its owner's hand",
    );
}

#[test]
fn modal_spell_freezes_bought_back_hand_destination() {
    static MODES: [AbilityDef; 2] = [
        AbilityDef::spell(
            "You gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::spell(
            "You gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ];
    static ABILITIES: [AbilityDef; 2] = [
        abilities::buyback(mana_cost!("{1}")),
        AbilityDef::modal_spell("Choose one.", &MODES),
    ];

    let definition_id = CardDefinitionId::new(50_100);
    let mut definition = CardDefinition::new(
        definition_id,
        "Modal Buyback Test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the modal Buyback fixture is valid");
    let spell = card(95_400, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let starting_life = game.players[PlayerId::One.index()].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell_id
                        && !choices.costs().additional().is_empty()
                        && choices.modes() == [ModeId(1)]
            )
        })
        .expect("the second modal branch can be cast with Buyback");
    game.apply(PlayerId::One, action)
        .expect("the bought-back modal spell is legal");

    assert_eq!(
        game.stack
            .last()
            .and_then(|object| object.ability.as_ref())
            .and_then(|ability| ability.resolution_destination),
        Some(SpellResolutionDestinationDef::Hand),
        "the selected modal payload freezes Buyback's hand destination",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        starting_life + 2,
        "the selected modal effect still resolves",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == definition_id),
        "the resolved modal spell returns to hand",
    );
}

#[test]
fn effectful_overload_freezes_bought_back_hand_destination() {
    static ABILITIES: [AbilityDef; 3] = [
        abilities::buyback(mana_cost!("{1}")),
        AbilityDef::spell(
            "You gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::overload(
            mana_cost!("{1}"),
            "You gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ];

    let definition_id = CardDefinitionId::new(50_101);
    let mut definition = CardDefinition::new(
        definition_id,
        "Overload Buyback Test",
        CardSet::ReturnToRavnica,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the overload Buyback fixture is valid");
    let spell = card(95_500, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    let starting_life = game.players[PlayerId::One.index()].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell_id
                        && choices.costs().alternative().is_some()
                        && !choices.costs().additional().is_empty()
            )
        })
        .expect("the effectful overload can be cast with Buyback");
    game.apply(PlayerId::One, action)
        .expect("the bought-back overload spell is legal");

    assert_eq!(
        game.stack
            .last()
            .and_then(|object| object.ability.as_ref())
            .and_then(|ability| ability.resolution_destination),
        Some(SpellResolutionDestinationDef::Hand),
        "the effectful alternative payload freezes Buyback's hand destination",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        starting_life + 2,
        "the overload effect replaces and resolves instead of the base effect",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == definition_id),
        "the resolved effectful overload returns to hand",
    );
}

#[test]
fn mandatory_return_cost_and_buyback_sacrifice_keep_distinct_actions() {
    static RETURN_AN_ISLAND: SpellAdditionalCostDef = SpellAdditionalCostDef::return_to_hand(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
        CostQuantityDef::Fixed(1),
    );
    static SACRIFICE_A_FOREST: SpellAdditionalCostDef = SpellAdditionalCostDef::sacrifice(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        CostQuantityDef::Fixed(1),
    );
    static ABILITIES: [AbilityDef; 2] = [
        abilities::buyback_with_additional_cost("Buyback—Sacrifice a Forest.", &SACRIFICE_A_FOREST),
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, return an Island you control to its owner's hand. You gain 1 life.",
            &[],
            RETURN_AN_ISLAND,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];

    let definition_id = CardDefinitionId::new(50_102);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mixed Spend Buyback Test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the mixed-spend Buyback fixture is valid");

    let spell = card(95_700, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let island = creature(95_701, cards::ISLAND, PlayerId::One);
    let island_id = island.card.id;
    let forest = creature(95_702, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.extend([island, forest]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } if *card == spell_id
                    && !choices.costs().additional().is_empty()
                    && sacrifices == &[island_id, forest_id]
            )
        })
        .expect("the mandatory return and optional Buyback costs compose");
    game.apply(PlayerId::One, action)
        .expect("each selected object can pay its own additional cost");

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::ISLAND),
        "the mandatory ReturnToHand cost returns its selected Island",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FOREST),
        "the optional Buyback cost sacrifices its selected Forest",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| { permanent.card.id != island_id && permanent.card.id != forest_id }),
        "both selected permanents leave the battlefield while the spell is cast",
    );

    pass_priority_pair(&mut game);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == definition_id),
        "the separately sacrificed Forest still pays Buyback's hand outcome",
    );
}
