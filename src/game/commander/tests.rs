use super::*;
use crate::card::{self, cards};
use crate::game::tests::{checkpoint_fixture, drain_pending};
use crate::game::{Action, BattlefieldArrival, GameResult, Step, Target, WinReason};
use crate::{Deck, Format, ManaColor};

fn staged(commanders: Vec<CardDefinitionId>) -> Game {
    let deck = Deck {
        commanders,
        main: vec![cards::FOREST; 99],
        sideboard: Vec::new(),
    };
    let opponent = Deck {
        commanders: vec![cards::SAVANNAH_LIONS],
        main: vec![cards::PLAINS; 99],
        sideboard: Vec::new(),
    };
    let mut game =
        Game::new_with_format(Format::Cedh, card::catalog().unwrap(), [deck, opponent], 1).unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game
}

fn castable(game: &Game, id: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
}

fn choose(game: &mut Game, options: Vec<u32>) {
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
}

fn enter(game: &mut Game, owner: PlayerId, index: usize) -> GameObjectId {
    let id = game.players[owner.index()].command[index].id;
    game.move_target_to_zone(
        Target::Card(id),
        ZoneKind::Battlefield,
        ZoneMoveCause::Rules,
        Some(BattlefieldArrival::under(owner)),
        ZonePlacement::Top,
    );
    drain_pending(game);
    game.battlefield.last().unwrap().card.id
}

#[test]
fn commander_setup_keeps_designations_out_of_library_and_checkpoints_them() {
    let game = staged(vec![cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    assert_eq!(game.players[0].life, 40);
    assert_eq!(game.hand(PlayerId::One).len(), 7);
    assert_eq!(game.library(PlayerId::One).len(), 92);
    assert_eq!(game.command_zone(PlayerId::One).len(), 2);
    for viewer in [PlayerId::One, PlayerId::Two] {
        let (wire, hidden) = checkpoint_fixture(&game, viewer);
        let restored = Game::from_observation_checkpoint(
            game.catalog.clone(),
            Format::Cedh,
            &wire,
            &hidden,
            2,
        )
        .unwrap();
        assert_eq!(game.commanders(viewer), restored.commanders(viewer));
        assert_eq!(
            game.command_zone(PlayerId::One),
            restored.command_zone(PlayerId::One)
        );
    }
}

#[test]
fn commander_cast_tax_is_individual_and_applies_only_from_command_zone() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let bear = game.players[0].command[0].id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    let action = castable(&game, bear).expect("initial commander cast costs printed mana");
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.commanders[0].casts, 1);
    assert_eq!(game.commanders[1].casts, 0);
    drain_pending(&mut game);
    let bear = game.battlefield[0].card.id;
    game.return_permanent_to_hand(bear);
    choose(&mut game, vec![0]);
    let returned = game.players[0]
        .command
        .iter()
        .find(|c| c.definition == cards::GRIZZLY_BEARS)
        .unwrap()
        .id;
    assert_ne!(bear, returned);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    assert!(castable(&game, returned).is_none());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.apply(PlayerId::One, castable(&game, returned).unwrap())
        .unwrap();
    assert_eq!(game.commanders[0].casts, 2);
    drain_pending(&mut game);
    let bear = game.battlefield[0].card.id;
    game.return_permanent_to_hand(bear);
    choose(&mut game, vec![1]);
    let held = game.players[0]
        .hand
        .iter()
        .find(|c| c.definition == cards::GRIZZLY_BEARS)
        .unwrap()
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.apply(PlayerId::One, castable(&game, held).unwrap())
        .unwrap();
    assert_eq!(game.commanders[0].casts, 2);
}

#[test]
fn commander_death_and_exile_choices_wait_for_sbas_and_declining_is_not_reoffered() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let bear = enter(&mut game, PlayerId::One, 0);
    game.move_permanents_to_graveyard(&[bear]);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(
        game.pending_decisions.is_empty(),
        "death happens before the return decision"
    );
    game.check_state_based_actions();
    assert_eq!(game.pending_decisions.len(), 1);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let restored =
        Game::from_observation_checkpoint(game.catalog.clone(), Format::Cedh, &wire, &hidden, 3)
            .unwrap();
    assert_eq!(restored.pending_decisions.len(), 1);
    choose(&mut game, vec![]);
    game.check_state_based_actions();
    assert!(game.pending_decisions.is_empty());
    let dead = game.players[0].graveyard[0].id;
    game.move_target_to_zone(
        Target::Card(dead),
        ZoneKind::Exile,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    game.check_state_based_actions();
    choose(&mut game, vec![0]);
    assert_eq!(game.players[0].command.len(), 1);
}

#[test]
fn commander_blink_during_resolution_does_not_offer_an_intervening_return() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let bear = enter(&mut game, PlayerId::One, 0);
    game.exile_permanent(bear);
    let exiled = game.players[0].exile[0].id;
    game.move_target_to_zone(
        Target::Card(exiled),
        ZoneKind::Battlefield,
        ZoneMoveCause::Rules,
        Some(BattlefieldArrival::under(PlayerId::One)),
        ZonePlacement::Top,
    );
    drain_pending(&mut game);
    game.check_state_based_actions();
    assert!(game.pending_decisions.is_empty());
    assert!(game.is_commander(game.battlefield[0].card.id));
}

#[test]
fn commander_damage_tracks_physical_card_through_control_and_zone_changes() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let bear = enter(&mut game, PlayerId::One, 0);
    game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::Two)), 20, true);
    game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::Two)), 1, false);
    assert_eq!(game.commanders[0].damage, [0, 20]);
    game.players[1].life = 40;
    game.return_permanent_to_hand(bear);
    choose(&mut game, vec![0]);
    let bear = enter(&mut game, PlayerId::One, 1);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .controller = PlayerId::Two;
    game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::Two)), 1, true);
    game.check_state_based_actions();
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentCommanderDamage
        })
    );
}

#[test]
fn commander_identity_is_not_inherited_by_a_second_card_or_a_token_copy() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let commander = enter(&mut game, PlayerId::One, 0);
    let ordinary = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    assert!(game.is_commander(commander));
    assert!(!game.is_commander(ordinary));
    game.create_token_copy(
        PlayerId::One,
        crate::game::tests::copied_characteristics(cards::GRIZZLY_BEARS),
        None,
        crate::CardPartId::PRIMARY,
    );
    drain_pending(&mut game);
    let token = game.battlefield.last().unwrap().card.id;
    assert!(!game.is_commander(token));
    for permanent in &game.battlefield {
        assert_eq!(
            game.trigger_object_matches(
                crate::ObjectPredicateDef::Commander,
                &game.trigger_event_object(permanent),
                commander,
                false
            ),
            permanent.card.id == commander
        );
    }
}

#[test]
fn commander_hidden_zone_replacement_belongs_to_owner_and_preserves_library_placement() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let bear = enter(&mut game, PlayerId::One, 0);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .controller = PlayerId::Two;
    game.move_permanents_to_zone(&[bear], ZoneKind::Library, ZonePlacement::Bottom);
    assert_eq!(game.pending_decisions[0].observation.player, PlayerId::One);
    assert!(game.is_commander(bear));
    choose(&mut game, vec![1]);
    let tucked = game.players[0].library.first().unwrap();
    assert_eq!(tucked.definition, cards::GRIZZLY_BEARS);
    assert!(game.is_commander(tucked.id));
    assert!(game.commanders(PlayerId::One)[0].object.is_none());
    assert!(game.commanders(PlayerId::Two)[0].object.is_none());
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::Two);
    let restored =
        Game::from_observation_checkpoint(game.catalog.clone(), Format::Cedh, &wire, &hidden, 4)
            .unwrap();
    assert_eq!(
        game.commanders(PlayerId::Two),
        restored.commanders(PlayerId::Two)
    );
}

#[test]
fn commander_spell_bounce_can_return_to_command_without_refunding_cast_history() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let bear = game.players[0].command[0].id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.apply(PlayerId::One, castable(&game, bear).unwrap())
        .unwrap();
    let spell = game.stack.last().unwrap().id;
    game.return_spell_to_hand(spell);
    assert_eq!(game.stack.len(), 1);
    choose(&mut game, vec![0]);
    assert!(game.stack.is_empty());
    assert_eq!(game.commanders[0].casts, 1);
    assert_eq!(game.players[0].command.len(), 1);
}

#[test]
fn commander_partners_do_not_combine_combat_damage() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let bear = enter(&mut game, PlayerId::One, 0);
    let lion = enter(&mut game, PlayerId::One, 0);
    game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::Two)), 11, true);
    game.damage_target_from_kind(Some(lion), Some(Target::Player(PlayerId::Two)), 10, true);
    game.check_state_based_actions();
    assert!(game.result.is_none());
    assert_eq!(game.commanders[0].damage, [0, 11]);
    assert_eq!(game.commanders[1].damage, [0, 10]);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let restored =
        Game::from_observation_checkpoint(game.catalog.clone(), Format::Cedh, &wire, &hidden, 4)
            .unwrap();
    assert_eq!(
        game.commanders(PlayerId::One),
        restored.commanders(PlayerId::One)
    );
}

#[test]
fn commander_tax_participates_in_ordinary_cost_reductions() {
    let mut game = staged(vec![cards::GRIZZLY_BEARS]);
    let bear = game.players[0].command[0].id;
    game.commanders[0].casts = 1;
    game.put_onto_battlefield(PlayerId::One, cards::HELM_OF_AWAKENING)
        .unwrap();
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
    assert!(
        castable(&game, bear).is_some(),
        "printed two plus tax two minus Helm one"
    );
}
