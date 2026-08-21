//! Saheeli, Sublime Artificer: a Servo for every noncreature spell, and a
//! minus that turns an artifact into something better for a turn.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let saheeli = game
        .put_onto_battlefield(PlayerId::One, cards::SAHEELI_SUBLIME_ARTIFICER)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == saheeli)
    {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, saheeli)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Casts `definition` from Player One's hand and lets everything settle.
fn cast(game: &mut Game, definition: CardDefinitionId, color: ManaColor, amount: u16) {
    let card = game
        .build_zone(PlayerId::One, &[definition])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, color, amount);
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("the spell is castable");
    game.apply(PlayerId::One, action).expect("it is castable");
    resolve(game);
}

fn servos(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(permanent, tokens::artifact_creature(&["Servo"], &[], 1, 1))
        })
        .count()
}

/// A noncreature spell makes a Servo.
#[test]
fn a_noncreature_spell_makes_a_servo() {
    let (mut game, _) = staged();
    cast(&mut game, cards::LIGHTNING_BOLT, ManaColor::Red, 1);

    assert_eq!(servos(&game), 1);
}

/// A creature spell does not, which is the half that names the trigger.
#[test]
fn a_creature_spell_makes_nothing() {
    let (mut game, _) = staged();
    cast(&mut game, cards::GRIZZLY_BEARS, ManaColor::Green, 2);

    assert_eq!(
        servos(&game),
        0,
        "a creature spell is not a noncreature one"
    );
}

/// The minus turns an artifact into a copy of a creature, and it is still an
/// artifact afterwards.
#[test]
fn the_minus_copies_a_creature_onto_an_artifact() {
    let (mut game, saheeli) = staged();
    let mox = game
        .put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let ability = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == saheeli
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(mox)))
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("the Mox may copy the Bears");
    game.apply(PlayerId::One, ability).expect("it activates");
    resolve(&mut game);

    let copy = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mox)
        .expect("the Mox is still there");
    assert_eq!(game.power(copy), Some(2), "a 2/2 now");
    let types = game.permanent_types(copy).expect("it has types");
    assert!(types.contains(CardType::Creature), "a creature");
    assert!(
        types.contains(CardType::Artifact),
        "and an artifact in addition to its other types",
    );
}

/// The copy lasts until end of turn and no longer.
#[test]
fn the_copy_ends_with_the_turn() {
    let (mut game, saheeli) = staged();
    let mox = game
        .put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let ability = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == saheeli),
        )
        .expect("the minus is available");
    game.apply(PlayerId::One, ability).expect("it activates");
    resolve(&mut game);

    game.step = Step::End;
    game.advance_step();
    drain_pending(&mut game);

    let mox = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mox)
        .expect("the Mox is still there");
    assert!(
        game.permanent_types(mox)
            .is_some_and(|types| !types.contains(CardType::Creature)),
        "a Mox again once the turn is over",
    );
}

/// Nothing may copy itself: the second target is another permanent.
#[test]
fn the_two_targets_must_differ() {
    let (mut game, saheeli) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(
                action,
                Action::ActivateAbility { source, .. } if *source == saheeli
            )),
        "one artifact alone cannot copy another",
    );
}
