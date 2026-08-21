//! Exhume: two mana that reanimates for everybody, which is only a deal
//! when your graveyard is the better one.

use super::*;

/// Player One holding an Exhume with two mana up, and `mine`/`theirs` in the
/// two graveyards.
fn staged(mine: &[CardDefinitionId], theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    for (seat, definitions) in [(PlayerId::One, mine), (PlayerId::Two, theirs)] {
        for definition in definitions {
            let card = game
                .build_zone(seat, &[*definition])
                .expect("cataloged")
                .into_iter()
                .next()
                .expect("one card");
            game.players[seat.index()].graveyard.push(card);
        }
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::EXHUME])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let exhume = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, exhume)
}

/// Passes until somebody is asked something, or the stack is quiet.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
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

fn cast(game: &mut Game, exhume: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == exhume))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

/// Whoever is being asked, and what they are being offered.
fn pending(game: &Game) -> Option<(PlayerId, Vec<CardDefinitionId>)> {
    let seat = game.decision_player()?;
    game.observe(seat).decision.map(|decision| {
        (
            seat,
            decision
                .options
                .iter()
                .filter_map(|option| {
                    option
                        .card
                        .and_then(|(_, characteristics)| characteristics.card_definition())
                })
                .collect(),
        )
    })
}

/// Answers the pending decision by naming the card of `wanted`.
fn take(game: &mut Game, wanted: CardDefinitionId) {
    let (seat, _) = pending(game).expect("somebody is being asked");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(wanted)
            })
        })
        .unwrap_or_else(|| panic!("{wanted:?} is offered"))
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

fn controller_of(game: &Game, definition: CardDefinitionId) -> Option<PlayerId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .map(|permanent| permanent.controller)
}

/// Both graveyards give one up, and each arrives under its own owner.
#[test]
fn each_player_reanimates_their_own() {
    let (mut game, exhume) = staged(&[cards::SERRA_ANGEL], &[cards::SAVANNAH_LIONS]);

    cast(&mut game, exhume);
    take(&mut game, cards::SERRA_ANGEL);
    take(&mut game, cards::SAVANNAH_LIONS);

    assert_eq!(
        controller_of(&game, cards::SERRA_ANGEL),
        Some(PlayerId::One),
        "the caster's Angel came back under the caster",
    );
    assert_eq!(
        controller_of(&game, cards::SAVANNAH_LIONS),
        Some(PlayerId::Two),
        "and their Lions under them, which is the cost of the card",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|card| card.definition == cards::EXHUME),
        "only the Exhume is left in the caster's graveyard",
    );
}

/// Nobody reaches across: each player is offered their own graveyard alone.
#[test]
fn nobody_is_offered_the_other_graveyard() {
    let (mut game, exhume) = staged(&[cards::SERRA_ANGEL], &[cards::SAVANNAH_LIONS]);

    cast(&mut game, exhume);

    let (seat, offered) = pending(&game).expect("somebody is being asked");
    let (mine, theirs) = if seat == PlayerId::One {
        (cards::SERRA_ANGEL, cards::SAVANNAH_LIONS)
    } else {
        (cards::SAVANNAH_LIONS, cards::SERRA_ANGEL)
    };
    assert!(offered.contains(&mine), "their own is offered: {offered:?}");
    assert!(
        !offered.contains(&theirs),
        "the other graveyard is not: {offered:?}",
    );
}

/// A player with nothing to bring back is never asked, and the other still
/// gets theirs.
#[test]
fn an_empty_graveyard_is_never_asked() {
    let (mut game, exhume) = staged(&[cards::SERRA_ANGEL], &[]);

    cast(&mut game, exhume);
    take(&mut game, cards::SERRA_ANGEL);

    assert!(pending(&game).is_none(), "nobody else was asked anything");
    assert_eq!(
        controller_of(&game, cards::SERRA_ANGEL),
        Some(PlayerId::One),
        "and the one creature came back",
    );
}

/// "A creature card": a land in the graveyard is not one, and a graveyard
/// with only lands is the same as an empty one.
#[test]
fn only_creature_cards_are_offered() {
    let (mut game, exhume) = staged(&[cards::SERRA_ANGEL, cards::MOUNTAIN], &[cards::PLAINS]);

    cast(&mut game, exhume);

    let (seat, offered) = pending(&game).expect("the caster is asked");
    assert_eq!(seat, PlayerId::One, "the player with a creature card");
    assert_eq!(
        offered,
        vec![cards::SERRA_ANGEL],
        "the Angel and nothing else",
    );

    take(&mut game, cards::SERRA_ANGEL);
    assert!(
        pending(&game).is_none(),
        "and a graveyard of lands is asked nothing",
    );
    assert_eq!(
        game.players[1].graveyard.len(),
        1,
        "their Plains stayed where it was",
    );
}

/// It is not a "may": with exactly one creature card down there, that card
/// is the only answer and there is no way to refuse it.
#[test]
fn the_only_creature_card_is_the_only_answer() {
    let (mut game, exhume) = staged(&[cards::SERRA_ANGEL], &[]);

    cast(&mut game, exhume);

    let (seat, offered) = pending(&game).expect("the caster is asked");
    let decision = game.observe(seat).decision.expect("just checked");
    assert_eq!(
        offered,
        vec![cards::SERRA_ANGEL],
        "one candidate, and it is the only answer",
    );
    assert_eq!(decision.minimum, 1, "with no way to name none of them");
    take(&mut game, cards::SERRA_ANGEL);
    assert_eq!(
        controller_of(&game, cards::SERRA_ANGEL),
        Some(PlayerId::One),
        "which is mandatory rather than optional",
    );
}
