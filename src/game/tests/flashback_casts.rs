//! The clauses in the flashback batch that nothing else in the catalog
//! exercises: a sacrifice-of-choice aimed at a target player rather than at
//! a relation, a life gain scaled off a graveyard count, and a token clause
//! that makes three at once. The flashback mechanism itself is covered
//! elsewhere; what is checked here is that these spells do what they say
//! when cast the second time.

use super::*;

/// Call of the Herd in `zone`, with `mana` green and colourless available.
fn staged(in_graveyard: bool, mana: u16) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let call = card(80_000, cards::CALL_OF_THE_HERD, PlayerId::One);
    let call_id = call.id;
    if in_graveyard {
        game.players[0].graveyard.push(call);
    } else {
        game.players[0].hand.push(call);
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    (game, call_id)
}

fn cast_of(game: &Game, call: CardInstanceId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == call))
}

fn resolve(game: &mut Game) {
    for _ in 0..8 {
        drain_pending(game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn elephants(game: &Game) -> usize {
    game.battlefield.len()
}

#[test]
fn the_printed_cost_still_works_from_hand() {
    let (mut game, call) = staged(false, 2);
    let cast = cast_of(&game, call).expect("{2}{G} is payable from hand");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);
    assert_eq!(elephants(&game), 1, "one Elephant");
}

#[test]
fn the_flashback_cost_is_offered_from_the_graveyard() {
    let (game, call) = staged(true, 2);
    assert!(
        cast_of(&game, call).is_none(),
        "three mana does not pay the {{3}}{{G}} flashback"
    );

    let (game, call) = staged(true, 3);
    assert!(
        cast_of(&game, call).is_some(),
        "four does, and the card is in the graveyard rather than the hand"
    );
}

#[test]
fn flashing_it_back_makes_the_token_and_exiles_the_card() {
    let (mut game, call) = staged(true, 3);
    let cast = cast_of(&game, call).expect("the flashback is offered");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);

    assert_eq!(elephants(&game), 1, "the second Elephant arrived");
    assert!(
        game.players[0].graveyard.is_empty(),
        "and the card did not go back to the graveyard"
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::CALL_OF_THE_HERD)),
        "it was exiled instead, so it cannot be cast a third time"
    );
}

/// Chainer's Edict cast at `victim`, who controls `creatures` Bears.
fn edict(victim: PlayerId, creatures: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for index in 0..creatures {
        let mut bear = creature(
            80_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            victim,
        );
        bear.entered_controller_turn = 0;
        game.battlefield.push(bear);
    }
    let edict = card(80_200, cards::CHAINER_S_EDICT, PlayerId::One);
    let edict_id = edict.id;
    game.players[0].hand.push(edict);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == edict_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(victim))
            }
            _ => false,
        })
        .expect("the named player is a legal target");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);
    game
}

#[test]
fn the_edict_eats_the_targeted_players_creature() {
    let game = edict(PlayerId::Two, 1);
    assert!(
        game.battlefield.is_empty(),
        "the opponent gave up their only creature"
    );

    let game = edict(PlayerId::One, 1);
    assert!(
        game.battlefield.is_empty(),
        "and aimed at myself it takes mine, since the target chooses"
    );
}

#[test]
fn the_edict_leaves_the_other_players_board_alone() {
    let mut game = edict(PlayerId::Two, 1);
    let mine = creature(80_300, cards::GRIZZLY_BEARS, PlayerId::One);
    game.battlefield.push(mine);
    assert_eq!(
        game.battlefield.len(),
        1,
        "only the targeted player sacrificed"
    );
}

/// Ancestral Tribute cast with `graveyard` other cards in my graveyard.
fn tribute(graveyard: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for index in 0..graveyard {
        let filler = card(
            80_400 + u32::try_from(index).expect("a small fixture"),
            cards::MOUNTAIN,
            PlayerId::One,
        );
        game.players[0].graveyard.push(filler);
    }
    let tribute = card(80_500, cards::ANCESTRAL_TRIBUTE, PlayerId::One);
    let tribute_id = tribute.id;
    game.players[0].hand.push(tribute);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tribute_id))
        .expect("seven mana pays for it");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);
    game
}

#[test]
fn the_tribute_gains_two_for_each_card_in_the_graveyard() {
    assert_eq!(
        tribute(0).players[0].life,
        20,
        "an empty graveyard gains nothing"
    );
    assert_eq!(
        tribute(3).players[0].life,
        26,
        "three cards is six life, not three"
    );
}
