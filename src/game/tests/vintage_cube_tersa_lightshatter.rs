//! Tersa Lightshatter: a hasty three-drop that trades a spent hand for a
//! fresh one, and turns a full graveyard into a card each attack.

use super::*;
use crate::ObjectSetBindingIndex;

/// Tersa on the battlefield with `graveyard` behind her, ready to attack.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for (index, definition) in graveyard.iter().enumerate() {
        game.players[0].graveyard.push(card(
            94_000 + u32::try_from(index).expect("a small graveyard"),
            *definition,
            PlayerId::One,
        ));
    }
    let tersa = game
        .put_onto_battlefield(PlayerId::One, cards::TERSA_LIGHTSHATTER)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, tersa)
}

fn attack(game: &mut Game, tersa: GameObjectId) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: tersa,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("she attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    drain_pending(game);
}

/// Seven cards is a threshold: six does nothing.
#[test]
fn six_cards_is_not_enough() {
    let (mut game, tersa) = staged(&[cards::MOUNTAIN; 6]);
    let before = game.players[0].exile.len();

    attack(&mut game, tersa);

    assert_eq!(game.players[0].graveyard.len(), 6, "nothing was exiled");
    assert_eq!(game.players[0].exile.len(), before);
}

/// Seven turns it on: one card leaves the graveyard for exile.
#[test]
fn seven_cards_exiles_one_at_random() {
    let (mut game, tersa) = staged(&[cards::MOUNTAIN; 7]);

    attack(&mut game, tersa);

    assert_eq!(game.players[0].graveyard.len(), 6);
    assert_eq!(game.players[0].exile.len(), 1);
}

/// And what it exiles can be played this turn, for its own cost.
#[test]
fn the_exiled_card_can_be_played_this_turn() {
    let (mut game, tersa) = staged(&[
        cards::MOUNTAIN,
        cards::MOUNTAIN,
        cards::MOUNTAIN,
        cards::MOUNTAIN,
        cards::MOUNTAIN,
        cards::MOUNTAIN,
        cards::MOUNTAIN,
    ]);
    attack(&mut game, tersa);
    let exiled = game.players[0].exile[0].id;

    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 0;

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if card == exiled)),
        "a land in exile is played from where it lies",
    );
}

/// The permission lasts the turn and no longer.
#[test]
fn the_permission_ends_with_the_turn() {
    let (mut game, tersa) = staged(&[cards::MOUNTAIN; 7]);
    attack(&mut game, tersa);
    let exiled = game.players[0].exile[0].id;
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 0;
    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if card == exiled)),
    );

    game.turns_started[PlayerId::One.index()] += 1;
    game.players[0].lands_played_this_turn = 0;

    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .all(|action| !matches!(action, Action::PlayLand { card, .. } if card == exiled)),
        "\"this turn\" was this turn",
    );
}

#[test]
fn random_selection_is_composed_with_the_exile_operation() {
    let catalog = poc::catalog().expect("catalog builds");
    let tersa = catalog
        .get(cards::TERSA_LIGHTSHATTER)
        .expect("Tersa is cataloged");
    let effect = tersa.rules.ability_clauses()[2]
        .declarative_effect()
        .expect("the attack trigger is declarative");
    let EffectDef::Sequence([selection, exile]) = effect else {
        panic!("Tersa should compose selection and exile");
    };
    let EffectDef::SelectAtRandomFromZone {
        source, binding, ..
    } = *selection
    else {
        panic!("Tersa should select randomly before exiling");
    };
    assert_eq!(source, ZoneKind::Graveyard);
    assert_eq!(binding, ObjectSetBindingIndex::PRIMARY);
    assert_eq!(
        *exile,
        EffectDef::ExileGrantingControllerPlayThisTurn {
            object: EffectRecipientDef::objects(ObjectSetDef::Binding(binding)),
        }
    );
}

/// Her enter trigger offers up to two cards and no more.
#[test]
fn entering_offers_up_to_two_discards() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for index in 0..4 {
        game.players[0]
            .hand
            .push(card(94_100 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.put_onto_battlefield(PlayerId::One, cards::TERSA_LIGHTSHATTER)
        .expect("cataloged");
    game.finish_rules_procedure();
    pass_until_decision(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enter trigger asks which cards to pitch");

    assert_eq!((decision.minimum, decision.maximum), (0, 2));
}

/// Discarding two draws two; the hand keeps its size and changes its
/// contents.
#[test]
fn discarding_two_draws_two() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..4 {
        game.players[0]
            .hand
            .push(card(94_200 + index, cards::MOUNTAIN, PlayerId::One));
    }
    for index in 0..4 {
        game.players[0]
            .library
            .push(card(94_300 + index, cards::FOREST, PlayerId::One));
    }
    game.put_onto_battlefield(PlayerId::One, cards::TERSA_LIGHTSHATTER)
        .expect("cataloged");
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enter trigger asks which cards to pitch");
    let chosen = decision
        .options
        .iter()
        .take(2)
        .map(|option| option.id)
        .collect::<Vec<_>>();

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: chosen,
        },
    )
    .expect("pitching two is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 4, "two out, two in");
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .filter(|card| card.definition == cards::FOREST)
            .count(),
        2,
        "the two drawn are the two off the library",
    );
    assert_eq!(game.players[0].graveyard.len(), 2);
}

/// Discarding none draws none: "up to" reaches zero.
#[test]
fn discarding_none_draws_none() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for index in 0..3 {
        game.players[0]
            .hand
            .push(card(94_400 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.players[0]
        .library
        .push(card(94_500, cards::FOREST, PlayerId::One));
    game.put_onto_battlefield(PlayerId::One, cards::TERSA_LIGHTSHATTER)
        .expect("cataloged");
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enter trigger asks which cards to pitch");

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("pitching nothing is legal");
    drain_pending(&mut game);

    assert_eq!(game.players[0].hand.len(), 3, "nothing in, nothing out");
    assert_eq!(game.players[0].library.len(), 1);
    assert!(game.players[0].graveyard.is_empty());
}
