//! Turn-long prevention that names a group of sources, and the activation
//! window that goes with the fog.
//!
//! These rules outlive the resolution that made them, so what they name has
//! to be re-read as each damage arrives: a creature that gains flying after
//! the Carpet is activated stops being covered by it. That is why the group
//! is stored rather than the list of creatures matching it at the time.

use super::*;

/// Activates the only ability the given permanent offers.
fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source: from, .. } if *from == source))
        .expect("the ability is offered");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

/// An attacker under player two, with a defender-side artifact for player one.
fn attacked_by(
    attacker: CardDefinitionId,
    artifact: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let device = creature(10_000, artifact, PlayerId::One);
    let device_id = device.card.id;
    game.battlefield.push(device);
    let mut foe = creature(10_001, attacker, PlayerId::Two);
    foe.attacking = true;
    foe.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let foe_id = foe.card.id;
    game.battlefield.push(foe);
    game.players[PlayerId::One.index()].mana_pool.colorless = 6;
    (game, device_id, foe_id)
}

#[test]
fn the_carpet_stops_the_ground_and_lets_the_air_through() {
    let (mut game, carpet, _) = attacked_by(cards::SEDGE_TROLL, cards::AL_ABARAS_CARPET);
    let mut flier = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    flier.attacking = true;
    flier.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(flier);

    let before = game.players[PlayerId::One.index()].life;
    activate(&mut game, carpet);
    game.deal_combat_damage();

    assert_eq!(
        before - game.players[PlayerId::One.index()].life,
        4,
        "the 2/2 on the ground is stopped and the 4/4 flier is not"
    );
}

/// The group is re-read as the damage arrives, so an attacker that gains
/// flying afterwards walks straight through the Carpet.
#[test]
fn gaining_flying_escapes_the_carpet() {
    let (mut game, carpet, attacker) = attacked_by(cards::SEDGE_TROLL, cards::AL_ABARAS_CARPET);
    let before = game.players[PlayerId::One.index()].life;
    activate(&mut game, carpet);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("still there")
        .temporary_keywords
        .push(KeywordAbility::Flying);

    game.deal_combat_damage();
    assert_eq!(before - game.players[PlayerId::One.index()].life, 2);
}

/// Scarecrow names the other half of the sky, and covers only its
/// controller: a creature it protects is not protected.
#[test]
fn the_scarecrow_stops_fliers_and_not_the_creatures_it_stands_by() {
    let (mut game, scarecrow, flier) = attacked_by(cards::SERRA_ANGEL, cards::SCARECROW);
    let bystander = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    let before = game.players[PlayerId::One.index()].life;
    activate(&mut game, scarecrow);
    game.deal_combat_damage();

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before,
        "the flier's damage to the player is prevented"
    );

    game.damage_target_from(Some(flier), Some(Target::Permanent(bystander_id)), 1);
    let bystander = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bystander_id)
        .expect("still there");
    assert_eq!(
        bystander.damage, 1,
        "the printed card protects its controller, not their creatures"
    );
}

/// Angus Mackenzie's window is the same one Berserk uses: open until combat
/// damage starts, then gone for the rest of the turn.
#[test]
fn angus_mackenzie_may_only_fog_before_damage() {
    let mut game = ready_game();
    // It has to have been around a turn to tap.
    game.turns_started[PlayerId::One.index()] = 1;
    let angus = creature(10_000, cards::ANGUS_MACKENZIE, PlayerId::One);
    let angus_id = angus.card.id;
    game.battlefield.push(angus);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.green = 1;
    pool.white = 1;
    pool.blue = 1;

    let offers = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == angus_id),
        )
    };

    assert_eq!(game.step, Step::PrecombatMain);
    assert!(offers(&game), "before damage");

    game.step = Step::CombatDamage;
    assert!(!offers(&game), "once damage has started");

    game.step = Step::PostcombatMain;
    assert!(!offers(&game), "and not later in the same turn either");
}
