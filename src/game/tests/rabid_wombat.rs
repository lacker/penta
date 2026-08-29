//! A static bonus that counts, and then scales.
//!
//! Rabid Wombat is +2/+2 per Aura, so the layer walk has to both count the
//! Auras attached to it and multiply. The count is live: an Aura arriving or
//! falling off moves the Wombat without anything being reapplied, and an
//! Aura on something else never counts at all.

use super::*;

fn wombat_board() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let wombat = creature(10_000, cards::RABID_WOMBAT, PlayerId::One);
    let wombat_id = wombat.card.id;
    game.battlefield.push(wombat);
    (game, wombat_id)
}

fn attach(
    game: &mut Game,
    id: u32,
    aura: CardDefinitionId,
    host: GameObjectId,
    controller: PlayerId,
) -> GameObjectId {
    let mut permanent = creature(id, aura, controller);
    permanent.attached_to = Some(host);
    let aura_id = permanent.card.id;
    game.battlefield.push(permanent);
    game.check_state_based_actions();
    aura_id
}

fn stats(game: &Game, permanent: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn each_aura_is_worth_two() {
    let (mut game, wombat) = wombat_board();
    // A printed 0/1.
    assert_eq!(stats(&game, wombat), (Some(0), Some(1)));

    // Unholy Strength is itself +2/+1, so the Wombat is 0+2+2 / 1+1+2.
    attach(
        &mut game,
        10_001,
        cards::UNHOLY_STRENGTH,
        wombat,
        PlayerId::One,
    );
    assert_eq!(stats(&game, wombat), (Some(4), Some(4)));

    attach(
        &mut game,
        10_002,
        cards::HOLY_STRENGTH,
        wombat,
        PlayerId::Two,
    );
    // And Holy Strength is +1/+2, from either player's side.
    assert_eq!(stats(&game, wombat), (Some(7), Some(8)));
}

/// The count is live, so removing an Aura shrinks it again.
#[test]
fn losing_an_aura_takes_the_bonus_back() {
    let (mut game, wombat) = wombat_board();
    let aura = attach(
        &mut game,
        10_001,
        cards::UNHOLY_STRENGTH,
        wombat,
        PlayerId::One,
    );
    assert_eq!(stats(&game, wombat), (Some(4), Some(4)));

    game.battlefield
        .retain(|permanent| permanent.card.id != aura);

    assert_eq!(stats(&game, wombat), (Some(0), Some(1)));
}

/// An Aura on something else is not attached to it, so it counts for nothing.
#[test]
fn an_aura_elsewhere_does_not_count() {
    let (mut game, wombat) = wombat_board();
    let other = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);
    attach(
        &mut game,
        10_002,
        cards::UNHOLY_STRENGTH,
        other_id,
        PlayerId::One,
    );

    assert_eq!(stats(&game, wombat), (Some(0), Some(1)));
}
