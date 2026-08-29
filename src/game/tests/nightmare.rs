//! Nightmare's Swamp-count characteristic-defining ability.
//!
//! The definition is live on the battlefield and in every other zone, and
//! counts only Swamps controlled by the card's controller.

use super::*;
use crate::ImplementationStatus;

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn stats(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = permanent(game, id);
    (
        game.power(permanent).expect("Nightmare is a creature"),
        game.toughness(permanent).expect("Nightmare is a creature"),
    )
}

fn stats_in_zone(zone: ZoneKind) -> (i16, i16) {
    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..4 {
        game.battlefield
            .push(creature(11_100 + index, cards::SWAMP, PlayerId::One));
    }
    for index in 0..3 {
        game.battlefield
            .push(creature(11_200 + index, cards::SWAMP, PlayerId::Two));
    }
    card_stats_in_zone(game, 11_000, cards::NIGHTMARE, PlayerId::One, zone)
}

#[test]
fn it_counts_only_swamps_you_control_and_updates_live() {
    let mut game = ready_game();
    game.battlefield.clear();
    let nightmare = creature(11_000, cards::NIGHTMARE, PlayerId::One);
    let nightmare_id = nightmare.card.id;
    game.battlefield.push(nightmare);
    game.battlefield
        .push(creature(11_100, cards::SWAMP, PlayerId::One));
    game.battlefield
        .push(creature(11_101, cards::SWAMP, PlayerId::One));
    game.battlefield
        .push(creature(11_200, cards::SWAMP, PlayerId::Two));
    game.battlefield
        .push(creature(11_201, cards::ISLAND, PlayerId::One));

    assert_eq!(stats(&game, nightmare_id), (2, 2));

    game.battlefield
        .push(creature(11_102, cards::SWAMP, PlayerId::One));
    assert_eq!(stats(&game, nightmare_id), (3, 3));
}

#[test]
fn the_characteristic_definition_works_in_every_modeled_card_zone() {
    for zone in [
        ZoneKind::Library,
        ZoneKind::Hand,
        ZoneKind::Battlefield,
        ZoneKind::Graveyard,
        ZoneKind::Stack,
        ZoneKind::Exile,
    ] {
        assert_eq!(stats_in_zone(zone), (4, 4), "wrong stats in {zone:?}");
    }
}

#[test]
fn nightmare_reports_complete_declarative_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::NIGHTMARE)
        .expect("Nightmare is cataloged");
    assert_eq!(
        card.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
    assert!(
        card.rules
            .ability_clauses()
            .iter()
            .all(|ability| ability.declarative_effect().is_some()),
    );
}
