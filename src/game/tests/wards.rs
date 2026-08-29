//! The Ward cycle.
//!
//! Five Auras that grant protection from a colour, and one of them is white
//! granting protection from white. Protection makes an existing attachment
//! illegal, so without the printed "This effect doesn't remove this Aura"
//! White Ward would fall off the instant it worked.

use super::*;

/// A creature with one Ward on it, both controlled by the same player.
fn warded(ward: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut aura = creature(10_001, ward, PlayerId::One);
    aura.attached_to = Some(host_id);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    game.check_state_based_actions();
    (game, host_id, aura_id)
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

fn has_protection_from(game: &Game, id: GameObjectId, color: ManaColor) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield");
    game.permanent_has_executable_keyword(permanent, protection_keyword(color))
}

#[test]
fn a_ward_grants_protection_from_its_named_color() {
    for (ward, color) in [
        (cards::BLACK_WARD, ManaColor::Black),
        (cards::BLUE_WARD, ManaColor::Blue),
        (cards::GREEN_WARD, ManaColor::Green),
        (cards::RED_WARD, ManaColor::Red),
        (cards::WHITE_WARD, ManaColor::White),
    ] {
        let (game, host_id, aura_id) = warded(ward);
        assert!(
            has_protection_from(&game, host_id, color),
            "the enchanted creature gains the protection"
        );
        assert!(
            on_battlefield(&game, aura_id),
            "and every Ward survives granting it"
        );
    }
}

/// The exception is not a general weakening of protection. A different white
/// Aura on a creature protected from white still falls off, which is what
/// White Ward would do without the clause it prints.
#[test]
fn protection_still_removes_other_auras_of_that_color() {
    let (mut game, host_id, ward_id) = warded(cards::WHITE_WARD);
    let mut blessing = creature(10_002, cards::BLESSING, PlayerId::One);
    blessing.attached_to = Some(host_id);
    let blessing_id = blessing.card.id;
    game.battlefield.push(blessing);
    game.check_state_based_actions();

    assert!(
        !on_battlefield(&game, blessing_id),
        "Blessing is a white Aura on a creature with protection from white"
    );
    assert!(
        on_battlefield(&game, ward_id),
        "and the Ward itself is still there, which is the whole exception"
    );
}

/// The exception belongs to the Aura, not to its host: it does not make the
/// creature keep an Aura some other effect would remove.
#[test]
fn a_ward_does_not_protect_a_different_aura_from_falling_off() {
    let (mut game, host_id, ward_id) = warded(cards::BLACK_WARD);
    let mut orphan = creature(10_002, cards::BLESSING, PlayerId::One);
    orphan.attached_to = None;
    let orphan_id = orphan.card.id;
    game.battlefield.push(orphan);
    game.check_state_based_actions();

    assert!(
        !on_battlefield(&game, orphan_id),
        "an Aura attached to nothing still dies"
    );
    assert!(on_battlefield(&game, ward_id) && on_battlefield(&game, host_id));
}
