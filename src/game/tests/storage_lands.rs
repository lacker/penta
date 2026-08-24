//! The five storage lands.
//!
//! One card in five colours: it enters tapped, may stay tapped as long as its
//! controller likes, banks a counter every upkeep it spends tapped, and
//! cashes the whole pile in at once. The size of the cash-in is a choice, so
//! it becomes one activation per size rather than one activation carrying an
//! unanswered question.

use super::*;
use crate::ImplementationStatus;

const STORAGE_LANDS: [(CardDefinitionId, ManaColor); 5] = [
    (cards::BOTTOMLESS_VAULT, ManaColor::Black),
    (cards::DWARVEN_HOLD, ManaColor::Red),
    (cards::HOLLOW_TREES, ManaColor::Green),
    (cards::ICATIAN_STORE, ManaColor::White),
    (cards::SAND_SILOS, ManaColor::Blue),
];

fn ready() -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

/// The land on the battlefield with `counters` storage counters, tapped or
/// not as asked.
fn land(definition: CardDefinitionId, counters: u16, tapped: bool) -> (Game, GameObjectId) {
    let mut game = ready();
    let mut permanent = creature(10_000, definition, PlayerId::One);
    permanent.entered_controller_turn = 0;
    permanent.tapped = tapped;
    let id = permanent.card.id;
    game.battlefield.push(permanent);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .set_counters(CounterKind::named("storage"), counters);
    (game, id)
}

/// Every land arrives tapped, which is what makes the first upkeep bank a
/// counter without any choice being made.
#[test]
fn every_storage_land_enters_tapped() {
    for (definition, _) in STORAGE_LANDS {
        let mut game = ready();
        game.put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("it is there");
        assert!(permanent.tapped, "{definition:?} should enter tapped");
    }
}

/// A tapped land banks; an untapped one does not.
#[test]
fn the_upkeep_trigger_reads_whether_the_land_is_tapped() {
    let counters = |game: &Game, id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .counters(CounterKind::named("storage"))
    };

    let (mut game, id) = land(cards::BOTTOMLESS_VAULT, 0, true);
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(counters(&game, id), 1, "tapped, so it banks");

    let (mut game, id) = land(cards::BOTTOMLESS_VAULT, 0, false);
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(counters(&game, id), 0, "untapped, so it does not");
}

/// Three counters means three offers -- one, two, or all three -- not one
/// offer of a fixed size and not an offer with nothing to remove.
#[test]
fn the_cash_in_offers_one_activation_per_removable_count() {
    let (game, id) = land(cards::DWARVEN_HOLD, 3, false);
    let amounts = game
        .mana_ability_activations(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .expect("still there"),
        )
        .into_iter()
        .map(|activation| activation.effect.amount)
        .collect::<Vec<_>>();

    assert_eq!(amounts, vec![1, 2, 3], "one activation per size");
}

/// With nothing banked there is nothing to cash in.
#[test]
fn an_empty_land_offers_no_cash_in() {
    let (game, id) = land(cards::DWARVEN_HOLD, 0, false);
    assert!(
        game.mana_ability_activations(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .expect("still there"),
        )
        .is_empty(),
        "removing zero counters would produce zero mana",
    );
}

/// Cashing in takes exactly the counters it charges and pays exactly that
/// many mana, in the land's own colour.
#[test]
fn cashing_in_spends_the_counters_and_pays_that_colour() {
    for (definition, color) in STORAGE_LANDS {
        let (mut game, id) = land(definition, 3, false);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateManaAbility { source, counters_removed, .. }
                        if *source == id && *counters_removed == Some(3)
                )
            })
            .expect("the largest of the three offers is there");
        game.apply(PlayerId::One, action).expect("it is activated");
        drain_pending(&mut game);

        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        assert_eq!(
            permanent.counters(CounterKind::named("storage")),
            0,
            "{definition:?} spent every counter",
        );
        assert!(permanent.tapped, "and tapped itself to do it");
        let pool = game.players[PlayerId::One.index()].mana_pool;
        assert_eq!(pool.total(), 3, "{definition:?} paid three");
        assert_eq!(pool.amount(color), 3, "{definition:?} paid its colour");
    }
}

#[test]
fn every_storage_land_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for (definition, _) in STORAGE_LANDS {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
