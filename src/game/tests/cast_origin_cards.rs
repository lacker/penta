//! Cast-origin consumers: a cast trigger, two permanent-entry checks, and a
//! resolving spell whose hand-only branch must not repeat from exile.

use super::*;

fn settle(game: &mut Game) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .map_or_else(
                    || {
                        decision
                            .options
                            .iter()
                            .take(decision.minimum.max(1).min(decision.maximum))
                            .map(|option| option.id)
                            .collect()
                    },
                    |option| vec![option.id],
                );
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn cast_of(game: &Game, player: PlayerId, card: GameObjectId) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(
            |action| matches!(action, Action::CastSpell { card: offered, .. } if *offered == card),
        )
        .expect("the card has a cast action")
}

fn wizard_count(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| game.effective_subtypes(permanent).contains(&"Wizard"))
        .count()
}

#[test]
fn burning_vengeance_only_sees_spells_cast_from_your_graveyard() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::BURNING_VENGEANCE)
        .expect("Burning Vengeance is cataloged");
    drain_pending(&mut game);

    let flashback = card(240_000, cards::THINK_TWICE, PlayerId::One);
    let flashback_id = flashback.id;
    game.players[0].graveyard.push(flashback);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(PlayerId::One, cast_of(&game, PlayerId::One, flashback_id))
        .expect("Think Twice flashes back");
    settle(&mut game);

    assert_eq!(game.players[1].life, 18, "the graveyard cast dealt two");
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::THINK_TWICE)
    );

    let hand_cast = card(240_001, cards::THINK_TWICE, PlayerId::One);
    let hand_id = hand_cast.id;
    game.players[0].hand.push(hand_cast);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.priority = PlayerId::One;
    game.apply(PlayerId::One, cast_of(&game, PlayerId::One, hand_id))
        .expect("Think Twice casts from hand");
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "the otherwise identical hand cast did not trigger it",
    );
}

#[test]
fn phage_remembers_whether_the_permanent_spell_came_from_hand() {
    let mut cast_game = ready_game();
    let phage = card(240_010, cards::PHAGE_THE_UNTOUCHABLE, PlayerId::One);
    let phage_id = phage.id;
    cast_game.players[0].hand.push(phage);
    cast_game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 7);
    cast_game
        .apply(PlayerId::One, cast_of(&cast_game, PlayerId::One, phage_id))
        .expect("Phage casts from hand");
    settle(&mut cast_game);

    assert!(cast_game.result.is_none(), "a hand-cast Phage is safe");
    assert!(
        cast_game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::PHAGE_THE_UNTOUCHABLE)
    );

    let mut put_game = ready_game();
    put_game
        .put_onto_battlefield(PlayerId::One, cards::PHAGE_THE_UNTOUCHABLE)
        .expect("Phage is cataloged");
    settle(&mut put_game);

    assert_eq!(
        put_game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentLostToAnEffect,
        }),
        "a Phage that was not cast from hand loses the game",
    );
}

#[test]
fn phage_combat_damage_to_a_player_ends_the_game() {
    let mut game = ready_game();
    let phage = creature(240_020, cards::PHAGE_THE_UNTOUCHABLE, PlayerId::One);
    let phage_id = phage.card.id;
    game.battlefield.push(phage);

    game.deal_combat_damage_to_player(phage_id, PlayerId::Two, 4);
    settle(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostToAnEffect,
        }),
    );
}

#[test]
fn transpose_creates_one_wizard_and_does_not_repeat_its_hand_only_branch() {
    let mut game = ready_game();
    let transpose = card(240_030, cards::TRANSPOSE, PlayerId::One);
    let transpose_id = transpose.id;
    game.players[0].hand.push(transpose);
    game.players[0]
        .hand
        .push(card(240_031, cards::MOUNTAIN, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(PlayerId::One, cast_of(&game, PlayerId::One, transpose_id))
        .expect("Transpose casts from hand");
    settle(&mut game);

    assert_eq!(game.players[0].life, 19);
    assert_eq!(wizard_count(&game), 1, "the hand cast made its Wizard");
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::TRANSPOSE)
        .expect("rebound exiled Transpose")
        .id;

    for _ in 0..60 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            game.advance_step();
        } else {
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("priority advances to rebound");
        }
    }
    assert_eq!(game.step, Step::Upkeep);
    let rebound = cast_of(&game, PlayerId::One, exiled);
    game.apply(PlayerId::One, rebound)
        .expect("the rebound cast is free");
    settle(&mut game);

    assert_eq!(game.players[0].life, 18, "both copies lost one life");
    assert_eq!(
        wizard_count(&game),
        1,
        "the exile cast made no second token"
    );
    assert_eq!(
        game.players[1].life, 19,
        "the first Wizard saw the rebounded noncreature spell",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::TRANSPOSE)
    );
}
