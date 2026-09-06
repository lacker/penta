//! A sorcery that shrinks every creature on the battlefield. The point worth
//! covering is the word "all": the effect reaches its own controller's
//! creatures, not just the opponent's, and it kills by toughness rather than
//! by damage.

use super::*;

#[test]
fn infest_shrinks_both_sides_and_buries_the_small_creatures() {
    let mut game = ready_game();
    let infest = card(32_000, cards::INFEST, PlayerId::One);
    game.players[0].hand.push(infest.clone());
    game.players[0].mana_pool.black = 3;
    game.battlefield.extend([
        creature(32_001, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(32_002, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(32_003, cards::SERRA_ANGEL, PlayerId::Two),
    ]);

    let cast = cast_action(infest.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let body = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| {
                (
                    game.power(permanent).expect("power"),
                    game.toughness(permanent).expect("toughness"),
                )
            })
    };
    assert_eq!(body(32_001), None, "the caster's own 2/1 dies too");
    assert_eq!(body(32_002), None, "so does the opponent's");
    assert_eq!(
        body(32_003),
        Some((2, 2)),
        "a 4/4 survives at two sizes smaller"
    );
}
