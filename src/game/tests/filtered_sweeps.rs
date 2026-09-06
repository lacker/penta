//! Two sweeps that reach only part of the board. Tremor's filter is a
//! negation, which is the shape that fails open when the inner predicate
//! cannot be evaluated -- a flier taking damage would be the symptom. Simoon
//! is scoped by a target instead, so it must leave the caster's own board
//! alone even though every creature is a creature.

use super::*;

#[test]
fn tremor_spares_the_fliers_and_nothing_else() {
    let mut game = ready_game();
    game.battlefield.clear();
    let tremor = card(45_000, cards::TREMOR, PlayerId::One);
    game.players[0].hand.push(tremor.clone());
    game.players[0].mana_pool.red = 1;
    game.battlefield.extend([
        // A 2/1 on each side dies; the 4/4 flier is untouched.
        creature(45_001, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(45_002, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(45_003, cards::SERRA_ANGEL, PlayerId::Two),
    ]);

    let cast = cast_action(tremor.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let damage = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| permanent.damage)
    };
    assert_eq!(
        damage(45_001),
        None,
        "the caster's own ground creature dies"
    );
    assert_eq!(damage(45_002), None, "so does the opponent's");
    assert_eq!(
        damage(45_003),
        Some(0),
        "the flier is outside the negated filter, not merely surviving it"
    );
}

#[test]
fn simoon_reaches_only_the_targeted_players_creatures() {
    let mut game = ready_game();
    game.battlefield.clear();
    let simoon = card(45_010, cards::SIMOON, PlayerId::One);
    game.players[0].hand.push(simoon.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;
    game.battlefield.extend([
        creature(45_011, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(45_012, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);

    let cast = cast_action(
        simoon.id,
        vec![Target::Player(PlayerId::Two)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let alive = |id| {
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(id))
    };
    assert!(alive(45_011), "the caster's own 2/1 was never in range");
    assert!(!alive(45_012), "the targeted player's 2/1 took the damage");
}
