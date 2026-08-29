//! Auras that take the permanent they are on.
//!
//! The printed clause is a live static control effect: it starts applying as
//! soon as the Aura is attached, never uses the stack, and follows the Aura if
//! another effect moves it to a different legal host.

use super::*;

/// Player two owns `host`; player one casts `aura` onto it.
fn stolen(aura: CardDefinitionId, host: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let target = creature(10_000, host, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);

    let spell = card(20_000, aura, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 4;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the Aura is castable onto it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    pass_priority_pair(&mut game);
    assert!(
        game.stack.is_empty() && game.pending_triggers.is_empty(),
        "the static control clause must not create a triggered ability"
    );
    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == aura)
        .expect("the Aura resolved onto the battlefield")
        .card
        .id;
    (game, target_id, aura_id)
}

fn controller(game: &Game, id: GameObjectId) -> Option<PlayerId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map(|permanent| permanent.controller)
}

#[test]
fn control_magic_takes_the_creature() {
    let (game, creature_id, _aura) = stolen(cards::CONTROL_MAGIC, cards::SERRA_ANGEL);

    assert_eq!(controller(&game, creature_id), Some(PlayerId::One));
}

/// The control is scoped to the Aura: destroy it and the creature goes home.
#[test]
fn destroying_control_magic_hands_the_creature_back() {
    let (mut game, creature_id, aura_id) = stolen(cards::CONTROL_MAGIC, cards::SERRA_ANGEL);
    assert_eq!(controller(&game, creature_id), Some(PlayerId::One));

    game.destroy_permanent(aura_id);
    drain_pending(&mut game);
    // Reverting control is a state-based action, checked the next time the
    // game looks rather than as the Aura leaves.
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert_eq!(
        controller(&game, creature_id),
        Some(PlayerId::Two),
        "the Aura is what was holding it"
    );
}

#[test]
fn steal_artifact_takes_the_artifact() {
    let (game, artifact_id, _aura) = stolen(cards::STEAL_ARTIFACT, cards::JUGGERNAUT);

    assert_eq!(controller(&game, artifact_id), Some(PlayerId::One));
}

#[test]
fn moving_control_magic_moves_its_static_control_effect() {
    let (mut game, first_id, aura_id) = stolen(cards::CONTROL_MAGIC, cards::SERRA_ANGEL);
    let second = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.push(second);

    assert!(game.try_attach(aura_id, second_id));
    game.check_state_based_actions();

    assert_eq!(controller(&game, first_id), Some(PlayerId::Two));
    assert_eq!(controller(&game, second_id), Some(PlayerId::One));
    assert!(game.stack.is_empty() && game.pending_triggers.is_empty());
}

#[test]
fn removing_control_magics_ability_ends_its_control_effect() {
    let (mut game, creature_id, aura_id) = stolen(cards::CONTROL_MAGIC, cards::SERRA_ANGEL);
    attach_constant_resolved_characteristics(
        &mut game,
        aura_id,
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );

    game.check_state_based_actions();

    assert_eq!(controller(&game, creature_id), Some(PlayerId::Two));
}

#[test]
fn newer_static_control_effect_wins_by_attachment_timestamp() {
    let (mut game, creature_id, first_aura) = stolen(cards::CONTROL_MAGIC, cards::SERRA_ANGEL);
    let second_aura = creature(10_001, cards::CONTROL_MAGIC, PlayerId::Two);
    let second_aura_id = second_aura.card.id;
    game.battlefield.push(second_aura);

    assert!(game.try_attach(second_aura_id, creature_id));
    game.check_state_based_actions();
    assert_eq!(controller(&game, creature_id), Some(PlayerId::Two));

    game.destroy_permanent(second_aura_id);
    drain_pending(&mut game);
    game.check_state_based_actions();
    assert_eq!(controller(&game, creature_id), Some(PlayerId::One));
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.card.id == first_aura && permanent.attached_to == Some(creature_id)
    }));
}

#[test]
fn both_control_clauses_are_static_abilities() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::CONTROL_MAGIC, cards::STEAL_ARTIFACT] {
        let card = catalog.get(definition).expect("the card is cataloged");
        let control = card.rules.ability_clauses().get(1).expect("control clause");
        assert!(matches!(
            control.definition,
            DeclarativeAbilityDef::Static(_)
        ));
    }
}
