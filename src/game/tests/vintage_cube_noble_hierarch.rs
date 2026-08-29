//! Noble Hierarch: one mana for three colours, and a 0/1 that is worth
//! attacking with on a turn nothing else does.

use super::*;

/// Her on the battlefield since last turn, with `others` beside her.
fn staged(others: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let hierarch = game
        .put_onto_battlefield(PlayerId::One, cards::NOBLE_HIERARCH)
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
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect();
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

/// She taps for any of her three colours, and only those.
#[test]
fn she_taps_for_green_white_or_blue() {
    for (color, expected) in [
        (ManaColor::Green, true),
        (ManaColor::White, true),
        (ManaColor::Blue, true),
        (ManaColor::Red, false),
        (ManaColor::Black, false),
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

/// Tapping her adds one of the colour chosen.
#[test]
fn tapping_her_makes_one_mana() {
    let (mut game, hierarch, _) = staged(&[]);

    let add_white = Action::ActivateManaAbility {
        source: hierarch,
        ability: mana_ability_for(&game, hierarch, ManaColor::White),
        color: ManaColor::White,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, add_white).expect("it taps");

    assert_eq!(game.players[0].mana_pool.white, 1);
    assert_eq!(game.players[0].mana_pool.green, 0, "one mana, one colour");
}

/// Exalted: a lone attacker gets +1/+1, even when the lone attacker is her.
#[test]
fn a_lone_attacker_gets_bigger() {
    let (mut game, hierarch, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];

    attack_with(&mut game, &[bears]);

    assert_eq!(stats(&game, bears), (Some(3), Some(3)), "+1/+1");
    assert_eq!(
        stats(&game, hierarch),
        (Some(0), Some(1)),
        "she stayed home"
    );
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

/// She counts herself: attacking alone, the 0/1 swings as a 1/2.
#[test]
fn she_exalts_herself() {
    let (mut game, hierarch, _) = staged(&[]);

    attack_with(&mut game, &[hierarch]);

    assert_eq!(stats(&game, hierarch), (Some(1), Some(2)));
}

/// The bonus is until end of turn, not a counter.
#[test]
fn the_bonus_wears_off() {
    let (mut game, hierarch, _) = staged(&[]);
    attack_with(&mut game, &[hierarch]);
    assert_eq!(stats(&game, hierarch), (Some(1), Some(2)));

    for _ in 0..40 {
        if game.turn > 9 {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }

    assert_eq!(stats(&game, hierarch), (Some(0), Some(1)), "back to a 0/1");
}
