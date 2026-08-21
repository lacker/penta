//! Creatures whose abilities turn on how they arrived.
//!
//! Containment Priest asks whether another creature was cast; Fury asks how
//! it was paid for itself. Both read a fact about the way in rather than
//! about the permanent, which is what these tests exercise.

use super::*;

/// The Priest answers how a creature arrived, not what it is. A reanimated
/// creature is exiled; the same creature cast normally is not; and a token is
/// exempt because the card says so.
#[test]
fn the_priest_exiles_creatures_that_arrive_without_being_cast() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::CONTAINMENT_PRIEST)
        .expect("cataloged");
    drain_pending(&mut game);

    // Put onto the battlefield from a graveyard: never cast.
    game.players[PlayerId::One.index()].graveyard.push(card(
        76_000,
        cards::SERRA_ANGEL,
        PlayerId::One,
    ));
    let reanimate = card(76_001, cards::REANIMATE, PlayerId::One);
    let reanimate_id = reanimate.id;
    game.players[PlayerId::One.index()].hand.push(reanimate);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    let angel_in_graveyard = game.players[PlayerId::One.index()].graveyard[0].id;
    game.apply(
        PlayerId::One,
        cast_action(
            reanimate_id,
            vec![Target::Card(angel_in_graveyard)],
            Vec::new(),
            0,
        ),
    )
    .expect("Reanimate can name a creature card");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::SERRA_ANGEL),
        "the reanimated Angel never reached the battlefield",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "it was exiled instead",
    );

    // A token is exempt.
    game.create_token(
        PlayerId::One,
        tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3),
    );
    drain_pending(&mut game);
    assert!(
        game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3)
        )),
        "a token is not a nontoken creature",
    );
}

/// Cast normally, the same creature is untouched -- including one cast from a
/// graveyard, which still goes through the stack.
#[test]
fn the_priest_leaves_a_cast_creature_alone() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::CONTAINMENT_PRIEST)
        .expect("cataloged");
    drain_pending(&mut game);

    let angel = card(76_100, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[PlayerId::One.index()].hand.push(angel);
    game.players[PlayerId::One.index()].mana_pool.white = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.apply(
        PlayerId::One,
        cast_action(angel_id, Vec::new(), Vec::new(), 0),
    )
    .expect("the Angel is castable");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "a cast creature arrives as usual",
    );
    assert!(game.players[PlayerId::One.index()].exile.is_empty());
}

/// Four damage split as the caster likes. The division is a second question
/// after the targets, because a trigger has no cast to settle it during.
#[test]
fn fury_splits_its_four_damage_across_the_targets_it_names() {
    let mut game = ready_game();
    game.battlefield.clear();
    let first = creature(77_000, cards::SERRA_ANGEL, PlayerId::Two);
    let first_id = first.card.id;
    game.battlefield.push(first);
    let second = creature(77_001, cards::SERRA_ANGEL, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.push(second);

    game.put_onto_battlefield(PlayerId::One, cards::FURY)
        .expect("cataloged");

    let targets = loop {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            break decision;
        }
        let player = game.priority;
        assert!(
            game.apply(player, Action::PassPriority).is_ok(),
            "the enters trigger is waiting on its targets",
        );
    };
    let both = targets
        .options
        .iter()
        .filter(|option| option.label == "Serra Angel")
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(both.len(), 2, "both Angels are eligible");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targets.id,
            options: both,
        },
    )
    .expect("naming two targets is legal");

    let division = game
        .observe(PlayerId::One)
        .decision
        .expect("the split is asked once the targets are known");
    assert!(
        division.prompt.contains("divide"),
        "the follow-up is the division, not another target",
    );
    let even = division
        .options
        .iter()
        .find(|option| option.label.starts_with("2 to"))
        .expect("two and two is one of the splits")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: division.id,
            options: vec![even],
        },
    )
    .expect("the split is chosen");
    drain_pending(&mut game);

    let damage = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("a 4/4 survives two damage")
            .damage
    };
    assert_eq!(
        (damage(first_id), damage(second_id)),
        (2, 2),
        "four damage over two targets is two apiece",
    );
}

/// Evoked, it still burns and then goes -- the sacrifice is a trigger
/// alongside the damage, not instead of it. Hard cast, it stays.
#[test]
fn an_evoked_fury_burns_and_then_sacrifices_itself() {
    for evoked in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        let bears = creature(77_100, cards::GRIZZLY_BEARS, PlayerId::Two);
        let bears_id = bears.card.id;
        game.battlefield.push(bears);

        let fury = card(77_101, cards::FURY, PlayerId::One);
        let fury_id = fury.id;
        game.players[PlayerId::One.index()].hand.push(fury);
        if evoked {
            game.players[PlayerId::One.index()].hand.push(card(
                77_102,
                cards::LIGHTNING_BOLT,
                PlayerId::One,
            ));
        } else {
            let pool = &mut game.players[PlayerId::One.index()].mana_pool;
            pool.red = 2;
            pool.colorless = 3;
        }

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                    if *card == fury_id && choices.costs().alternative().is_some() == evoked)
            })
            .unwrap_or_else(|| panic!("Fury is castable (evoked: {evoked})"));
        game.apply(PlayerId::One, cast).expect("it is cast");

        // Answer the damage trigger at the only creature on the board.
        for _ in 0..12 {
            if let Some(decision) = game.observe(PlayerId::One).decision {
                // Ordering asks for every trigger; a target or a split asks
                // for as many as it says.
                let chosen = decision
                    .options
                    .iter()
                    .take(decision.minimum.max(1))
                    .map(|option| option.id)
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

        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != bears_id),
            "a 2/2 takes four and dies either way (evoked: {evoked})",
        );
        assert_eq!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::FURY),
            !evoked,
            "an evoked Fury sacrifices itself; a hard-cast one stays",
        );
    }
}
