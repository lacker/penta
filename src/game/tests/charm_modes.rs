//! A charm whose three modes do not share a target restriction. Choosing a
//! mode picks its own target slot, so what needs covering is that a mode
//! whose restriction nothing on the board satisfies is not offered, while
//! the other two still are -- and that taking one does that mode and not
//! another.

use super::*;

/// Fever Charm in hand with `board` creatures under player two.
fn staged(board: &[CardDefinitionId]) -> (Game, CardInstanceId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let mut ids = Vec::new();
    for (index, definition) in board.iter().enumerate() {
        let mut permanent = creature(
            81_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::Two,
        );
        permanent.entered_controller_turn = 0;
        ids.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    let charm = card(81_100, cards::FEVER_CHARM, PlayerId::One);
    let charm_id = charm.id;
    game.players[0].hand.push(charm);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    (game, charm_id, ids)
}

/// The distinct modes offered for the charm, by the text each one prints.
fn offered_modes(game: &Game, charm: CardInstanceId) -> Vec<String> {
    let mut modes: Vec<String> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == charm => {
                Some(format!("{:?}", choices.modes()))
            }
            _ => None,
        })
        .collect();
    modes.sort();
    modes.dedup();
    modes
}

#[test]
fn a_wizardless_board_hides_the_wizard_mode() {
    let (game, charm, _) = staged(&[cards::GRIZZLY_BEARS]);
    assert_eq!(
        offered_modes(&game, charm).len(),
        2,
        "haste and the pump, but nothing to point three damage at"
    );

    let (game, charm, _) = staged(&[cards::GRIZZLY_BEARS, cards::PATRON_WIZARD]);
    assert_eq!(
        offered_modes(&game, charm).len(),
        3,
        "a Wizard on the board opens the third mode"
    );
}

#[test]
fn the_damage_mode_kills_the_wizard_it_names() {
    let (mut game, charm, ids) = staged(&[cards::PATRON_WIZARD]);
    let wizard = ids[0];
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == charm
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(wizard))
                    && format!("{:?}", choices.modes()).contains('2')
            }
            _ => false,
        })
        .expect("some mode aims at the Wizard");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    for _ in 0..8 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.battlefield.is_empty(),
        "three damage finished off a 2/2 Wizard"
    );
}
