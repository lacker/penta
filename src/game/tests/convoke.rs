//! Convoke payment planning and execution, exercised through Sprout Swarm.

use super::*;

static TRUE_COLORLESS_CONVOKE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::spell(
        "Draw a card.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    abilities::convoke(),
];

static SACRIFICE_CREATURE_FOR_TWO_GREEN: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "Sacrifice a creature: Add {G}{G}.",
    &[AbilityCostDef::SacrificePermanent {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        controller: PlayerRelation::You,
    }],
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
)];

static TAP_FOR_COLORLESS: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "{T}: Add {C}.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
)];

fn game_with_sprout(permanents: Vec<Permanent>) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield = permanents;
    let sprout = card(90_000, cards::SPROUT_SWARM, PlayerId::One);
    let sprout_id = sprout.id;
    game.players[PlayerId::One.index()].hand.push(sprout);
    (game, sprout_id)
}

fn sprout_cast(game: &Game, spell: GameObjectId, bought_back: bool) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell
                        && choices.costs().additional().is_empty() != bought_back
            )
        })
}

fn is_tapped(game: &Game, object: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .expect("the payment creature remains on the battlefield")
        .tapped
}

fn saproling_count(game: &Game) -> usize {
    let saproling = tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1);
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, saproling))
        .count()
}

fn assert_preview_contains_each_once(game: &Game, action: &Action, expected: &[GameObjectId]) {
    let sources = game.mana_sources_for_action(PlayerId::One, action);
    assert_eq!(
        sources.len(),
        expected.len(),
        "the preview names each physical payment source once",
    );
    for object in expected {
        assert_eq!(
            sources.iter().filter(|source| *source == object).count(),
            1,
            "each payment source appears exactly once",
        );
    }
}

fn two_pair_hybrid_cost_with_generic(generic: u16) -> ManaCost {
    let mut cost = ManaCost {
        generic,
        ..ManaCost::default()
    };
    cost.hybrid[HybridPair::WhiteBlue.index()] = 1;
    cost.hybrid[HybridPair::WhiteBlack.index()] = 1;
    cost
}

fn assert_tapped_state(game: &Game, sources: &[GameObjectId], expected: bool) {
    for source in sources {
        assert_eq!(
            is_tapped(game, *source),
            expected,
            "payment sources have the expected tapped state",
        );
    }
}

fn decision_option_is_card(
    option: &DecisionOption,
    object: GameObjectId,
    definition: CardDefinitionId,
) -> bool {
    matches!(
        option.card,
        Some((
            candidate,
            ObjectCharacteristics::Card {
                definition: candidate_definition,
                part: CardPartId::PRIMARY,
            },
        )) if candidate == object && candidate_definition == definition
    )
}

fn game_with_two_pair_hybrid_convoke(
    first: CardDefinitionId,
    second: CardDefinitionId,
) -> (Game, GameObjectId, [GameObjectId; 2]) {
    game_with_two_pair_hybrid_convoke_and_generic(first, second, 0)
}

fn game_with_two_pair_hybrid_convoke_and_generic(
    first: CardDefinitionId,
    second: CardDefinitionId,
    generic: u16,
) -> (Game, GameObjectId, [GameObjectId; 2]) {
    let definition_id = CardDefinitionId::new(50_001);
    let mut definition = CardDefinition::new(
        definition_id,
        "Two-pair hybrid convoke test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(two_pair_hybrid_cost_with_generic(generic))
        .with_abilities(&TRUE_COLORLESS_CONVOKE_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the hybrid test definition is valid");
    let first = creature(79_900, first, PlayerId::One);
    let second = creature(79_901, second, PlayerId::One);
    let source_ids = [first.card.id, second.card.id];
    game.battlefield.extend([first, second]);
    let spell = card(89_900, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    (game, spell_id, source_ids)
}

#[test]
fn one_green_creature_cannot_pay_both_sprout_swarm_symbols() {
    let green = creature(80_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let (game, sprout) = game_with_sprout(vec![green]);

    assert!(
        sprout_cast(&game, sprout, false).is_none(),
        "one creature contributes only one unit, not both {{1}} and {{G}}",
    );
}

#[test]
fn shared_hybrid_color_cannot_be_counted_for_two_convoke_symbols() {
    let (game, spell, _) =
        game_with_two_pair_hybrid_convoke(cards::SAVANNAH_LIONS, cards::GRIZZLY_BEARS);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == spell)),
        "white plus green cannot pay {{W/U}}{{W/B}} by claiming white twice",
    );
}

#[test]
fn hybrid_convoke_uses_one_global_color_assignment_when_applied() {
    let (mut game, spell, source_ids) =
        game_with_two_pair_hybrid_convoke(cards::SAVANNAH_LIONS, cards::ISLAND);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the white convoker pays {W/B} while Island mana pays {W/U}");
    assert_preview_contains_each_once(&game, &action, &source_ids);

    game.apply(PlayerId::One, action)
        .expect("the advertised global hybrid assignment applies without a payment panic");
    assert!(
        source_ids
            .into_iter()
            .all(|source| is_tapped(&game, source)),
        "the creature convokes and the Island pays the residual hybrid symbol",
    );
    assert_eq!(game.stack.len(), 1, "the paid-for spell is on the stack");
}

#[test]
fn activated_mana_is_not_counted_twice_in_the_convoke_residual() {
    let (mut game, spell, source_ids) = game_with_two_pair_hybrid_convoke_and_generic(
        cards::MERFOLK_OF_THE_PEARL_TRIDENT,
        cards::PLAINS,
        1,
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("blue convoke, Plains mana, and floating green pay {1}{W/U}{W/B}");
    assert_preview_contains_each_once(&game, &action, &source_ids);

    game.apply(PlayerId::One, action)
        .expect("the advertised mixed convoke and mana payment applies");
    assert!(
        source_ids
            .into_iter()
            .all(|source| is_tapped(&game, source)),
        "the creature convokes and the Plains is activated exactly once",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "the one Plains mana and one floating mana are both spent",
    );
    assert!(
        game.players[PlayerId::One.index()].mana.is_empty(),
        "no attributed mana remains after the exact payment",
    );
    assert_eq!(game.stack.len(), 1, "the paid-for spell is on the stack");
}

#[test]
fn green_and_non_green_creatures_fully_convoke_sprout_swarm() {
    for (generic_creature, label) in [
        (cards::MERFOLK_OF_THE_PEARL_TRIDENT, "off-color"),
        (cards::ORNITHOPTER, "colorless"),
    ] {
        let green = creature(80_100, cards::GRIZZLY_BEARS, PlayerId::One);
        let green_id = green.card.id;
        let generic = creature(80_101, generic_creature, PlayerId::One);
        let generic_id = generic.card.id;
        let (mut game, sprout) = game_with_sprout(vec![green, generic]);

        let action = sprout_cast(&game, sprout, false)
            .unwrap_or_else(|| panic!("a green and {label} creature can pay the spell"));
        assert_preview_contains_each_once(&game, &action, &[green_id, generic_id]);

        game.apply(PlayerId::One, action)
            .expect("the all-convoke cast is legal");
        assert!(is_tapped(&game, green_id), "the green creature convokes");
        assert!(
            is_tapped(&game, generic_id),
            "the {label} creature pays the generic symbol",
        );
        assert_eq!(
            game.players[PlayerId::One.index()].mana_pool,
            ManaPool::default(),
            "convoke does not put synthetic mana in the aggregate pool",
        );
        assert!(
            game.players[PlayerId::One.index()].mana.is_empty(),
            "convoke does not leave attributed mana behind",
        );

        pass_priority_pair(&mut game);
        assert_eq!(
            saproling_count(&game),
            1,
            "the fully convoked Sprout Swarm resolves normally",
        );
    }
}

#[test]
fn wrong_color_creatures_cannot_pay_sprout_swarms_green_symbol() {
    let first = creature(80_200, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let second = creature(80_201, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let (game, sprout) = game_with_sprout(vec![first, second]);

    assert!(
        sprout_cast(&game, sprout, false).is_none(),
        "two blue creatures have enough units but neither can pay {{G}}",
    );
}

#[test]
fn tapped_and_opposing_creatures_are_not_convoke_sources() {
    let mut tapped_green = creature(80_300, cards::GRIZZLY_BEARS, PlayerId::One);
    tapped_green.tapped = true;
    let own_blue = creature(80_301, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let (game, sprout) = game_with_sprout(vec![tapped_green, own_blue]);
    assert!(
        sprout_cast(&game, sprout, false).is_none(),
        "a tapped green creature cannot supply the missing {{G}}",
    );

    let opposing_green = creature(80_302, cards::GRIZZLY_BEARS, PlayerId::Two);
    let own_blue = creature(80_303, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let (game, sprout) = game_with_sprout(vec![opposing_green, own_blue]);
    assert!(
        sprout_cast(&game, sprout, false).is_none(),
        "an opponent's green creature cannot supply the missing {{G}}",
    );
}

#[test]
fn summoning_sick_creatures_can_convoke() {
    let green = creature(80_400, cards::GRIZZLY_BEARS, PlayerId::One);
    let green_id = green.card.id;
    let blue = creature(80_401, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let blue_id = blue.card.id;
    let (mut game, sprout) = game_with_sprout(vec![green, blue]);
    game.turn = 7;
    game.turns_started[PlayerId::One.index()] = 7;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 7;
    }

    let action = sprout_cast(&game, sprout, false)
        .expect("summoning sickness does not prohibit a creature from convoking");
    game.apply(PlayerId::One, action)
        .expect("both summoning-sick creatures may be tapped to convoke");

    assert!(is_tapped(&game, green_id));
    assert!(is_tapped(&game, blue_id));
}

#[test]
fn llanowar_elves_cannot_make_mana_and_convoke_for_the_same_spell() {
    let elf = creature(80_500, cards::LLANOWAR_ELVES, PlayerId::One);
    let (game, sprout) = game_with_sprout(vec![elf]);
    assert!(
        sprout_cast(&game, sprout, false).is_none(),
        "one Llanowar Elves cannot pay twice by making {{G}} and convoking",
    );

    let elf = creature(80_501, cards::LLANOWAR_ELVES, PlayerId::One);
    let elf_id = elf.card.id;
    let helper = creature(80_502, cards::ORNITHOPTER, PlayerId::One);
    let helper_id = helper.card.id;
    let (mut game, sprout) = game_with_sprout(vec![elf, helper]);
    let action = sprout_cast(&game, sprout, false)
        .expect("Llanowar Elves and one other creature provide two distinct payments");
    assert_preview_contains_each_once(&game, &action, &[elf_id, helper_id]);

    game.apply(PlayerId::One, action)
        .expect("the two-source payment is legal");
    assert!(is_tapped(&game, elf_id));
    assert!(is_tapped(&game, helper_id));
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "mana made by Llanowar Elves is spent rather than left behind",
    );
    assert!(game.players[PlayerId::One.index()].mana.is_empty());
}

#[test]
fn sprout_swarm_buyback_requires_five_convoke_creatures() {
    for creature_count in [4_u32, 5] {
        let mut creatures = Vec::new();
        let mut creature_ids = Vec::new();
        for offset in 0..creature_count {
            let creature = creature(80_600 + offset, cards::GRIZZLY_BEARS, PlayerId::One);
            creature_ids.push(creature.card.id);
            creatures.push(creature);
        }
        let (mut game, sprout) = game_with_sprout(creatures);
        assert!(
            sprout_cast(&game, sprout, false).is_some(),
            "four or more creatures can always pay the ordinary cost",
        );

        let bought_back = sprout_cast(&game, sprout, true);
        if creature_count == 4 {
            assert!(
                bought_back.is_none(),
                "four creatures cannot pay the bought-back {{4}}{{G}} total",
            );
            continue;
        }

        let action = bought_back.expect("five creatures can pay the bought-back {4}{G} total");
        let (cost, x, _, _) = game
            .mana_requirement(PlayerId::One, &action)
            .expect("the selected cast has a mana requirement");
        assert_eq!(cost, mana_cost!("{4}{G}"));
        assert_eq!(x, 0);
        assert_preview_contains_each_once(&game, &action, &creature_ids);

        game.apply(PlayerId::One, action)
            .expect("all five creatures may convoke the bought-back spell");
        assert!(
            creature_ids.iter().all(|object| is_tapped(&game, *object)),
            "each creature is tapped exactly once for the five-unit cost",
        );
        pass_priority_pair(&mut game);

        assert!(
            game.players[PlayerId::One.index()]
                .hand
                .iter()
                .any(|card| card.definition == cards::SPROUT_SWARM),
            "the fully convoked bought-back spell returns to hand",
        );
        assert_eq!(saproling_count(&game), 1);
    }
}

#[test]
fn mana_confluence_life_costs_are_aggregated_for_a_convoke_payment() {
    let first_confluence = creature(80_650, cards::MANA_CONFLUENCE, PlayerId::One);
    let first_confluence_id = first_confluence.card.id;
    let second_confluence = creature(80_651, cards::MANA_CONFLUENCE, PlayerId::One);
    let second_confluence_id = second_confluence.card.id;
    let first_creature = creature(80_652, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_creature_id = first_creature.card.id;
    let second_creature = creature(80_653, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_creature_id = second_creature.card.id;
    let third_creature = creature(80_654, cards::GRIZZLY_BEARS, PlayerId::One);
    let third_creature_id = third_creature.card.id;
    let payment_sources = [
        first_confluence_id,
        second_confluence_id,
        first_creature_id,
        second_creature_id,
        third_creature_id,
    ];
    let (mut game, sprout) = game_with_sprout(vec![
        first_confluence,
        second_confluence,
        first_creature,
        second_creature,
        third_creature,
    ]);

    game.players[PlayerId::One.index()].life = 2;
    let bought_back = sprout_cast(&game, sprout, true)
        .expect("two life can pay both Mana Confluence activation costs");
    assert_preview_contains_each_once(&game, &bought_back, &payment_sources);

    game.players[PlayerId::One.index()].life = 1;
    assert!(
        sprout_cast(&game, sprout, false).is_some(),
        "the three creatures still make ordinary Sprout Swarm castable",
    );
    assert!(
        sprout_cast(&game, sprout, true).is_none(),
        "one life cannot be committed to two Mana Confluence activations",
    );
    assert!(
        !game.is_legal_action(PlayerId::One, &bought_back),
        "a bought-back action observed with two life becomes illegal at one life",
    );
    assert!(
        game.apply(PlayerId::One, bought_back).is_err(),
        "authoritative application rejects the aggregate life overcommit",
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    assert!(
        payment_sources
            .iter()
            .all(|source| !is_tapped(&game, *source))
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.id == sprout),
        "rejecting the stale action does not move the spell",
    );
}

#[test]
fn a_colorless_creature_cannot_convoke_a_true_colorless_symbol() {
    let definition_id = CardDefinitionId::new(50_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "True colorless convoke test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    definition.rules =
        CardRules::new_instant(mana_cost!("{C}")).with_abilities(&TRUE_COLORLESS_CONVOKE_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the test definition is valid");
    let ornithopter = creature(80_700, cards::ORNITHOPTER, PlayerId::One);
    game.battlefield.push(ornithopter);
    let spell = card(90_700, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    let is_castable = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
    };
    assert!(
        !is_castable(&game),
        "colorless convoke pays generic mana, not a true {{C}} symbol",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(
        is_castable(&game),
        "actual colorless mana pays {{C}}, proving the test spell is otherwise castable",
    );
}

#[test]
fn prospector_collision_falls_back_to_an_all_convoke_payment() {
    // With the Fanatic encountered first, a greedy payment can try to convoke
    // it and then sacrifice it to the Prospector. That conflicting preview
    // must not make an otherwise legal cast disappear or panic when applied.
    let fanatic = creature(81_000, cards::MOGG_FANATIC, PlayerId::One);
    let fanatic_id = fanatic.card.id;
    let prospector = creature(81_001, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    let first_green = creature(81_002, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_green_id = first_green.card.id;
    let second_green = creature(81_003, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_green_id = second_green.card.id;
    let third_green = creature(81_004, cards::GRIZZLY_BEARS, PlayerId::One);
    let third_green_id = third_green.card.id;
    let expected = [
        fanatic_id,
        prospector_id,
        first_green_id,
        second_green_id,
        third_green_id,
    ];
    let (mut game, sprout) = game_with_sprout(vec![
        fanatic,
        prospector,
        first_green,
        second_green,
        third_green,
    ]);

    let action = sprout_cast(&game, sprout, true)
        .expect("the conflicting Prospector route does not hide an all-convoke payment");
    assert_preview_contains_each_once(&game, &action, &expected);

    game.apply(PlayerId::One, action)
        .expect("the advertised all-convoke payment applies without a planning panic");
    assert!(
        expected.iter().all(|object| is_tapped(&game, *object)),
        "all five creatures convoke instead of sacrificing the Fanatic",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == fanatic_id),
        "the rejected Prospector route did not sacrifice the Fanatic",
    );
}

#[test]
fn a_mana_source_activates_before_another_planned_activation_sacrifices_it() {
    let definition_id = CardDefinitionId::new(50_001);
    let mut definition = CardDefinition::new(
        definition_id,
        "Sacrifice mana source collision test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(ManaCost::default())
        .with_abilities(&SACRIFICE_CREATURE_FOR_TWO_GREEN);
    synchronize_single_part_definition(&mut definition);

    let mana_creature_definition_id = CardDefinitionId::new(50_002);
    let mut mana_creature_definition = CardDefinition::new(
        mana_creature_definition_id,
        "Consumed mana creature test",
        CardSet::FutureSight,
        crate::card::CardRules::unsupported(),
    );
    mana_creature_definition.rules =
        CardRules::new_creature(ManaCost::default(), &["Construct"], 1, 1)
            .with_abilities(&TAP_FOR_COLORLESS);
    synchronize_single_part_definition(&mut mana_creature_definition);

    for sacrifice_source_first in [true, false] {
        let source = creature(81_050, definition_id, PlayerId::One);
        let source_id = source.card.id;
        let mana_creature = creature(81_051, mana_creature_definition_id, PlayerId::One);
        let mana_creature_id = mana_creature.card.id;
        let permanents = if sacrifice_source_first {
            vec![source, mana_creature]
        } else {
            vec![mana_creature, source]
        };
        let (mut game, sprout) = game_with_sprout(permanents);
        let mut definitions = game
            .catalog
            .definitions()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        definitions.push(definition.clone());
        definitions.push(mana_creature_definition.clone());
        game.catalog = CardCatalog::new(definitions).expect("the test definition is valid");
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

        assert!(
            sprout_cast(&game, sprout, false).is_some(),
            "the floating mana and either synthetic mana ability can pay ordinary Sprout Swarm",
        );
        let bought_back = sprout_cast(&game, sprout, true)
            .expect("the creature can make mana before the later activation sacrifices it");
        assert_preview_contains_each_once(&game, &bought_back, &[mana_creature_id, source_id]);

        game.apply(PlayerId::One, bought_back)
            .expect("the dependency-ordered mana plan applies in either battlefield order");
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == mana_creature_id),
            "the creature was tapped for mana and then sacrificed",
        );
        assert!(
            game.players[PlayerId::One.index()]
                .graveyard
                .iter()
                .any(|card| card.definition == mana_creature_definition_id),
        );
        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.id == source_id),
            "the sacrificing artifact remains on the battlefield",
        );
        pass_priority_pair(&mut game);
        assert!(
            game.players[PlayerId::One.index()]
                .hand
                .iter()
                .any(|card| card.definition == cards::SPROUT_SWARM),
            "the ordered payment preserves the bought-back resolution outcome",
        );
    }
}

#[test]
fn prospector_can_make_mana_and_then_convoke_for_the_same_spell() {
    let mut tapped_fanatic = creature(81_100, cards::MOGG_FANATIC, PlayerId::One);
    let fanatic_id = tapped_fanatic.card.id;
    tapped_fanatic.tapped = true;
    let prospector = creature(81_101, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    let first_green = creature(81_102, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_green_id = first_green.card.id;
    let second_green = creature(81_103, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_green_id = second_green.card.id;
    let third_green = creature(81_104, cards::GRIZZLY_BEARS, PlayerId::One);
    let third_green_id = third_green.card.id;
    let (mut game, sprout) = game_with_sprout(vec![
        tapped_fanatic,
        prospector,
        first_green,
        second_green,
        third_green,
    ]);

    let action = sprout_cast(&game, sprout, true).expect(
        "sacrificing the tapped Goblin for mana and then convoking the Prospector pays {4}{G}",
    );
    let preview = game.mana_sources_for_action(PlayerId::One, &action);
    assert_eq!(
        preview.len(),
        4,
        "the Prospector appears once even though it contributes mana and convoke",
    );
    for source in [
        prospector_id,
        first_green_id,
        second_green_id,
        third_green_id,
    ] {
        assert_eq!(
            preview
                .iter()
                .filter(|candidate| **candidate == source)
                .count(),
            1,
            "each surviving payment source appears exactly once",
        );
    }
    assert!(
        !preview.contains(&fanatic_id),
        "the sacrificed Goblin is a cost object, not an additional mana source",
    );

    game.apply(PlayerId::One, action)
        .expect("the Prospector activation resolves before it is tapped to convoke");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fanatic_id),
        "the tapped Fanatic was sacrificed for red mana",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOGG_FANATIC),
        "the sacrificed Fanatic reaches the graveyard",
    );
    assert!(is_tapped(&game, prospector_id));
    assert!(is_tapped(&game, first_green_id));
    assert!(is_tapped(&game, second_green_id));
    assert!(is_tapped(&game, third_green_id));
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "the Prospector's red mana is consumed by the bought-back spell",
    );
    assert!(game.players[PlayerId::One.index()].mana.is_empty());

    pass_priority_pair(&mut game);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SPROUT_SWARM),
        "the paid buyback still returns Sprout Swarm to hand",
    );
}

#[test]
fn prospector_can_sacrifice_two_goblins_then_convoke_for_the_same_spell() {
    let mut first_fanatic = creature(81_110, cards::MOGG_FANATIC, PlayerId::One);
    let first_fanatic_id = first_fanatic.card.id;
    first_fanatic.tapped = true;
    let mut second_fanatic = creature(81_111, cards::MOGG_FANATIC, PlayerId::One);
    let second_fanatic_id = second_fanatic.card.id;
    second_fanatic.tapped = true;
    let prospector = creature(81_112, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    let first_green = creature(81_113, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_green_id = first_green.card.id;
    let second_green = creature(81_114, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_green_id = second_green.card.id;
    let (mut game, sprout) = game_with_sprout(vec![
        first_fanatic,
        second_fanatic,
        prospector,
        first_green,
        second_green,
    ]);

    let action = sprout_cast(&game, sprout, true)
        .expect("two Prospector activations, its convoke tap, and two green creatures pay {4}{G}");
    assert_preview_contains_each_once(
        &game,
        &action,
        &[prospector_id, first_green_id, second_green_id],
    );

    game.apply(PlayerId::One, action)
        .expect("both source-preserving Prospector activations execute before convoke");
    for fanatic in [first_fanatic_id, second_fanatic_id] {
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == fanatic),
            "each tapped Goblin is sacrificed exactly once",
        );
    }
    let prospector = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == prospector_id)
        .expect("the source-preserving Prospector remains on the battlefield");
    assert!(
        prospector.tapped,
        "the Prospector convokes after making mana"
    );
    assert_eq!(
        prospector
            .activations_this_turn
            .iter()
            .map(|(_, count)| u16::from(*count))
            .sum::<u16>(),
        2,
        "the same mana ability was activated once per sacrificed Goblin",
    );
    assert!(is_tapped(&game, first_green_id));
    assert!(is_tapped(&game, second_green_id));
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
    );
    assert!(game.players[PlayerId::One.index()].mana.is_empty());

    pass_priority_pair(&mut game);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SPROUT_SWARM),
        "buyback returns the multiply paid spell to hand",
    );
}

#[test]
fn replacement_choice_during_planned_mana_activation_suspends_the_cast() {
    let mut tapped_fanatic = creature(81_150, cards::MOGG_FANATIC, PlayerId::One);
    let fanatic_id = tapped_fanatic.card.id;
    tapped_fanatic.tapped = true;
    let prospector = creature(81_151, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    let first_green = creature(81_152, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_green_id = first_green.card.id;
    let second_green = creature(81_153, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_green_id = second_green.card.id;
    let third_green = creature(81_154, cards::GRIZZLY_BEARS, PlayerId::One);
    let third_green_id = third_green.card.id;
    let first_rest = creature(81_155, cards::REST_IN_PEACE, PlayerId::Two);
    let first_rest_id = first_rest.card.id;
    let second_rest = creature(81_156, cards::REST_IN_PEACE, PlayerId::Two);
    let (mut game, sprout) = game_with_sprout(vec![
        tapped_fanatic,
        prospector,
        first_green,
        second_green,
        third_green,
        first_rest,
        second_rest,
    ]);

    let action = sprout_cast(&game, sprout, true)
        .expect("Prospector mana plus four convoke payments can pay for bought-back Sprout Swarm");
    game.apply(PlayerId::One, action)
        .expect("the advertised cast suspends instead of trying to spend deferred mana");

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the two Rest in Peace replacements require an ordering choice");
    assert_eq!(
        decision.prompt,
        "Choose a replacement effect for Mogg Fanatic"
    );
    assert_eq!(decision.options.len(), 2);
    assert!(
        game.stack.is_empty(),
        "the half-paid spell is not exposed on the stack",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == fanatic_id),
        "the prospective sacrifice has not committed before its replacement is chosen",
    );
    let convoke_sources = [
        prospector_id,
        first_green_id,
        second_green_id,
        third_green_id,
    ];
    assert_tapped_state(&game, &convoke_sources, false);

    let replacement = decision
        .options
        .iter()
        .find(|option| decision_option_is_card(option, first_rest_id, cards::REST_IN_PEACE))
        .expect("the first Rest in Peace is one of the frozen replacements")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![replacement],
        },
    )
    .expect("choosing the replacement resumes and completes spell payment");

    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.stack.len(),
        1,
        "the fully paid spell is now on the stack"
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::MOGG_FANATIC),
        "Rest in Peace replaces the Prospector sacrifice destination",
    );
    assert_tapped_state(&game, &convoke_sources, true);
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "the resumed cast spends the Prospector mana",
    );
    assert!(game.players[PlayerId::One.index()].mana.is_empty());

    pass_priority_pair(&mut game);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SPROUT_SWARM),
        "paid buyback still returns the resumed spell to hand",
    );
}

#[test]
fn prospector_prefers_a_harmless_convoke_tap_over_sacrificing_a_goblin() {
    let prospector = creature(81_200, cards::SKIRK_PROSPECTOR, PlayerId::One);
    let prospector_id = prospector.card.id;
    let fanatic = creature(81_201, cards::MOGG_FANATIC, PlayerId::One);
    let fanatic_id = fanatic.card.id;
    let (mut game, sprout) = game_with_sprout(vec![prospector, fanatic]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let action = sprout_cast(&game, sprout, false)
        .expect("floating green plus one harmless convoke tap pays {1}{G}");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![prospector_id],
        "the default payment convokes the Prospector instead of sacrificing a Goblin",
    );

    game.apply(PlayerId::One, action)
        .expect("the preferred non-destructive payment applies");
    assert!(is_tapped(&game, prospector_id));
    assert!(
        !is_tapped(&game, fanatic_id),
        "the Fanatic was neither tapped nor sacrificed",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| [prospector_id, fanatic_id].contains(&permanent.card.id))
            .count(),
        2,
        "both Goblins remain on the battlefield",
    );
}

#[test]
fn many_wrong_color_multicolored_creatures_do_not_offer_a_convoke_cast() {
    let creatures = (0..25)
        .map(|offset| creature(81_300 + offset, cards::BLISTERCOIL_WEIRD, PlayerId::One))
        .collect();
    let (game, sprout) = game_with_sprout(creatures);

    let sprout_casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == sprout))
        .count();
    assert_eq!(
        sprout_casts, 0,
        "twenty-five blue/red choices still cannot satisfy Sprout Swarm's green symbol",
    );
}
