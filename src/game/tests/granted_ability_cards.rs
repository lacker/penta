//! Two cards that hand an ability to a group of creatures.
//!
//! The Slumlord's is a keyword, so the only question is who is a Rat. The
//! Archangel's is a triggered ability, and exalted is defined so that several
//! instances each trigger separately -- so the grant has to be worth one
//! trigger per creature, not one for the board.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers each waiting decision by taking the last option, which for a
/// "you may create" is the branch that accepts.
fn drain_accepting(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn deadly(game: &Game, id: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    game.permanent_has_executable_keyword(permanent, KeywordAbility::Deathtouch)
}

/// A nontoken death makes a Rat, and the Rat it makes is deadly while the
/// Ogre that made it is not.
#[test]
fn the_slumlord_makes_rats_and_sharpens_them() {
    let mut game = ready();
    let slumlord = creature(10_000, cards::OGRE_SLUMLORD, PlayerId::One);
    let slumlord_id = slumlord.card.id;
    game.battlefield.push(slumlord);
    let bear = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    game.destroy_permanent(bear_id);
    game.check_state_based_actions();
    drain_accepting(&mut game);

    let rat = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Rat"], &[ManaColor::Black], 1, 1),
            )
        })
        .expect("the bear's death made one")
        .card
        .id;
    assert!(deadly(&game, rat), "Rats you control have deathtouch");
    assert!(
        !deadly(&game, slumlord_id),
        "and the Slumlord is an Ogre Rogue",
    );
}

/// The Rat it makes is a token, so it cannot itself feed the trigger.
#[test]
fn the_slumlord_ignores_its_own_rats_dying() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::OGRE_SLUMLORD, PlayerId::One));
    let rat = token_permanent(
        10_100,
        tokens::creature(&["Rat"], &[ManaColor::Black], 1, 1),
        PlayerId::One,
    );
    let rat_id = rat.card.id;
    game.battlefield.push(rat);

    game.destroy_permanent(rat_id);
    game.check_state_based_actions();
    drain_accepting(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Rat"], &[ManaColor::Black], 1, 1)
            ))
            .count(),
        0,
        "a token dying is not another nontoken creature",
    );
}

/// Attacks alone with the named creature and returns its power afterwards.
fn attack_alone_with(others: usize, attacker_is_the_angel: bool) -> Option<i16> {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::SUBLIME_ARCHANGEL, PlayerId::One));
    for index in 0..others {
        game.battlefield.push(creature(
            10_100 + u32::try_from(index).expect("a short list"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }

    let attacker = if attacker_is_the_angel {
        10_000
    } else {
        10_100
    };
    let attacker_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id.0 == attacker)
        .expect("it is there")
        .card
        .id;

    game.step = Step::DeclareAttackers;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: attacker_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("one attacker is legal");
    game.finish_declaring_attackers();
    drain_accepting(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("still there");
    game.power(permanent)
}

/// One exalted instance per creature, so the bonus scales with the board.
#[test]
fn the_archangel_grants_one_exalted_instance_each() {
    assert_eq!(
        attack_alone_with(0, true),
        Some(5),
        "the Angel's own exalted only",
    );
    assert_eq!(
        attack_alone_with(2, false),
        Some(5),
        "a 2/2 plus the Angel's instance and one for each of the two bears",
    );
    assert_eq!(
        attack_alone_with(3, false),
        Some(6),
        "and it scales with the board rather than firing once for it",
    );
}
