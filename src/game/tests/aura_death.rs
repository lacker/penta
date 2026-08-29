//! Auras that trigger on their host dying.
//!
//! The amount is the dead creature's power, which is only knowable from last
//! known information: by the time the trigger resolves the creature is in a
//! graveyard. What these check is that the number is read from the creature
//! as it last was, pumps included, rather than from its printed stats.

use super::*;

/// An Aura on a creature, both controlled by player one.
fn enchanted(aura: CardDefinitionId, pump: i16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    attach_constant_resolved_characteristics(
        &mut game,
        host_id,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(i32::from(pump)),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );

    let mut enchantment = creature(10_001, aura, PlayerId::One);
    enchantment.attached_to = Some(host_id);
    game.battlefield.push(enchantment);
    game.check_state_based_actions();
    (game, host_id)
}

#[test]
fn murder_investigation_makes_one_soldier_per_power() {
    // Sedge Troll is a 2/2, pumped to a 4/2.
    let (mut game, host) = enchanted(cards::MURDER_INVESTIGATION, 2);
    game.destroy_permanent(host);
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
            ))
            .count(),
        4,
        "four Soldiers for a creature that died as a 4/2"
    );
}

/// The Aura is in the graveyard by the time this resolves too, so the trigger
/// has to survive its own source leaving.
#[test]
fn the_aura_itself_is_gone_when_the_trigger_resolves() {
    let (mut game, host) = enchanted(cards::MURDER_INVESTIGATION, 0);
    game.destroy_permanent(host);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MURDER_INVESTIGATION),
        "the Aura fell off and died with its host"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
            ))
            .count(),
        2,
        "and its trigger still made two Soldiers"
    );
}

/// Dying Wish targets, and the target is the point: the life leaves whoever
/// was chosen rather than the Aura's controller. Answering the choice with
/// the first candidate would pick its own controller and net to nothing,
/// which is why this names the opponent explicitly.
#[test]
fn dying_wish_drains_the_chosen_player() {
    let (mut game, host) = enchanted(cards::DYING_WISH, 1);
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    game.destroy_permanent(host);
    game.check_state_based_actions();
    game.priority = PlayerId::One;
    // The trigger reaches the stack on the next priority pass, and only then
    // is its target asked for.
    game.apply(PlayerId::One, Action::PassPriority)
        .expect("priority passes");

    let decision = game
        .pending_decisions
        .first()
        .expect("the trigger asks for its target")
        .observation
        .clone();
    let opponent = decision
        .options
        .iter()
        .last()
        .expect("the other player is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![opponent],
        },
    )
    .expect("the choice is submitted");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        before[1] - 3,
        "the chosen player lost three"
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before[0] + 3,
        "and its controller gained three"
    );
}
