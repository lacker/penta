//! Spells with two target slots, and a trigger that fires once per blocker.
//! Both are places where a count is easy to get wrong by one: "another
//! target creature" must refuse to take the first one twice, and "becomes
//! blocked by a creature" fires for each blocker rather than once for the
//! block.

use super::*;

fn creature_targets(game: &Game, spell: CardInstanceId) -> Vec<Vec<Target>> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => Some(
                choices
                    .targets()
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect(),
            ),
            _ => None,
        })
        .collect()
}

#[test]
fn another_target_refuses_to_name_the_same_creature_twice() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(57_000, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(57_001, cards::GRIZZLY_BEARS, PlayerId::Two));
    let spell = card(57_010, cards::STEAL_STRENGTH, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.black = 2;

    let offered = creature_targets(&game, spell_id);
    assert!(!offered.is_empty(), "the spell is castable");
    assert!(
        offered.iter().all(|targets| targets[0] != targets[1]),
        "no offer names one creature for both slots"
    );
    assert!(
        offered.iter().any(|targets| targets
            == &vec![
                Target::Permanent(GameObjectId(57_000)),
                Target::Permanent(GameObjectId(57_001)),
            ]),
        "pointing the bonus at your own and the penalty at theirs is legal"
    );
}

#[test]
fn shower_of_sparks_splits_its_damage_between_the_two_slots() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(57_100, cards::GRIZZLY_BEARS, PlayerId::Two));
    let spell = card(57_110, cards::SHOWER_OF_SPARKS, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.red = 1;

    // Take the offer the enumeration made rather than rebuilding its
    // choices: the slots carry ids the fixture does not know.
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } if *card == spell.id => choices
                .targets()
                .iter()
                .flat_map(TargetSelection::targets)
                .copied()
                .eq([
                    Target::Permanent(GameObjectId(57_100)),
                    Target::Player(PlayerId::Two),
                ]),
            _ => false,
        })
        .expect("the Bears and the opponent are a legal pair");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 19, "one damage to the player");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(57_100))
            .expect("the Bears survived")
            .damage,
        1,
        "and one to the creature"
    );
}

/// "Becomes blocked by a creature" names the blocker, so two blockers are
/// two triggers -- unlike "becomes blocked", which fires once however many
/// creatures block.
#[test]
fn blocked_by_a_creature_fires_once_per_blocker() {
    for (blockers, expected) in [(1, (5, 5)), (2, (6, 6))] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.turns_started[PlayerId::One.index()] = 5;
        let mut wolverines = creature(57_200, cards::RABID_WOLVERINES, PlayerId::One);
        wolverines.attacking = true;
        wolverines.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        let wolverines_id = wolverines.card.id;
        game.battlefield.push(wolverines);
        for index in 0..blockers {
            let mut bear = creature(
                57_300 + u32::try_from(index).expect("a small fixture"),
                cards::GRIZZLY_BEARS,
                PlayerId::Two,
            );
            bear.entered_controller_turn = 0;
            bear.blocking = vec![wolverines_id];
            game.battlefield.push(bear);
        }
        game.step = Step::DeclareBlockers;
        game.attackers_declared = true;
        game.finish_declaring_blockers();
        drain_pending(&mut game);

        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == wolverines_id)
            .expect("the Wolverines are on the battlefield");
        assert_eq!(
            (
                game.power(permanent).expect("power"),
                game.toughness(permanent).expect("toughness")
            ),
            expected,
            "a 4/4 with {blockers} blocker(s)"
        );
    }
}
