use super::tests::{creature, ready_game};
use super::*;
use crate::CardInstanceId;
use crate::card::cards;

fn planeswalker(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    loyalty: u16,
) -> Permanent {
    let mut permanent = creature(id, definition, controller);
    permanent.set_counters(CounterKind::Loyalty, loyalty);
    permanent
}

#[test]
fn attackers_choose_between_the_player_and_each_opposing_planeswalker() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let attacker_id = attacker.card.id;
    let opposing_walker = planeswalker(10_001, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let opposing_walker_id = opposing_walker.card.id;
    let own_walker = planeswalker(10_002, cards::LILIANA_OF_THE_VEIL, PlayerId::One, 3);
    let own_walker_id = own_walker.card.id;
    game.battlefield = vec![attacker, opposing_walker, own_walker];

    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: attacker_id,
        defender: AttackDefender::Player(PlayerId::Two),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: attacker_id,
        defender: AttackDefender::Planeswalker(opposing_walker_id),
    }));
    assert!(!actions.contains(&Action::DeclareAttacker {
        attacker: attacker_id,
        defender: AttackDefender::Planeswalker(own_walker_id),
    }));

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: attacker_id,
            defender: AttackDefender::Planeswalker(opposing_walker_id),
        },
    )
    .unwrap();
    assert_eq!(
        game.battlefield[0].attack_defender,
        Some(AttackDefender::Planeswalker(opposing_walker_id))
    );
    let observed = game
        .observe(PlayerId::One)
        .battlefield
        .into_iter()
        .find(|permanent| permanent.id == attacker_id)
        .expect("the attacking creature is observable");
    assert_eq!(
        observed.attack_defender,
        Some(AttackDefender::Planeswalker(opposing_walker_id))
    );
    assert!(!observed.blocked_this_combat);
}

#[test]
fn unblocked_combat_damage_hits_the_chosen_planeswalker_not_its_controller() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let walker = planeswalker(10_001, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let walker_id = walker.card.id;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    game.battlefield = vec![attacker, walker];
    let life_before = game.players[PlayerId::Two.index()].life;

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, life_before);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == walker_id)
            .map(|permanent| permanent.counters(CounterKind::Loyalty)),
        Some(3)
    );
}

#[test]
fn a_planeswalker_leaving_combat_does_not_redirect_damage_to_its_controller() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let walker = planeswalker(10_001, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let walker_id = walker.card.id;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    game.battlefield = vec![attacker, walker];
    game.destroy_permanent(walker_id);
    let life_before = game.players[PlayerId::Two.index()].life;

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, life_before);
}

#[test]
fn a_nontrampling_attacker_stays_blocked_after_its_blocker_leaves() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    let blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    game.declare_blocker(blocker_id, attacker_id);
    game.finish_declaring_blockers();
    game.destroy_permanent(blocker_id);
    let life_before = game.players[PlayerId::Two.index()].life;

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, life_before);
    assert!(game.battlefield[0].blocked);
}

#[test]
fn trample_over_a_planeswalker_never_spills_to_its_controller() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    let walker = planeswalker(10_002, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 10);
    let walker_id = walker.card.id;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    attacker.blocked = true;
    game.battlefield = vec![attacker, blocker, walker];
    game.begin_combat_damage_assignment();
    let life_before = game.players[PlayerId::Two.index()].life;
    let assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: vec![
            CombatDamageAssignment {
                recipient: Target::Permanent(blocker_id),
                amount: 2,
            },
            CombatDamageAssignment {
                recipient: Target::Permanent(walker_id),
                amount: 4,
            },
        ],
    };

    assert!(game.legal_actions(PlayerId::One).contains(&assignment));
    game.apply(PlayerId::One, assignment).unwrap();

    assert_eq!(game.players[PlayerId::Two.index()].life, life_before);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == walker_id)
            .map(|permanent| permanent.counters(CounterKind::Loyalty)),
        Some(6)
    );
}

#[test]
fn trample_assigns_all_damage_to_the_last_blocker_when_its_planeswalker_leaves() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    attacker.temporary_keywords.push(KeywordAbility::Lifelink);
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    let walker = planeswalker(10_002, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let walker_id = walker.card.id;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    game.battlefield = vec![attacker, blocker, walker];
    game.destroy_permanent(walker_id);
    let defending_life = game.players[PlayerId::Two.index()].life;
    let attacking_life = game.players[PlayerId::One.index()].life;

    game.start_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, defending_life);
    assert_eq!(game.players[PlayerId::One.index()].life, attacking_life + 6);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id)
    );
}

#[test]
fn trample_can_divide_freely_among_blockers_after_its_planeswalker_leaves() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut first = creature(10_001, cards::ATOG, PlayerId::Two);
    first.blocking = vec![attacker_id];
    let first_id = first.card.id;
    let mut second = creature(10_002, cards::ATOG, PlayerId::Two);
    second.blocking = vec![attacker_id];
    let second_id = second.card.id;
    let walker = planeswalker(10_003, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let walker_id = walker.card.id;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    attacker.blocked = true;
    game.battlefield = vec![attacker, first, second, walker];
    game.destroy_permanent(walker_id);
    game.begin_combat_damage_assignment();

    let divided = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: vec![
            CombatDamageAssignment {
                recipient: Target::Permanent(first_id),
                amount: 1,
            },
            CombatDamageAssignment {
                recipient: Target::Permanent(second_id),
                amount: 5,
            },
        ],
    };
    assert!(
        game.legal_actions(PlayerId::One).contains(&divided),
        "without a defender recipient, Foundations permits any split among the blockers",
    );
}

#[test]
fn player_damage_behaviors_do_not_trigger_when_a_planeswalker_is_hit() {
    let mut game = ready_game();
    let mut specter = creature(10_000, cards::HYPNOTIC_SPECTER, PlayerId::One);
    specter.attacking = true;
    let walker = planeswalker(10_001, cards::VRASKA_THE_UNSEEN, PlayerId::Two, 5);
    let walker_id = walker.card.id;
    specter.attack_defender = Some(AttackDefender::Planeswalker(walker_id));
    game.battlefield = vec![specter, walker];
    game.players[PlayerId::Two.index()].hand.push(CardInstance {
        id: CardInstanceId(10_002),
        definition: cards::MOUNTAIN,
        owner: PlayerId::Two,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(10_002)]),
        characteristics: CharacteristicSource::Card(cards::MOUNTAIN),
        counters: crate::game::counters::Counters::new(),
    });

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
}
