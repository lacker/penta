//! The spells paid for by returning lands, and the one paid for by exiling a
//! card.
//!
//! What separates these from an ordinary additional cost is where the spent
//! objects end up: the lands come back to hand rather than dying, and
//! Pyrokinesis exiles rather than discards. Each test checks the destination,
//! not merely that something was spent.

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
    game.players[PlayerId::One.index()].graveyard.clear();
    game
}

fn settle(game: &mut Game) {
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn cast_includes_mana_payment(game: &Game, action: &Action) -> bool {
    let Action::CastSpell {
        card,
        choices,
        sacrifices,
    } = action
    else {
        panic!("expected cast")
    };
    let (signature, _, source_zone) = game
        .validated_cast_signature(PlayerId::One, *card, choices, sacrifices)
        .expect("the offered cast validates");
    game.cast_object_payments_and_life(
        PlayerId::One,
        *card,
        &signature,
        CastCostContext {
            source_zone,
            offer: None,
        },
        sacrifices,
    )
    .2
}

#[test]
fn mercadian_masques_legates_cast_without_a_mana_payment() {
    for (spell, own_land, opposing_land) in [
        (cards::CHO_ARRIM_LEGATE, cards::PLAINS, cards::SWAMP),
        (cards::SAPRAZZAN_LEGATE, cards::ISLAND, cards::MOUNTAIN),
        (cards::DEEPWOOD_LEGATE, cards::SWAMP, cards::FOREST),
        (cards::KYREN_LEGATE, cards::MOUNTAIN, cards::PLAINS),
        (cards::RUSHWOOD_LEGATE, cards::FOREST, cards::ISLAND),
    ] {
        let mut game = ready();
        game.battlefield
            .push(creature(19_998, own_land, PlayerId::One));
        game.battlefield
            .push(creature(19_999, opposing_land, PlayerId::Two));
        let legate = card(20_000, spell, PlayerId::One);
        let legate_id = legate.id;
        game.players[PlayerId::One.index()].hand.push(legate);

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                    if *card == legate_id && choices.costs().alternative().is_some())
            })
            .expect("the matching lands offer the Legate alternative");

        assert!(
            !cast_includes_mana_payment(&game, &cast),
            "the Legate cast has no mana-payment component",
        );
    }
}

/// Islands on the battlefield, and the free cast of `spell` if one is
/// offered. A counterspell needs something to point at, so the stack always
/// holds one.
fn free_cast(spell: CardDefinitionId, islands: usize) -> (Game, Option<Action>) {
    free_cast_over(spell, &vec![cards::ISLAND; islands])
}

/// The same, with the lands named rather than counted: what the cost asks
/// for is the Island type, and a card can carry that without being one.
fn free_cast_over(spell: CardDefinitionId, lands: &[CardDefinitionId]) -> (Game, Option<Action>) {
    let mut game = ready();
    game.stack.push(crate::game::tests::spell(
        21_000,
        cards::GRIZZLY_BEARS,
        PlayerId::Two,
        0,
    ));
    for (index, definition) in lands.iter().enumerate() {
        game.battlefield.push(creature(
            10_000 + u32::try_from(index).expect("small"),
            *definition,
            PlayerId::One,
        ));
    }
    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell_id && choices.costs().alternative().is_some())
        });
    (game, cast)
}

#[test]
fn gush_is_free_with_two_islands() {
    let (_, cast) = free_cast(cards::GUSH, 2);
    assert!(cast.is_some(), "two Islands pay for it with no mana at all");
}

#[test]
fn gush_needs_both_islands() {
    let (_, cast) = free_cast(cards::GUSH, 1);
    assert!(cast.is_none(), "one Island is not two");
}

/// The lands come back rather than dying, which is the whole difference
/// between this cost and a sacrifice.
#[test]
fn gush_returns_its_islands_to_hand() {
    let (mut game, cast) = free_cast(cards::GUSH, 2);
    game.apply(PlayerId::One, cast.expect("the free cast is offered"))
        .expect("it is cast");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ISLAND),
        "both Islands left the battlefield",
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::ISLAND)
            .count(),
        2,
        "and both are in hand, not in the graveyard",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .all(|card| { card.definition != cards::ISLAND }),
        "nothing was sacrificed",
    );
}

#[test]
fn thwart_wants_three_islands() {
    assert!(free_cast(cards::THWART, 2).1.is_none(), "two is not three");
    assert!(free_cast(cards::THWART, 3).1.is_some(), "three pays for it");
}

#[test]
fn daze_wants_one_island() {
    assert!(
        free_cast(cards::DAZE, 0).1.is_none(),
        "no Island, no free Daze"
    );
    assert!(
        free_cast(cards::DAZE, 1).1.is_some(),
        "one Island is enough"
    );
}

/// Pyrokinesis exiles the red card it spends: it never becomes a graveyard
/// card, which matters to everything that reads a graveyard.
#[test]
fn pyrokinesis_exiles_the_card_it_spends() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    let pyro = card(20_000, cards::PYROKINESIS, PlayerId::One);
    let pyro_id = pyro.id;
    game.players[PlayerId::One.index()].hand.push(pyro);
    game.players[PlayerId::One.index()].hand.push(card(
        20_001,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == pyro_id && choices.costs().alternative().is_some())
        })
        .expect("a red card in hand pays for it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        1,
        "the red card was exiled",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::LIGHTNING_BOLT),
        "and not discarded",
    );
}

/// With no red card in hand there is nothing to exile, so the free cast is
/// not offered at all.
#[test]
fn pyrokinesis_needs_a_red_card_to_exile() {
    let mut game = ready();
    let pyro = card(20_000, cards::PYROKINESIS, PlayerId::One);
    let pyro_id = pyro.id;
    game.players[PlayerId::One.index()].hand.push(pyro);
    game.players[PlayerId::One.index()].hand.push(card(
        20_001,
        cards::ANCESTRAL_RECALL,
        PlayerId::One,
    ));

    let free = game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(action, Action::CastSpell { card, choices, .. }
            if card == pyro_id && choices.costs().alternative().is_some())
    });
    assert!(!free, "a blue card is not a red card");
}

/// A Daze with no Island is still a Daze: what the missing land costs is
/// the alternative, not the card.
#[test]
fn daze_without_an_island_is_still_castable_for_its_printed_cost() {
    let (mut game, free) = free_cast(cards::DAZE, 0);
    assert!(free.is_none(), "nothing to return");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(20_000))),
        "two mana is what it says on the card",
    );
}

/// "An Island you control" is a land with the Island type, which a Tundra
/// has and a Plains does not.
#[test]
fn daze_takes_any_land_with_the_island_type() {
    assert!(
        free_cast_over(cards::DAZE, &[cards::TUNDRA]).1.is_some(),
        "a Tundra is an Island as well as a Plains",
    );
    assert!(
        free_cast_over(cards::DAZE, &[cards::PLAINS]).1.is_none(),
        "and a Plains is not one at all",
    );
}

/// "The mana value of the spell is determined by only its mana cost, no
/// matter what the total cost to cast that spell was." A Daze that cost an
/// Island and no mana is still a two-drop on the stack.
#[test]
fn a_free_daze_is_still_worth_two() {
    let (mut game, cast) = free_cast(cards::DAZE, 1);
    game.apply(PlayerId::One, cast.expect("one Island pays for it"))
        .expect("it is cast");

    let daze = game
        .stack
        .iter()
        .find(|object| object.card.definition.card_definition() == Some(cards::DAZE))
        .expect("it is on the stack");
    assert_eq!(
        game.stack_spell_mana_value(daze),
        2,
        "an alternative cost is what was paid, not what it is worth",
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::ISLAND)
            .count(),
        1,
        "and the Island came back to hand rather than dying",
    );
}

/// "Two Islands you control" is the land type rather than the card name: a
/// Tropical Island and an Underground Sea are two Islands, and a Wasteland
/// beside them is none.
#[test]
fn gush_reads_the_island_type_rather_than_the_card() {
    assert!(
        free_cast_over(
            cards::GUSH,
            &[cards::TROPICAL_ISLAND, cards::UNDERGROUND_SEA],
        )
        .1
        .is_some(),
        "two duals with the Island type pay for it",
    );
    assert!(
        free_cast_over(cards::GUSH, &[cards::TROPICAL_ISLAND, cards::WASTELAND])
            .1
            .is_none(),
        "and a colourless land is not the second one",
    );
}

/// "Islands *you control*": theirs are no help, however many they have.
#[test]
fn gush_cannot_bounce_their_islands() {
    let mut game = ready();
    for index in 0..3 {
        game.battlefield
            .push(creature(11_000 + index, cards::ISLAND, PlayerId::Two));
    }
    let gush = card(20_010, cards::GUSH, PlayerId::One);
    let gush_id = gush.id;
    game.players[PlayerId::One.index()].hand.push(gush);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == gush_id && choices.costs().alternative().is_some())
        }),
        "three Islands across the table are nobody's to return",
    );
}

/// The Islands are returned to pay for the spell rather than by it, so a
/// Gush that never resolves has still cost them: the lands are in hand and
/// the two cards are not.
#[test]
fn a_countered_gush_still_costs_its_islands() {
    let (mut game, cast) = free_cast(cards::GUSH, 2);
    game.stack.clear();
    let counter = card(20_020, cards::COUNTERSPELL, PlayerId::Two);
    let counter_id = counter.id;
    game.players[PlayerId::Two.index()].hand.push(counter);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);
    let library = game.players[PlayerId::One.index()].library.len();

    game.apply(PlayerId::One, cast.expect("two Islands pay for it"))
        .expect("it is cast");
    let gush = game.stack.last().expect("the Gush is on the stack").id;
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::ISLAND)
            .count(),
        2,
        "the lands came back as the cost was paid, before anything resolved",
    );

    game.priority = PlayerId::Two;
    let answer = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == counter_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(gush))
            }
            _ => false,
        })
        .expect("a Counterspell answers it");
    game.apply(PlayerId::Two, answer).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library,
        "nothing was drawn",
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::ISLAND)
            .count(),
        2,
        "and the two lands are still bounced: a cost is paid whatever follows",
    );
}
