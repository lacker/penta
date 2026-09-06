//! An Aura that both resizes its host and hands it keywords. The clause is
//! one applied effect with several leaves, so what needs covering is that
//! every leaf lands and that all of them stop together when the Aura goes --
//! a granted keyword outliving the Aura would be invisible to the stats.

use super::*;

/// Serra's Embrace on a Grizzly Bears player one controls.
fn embraced() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let bear = creature(55_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    let spell = card(55_001, cards::SERRA_S_EMBRACE, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 2;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the Aura is castable onto the Bears");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);
    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::SERRA_S_EMBRACE))
        .expect("the Aura resolved")
        .card
        .id;
    (game, bear_id, aura_id)
}

fn shape(game: &Game, id: GameObjectId) -> (i16, i16, bool, bool) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Bears are on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
        game.has_flying(permanent),
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance),
    )
}

#[test]
fn every_leaf_of_the_clause_lands() {
    let (game, bear, _) = embraced();
    assert_eq!(
        shape(&game, bear),
        (4, 4, true, true),
        "a 2/2 became a 4/4 with both keywords"
    );
}

#[test]
fn losing_the_aura_takes_all_of_it_back() {
    let (mut game, bear, aura) = embraced();
    game.destroy_permanent(aura);
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert_eq!(
        shape(&game, bear),
        (2, 2, false, false),
        "the stats and both keywords went together"
    );
}
