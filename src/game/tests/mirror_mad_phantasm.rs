//! Mirror-Mad Phantasm's owner-relative shuffle and reveal procedure.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started = [5, 5];
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    for player in &mut game.players {
        player.library.clear();
        player.graveyard.clear();
    }
    game
}

fn activate(game: &mut Game, player: PlayerId, source: GameObjectId) {
    game.priority = player;
    game.players[player.index()].mana_pool.blue = 1;
    game.players[player.index()].mana_pool.colorless = 1;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .expect("Mirror-Mad's activated ability is offered");
    game.apply(player, action).expect("the ability activates");
}

#[test]
fn it_shuffles_then_reveals_itself_back_onto_the_battlefield() {
    let mut game = ready();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::MIRROR_MAD_PHANTASM)
        .expect("cataloged");

    activate(&mut game, PlayerId::One, source);
    drain_pending(&mut game);

    assert!(game.players[0].library.is_empty());
    assert!(game.players[0].graveyard.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MIRROR_MAD_PHANTASM),
        "the only card shuffled in is revealed and returned",
    );
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            player: PlayerId::One,
            definition,
            ..
        } if *definition == cards::MIRROR_MAD_PHANTASM
    )));
}

#[test]
fn a_stolen_phantasm_uses_its_owners_library_and_returns_under_their_control() {
    let mut game = ready();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::MIRROR_MAD_PHANTASM)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("it entered")
        .controller = PlayerId::Two;

    activate(&mut game, PlayerId::Two, source);
    drain_pending(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MIRROR_MAD_PHANTASM)
        .expect("the owner's library produced it");
    assert_eq!(returned.card.owner, PlayerId::One);
    assert_eq!(returned.controller, PlayerId::One);
    assert_eq!(game.players[1].mana_pool.blue, 0, "the controller paid");
}

#[test]
fn a_source_removed_before_resolution_skips_the_shuffle_and_reveal() {
    let mut game = ready();
    game.players[0]
        .library
        .push(card(20_000, cards::LIGHTNING_BOLT, PlayerId::One));
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::MIRROR_MAD_PHANTASM)
        .expect("cataloged");

    activate(&mut game, PlayerId::One, source);
    game.return_permanent_to_hand(source);
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 1, "the Phantasm stayed in hand");
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "the unavailable mandatory cost skips its conditional remainder",
    );
    assert!(game.players[0].graveyard.is_empty());
    assert!(
        !game
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. })),
        "no cards are revealed when the resolution cost cannot begin",
    );
}

#[test]
fn a_creature_that_gains_the_ability_mills_the_whole_library_when_no_name_matches() {
    let mut game = ready();
    game.players[0].library.extend([
        card(20_100, cards::LIGHTNING_BOLT, PlayerId::One),
        card(20_101, cards::GRIZZLY_BEARS, PlayerId::One),
    ]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let mirror_mad_ability = game
        .catalog
        .get(cards::MIRROR_MAD_PHANTASM)
        .expect("cataloged")
        .rules
        .ability_clauses()[1];
    let angel = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .expect("the Angel entered");
    let mut copied = copied_characteristics(cards::SERRA_ANGEL);
    copied.added_abilities.push(CopiableAbility {
        origin: AbilityOrigin::Printed {
            definition: cards::MIRROR_MAD_PHANTASM,
            part: CardPartId::PRIMARY,
            ability: AbilityId(1),
        },
        definition: mirror_mad_ability,
    });
    angel.copy_effect = Some(copied);

    activate(&mut game, PlayerId::One, source);
    drain_pending(&mut game);

    assert!(game.players[0].library.is_empty());
    assert_eq!(
        game.players[0].graveyard.len(),
        3,
        "the two old cards and the shuffled-in Angel were all revealed and milled",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "nothing named Mirror-Mad Phantasm was found",
    );
}
