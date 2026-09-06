//! Riders on "whenever this creature deals combat damage to a player". The
//! event is batched over the whole damage step, so what needs covering is
//! that a single creature's own predicate still selects it out of the batch,
//! that the rider reads the damaged player rather than the ability's
//! controller, and that blocking the attacker stops the whole thing.

use super::*;

/// `attacker` swinging at player two, blocked by `blockers` Bears, with
/// player two also holding two cards and a spare Bears at home.
fn swing(attacker: CardDefinitionId, blockers: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, holder) in [PlayerId::One, PlayerId::Two].into_iter().enumerate() {
        game.players[holder.index()].hand.clear();
        for offset in 0..2 {
            let filler = card(
                63_200 + u32::try_from(index * 4 + offset).expect("a small fixture"),
                cards::MOUNTAIN,
                holder,
            );
            game.players[holder.index()].hand.push(filler);
        }
    }
    let mut threat = creature(63_000, attacker, PlayerId::One);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let threat_id = threat.card.id;
    game.battlefield.push(threat);
    let mut bystander = creature(63_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    bystander.entered_controller_turn = 0;
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);
    for index in 0..blockers {
        let mut blocker = creature(
            63_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        );
        blocker.entered_controller_turn = 0;
        blocker.blocking = vec![threat_id];
        game.battlefield.push(blocker);
    }
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    // The batched event is captured as the damage is dealt; a round of
    // priority is what turns the capture into a target choice and then a
    // trigger on the stack.
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() || !game.stack.is_empty() {
            break;
        }
        if game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        game.apply(holder, Action::PassPriority)
            .expect("passing priority is always legal");
    }
    (game, bystander_id)
}

fn hands(game: &Game) -> (usize, usize) {
    (game.players[0].hand.len(), game.players[1].hand.len())
}

/// The labels the pending decision offers, whoever it belongs to.
fn offered_labels(game: &Game, player: PlayerId) -> Vec<String> {
    game.observe(player)
        .decision
        .map(|decision| {
            decision
                .options
                .iter()
                .map(|option| option.label.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn the_damaged_player_is_the_one_who_discards() {
    let (mut game, _) = swing(cards::HEADHUNTER, 0);
    assert_eq!(hands(&game), (2, 2), "both players start holding two");
    pass_until_decision(&mut game);
    choose_all_offered(&mut game, PlayerId::Two);

    assert_eq!(
        hands(&game),
        (2, 1),
        "the player who took the damage discarded, not the attacker"
    );
}

#[test]
fn a_blocked_attacker_never_triggers() {
    let (mut game, _) = swing(cards::HEADHUNTER, 1);
    pass_until_decision(&mut game);

    assert!(
        game.pending_decisions.is_empty() && game.stack.is_empty(),
        "nothing was put on the stack to discard to"
    );
    assert_eq!(hands(&game), (2, 2), "and both hands are intact");
}

#[test]
fn the_ping_only_offers_the_damaged_player_s_creatures() {
    let (game, _) = swing(cards::SKIRK_COMMANDO, 0);
    let labels = offered_labels(&game, PlayerId::One);

    assert!(
        labels.iter().any(|label| label.contains("Grizzly Bears")),
        "the defender's Bears may be pinged, but the offer was {labels:?}"
    );
    assert!(
        !labels.iter().any(|label| label.contains("Skirk Commando")),
        "and the Commando cannot point the ping at itself"
    );
}

#[test]
fn taking_the_ping_kills_the_creature_it_names() {
    let (mut game, bystander) = swing(cards::SKIRK_COMMANDO, 0);
    choose_decision_by_label(&mut game, PlayerId::One, "Grizzly Bears");
    pass_until_decision(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != bystander),
        "two damage finished the Bears off"
    );
}
