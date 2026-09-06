//! A counterspell that reads what its target is aimed at, and a one-turn
//! prohibition put on a player. Both are conditions nothing else on the
//! board reflects: Confound is simply not castable when the spell on the
//! stack names a player, and Turf Wound leaves no visible mark beyond the
//! land drop it takes away.

use super::*;

/// Confound in player one's hand with a Bolt of player two's on the stack,
/// aimed at `target`.
fn responding_to(target: Target) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut bear = creature(86_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    game.battlefield.push(bear);
    let confound = card(86_100, cards::CONFOUND, PlayerId::One);
    let confound_id = confound.id;
    game.players[0].hand.push(confound);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.stack.push(spell_with_targets(
        86_200,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![target],
        0,
    ));
    game.priority = PlayerId::One;
    (game, confound_id)
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

#[test]
fn confound_answers_a_bolt_aimed_at_a_creature() {
    let (game, confound) = responding_to(Target::Permanent(GameObjectId(86_000)));
    assert!(
        castable(&game, confound),
        "the Bolt targets a creature, so it may be countered"
    );
}

#[test]
fn confound_is_stuck_against_a_bolt_to_the_face() {
    let (game, confound) = responding_to(Target::Player(PlayerId::One));
    assert!(
        !castable(&game, confound),
        "the same Bolt aimed at a player is not a legal target"
    );
}

/// Turf Wound resolved against `victim`, with a Forest in their hand.
fn wounded(cast: bool) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[1]
        .hand
        .push(card(86_300, cards::FOREST, PlayerId::Two));
    if cast {
        let wound = card(86_400, cards::TURF_WOUND, PlayerId::One);
        let wound_id = wound.id;
        game.players[0].hand.push(wound);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == wound_id
                        && choices
                            .targets()
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|target| *target == Target::Player(PlayerId::Two))
                }
                _ => false,
            })
            .expect("the opponent is a legal target");
        game.apply(PlayerId::One, action)
            .expect("the cast is legal");
        pass_priority_pair(&mut game);
    }
    // Hand the turn over so the victim would ordinarily get their land drop.
    game.active_player = PlayerId::Two;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game
}

fn can_play_land(game: &Game) -> bool {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .any(|action| matches!(action, Action::PlayLand { .. }))
}

#[test]
fn the_land_drop_is_there_without_the_wound() {
    assert!(
        can_play_land(&wounded(false)),
        "an untouched player plays their Forest"
    );
}

#[test]
fn turf_wound_takes_the_land_drop_away() {
    assert!(
        !can_play_land(&wounded(true)),
        "and cannot once Turf Wound has resolved at them"
    );
}
