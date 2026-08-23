//! Shroud granted for a while, and shroud granted on a condition.
//!
//! Homarid Warrior buys its shroud with a tap and a skipped untap step, so
//! the three halves of one activation have to happen together. Spectral
//! Cloak's shroud is conditional instead: the condition rides on the
//! recipient, so tapping the host takes the shroud away without the Aura
//! being touched, and untapping gives it back.

use super::*;
use crate::ImplementationStatus;

fn has_shroud(game: &Game, permanent: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .is_some_and(|candidate| {
            game.permanent_has_executable_keyword(candidate, KeywordAbility::Shroud)
        })
}

/// Whether a spell that player could cast may choose this permanent.
fn can_be_targeted_by(game: &Game, permanent: GameObjectId, player: PlayerId) -> bool {
    let candidate = game
        .battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("still there");
    game.permanent_can_be_targeted_by(candidate, player, GameObjectId(9_999), false)
}

#[test]
fn homarid_warrior_pays_for_its_shroud_with_a_tap() {
    let mut game = ready_game();
    let warrior = creature(10_000, cards::HOMARID_WARRIOR, PlayerId::One);
    let warrior_id = warrior.card.id;
    game.battlefield.push(warrior);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;

    assert!(!has_shroud(&game, warrior_id));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == warrior_id))
        .expect("the ability is offered");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert!(has_shroud(&game, warrior_id), "it gained shroud");
    let warrior = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == warrior_id)
        .expect("still there");
    assert!(warrior.tapped, "and tapped itself doing it");
    assert!(
        warrior.skipped_untap_steps > 0,
        "and owes an untap step besides"
    );
}

/// Shroud stops its own controller too, which is what separates it from
/// hexproof.
#[test]
fn shroud_stops_both_players() {
    let mut game = ready_game();
    let cloak_host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = cloak_host.card.id;
    game.battlefield.push(cloak_host);
    let mut cloak = creature(10_001, cards::SPECTRAL_CLOAK, PlayerId::One);
    cloak.attached_to = Some(host_id);
    game.battlefield.push(cloak);
    game.check_state_based_actions();

    for player in [PlayerId::One, PlayerId::Two] {
        assert!(
            !can_be_targeted_by(&game, host_id, player),
            "shroud is not hexproof"
        );
    }
}

/// The condition is read live: the same Aura grants or withholds shroud
/// depending on whether the host is tapped right now.
#[test]
fn tapping_the_host_takes_the_cloaks_shroud_away() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut cloak = creature(10_001, cards::SPECTRAL_CLOAK, PlayerId::One);
    cloak.attached_to = Some(host_id);
    game.battlefield.push(cloak);
    game.check_state_based_actions();

    assert!(has_shroud(&game, host_id), "untapped, so cloaked");

    let _ = game.tap_permanent(host_id);
    assert!(!has_shroud(&game, host_id), "tapped, so exposed");

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == host_id)
        .expect("still there")
        .tapped = false;
    assert!(has_shroud(&game, host_id), "and untapping cloaks it again");
}

/// A creature the Aura is not attached to is never covered, tapped or not.
#[test]
fn the_cloak_covers_only_its_own_host() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let other = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);
    let mut cloak = creature(10_001, cards::SPECTRAL_CLOAK, PlayerId::One);
    cloak.attached_to = Some(host_id);
    game.battlefield.push(cloak);
    game.check_state_based_actions();

    assert!(has_shroud(&game, host_id));
    assert!(!has_shroud(&game, other_id));
}

#[test]
fn every_shroud_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::HOMARID_WARRIOR, cards::SPECTRAL_CLOAK] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
