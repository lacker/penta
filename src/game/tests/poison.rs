//! Poison counters.
//!
//! A second way to lose that has nothing to do with life. What these check is
//! that the counters accumulate on the player rather than the creature, that
//! ten of them end the game with their own reason, and that a seat short of
//! ten is in no danger at all.

use super::*;

fn poison_board(source: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let attacker = creature(10_000, source, PlayerId::One);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    (game, attacker_id)
}

fn poison_counters(game: &Game, player: PlayerId) -> u16 {
    game.observe(player).poison_counters[player.index()]
}

#[test]
fn damage_to_a_player_hands_out_the_printed_number_of_counters() {
    for (source, expected) in [(cards::PIT_SCORPION, 1), (cards::MARSH_VIPER, 2)] {
        let (mut game, attacker_id) = poison_board(source);
        game.damage_target_from(Some(attacker_id), Some(Target::Player(PlayerId::Two)), 1);
        drain_pending(&mut game);

        assert_eq!(
            poison_counters(&game, PlayerId::Two),
            expected,
            "the counters go on the damaged player"
        );
        assert_eq!(
            poison_counters(&game, PlayerId::One),
            0,
            "and only on that one"
        );
    }
}

/// One counter per damage event, not per point: a 1/1 dealing one damage and
/// a creature dealing five both hand over exactly what the card prints.
#[test]
fn the_count_does_not_scale_with_the_damage() {
    let (mut game, attacker_id) = poison_board(cards::PIT_SCORPION);
    game.damage_target_from(Some(attacker_id), Some(Target::Player(PlayerId::Two)), 5);
    drain_pending(&mut game);

    assert_eq!(poison_counters(&game, PlayerId::Two), 1);
}

#[test]
fn eight_counters_is_not_a_loss_and_ten_is() {
    let (mut game, attacker_id) = poison_board(cards::MARSH_VIPER);
    for _ in 0..4 {
        game.damage_target_from(Some(attacker_id), Some(Target::Player(PlayerId::Two)), 1);
        drain_pending(&mut game);
    }
    assert_eq!(poison_counters(&game, PlayerId::Two), 8);
    assert!(game.result().is_none(), "eight counters is survivable");
    assert!(
        game.players[PlayerId::Two.index()].life > 0,
        "and none of this touched their life total"
    );

    game.damage_target_from(Some(attacker_id), Some(Target::Player(PlayerId::Two)), 1);
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentPoisoned,
        }),
        "ten counters is a loss, and it reports why"
    );
}

/// Serpent Generator prints the trigger on the token rather than on itself,
/// so the artifact never poisons anyone; its Snakes do.
#[test]
fn serpent_generators_snakes_carry_the_trigger() {
    let mut game = ready_game();
    let generator = creature(10_000, cards::SERPENT_GENERATOR, PlayerId::One);
    let generator_id = generator.card.id;
    game.battlefield.push(generator);
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == generator_id)
        })
        .expect("the Generator can make a Snake");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let snake_id = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, poisonous_snake_token()))
        .expect("a Snake token entered")
        .card
        .id;

    game.damage_target_from(Some(generator_id), Some(Target::Player(PlayerId::Two)), 1);
    drain_pending(&mut game);
    assert_eq!(
        poison_counters(&game, PlayerId::Two),
        0,
        "the Generator itself is not poisonous"
    );

    game.damage_target_from(Some(snake_id), Some(Target::Player(PlayerId::Two)), 1);
    drain_pending(&mut game);
    assert_eq!(
        poison_counters(&game, PlayerId::Two),
        1,
        "but the token it made is"
    );
}
