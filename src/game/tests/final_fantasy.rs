//! Regression coverage for FINAL FANTASY combinations of existing game operations.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game.cards_drawn_this_turn = [0, 0];
    game
}

fn settle(game: &mut Game) {
    for _ in 0..64 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = if matches!(
                pending.continuation,
                DecisionContinuation::OptionalEffect { .. } | DecisionContinuation::PayOr { .. }
            ) {
                vec![decision.options.last().unwrap().id]
            } else {
                decision
                    .options
                    .iter()
                    .map(|o| o.id)
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("resolution did not settle");
}

fn held(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let cards = game.build_zone(PlayerId::One, &[definition]).unwrap();
    let id = cards[0].id;
    game.players[0].hand.extend(cards);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    id
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

fn activate(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} | Action::ActivateManaAbility {source,..} if *source == id))
        .expect("ability is available");
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn job_select_equips_a_surviving_hero_and_grants_its_cast_trigger() {
    let mut game = board(&[cards::FOREST]);
    let rod = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_MAGE_S_ROD)
        .unwrap();
    settle(&mut game);
    let hero = permanent(&game, cards::BLACK_MAGE_S_ROD)
        .attached_to
        .unwrap();
    let body = game.battlefield.iter().find(|p| p.card.id == hero).unwrap();
    assert_eq!(game.power(body), Some(2));
    assert!(game.effective_subtypes(body).contains(&"Hero"));
    assert!(game.effective_subtypes(body).contains(&"Wizard"));
    let life = game.players[1].life;
    cast(&mut game, cards::DREAMS_OF_LAGUNA);
    assert_eq!(game.players[1].life, life - 1);
    game.destroy_permanents(&[rod], false);
    settle(&mut game);
    let body = game.battlefield.iter().find(|p| p.card.id == hero).unwrap();
    assert_eq!(game.power(body), Some(1));
    assert!(!game.effective_subtypes(body).contains(&"Wizard"));
}

#[test]
fn tiered_fire_selects_exactly_one_cost_and_damage_amount() {
    for (mode, damage, cost) in [(0, 1, 1), (1, 2, 3), (2, 3, 6)] {
        let mut game = board(&[]);
        game.put_onto_battlefield(PlayerId::Two, cards::IRON_GIANT)
            .unwrap();
        let id = held(&mut game, cards::FIRE_MAGIC);
        let before = game.players[0].mana_pool.total();
        let actions = game.legal_actions(PlayerId::One);
        assert!(
            actions
                .iter()
                .filter_map(|a| match a {
                    Action::CastSpell { card, choices, .. } if *card == id => Some(choices),
                    _ => None,
                })
                .all(|c| c.modes().len() == 1)
        );
        let action=actions.into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id&&choices.modes()[0].index()==mode)).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert_eq!(permanent(&game, cards::IRON_GIANT).damage, damage);
        assert_eq!(before - game.players[0].mana_pool.total(), cost);
    }
}

#[test]
fn tiered_limit_break_scales_current_power_and_toughness() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let id = held(&mut game, cards::TIFA_S_LIMIT_BREAK);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id&&choices.modes()[0].index()==2&&choices.iter_targets().any(|t|*t==Target::Permanent(bear)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.power(permanent(&game, cards::GRIZZLY_BEARS)), Some(6));
    assert_eq!(
        game.toughness(permanent(&game, cards::GRIZZLY_BEARS)),
        Some(6)
    );
}

#[test]
fn vincent_limit_break_returns_the_creature_without_its_old_buff() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let id = held(&mut game, cards::VINCENT_S_LIMIT_BREAK);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.modes()[0].index()==1)).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.power(permanent(&game, cards::GRIZZLY_BEARS)), Some(5));
    game.destroy_permanents(&[bear], false);
    settle(&mut game);
    let returned = permanent(&game, cards::GRIZZLY_BEARS);
    assert_ne!(returned.card.id, bear);
    assert!(returned.tapped);
    assert_eq!(game.power(returned), Some(2));
}

#[test]
fn titan_saga_returns_milled_lands_tapped_then_finishes() {
    let mut game = board(&[
        cards::FOREST,
        cards::ISLAND,
        cards::GRIZZLY_BEARS,
        cards::MOUNTAIN,
        cards::PLAINS,
    ]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let titan = game
        .put_onto_battlefield(PlayerId::One, cards::SUMMON_TITAN)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].graveyard.len(), 5);
    game.add_counters_to_permanent(titan, CounterKind::Lore, 1);
    settle(&mut game);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| game
                .permanent_types(p)
                .is_some_and(|types| types.contains(CardType::Land))
                && p.tapped)
            .count(),
        4
    );
    game.add_counters_to_permanent(titan, CounterKind::Lore, 1);
    settle(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == titan));
    assert_eq!(
        game.power(game.battlefield.iter().find(|p| p.card.id == bear).unwrap()),
        Some(6)
    );
}

#[test]
fn adventure_land_is_cast_as_an_adventure_and_then_played_as_a_land() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::LINDBLUM_INDUSTRIAL_REGENCY);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.play_option()==PlayOptionId(1))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.battlefield.iter().any(|p|p.card.definition.is_token()&&game.effective_subtypes(p).contains(&"Wizard")));
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|c| c.definition == cards::LINDBLUM_INDUSTRIAL_REGENCY)
        .unwrap()
        .id;
    let land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::PlayLand{card,..}if *card==exiled))
        .expect("Adventure permits its land face from exile");
    game.apply(PlayerId::One, land).unwrap();
    settle(&mut game);
    let town = permanent(&game, cards::LINDBLUM_INDUSTRIAL_REGENCY);
    assert!(town.tapped);
    assert!(
        game.permanent_types(town)
            .is_some_and(|types| types.contains(CardType::Land))
    );
}

#[test]
fn card_collection_transforms_into_a_noncreature_vehicle_with_printed_stats() {
    let mut game = board(&[cards::FOREST, cards::ISLAND, cards::MOUNTAIN]);
    let gy = game.build_zone(PlayerId::One, &[cards::PLAINS; 6]).unwrap();
    game.players[0].graveyard.extend(gy);
    game.put_onto_battlefield(PlayerId::One, cards::SIDEQUEST_CARD_COLLECTION)
        .unwrap();
    settle(&mut game);
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    let vehicle = permanent(&game, cards::SIDEQUEST_CARD_COLLECTION);
    assert!(
        !game
            .permanent_types(vehicle)
            .is_some_and(|types| types.contains(CardType::Creature))
    );
    assert!(game.effective_subtypes(vehicle).contains(&"Vehicle"));
    assert_eq!(game.effective_rules(vehicle).unwrap().mana_cost(), None);
}

#[test]
fn memories_returning_handles_every_short_library_size() {
    for count in 0..=5 {
        let mut game = board(
            &[
                cards::FOREST,
                cards::ISLAND,
                cards::SWAMP,
                cards::MOUNTAIN,
                cards::PLAINS,
            ][..count],
        );
        cast(&mut game, cards::MEMORIES_RETURNING);
        assert_eq!(game.players[0].hand.len(), count.div_ceil(2));
        assert_eq!(game.players[0].library.len(), count / 2);
    }
}

#[test]
fn town_greeter_only_gains_life_for_a_town_taken_from_its_mill() {
    for land in [cards::ADVENTURER_S_INN, cards::FOREST] {
        let mut game = board(&[
            land,
            cards::GRIZZLY_BEARS,
            cards::GRIZZLY_BEARS,
            cards::GRIZZLY_BEARS,
        ]);
        let before = game.players[0].life;
        game.put_onto_battlefield(PlayerId::One, cards::TOWN_GREETER)
            .unwrap();
        settle(&mut game);
        assert_eq!(
            game.players[0].life,
            before
                + if land == cards::ADVENTURER_S_INN {
                    2
                } else {
                    0
                }
        );
        assert_eq!(game.players[0].hand.len(), 1);
    }
}

#[test]
fn fire_crystal_delayed_sacrifice_survives_the_crystal_leaving() {
    let mut game = board(&[]);
    let crystal = game
        .put_onto_battlefield(PlayerId::One, cards::THE_FIRE_CRYSTAL)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    activate(&mut game, crystal);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.destroy_permanents(&[crystal], false);
    settle(&mut game);
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == copy));
}

#[test]
fn self_destruct_deals_both_damage_assignments_even_when_its_source_dies() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let before = game.players[1].life;
    let id = held(&mut game, cards::SELF_DESTRUCT);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.iter_targets().any(|t|*t==Target::Player(PlayerId::Two)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].life, before - 2);
    assert!(!game.battlefield.iter().any(|p| p.card.id == bear));
}

#[test]
fn wind_crystal_doubles_one_life_gain_event() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::THE_WIND_CRYSTAL)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::AERITH_GAINSBOROUGH)
        .unwrap();
    let before = game.players[0].life;
    game.put_onto_battlefield(PlayerId::One, cards::ADVENTURER_S_INN)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].life, before + 4);
    assert_eq!(
        permanent(&game, cards::AERITH_GAINSBOROUGH).counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn wastes_is_basic_and_produces_colorless_mana() {
    let mut game = board(&[]);
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::WASTES)
        .unwrap();
    activate(&mut game, id);
    assert_eq!(game.players[0].mana_pool.total(), 1);
    assert!(
        game.effective_rules(permanent(&game, cards::WASTES))
            .unwrap()
            .has_supertype(CardSupertype::Basic)
    );
}

#[test]
fn ninjas_blades_uses_discarded_mana_value_and_the_damaged_player() {
    let mut game = board(&[cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::NINJA_S_BLADES)
        .unwrap();
    settle(&mut game);
    let hero = permanent(&game, cards::NINJA_S_BLADES).attached_to.unwrap();
    held(&mut game, cards::IRON_GIANT);
    let before = game.players[1].life;
    game.damage_target_from_kind(Some(hero), Some(Target::Player(PlayerId::Two)), 2, true);
    settle(&mut game);
    assert_eq!(game.players[1].life, before - 9);
    assert_eq!(game.players[0].life, 20);
}

#[test]
fn triple_triad_only_grants_the_lower_mana_value_opposing_card() {
    for other in [cards::GRIZZLY_BEARS, cards::IRON_GIANT] {
        let mut game = board(&[cards::HILL_GIGAS]);
        game.players[1].library = game.build_zone(PlayerId::Two, &[other]).unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::TRIPLE_TRIAD)
            .unwrap();
        game.commit_next_turn(PlayerId::One, Vec::new());
        settle(&mut game);
        game.step = Step::PrecombatMain;
        let mine = game.players[0].exile[0].id;
        let theirs = game.players[1].exile[0].id;
        let actions = game.legal_actions(PlayerId::One);
        assert!(
            actions
                .iter()
                .any(|a| matches!(a,Action::CastSpell{card,..}if *card==mine))
        );
        assert_eq!(
            actions
                .iter()
                .any(|a| matches!(a,Action::CastSpell{card,..}if *card==theirs)),
            other == cards::GRIZZLY_BEARS
        );
    }
}

#[test]
fn diamond_weapon_prevents_combat_damage_but_takes_noncombat_damage() {
    let mut game = board(&[]);
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::DIAMOND_WEAPON)
        .unwrap();
    game.damage_target_from_kind(None, Some(Target::Permanent(id)), 5, true);
    assert_eq!(permanent(&game, cards::DIAMOND_WEAPON).damage, 0);
    game.damage_target_from_kind(None, Some(Target::Permanent(id)), 3, false);
    assert_eq!(permanent(&game, cards::DIAMOND_WEAPON).damage, 3);
}
