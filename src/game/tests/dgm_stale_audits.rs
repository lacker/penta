//! Four Dragon's Maze cards whose audit lines named machinery built since.
//!
//! Extort moved into the shared ability helpers so the Pontiff can grant it,
//! and Emmara's shield is the first prevention installed on a whole group
//! rather than on one permanent.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

/// The Krasis evolves off a bigger arrival and ignores a smaller one.
#[test]
fn the_krasis_evolves_off_a_bigger_creature() {
    let mut game = ready();
    let krasis = creature(10_000, cards::BATTERING_KRASIS, PlayerId::One);
    let krasis_id = krasis.card.id;
    game.battlefield.push(krasis);

    for (index, definition) in [cards::GRIZZLY_BEARS, cards::AIR_ELEMENTAL]
        .into_iter()
        .enumerate()
    {
        game.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent: creature(
                10_100 + u32::try_from(index).expect("a short list"),
                definition,
                PlayerId::One,
            ),
            from: ZoneKind::Hand,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
        drain_pending(&mut game);
    }

    // The 2/2 has greater toughness than a 2/1, and the 4/4 is greater on
    // both, so each arrival is worth one counter.
    assert_eq!(counters(&game, krasis_id), 2);
}

/// Counts how many extort payments one spell offers with the given board.
fn extort_offers(others: usize) -> usize {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::PONTIFF_OF_BLIGHT, PlayerId::One));
    for index in 0..others {
        game.battlefield.push(creature(
            10_100 + u32::try_from(index).expect("a short list"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }

    let spell = card(20_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("two mana covers a bear");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");

    // Each instance is its own trigger, and each resolves into its own
    // payment offer. Decline them one at a time and count how many the one
    // spell owed; anything else waiting (the ordering decision) takes its
    // first option.
    let mut offers = 0;
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            if decision.prompt.starts_with("Extort") {
                offers += 1;
            }
            let take = decision.minimum.max(1).min(decision.maximum);
            let chosen = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: chosen,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    offers
}

/// "If a creature has multiple instances of extort, each triggers separately",
/// so one spell owes one offer per creature plus the Pontiff's own.
#[test]
fn the_pontiff_grants_one_extort_instance_each() {
    assert_eq!(extort_offers(0), 1, "the Pontiff's printed instance");
    assert_eq!(extort_offers(2), 3, "and one for each other creature");
}

/// Emmara covers tokens and nothing else, including herself.
#[test]
fn emmara_shields_creature_tokens_only() {
    let mut game = ready();
    let emmara = creature(10_000, cards::EMMARA_TANDRIS, PlayerId::One);
    let emmara_id = emmara.card.id;
    game.battlefield.push(emmara);
    let token = token_permanent(
        10_100,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
        PlayerId::One,
    );
    let token_id = token.card.id;
    game.battlefield.push(token);
    let bear = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    for id in [token_id, bear_id, emmara_id] {
        game.damage_target_from(None, Some(Target::Permanent(id)), 1);
    }
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == token_id),
        "a 1/1 token survived a point it would otherwise die to",
    );
    let marked = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .damage
    };
    assert_eq!(marked(bear_id), 1, "a creature card takes its damage");
    assert_eq!(marked(emmara_id), 1, "and Emmara is a card too");
}

/// All damage, not only combat damage -- the token above took a noncombat
/// point, and a token entering after Emmara is covered as well.
#[test]
fn emmara_covers_a_token_that_arrives_later() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::EMMARA_TANDRIS, PlayerId::One));

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: token_permanent(
            10_100,
            tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
            PlayerId::One,
        ),
        from: ZoneKind::Battlefield,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    let token_id = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
            )
        })
        .expect("it arrived")
        .card
        .id;
    game.damage_target_from_kind(None, Some(Target::Permanent(token_id)), 5, true);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == token_id),
        "the shield reaches a token Emmara did not see enter",
    );
}
