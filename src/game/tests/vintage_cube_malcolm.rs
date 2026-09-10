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

/// Casting without paying changes the cost, not the spell's destination.
#[test]
fn a_free_spell_keeps_its_ordinary_destination() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::LIGHTNING_BOLT,
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
        ],
    );

    for connection in 1..=4 {
        connect(&mut game, malcolm, cards::LIGHTNING_BOLT, connection == 4);
    }

    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the free Bolt resolves to the graveyard",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .all(|card| card.definition != cards::LIGHTNING_BOLT),
        "a free cost supplies no exile replacement",
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

/// "You may not play land cards discarded this way." The permission is to
/// cast, and a land is played rather than cast, so the fourth connection
/// buys nothing when what it threw away was a land.
#[test]
fn a_discarded_land_is_not_castable() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::MOUNTAIN,
            cards::MOUNTAIN,
            cards::MOUNTAIN,
            cards::MOUNTAIN,
        ],
    );

    for connection in 1..=4 {
        connect(&mut game, malcolm, cards::MOUNTAIN, connection == 4);
        assert_eq!(chorus(&game, malcolm), connection);
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MOUNTAIN),
        "no land came back",
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::MOUNTAIN)
            .count(),
        4,
        "all four discards are still in the graveyard",
    );
}

/// "If Malcolm isn't on the battlefield as its triggered ability resolves,
/// you won't put a chorus counter on it, but you'll still draw a card and
/// discard a card. You may still cast the discarded card if he had four or
/// more chorus counters when he was last on the battlefield."
#[test]
fn a_dead_malcolm_still_loots_and_still_pays() {
    let (mut game, malcolm) = staged(&[], &[cards::MOX_JET, cards::MOX_JET]);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == malcolm)
        .expect("he is there")
        .add_counters(CounterKind::named("chorus"), 4);

    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.priority = PlayerId::One;
    game.declare_attacker(malcolm, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();

    // The trigger is waiting; he is not there to receive its counter.
    game.move_permanents_to_graveyard(&[malcolm]);
    let library = game.players[PlayerId::One.index()].library.len();

    for _ in 0..24 {
        if let Some(offer) = game
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
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect();
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

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library - 1,
        "the draw happened without him",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MOX_JET),
        "and the four counters he had when he left still paid for the discard",
    );
}

/// "If the spell has {X} in its mana cost, you must choose 0 as the value of
/// X." Free is free at its smallest: the Rabbit arrives as the 1/2 it is
/// printed as, with no counters bought along the way.
#[test]
fn a_free_cast_reads_x_as_zero() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::JACKED_RABBIT,
            cards::JACKED_RABBIT,
            cards::JACKED_RABBIT,
            cards::JACKED_RABBIT,
        ],
    );

    for connection in 1..=4 {
        connect(&mut game, malcolm, cards::JACKED_RABBIT, connection == 4);
        assert_eq!(chorus(&game, malcolm), connection);
    }

    let rabbit = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::JACKED_RABBIT))
        .expect("the fourth discard was cast for nothing");
    assert_eq!(
        (game.power(rabbit), game.toughness(rabbit)),
        (Some(1), Some(2)),
        "X was zero, so Ravenous brought no counters with it",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.total(),
        0,
        "and nothing was paid for any of it",
    );
}

/// "You cast the spell as part of the resolution of the ability. You can't
/// wait to cast the spell later in the turn." Once the offer is declined the
/// card is an ordinary card in the graveyard, however much mana is lying
/// around afterwards.
#[test]
fn a_declined_offer_does_not_keep_until_later_in_the_turn() {
    let (mut game, malcolm) = staged(
        &[],
        // Drawn from the back, so the Bolt is the fourth card and the
        // offer is made on it.
        &[
            cards::LIGHTNING_BOLT,
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
        ],
    );

    for _ in 0..4 {
        connect(&mut game, malcolm, cards::LIGHTNING_BOLT, false);
    }
    assert_eq!(chorus(&game, malcolm), 4);
    let discarded = game.players[PlayerId::One.index()]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::LIGHTNING_BOLT)
        .expect("the fourth discard went to the graveyard")
        .id;

    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == discarded)),
        "the permission ended with the ability that gave it",
    );
}

/// "If you cast a card 'without paying its mana cost' ... if the card has
/// any mandatory additional costs, you must pay those to cast the card."
/// Bone Shards wants a creature sacrificed or a card discarded whatever else
/// is paying for it. Free covers the mana and nothing more, and the only
/// creature on your side is Malcolm -- so the spell eats its own enabler and
/// is left pointing at nothing.
#[test]
fn a_free_cast_still_pays_a_mandatory_additional_cost() {
    let (mut game, malcolm) = staged(
        &[cards::ISLAND],
        &[
            cards::BONE_SHARDS,
            cards::MOX_JET,
            cards::MOX_JET,
            cards::MOX_JET,
        ],
    );
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    for connection in 1..=3 {
        connect(&mut game, malcolm, cards::MOX_JET, false);
        assert_eq!(chorus(&game, malcolm), connection);
    }
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == malcolm),
        "he is still here to be spent",
    );

    connect(&mut game, malcolm, cards::BONE_SHARDS, true);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != malcolm),
        "the free cast still demanded a sacrifice and Malcolm was it",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MALCOLM_ALLURING_SCOUNDREL),
        "sacrificed rather than merely gone",
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::ISLAND],
        "the Island was never the price: a creature was",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.total(),
        0,
        "and no mana was spent, which is what free means",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == bear),
        "the bear outlives it: the spell paid for itself with its own target",
    );
}

/// "If you cast a card 'without paying its mana cost', you can't choose to
/// cast it for any alternative costs." Mine Collapse has one of its own --
/// sacrifice a Mountain, on your turn, which this is -- and free is already
/// the alternative being used. So the Mountain is still standing when the
/// Collapse has resolved.
#[test]
fn a_free_cast_does_not_take_the_cards_own_alternative_cost() {
    let (mut game, malcolm) = staged(
        &[],
        &[
            cards::MINE_COLLAPSE,
            cards::MINE_COLLAPSE,
            cards::MINE_COLLAPSE,
            cards::MINE_COLLAPSE,
        ],
    );
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    // Something for the five damage to land on, since a Collapse with no
    // legal target is never offered at all.
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);

    let creatures = |game: &Game| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                game.permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Creature))
            })
            .count()
    };
    assert_eq!(creatures(&game), 2, "Malcolm and a bear to point at");

    for connection in 1..=4 {
        connect(&mut game, malcolm, cards::MINE_COLLAPSE, connection == 4);
    }

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::MOUNTAIN),
        "the Mountain was never offered up: free is the alternative already",
    );
    assert_eq!(
        creatures(&game),
        1,
        "and the Collapse resolved all the same, for its five",
    );
}
