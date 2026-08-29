//! Turning a creature into an artifact and growing it at the same time.
//!
//! Ashnod's Transmogrant is gone by the time either half lands, so nothing is
//! scoped to it surviving: the counter and the type both stay.

use super::*;

fn transmogrified(target: CardDefinitionId) -> (Game, GameObjectId, bool) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let transmogrant = creature(10_000, cards::ASHNODS_TRANSMOGRANT, PlayerId::One);
    let transmogrant_id = transmogrant.card.id;
    game.battlefield.push(transmogrant);
    let subject = creature(10_001, target, PlayerId::One);
    let subject_id = subject.card.id;
    game.battlefield.push(subject);

    let offered = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, .. } if *source == transmogrant_id)
    });
    let was_offered = offered.is_some();
    if let Some(action) = offered {
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        drain_pending(&mut game);
    }
    (game, subject_id, was_offered)
}

#[test]
fn it_grows_the_creature_and_makes_it_an_artifact() {
    let (game, subject_id, offered) = transmogrified(cards::SAVANNAH_LIONS);
    assert!(offered, "a nonartifact creature is a legal target");

    let subject = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == subject_id)
        .expect("still there");
    assert_eq!(game.power(subject), Some(3), "a 2/1 with a counter on it");
    assert_eq!(game.toughness(subject), Some(2));
    assert!(
        game.permanent_types(subject)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "and an artifact now"
    );
    assert!(
        game.permanent_types(subject)
            .is_some_and(|types| types.contains(CardType::Creature)),
        "in addition to its other types, not instead of them"
    );
}

/// The control: it names a nonartifact creature, so an artifact creature is
/// not a legal target and the ability is never offered.
#[test]
fn it_will_not_point_at_an_artifact_creature() {
    let (_game, _subject, offered) = transmogrified(cards::JUGGERNAUT);
    assert!(!offered);
}
