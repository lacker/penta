//! Regression cases for the existing primitives composed by the eternal imports.

use super::*;

fn hold(game: &mut Game, player: PlayerId, definition: CardDefinitionId) -> GameObjectId {
    let card = game.build_zone(player, &[definition]).unwrap().remove(0);
    let id = card.id;
    game.players[player.index()].hand.push(card);
    id
}

fn cast(game: &mut Game, player: PlayerId, held: GameObjectId, x: u16) {
    game.active_player = player;
    game.priority = player;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == held && choices.x() == x)
        })
        .expect("an affordable cast");
    game.apply(player, action).unwrap();
}

fn on_board(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn deafening_silence_counts_noncreatures_separately_for_each_player() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::DEAFENING_SILENCE_10)
        .unwrap();
    for player in [PlayerId::One, PlayerId::Two] {
        game.add_unrestricted_mana(player, ManaColor::Green, 10);
        let bears = hold(&mut game, player, cards::GRIZZLY_BEARS);
        cast(&mut game, player, bears, 0);
        drain_pending(&mut game);
        let vial = hold(&mut game, player, cards::AETHER_VIAL_91);
        cast(&mut game, player, vial, 0);
        drain_pending(&mut game);
        let second = hold(&mut game, player, cards::AETHER_VIAL_91);
        let creature = hold(&mut game, player, cards::GRIZZLY_BEARS);
        game.priority = player;
        let actions = game.legal_actions(player);
        assert!(!actions.iter().any(|action| matches!(action,
            Action::CastSpell { card, .. } if *card == second)));
        assert!(actions.iter().any(|action| matches!(action,
            Action::CastSpell { card, .. } if *card == creature)));
    }
}

#[test]
fn aether_vial_chooses_at_resolution_without_casting() {
    let mut game = ready_game();
    let vial = game
        .put_onto_battlefield(PlayerId::One, cards::AETHER_VIAL_91)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == vial)
        .unwrap()
        .set_counters(CounterKind::named("charge"), 1);
    let bears = hold(&mut game, PlayerId::One, cards::GRIZZLY_BEARS);
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == vial))
        .unwrap();
    game.apply(PlayerId::One, activate).unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == vial)
        .unwrap()
        .set_counters(CounterKind::named("charge"), 2);
    pass_until_decision(&mut game);
    assert_eq!(
        game.pending_decisions
            .first()
            .unwrap()
            .observation
            .visibility,
        DecisionVisibility::Private
    );
    drain_pending(&mut game);
    assert!(!game.players[0].hand.iter().any(|card| card.id == bears));
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
    assert!(game.players[0].mana.is_empty());
}

#[test]
fn temporary_lockdown_returns_both_players_permanents_when_removed() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::AETHER_VIAL_91)
        .unwrap();
    let land = game
        .put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .unwrap();
    let lockdown = game
        .put_onto_battlefield(PlayerId::One, cards::TEMPORARY_LOCKDOWN)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[1].exile.len(), 1);
    assert!(on_board(&game, land));
    let held = hold(&mut game, PlayerId::One, cards::DISENCHANT);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action|
        matches!(action, Action::CastSpell { card, choices, .. } if *card == held
            && choices.targets().iter().any(|selection| selection.targets().contains(&Target::Permanent(lockdown))))
    ).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert!(game.players.iter().all(|player| player.exile.is_empty()));
    for (definition, owner) in [
        (cards::GRIZZLY_BEARS, PlayerId::One),
        (cards::AETHER_VIAL_91, PlayerId::Two),
    ] {
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == definition && p.controller == owner)
        );
    }
}

#[test]
fn canoptek_counts_an_artifact_land_once_and_only_cards_actually_exiled() {
    let mut game = ready_game();
    game.players[1].graveyard = game
        .build_zone(
            PlayerId::Two,
            &[
                cards::VAULT_OF_WHISPERS_286,
                cards::MOUNTAIN,
                cards::GRIZZLY_BEARS,
            ],
        )
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::CANOPTEK_SCARAB_SWARM)
        .unwrap();
    // The trigger's target choice is the only opponent, selected through its offered option.
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|p| p.observation.clone())
        {
            let chosen = decision
                .options
                .iter()
                .find(|o| o.label.contains("Player 2"))
                .unwrap_or_else(|| decision.options.last().unwrap())
                .id;
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![chosen],
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        } else {
            let player = game.priority;
            game.apply(player, Action::PassPriority).unwrap();
        }
    }
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[1].exile.len(), 3);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.token_characteristics.is_some())
            .count(),
        2
    );
}

#[test]
fn gaeas_will_has_a_green_indicator_and_no_payable_printed_cost() {
    let mut game = ready_game();
    let definition = game.catalog.get(cards::GAEA_S_WILL).unwrap();
    assert!(definition.rules.has_color(ManaColor::Green));
    let held = hold(&mut game, PlayerId::One, cards::GAEA_S_WILL);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 10);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
    );
}

#[test]
fn pre_war_formalwear_attaches_to_the_returned_incarnation() {
    let mut game = ready_game();
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let equipment = game
        .put_onto_battlefield(PlayerId::One, cards::PRE_WAR_FORMALWEAR_21)
        .unwrap();
    drain_pending(&mut game);
    let bears = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
        .unwrap();
    assert_eq!(game.power(bears), Some(4));
    assert_eq!(game.toughness(bears), Some(4));
    assert!(on_board(&game, equipment));
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn infernal_tutor_uses_the_revealed_name_or_an_unrestricted_hellbent_search() {
    for hellbent in [false, true] {
        let mut game = ready_game();
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::MOUNTAIN, cards::GRIZZLY_BEARS])
            .unwrap();
        if !hellbent {
            hold(&mut game, PlayerId::One, cards::MOUNTAIN);
        }
        let held = hold(&mut game, PlayerId::One, cards::INFERNAL_TUTOR);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
        cast(&mut game, PlayerId::One, held, 0);
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), if hellbent { 1 } else { 2 });
        if !hellbent {
            assert!(
                game.players[0]
                    .hand
                    .iter()
                    .all(|card| card.definition == cards::MOUNTAIN)
            );
        }
        assert_eq!(game.players[0].library.len(), 1);
    }
}

#[test]
fn consign_can_replicate_a_colorless_counter_but_cannot_target_a_blue_spell() {
    for (spell, legal) in [(cards::AETHER_VIAL_91, true), (cards::PONDER, false)] {
        let mut game = ready_game();
        let theirs = hold(&mut game, PlayerId::Two, spell);
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
        cast(&mut game, PlayerId::Two, theirs, 0);
        let held = hold(&mut game, PlayerId::One, cards::CONSIGN_TO_MEMORY_54);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        game.priority = PlayerId::One;
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == held && choices.costs().additional().len() == 1)
            });
        assert_eq!(cast.is_some(), legal);
        if let Some(cast) = cast {
            game.apply(PlayerId::One, cast).unwrap();
            assert_eq!(
                game.stack.len(),
                3,
                "original spell, counterspell, replicate trigger"
            );
            drain_pending(&mut game);
            assert!(game.stack.is_empty());
            assert!(
                game.players[1]
                    .graveyard
                    .iter()
                    .any(|card| card.definition == spell)
            );
        }
    }
}

#[test]
fn fantasticar_counts_only_noncreature_spells_and_sacrifices_for_four_tokens() {
    let mut game = ready_game();
    let car = game
        .put_onto_battlefield(PlayerId::One, cards::THE_FANTASTICAR)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let creature = hold(&mut game, PlayerId::One, cards::ORNITHOPTER);
    cast(&mut game, PlayerId::One, creature, 0);
    drain_pending(&mut game);
    for count in 1..=4 {
        let held = hold(&mut game, PlayerId::One, cards::AETHER_VIAL_91);
        cast(&mut game, PlayerId::One, held, 0);
        for _ in 0..32 {
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|p| p.observation.clone())
            {
                if decision.prompt.contains("fourth noncreature") {
                    assert_eq!(
                        decision.options.len(),
                        2,
                        "only decline or sacrifice the source"
                    );
                }
                let options = if decision.minimum > 1 {
                    decision
                        .options
                        .iter()
                        .take(decision.minimum)
                        .map(|o| o.id)
                        .collect()
                } else {
                    vec![decision.options.last().unwrap().id]
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
                break;
            } else {
                let priority = game.priority;
                game.apply(priority, Action::PassPriority).unwrap();
            }
        }
        assert_eq!(on_board(&game, car), count < 4);
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.token_characteristics.is_some())
                .count(),
            if count == 4 { 4 } else { 0 }
        );
    }
}
