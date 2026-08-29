//! Ignoble Hierarch: the same one-mana accelerant as Noble Hierarch, in the
//! other three colours.

use super::*;

/// Him on the battlefield since last turn, with `others` beside him.
fn staged(others: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let hierarch = game
        .put_onto_battlefield(PlayerId::One, cards::IGNOBLE_HIERARCH)
        .expect("cataloged");
    let mut ids = Vec::new();
    for definition in others {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, hierarch, ids)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn attack_with(game: &mut Game, attackers: &[GameObjectId]) {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    for attacker in attackers {
        game.declare_attacker(*attacker, AttackDefender::Player(PlayerId::Two));
    }
    game.finish_declaring_attackers();
    settle(game);
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

/// He taps for any of his three colours, and only those. The pair of
/// Hierarchs between them cover all five, and neither covers the other's.
#[test]
fn he_taps_for_black_red_or_green() {
    for (color, expected) in [
        (ManaColor::Black, true),
        (ManaColor::Red, true),
        (ManaColor::Green, true),
        (ManaColor::White, false),
        (ManaColor::Blue, false),
    ] {
        let (game, hierarch, _) = staged(&[]);
        let offered = game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, color: produced, .. }
                    if source == hierarch && produced == color
            )
        });
        assert_eq!(offered, expected, "{color:?}");
    }
}

/// Tapping him adds one mana of the colour chosen.
#[test]
fn tapping_him_makes_one_mana() {
    let (mut game, hierarch, _) = staged(&[]);

    let add_black = Action::ActivateManaAbility {
        source: hierarch,
        ability: mana_ability_for(&game, hierarch, ManaColor::Black),
        color: ManaColor::Black,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, add_black).expect("it taps");

    assert_eq!(game.players[0].mana_pool.black, 1);
    assert_eq!(game.players[0].mana_pool.red, 0, "one mana, one colour");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == hierarch)
            .expect("he is there")
            .tapped,
        "tapping him is what paid for it",
    );
}

/// Exalted: a lone attacker gets +1/+1 while he stays home.
#[test]
fn a_lone_attacker_gets_bigger() {
    let (mut game, hierarch, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];

    attack_with(&mut game, &[bears]);

    assert_eq!(stats(&game, bears), (Some(3), Some(3)), "+1/+1");
    assert_eq!(stats(&game, hierarch), (Some(0), Some(1)), "he stayed home");
}

/// Two attackers is not attacking alone, so nothing is exalted.
#[test]
fn two_attackers_get_nothing() {
    let (mut game, hierarch, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];

    attack_with(&mut game, &[bears, hierarch]);

    assert_eq!(stats(&game, bears), (Some(2), Some(2)));
    assert_eq!(stats(&game, hierarch), (Some(0), Some(1)));
}

/// He counts himself: attacking alone, the 0/1 swings as a 1/2.
#[test]
fn he_exalts_himself() {
    let (mut game, hierarch, _) = staged(&[]);

    attack_with(&mut game, &[hierarch]);

    assert_eq!(stats(&game, hierarch), (Some(1), Some(2)));
}
