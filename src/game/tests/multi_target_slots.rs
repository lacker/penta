//! Slots that hold more than one object. A fixed-count slot takes exactly
//! that many and an X slot takes as many as the caster paid for, and in both
//! cases the effect runs once per member rather than once for the slot --
//! which is what separates Swelter from a spell that deals two damage once.

use super::*;

fn offered_targets(game: &Game, spell: CardInstanceId) -> Vec<Vec<Target>> {
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
fn a_fixed_slot_takes_exactly_two_and_hits_both() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for index in 0..3u32 {
        game.battlefield.push(creature(
            70_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        ));
    }
    let swelter = card(70_010, cards::SWELTER, PlayerId::One);
    game.players[0].hand.push(swelter.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 3;

    let offers = offered_targets(&game, swelter.id);
    assert!(!offers.is_empty(), "the spell is castable");
    assert!(
        offers.iter().all(|targets| targets.len() == 2),
        "every offer names exactly two creatures, never one or three"
    );

    let chosen = [
        Target::Permanent(GameObjectId(70_000)),
        Target::Permanent(GameObjectId(70_001)),
    ];
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == swelter.id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .copied()
                        .eq(chosen.iter().copied())
            }
            _ => false,
        })
        .expect("those two are a legal pair");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    game.check_state_based_actions();

    let alive: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect();
    assert_eq!(
        alive,
        vec![GameObjectId(70_002)],
        "both named creatures took two damage, and the third took none"
    );
}

#[test]
fn an_x_slot_takes_as_many_as_x_and_counters_each_one() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for index in 0..3u32 {
        let mut bear = creature(70_100 + index, cards::GRIZZLY_BEARS, PlayerId::One);
        bear.entered_controller_turn = 0;
        game.battlefield.push(bear);
    }
    let thrive = card(70_110, cards::THRIVE, PlayerId::One);
    game.players[0].hand.push(thrive.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 2;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == thrive.id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .count()
                        == 2
            }
            _ => false,
        })
        .expect("X = 2 names two creatures");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let counters: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.counters(CounterKind::PlusOnePlusOne))
        .collect();
    assert_eq!(
        counters.iter().filter(|count| **count == 1).count(),
        2,
        "each of the two named creatures got one counter"
    );
    assert_eq!(
        counters.iter().filter(|count| **count == 0).count(),
        1,
        "and the third got none"
    );
}
