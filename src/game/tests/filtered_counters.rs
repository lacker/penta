//! Counterspells that only answer part of the stack, and a sweeper that only
//! answers part of the board. Both are places where the filter is the card:
//! a Gainsay that could hit anything, or a Plague Wind that hit both sides,
//! would still look right in every other respect.

use super::*;

/// `spell` cast by player one, the active player, with `answer` in player
/// two's hand and the mana to cast it.
fn on_the_stack(spell: CardDefinitionId, answer: CardDefinitionId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let threat = card(66_000, spell, PlayerId::One);
    game.players[0].hand.push(threat.clone());
    game.players[0].mana_pool.red = 4;
    game.players[0].mana_pool.blue = 4;
    game.players[0].mana_pool.green = 4;
    game.players[0].mana_pool.colorless = 4;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == threat.id))
        .expect("the threat is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    game.apply(PlayerId::One, Action::PassPriority)
        .expect("the caster passes");

    let counter = card(66_010, answer, PlayerId::Two);
    let counter_id = counter.id;
    game.players[1].hand.push(counter);
    game.players[1].mana_pool.blue = 3;
    game.players[1].mana_pool.colorless = 3;
    (game, counter_id)
}

fn can_counter(game: &Game, counter: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == counter))
}

#[test]
fn gainsay_answers_a_blue_spell_and_not_a_red_one() {
    let (blue, counter) = on_the_stack(cards::ANCESTRAL_RECALL, cards::GAINSAY);
    assert!(
        can_counter(&blue, counter),
        "a blue spell is a legal target"
    );

    let (red, counter) = on_the_stack(cards::LIGHTNING_BOLT, cards::GAINSAY);
    assert!(
        !can_counter(&red, counter),
        "and a red one is not, so the spell cannot even be cast"
    );
}

#[test]
fn envelop_answers_a_sorcery_and_not_an_instant() {
    let (sorcery, counter) = on_the_stack(cards::WHIRLWIND, cards::ENVELOP);
    assert!(can_counter(&sorcery, counter));

    let (instant, counter) = on_the_stack(cards::LIGHTNING_BOLT, cards::ENVELOP);
    assert!(!can_counter(&instant, counter), "an instant is outside it");
}

#[test]
fn plague_wind_leaves_its_casters_board_alone() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(66_100, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(66_101, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(66_102, cards::SEDGE_TROLL, PlayerId::Two));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == GameObjectId(66_102))
        .expect("the Troll is there")
        .regeneration_shields = 1;

    let wind = card(66_110, cards::PLAGUE_WIND, PlayerId::One);
    game.players[0].hand.push(wind.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 7;
    let cast = cast_action(wind.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    game.check_state_based_actions();

    let ids: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect();
    assert_eq!(
        ids,
        vec![GameObjectId(66_100)],
        "only the caster's own creature is left, shield and all"
    );
}
