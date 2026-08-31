//! Spree: each chosen mode brings its own additional cost, targets, and
//! instruction while the resulting spell remains one modal stack object.

use super::*;

fn cast_with_modes(game: &Game, card: GameObjectId, modes: &[ModeId]) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } => *candidate == card && choices.modes() == modes,
            _ => false,
        })
}

#[test]
fn spree_charges_for_each_selected_mode() {
    let mut game = ready_game();
    let derailment = card(97_000, cards::EXPLOSIVE_DERAILMENT, PlayerId::One);
    let derailment_id = derailment.id;
    game.players[0].hand.push(derailment);
    game.battlefield
        .push(creature(97_001, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(97_002, cards::BLACK_LOTUS, PlayerId::Two));
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 3;

    assert!(
        cast_with_modes(&game, derailment_id, &[ModeId(0)]).is_some(),
        "the base cost plus one {{2}} mode is affordable",
    );
    assert!(
        cast_with_modes(&game, derailment_id, &[ModeId(1)]).is_some(),
        "either individual mode costs the same",
    );
    assert!(
        cast_with_modes(&game, derailment_id, &[ModeId(0), ModeId(1)]).is_none(),
        "both modes require the base cost plus both additional costs",
    );

    game.players[0].mana_pool.colorless += 1;
    assert!(
        cast_with_modes(&game, derailment_id, &[ModeId(0), ModeId(1)]).is_some(),
        "one more mana pays the second mode",
    );
}

#[test]
fn explosive_derailment_modes_keep_independent_targets() {
    let mut game = ready_game();
    let bears = creature(97_010, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let lotus = creature(97_011, cards::BLACK_LOTUS, PlayerId::Two);
    let lotus_id = lotus.card.id;
    game.battlefield.push(lotus);
    let derailment = card(97_012, cards::EXPLOSIVE_DERAILMENT, PlayerId::One);
    let derailment_id = derailment.id;
    game.players[0].hand.push(derailment);
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } if *card == derailment_id => {
                choices.modes() == [ModeId(0), ModeId(1)]
                    && choices
                        .iter_targets()
                        .copied()
                        .eq([Target::Permanent(bears_id), Target::Permanent(lotus_id)])
            }
            _ => false,
        })
        .expect("the damage and destroy modes choose their own targets");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "four damage kills the creature",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == lotus_id),
        "the other mode destroys the artifact",
    );
}

#[test]
fn rustler_rampage_supports_player_and_permanent_targets_together() {
    let mut game = ready_game();
    let own = creature(97_020, cards::GRIZZLY_BEARS, PlayerId::One);
    let own_id = own.card.id;
    game.battlefield.push(own);
    for id in [97_021, 97_022] {
        let mut opposing = creature(id, cards::GRIZZLY_BEARS, PlayerId::Two);
        opposing.tapped = true;
        game.battlefield.push(opposing);
    }
    let rampage = card(97_023, cards::RUSTLER_RAMPAGE, PlayerId::One);
    let rampage_id = rampage.id;
    game.players[0].hand.push(rampage);
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } if *card == rampage_id => {
                choices.modes() == [ModeId(0), ModeId(1)]
                    && choices
                        .iter_targets()
                        .copied()
                        .eq([Target::Player(PlayerId::Two), Target::Permanent(own_id)])
            }
            _ => false,
        })
        .expect("the modes accept a player and a creature target");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .all(|permanent| !permanent.tapped),
        "all creatures controlled by the targeted player untap",
    );
    let own = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == own_id)
        .expect("the targeted creature remains");
    assert!(
        game.permanent_has_executable_keyword(own, KeywordAbility::DoubleStrike),
        "the independently targeted creature gains double strike",
    );
}

#[test]
fn dance_counts_the_land_fetched_by_its_earlier_mode() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    for _ in 0..2 {
        game.put_onto_battlefield(PlayerId::One, cards::FOREST)
            .expect("cataloged land");
    }
    game.players[0]
        .library
        .push(card(97_030, cards::MOUNTAIN, PlayerId::One));
    let dance = card(97_031, cards::DANCE_OF_THE_TUMBLEWEEDS, PlayerId::One);
    let dance_id = dance.id;
    game.players[0].hand.push(dance);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 5;

    let action = cast_with_modes(&game, dance_id, &[ModeId(0), ModeId(1)])
        .expect("both Dance modes are affordable");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| game
                .effective_rules(permanent)
                .is_some_and(|rules| rules.has_type(CardType::Land)))
            .count(),
        3,
        "the first mode put the searched land onto the battlefield",
    );
    let elemental = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition.is_token()
                && Game::effective_rules_source(permanent)
                    .token_characteristics()
                    .is_some_and(|token| token.name() == "Elemental")
        })
        .expect("the second mode created its Elemental");
    assert_eq!(game.power(elemental), Some(3));
    assert_eq!(
        game.toughness(elemental),
        Some(3),
        "the token includes the land fetched by the earlier printed mode",
    );
}
