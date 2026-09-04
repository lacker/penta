//! Shared behavior for bushido, flanking, and bloodthirst.

use super::*;

fn blocked_by(
    attacker: CardDefinitionId,
    blocker: CardDefinitionId,
    support: Option<CardDefinitionId>,
) -> (Game, [GameObjectId; 2]) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacking.card.id;
    game.battlefield.push(attacking);

    if let Some(support) = support {
        game.battlefield
            .push(creature(10_002, support, PlayerId::One));
    }

    let mut defending = creature(10_001, blocker, PlayerId::Two);
    defending.blocking = vec![attacker_id];
    let blocker_id = defending.card.id;
    game.battlefield.push(defending);
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, [attacker_id, blocker_id])
}

fn stats(game: &Game, object: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .expect("the creature remains on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn keyword_mechanics_bushido_triggers_when_blocking_or_blocked() {
    let (attacking, [ronin, _]) = blocked_by(cards::BATTLE_MAD_RONIN, cards::GRIZZLY_BEARS, None);
    assert_eq!(stats(&attacking, ronin), (Some(3), Some(3)));

    let (blocking, [_, retainer]) = blocked_by(cards::GRIZZLY_BEARS, cards::DEVOTED_RETAINER, None);
    assert_eq!(stats(&blocking, retainer), (Some(2), Some(2)));
}

#[test]
fn keyword_mechanics_fumiko_counts_all_attacking_creatures_for_bushido_x() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    let mut fumiko = creature(10_010, cards::FUMIKO_THE_LOWBLOOD, PlayerId::One);
    fumiko.attacking = true;
    fumiko.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let fumiko_id = fumiko.card.id;
    let mut companion = creature(10_011, cards::GRIZZLY_BEARS, PlayerId::One);
    companion.attacking = true;
    companion.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let mut blocker = creature(10_012, cards::WALL_OF_STONE, PlayerId::Two);
    blocker.blocking = vec![fumiko_id];
    game.battlefield = vec![fumiko, companion, blocker];

    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(stats(&game, fumiko_id), (Some(5), Some(4)));
}

#[test]
fn keyword_mechanics_flanking_stacks_and_respects_flanking_blockers() {
    let (single, [_, blocker]) = blocked_by(cards::BENALISH_CAVALRY, cards::WALL_OF_STONE, None);
    assert_eq!(stats(&single, blocker), (Some(-1), Some(7)));

    let (doubled, [_, blocker]) = blocked_by(
        cards::BENALISH_CAVALRY,
        cards::WALL_OF_STONE,
        Some(cards::CAVALRY_MASTER),
    );
    assert_eq!(stats(&doubled, blocker), (Some(-2), Some(6)));

    let (immune, [_, blocker]) = blocked_by(cards::BENALISH_CAVALRY, cards::BENALISH_CAVALRY, None);
    assert_eq!(stats(&immune, blocker), (Some(2), Some(2)));
}

#[test]
fn keyword_mechanics_bloodthirst_uses_damage_and_granted_instances_stack() {
    let mut damaged = ready_game();
    damaged.deal_damage_simultaneously(vec![DamageAssignment {
        source: None,
        target: Some(Target::Player(PlayerId::Two)),
        amount: 1,
        combat: false,
    }]);
    let berserker = damaged
        .put_onto_battlefield(PlayerId::One, cards::STORMBLOOD_BERSERKER)
        .expect("the bloodthirst creature is in the catalog");
    drain_pending(&mut damaged);
    let permanent = damaged
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == berserker)
        .expect("the creature entered");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 2);

    let mut life_loss_only = ready_game();
    life_loss_only.players[PlayerId::Two.index()].life -= 1;
    let prowler = life_loss_only
        .put_onto_battlefield(PlayerId::One, cards::BLOODSCALE_PROWLER)
        .expect("the bloodthirst creature is in the catalog");
    drain_pending(&mut life_loss_only);
    let permanent = life_loss_only
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == prowler)
        .expect("the creature entered");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 0);

    let mut granted = ready_game();
    granted
        .put_onto_battlefield(PlayerId::One, cards::BLOODLORD_OF_VAASGOTH)
        .expect("Bloodlord is in the catalog");
    drain_pending(&mut granted);
    granted.deal_damage_simultaneously(vec![DamageAssignment {
        source: None,
        target: Some(Target::Player(PlayerId::Two)),
        amount: 1,
        combat: false,
    }]);
    let vampire = card(20_000, cards::SHADOW_ALLEY_DENIZEN, PlayerId::One);
    let vampire_id = vampire.id;
    granted.players[PlayerId::One.index()].hand.push(vampire);
    granted.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    granted
        .apply(
            PlayerId::One,
            cast_action(vampire_id, Vec::new(), Vec::new(), 0),
        )
        .expect("the Vampire can be cast");
    drain_pending(&mut granted);
    let vampire = granted
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SHADOW_ALLEY_DENIZEN)
        .expect("the Vampire resolved");
    assert_eq!(vampire.counters(CounterKind::PlusOnePlusOne), 3);
}

#[test]
fn keyword_mechanics_skarrgan_firebird_requires_opponent_damage() {
    let mut game = ready_game();
    let firebird = card(30_000, cards::SKARRGAN_FIREBIRD, PlayerId::One);
    let firebird_id = firebird.id;
    game.players[PlayerId::One.index()].graveyard.push(firebird);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);

    let can_return = |game: &Game| {
        game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if source == firebird_id)
        })
    };
    assert!(!can_return(&game));

    game.deal_damage_simultaneously(vec![DamageAssignment {
        source: None,
        target: Some(Target::Player(PlayerId::Two)),
        amount: 1,
        combat: false,
    }]);
    assert!(can_return(&game));
}
