//! Malcolm, Alluring Scoundrel: a two-mana flier that loots on every
//! connection and, from the fourth on, hands the loot back for free.

use super::*;

/// Malcolm on the battlefield with `library` to draw from and `hand` in
/// hand, facing an empty board.
fn staged(hand: &[CardDefinitionId], library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    for (index, definition) in library.iter().enumerate() {
        game.players[PlayerId::One.index()].library.push(card(
            87_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    for (index, definition) in hand.iter().enumerate() {
        game.players[PlayerId::One.index()].hand.push(card(
            87_100 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    let malcolm = game
        .put_onto_battlefield(PlayerId::One, cards::MALCOLM_ALLURING_SCOUNDREL)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 5;
    (game, malcolm)
}

/// Connects with the defending player and answers whatever the loot asks,
/// discarding `discard` and accepting any offer when `cast` is set.
fn connect(game: &mut Game, malcolm: GameObjectId, discard: CardDefinitionId, cast: bool) {
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.priority = PlayerId::One;
    game.declare_attacker(malcolm, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    for _ in 0..24 {
        // The offer is accepted by casting rather than by answering its
        // decision, which only carries the refusal.
        if cast
            && let Some(offer) = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .find(|action| matches!(action, Action::CastSpell { .. }))
        {
            game.apply(PlayerId::One, offer).expect("the offer stands");
            continue;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let discarding = decision.options.iter().find(|option| {
                matches!(
                    option.card,
                    Some((_, ObjectCharacteristics::Card { definition, .. })) if definition == discard
                )
            });
            let options = match discarding {
                Some(option) if decision.maximum == 1 => vec![option.id],
                _ if decision.minimum == 0 && !cast => Vec::new(),
                _ => decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .collect(),
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn chorus(game: &Game, malcolm: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == malcolm)
        .expect("he is there")
        .counters(CounterKind::named("chorus"))
}

/// Flash and flying.
#[test]
fn he_flashes_in_and_flies() {
    let (game, malcolm) = staged(&[], &[]);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == malcolm)
        .expect("he is there");

    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(2), Some(1)),
    );
}

/// One connection: a counter, a card drawn, a card discarded, and no offer.
#[test]
fn connecting_once_loots_and_offers_nothing() {
    let (mut game, malcolm) = staged(&[cards::MOX_JET], &[cards::LIGHTNING_BOLT]);

    connect(&mut game, malcolm, cards::MOX_JET, true);

    assert_eq!(chorus(&game, malcolm), 1);
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "the Bolt is drawn and the Mox is thrown away",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::MOX_JET),
        "one counter is not four, so nothing is cast",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOX_JET),
    );
}

/// The fourth connection is the one that pays: the discarded card may be
/// cast for nothing.
#[test]
fn the_fourth_connection_casts_the_discard() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
        ],
    );

    for connection in 1..=4 {
        connect(&mut game, malcolm, cards::MOX_JET, connection == 4);
        assert_eq!(chorus(&game, malcolm), connection);
    }

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MOX_JET),
        "the fourth discard comes back onto the battlefield for free",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.total(),
        0,
        "it cost nothing to cast, and nothing was there to pay with",
    );
}

/// The offer is optional: declining leaves the card in the graveyard.
#[test]
fn the_offer_may_be_declined() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
        ],
    );

    for _ in 0..4 {
        connect(&mut game, malcolm, cards::MOX_JET, false);
    }

    assert_eq!(chorus(&game, malcolm), 4);
    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::MOX_JET)
            .count(),
        4,
        "every discard stayed put",
    );
}
