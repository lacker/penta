//! Mindshrieker: the card milled by its activated ability sets its temporary
//! size bonus.

use super::*;
use crate::ImplementationStatus;

/// Mindshrieker under player one's control, with `library` stacked for player
/// two so the last entry is on top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].library.clear();
    for (index, definition) in library.iter().enumerate() {
        game.players[PlayerId::Two.index()].library.push(card(
            92_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::Two,
        ));
    }
    let shrieker = game
        .put_onto_battlefield(PlayerId::One, cards::MINDSHRIEKER)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.priority = PlayerId::One;
    (game, shrieker)
}

fn activate_for_opponent(game: &mut Game, shrieker: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    ..
                } if *source == shrieker
                    && *targets == activated_targets(Target::Player(PlayerId::Two))
            )
        })
        .expect("the opponent is a legal target");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    pass_priority_pair(game);
}

fn size(game: &Game, shrieker: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == shrieker)
        .expect("Mindshrieker remains on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn the_milled_card_sets_the_bonus_from_its_mana_value() {
    let (mut game, shrieker) = staged(&[cards::SERRA_ANGEL]);

    activate_for_opponent(&mut game, shrieker);

    assert_eq!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "the targeted player mills the top card",
    );
    assert_eq!(size(&game, shrieker), (Some(6), Some(6)));
}

#[test]
fn an_empty_library_gives_no_bonus() {
    let (mut game, shrieker) = staged(&[]);

    activate_for_opponent(&mut game, shrieker);

    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
    assert_eq!(size(&game, shrieker), (Some(1), Some(1)));
}

#[test]
fn mindshrieker_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::MINDSHRIEKER)
        .expect("Mindshrieker is cataloged");
    assert_eq!(
        card.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
}
