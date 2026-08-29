//! Two old cards whose clauses read a creature's combat relation.
//!
//! Both audit lines claimed combat declaration constraints were missing.
//! What they actually needed was a blocks-or-becomes-blocked trigger with a
//! branch either way, and a static whose recipient is narrowed by what the
//! creature is doing right now.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn strikes_first(game: &Game, id: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    game.permanent_has_executable_keyword(permanent, KeywordAbility::FirstStrike)
}

/// Blocks an attacker with the Slug and answers its offer, returning the
/// first-strike state of the Slug and the attacker in that order.
fn slug_blocks(pay: bool) -> (bool, bool) {
    let mut game = ready();
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let slug = creature(10_100, cards::SPITTING_SLUG, PlayerId::One);
    let slug_id = slug.card.id;
    game.battlefield.push(slug);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: slug_id,
            attacker: attacker_id,
        },
    )
    .expect("the block is legal");
    game.finish_declaring_blockers();

    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let chosen = if pay {
                decision
                    .options
                    .iter()
                    .find(|option| option.label != "Decline")
                    .expect("paying is an option")
                    .id
            } else {
                decision
                    .options
                    .iter()
                    .find(|option| option.label == "Decline")
                    .expect("the payment is optional")
                    .id
            };
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![chosen],
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    (
        strikes_first(&game, slug_id),
        strikes_first(&game, attacker_id),
    )
}

/// Declining is not nothing -- the first strike goes to the other side.
#[test]
fn the_slug_hands_first_strike_to_whichever_side_paid() {
    assert_eq!(slug_blocks(true), (true, false), "paid, so it keeps it");
    assert_eq!(
        slug_blocks(false),
        (false, true),
        "declined, so the attacker gets it instead",
    );
}

/// Arcades gives +0/+2 to untapped creatures, and takes it back the moment
/// one taps.
#[test]
fn arcades_covers_untapped_creatures_only() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::ARCADES_SABBOTH, PlayerId::One));
    let bear = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert_eq!(stats(&game, bear_id), (Some(2), Some(4)), "a 2/2 plus 0/2");

    let index = game
        .battlefield
        .iter()
        .position(|permanent| permanent.card.id == bear_id)
        .expect("still there");
    game.battlefield[index].tapped = true;
    assert_eq!(
        stats(&game, bear_id),
        (Some(2), Some(2)),
        "tapping gives it back",
    );
}

/// And an attacking creature is excluded even while untapped, which is what
/// makes this a defensive anthem rather than a general one.
#[test]
fn arcades_drops_a_creature_that_attacks() {
    let mut game = ready();
    game.active_player = PlayerId::One;
    game.battlefield
        .push(creature(10_000, cards::ARCADES_SABBOTH, PlayerId::One));
    // Vigilance, so attacking does not tap it and the exclusion has to come
    // from the attacking half of the clause on its own.
    let angel = creature(10_100, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);

    assert_eq!(stats(&game, angel_id), (Some(4), Some(6)), "a 4/4 plus 0/2");

    game.step = Step::DeclareAttackers;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: angel_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("attacking is legal");
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == angel_id)
            .expect("still there")
            .tapped,
        "vigilance, so the untapped half of the clause still holds",
    );
    assert_eq!(
        stats(&game, angel_id),
        (Some(4), Some(4)),
        "attacking costs it the toughness anyway",
    );
    assert_eq!(
        stats(&game, GameObjectId(10_000)),
        (Some(7), Some(9)),
        "and Arcades, sitting home untapped, covers itself",
    );
}
