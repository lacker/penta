//! Warp is a spell-resolution instruction, with a later-turn exile permission.
use super::*;

fn stage(definition: CardDefinitionId, prepared: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.prepared_engine.set_enabled(prepared);
    let spell = card(241_000, definition, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    (game, id)
}

fn cast(game: &mut Game, id: GameObjectId, warped: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == id && choices.costs().alternative().is_some() == warped)
        })
        .expect("the requested cost is legal");
    game.apply(PlayerId::One, action).unwrap();
}

fn end_step(game: &mut Game) {
    // The fixture uses Old School rules. Excess setup mana must not burn
    // the caster out of the game while advancing to the delayed trigger.
    for player in &mut game.players {
        player.mana_pool = ManaPool::default();
        player.mana.clear();
    }
    game.consecutive_passes = 0;
    game.step = Step::PostcombatMain;
    game.priority = game.active_player;
    pass_priority_pair(game);
    assert_eq!(game.step, Step::End);
}

fn round_trip(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        241_100,
    )
    .expect("Warp's frozen program and permissions reconstruct");
    rebuilt
        .prepared_engine
        .set_enabled(game.prepared_engine.enabled());
    rebuilt
}

#[test]
fn warp_riddler_resolves_without_an_extra_enters_trigger_and_recasts_later() {
    for prepared in [false, true] {
        let (mut game, id) = stage(cards::QUANTUM_RIDDLER, prepared);
        cast(&mut game, id, true);
        game = round_trip(&game);
        pass_priority_pair(&mut game);
        assert_eq!(game.installed_triggers.len(), 1);
        assert_eq!(
            game.stack.len(),
            1,
            "only the printed draw trigger uses the stack"
        );
        drain_pending(&mut game);
        assert_eq!(
            game.players[0].hand.len(),
            2,
            "the empty hand draws one plus one"
        );
        game = round_trip(&game);
        end_step(&mut game);
        assert_eq!(game.stack.len(), 1, "the delayed exile is counterable");
        drain_pending(&mut game);
        let exiled = game.players[0].exile[0].id;
        assert!(game.exile_play_permission(exiled, PlayerId::One).is_none());
        game = round_trip(&game);
        assert!(game.exile_play_permission(exiled, PlayerId::One).is_none());
        game.active_player = PlayerId::Two;
        assert!(
            game.exile_play_permission(exiled, PlayerId::One).is_some(),
            "even the opponent's next turn is a later turn"
        );
        game.active_player = PlayerId::One;
        game.turns_started[0] += 1;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        for color in [ManaColor::Blue, ManaColor::Colorless] {
            game.add_unrestricted_mana(PlayerId::One, color, 5);
        }
        cast(&mut game, exiled, false);
        drain_pending(&mut game);
        assert!(
            game.installed_triggers.is_empty(),
            "the normal recast is permanent"
        );
        assert_eq!(game.players[0].hand.len(), 3);
    }
}

#[test]
fn warp_does_not_apply_to_normal_casts_or_countered_spells() {
    for countered in [false, true] {
        let (mut game, id) = stage(cards::BYGONE_COLOSSUS, true);
        cast(&mut game, id, countered);
        if countered {
            let spell = game.stack.pop().unwrap();
            game.finish_stack_resolution(&spell, false);
        } else {
            drain_pending(&mut game);
        }
        assert!(game.installed_triggers.is_empty());
    }
}

#[test]
fn warp_survives_ability_loss_and_grants_the_owner_permission_after_control_changes() {
    let (mut game, id) = stage(cards::BYGONE_COLOSSUS, true);
    cast(&mut game, id, true);
    drain_pending(&mut game);
    let source = game.battlefield[0].card.id;
    game.battlefield[0].controller = PlayerId::Two;
    game.put_onto_battlefield(PlayerId::One, cards::HUMILITY)
        .unwrap();
    end_step(&mut game);
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == source));
    let permission = game
        .exile_play_permissions
        .iter()
        .find(|p| p.card == game.players[0].exile[0].id)
        .unwrap();
    assert_eq!(permission.player, PlayerId::One);
    assert!(!permission.lands_may_be_played);
    assert!(game.players[1].exile.is_empty());
}

#[test]
fn warp_does_not_follow_a_blinked_permanent_or_grant_permission_for_another_exile() {
    let (mut game, id) = stage(cards::BYGONE_COLOSSUS, true);
    cast(&mut game, id, true);
    drain_pending(&mut game);
    let source = game.battlefield[0].card.id;
    let exiled = game.exile_permanent_returning_card(source).unwrap();
    assert!(game.exile_play_permissions.is_empty());
    game.return_exiled_card(exiled, ZoneKind::Battlefield, None, None, false, None);
    let new_source = game.battlefield[0].card.id;
    game = round_trip(&game);
    end_step(&mut game);
    drain_pending(&mut game);
    assert!(game.battlefield.iter().any(|p| p.card.id == new_source));
    assert!(game.exile_play_permissions.is_empty());
}

#[test]
fn warp_copied_spell_installs_its_own_delayed_trigger() {
    let (mut game, id) = stage(cards::BYGONE_COLOSSUS, true);
    cast(&mut game, id, true);
    let original = game.stack.last().unwrap().clone();
    game.push_copy(original, PlayerId::One, Vec::new());
    drain_pending(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    assert_eq!(game.installed_triggers.len(), 2);
    game = round_trip(&game);
    end_step(&mut game);
    drain_pending(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "the spell copy became a token"
    );
    assert_eq!(game.exile_play_permissions.len(), 1);
}

#[test]
fn warp_timeline_culler_uses_its_graveyard_permission_and_life_cost() {
    let (mut game, id) = stage(cards::TIMELINE_CULLER, true);
    let card = game.players[0].hand.pop().unwrap();
    game.players[0].graveyard.push(card);
    cast(&mut game, id, true);
    assert_eq!(game.players[0].life, 18);
    drain_pending(&mut game);
    assert_eq!(game.installed_triggers.len(), 1);
    end_step(&mut game);
    drain_pending(&mut game);
    assert_eq!(game.players[0].exile.len(), 1);
}

#[test]
fn warp_riddler_adds_once_to_a_whole_draw_instruction_and_stacks_additively() {
    for (copies, held, draw, expected) in [
        (1, 0, 3, 4),
        (1, 1, 3, 4),
        (1, 2, 3, 3),
        (2, 0, 3, 5),
        (2, 0, 0, 0),
    ] {
        let mut game = ready_game();
        for _ in 0..copies {
            game.put_onto_battlefield(PlayerId::One, cards::QUANTUM_RIDDLER)
                .unwrap();
        }
        drain_pending(&mut game);
        game.players[0].hand.clear();
        for i in 0..held {
            game.players[0]
                .hand
                .push(card(241_200 + i, cards::ISLAND, PlayerId::One));
        }
        game.draw_instruction(PlayerId::One, draw);
        drain_pending(&mut game);
        assert_eq!(
            game.players[0].hand.len(),
            usize::try_from(held + expected).unwrap()
        );
    }
}

#[test]
fn warp_starfield_vocalist_doubles_enters_but_not_the_resolution_instruction() {
    let (mut game, id) = stage(cards::QUANTUM_RIDDLER, true);
    game.put_onto_battlefield(PlayerId::One, cards::STARFIELD_VOCALIST)
        .unwrap();
    drain_pending(&mut game);
    cast(&mut game, id, true);
    pass_priority_pair(&mut game);
    assert_eq!(game.installed_triggers.len(), 1);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        3,
        "first draw becomes two, next draw stays one"
    );
}

#[test]
fn warp_countering_the_delayed_trigger_leaves_the_permanent_indefinitely() {
    let (mut game, id) = stage(cards::BYGONE_COLOSSUS, true);
    cast(&mut game, id, true);
    drain_pending(&mut game);
    end_step(&mut game);
    let trigger = game.stack.pop().unwrap();
    game.finish_stack_resolution(&trigger, false);
    assert!(game.installed_triggers.is_empty());
    game.turns_started[0] += 1;
    end_step(&mut game);
    assert!(game.stack.is_empty());
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.exile_play_permissions.is_empty());
}

#[test]
fn warp_riddler_instruction_replacement_precedes_individual_draw_replacements() {
    static DOUBLE: [AbilityDef; 1] = [AbilityDef::replacement_for(
        "If you would draw a card, draw two cards instead.",
        ReplacementEventDef::WouldDraw {
            player: PlayerRelation::You,
            during_own_draw_step: false,
            except_first_in_draw_step: false,
        },
        ReplacementEffectDef::Sequence(&[
            ReplacementEffectDef::ReplaceEventWithNothing,
            ReplacementEffectDef::Perform(&EffectDef::Sequence(&[
                EffectDef::ContinueReplacedDraw,
                EffectDef::ContinueReplacedDraw,
            ])),
        ]),
    )];
    let mut game = ready_game();
    let mut definition = game.catalog.get(cards::HOWLING_MINE).unwrap().clone();
    definition.rules = CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&DOUBLE);
    synchronize_single_part_definition(&mut definition);
    game.catalog = CardCatalog::new(game.catalog.definitions().into_iter().map(|card| {
        if card.id == definition.id {
            definition.clone()
        } else {
            card.clone()
        }
    }))
    .unwrap();
    game.prepared_engine = crate::prepared_engine::PreparedEngine::compile(&game.catalog);
    game.put_onto_battlefield(PlayerId::One, cards::QUANTUM_RIDDLER)
        .unwrap();
    drain_pending(&mut game);
    game.players[0].hand.clear();
    game.put_onto_battlefield(PlayerId::One, cards::HOWLING_MINE)
        .unwrap();
    game.draw_instruction(PlayerId::One, 2);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        6,
        "(two plus one) draws are each doubled"
    );
}
