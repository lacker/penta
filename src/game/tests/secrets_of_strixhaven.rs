//! Secrets of Strixhaven compositions and casting regressions.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.players[0].library.reverse();
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
fn repartee_triggers_once_for_a_spell_with_creature_targets() {
    let mut game = board(&[cards::FOREST; 4]);
    game.put_onto_battlefield(PlayerId::One, cards::LECTURING_SCORNMAGE)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::INFORMED_INKWRIGHT)
        .unwrap();
    cast(&mut game, cards::INTERJECTION);
    assert_eq!(
        permanent(&game, cards::LECTURING_SCORNMAGE).counters(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn fractal_anomaly_counts_cards_drawn_this_turn() {
    let mut game = board(&[]);
    game.cards_drawn_this_turn[0] = 3;
    cast(&mut game, cards::FRACTAL_ANOMALY);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(token), Some(3));
    assert_eq!(token.counters(CounterKind::PlusOnePlusOne), 3);
}

#[test]
fn slowlands_exclude_themselves_from_the_two_other_lands_requirement() {
    for n in 0..3 {
        let mut game = board(&[]);
        for _ in 0..n {
            game.put_onto_battlefield(PlayerId::One, cards::FOREST)
                .unwrap();
        }
        game.put_onto_battlefield(PlayerId::One, cards::DEATHCAP_GLADE)
            .unwrap();
        assert_eq!(permanent(&game, cards::DEATHCAP_GLADE).tapped, n < 2);
    }
}

#[test]
fn infusion_static_bonuses_follow_life_gain_and_turn_reset() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::ULNA_ALLEY_SHOPKEEP)
        .unwrap();
    assert_eq!(
        game.power(permanent(&game, cards::ULNA_ALLEY_SHOPKEEP)),
        Some(2)
    );
    game.life_gained_this_turn[0] = 1;
    assert_eq!(
        game.power(permanent(&game, cards::ULNA_ALLEY_SHOPKEEP)),
        Some(4)
    );
    game.life_gained_this_turn[0] = 0;
    assert_eq!(
        game.power(permanent(&game, cards::ULNA_ALLEY_SHOPKEEP)),
        Some(2)
    );
}

#[test]
fn converge_entry_counts_colors_actually_paid() {
    for color_count in [1_u16, 3, 5] {
        let mut game = board(&[]);
        let id = held(&mut game, cards::RANCOROUS_ARCHAIC);
        game.players[0].mana_pool = ManaPool::default();
        for color in ManaColor::COLORS.into_iter().take(usize::from(color_count)) {
            game.add_unrestricted_mana(PlayerId::One, color, 1);
        }
        game.add_unrestricted_mana(PlayerId::One, ManaColor::COLORS[0], 5 - color_count);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert_eq!(
            permanent(&game, cards::RANCOROUS_ARCHAIC).counters(CounterKind::PlusOnePlusOne),
            color_count
        );
    }
}

#[test]
fn aziza_copies_only_after_tapping_three_untapped_creatures() {
    for available in [2, 3] {
        let mut game = board(&[]);
        game.put_onto_battlefield(PlayerId::One, cards::AZIZA_MAGE_TOWER_CAPTAIN)
            .unwrap();
        for _ in 1..available {
            game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
                .unwrap();
        }
        cast(&mut game, cards::TOME_BLAST);
        assert_eq!(
            game.battlefield.iter().filter(|p| p.tapped).count(),
            if available == 3 { 3 } else { 0 }
        );
        assert_eq!(
            game.players.iter().map(|p| i32::from(p.life)).sum::<i32>(),
            if available == 3 { 36 } else { 38 }
        );
    }
}

#[test]
fn heated_argument_exiles_a_card_before_its_controller_damage() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    game.players[0].graveyard = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
    cast(&mut game, cards::HEATED_ARGUMENT);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|c| c.definition != cards::FOREST)
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[1].life, 18);
}

#[test]
fn page_grandeur_reveals_through_a_spell_and_bottoms_the_rest() {
    let mut game = board(&[cards::FOREST, cards::INTERJECTION, cards::ISLAND]);
    game.put_onto_battlefield(PlayerId::One, cards::PAGE_LOOSE_LEAF)
        .unwrap();
    held(&mut game, cards::PAGE_LOOSE_LEAF);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::INTERJECTION);
    assert_eq!(
        game.players[0].library.last().unwrap().definition,
        cards::ISLAND
    );
    assert_eq!(game.players[0].library[0].definition, cards::FOREST);
}

#[test]
fn flashback_grants_the_target_cards_actual_mana_cost() {
    let mut game = board(&[cards::FOREST; 3]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::QUICK_STUDY])
        .unwrap();
    cast(&mut game, cards::FLASHBACK);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::QUICK_STUDY)
    );
}

#[test]
fn group_project_flashback_taps_three_creatures_and_exiles_the_spell() {
    let mut game = board(&[]);
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .unwrap();
    }
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GROUP_PROJECT])
        .unwrap();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.iter().filter(|p| p.tapped).count(), 3);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::GROUP_PROJECT)
    );
}

#[test]
fn mind_roots_takes_an_actual_discarded_land_under_your_control() {
    let mut game = board(&[]);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::FOREST, cards::INTERJECTION])
        .unwrap();
    let id = held(&mut game, cards::MIND_ROOTS);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.iter_targets().any(|t|*t == Target::Player(PlayerId::Two)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let land = permanent(&game, cards::FOREST);
    assert_eq!(land.controller, PlayerId::One);
    assert!(land.tapped);
    assert_eq!(land.card.owner, PlayerId::Two);
    assert_eq!(game.players[1].graveyard.len(), 1);
}

#[test]
fn diary_activation_uses_page_counters_for_its_discount() {
    let mut game = board(&[cards::FOREST; 2]);
    game.put_onto_battlefield(PlayerId::One, cards::DIARY_OF_DREAMS)
        .unwrap();
    for _ in 0..5 {
        cast(&mut game, cards::TOME_BLAST);
    }
    game.players[0].mana_pool = ManaPool::default();
    let id = permanent(&game, cards::DIARY_OF_DREAMS).card.id;
    assert_eq!(
        permanent(&game, cards::DIARY_OF_DREAMS).counters(CounterKind::named("page")),
        5
    );
    activate(&mut game, id);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn dinas_guidance_decides_the_destination_after_searching() {
    let mut game = board(&[cards::SAVANNAH_LIONS]);
    cast(&mut game, cards::DINA_S_GUIDANCE);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::SAVANNAH_LIONS)
    );
    assert!(game.players[0].library.is_empty());
}

#[test]
fn stoneglider_additional_cost_offers_graveyard_exile_or_extra_mana() {
    for exile in [false, true] {
        let mut game = board(&[]);
        let id = held(&mut game, cards::SOARING_STONEGLIDER);
        game.players[0].mana_pool = ManaPool::default();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
        if exile {
            game.players[0].graveyard =
                game.build_zone(PlayerId::One, &[cards::FOREST; 2]).unwrap();
        } else {
            assert!(
                !game
                    .legal_actions(PlayerId::One)
                    .iter()
                    .any(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
            );
            game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
        }
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].exile.len(), if exile { 2 } else { 0 });
        assert_eq!(
            game.power(permanent(&game, cards::SOARING_STONEGLIDER)),
            Some(4)
        );
    }
}

#[test]
fn killian_returns_from_the_graveyard_after_combat_damage_and_payment() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::KILLIAN_S_CONFIDENCE])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    let attacker = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    game.declare_attacker(attacker, AttackDefender::Player(PlayerId::Two));
    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.players[0].hand[0].definition,
        cards::KILLIAN_S_CONFIDENCE
    );
}
