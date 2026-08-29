//! More spells cataloged for the Vintage Cube pool: the ones cast for
//! something other than their mana cost, cast a second time from the
//! graveyard, or cast with more than one mode at once.

use super::*;

/// The free cast is gated on whose turn it is, and the printed one is not.
/// A green card in hand pays for it only while someone else is the active
/// player.
#[test]
fn force_of_vigor_is_free_only_on_someone_elses_turn() {
    let free_cast_offered = |active: PlayerId| {
        let mut game = ready_game();
        game.active_player = active;
        let force = card(57_000, cards::FORCE_OF_VIGOR, PlayerId::One);
        let force_id = force.id;
        game.players[PlayerId::One.index()].hand.push(force);
        game.players[PlayerId::One.index()].hand.push(card(
            57_001,
            cards::BIRDS_OF_PARADISE,
            PlayerId::One,
        ));
        game.battlefield
            .push(creature(57_002, cards::BLACK_LOTUS, PlayerId::Two));
        game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if card == force_id && choices.costs().alternative().is_some())
        })
    };

    assert!(
        !free_cast_offered(PlayerId::One),
        "on your own turn there is no free cast, whatever is in hand",
    );
    assert!(
        free_cast_offered(PlayerId::Two),
        "on someone else's turn a green card pays for it",
    );
}

/// "Up to two" means the spell can take one, and "artifacts and/or
/// enchantments" means it does not care which.
#[test]
fn force_of_vigor_destroys_both_kinds_at_once() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.active_player = PlayerId::Two;
    let lotus = creature(57_100, cards::BLACK_LOTUS, PlayerId::Two);
    let lotus_id = lotus.card.id;
    game.battlefield.push(lotus);
    let arena = creature(57_101, cards::PHYREXIAN_ARENA, PlayerId::Two);
    let arena_id = arena.card.id;
    game.battlefield.push(arena);
    // Not an artifact or an enchantment, so never a legal target.
    let bears = creature(57_102, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let force = card(57_103, cards::FORCE_OF_VIGOR, PlayerId::One);
    let force_id = force.id;
    game.players[PlayerId::One.index()].hand.push(force);
    game.players[PlayerId::One.index()].hand.push(card(
        57_104,
        cards::BIRDS_OF_PARADISE,
        PlayerId::One,
    ));

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == force_id
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(bears_id))
                }))
        }),
        "a creature is not an artifact or an enchantment",
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == force_id
                && choices.costs().alternative().is_some()
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(lotus_id))
                        && selection.targets().contains(&Target::Permanent(arena_id))
                }))
        })
        .expect("both halves of the board can go at once");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != lotus_id && permanent.card.id != arena_id),
        "the artifact and the enchantment are both destroyed",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "and the creature is untouched",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        1,
        "the green card it spent was exiled, not discarded",
    );
}

/// Two damage, and then the same two again from the graveyard -- after which
/// the card is exiled rather than left to be flashed back twice.
#[test]
fn firebolt_burns_from_hand_and_once_more_from_the_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let bolt = card(73_000, cards::FIREBOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.players[0].mana_pool.red = 1;
    let start = game.players[PlayerId::Two.index()].life;

    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("it is cast from hand");
    drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].life, start - 2);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FIREBOLT),
        "and it lands in the graveyard, where the flashback lives",
    );

    let from_graveyard = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::FIREBOLT)
        .expect("still there")
        .id;
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 4;
    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == from_graveyard
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Player(PlayerId::Two))
                }))
        })
        .expect("flashback is offered from the graveyard");
    game.apply(PlayerId::One, flashback).expect("it is cast");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::Two.index()].life, start - 4);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::FIREBOLT),
        "flashback exiles it rather than returning it",
    );
    assert_eq!(game.players[0].exile.len(), 1);
}

/// The chain is the opponent's to continue. Unlike Chain of Vapor, passing it
/// on costs nothing -- so what stops it is a player choosing to stop it, or
/// running out of cards to lose.
#[test]
fn chain_of_smog_discards_two_and_offers_the_chain_back() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()].hand.clear();
    for (instance, definition) in [
        (75_000, cards::LIGHTNING_BOLT),
        (75_001, cards::SERRA_ANGEL),
        (75_002, cards::FOREST),
    ] {
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(instance, definition, PlayerId::Two));
    }

    let chain = card(75_100, cards::CHAIN_OF_SMOG, PlayerId::One);
    let chain_id = chain.id;
    game.players[PlayerId::One.index()].hand.push(chain);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("it can name a player");

    // The discard is theirs to choose, so answer it for them.
    for _ in 0..8 {
        let Some(decision) = game.observe(PlayerId::Two).decision else {
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
            continue;
        };
        if decision.prompt.contains("copy") {
            // The chain offer: this is what the test came for.
            assert_eq!(
                game.players[PlayerId::Two.index()].hand.len(),
                1,
                "two cards went first",
            );
            assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
            return;
        }
        let chosen = decision
            .options
            .iter()
            .take(decision.minimum)
            .map(|option| option.id)
            .collect::<Vec<_>>();
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: decision.id,
                options: chosen,
            },
        )
        .expect("the discard choice is legal");
    }
    panic!("the chain was never offered back to the player who was hit");
}

/// The free cast is gated twice: on a Swamp, and on having the life to
/// spend. Both are checked before the option is offered rather than at
/// resolution, so an unpayable alternative never appears as a legal action.
#[test]
fn snuff_out_is_free_only_with_a_swamp_and_the_life_to_pay() {
    let free_offered = |swamp: bool, life: i16| {
        let mut game = ready_game();
        game.battlefield.clear();
        if swamp {
            game.battlefield
                .push(creature(79_000, cards::SWAMP, PlayerId::One));
        }
        game.players[PlayerId::One.index()].life = life;
        let snuff = card(79_001, cards::SNUFF_OUT, PlayerId::One);
        let snuff_id = snuff.id;
        game.players[PlayerId::One.index()].hand.push(snuff);
        game.battlefield
            .push(creature(79_002, cards::GRIZZLY_BEARS, PlayerId::Two));
        game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if card == snuff_id && choices.costs().alternative().is_some())
        })
    };

    assert!(
        free_offered(true, 20),
        "a Swamp and twenty life pays for it"
    );
    assert!(!free_offered(false, 20), "no Swamp, no free cast");
    // CR 118.4: life may be paid when the total is at least the amount, so
    // exactly four is payable and takes its controller to zero.
    assert!(free_offered(true, 4), "four life can pay four");
    assert!(
        !free_offered(true, 3),
        "and three cannot, so the option is not offered at all",
    );
}

/// Casting it for free costs the four life and kills what it names -- and it
/// will not name a black creature.
#[test]
fn snuff_out_pays_four_life_and_destroys_a_nonblack_creature() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(79_100, cards::SWAMP, PlayerId::One));
    let bears = creature(79_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let djinn = creature(79_102, cards::JUZAM_DJINN, PlayerId::Two);
    let djinn_id = djinn.card.id;
    game.battlefield.push(djinn);
    game.players[PlayerId::One.index()].life = 20;

    let snuff = card(79_103, cards::SNUFF_OUT, PlayerId::One);
    let snuff_id = snuff.id;
    game.players[PlayerId::One.index()].hand.push(snuff);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == snuff_id
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(djinn_id))
                }))
        }),
        "a black creature is not a legal target",
    );

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == snuff_id
                && choices.costs().alternative().is_some()
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(bears_id))
                }))
        })
        .expect("the free cast can name the green creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        16,
        "the life is paid as the spell is cast",
    );
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != bears_id),
        "and the creature is destroyed",
    );
}

/// "Pay X life" is a cost, so the casts on offer stop at the life its caster
/// actually has. Paying none is always available.
#[test]
fn toxic_deluge_is_offered_for_as_much_life_as_you_have() {
    for life in [3_i16, 20] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].life = life;
        let deluge = card(77_000, cards::TOXIC_DELUGE, PlayerId::One);
        let deluge_id = deluge.id;
        game.players[0].hand.push(deluge);
        game.players[0].mana_pool.black = 1;
        game.players[0].mana_pool.colorless = 2;

        let mut offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == deluge_id => Some(choices.x()),
                _ => None,
            })
            .collect::<Vec<_>>();
        offered.sort_unstable();

        assert_eq!(
            offered,
            (0..=u16::try_from(life).unwrap()).collect::<Vec<_>>(),
            "with {life} life",
        );
    }
}

#[test]
fn spell_life_and_mana_source_life_share_one_cast_budget() {
    let mut game = ready_game();
    game.battlefield.clear();
    let sources = (0..3)
        .map(|_| {
            game.put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
                .expect("cataloged")
        })
        .collect::<Vec<_>>();
    let deluge = card(77_005, cards::TOXIC_DELUGE, PlayerId::One);
    let deluge_id = deluge.id;
    game.players[PlayerId::One.index()].hand.push(deluge);

    game.players[PlayerId::One.index()].life = 4;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == deluge_id && choices.x() == 1)
        })
        .expect("one spell life plus three mana-source life is affordable at four");
    let preview = game.mana_sources_for_action(PlayerId::One, &cast);
    assert_eq!(preview.len(), sources.len());
    assert!(sources.iter().all(|source| preview.contains(source)));

    game.players[PlayerId::One.index()].life = 3;
    assert!(
        !game.is_legal_action(PlayerId::One, &cast),
        "the spell cannot reserve one life and also spend all three on mana abilities",
    );
    assert!(
        game.mana_sources_for_action(PlayerId::One, &cast)
            .is_empty(),
        "the stale action preview uses the same aggregate life budget",
    );
    assert!(game.apply(PlayerId::One, cast.clone()).is_err());
    assert_eq!(game.players[PlayerId::One.index()].life, 3);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.id == deluge_id),
        "rejection happens before the spell leaves hand",
    );
    assert!(sources.iter().all(|source| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == *source)
            .is_some_and(|permanent| !permanent.tapped)
    }));

    game.players[PlayerId::One.index()].life = 4;
    game.apply(PlayerId::One, cast)
        .expect("the same aggregate payment is legal with four life");
    assert_eq!(game.players[PlayerId::One.index()].life, 0);
}

/// The life is paid as the spell is cast, and the same X is what every
/// creature shrinks by. A creature whose toughness reaches zero dies.
#[test]
fn toxic_deluge_pays_its_life_and_shrinks_every_creature() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(77_010, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let angel = creature(77_011, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let deluge = card(77_012, cards::TOXIC_DELUGE, PlayerId::One);
    let deluge_id = deluge.id;
    game.players[0].hand.push(deluge);
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 2;
    let before = game.players[0].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == deluge_id && choices.x() == 3)
        })
        .expect("three life is affordable");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, before - 3, "the life is paid");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "a 2/2 does not survive -3/-3",
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel_id)
        .expect("a 4/4 does");
    assert_eq!(game.toughness(angel), Some(1));
}

/// The spell goes back to its owner's hand rather than the graveyard, and
/// Reprieve replaces itself.
#[test]
fn reprieve_returns_a_spell_and_draws() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = card(81_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    game.players[0].hand.push(bears);
    let reprieve = card(81_001, cards::REPRIEVE, PlayerId::Two);
    let reprieve_id = reprieve.id;
    game.players[1].hand.push(reprieve);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::White, 2);
    let held_before = game.players[1].hand.len();

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bears_id))
        .expect("the Bears are castable");
    game.apply(PlayerId::One, cast).expect("they are cast");
    let spell = game.stack.last().expect("the Bears are on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority).unwrap();

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == reprieve_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(spell))
            }
            _ => false,
        })
        .expect("Reprieve can point at a spell");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.stack.is_empty(),
        "the Bears left the stack with Reprieve",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "and went back to their owner's hand",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "rather than to the graveyard",
    );
    // Reprieve left the hand and one card was drawn, so the count is level.
    assert_eq!(game.players[1].hand.len(), held_before);
}

/// Returning a spell is not countering it, so a spell that cannot be
/// countered goes back all the same.
#[test]
fn reprieve_answers_a_spell_that_cannot_be_countered() {
    let mut game = ready_game();
    game.battlefield.clear();
    let halfling = creature(81_010, cards::DELIGHTED_HALFLING, PlayerId::One);
    let halfling_id = halfling.card.id;
    game.battlefield.push(halfling);
    let tifa = card(81_011, cards::TIFA_LOCKHART, PlayerId::One);
    let tifa_id = tifa.id;
    game.players[0].hand.push(tifa);
    let reprieve = card(81_012, cards::REPRIEVE, PlayerId::Two);
    let reprieve_id = reprieve.id;
    game.players[1].hand.push(reprieve);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::White, 2);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: halfling_id,
            ability: mana_ability_for(&game, halfling_id, ManaColor::Green),
            color: ManaColor::Green,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for green");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tifa_id))
        .expect("a legendary spell is castable on this mana");
    game.apply(PlayerId::One, cast).expect("it is cast");
    let spell = game.stack.last().expect("Tifa is on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority).unwrap();

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == reprieve_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(spell))
            }
            _ => false,
        })
        .expect("Reprieve can point at an uncounterable spell");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(game.stack.is_empty(), "she left the stack");
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::TIFA_LOCKHART),
        "and is back in hand despite not being counterable",
    );
}

/// "Sacrifice a creature or discard a card" is one cost with two ways to pay
/// it, so both are on offer at once and each names its own object.
#[test]
fn bone_shards_offers_both_ways_to_pay() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(82_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let angel = creature(82_001, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let shards = card(82_002, cards::BONE_SHARDS, PlayerId::One);
    let shards_id = shards.id;
    game.players[0].hand.push(shards);
    let spare = card(82_003, cards::LIGHTNING_BOLT, PlayerId::One);
    let spare_id = spare.id;
    game.players[0].hand.push(spare);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);

    let mut paid = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } if card == shards_id
                && choices
                    .iter_targets()
                    .any(|target| *target == Target::Permanent(angel_id)) =>
            {
                Some(sacrifices)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    paid.sort_unstable();

    assert_eq!(
        paid,
        vec![vec![bears_id], vec![spare_id]],
        "the creature on the battlefield and the card in hand are both ways to pay",
    );
}

/// Paying by discarding puts the card in the graveyard and the target is
/// destroyed all the same.
#[test]
fn bone_shards_discards_its_price_and_destroys() {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = creature(82_010, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let shards = card(82_011, cards::BONE_SHARDS, PlayerId::One);
    let shards_id = shards.id;
    game.players[0].hand.push(shards);
    let spare = card(82_012, cards::LIGHTNING_BOLT, PlayerId::One);
    let spare_id = spare.id;
    game.players[0].hand.push(spare);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card, sacrifices, ..
            } => *card == shards_id && sacrifices == &[spare_id],
            _ => false,
        })
        .expect("discarding is a way to pay");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the discarded card is in the graveyard",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel_id),
        "and the Angel is destroyed",
    );
}

/// The free cast pays both halves: one life and a blue card out of hand,
/// exiled rather than discarded.
#[test]
fn force_of_will_pays_a_life_and_exiles_a_blue_card() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = card(86_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    game.players[0].hand.push(bears);
    let force = card(86_001, cards::FORCE_OF_WILL, PlayerId::Two);
    let force_id = force.id;
    game.players[1].hand.push(force);
    let blue = card(86_002, cards::COUNTERSPELL, PlayerId::Two);
    let blue_id = blue.id;
    game.players[1].hand.push(blue);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    let life = game.players[1].life;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bears_id))
        .expect("the Bears are castable");
    game.apply(PlayerId::One, cast).expect("they are cast");
    let spell = game.stack.last().expect("the Bears are on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority).unwrap();

    // Player Two has no mana at all, so every Force cast on offer is the
    // free one.
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                *card == force_id
                    && sacrifices == &[blue_id]
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(spell))
            }
            _ => false,
        })
        .expect("the free cast exiles the blue card");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, life - 1, "one life is paid");
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::COUNTERSPELL),
        "and the blue card is exiled rather than discarded",
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::COUNTERSPELL),
        "never reaching the graveyard",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "and the spell is countered",
    );
}

/// A player with no blue card to exile has no free cast at all. The life
/// half gates nothing in practice: life may be paid down to zero (CR 118.4),
/// and a player already at zero has lost.
#[test]
fn force_of_will_needs_a_blue_card_to_exile() {
    let free_cast_offered = |blue: bool| {
        let mut game = ready_game();
        game.battlefield.clear();
        let bears = card(86_010, cards::GRIZZLY_BEARS, PlayerId::One);
        let bears_id = bears.id;
        game.players[0].hand.push(bears);
        let force = card(86_011, cards::FORCE_OF_WILL, PlayerId::Two);
        let force_id = force.id;
        game.players[1].hand.push(force);
        game.players[1].hand.push(card(
            86_012,
            if blue {
                cards::COUNTERSPELL
            } else {
                cards::GRIZZLY_BEARS
            },
            PlayerId::Two,
        ));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bears_id))
            .expect("the Bears are castable");
        game.apply(PlayerId::One, cast).expect("they are cast");
        game.apply(PlayerId::One, Action::PassPriority).unwrap();

        game.legal_actions(PlayerId::Two)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == force_id))
    };

    assert!(free_cast_offered(true), "a blue card pays for it");
    assert!(
        !free_cast_offered(false),
        "and a green card cannot be the blue one",
    );
}

/// Cast for its printed cost the card goes to the graveyard; bought back it
/// returns to hand instead, and either way the creature comes back hasty.
#[test]
fn corpse_dance_returns_itself_only_when_bought_back() {
    for bought_back in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].graveyard.clear();
        game.players[0]
            .graveyard
            .push(card(99_000, cards::GRIZZLY_BEARS, PlayerId::One));
        let dance = card(99_001, cards::CORPSE_DANCE, PlayerId::One);
        let dance_id = dance.id;
        game.players[0].hand.push(dance);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == dance_id && choices.costs().additional().is_empty() != bought_back
                }
                _ => false,
            })
            .unwrap_or_else(|| panic!("a cast with bought_back={bought_back} is offered"));
        game.apply(PlayerId::One, cast).expect("it is cast");
        pass_until_decision(&mut game);
        drain_pending(&mut game);

        let bears = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
            .expect("the creature comes back either way");
        assert!(
            game.permanent_has_executable_keyword(bears, KeywordAbility::Haste),
            "with haste",
        );

        let in_hand = game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::CORPSE_DANCE);
        let in_graveyard = game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CORPSE_DANCE);
        assert_eq!(in_hand, bought_back, "bought back means back in hand");
        assert_eq!(
            in_graveyard, !bought_back,
            "and otherwise it is spent like any other instant",
        );
    }
}

/// "You can cast a spell using flashback even if it was somehow put into
/// your graveyard without having been cast", and "the mana value of the
/// spell is determined only by its mana cost, no matter what the total cost
/// to cast it was": a milled Firebolt flashed back for five is still a
/// one-mana spell, which a Spell Blast answers by naming X=1.
#[test]
fn a_milled_firebolt_flashes_back_and_stays_a_one_drop() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].hand.clear();
    // Straight into the graveyard: it was never cast from anywhere.
    game.players[0]
        .graveyard
        .push(card(73_500, cards::FIREBOLT, PlayerId::One));
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 4;
    let blast = card(73_501, cards::SPELL_BLAST, PlayerId::Two);
    let blast_id = blast.id;
    game.players[1].hand.push(blast);
    game.players[1].mana_pool.blue = 1;
    game.players[1].mana_pool.colorless = 4;

    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == GameObjectId(73_500)
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Player(PlayerId::Two))
                }))
        })
        .expect("flashback does not ask how the card got there");
    game.apply(PlayerId::One, flashback).expect("it is cast");
    let on_stack = game.stack.last().expect("it is on the stack").id;
    game.priority = PlayerId::Two;

    let answers = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == blast_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(on_stack)) =>
            {
                Some(choices.x())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        answers,
        vec![1],
        "one, from the printed cost: the five it was paid for is not its size",
    );
}
