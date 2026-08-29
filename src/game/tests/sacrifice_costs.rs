//! Three spells that spend something on the way to the stack.
//!
//! An additional cost is paid as the spell is cast rather than as it
//! resolves: with nothing to spend there is nothing to cast, and what was
//! spent is already gone by the time the spell does its work. A cost naming
//! two objects enumerates every pair rather than every candidate.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

fn casts(game: &Game, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

/// No creature, no cast: the additional cost has to be payable.
#[test]
fn bone_splinters_needs_a_creature_to_eat() {
    let mut game = ready();
    let spell = card(20_000, cards::BONE_SPLINTERS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    let victim = creature(10_100, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    assert!(
        casts(&game, spell_id).is_empty(),
        "a target but nothing of yours to sacrifice",
    );

    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One));
    assert!(!casts(&game, spell_id).is_empty(), "now it can be paid");

    let action = casts(&game, spell_id)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { sacrifices, choices, .. }
                if sacrifices.contains(&GameObjectId(10_000))
                    && choices.targets().iter().flat_map(crate::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim_id)))
        })
        .expect("sacrifice the Bear, destroy the Elemental");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the Bear paid on the way up and the Elemental died on the way down",
    );
}

/// The Plunge is a sorcery, so its three red arrive as it resolves.
#[test]
fn infernal_plunge_trades_a_creature_for_three_red() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One));
    let spell = card(20_000, cards::INFERNAL_PLUNGE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    let action = casts(&game, spell_id)
        .into_iter()
        .next()
        .expect("one creature, one way to pay");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert!(game.battlefield.is_empty(), "the Bear was the cost");
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.red,
        3,
        "and three red came back",
    );
}

/// A cost naming two cards enumerates pairs, not candidates: three creature
/// cards in the graveyard is three ways to pay, not three cards to pick.
#[test]
fn the_goliath_offers_every_pair_of_creature_cards() {
    let ways_to_pay = |creatures: u32, others: u32| {
        let mut game = ready();
        let spell = card(20_000, cards::SKAAB_GOLIATH, PlayerId::One);
        let spell_id = spell.id;
        game.players[PlayerId::One.index()].hand.push(spell);
        game.players[PlayerId::One.index()].mana_pool.blue = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 5;
        for index in 0..creatures {
            game.players[PlayerId::One.index()].graveyard.push(card(
                30_000 + index,
                cards::GRIZZLY_BEARS,
                PlayerId::One,
            ));
        }
        for index in 0..others {
            game.players[PlayerId::One.index()].graveyard.push(card(
                31_000 + index,
                cards::LIGHTNING_BOLT,
                PlayerId::One,
            ));
        }
        casts(&game, spell_id).len()
    };

    assert_eq!(ways_to_pay(1, 0), 0, "one creature card cannot pay for two");
    assert_eq!(ways_to_pay(2, 0), 1, "exactly one pair");
    assert_eq!(ways_to_pay(3, 0), 3, "three pairs from three cards");
    assert_eq!(
        ways_to_pay(2, 5),
        1,
        "the instants in the graveyard are not creature cards",
    );
}

/// Paying it exiles both chosen cards, and nothing else.
#[test]
fn the_goliath_exiles_both_cards_it_names() {
    let mut game = ready();
    let spell = card(20_000, cards::SKAAB_GOLIATH, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    for index in 0..3 {
        game.players[PlayerId::One.index()].graveyard.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }

    let action = casts(&game, spell_id)
        .into_iter()
        .next()
        .expect("three pairs to choose from");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        2,
        "two exiled, no more",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "and the third stayed put",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SKAAB_GOLIATH),
        "the Goliath resolved",
    );
}
