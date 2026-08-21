//! Elite Spellbinder: the card comes back, a turn later and two mana worse.

use super::*;

/// Player One holding a Spellbinder with three mana up, and `theirs` in
/// Player Two's hand.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    for definition in theirs {
        let card = game
            .build_zone(PlayerId::Two, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[1].hand.push(card);
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::ELITE_SPELLBINDER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let spellbinder = card.id;
    game.players[0].hand.push(card);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    (game, spellbinder)
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

/// Casts the Spellbinder and lets its trigger reach the point where it asks
/// which card to take.
fn cast(game: &mut Game, spellbinder: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spellbinder))
        .expect("three mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
    // With one opponent the trigger still asks who it targets, and that
    // choice comes before the one the card is about.
    if let Some(seat) = deciding(game)
        && game
            .observe(seat)
            .decision
            .is_some_and(|decision| decision.prompt.contains("target opponent"))
    {
        let decision = game.observe(seat).decision.expect("just checked");
        let option = decision.options.first().expect("the one opponent").id;
        game.apply(
            seat,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .expect("the only answer is legal");
        settle(game);
    }
}

/// Answers the pending decision by naming `wanted`, or by taking nothing
/// when `wanted` is `None`.
fn take(game: &mut Game, wanted: Option<CardDefinitionId>) {
    let seat = deciding(game).expect("somebody is being asked");
    let decision = game.observe(seat).decision.expect("just checked");
    let options = match wanted {
        Some(definition) => vec![
            decision
                .options
                .iter()
                .find(|option| {
                    option
                        .card
                        .is_some_and(|(_, found)| found.card_definition() == Some(definition))
                })
                .unwrap_or_else(|| panic!("{definition:?} is offered"))
                .id,
        ],
        None => Vec::new(),
    };
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("the answer is legal");
    settle(game);
}

/// The exiled card as its owner now sees it, if it is there.
fn in_their_exile(game: &Game, definition: CardDefinitionId) -> Option<GameObjectId> {
    game.players[1]
        .exile
        .iter()
        .find(|card| card.definition == definition)
        .map(|card| card.id)
}

/// Hands Player Two priority with `mana` colourless available and reports
/// whether they can cast `card` from exile.
fn they_can_cast(game: &mut Game, card: GameObjectId, mana: u16) -> bool {
    game.priority = PlayerId::Two;
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, mana);
    let castable = game
        .legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card));
    game.players[1].mana_pool = ManaPool::default();
    castable
}

/// The trigger takes a card out of their hand and puts it in their exile,
/// not yours.
#[test]
fn it_exiles_a_nonland_card_from_their_hand() {
    let (mut game, spellbinder) = staged(&[cards::SERRA_ANGEL, cards::MOUNTAIN]);
    cast(&mut game, spellbinder);

    take(&mut game, Some(cards::SERRA_ANGEL));

    assert!(
        in_their_exile(&game, cards::SERRA_ANGEL).is_some(),
        "the Angel is in its owner's exile",
    );
    assert_eq!(
        game.players[1].hand.len(),
        1,
        "one card left in their hand, the Mountain",
    );
    assert!(game.players[0].exile.is_empty(), "and none of it is yours");
}

/// "A nonland card": a hand of lands is looked at and left alone.
#[test]
fn a_land_is_never_a_candidate() {
    let (mut game, spellbinder) = staged(&[cards::MOUNTAIN, cards::ISLAND]);
    cast(&mut game, spellbinder);

    assert!(
        deciding(&game).is_none(),
        "with nothing to take, nothing is asked",
    );
    assert_eq!(game.players[1].hand.len(), 2, "their lands stayed put");
}

/// "You may exile": declining is a legal answer and takes nothing.
#[test]
fn taking_nothing_is_allowed() {
    let (mut game, spellbinder) = staged(&[cards::SERRA_ANGEL]);
    cast(&mut game, spellbinder);
    let seat = deciding(&game).expect("the choice is offered");
    assert_eq!(
        game.observe(seat).decision.expect("just checked").minimum,
        0,
        "with no obligation to take one",
    );

    take(&mut game, None);

    assert_eq!(game.players[1].hand.len(), 1, "the Angel is still in hand");
    assert!(game.players[1].exile.is_empty(), "and nothing was exiled");
}

/// The owner may cast it from exile, and it costs two more than it prints.
#[test]
fn its_owner_may_cast_it_for_two_more() {
    let (mut game, spellbinder) = staged(&[cards::LIGHTNING_BOLT]);
    cast(&mut game, spellbinder);
    take(&mut game, Some(cards::LIGHTNING_BOLT));
    let bolt = in_their_exile(&game, cards::LIGHTNING_BOLT).expect("it is exiled");

    assert!(
        !they_can_cast(&mut game, bolt, 2),
        "two mana is one short of the red one plus the surcharge",
    );
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    assert!(
        they_can_cast(&mut game, bolt, 2),
        "three -- the printed one and the two more -- casts it",
    );
}

/// Only its owner: the player who exiled it never gets to cast it.
#[test]
fn the_exiler_cannot_cast_it() {
    let (mut game, spellbinder) = staged(&[cards::LIGHTNING_BOLT]);
    cast(&mut game, spellbinder);
    take(&mut game, Some(cards::LIGHTNING_BOLT));
    let bolt = in_their_exile(&game, cards::LIGHTNING_BOLT).expect("it is exiled");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt)),
        "the card is theirs to play, not yours",
    );
}

/// "For as long as that card remains exiled": the permission and its tax
/// outlive the Spellbinder, and the card does not come back when it dies.
#[test]
fn killing_it_neither_returns_the_card_nor_lifts_the_tax() {
    let (mut game, spellbinder) = staged(&[cards::LIGHTNING_BOLT]);
    cast(&mut game, spellbinder);
    take(&mut game, Some(cards::LIGHTNING_BOLT));
    let bolt = in_their_exile(&game, cards::LIGHTNING_BOLT).expect("it is exiled");
    let body = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ELITE_SPELLBINDER)
        .expect("it resolved")
        .card
        .id;

    game.move_permanents_to_graveyard(&[body]);
    settle(&mut game);

    assert!(
        in_their_exile(&game, cards::LIGHTNING_BOLT).is_some(),
        "the Bolt is still in exile",
    );
    assert!(
        !they_can_cast(&mut game, bolt, 1),
        "and still costs two more than it prints",
    );
    assert!(
        {
            game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
            they_can_cast(&mut game, bolt, 2)
        },
        "which three mana still pays",
    );
}

/// A 3/1 flier besides.
#[test]
fn it_is_a_three_one_flier() {
    let (mut game, spellbinder) = staged(&[]);
    cast(&mut game, spellbinder);

    let body = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ELITE_SPELLBINDER)
        .expect("it resolved");
    assert_eq!(game.power(body), Some(3));
    assert_eq!(game.toughness(body), Some(1));
    assert!(game.permanent_has_executable_keyword(body, KeywordAbility::Flying));
}
