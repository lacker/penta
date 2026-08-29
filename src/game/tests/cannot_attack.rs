//! A group-wide "can't attack".
//!
//! Different from the printed "can't attack unless ..." a creature says about
//! itself: this one is applied from elsewhere, so it covers whatever the
//! query matches and stops when its source leaves. What these check is who is
//! covered, who is exempt, and that the exemptions are read from the live
//! board rather than the printed one.

use super::*;

fn attackers(game: &Game) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareAttacker { attacker, .. } => Some(attacker),
            _ => None,
        })
        .collect()
}

/// A board where player one is the attacker and everything is ready to swing.
fn attack_ready() -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game
}

fn add(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let permanent = creature(id, definition, PlayerId::One);
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    permanent_id
}

#[test]
fn the_evil_eye_stops_everything_that_is_not_an_eye() {
    let mut game = attack_ready();
    let eye = add(&mut game, 10_000, cards::EVIL_EYE_OF_ORMS_BY_GORE);
    let troll = add(&mut game, 10_001, cards::SEDGE_TROLL);

    let legal = attackers(&game);
    assert!(legal.contains(&eye), "the Eye itself is an Eye");
    assert!(
        !legal.contains(&troll),
        "and a Troll is not, so it cannot attack"
    );
}

/// The prohibition is continuous, so removing its source restores the attack.
#[test]
fn removing_the_source_gives_the_attack_back() {
    let mut game = attack_ready();
    let eye = add(&mut game, 10_000, cards::EVIL_EYE_OF_ORMS_BY_GORE);
    let troll = add(&mut game, 10_001, cards::SEDGE_TROLL);
    assert!(!attackers(&game).contains(&troll));

    game.battlefield
        .retain(|permanent| permanent.card.id != eye);
    assert!(
        attackers(&game).contains(&troll),
        "nothing is holding it back once the Eye has gone"
    );
}

/// Akron Legionnaire exempts its own name and artifact creatures, which is
/// two different reasons to be allowed to attack.
#[test]
fn akron_exempts_its_own_name_and_artifact_creatures() {
    let mut game = attack_ready();
    let akron = add(&mut game, 10_000, cards::AKRON_LEGIONNAIRE);
    let second_akron = add(&mut game, 10_001, cards::AKRON_LEGIONNAIRE);
    let juggernaut = add(&mut game, 10_002, cards::JUGGERNAUT);
    let troll = add(&mut game, 10_003, cards::SEDGE_TROLL);

    let legal = attackers(&game);
    assert!(legal.contains(&akron), "it never stops itself");
    assert!(
        legal.contains(&second_akron),
        "nor another creature with its name"
    );
    assert!(
        legal.contains(&juggernaut),
        "an artifact creature is exempt too"
    );
    assert!(!legal.contains(&troll), "and everything else is held back");
}

/// It only covers creatures its controller controls, which is what "creatures
/// you control" says.
#[test]
fn an_opponents_creatures_are_untouched() {
    let mut game = attack_ready();
    add(&mut game, 10_000, cards::EVIL_EYE_OF_ORMS_BY_GORE);
    let theirs = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    game.active_player = PlayerId::Two;
    game.turns_started[PlayerId::Two.index()] = 5;
    assert!(
        game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == theirs_id)
        }),
        "the Eye says nothing about creatures the other player controls"
    );
}
