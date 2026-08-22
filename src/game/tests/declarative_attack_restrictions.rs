//! Predicate-driven attack prohibitions and declaration costs.

use super::*;
use crate::ImplementationStatus;

fn ready_attackers(definitions: &[CardDefinitionId]) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.turns_started[PlayerId::One.index()] = 1;
    let ids = definitions
        .iter()
        .enumerate()
        .map(|(index, definition)| {
            let permanent = creature(
                10_000 + u32::try_from(index).expect("a small fixture"),
                *definition,
                PlayerId::One,
            );
            let id = permanent.card.id;
            game.battlefield.push(permanent);
            id
        })
        .collect();
    (game, ids)
}

fn add_planeswalker(game: &mut Game) -> GameObjectId {
    let mut planeswalker = creature(10_500, cards::VRASKA_THE_UNSEEN, PlayerId::Two);
    planeswalker.set_counters(CounterKind::Loyalty, 5);
    let id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    id
}

fn offers_attack(game: &Game, attacker: GameObjectId, defender: AttackDefender) -> bool {
    game.legal_actions(PlayerId::One)
        .contains(&Action::DeclareAttacker { attacker, defender })
}

#[test]
fn moat_uses_a_live_keyword_predicate_for_every_defender() {
    let (mut game, attackers) = ready_attackers(&[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL]);
    let walker = add_planeswalker(&mut game);
    game.battlefield
        .push(creature(10_501, cards::MOAT, PlayerId::Two));

    for defender in [
        AttackDefender::Player(PlayerId::Two),
        AttackDefender::Planeswalker(walker),
    ] {
        assert!(!offers_attack(&game, attackers[0], defender));
        assert!(offers_attack(&game, attackers[1], defender));
    }

    game.battlefield
        .retain(|permanent| permanent.card.definition != cards::MOAT);
    assert!(offers_attack(
        &game,
        attackers[0],
        AttackDefender::Player(PlayerId::Two)
    ));
}

#[test]
fn ensnaring_bridge_reads_its_controllers_current_hand_size() {
    let (mut game, attackers) = ready_attackers(&[cards::SAVANNAH_LIONS, cards::LLANOWAR_ELVES]);
    let walker = add_planeswalker(&mut game);
    game.battlefield
        .push(creature(10_501, cards::ENSNARING_BRIDGE, PlayerId::Two));
    game.players[PlayerId::Two.index()]
        .hand
        .push(card(10_600, cards::PLAINS, PlayerId::Two));

    for defender in [
        AttackDefender::Player(PlayerId::Two),
        AttackDefender::Planeswalker(walker),
    ] {
        assert!(!offers_attack(&game, attackers[0], defender));
        assert!(offers_attack(&game, attackers[1], defender));
    }

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attackers[1])
        .unwrap()
        .set_counters(CounterKind::PlusOnePlusOne, 1);
    assert!(
        !offers_attack(&game, attackers[1], AttackDefender::Player(PlayerId::Two)),
        "the predicate reads current power while attackers are chosen"
    );

    game.players[PlayerId::Two.index()]
        .hand
        .push(card(10_601, cards::MOUNTAIN, PlayerId::Two));
    assert!(offers_attack(
        &game,
        attackers[0],
        AttackDefender::Player(PlayerId::Two)
    ));
}

#[test]
fn elephant_grass_prohibits_black_and_taxes_nonblack_only_when_attacking_you() {
    let (mut game, attackers) = ready_attackers(&[cards::JUZAM_DJINN, cards::SAVANNAH_LIONS]);
    let walker = add_planeswalker(&mut game);
    game.battlefield
        .push(creature(10_501, cards::ELEPHANT_GRASS, PlayerId::Two));
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    assert!(!offers_attack(
        &game,
        attackers[0],
        AttackDefender::Player(PlayerId::Two)
    ));
    assert!(offers_attack(
        &game,
        attackers[0],
        AttackDefender::Planeswalker(walker)
    ));
    assert!(offers_attack(
        &game,
        attackers[1],
        AttackDefender::Player(PlayerId::Two)
    ));
    assert!(offers_attack(
        &game,
        attackers[1],
        AttackDefender::Planeswalker(walker)
    ));

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: attackers[1],
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 0);
}

#[test]
fn elephant_grass_costs_add_and_cannot_use_the_attacker_as_a_mana_source() {
    let (mut game, attackers) = ready_attackers(&[cards::SAVANNAH_LIONS, cards::SAVANNAH_LIONS]);
    game.battlefield
        .push(creature(10_501, cards::ELEPHANT_GRASS, PlayerId::Two));
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let defender = AttackDefender::Player(PlayerId::Two);
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: attackers[0],
            defender,
        },
    )
    .unwrap();
    assert!(
        !offers_attack(&game, attackers[1], defender),
        "the two per creature is checked against the whole declaration"
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: attackers[1],
            defender,
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 0);

    let (mut game, elves) = ready_attackers(&[cards::LLANOWAR_ELVES]);
    game.battlefield
        .push(creature(10_501, cards::ELEPHANT_GRASS, PlayerId::Two));
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(
        !offers_attack(&game, elves[0], defender),
        "a creature cannot tap for mana and tap to attack"
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    assert!(offers_attack(&game, elves[0], defender));
    game.battlefield
        .push(creature(10_502, cards::ELEPHANT_GRASS, PlayerId::Two));
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    assert!(
        !offers_attack(&game, elves[0], defender),
        "two independent Grass taxes add to four"
    );
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    assert!(offers_attack(&game, elves[0], defender));
}

#[test]
fn an_attack_requirement_never_forces_an_elephant_grass_payment() {
    let (mut game, _) = ready_attackers(&[cards::JUGGERNAUT]);
    game.battlefield
        .push(creature(10_501, cards::ELEPHANT_GRASS, PlayerId::Two));
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers),
        "attacks each combat if able does not require paying an optional cost"
    );
}

#[test]
fn elephant_grass_cumulative_upkeep_counts_age_and_sacrifices_when_unpaid() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let grass = creature(10_000, cards::ELEPHANT_GRASS, PlayerId::One);
    let grass_id = grass.card.id;
    game.battlefield.push(grass);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.handle_upkeep_triggers();
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the first upkeep costs one");
    let pay = decision
        .options
        .iter()
        .find(|option| option.label == "Pay the cost")
        .expect("one mana is enough")
        .id;
    game.choose_decision(PlayerId::One, decision.id, &[pay]);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == grass_id)
            .unwrap()
            .counters(CounterKind::Age),
        1
    );

    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != grass_id),
        "the second age counter makes the unpaid upkeep cost two"
    );
}

#[test]
fn callers_myriad_triggers_but_creates_nothing_in_a_two_player_game() {
    let (mut game, callers) = ready_attackers(&[cards::CALLER_OF_THE_PACK]);
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: callers[0],
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    assert!(game.events.iter().any(
        |event| matches!(event, GameEvent::AbilityTriggered { source, .. } if *source == callers[0])
    ));
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::CALLER_OF_THE_PACK)
            .count(),
        1
    );
}

#[test]
fn the_attack_restriction_cards_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::ISLAND_SANCTUARY,
        cards::MOAT,
        cards::ELEPHANT_GRASS,
        cards::ENSNARING_BRIDGE,
        cards::CALLER_OF_THE_PACK,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name
        );
    }
}
