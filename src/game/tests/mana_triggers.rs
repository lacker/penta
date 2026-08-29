//! Mana added off somebody else's tap.
//!
//! "Its controller adds an additional {G}" points at the player who tapped the
//! land, not at the player who owns the thing watching. Wild Growth can sit on
//! an opponent's land and the Gauntlet watches every Mountain on the table, so
//! both cards read wrong if the mana lands in the wrong pool.

use super::*;

fn tap_for(game: &mut Game, player: PlayerId, land: GameObjectId, color: ManaColor) {
    // A mana ability still wants its controller to hold priority.
    game.priority = player;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source,
                    color: produced,
                    ..
                } if *source == land && *produced == color
            )
        })
        .expect("the requested mana ability is offered");
    game.apply(player, action).expect("the land taps for mana");
}

fn pool(game: &Game, player: PlayerId) -> ManaPool {
    game.players[player.index()].mana_pool
}

/// Wild Growth on a land its own controller owns: the ordinary case, and the
/// one that would pass even if the recipient were read from the Aura.
#[test]
fn wild_growth_adds_to_its_own_controllers_pool() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let mut aura = creature(10_001, cards::WILD_GROWTH, PlayerId::One);
    aura.attached_to = Some(forest_id);
    game.battlefield.push(aura);

    tap_for(&mut game, PlayerId::One, forest_id, ManaColor::Green);

    assert_eq!(
        pool(&game, PlayerId::One).green,
        2,
        "the Forest's own green plus the Aura's"
    );
}

/// The case that decides it: the Aura is player one's, the land is player
/// two's, and the extra mana belongs to player two.
#[test]
fn wild_growth_pays_the_land_controller_not_the_aura_controller() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::Two);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let mut aura = creature(10_001, cards::WILD_GROWTH, PlayerId::One);
    aura.attached_to = Some(forest_id);
    game.battlefield.push(aura);

    tap_for(&mut game, PlayerId::Two, forest_id, ManaColor::Green);

    assert_eq!(
        pool(&game, PlayerId::Two).green,
        2,
        "the land's controller gets both"
    );
    assert_eq!(
        pool(&game, PlayerId::One).green,
        0,
        "and the Aura's controller gets none of it"
    );
}

#[test]
fn overgrowth_adds_two_fixed_green_mana() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let mut aura = creature(10_001, cards::OVERGROWTH, PlayerId::One);
    aura.attached_to = Some(forest_id);
    game.battlefield.push(aura);

    tap_for(&mut game, PlayerId::One, forest_id, ManaColor::Green);

    assert_eq!(pool(&game, PlayerId::One).green, 3);
}

#[test]
fn dawns_reflection_offers_every_two_mana_color_combination() {
    let mut game = ready_game();
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);
    let mut aura = creature(10_001, cards::DAWNS_REFLECTION, PlayerId::One);
    aura.attached_to = Some(mountain_id);
    game.battlefield.push(aura);
    game.priority = PlayerId::One;

    let choices = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == mountain_id)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        choices.len(),
        15,
        "five colors divide two mana fifteen ways"
    );
    let white_blue = choices
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    triggered_mana: Some(splits),
                    ..
                } if splits.len() == 1
                    && splits[0].get(ManaColor::White) == 1
                    && splits[0].get(ManaColor::Blue) == 1
            )
        })
        .expect("one choice adds white and blue");
    game.apply(PlayerId::One, white_blue).unwrap();

    assert_eq!(pool(&game, PlayerId::One).red, 1);
    assert_eq!(pool(&game, PlayerId::One).white, 1);
    assert_eq!(pool(&game, PlayerId::One).blue, 1);
}

#[test]
fn shimmerwilds_growth_reuses_its_chosen_color_for_land_and_mana() {
    let mut game = ready_game();
    let plains = creature(10_000, cards::PLAINS, PlayerId::One);
    let plains_id = plains.card.id;
    game.battlefield.push(plains);
    let mut aura = creature(10_001, cards::SHIMMERWILDS_GROWTH, PlayerId::One);
    aura.attached_to = Some(plains_id);
    aura.chosen_color = Some(ManaColor::Blue);
    game.battlefield.push(aura);

    assert_eq!(
        game.object_colors(plains_id),
        [false, true, false, false, false],
        "the attached land is the Aura's chosen color"
    );
    tap_for(&mut game, PlayerId::One, plains_id, ManaColor::White);

    assert_eq!(pool(&game, PlayerId::One).white, 1);
    assert_eq!(pool(&game, PlayerId::One).blue, 1);
}

#[test]
fn shimmerwilds_growth_chooses_its_color_as_it_enters() {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    let plains = creature(10_000, cards::PLAINS, PlayerId::One);
    let plains_id = plains.card.id;
    game.battlefield.push(plains);
    let spell = card(20_000, cards::SHIMMERWILDS_GROWTH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the land is a legal Aura target");
    game.apply(PlayerId::One, action).expect("the Aura is cast");
    pass_priority_pair(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the Aura asks for a color");
    let blue = decision
        .options
        .iter()
        .find(|option| option.label == "Blue")
        .expect("blue is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![blue],
        },
    )
    .expect("blue is a legal choice");
    drain_pending(&mut game);

    let aura = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SHIMMERWILDS_GROWTH)
        .expect("the Aura entered");
    assert_eq!(aura.attached_to, Some(plains_id));
    assert_eq!(aura.chosen_color, Some(ManaColor::Blue));
}

/// The Gauntlet watches every Mountain, including the ones across the table.
#[test]
fn the_gauntlet_pays_whoever_tapped_the_mountain() {
    let mut game = ready_game();
    let gauntlet = creature(10_000, cards::GAUNTLET_OF_MIGHT, PlayerId::One);
    game.battlefield.push(gauntlet);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);

    tap_for(&mut game, PlayerId::Two, mountain_id, ManaColor::Red);

    assert_eq!(
        pool(&game, PlayerId::Two).red,
        2,
        "an opponent's Mountain still doubles, for them"
    );
    assert_eq!(pool(&game, PlayerId::One).red, 0);
}

#[test]
fn mana_flare_pays_whoever_tapped_the_land() {
    let mut game = ready_game();
    let flare = creature(10_000, cards::MANA_FLARE, PlayerId::One);
    let flare_id = flare.card.id;
    game.battlefield.push(flare);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);

    tap_for(&mut game, PlayerId::Two, mountain_id, ManaColor::Red);

    assert_eq!(pool(&game, PlayerId::Two).red, 2);
    assert_eq!(pool(&game, PlayerId::One).red, 0);
    assert!(
        game.players[PlayerId::Two.index()]
            .mana
            .iter()
            .any(|mana| mana.source.is_some_and(|source| source.object == flare_id))
    );
}

#[test]
fn mana_flare_chooses_among_every_type_the_land_produced() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MANA_FLARE, PlayerId::One));
    let expanse = creature(10_001, cards::SKYCLOUD_EXPANSE, PlayerId::One);
    let expanse_id = expanse.card.id;
    game.battlefield.push(expanse);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;

    let choices = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == expanse_id)
        })
        .collect::<Vec<_>>();
    assert_eq!(choices.len(), 2, "the Flare may choose white or blue");
    let add_blue = choices
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    triggered_mana: Some(split),
                    ..
                } if split.len() == 1 && split[0].get(ManaColor::Blue) == 1
            )
        })
        .expect("one action selects blue for Mana Flare");
    game.apply(PlayerId::One, add_blue).unwrap();

    assert_eq!(pool(&game, PlayerId::One).white, 1, "the land adds white");
    assert_eq!(
        pool(&game, PlayerId::One).blue,
        2,
        "the land and Mana Flare each add blue"
    );
}

#[test]
fn multiple_mana_flares_make_independent_type_choices() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MANA_FLARE, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::MANA_FLARE, PlayerId::Two));
    let expanse = creature(10_002, cards::SKYCLOUD_EXPANSE, PlayerId::One);
    let expanse_id = expanse.card.id;
    game.battlefield.push(expanse);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;

    let choices = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == expanse_id)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        choices.len(),
        4,
        "each Flare makes its own white-or-blue choice"
    );
    let split = choices
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    triggered_mana: Some(split),
                    ..
                } if split.len() == 2
                    && split[0].get(ManaColor::White) == 1
                    && split[1].get(ManaColor::Blue) == 1
            )
        })
        .expect("one Flare can choose each type");
    game.apply(PlayerId::One, split).unwrap();

    assert_eq!(pool(&game, PlayerId::One).white, 2);
    assert_eq!(pool(&game, PlayerId::One).blue, 2);
}

#[test]
fn mana_flare_adds_one_for_a_land_that_produces_several_of_one_type() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MANA_FLARE, PlayerId::One));
    let tomb = creature(10_001, cards::ANCIENT_TOMB, PlayerId::One);
    let tomb_id = tomb.card.id;
    game.battlefield.push(tomb);

    tap_for(&mut game, PlayerId::One, tomb_id, ManaColor::Colorless);

    assert_eq!(pool(&game, PlayerId::One).colorless, 3);
}

#[test]
fn mana_flare_ignores_nonland_mana_sources_and_informs_planning() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MANA_FLARE, PlayerId::One));
    let elf = creature(10_001, cards::LLANOWAR_ELVES, PlayerId::One);
    let elf_id = elf.card.id;
    game.battlefield.push(elf);

    tap_for(&mut game, PlayerId::One, elf_id, ManaColor::Green);
    assert_eq!(pool(&game, PlayerId::One).green, 1, "the Elf is not a land");
    game.players[PlayerId::One.index()].mana_pool = ManaPool::default();
    game.players[PlayerId::One.index()].mana.clear();

    let mountain = creature(10_002, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);
    assert!(
        game.can_pay_cost(PlayerId::One, mana_cost!("{2}"), 0),
        "the planner counts the Mountain and its Mana Flare trigger"
    );
    game.activate_mana_for_cost(PlayerId::One, mana_cost!("{2}"), 0);
    let spent = game.pay_player_cost(PlayerId::One, mana_cost!("{2}"), 0);
    assert_eq!(spent.len(), 2, "the planned activation pays the whole cost");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

/// The other half of the Gauntlet, and the control for the trigger's
/// predicate: it names Mountains, so nothing else pays out.
#[test]
fn the_gauntlet_pumps_red_creatures_and_ignores_other_lands() {
    let mut game = ready_game();
    let gauntlet = creature(10_000, cards::GAUNTLET_OF_MIGHT, PlayerId::One);
    game.battlefield.push(gauntlet);
    let goblin = creature(10_001, cards::MONSS_GOBLIN_RAIDERS, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    let plains = creature(10_002, cards::PLAINS, PlayerId::One);
    let plains_id = plains.card.id;
    game.battlefield.push(plains);

    let goblin = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == goblin_id)
        .expect("still there");
    assert_eq!(game.power(goblin), Some(2), "a 1/1 Goblin is red");

    tap_for(&mut game, PlayerId::One, plains_id, ManaColor::White);
    assert_eq!(
        pool(&game, PlayerId::One).white,
        1,
        "a Plains is not a Mountain"
    );
}
