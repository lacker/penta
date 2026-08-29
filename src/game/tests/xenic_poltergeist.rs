//! Animating an artifact at the size it cost.
//!
//! Two halves of one clause: the type goes on, and a base size comes with it.
//! Both numbers are the artifact's own mana value, so a Mox stands up as a
//! 0/0 and dies where a Juggernaut walks away a 4/4.

use super::*;

/// The Poltergeist pointed at `artifact`, if the ability is offered at all.
fn animated(artifact: CardDefinitionId) -> (Game, GameObjectId, bool) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let poltergeist = creature(10_000, cards::XENIC_POLTERGEIST, PlayerId::One);
    let poltergeist_id = poltergeist.card.id;
    game.battlefield.push(poltergeist);
    let subject = creature(10_001, artifact, PlayerId::One);
    let subject_id = subject.card.id;
    game.battlefield.push(subject);

    let offered = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, .. } if *source == poltergeist_id)
    });
    let was_offered = offered.is_some();
    if let Some(action) = offered {
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        drain_pending(&mut game);
    }
    (game, subject_id, was_offered)
}

fn subject_of(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
}

#[test]
fn it_stands_the_artifact_up_at_its_mana_value() {
    let (game, subject, offered) = animated(cards::JAYEMDAE_TOME);
    assert!(offered, "a noncreature artifact is a legal target");

    let subject = subject_of(&game, subject);
    assert!(
        game.permanent_types(subject)
            .is_some_and(|types| types.contains(CardType::Creature)),
        "a creature now",
    );
    assert!(
        game.permanent_types(subject)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "and still an artifact",
    );
    assert_eq!(game.power(subject), Some(4), "Jayemdae Tome costs four");
    assert_eq!(game.toughness(subject), Some(4));
}

/// A zero-cost artifact stands up as a 0/0, which state-based actions put
/// straight into the graveyard. That is the card working, not failing.
#[test]
fn a_free_artifact_animates_into_nothing() {
    let (mut game, subject, offered) = animated(cards::BLACK_LOTUS);
    assert!(offered);

    game.check_state_based_actions();
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == subject),
        "a 0/0 does not stay on the battlefield",
    );
}

/// The control: it names a noncreature artifact, so an artifact that is
/// already a creature is never offered.
#[test]
fn it_will_not_point_at_an_artifact_creature() {
    let (_game, _subject, offered) = animated(cards::JUGGERNAUT);
    assert!(!offered);
}
