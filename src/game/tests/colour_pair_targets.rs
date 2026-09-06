//! A target slot restricted to two colours. The Invasion Weavers each help
//! the two colours their own is not, which is the whole design: the ability
//! is useless in a mono-coloured deck and the slot has to refuse everything
//! outside the pair -- including the Weaver itself.

use super::*;

/// Rage Weaver, which helps black or green creatures, with a board of one
/// creature of each colour.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut weaver = creature(78_000, cards::RAGE_WEAVER, PlayerId::One);
    weaver.entered_controller_turn = 0;
    let weaver_id = weaver.card.id;
    game.battlefield.push(weaver);
    for (index, definition) in [
        cards::RAZORTOOTH_RATS, // black
        cards::GRIZZLY_BEARS,   // green
        cards::SAVANNAH_LIONS,  // white
    ]
    .into_iter()
    .enumerate()
    {
        let mut other = creature(
            78_100 + u32::try_from(index).expect("a small fixture"),
            definition,
            PlayerId::One,
        );
        other.entered_controller_turn = 0;
        game.battlefield.push(other);
    }
    game.players[0].mana_pool.colorless = 2;
    (game, weaver_id)
}

fn offered(game: &Game, weaver: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == weaver => Some(targets),
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .filter_map(|target| match target {
            Target::Permanent(id) => Some(id),
            _ => None,
        })
        .collect()
}

#[test]
fn only_the_two_named_colours_are_offered() {
    let (game, weaver) = staged();
    let mut targets = offered(&game, weaver);
    targets.sort_unstable();
    assert_eq!(
        targets,
        vec![GameObjectId(78_100), GameObjectId(78_101)],
        "the black and green creatures, and neither the white one nor the red Weaver"
    );
}

#[test]
fn the_grant_lands_on_the_creature_it_named() {
    let (mut game, weaver) = staged();
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == weaver
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(GameObjectId(78_101)))
            }
            _ => false,
        })
        .expect("the green creature is a legal target");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(78_101))
        .expect("the Bears are on the battlefield");
    assert!(
        game.permanent_has_executable_keyword(bear, KeywordAbility::Haste),
        "it has haste for the turn"
    );
}
