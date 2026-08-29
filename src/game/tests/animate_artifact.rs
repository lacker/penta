//! Animate Artifact and CR 613.6's one-recipient-set rule.
//!
//! The Aura chooses a noncreature artifact when its type-changing component
//! starts in layer 4. Its base-size component keeps that same object in layer
//! 7b rather than asking again after the object has become a creature.

use super::*;

fn attached_to(host_definition: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let host = creature(10_000, host_definition, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut aura = creature(10_001, cards::ANIMATE_ARTIFACT, PlayerId::One);
    aura.attached_to = Some(host_id);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    (game, host_id, aura_id)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is still on the battlefield")
}

fn is_creature(game: &Game, id: GameObjectId) -> bool {
    game.permanent_types(permanent(game, id))
        .is_some_and(|types| types.contains(CardType::Creature))
}

#[test]
fn the_attached_noncreature_artifact_keeps_its_layer_four_selection_in_layer_seven() {
    let (game, tome_id, _) = attached_to(cards::JAYEMDAE_TOME);
    let tome = permanent(&game, tome_id);

    assert!(is_creature(&game, tome_id));
    assert!(
        game.permanent_types(tome)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "the animation adds Creature without replacing Artifact",
    );
    assert_eq!(game.power(tome), Some(4));
    assert_eq!(game.toughness(tome), Some(4));
}

#[test]
fn an_artifact_that_was_already_a_creature_keeps_its_own_body() {
    let (game, juggernaut_id, aura_id) = attached_to(cards::JUGGERNAUT);
    let juggernaut = permanent(&game, juggernaut_id);

    assert_eq!(game.power(juggernaut), Some(5));
    assert_eq!(game.toughness(juggernaut), Some(3));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura_id),
        "Enchant artifact remains legal even when the artifact is a creature",
    );
}

#[test]
fn a_resolved_animation_also_keeps_animate_artifact_from_replacing_the_body() {
    let mut game = ready_game();
    game.turns_started = [1, 1];
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;

    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == factory_id)
        })
        .expect("the Factory can animate");
    game.apply(PlayerId::One, animate)
        .expect("the animation activates");
    drain_pending(&mut game);

    let mut aura = creature(10_001, cards::ANIMATE_ARTIFACT, PlayerId::One);
    aura.attached_to = Some(factory_id);
    game.battlefield.push(aura);

    let factory = permanent(&game, factory_id);
    assert!(is_creature(&game, factory_id));
    assert_eq!(game.power(factory), Some(2));
    assert_eq!(game.toughness(factory), Some(2));
}

#[test]
fn removing_the_aura_ends_both_components_together() {
    let (mut game, tome_id, aura_id) = attached_to(cards::JAYEMDAE_TOME);
    assert!(is_creature(&game, tome_id));

    game.battlefield
        .retain(|permanent| permanent.card.id != aura_id);

    let tome = permanent(&game, tome_id);
    assert!(!is_creature(&game, tome_id));
    assert_eq!(game.power(tome), None);
    assert_eq!(game.toughness(tome), None);
}

#[test]
fn a_zero_mana_artifact_animates_to_zero_zero_and_dies() {
    let (mut game, lotus_id, aura_id) = attached_to(cards::BLACK_LOTUS);
    assert_eq!(game.power(permanent(&game, lotus_id)), Some(0));
    assert_eq!(game.toughness(permanent(&game, lotus_id)), Some(0));

    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == lotus_id),
        "the animated 0/0 dies",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura_id),
        "then the unattached Aura goes too",
    );
}
