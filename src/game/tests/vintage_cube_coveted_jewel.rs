//! Coveted Jewel: three cards and three mana, for as long as you can keep
//! anything from getting through.

use super::*;

/// The Jewel on the battlefield under player One, with player Two holding
/// `attackers` ready to come at them.
fn staged(attackers: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    let jewel = game
        .put_onto_battlefield(PlayerId::One, cards::COVETED_JEWEL)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut ids = Vec::new();
    for (index, definition) in attackers.iter().enumerate() {
        let creature = creature(
            280_000 + u32::try_from(index).expect("few creatures"),
            *definition,
            PlayerId::Two,
        );
        ids.push(creature.card.id);
        game.battlefield.push(creature);
    }
    drain_pending(&mut game);
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started = [5, 5];
    game.turn = 10;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    (game, jewel, ids)
}

/// Player Two attacks player One with `attackers`, `blocked` of them stopped
/// by a creature player One controls, and runs blocking out to its triggers.
fn attack(game: &mut Game, attackers: &[GameObjectId], blocked: &[(GameObjectId, GameObjectId)]) {
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if attackers.contains(&permanent.card.id) {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::One));
        }
        if let Some((_, attacker)) = blocked
            .iter()
            .find(|(blocker, _)| *blocker == permanent.card.id)
        {
            permanent.blocking = vec![*attacker];
        }
    }
    game.finish_declaring_blockers();
    drain_pending(game);
}

fn controller_of(game: &Game, id: GameObjectId) -> Option<PlayerId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map(|permanent| permanent.controller)
}

/// It draws three as it arrives.
#[test]
fn it_draws_three_when_it_enters() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let before = game.players[0].library.len();

    game.put_onto_battlefield(PlayerId::One, cards::COVETED_JEWEL)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 3);
    assert_eq!(game.players[0].library.len(), before - 3);
}

/// Tapping it adds three mana of the one color chosen.
#[test]
fn it_taps_for_three_of_one_color() {
    let (mut game, jewel, _) = staged(&[]);
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;

    let add_red = Action::ActivateManaAbility {
        source: jewel,
        ability: mana_ability_for(&game, jewel, ManaColor::Red),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&add_red));
    game.apply(PlayerId::One, add_red).expect("it activates");

    assert_eq!(
        game.players[0].mana_pool.red, 3,
        "three, and all of them red"
    );
    assert_eq!(
        game.players[0].mana_pool.green, 0,
        "one color, not a spread"
    );
}

/// Two creatures through at once is one trigger: three cards, not six, and
/// one artifact changing hands.
#[test]
fn one_or_more_getting_through_is_a_single_trigger() {
    let (mut game, jewel, attackers) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    game.tap_permanent(jewel);

    attack(&mut game, &attackers, &[]);

    assert_eq!(
        game.players[1].hand.len(),
        3,
        "three cards for the whole crew, not three apiece",
    );
    assert_eq!(controller_of(&game, jewel), Some(PlayerId::Two));
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == jewel)
            .expect("it is still on the battlefield")
            .tapped,
        "and it arrives untapped, ready for their own turn",
    );
}

/// Block them all and it stays where it is.
#[test]
fn blocking_everything_keeps_it() {
    let (mut game, jewel, attackers) = staged(&[cards::GRIZZLY_BEARS]);
    let wall = creature(280_500, cards::SERRA_ANGEL, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    drain_pending(&mut game);

    attack(&mut game, &attackers, &[(wall_id, attackers[0])]);

    assert!(game.players[1].hand.is_empty(), "nothing got through");
    assert_eq!(controller_of(&game, jewel), Some(PlayerId::One));
}

/// One blocked and one through is still a trigger, and still three cards.
#[test]
fn one_through_out_of_two_is_still_three_cards() {
    let (mut game, jewel, attackers) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let wall = creature(280_600, cards::SERRA_ANGEL, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    drain_pending(&mut game);

    attack(&mut game, &attackers, &[(wall_id, attackers[0])]);

    assert_eq!(game.players[1].hand.len(), 3);
    assert_eq!(controller_of(&game, jewel), Some(PlayerId::Two));
}

/// The clause reads "attack you": the Jewel's own controller attacking with
/// unblocked creatures does not hand it over.
#[test]
fn your_own_unblocked_attackers_take_nothing() {
    let (mut game, jewel, _) = staged(&[]);
    let bears = creature(280_700, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == bears_id {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert!(game.players[1].hand.is_empty());
    assert_eq!(controller_of(&game, jewel), Some(PlayerId::One));
}

/// "A creature attacking a planeswalker you control won't cause Coveted
/// Jewel's last ability to trigger." The clause reads "attack you".
#[test]
fn an_attack_on_your_planeswalker_takes_nothing() {
    let (mut game, jewel, attackers) = staged(&[cards::GRIZZLY_BEARS]);
    let walker = game
        .put_onto_battlefield(PlayerId::One, cards::JACE_THE_MIND_SCULPTOR)
        .expect("cataloged");
    drain_pending(&mut game);
    let hand = game.players[1].hand.len();

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attackers[0])
    {
        permanent.attacking = true;
        permanent.attack_defender = Some(AttackDefender::Planeswalker(walker));
    }
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(
        controller_of(&game, jewel),
        Some(PlayerId::One),
        "the Jewel stays: nobody attacked its controller",
    );
    assert_eq!(
        game.players[1].hand.len(),
        hand,
        "and they drew nothing for it",
    );
}

/// "Untap it": the Jewel changes hands ready to be used, however tapped out
/// it was when they came through.
#[test]
fn what_changes_hands_arrives_untapped() {
    let (mut game, jewel, attackers) = staged(&[cards::GRIZZLY_BEARS]);
    game.priority = PlayerId::One;
    let mana = Action::ActivateManaAbility {
        source: jewel,
        ability: mana_ability_for(&game, jewel, ManaColor::Blue),
        color: ManaColor::Blue,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, mana).expect("it taps for three");
    assert_eq!(game.players[0].mana_pool.blue, 3, "three blue");
    game.priority = PlayerId::Two;

    attack(&mut game, &attackers, &[]);

    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == jewel)
        .expect("the Jewel is still on the battlefield");
    assert_eq!(stolen.controller, PlayerId::Two, "under new management");
    assert!(!stolen.tapped, "and untapped, whatever it was before");
}
