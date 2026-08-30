//! Heretic's Punishment: the cards milled by its activated ability determine
//! one damage amount from their greatest mana value, not their total.

use super::*;

/// The enchantment under player one's control, with that player's library
/// stacked in bottom-to-top order.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    for (index, definition) in library.iter().enumerate() {
        game.players[PlayerId::One.index()].library.push(card(
            93_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    let punishment = game
        .put_onto_battlefield(PlayerId::One, cards::HERETIC_S_PUNISHMENT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.priority = PlayerId::One;
    (game, punishment)
}

fn activate_at(game: &mut Game, punishment: GameObjectId, target: Target) {
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
                } if *source == punishment && *targets == activated_targets(target)
            )
        })
        .expect("the target is legal");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    pass_priority_pair(game);
}

#[test]
fn it_deals_the_greatest_milled_mana_value_rather_than_the_total() {
    let (mut game, punishment) = staged(&[
        cards::BLIGHTSTEEL_COLOSSUS,
        cards::SERRA_ANGEL,
        cards::FOREST,
        cards::LIGHTNING_BOLT,
    ]);

    activate_at(&mut game, punishment, Target::Player(PlayerId::Two));

    assert_eq!(
        game.players[PlayerId::One.index()]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::BLIGHTSTEEL_COLOSSUS],
        "the higher-mana-value fourth card was not milled and does not count",
    );
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 3);
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        15,
        "mana values one, zero, and five deal five rather than six",
    );
}

#[test]
fn a_short_library_uses_only_the_cards_that_were_actually_milled() {
    let (mut game, punishment) = staged(&[cards::SERRA_ANGEL]);
    let creature = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    activate_at(&mut game, punishment, Target::Permanent(creature));

    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != creature),
        "the lone five-mana card deals five damage to the creature",
    );
}

#[test]
fn an_empty_library_deals_zero_damage() {
    let (mut game, punishment) = staged(&[]);

    activate_at(&mut game, punishment, Target::Player(PlayerId::Two));

    assert_eq!(game.players[PlayerId::Two.index()].life, 20);
    assert!(game.players[PlayerId::One.index()].graveyard.is_empty());
}
