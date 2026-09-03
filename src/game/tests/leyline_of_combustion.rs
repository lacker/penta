//! Leyline of Combustion's grouped target-selection trigger.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_COMBUSTION)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority can pass");
    }
    drain_pending(game);
}

/// The printed "you and/or at least one permanent" is one event for the
/// targeting spell, even when both halves of that phrase match at once.
#[test]
fn groups_all_targets_chosen_for_one_spell() {
    let mut game = staged();
    let lotus = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_LOTUS)
        .expect("cataloged");
    drain_pending(&mut game);
    let command = game
        .build_zone(PlayerId::Two, &[cards::KOLAGHAN_S_COMMAND])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let held = command.id;
    game.players[1].hand.push(command);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == held
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::One))
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(lotus))
            }
            _ => false,
        })
        .expect("the Command can target both halves of the Leyline clause");
    game.apply(PlayerId::Two, cast).expect("it is cast");

    settle(&mut game);
    assert_eq!(game.players[1].life, 18, "the opponent takes 2, not 4");
}

/// Activated abilities use the same atomic target-selection event as spells.
#[test]
fn triggers_for_an_opponents_targeting_ability() {
    let mut game = staged();
    let lotus = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_LOTUS)
        .expect("cataloged");
    let key = game
        .put_onto_battlefield(PlayerId::Two, cards::MANIFOLD_KEY)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);

    let activate = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == key
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(lotus))
            }
            _ => false,
        })
        .expect("the Key can target the opposing artifact");
    game.apply(PlayerId::Two, activate)
        .expect("the ability activates");

    settle(&mut game);
    assert_eq!(game.players[1].life, 18, "the activating opponent takes 2");
}

/// Changing an object already on the stack is another target-selection
/// occurrence, not another cast. It still makes the newly named permanent a
/// target and therefore satisfies the same declarative target filter.
#[test]
fn triggers_when_an_opponents_spell_is_retargeted_to_your_permanent() {
    let mut game = staged();
    let yours = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_LOTUS)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bolt = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let held = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);

    game.apply(
        PlayerId::Two,
        cast_action(held, vec![Target::Permanent(theirs)], Vec::new(), 0),
    )
    .expect("the Bolt is cast at its controller's creature");
    let spell = game.stack.last().expect("the Bolt is on the stack").id;
    assert!(
        game.pending_triggers.is_empty(),
        "the original target is outside the Leyline controller's side"
    );

    game.change_stack_targets(
        spell,
        &[TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(yours),
        )],
    );
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "the retargeting occurrence dealt 2 without publishing another cast"
    );
}

/// A copy is a new spell with its own targets. Inheriting the original
/// target unchanged still makes that player a target of the copy.
#[test]
fn triggers_for_a_copied_spell_with_its_inherited_target() {
    let mut game = staged();
    let original = spell_with_targets(
        180_000,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::One)],
        0,
    );
    let inherited = vec![TargetSelection::single(
        TargetSlotId(0),
        Target::Player(PlayerId::One),
    )];

    game.push_copy(original, PlayerId::Two, inherited);
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "the copy's inherited target is new relative to the copy"
    );
}

/// Ability copies follow the same rule. The Leyline arrives only after the
/// original ability was activated, so only the copy can trigger it.
#[test]
fn triggers_for_a_copied_ability_with_its_inherited_target() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let lotus = game
        .put_onto_battlefield(PlayerId::One, cards::BLACK_LOTUS)
        .expect("cataloged");
    let key = game
        .put_onto_battlefield(PlayerId::Two, cards::MANIFOLD_KEY)
        .expect("cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
    game.priority = PlayerId::Two;

    let activate = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == key
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(lotus))
            }
            _ => false,
        })
        .expect("the Key can target the opposing artifact");
    game.apply(PlayerId::Two, activate)
        .expect("the ability activates");
    assert!(
        game.pending_triggers.is_empty(),
        "the original ability was targeted before the Leyline arrived"
    );

    let original = game
        .stack
        .last()
        .expect("the ability is on the stack")
        .clone();
    let inherited = original
        .ability
        .as_ref()
        .expect("an activated ability has a payload")
        .targets
        .clone();
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_COMBUSTION)
        .expect("cataloged");
    game.push_copy(original, PlayerId::Two, inherited);
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "the copied ability has its own targeting occurrence"
    );
}
