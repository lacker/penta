//! Populate.
//!
//! Choose a creature token you control, then copy it. The choice is not a
//! target, so nothing is rechecked; and a board with no creature tokens is
//! not a failure, it simply does nothing.

use super::*;

fn tokens_of(game: &Game, token: TokenCharacteristics) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, token))
        .count()
}

/// Casts Rootborn Defenses, which is populate with nothing else in front of
/// it, and answers whatever choice it asks.
fn populate_with(game: &mut Game) {
    let spell = card(10_000, cards::ROOTBORN_DEFENSES, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("it can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(game);
}

#[test]
fn populate_copies_a_creature_token_you_control() {
    let mut game = ready_game();
    let token = token_permanent(
        10_001,
        tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1),
        PlayerId::One,
    );
    game.battlefield.push(token);

    populate_with(&mut game);

    assert_eq!(
        tokens_of(
            &game,
            tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
        ),
        2,
        "the chosen token was copied"
    );
}

#[test]
fn populate_preserves_the_tokens_complete_copiable_values() {
    let mut game = ready_game();
    let mut original = token_permanent(
        10_001,
        token_with_vigilance(tokens::creature(&["Knight"], &[ManaColor::White], 2, 2)),
        PlayerId::One,
    );
    original.copy_effect = Some(CopiableCharacteristics {
        base: ObjectCharacteristics::token(
            token_with_vigilance(tokens::creature(&["Knight"], &[ManaColor::White], 2, 2)),
            CardPartId::PRIMARY,
        ),
        added_types: CardTypeSet::single(CardType::Artifact),
        added_abilities: Vec::new(),
        retain_printed_subtypes: false,
        base_power_toughness: None,
        colors: None,
        added_creature_types: Vec::new(),
        no_mana_cost: false,
    });
    game.battlefield.push(original);

    populate_with(&mut game);

    let knights = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_vigilance(tokens::creature(&["Knight"], &[ManaColor::White], 2, 2)),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(knights.len(), 2);
    for knight in knights {
        assert!(
            game.permanent_types(knight)
                .is_some_and(|types| types.contains(CardType::Artifact)),
            "the copy-process type exception is copied along with the token base",
        );
        assert!(
            game.permanent_has_executable_keyword(knight, KeywordAbility::Vigilance),
            "the inline token's own rules are copied too",
        );
    }
}

/// A nontoken creature is not a candidate, however big it is.
#[test]
fn a_nontoken_creature_is_not_copied() {
    let mut game = ready_game();
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    game.battlefield.push(troll);

    populate_with(&mut game);

    assert_eq!(
        game.battlefield.len(),
        1,
        "a printed creature is not copied"
    );
    assert!(!game.battlefield[0].card.definition.is_token());
}

/// Nor is a token an opponent controls.
#[test]
fn an_opponents_token_is_not_copied() {
    let mut game = ready_game();
    let theirs = token_permanent(
        10_001,
        tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1),
        PlayerId::Two,
    );
    game.battlefield.push(theirs);

    populate_with(&mut game);

    assert_eq!(
        tokens_of(
            &game,
            tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
        ),
        1
    );
}

/// With nothing to copy the spell still resolves; the rest of its text has to
/// happen either way.
#[test]
fn nothing_to_copy_is_not_a_failure() {
    let mut game = ready_game();
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    populate_with(&mut game);

    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("still there");
    assert!(
        game.permanent_has_executable_keyword(troll, KeywordAbility::Indestructible),
        "the indestructible half still happened"
    );
}

/// A card that makes a token and then populates copies the one it just made,
/// which is the ordering the word "then" carries.
#[test]
fn making_a_token_first_gives_populate_something_to_copy() {
    let mut game = ready_game();
    let spell = card(10_000, cards::COURSERS_ACCORD, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("it can be cast");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(
        tokens_of(
            &game,
            tokens::creature(&["Centaur"], &[ManaColor::Green], 3, 3)
        ),
        2,
        "one made, then one copied from it"
    );
}
