//! Urborg, Tomb of Yawgmoth: every land on the battlefield is a Swamp, and
//! the battlefield is as far as that reaches.

use super::*;

/// Urborg out with a Mountain beside it, a Mountain in the graveyard and a
/// second Urborg in hand.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let urborg = game
        .put_onto_battlefield(PlayerId::One, cards::URBORG_TOMB_OF_YAWGMOTH)
        .expect("cataloged");
    let mountain = game
        .put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, urborg, mountain)
}

fn is_a_swamp(game: &Game, permanent: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("it is on the battlefield");
    game.effective_land_types(permanent)[BasicLandType::Swamp.index()]
}

/// A card in `zone` asked whether it is a Swamp, which is what a spell that
/// names one reads.
fn card_is_a_swamp(game: &Game, card: &CardInstance, zone: ZoneKind, source: GameObjectId) -> bool {
    game.card_object_matches(
        ObjectPredicateDef::Subtype(crate::card::SubtypeDef::Literal("Swamp")),
        card,
        zone,
        source,
    )
}

/// "Urborg's ability causes each land on the battlefield to have the land
/// type Swamp" -- its own included, which is what makes it tap for black.
#[test]
fn every_land_on_the_battlefield_is_a_swamp() {
    let (game, urborg, mountain) = staged();

    assert!(is_a_swamp(&game, mountain), "the Mountain is one");
    assert!(is_a_swamp(&game, urborg), "and so is Urborg itself");
}

/// "Urborg, Tomb of Yawgmoth isn't a Swamp while it's not on the
/// battlefield."
#[test]
fn urborg_is_no_swamp_anywhere_else() {
    let (mut game, urborg, _) = staged();
    let second = game
        .build_zone(PlayerId::One, &[cards::URBORG_TOMB_OF_YAWGMOTH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");

    assert!(
        !card_is_a_swamp(&game, &second, ZoneKind::Hand, urborg),
        "one in hand is a land card and no Swamp",
    );
    game.players[0].graveyard.push(second);
    let buried = game.players[0].graveyard.last().expect("it is there");
    assert!(
        !card_is_a_swamp(&game, buried, ZoneKind::Graveyard, urborg),
        "and neither is one in a graveyard",
    );
}

/// "Land cards not on the battlefield aren't Swamps while Urborg is on the
/// battlefield." The clause names permanents, so the graveyard keeps its
/// Mountains.
#[test]
fn a_land_card_off_the_battlefield_is_untouched() {
    let (mut game, urborg, mountain) = staged();
    let card = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(card);
    let buried = game.players[0].graveyard.last().expect("it is there");

    assert!(
        !card_is_a_swamp(&game, buried, ZoneKind::Graveyard, urborg),
        "a Mountain in the graveyard is a Mountain",
    );
    assert!(
        is_a_swamp(&game, mountain),
        "while the one on the battlefield is a Swamp too",
    );
}

/// "Nothing else changes about those lands, including their names, other
/// subtypes, other abilities, and whether they're legendary, basic, or
/// snow." A Wasteland still cannot name a basic Mountain that Urborg has
/// made a Swamp, and it can name Urborg.
#[test]
fn a_swamp_by_urborg_is_still_as_basic_as_it_was() {
    let (mut game, urborg, mountain) = staged();
    let wasteland = game
        .put_onto_battlefield(PlayerId::Two, cards::WASTELAND)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::Two;

    let named = |game: &Game, land: GameObjectId| {
        game.legal_actions(PlayerId::Two).into_iter().any(|action| {
            matches!(
                &action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == wasteland
                        && targets
                            .iter()
                            .any(|selection| selection.targets() == [Target::Permanent(land)])
            )
        })
    };
    assert!(
        !named(&game, mountain),
        "the Mountain is a Swamp and still basic",
    );
    assert!(named(&game, urborg), "Urborg is the nonbasic land here");
}
