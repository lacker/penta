//! Memory's Journey's linked player-and-graveyard targets.

use super::*;
use crate::ImplementationStatus;

fn journey_action(
    game: &Game,
    spell: GameObjectId,
    player: PlayerId,
    cards: &[GameObjectId],
) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            let Action::CastSpell {
                card: source,
                choices,
                ..
            } = action
            else {
                return false;
            };
            *source == spell
                && choices.targets().len() == 2
                && choices.targets()[0].targets() == [Target::Player(player)]
                && choices.targets()[1]
                    .targets()
                    .iter()
                    .copied()
                    .eq(cards.iter().copied().map(Target::Card))
        })
}

#[test]
fn journey_only_offers_cards_from_the_target_players_graveyard() {
    let mut game = ready_game();
    let journey = card(20_000, cards::MEMORY_S_JOURNEY, PlayerId::One);
    let journey_id = journey.id;
    let ours = card(20_001, cards::MOUNTAIN, PlayerId::One);
    let theirs = card(20_002, cards::FOREST, PlayerId::Two);
    game.players[0].hand.push(journey);
    game.players[0].graveyard.push(ours.clone());
    game.players[1].graveyard.push(theirs.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    assert!(journey_action(&game, journey_id, PlayerId::One, &[ours.id]).is_some());
    assert!(journey_action(&game, journey_id, PlayerId::Two, &[theirs.id]).is_some());
    assert!(journey_action(&game, journey_id, PlayerId::One, &[theirs.id]).is_none());
    assert!(journey_action(&game, journey_id, PlayerId::Two, &[ours.id]).is_none());

    let Action::CastSpell {
        card,
        choices,
        sacrifices,
    } = journey_action(&game, journey_id, PlayerId::One, &[ours.id])
        .expect("the honest linked selection is offered")
    else {
        unreachable!("the helper only returns a cast action")
    };
    let forged = Action::CastSpell {
        card,
        choices: choices.with_targets(vec![
            TargetSelection::single(TargetSlotId(0), Target::Player(PlayerId::One)),
            TargetSelection::single(TargetSlotId(1), Target::Card(theirs.id)),
        ]),
        sacrifices,
    };
    assert!(
        game.apply(PlayerId::One, forged).is_err(),
        "submitted actions revalidate the player-card ownership link"
    );
}

#[test]
fn journey_moves_up_to_three_cards_then_shuffles_that_players_library() {
    let mut game = ready_game_with_seed(7);
    let journey = card(20_000, cards::MEMORY_S_JOURNEY, PlayerId::One);
    let journey_id = journey.id;
    let returned = [
        card(20_001, cards::FOREST, PlayerId::Two),
        card(20_002, cards::MOUNTAIN, PlayerId::Two),
        card(20_003, cards::PLAINS, PlayerId::Two),
    ];
    let returned_ids = returned.iter().map(|card| card.id).collect::<Vec<_>>();
    let returned_definitions = returned
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    game.players[0].hand.push(journey);
    game.players[1].graveyard.extend(returned);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let library_before = game.players[1].library.len();

    let action = journey_action(&game, journey_id, PlayerId::Two, &returned_ids)
        .expect("all three cards are legal linked targets");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    pass_priority_pair(&mut game);

    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[1].library.len(), library_before + 3);
    assert!(returned_definitions.iter().all(|definition| {
        game.players[1]
            .library
            .iter()
            .any(|card| card.definition == *definition)
    }));
}

#[test]
fn journey_resolves_for_cards_that_remain_legal() {
    let mut game = ready_game();
    let journey = card(20_000, cards::MEMORY_S_JOURNEY, PlayerId::One);
    let journey_id = journey.id;
    let first = card(20_001, cards::FOREST, PlayerId::Two);
    let second = card(20_002, cards::MOUNTAIN, PlayerId::Two);
    game.players[0].hand.push(journey);
    game.players[1]
        .graveyard
        .extend([first.clone(), second.clone()]);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let action = journey_action(&game, journey_id, PlayerId::Two, &[first.id, second.id])
        .expect("both cards begin as legal targets");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    game.players[1].graveyard.retain(|card| card.id != first.id);
    let library_before = game.players[1].library.len();

    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].library.len(), library_before + 1);
    assert!(
        game.players[1]
            .library
            .iter()
            .any(|card| card.definition == second.definition)
    );
}

#[test]
fn journey_copy_retargeting_preserves_the_player_graveyard_link() {
    let mut game = ready_game();
    let journey = card(20_000, cards::MEMORY_S_JOURNEY, PlayerId::One);
    let journey_id = journey.id;
    let ours = card(20_001, cards::MOUNTAIN, PlayerId::One);
    let theirs = card(20_002, cards::FOREST, PlayerId::Two);
    game.players[0].hand.push(journey);
    game.players[0].graveyard.push(ours.clone());
    game.players[1].graveyard.push(theirs.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let action = journey_action(&game, journey_id, PlayerId::One, &[ours.id])
        .expect("the original linked targets are legal");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    let spell = game.stack.last().expect("Journey is on the stack");

    let choices = game.copy_target_choices(spell, PlayerId::One);
    let has_pair = |player, card| {
        choices.iter().any(|targets| {
            targets[0].targets() == [Target::Player(player)]
                && targets[1].targets() == [Target::Card(card)]
        })
    };
    assert!(has_pair(PlayerId::One, ours.id));
    assert!(has_pair(PlayerId::Two, theirs.id));
    assert!(!has_pair(PlayerId::One, theirs.id));
}

#[test]
fn journey_can_target_no_cards_and_has_green_flashback() {
    let mut game = ready_game();
    let journey = card(20_000, cards::MEMORY_S_JOURNEY, PlayerId::One);
    let journey_id = journey.id;
    game.players[0].graveyard.push(journey);
    game.players[0].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == journey_id
                        && choices.costs().alternative() == Some(AlternativeCostId(1))
                        && choices.targets()[0].targets() == [Target::Player(PlayerId::One)]
                        && choices.targets()[1].targets().is_empty()
            )
        })
        .expect("green flashback can choose zero graveyard-card targets");
    game.apply(PlayerId::One, action)
        .expect("flashback cast is legal");
    pass_priority_pair(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::MEMORY_S_JOURNEY)
    );
}

#[test]
fn journey_reports_complete_declarative_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let journey = catalog
        .get(cards::MEMORY_S_JOURNEY)
        .expect("Memory's Journey is cataloged");
    assert_eq!(
        journey.rules.implementation_status(),
        ImplementationStatus::Complete
    );
    assert!(
        journey
            .rules
            .ability_clauses()
            .iter()
            .all(|ability| { ability.effect.execution == EffectExecutionDef::Declarative })
    );
}
