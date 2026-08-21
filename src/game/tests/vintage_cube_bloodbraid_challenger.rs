//! Bloodbraid Challenger, and cascade: dig for something cheaper, cast it
//! for nothing, and put the rest back underneath.

use super::*;

/// Player One holding a Challenger with `library` stacked so the last entry
/// is on top and enough mana to cast it.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
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
        .build_zone(PlayerId::One, &[cards::BLOODBRAID_CHALLENGER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let challenger = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, challenger)
}

/// Casts the Challenger and passes until something asks a question.
fn cast(game: &mut Game, challenger: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == challenger))
        .expect("five mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

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

fn decline(game: &mut Game) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("cascade offered the card it turned up");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == "Decline")
        .expect("declining is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("declining is legal");
    settle(game);
}

fn definitions(cards: &[CardInstance]) -> Vec<CardDefinitionId> {
    cards.iter().map(|card| card.definition).collect()
}

/// Cascade walks past the lands and stops on the first cheaper nonland card.
#[test]
fn it_digs_past_lands_to_the_first_cheaper_spell() {
    let (mut game, challenger) = staged(&[
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
        cards::MOUNTAIN,
        cards::FOREST,
    ]);

    cast(&mut game, challenger);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the Bolt was turned up");
    assert_eq!(
        decision
            .options
            .first()
            .and_then(|option| option.card)
            .and_then(|(_, characteristics)| characteristics.card_definition()),
        Some(cards::LIGHTNING_BOLT),
        "two lands were walked past and the Bolt is the offer",
    );
}

/// Declining leaves nothing in exile: every card the dig touched goes to the
/// bottom of the library.
#[test]
fn declining_puts_the_whole_pile_on_the_bottom() {
    let (mut game, challenger) = staged(&[
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
        cards::MOUNTAIN,
        cards::FOREST,
    ]);

    cast(&mut game, challenger);
    decline(&mut game);

    assert!(
        game.players[0].exile.is_empty(),
        "nothing stayed in exile: {:?}",
        definitions(&game.players[0].exile),
    );
    assert_eq!(
        game.players[0].library.len(),
        4,
        "all four cards are back in the library",
    );
    assert_eq!(
        game.players[0].library.last().map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "and what was under the dig is now on top",
    );
}

/// Casting the card cascade turned up costs nothing, and the rest of the
/// pile still goes home.
#[test]
fn the_turned_up_card_is_cast_for_free() {
    let (mut game, challenger) = staged(&[
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
        cards::MOUNTAIN,
        cards::FOREST,
    ]);
    cast(&mut game, challenger);
    let pool = game.players[0].mana_pool.total();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { choices, .. } => choices
                .targets()
                .iter()
                .any(|selection| selection.targets().contains(&Target::Player(PlayerId::Two))),
            _ => false,
        })
        .expect("the Bolt may be cast from exile at the opponent");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(&mut game);

    assert_eq!(
        game.players[0].mana_pool.total(),
        pool,
        "no mana was spent on it",
    );
    assert_eq!(game.players[1].life, 17, "and it resolved");
    assert!(
        game.players[0].exile.is_empty(),
        "the rest of the pile went home even though the Bolt was cast: {:?}",
        definitions(&game.players[0].exile),
    );
    assert_eq!(
        game.players[0].library.len(),
        3,
        "three cards back, the Bolt gone to a graveyard",
    );
}

/// "Costs less" is a real bound: a nonland card that costs as much as the
/// cascading spell is walked past like a land. Both a second Challenger and
/// a Serra Angel are five, and neither is less than five.
#[test]
fn a_card_costing_the_same_is_walked_past() {
    let (mut game, challenger) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::SERRA_ANGEL,
        cards::BLOODBRAID_CHALLENGER,
    ]);

    cast(&mut game, challenger);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("something was turned up");
    assert_eq!(
        decision
            .options
            .first()
            .and_then(|option| option.card)
            .and_then(|(_, characteristics)| characteristics.card_definition()),
        Some(cards::LIGHTNING_BOLT),
        "the dig went past both five-drops to the Bolt underneath them",
    );
}

/// A library with nothing cheaper in it is emptied into exile and put back:
/// no offer, no cast, no cards lost.
#[test]
fn a_library_with_nothing_cheaper_gives_it_all_back() {
    let (mut game, challenger) = staged(&[cards::MOUNTAIN, cards::FOREST, cards::PLAINS]);

    cast(&mut game, challenger);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "nothing was turned up to offer",
    );
    assert!(
        game.players[0].exile.is_empty(),
        "and nothing stayed exiled"
    );
    assert_eq!(
        game.players[0].library.len(),
        3,
        "every card is back in the library",
    );
}

/// The plain half of the card, so a cascade test is not also the first check
/// that the Challenger arrives able to attack.
#[test]
fn the_challenger_lands_with_haste() {
    let (mut game, challenger) = staged(&[cards::MOUNTAIN, cards::FOREST]);
    cast(&mut game, challenger);
    settle(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BLOODBRAID_CHALLENGER)
        .expect("it resolved onto the battlefield");
    assert!(
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste),
        "and it can attack the turn it arrives",
    );
    assert_eq!(game.power(permanent), Some(4), "a 4/3");
}

/// Escape brings it back for the same five mana plus three cards.
#[test]
fn it_escapes_from_the_graveyard() {
    let (mut game, _challenger) = staged(&[]);
    game.players[0].hand.clear();
    let cards = game
        .build_zone(
            PlayerId::One,
            &[
                cards::BLOODBRAID_CHALLENGER,
                cards::MOUNTAIN,
                cards::FOREST,
                cards::PLAINS,
            ],
        )
        .expect("cataloged");
    let challenger = cards[0].id;
    game.players[0].graveyard.extend(cards);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == challenger)),
        "five mana and three other cards in the graveyard",
    );

    game.players[0]
        .graveyard
        .retain(|card| card.id == challenger);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == challenger)),
        "and with nothing else down there it stays put",
    );
}
