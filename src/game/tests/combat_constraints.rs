//! Combat triggers that change what a creature is afterwards.
//!
//! Three cards whose audit lines all named a missing combat constraint, and
//! none of which needed one: a blocker that stops being a Wall, a Dwarf that
//! grows against Orcs, and a Ram that takes the Wall down with it.

use super::*;
use crate::ImplementationStatus;

/// `attacker` attacking player two, blocked by `blocker`, with the blockers
/// declared through the real procedure so the triggers fire.
fn blocked_attack(
    attacker: CardDefinitionId,
    blocker: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacking.card.id;
    game.battlefield.push(attacking);
    let blocking = creature(10_001, blocker, PlayerId::Two);
    let blocker_id = blocking.card.id;
    game.battlefield.push(blocking);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.declare_blocker(blocker_id, attacker_id);
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, attacker_id, blocker_id)
}

fn permanent(game: &Game, id: GameObjectId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
}

#[test]
fn the_land_wurm_stops_being_a_wall_once_it_blocks() {
    let (game, _attacker, wurm) = blocked_attack(cards::SEDGE_TROLL, cards::ELDER_LAND_WURM);

    let wurm = permanent(&game, wurm).expect("still there");
    assert!(
        !game.permanent_has_executable_keyword(wurm, KeywordAbility::Defender),
        "blocking took defender off it"
    );
    assert!(
        game.permanent_has_executable_keyword(wurm, KeywordAbility::Trample),
        "and left the rest of its printed keywords alone"
    );
}

/// The control: a Wurm that has not blocked still cannot attack.
#[test]
fn the_land_wurm_keeps_defender_until_it_blocks() {
    let mut game = ready_game();
    let wurm = creature(10_000, cards::ELDER_LAND_WURM, PlayerId::One);
    let wurm_id = wurm.card.id;
    game.battlefield.push(wurm);

    let wurm = permanent(&game, wurm_id).expect("still there");
    assert!(game.permanent_has_executable_keyword(wurm, KeywordAbility::Defender));
}

#[test]
fn the_dwarven_soldier_grows_against_an_orc() {
    let (game, _attacker, soldier) =
        blocked_attack(cards::ORCISH_ARTILLERY, cards::DWARVEN_SOLDIER);

    let soldier = permanent(&game, soldier).expect("still there");
    assert_eq!(
        game.toughness(soldier),
        Some(3),
        "a 2/1 that blocked an Orc is a 2/3"
    );
}

/// The control, and the point of the subtype in the trigger: anything that is
/// not an Orc leaves it alone.
#[test]
fn the_dwarven_soldier_ignores_everything_else() {
    let (game, _attacker, soldier) = blocked_attack(cards::SEDGE_TROLL, cards::DWARVEN_SOLDIER);

    let soldier = permanent(&game, soldier).expect("still there");
    assert_eq!(game.toughness(soldier), Some(1), "a Troll is not an Orc");
}

#[test]
fn the_battering_ram_installs_a_delayed_trigger_for_the_wall_that_blocked_it() {
    let (game, _ram, wall) = blocked_attack(cards::BATTERING_RAM, cards::WALL_OF_STONE);

    assert!(permanent(&game, wall).is_some(), "the Wall is still there");
    assert_eq!(
        game.installed_triggers.len(),
        1,
        "the blocking trigger created a one-shot end-of-combat trigger"
    );
}

/// The control: an ordinary blocker is not a Wall, so no trigger is installed.
#[test]
fn the_battering_ram_leaves_a_non_wall_blocker_alone() {
    let (game, _ram, blocker) = blocked_attack(cards::BATTERING_RAM, cards::SEDGE_TROLL);

    let blocker = permanent(&game, blocker).expect("still there");
    assert_eq!(blocker.card.definition, cards::SEDGE_TROLL);
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn the_battering_rams_delayed_destruction_uses_the_stack_and_can_be_countered() {
    let (mut game, _ram, wall) = blocked_attack(cards::BATTERING_RAM, cards::WALL_OF_STONE);

    game.step = Step::EndOfCombat;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::EndOfCombat,
        player: game.active_player,
    });
    game.finish_rules_procedure();

    assert!(
        game.installed_triggers.is_empty(),
        "the one-shot listener fired"
    );
    assert_eq!(game.stack.len(), 1, "the delayed ability is on the stack");
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert!(game.can_be_countered(&game.stack[0]));
    assert!(
        permanent(&game, wall).is_some(),
        "nothing happened at the boundary"
    );

    let delayed = game.stack[0].id;
    game.counter_spell(delayed);
    assert!(game.stack.is_empty());
    assert!(
        permanent(&game, wall).is_some(),
        "countering the delayed trigger saves the Wall"
    );
}

/// "Has base power 0" is half of a base-setting effect: the toughness under
/// it is untouched, which is the whole reason it is its own operation.
#[test]
fn the_singing_tree_zeroes_power_and_leaves_toughness_alone() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let tree = creature(10_001, cards::SINGING_TREE, PlayerId::Two);
    let tree_id = tree.card.id;
    game.battlefield.push(tree);
    // Past the declaration, so priority offers ordinary abilities rather
    // than only blocks.
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::Two;

    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == tree_id),
        )
        .expect("the Tree can point at an attacker");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let angel = permanent(&game, attacker_id).expect("still there");
    assert_eq!(game.power(angel), Some(0), "a 4/4 Angel with no power");
    assert_eq!(
        game.toughness(angel),
        Some(4),
        "and the toughness the setter never named"
    );
}

/// The control: an attacking creature is what the ability can point at, so a
/// creature sitting at home is not offered.
#[test]
fn the_singing_tree_only_points_at_attackers() {
    let mut game = ready_game();
    game.turns_started[PlayerId::Two.index()] = 5;
    let idle = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    game.battlefield.push(idle);
    let tree = creature(10_001, cards::SINGING_TREE, PlayerId::Two);
    let tree_id = tree.card.id;
    game.battlefield.push(tree);
    game.priority = PlayerId::Two;

    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == tree_id)
        }),
        "nothing is attacking, so there is nothing to sing at"
    );
}

#[test]
fn the_authored_identities_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::ELDER_LAND_WURM,
        cards::DWARVEN_SOLDIER,
        cards::BATTERING_RAM,
        cards::SINGING_TREE,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
