//! Abhorrent Oculus: six cards out of the graveyard for a 5/5 flier, and a
//! body off the top every time somebody else takes a turn.

use super::*;

/// Player One holding an Oculus with `graveyard` behind it, `library`
/// stacked so the last entry is on top, and three mana up.
fn staged(graveyard: usize, library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].library.clear();
    for _ in 0..graveyard {
        let card = game
            .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].graveyard.push(card);
    }
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::ABHORRENT_OCULUS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let oculus = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, oculus)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn cast(game: &mut Game, oculus: GameObjectId) -> bool {
    let Some(action) = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == oculus))
    else {
        return false;
    };
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
    true
}

/// Answers the pending decision by naming the option for `wanted`.
fn manifest(game: &mut Game, wanted: CardDefinitionId) {
    let seat = deciding(game).expect("manifest dread asks which card goes down");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(wanted)
            })
        })
        .unwrap_or_else(|| panic!("{wanted:?} is one of the two"))
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

fn face_down(game: &Game) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.face_down)
}

/// Six cards leave the graveyard as the Oculus is cast, and none of them
/// ends up in a graveyard.
#[test]
fn casting_it_exiles_six_cards_from_your_graveyard() {
    let (mut game, oculus) = staged(6, &[]);

    assert!(cast(&mut game, oculus), "six cards pay the additional cost");

    assert!(
        game.players[0].graveyard.is_empty(),
        "the six left the graveyard: {:?}",
        game.players[0].graveyard.len(),
    );
    assert_eq!(
        game.players[0].exile.len(),
        6,
        "and were exiled rather than buried",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ABHORRENT_OCULUS),
        "with the Oculus on the battlefield",
    );
}

/// Five is not six: the cost cannot be paid at all.
#[test]
fn five_cards_is_not_enough_to_cast_it() {
    let (mut game, oculus) = staged(5, &[]);

    assert!(
        !cast(&mut game, oculus),
        "an additional cost is paid in full or not at all",
    );
}

/// The upkeep trigger manifests dread: one of the top two goes down face
/// down as a 2/2 and the other goes to the graveyard.
#[test]
fn each_opponents_upkeep_manifests_dread() {
    let (mut game, oculus) = staged(6, &[cards::MOUNTAIN, cards::SERRA_ANGEL]);
    assert!(cast(&mut game, oculus), "it casts");
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.step = Step::Upkeep;
    settle(&mut game);

    manifest(&mut game, cards::SERRA_ANGEL);

    let body = face_down(&game).expect("something went down face down");
    assert_eq!(game.power(body), Some(2), "a 2/2");
    assert_eq!(
        body.controller,
        PlayerId::One,
        "under the Oculus's controller"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "and the other of the two went to the graveyard",
    );
    assert!(game.players[0].library.is_empty(), "both came off the top");
}

/// A face-down permanent is a body and nothing else: no name, no types
/// beyond creature, no abilities.
#[test]
fn what_goes_down_is_a_body_rather_than_the_card() {
    let (mut game, oculus) = staged(6, &[cards::MOUNTAIN, cards::SERRA_ANGEL]);
    assert!(cast(&mut game, oculus), "it casts");
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.step = Step::Upkeep;
    settle(&mut game);
    manifest(&mut game, cards::SERRA_ANGEL);

    let body = face_down(&game).expect("something went down face down");
    assert!(
        !game.permanent_has_executable_keyword(body, KeywordAbility::Flying),
        "a face-down Serra Angel does not fly",
    );
    assert_eq!(
        body.card.definition,
        cards::SERRA_ANGEL,
        "though the card under it is unchanged, which is what turns it up",
    );
}

/// "Turn it face up any time for its mana cost if it's a creature card":
/// the Angel comes up for five, and comes up as an Angel.
#[test]
fn a_manifested_creature_card_turns_up_for_its_mana_cost() {
    let (mut game, oculus) = staged(6, &[cards::MOUNTAIN, cards::SERRA_ANGEL]);
    assert!(cast(&mut game, oculus), "it casts");
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.step = Step::Upkeep;
    settle(&mut game);
    manifest(&mut game, cards::SERRA_ANGEL);
    let body = face_down(&game).expect("it is face down").card.id;
    // Turning a permanent face up is a special action, so it needs
    // priority: on somebody else's upkeep that is theirs until they pass.
    game.priority = PlayerId::One;

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::TurnFaceUp { permanent } if *permanent == body)),
        "not without the mana",
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TurnFaceUp { permanent } if *permanent == body))
        .expect("five mana turns it up");
    game.apply(PlayerId::One, action).expect("it turns up");
    settle(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == body)
        .expect("it is still there");
    assert!(!angel.face_down, "face up now");
    assert_eq!(game.power(angel), Some(4), "a 4/4 Serra Angel");
    assert!(
        game.permanent_has_executable_keyword(angel, KeywordAbility::Flying),
        "with its flying back",
    );
}

/// A manifested noncreature card stays down: there is nothing to pay.
#[test]
fn a_manifested_noncreature_card_cannot_be_turned_up() {
    let (mut game, oculus) = staged(6, &[cards::SERRA_ANGEL, cards::MOUNTAIN]);
    assert!(cast(&mut game, oculus), "it casts");
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.step = Step::Upkeep;
    settle(&mut game);
    manifest(&mut game, cards::MOUNTAIN);
    let body = face_down(&game).expect("it is face down").card.id;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 8);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::TurnFaceUp { permanent } if *permanent == body)),
        "a land is not a creature card, so no mana cost turns it up",
    );
}
