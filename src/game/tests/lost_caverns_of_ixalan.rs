//! Composition regressions for the WOE–HOB Standard Ixalan audit.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let mut cards = game.build_zone(PlayerId::One, &[definition]).unwrap();
    let id = cards[0].id;
    game.players[0].hand.append(&mut cards);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 8);
    }
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("a complete legal cast exists");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(game);
}

#[test]
fn defossilize_explores_the_returned_successor_twice() {
    let mut game = board(&[cards::FOREST, cards::ISLAND]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let old = game.players[0].graveyard[0].id;
    cast(&mut game, cards::DEFOSSILIZE);
    let returned = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
        .unwrap();
    assert_ne!(returned.card.id, old);
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(game.players[0].library.is_empty());
}

#[test]
fn bonehoard_binds_both_exiled_cards_and_grants_land_play() {
    let mut game = board(&[cards::LIGHTNING_BOLT, cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::BONEHOARD_DRACOSAUR)
        .unwrap();
    game.step = Step::Upkeep;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerId::One,
    });
    drain_pending(&mut game);
    assert_eq!(game.players[0].exile.len(), 2);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let land = game.players[0]
        .exile
        .iter()
        .find(|c| c.definition == cards::FOREST)
        .unwrap()
        .id;
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::PlayLand {card, ..} if *card == land))
    );
}

#[test]
fn starving_revenant_keeps_two_draws_as_two_life_loss_pairs() {
    let mut game = board(&[cards::FOREST, cards::ISLAND]);
    let start = game.players[0].life;
    game.put_onto_battlefield(PlayerId::One, cards::STARVING_REVENANT)
        .unwrap();
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|p| p.observation.clone())
        {
            // Decline the optional graveyard selection; accept the mandatory
            // acknowledgement and the complete ordering of the kept cards.
            let options = decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|o| o.id)
                .collect();
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
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    assert_eq!(game.players[0].life, start - 6);
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn attack_while_condition_is_not_rechecked_on_resolution() {
    let mut game = board(&[]);
    let hammer = game
        .put_onto_battlefield(PlayerId::One, cards::PUGNACIOUS_HAMMERSKULL)
        .unwrap();
    for p in &mut game.battlefield {
        p.entered_controller_turn = 0;
    }
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: hammer,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::COLOSSADACTYL)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == hammer)
            .unwrap()
            .counters(CounterKind::Stun),
        1
    );
}

#[test]
fn kitesail_overwrites_both_targets_and_restores_them_when_it_leaves() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    let pirate = game
        .put_onto_battlefield(PlayerId::One, cards::KITESAIL_LARCENIST)
        .unwrap();
    drain_pending(&mut game);
    for id in [bear, angel] {
        let p = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
        assert!(
            game.permanent_types(p)
                .unwrap()
                .contains(CardType::Artifact)
        );
        assert!(
            !game
                .permanent_types(p)
                .unwrap()
                .contains(CardType::Creature)
        );
        assert!(!game.mana_ability_activations(p).is_empty());
    }
    game.move_permanents_to_graveyard(&[pirate]);
    for id in [bear, angel] {
        let p = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
        assert!(
            game.permanent_types(p)
                .unwrap()
                .contains(CardType::Creature)
        );
        assert!(
            !game
                .permanent_types(p)
                .unwrap()
                .contains(CardType::Artifact)
        );
    }
}

#[test]
fn graveyard_reanimation_finality_is_present_before_the_creature_can_die() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::VAMPIRE_NIGHTHAWK])
        .unwrap();
    let before = game.players[0].life;
    game.put_onto_battlefield(PlayerId::One, cards::QUEEN_S_BAY_PALADIN)
        .unwrap();
    drain_pending(&mut game);
    let returned = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::VAMPIRE_NIGHTHAWK)
        .unwrap();
    assert_eq!(returned.counters(CounterKind::Finality), 1);
    assert_eq!(game.players[0].life, before - 3);
    let id = returned.card.id;
    game.move_permanents_to_graveyard(&[id]);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::VAMPIRE_NIGHTHAWK)
    );
}

#[test]
fn matzalantli_requires_four_distinct_permanent_types() {
    let mut game = board(&[]);
    let door = game
        .put_onto_battlefield(PlayerId::One, cards::MATZALANTLI_THE_GREAT_DOOR)
        .unwrap();
    game.players[0].graveyard = game
        .build_zone(
            PlayerId::One,
            &[
                cards::FOREST,
                cards::GRIZZLY_BEARS,
                cards::CRUSADE,
                cards::LIGHTNING_BOLT,
            ],
        )
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let transform = activated_ability_for(&game, door, 1);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&plain_activation(door, transform))
    );
    let mut artifact = game.build_zone(PlayerId::One, &[cards::SOL_RING]).unwrap();
    game.players[0].graveyard.append(&mut artifact);
    game.apply(PlayerId::One, plain_activation(door, transform))
        .unwrap();
    drain_pending(&mut game);
    let land = game.battlefield.iter().find(|p| p.card.id == door).unwrap();
    assert!(game.permanent_types(land).unwrap().contains(CardType::Land));
    assert!(
        !game
            .permanent_types(land)
            .unwrap()
            .contains(CardType::Artifact)
    );
}

#[test]
fn saheeli_delayed_sacrifice_survives_her_leaving() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let saheeli = game
        .put_onto_battlefield(PlayerId::One, cards::SAHEELI_THE_SUN_S_BRILLIANCE)
        .unwrap();
    for p in &mut game.battlefield {
        p.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility {source,..} if *source==saheeli))
        .unwrap();
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(
        game.permanent_types(copy)
            .unwrap()
            .contains(CardType::Artifact)
    );
    assert!(game.permanent_has_executable_keyword(copy, KeywordAbility::Haste));
    game.move_permanents_to_graveyard(&[saheeli]);
    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn sanguine_evangelist_keeps_both_pending_entry_and_death_triggers() {
    let mut game = board(&[]);
    let vampire = game
        .put_onto_battlefield(PlayerId::One, cards::SANGUINE_EVANGELIST)
        .unwrap();
    game.move_permanents_to_graveyard(&[vampire]);
    drain_pending(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    for bat in &game.battlefield {
        assert!(bat.card.definition.is_token());
        assert_eq!(game.power(bat), Some(1));
        assert!(game.permanent_has_executable_keyword(bat, KeywordAbility::Flying));
    }
}

#[test]
fn resplendent_angel_uses_life_gained_rather_than_current_life() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::RESPLENDENT_ANGEL)
        .unwrap();
    game.gain_life(PlayerId::One, 5);
    game.players[0].life = 3;
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    let angel = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(angel), Some(4));
    assert!(game.permanent_has_executable_keyword(angel, KeywordAbility::Flying));
    assert!(game.permanent_has_executable_keyword(angel, KeywordAbility::Vigilance));
}

#[test]
fn self_reflection_is_castable_on_your_creature_and_rejects_opponents() {
    let mut game = board(&[]);
    let ours = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let spell = game
        .build_zone(PlayerId::One, &[cards::SELF_REFLECTION])
        .unwrap();
    let id = spell[0].id;
    game.players[0].hand.extend(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action,Action::CastSpell {card,..} if *card==id))
        .collect();
    assert!(!casts.is_empty());
    assert!(casts.iter().all(|action|matches!(action,Action::CastSpell {choices,..} if choices.iter_targets().any(|target|*target==Target::Permanent(ours)) && !choices.iter_targets().any(|target|*target==Target::Permanent(theirs)))));
    game.apply(PlayerId::One, casts[0].clone()).unwrap();
    drain_pending(&mut game);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(copy), Some(2));
    assert_eq!(game.toughness(copy), Some(2));
}
