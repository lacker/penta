//! Graveyard arrivals, optional mill payoffs, and delayed object identity.
use super::*;

fn staged(definition: CardDefinitionId, zone: ZoneKind) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let object = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = object.id;
    match zone {
        ZoneKind::Library => game.players[0].library.push(object),
        ZoneKind::Graveyard => game.players[0].graveyard.push(object),
        ZoneKind::Hand => game.players[0].hand.push(object),
        _ => unreachable!(),
    }
    (game, id)
}

fn settle(game: &mut Game, accept: bool) {
    game.finish_rules_procedure();
    for _ in 0..64 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|p| p.observation.clone())
        {
            let label = if accept { "Do it" } else { "Decline" };
            let option = decision
                .options
                .iter()
                .find(|o| o.label == label)
                .unwrap_or_else(|| decision.options.first().expect("offered option"));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option.id],
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

fn move_card(game: &mut Game, id: GameObjectId, from: ZoneKind, to: ZoneKind) {
    game.move_card_from_nonbattlefield_zone(
        id,
        from,
        to,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
    )
    .unwrap();
}

fn end_step(game: &mut Game, player: PlayerId) {
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player,
    });
    settle(game, true);
}

#[test]
fn creeping_chill_requires_library_origin_and_successful_optional_exile() {
    for (from, accept, expected) in [
        (ZoneKind::Library, true, true),
        (ZoneKind::Library, false, false),
        (ZoneKind::Hand, true, false),
    ] {
        let (mut game, chill) = staged(cards::CREEPING_CHILL, from);
        move_card(&mut game, chill, from, ZoneKind::Graveyard);
        settle(&mut game, accept);
        assert_eq!(game.players[0].life, if expected { 23 } else { 20 });
        assert_eq!(game.players[1].life, if expected { 17 } else { 20 });
        assert_eq!(game.players[0].exile.len(), usize::from(expected));
    }
    let (mut game, chill) = staged(cards::CREEPING_CHILL, ZoneKind::Library);
    move_card(&mut game, chill, ZoneKind::Library, ZoneKind::Graveyard);
    game.finish_rules_procedure();
    let grave = game.players[0].graveyard[0].id;
    move_card(&mut game, grave, ZoneKind::Graveyard, ZoneKind::Hand);
    settle(&mut game, true);
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[1].life, 20);
}

#[test]
fn ghoul_counts_gross_life_gain_and_only_its_owners_end_step() {
    let (mut game, _) = staged(cards::SILVERSMOTE_GHOUL, ZoneKind::Graveyard);
    game.gain_life(PlayerId::One, 2);
    end_step(&mut game, PlayerId::One);
    assert!(game.battlefield.is_empty());
    game.gain_life(PlayerId::One, 1);
    game.players[0].life -= 5;
    end_step(&mut game, PlayerId::Two);
    assert!(game.battlefield.is_empty());
    end_step(&mut game, PlayerId::One);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn amalgam_waits_for_next_end_step_and_does_not_follow_a_new_graveyard_object() {
    for move_away in [false, true] {
        let (mut game, amalgam) = staged(cards::PRIZED_AMALGAM, ZoneKind::Graveyard);
        let narco = game
            .build_zone(PlayerId::One, &[cards::NARCOMOEBA])
            .unwrap()
            .remove(0);
        let narco_id = narco.id;
        game.players[0].library.push(narco);
        move_card(&mut game, narco_id, ZoneKind::Library, ZoneKind::Graveyard);
        settle(&mut game, true);
        assert_eq!(game.installed_triggers.len(), 1);
        assert_eq!(game.battlefield.len(), 1);
        if move_away {
            move_card(&mut game, amalgam, ZoneKind::Graveyard, ZoneKind::Hand);
            let fresh = game.players[0].hand[0].id;
            move_card(&mut game, fresh, ZoneKind::Hand, ZoneKind::Graveyard);
        }
        end_step(&mut game, PlayerId::Two);
        let returned = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::PRIZED_AMALGAM);
        assert_eq!(returned.is_some(), !move_away);
        if let Some(returned) = returned {
            assert!(returned.tapped);
        }
    }
}

#[test]
fn amalgam_checks_original_caster_when_a_graveyard_spell_changes_control() {
    let (mut game, _) = staged(cards::PRIZED_AMALGAM, ZoneKind::Graveyard);
    let crawler = game
        .build_zone(PlayerId::One, &[cards::UNDERWORLD_RAGE_HOUND])
        .unwrap()
        .remove(0);
    let id = crawler.id;
    game.players[0].graveyard.push(crawler);
    let fodder = game.build_zone(PlayerId::One, &[cards::SWAMP; 3]).unwrap();
    game.players[0].graveyard.extend(fodder);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action,
                Action::CastSpell { card, sacrifices, .. }
                    if *card == id && sacrifices.iter().all(|card| {
                        game.players[0].graveyard.iter().any(|held| {
                            held.id == *card && held.definition == cards::SWAMP
                        })
                    })
            )
        })
        .expect("Rage-Hound can escape using the three Swamps");
    game.apply(PlayerId::One, cast).unwrap();
    assert!(
        game.installed_triggers.is_empty(),
        "casting alone is insufficient"
    );
    game.stack.iter_mut().next_back().unwrap().controller = PlayerId::Two;
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    game = Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 1)
        .expect("the original caster survives checkpoint reconstruction");
    settle(&mut game, true);
    assert_eq!(game.installed_triggers.len(), 1);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    game = Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
        .expect("the delayed return survives checkpoint reconstruction");
    end_step(&mut game, PlayerId::One);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::PRIZED_AMALGAM && p.tapped)
    );
}
