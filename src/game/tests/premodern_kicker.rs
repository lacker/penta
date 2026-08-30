//! Kicker: the same spell, cast for more, doing more.
//!
//! Both cards target first and ask how big the thing was afterwards, so a
//! too-large target can be named and simply survives. What the kicker buys is
//! a higher ceiling, not a different target -- which is what these tests
//! pin down, in both directions.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

fn resolve(game: &mut Game) {
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Every way of casting `card` at `target` that the player is offered, keyed
/// by the total mana each one costs.
fn casts_at(game: &Game, card: GameObjectId, target: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| match action {
            Action::CastSpell {
                card: cast,
                choices,
                ..
            } => {
                *cast == card
                    && choices
                        .targets()
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(target)))
            }
            _ => false,
        })
        .collect()
}

fn cast_overload(kicked: bool, artifact: CardDefinitionId) -> Game {
    let mut game = ready();
    let target = creature(10_000, artifact, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let overload = card(20_000, cards::OVERLOAD, PlayerId::One);
    let overload_id = overload.id;
    game.players[PlayerId::One.index()].hand.push(overload);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    if kicked {
        game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    }

    let offered = casts_at(&game, overload_id, target_id);
    assert_eq!(
        offered.len(),
        if kicked { 2 } else { 1 },
        "one red pays only the printed cost; three pays either",
    );
    // Kicker is an optional additional cost, independent of the printed
    // mana cost rather than a second total cost.
    let chosen = offered
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { choices, .. }
                if choices.costs().alternative().is_none()
                    && choices.costs().additional().is_empty() != kicked)
        })
        .expect("a cast was offered");
    game.apply(PlayerId::One, chosen).expect("it is cast");
    resolve(&mut game);
    game
}

fn artifact_survived(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == CardInstanceId(10_000))
}

/// Unkicked, the ceiling is two: a one-mana artifact dies.
#[test]
fn unkicked_overload_destroys_a_cheap_artifact() {
    let game = cast_overload(false, cards::BLACK_VISE);
    assert!(!artifact_survived(&game), "a Vise costs one, so it dies");
}

/// And a three-mana artifact is named, targeted legally, and simply lives --
/// the spell resolves and does nothing.
#[test]
fn unkicked_overload_leaves_an_expensive_artifact_alone() {
    let game = cast_overload(false, cards::BASALT_MONOLITH);
    assert!(
        artifact_survived(&game),
        "a Monolith costs three, over the unkicked ceiling of two",
    );
    assert!(game.stack.is_empty(), "the spell still resolved");
}

/// Kicked, the ceiling is five, so the same artifact dies.
#[test]
fn kicked_overload_destroys_what_the_unkicked_spell_could_not() {
    let game = cast_overload(true, cards::BASALT_MONOLITH);
    assert!(
        !artifact_survived(&game),
        "three is under the kicked ceiling of five",
    );
}

/// The kicked ceiling is five, not "anything": a six-mana artifact survives
/// even the kicked spell, so the kicker raises the bar rather than removing
/// it.
#[test]
fn even_kicked_overload_has_a_ceiling() {
    let game = cast_overload(true, cards::MANA_MATRIX);
    assert!(
        artifact_survived(&game),
        "six is over the kicked ceiling of five",
    );
}

/// Paying the kicker adds two generic mana to the printed cost: one red alone
/// offers only the unkicked cast.
#[test]
fn the_kicker_is_not_offered_without_the_extra_mana() {
    let mut game = ready();
    let mine = creature(10_000, cards::HOWLING_MINE, PlayerId::Two);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let overload = card(20_000, cards::OVERLOAD, PlayerId::One);
    let overload_id = overload.id;
    game.players[PlayerId::One.index()].hand.push(overload);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    assert_eq!(
        casts_at(&game, overload_id, mine_id).len(),
        1,
        "two mana total is one short of the kicked cost",
    );
}

/// Prohibit reads the mana value of a spell on the stack rather than a
/// permanent on the battlefield, so it exercises the other half of the same
/// condition.
///
/// Whether the spell was countered is read from whether it *did* anything --
/// a spell that resolves ends up in the graveyard just as a countered one
/// does, so the graveyard alone cannot tell them apart. A sentinel creature
/// on the battlefield is the witness: a resolving Wrath kills it, and a
/// resolving Grizzly Bears stands beside it.
fn prohibit_against(kicked: bool, target: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.turn = 5;
    // Their turn, so a sorcery is castable; Prohibit answers at instant speed.
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));

    let theirs = card(21_000, target, PlayerId::Two);
    let theirs_id = theirs.id;
    game.players[PlayerId::Two.index()].hand.push(theirs);
    game.players[PlayerId::Two.index()].mana_pool.colorless = 4;
    game.players[PlayerId::Two.index()].mana_pool.white = 2;
    game.players[PlayerId::Two.index()].mana_pool.green = 2;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == theirs_id))
        .expect("they can afford their own spell");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    // The caster keeps priority after casting; the answer comes after they
    // pass, with the spell still on the stack.
    game.apply(PlayerId::Two, Action::PassPriority)
        .expect("they pass with their spell on the stack");

    let prohibit = card(20_000, cards::PROHIBIT, PlayerId::One);
    let prohibit_id = prohibit.id;
    game.players[PlayerId::One.index()].hand.push(prohibit);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = if kicked { 3 } else { 1 };

    let chosen = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == prohibit_id
                    && choices.costs().alternative().is_none()
                    && choices.costs().additional().is_empty() != kicked)
        })
        .expect("Prohibit is castable and can name the spell");
    game.apply(PlayerId::One, chosen).expect("it is cast");
    for _ in 0..12 {
        if game.stack.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game
}

fn sentinel_survived(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == CardInstanceId(10_000))
}

/// A two-mana spell is at the unkicked ceiling, so it never resolves and no
/// bear joins the board.
#[test]
fn unkicked_prohibit_counters_a_two_mana_spell() {
    let game = prohibit_against(false, cards::GRIZZLY_BEARS);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS),
        "the bear was countered on its way in",
    );
    assert!(sentinel_survived(&game), "and nothing else happened");
}

/// Four is over the unkicked ceiling, so the Wrath resolves and the sentinel
/// dies -- Prohibit resolved and simply did nothing.
#[test]
fn unkicked_prohibit_cannot_stop_a_four_mana_spell() {
    let game = prohibit_against(false, cards::WRATH_OF_GOD);
    assert!(
        !sentinel_survived(&game),
        "a Wrath costs four, over the unkicked ceiling of two",
    );
}

/// Kicked, the ceiling is four, so the same Wrath is countered and the
/// sentinel lives.
#[test]
fn kicked_prohibit_stops_the_four_mana_spell() {
    let game = prohibit_against(true, cards::WRATH_OF_GOD);
    assert!(
        sentinel_survived(&game),
        "four is at the kicked ceiling, so the Wrath never resolved",
    );
}
