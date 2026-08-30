//! Underworld Breach: a graveyard turned into a hand for exactly one turn.

use super::*;

/// Player One with a Breach on the battlefield and `graveyard` behind it.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    for definition in graveyard {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].graveyard.push(card);
    }
    let breach = game
        .put_onto_battlefield(PlayerId::One, cards::UNDERWORLD_BREACH)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, breach)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if !game.pending_decisions.is_empty() {
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

/// The id a card of `definition` has in the graveyard.
fn in_graveyard(game: &Game, definition: CardDefinitionId) -> Option<GameObjectId> {
    game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == definition)
        .map(|card| card.id)
}

/// Every way the graveyard card of `definition` could be cast right now.
fn escapes(game: &Game, definition: CardDefinitionId) -> Vec<Action> {
    let Some(card) = in_graveyard(game, definition) else {
        return Vec::new();
    };
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card))
        .collect()
}

/// A Lightning Bolt in the graveyard escapes for its own {R} plus three
/// other cards.
#[test]
fn a_nonland_card_escapes_for_its_mana_cost_and_three_others() {
    let (mut game, _breach) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let offers = escapes(&game, cards::LIGHTNING_BOLT);
    assert!(!offers.is_empty(), "the Breach makes it castable");

    let action = offers
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { choices, .. } => choices
                .targets()
                .iter()
                .any(|selection| selection.targets().contains(&Target::Player(PlayerId::Two))),
            _ => false,
        })
        .expect("it can point at a player");
    game.apply(PlayerId::One, action).expect("it escapes");
    assert!(
        game.stack.last().is_some_and(|spell| {
            spell.cast.as_ref().is_some_and(|cast| {
                cast.source_zone == Some(CastSourceZone::Graveyard)
                    && cast.alternative == Some(AlternativeCastKindDef::Escape)
            })
        }),
        "the granted Escape ability records the cast alternative",
    );
    settle(&mut game);

    assert_eq!(game.players[1].life, 17, "the Bolt resolved");
    assert_eq!(
        game.players[0].exile.len(),
        3,
        "the three that paid for it were exiled rather than buried",
    );
    // Escape exiles what it costs, not what it casts: the Bolt is back in
    // the graveyard afterwards, ready to escape again for three more.
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "and the Bolt itself came back down",
    );
}

/// Without a Breach the same graveyard is inert.
#[test]
fn nothing_escapes_without_the_breach() {
    let (mut game, breach) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.battlefield
        .retain(|permanent| permanent.card.id != breach);

    assert!(
        escapes(&game, cards::LIGHTNING_BOLT).is_empty(),
        "a graveyard is not a hand on its own",
    );
}

/// Three *other* cards: with only two behind it, the cost cannot be paid.
#[test]
fn two_other_cards_is_not_enough() {
    let (mut game, _breach) = staged(&[cards::LIGHTNING_BOLT, cards::PLAINS, cards::ISLAND]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        escapes(&game, cards::LIGHTNING_BOLT).is_empty(),
        "an additional cost is paid in full or not at all",
    );
}

/// "Each nonland card": a land in the graveyard gains nothing.
#[test]
fn a_land_in_the_graveyard_gains_nothing() {
    let (mut game, _breach) = staged(&[
        cards::MOUNTAIN,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
        cards::FOREST,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 4);

    assert!(
        escapes(&game, cards::MOUNTAIN).is_empty(),
        "a land is not a nonland card",
    );
}

/// Your graveyard, not theirs.
#[test]
fn it_does_not_reach_the_other_graveyard() {
    let (mut game, _breach) = staged(&[cards::PLAINS, cards::ISLAND, cards::SWAMP]);
    let theirs = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let bolt = theirs.id;
    game.players[1].graveyard.push(theirs);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt)),
        "the Breach speaks about your own graveyard",
    );
}

/// One step of "let the turn get on with it": answer anything pending,
/// finish a combat declaration when one is open, and otherwise pass.
fn advance(game: &mut Game) -> bool {
    if let Some(seat) = game.pending_decisions.first().map(|p| p.observation.player) {
        let decision = game.observe(seat).decision.expect("just checked");
        let options = decision
            .options
            .iter()
            .take(decision.minimum)
            .map(|option| option.id)
            .collect();
        return game
            .apply(
                seat,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .is_ok();
    }
    for action in [
        Action::FinishDeclaringAttackers,
        Action::FinishDeclaringBlockers,
    ] {
        for seat in [PlayerId::One, PlayerId::Two] {
            if game.legal_actions(seat).contains(&action) {
                return game.apply(seat, action.clone()).is_ok();
            }
        }
    }
    let player = game.priority;
    game.apply(player, Action::PassPriority).is_ok()
}

/// One turn only: the end step takes the Breach with it.
#[test]
fn it_sacrifices_itself_at_the_end_step() {
    let (mut game, breach) = staged(&[]);

    for _ in 0..60 {
        if !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == breach)
        {
            break;
        }
        if !advance(&mut game) {
            break;
        }
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == breach),
        "the Breach is gone",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::UNDERWORLD_BREACH),
        "sacrificed into its owner's graveyard",
    );
}

/// "If a card has multiple abilities giving you permission to cast it, such
/// as ... an escape ability and a flashback ability, you choose which one to
/// apply." A Firebolt under a Breach is offered both ways, and each is
/// priced its own way -- flashback for {4}{R}, escape for {R} and three
/// cards.
#[test]
fn a_flashback_card_is_offered_both_permissions() {
    let (mut game, _breach) =
        staged(&[cards::FIREBOLT, cards::PLAINS, cards::ISLAND, cards::SWAMP]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    assert_eq!(
        escapes(&game, cards::FIREBOLT).len(),
        2,
        "one red and three cards buys the escape, at either target",
    );

    // The flashback price on top of it: five mana, and no cards exiled.
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let firebolt = in_graveyard(&game, cards::FIREBOLT).expect("it is there");
    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == firebolt
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
                    && choices.costs().alternative().is_some()
            }
            _ => false,
        })
        .expect("flashback is a permission of its own");
    game.apply(PlayerId::One, flashback).expect("it is cast");
    settle(&mut game);

    assert_eq!(game.players[1].life, 18, "the Firebolt resolved");
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "flashback exiled the card itself and nothing else",
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        3,
        "the three that escape would have eaten are untouched",
    );
}

/// "After an escaped spell resolves, it returns to its owner's graveyard ...
/// Perhaps it will escape again." Six other cards is two Bolts out of one.
#[test]
fn an_escaped_spell_can_escape_again() {
    let (mut game, _breach) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
        cards::MOUNTAIN,
        cards::FOREST,
        cards::GRIZZLY_BEARS,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);

    for attempt in 0..2 {
        let action = escapes(&game, cards::LIGHTNING_BOLT)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { choices, .. } => choices
                    .targets()
                    .iter()
                    .any(|selection| selection.targets().contains(&Target::Player(PlayerId::Two))),
                _ => false,
            })
            .unwrap_or_else(|| panic!("three more cards pay for escape {attempt}"));
        game.apply(PlayerId::One, action).expect("it escapes");
        settle(&mut game);
    }

    assert_eq!(game.players[1].life, 14, "the same Bolt, twice");
    assert_eq!(
        game.players[0].exile.len(),
        6,
        "three cards each time, and none of them the Bolt",
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "which is back again, wanting only three more",
    );
}

/// "If a card has no mana cost, its escape cost is an unpayable cost, so you
/// can't cast it for that cost." An Ancestral Vision has none to be equal to.
#[test]
fn a_card_with_no_mana_cost_cannot_escape() {
    let (mut game, _breach) = staged(&[
        cards::ANCESTRAL_VISION,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 8);

    assert!(
        escapes(&game, cards::ANCESTRAL_VISION).is_empty(),
        "no mana cost is not a cost of zero: there is nothing to pay",
    );
}

/// "Escape's permission doesn't change when you may cast the spell from your
/// graveyard." A Firebolt is still a sorcery, so their turn is no time for
/// it.
#[test]
fn escape_does_not_change_when_a_sorcery_may_be_cast() {
    let (mut game, _breach) =
        staged(&[cards::FIREBOLT, cards::PLAINS, cards::ISLAND, cards::SWAMP]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);
    assert!(
        !escapes(&game, cards::FIREBOLT).is_empty(),
        "your own main phase is the time for it",
    );

    game.active_player = PlayerId::Two;
    assert!(
        escapes(&game, cards::FIREBOLT).is_empty(),
        "and their turn is not, escape or no escape",
    );
}

/// "If it is a permanent spell, it enters the battlefield and will return to
/// its owner's graveyard if it dies later." Every other test here escapes an
/// instant or a sorcery, which goes back down as it resolves; a creature
/// stays where it lands.
#[test]
fn an_escaped_permanent_stays_on_the_battlefield() {
    let (mut game, _breach) = staged(&[
        cards::GRIZZLY_BEARS,
        cards::PLAINS,
        cards::ISLAND,
        cards::SWAMP,
    ]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let action = escapes(&game, cards::GRIZZLY_BEARS)
        .into_iter()
        .next()
        .expect("two mana and three cards pay for it");
    game.apply(PlayerId::One, action).expect("it escapes");
    settle(&mut game);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the creature resolved onto the battlefield");
    assert_eq!(
        (game.power(bears), game.toughness(bears)),
        (Some(2), Some(2)),
        "a 2/2 like any other",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "the graveyard is empty: three cards paid and the Bears is standing",
    );
    assert_eq!(
        game.players[0].exile.len(),
        3,
        "and the three that paid are in exile",
    );

    // "It will return to its owner's graveyard if it dies later."
    let bears = bears.card.id;
    game.destroy_permanent(bears);
    game.check_state_based_actions();

    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::GRIZZLY_BEARS],
        "and dying is what puts it back where it escaped from",
    );
}
