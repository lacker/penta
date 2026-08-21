//! Stormchaser's Talent: a Class that starts as an Otter, buys back a spell
//! at level 2, and makes an Otter per spell at level 3.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let talent = game
        .put_onto_battlefield(PlayerId::One, cards::STORMCHASERS_TALENT)
        .expect("cataloged");
    settle(&mut game);
    game.priority = PlayerId::One;
    (game, talent)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn otters(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_prowess(tokens::creature(
                    &["Otter"],
                    &[ManaColor::Blue, ManaColor::Red],
                    1,
                    1,
                )),
            )
        })
        .count()
}

fn level_counters(game: &Game, talent: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == talent)
        .map_or(0, |permanent| permanent.counters(CounterKind::Level))
}

/// Every way of levelling the Class that is on offer right now.
fn level_ups(game: &Game, talent: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == talent),
        )
        .collect()
}

/// Puts an instant in Player One's hand and casts it, targeting nothing.
fn cast_a_cantrip(game: &mut Game, id: u32) {
    let spell = card(30_000 + id, cards::ANCESTRAL_RECALL, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.priority = PlayerId::One;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("one blue buys an Ancestral Recall");
    game.apply(PlayerId::One, cast).expect("it is castable");
    settle(game);
}

/// A Class enters at level 1 with an Otter and no counters.
#[test]
fn it_enters_at_level_one_with_an_otter() {
    let (game, talent) = staged();

    assert_eq!(otters(&game), 1, "the entry made one");
    assert_eq!(level_counters(&game, talent), 0, "level 1 is no counters");
}

/// Level 2 is one counter, and reaching it returns a spell from the
/// graveyard.
#[test]
fn level_two_buys_back_a_spell() {
    let (mut game, talent) = staged();
    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);

    let level = level_ups(&game, talent)
        .into_iter()
        .next()
        .expect("four mana buys level 2");
    game.apply(PlayerId::One, level).expect("it levels up");
    settle(&mut game);

    assert_eq!(level_counters(&game, talent), 1, "level 2 is one counter");
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and the Bolt came back",
    );
}

/// Below level 3 a cantrip makes nothing.
#[test]
fn casting_a_spell_below_level_three_makes_no_otter() {
    let (mut game, _) = staged();
    cast_a_cantrip(&mut game, 1);

    assert_eq!(otters(&game), 1, "still just the entry Otter");
}

/// At level 3 every instant or sorcery is another Otter.
#[test]
fn level_three_makes_an_otter_per_spell() {
    let (mut game, talent) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == talent)
    {
        permanent.set_counters(CounterKind::Level, 2);
    }

    cast_a_cantrip(&mut game, 1);
    assert_eq!(otters(&game), 2, "one more");
    cast_a_cantrip(&mut game, 2);
    assert_eq!(otters(&game), 3, "and another");
}

/// A level already reached is not for sale again.
#[test]
fn a_level_is_bought_only_once() {
    let (mut game, talent) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 10);
    let before = level_ups(&game, talent).len();
    assert_eq!(before, 2, "both levels are on offer from level 1");

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == talent)
    {
        permanent.set_counters(CounterKind::Level, 2);
    }

    assert!(
        level_ups(&game, talent).is_empty(),
        "a Class at level 3 has nothing left to buy",
    );
}

/// The Otter has prowess: a noncreature spell pumps it.
#[test]
fn the_otter_has_prowess() {
    let (mut game, _) = staged();
    let otter = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                token_with_prowess(tokens::creature(
                    &["Otter"],
                    &[ManaColor::Blue, ManaColor::Red],
                    1,
                    1,
                )),
            )
        })
        .expect("the entry made one")
        .card
        .id;
    let power_of = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == otter)
            .and_then(|permanent| game.power(permanent))
    };
    assert_eq!(power_of(&game), Some(1));

    cast_a_cantrip(&mut game, 1);

    assert_eq!(power_of(&game), Some(2), "+1/+1 until end of turn");
}

/// Level 3 may be bought straight from level 1, and the Class passes through
/// level 2 on the way -- so that clause fires too.
#[test]
fn jumping_to_level_three_still_passes_through_two() {
    let (mut game, talent) = staged();
    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 6);

    let jump = level_ups(&game, talent)
        .into_iter()
        .last()
        .expect("six mana buys level 3 outright");
    game.apply(PlayerId::One, jump).expect("it levels up");
    settle(&mut game);

    assert_eq!(level_counters(&game, talent), 2, "level 3 is two counters");
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and the level-2 clause fired on the way past",
    );
}
