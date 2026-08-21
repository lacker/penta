//! Get Lost: two mana that answers three card types, and two Maps as the
//! price.

use super::*;

/// Player One holding a Get Lost with two mana up, and `theirs` on the
/// battlefield under Player Two.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in theirs {
        game.put_onto_battlefield(PlayerId::Two, *definition)
            .expect("cataloged");
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::GET_LOST])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let get_lost = card.id;
    game.players[0].hand.push(card);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, get_lost)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
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

/// Casts it at `target` and lets it resolve.
fn cast_at(game: &mut Game, get_lost: GameObjectId, target: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == get_lost
                    && choices
                        .iter_targets()
                        .any(|found| *found == Target::Permanent(target))
            }
            _ => false,
        })
        .expect("it is castable at that permanent");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

fn permanents_of(game: &Game, definition: CardDefinitionId) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == definition)
        .collect()
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    !permanents_of(game, definition).is_empty()
}

fn maps(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, tokens::map()))
        .collect()
}

/// It kills a creature and hands its controller two Maps.
#[test]
fn it_destroys_a_creature_and_pays_two_maps() {
    let (mut game, get_lost) = staged(&[cards::SERRA_ANGEL]);
    let angel = permanents_of(&game, cards::SERRA_ANGEL)[0].card.id;

    cast_at(&mut game, get_lost, angel);

    assert!(!on_battlefield(&game, cards::SERRA_ANGEL), "the Angel died");
    let maps = maps(&game);
    assert_eq!(maps.len(), 2, "two Maps");
    for map in maps {
        assert_eq!(
            map.controller,
            PlayerId::Two,
            "under the dead creature's controller, not yours",
        );
    }
}

/// "Creature, enchantment, or planeswalker": an enchantment is one of the
/// three.
#[test]
fn it_reaches_an_enchantment() {
    let (mut game, get_lost) = staged(&[cards::CIRCLE_OF_PROTECTION_BLUE]);
    let circle = permanents_of(&game, cards::CIRCLE_OF_PROTECTION_BLUE)[0]
        .card
        .id;

    cast_at(&mut game, get_lost, circle);

    assert!(
        !on_battlefield(&game, cards::CIRCLE_OF_PROTECTION_BLUE),
        "the Circle is gone",
    );
    assert_eq!(maps(&game).len(), 2);
}

/// An artifact is not one of the three.
#[test]
fn an_artifact_is_not_a_legal_target() {
    let (game, get_lost) = staged(&[cards::SOL_RING]);
    let ring = permanents_of(&game, cards::SOL_RING)[0].card.id;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == get_lost
                    && choices.iter_targets().any(|found| *found == Target::Permanent(ring)))
        }),
        "a Sol Ring is none of creature, enchantment, or planeswalker",
    );
}

/// The Map explores: a land off the top goes to hand and the creature stays
/// the size it was.
#[test]
fn a_map_explores_a_land_into_hand() {
    let (mut game, get_lost) = staged(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    let angel = permanents_of(&game, cards::SERRA_ANGEL)[0].card.id;
    cast_at(&mut game, get_lost, angel);
    let bears = permanents_of(&game, cards::GRIZZLY_BEARS)[0].card.id;
    let map = maps(&game)[0].card.id;
    let land = game
        .build_zone(PlayerId::Two, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].library.push(land);
    let before = game.players[1].hand.len();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;

    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("the Map can point at their own Bears");
    game.apply(PlayerId::Two, action).expect("it activates");
    settle(&mut game);

    assert!(deciding(&game).is_none(), "a land asks nothing");
    assert_eq!(
        game.players[1].hand.len(),
        before + 1,
        "the land went to hand",
    );
    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is still here");
    assert_eq!(
        bears.counters(CounterKind::PlusOnePlusOne),
        0,
        "and no counter went on",
    );
}

/// A nonland grows the creature and then asks where the card goes.
#[test]
fn a_nonland_grows_the_creature_and_asks() {
    let (mut game, get_lost) = staged(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    let angel = permanents_of(&game, cards::SERRA_ANGEL)[0].card.id;
    cast_at(&mut game, get_lost, angel);
    let bears = permanents_of(&game, cards::GRIZZLY_BEARS)[0].card.id;
    let map = maps(&game)[0].card.id;
    let spell = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].library.push(spell);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("it is activatable");
    game.apply(PlayerId::Two, action).expect("it activates");
    settle(&mut game);

    let seat = deciding(&game).expect("it asks where the card goes");
    assert_eq!(seat, PlayerId::Two, "the exploring creature's controller");
    let counters = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is here")
        .counters(CounterKind::PlusOnePlusOne);
    assert_eq!(counters, 1, "the counter goes on before the card is placed");

    // Bury it: the other half of the choice.
    let decision = game.observe(seat).decision.expect("just checked");
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .expect("the answer is legal");
    settle(&mut game);

    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt went to the graveyard",
    );
}

/// Leaving it on top is the half that moves nothing.
#[test]
fn the_revealed_card_may_stay_on_top() {
    let (mut game, get_lost) = staged(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    let angel = permanents_of(&game, cards::SERRA_ANGEL)[0].card.id;
    cast_at(&mut game, get_lost, angel);
    let bears = permanents_of(&game, cards::GRIZZLY_BEARS)[0].card.id;
    let map = maps(&game)[0].card.id;
    let spell = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].library.push(spell);
    let library = game.players[1].library.len();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("it is activatable");
    game.apply(PlayerId::Two, action).expect("it activates");
    settle(&mut game);
    let seat = deciding(&game).expect("it asks");
    let decision = game.observe(seat).decision.expect("just checked");

    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .expect("the answer is legal");
    settle(&mut game);

    assert_eq!(
        game.players[1].library.len(),
        library,
        "the Bolt is still in the library",
    );
    assert_eq!(
        game.players[1].library.last().map(|card| card.definition),
        Some(cards::LIGHTNING_BOLT),
        "and still on top",
    );
}

/// The Map spends itself: it is sacrificed as a cost.
#[test]
fn the_map_sacrifices_itself() {
    let (mut game, get_lost) = staged(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    let angel = permanents_of(&game, cards::SERRA_ANGEL)[0].card.id;
    cast_at(&mut game, get_lost, angel);
    let bears = permanents_of(&game, cards::GRIZZLY_BEARS)[0].card.id;
    let map = maps(&game)[0].card.id;
    let land = game
        .build_zone(PlayerId::Two, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].library.push(land);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == map
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("it is activatable");
    game.apply(PlayerId::Two, action).expect("it activates");
    settle(&mut game);

    assert_eq!(maps(&game).len(), 1, "one Map spent, one left");
}
