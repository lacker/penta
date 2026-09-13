//! Malevolent Rumble: two mana finds a permanent, fills the graveyard with
//! the rest, and leaves the mana that makes the next spell a turn early.

use super::*;

/// Player One holding the Rumble with two mana up and `library` on top of
/// their library -- the last entry is the top card.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.players[0].graveyard.clear();
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
        .build_zone(PlayerId::One, &[cards::MALEVOLENT_RUMBLE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let held = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, held)
}

/// Casts it, taking the first card offered when `take` is set.
fn cast(game: &mut Game, held: GameObjectId, take: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it is cast");
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if take {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.maximum.min(1))
                    .collect()
            } else {
                Vec::new()
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
    drain_pending(game);
}

fn spawn(game: &Game) -> Option<&Permanent> {
    game.battlefield.iter().find(|permanent| {
        game.effective_subtypes(permanent)
            .contains(crate::card::Subtype::named("Spawn"))
    })
}

/// It takes the permanent, buries the other three, and leaves a Spawn.
#[test]
fn it_finds_a_permanent_and_buries_the_rest() {
    let (mut game, held) = staged(&[
        cards::ISLAND,
        cards::LIGHTNING_BOLT,
        cards::ANCESTRAL_RECALL,
        cards::GIANT_GROWTH,
        cards::GRIZZLY_BEARS,
    ]);

    cast(&mut game, held, true);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "the only permanent among the four is in hand",
    );
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.players[0].graveyard.len(),
        4,
        "the other three and the Rumble itself",
    );
    assert_eq!(game.players[0].library.len(), 1, "one card was never seen");

    let token = spawn(&game).expect("the Spawn is there");
    assert_eq!(game.power(token), Some(0));
    assert_eq!(game.toughness(token), Some(1));
}

/// Four spells means nothing to take, and all four are buried.
#[test]
fn nothing_takeable_buries_all_four() {
    let (mut game, held) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::LIGHTNING_BOLT,
        cards::ANCESTRAL_RECALL,
        cards::GIANT_GROWTH,
    ]);

    cast(&mut game, held, true);

    assert!(game.players[0].hand.is_empty(), "there was nothing to take");
    assert_eq!(
        game.players[0].graveyard.len(),
        5,
        "all four and the Rumble",
    );
    assert!(spawn(&game).is_some(), "the Spawn arrives regardless");
}

/// "You may": declining buries all four too.
#[test]
fn the_card_may_be_declined() {
    let (mut game, held) = staged(&[
        cards::ISLAND,
        cards::GRIZZLY_BEARS,
        cards::GRIZZLY_BEARS,
        cards::GRIZZLY_BEARS,
    ]);

    cast(&mut game, held, false);

    assert!(game.players[0].hand.is_empty(), "nothing was taken");
    assert_eq!(game.players[0].graveyard.len(), 5);
}

/// The Spawn sacrifices itself for one colourless.
#[test]
fn the_spawn_pays_for_a_spell() {
    let (mut game, held) = staged(&[
        cards::ISLAND,
        cards::GRIZZLY_BEARS,
        cards::GRIZZLY_BEARS,
        cards::GRIZZLY_BEARS,
    ]);
    cast(&mut game, held, false);
    let token = spawn(&game).expect("the Spawn is there").card.id;
    game.players[0].mana_pool = ManaPool::default();

    let add = Action::ActivateManaAbility {
        source: token,
        ability: mana_ability_for(&game, token, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, add)
        .expect("it sacrifices itself");
    drain_pending(&mut game);

    assert_eq!(game.players[0].mana_pool.colorless, 1);
    assert!(spawn(&game).is_none(), "and the token is gone");
}

/// "A permanent card" is every kind of permanent, and it is one of them.
/// A land, an artifact, an enchantment and a creature are all four on offer,
/// the pick is the caster's, and the three passed over are buried the same
/// as spells would have been.
#[test]
fn every_permanent_type_is_on_offer_and_only_one_is_taken() {
    let (mut game, held) = staged(&[
        cards::GRIZZLY_BEARS,
        cards::REST_IN_PEACE,
        cards::BLACK_LOTUS,
        cards::ISLAND,
    ]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it is cast");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the four are offered");
    let mut offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    offered.sort_unstable();
    let mut wanted = vec![
        cards::GRIZZLY_BEARS,
        cards::REST_IN_PEACE,
        cards::BLACK_LOTUS,
        cards::ISLAND,
    ];
    wanted.sort_unstable();
    assert_eq!(
        offered, wanted,
        "a creature, an enchantment, an artifact and a land"
    );
    assert_eq!(decision.maximum, 1, "and one of them is all you get");

    // Take the land, which is the one an ordinary "search for a creature"
    // would have walked past.
    let island = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
                == Some(cards::ISLAND)
        })
        .expect("the Island is one of them")
        .id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![island],
        },
    )
    .expect("taking the land is a legal answer");
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    drain_pending(&mut game);

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::ISLAND],
        "the land the caster named, and nothing else",
    );
    for definition in [
        cards::GRIZZLY_BEARS,
        cards::REST_IN_PEACE,
        cards::BLACK_LOTUS,
    ] {
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == definition),
            "{definition:?} was buried with the rest",
        );
    }
    assert!(spawn(&game).is_some(), "and the Spawn arrives as always");
}
