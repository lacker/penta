//! Kolaghan's Command: three mana that is two cards on every board.

use super::*;

const RETURN: usize = 0;
const DISCARD: usize = 1;
const DESTROY: usize = 2;
const BOLT: usize = 3;

fn mode(index: usize) -> ModeId {
    ModeId::from_index(index).expect("one of the four")
}

/// The Command in hand with the mana for it, `mine` in your graveyard,
/// `theirs` on their battlefield, and a card in their hand.
fn staged(mine: &[CardDefinitionId], theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[0].graveyard.clear();
    for definition in mine {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].graveyard.push(card);
    }
    for definition in theirs {
        game.put_onto_battlefield(PlayerId::Two, *definition)
            .expect("cataloged");
    }
    let held = game
        .build_zone(PlayerId::Two, &[cards::MOUNTAIN])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[1].hand.push(held);
    let command = game
        .build_zone(PlayerId::One, &[cards::KOLAGHAN_S_COMMAND])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = command.id;
    game.players[0].hand.push(command);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.players[0].life = 20;
    game.players[1].life = 20;
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id)
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
    game.check_state_based_actions();
}

/// Every pair of modes on offer, with the targets each would take.
fn casts(game: &Game, command: GameObjectId) -> Vec<(Vec<ModeId>, Vec<Target>)> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } if id == command => Some((
                choices.modes().to_vec(),
                choices.iter_targets().copied().collect(),
            )),
            _ => None,
        })
        .collect()
}

fn cast_with(game: &mut Game, command: GameObjectId, wanted: &[usize], targets: &[Target]) {
    let modes = wanted.iter().copied().map(mode).collect::<Vec<_>>();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } => {
                *id == command
                    && choices.modes() == modes
                    && choices.iter_targets().copied().collect::<Vec<_>>() == targets
            }
            _ => false,
        })
        .expect("that combination of modes and targets is on offer");
    game.apply(PlayerId::One, action).expect("it is cast");
    settle(game);
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// The two halves the deck plays it for: a creature back and their artifact
/// gone.
#[test]
fn it_returns_a_creature_and_destroys_an_artifact() {
    let (mut game, command) = staged(&[cards::SERRA_ANGEL], &[cards::SOL_RING]);
    let ring = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SOL_RING)
        .expect("it is here")
        .card
        .id;

    let angel = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::SERRA_ANGEL)
        .expect("it is in the graveyard")
        .id;

    cast_with(
        &mut game,
        command,
        &[RETURN, DESTROY],
        &[Target::Card(angel), Target::Permanent(ring)],
    );

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the Angel is back in hand",
    );
    assert!(!on_battlefield(&game, cards::SOL_RING), "and the Ring gone");
}

/// The other two: two damage and a discard.
#[test]
fn it_bolts_and_makes_them_discard() {
    let (mut game, command) = staged(&[], &[]);

    // Both halves aimed across the table: "target player" would happily
    // take you instead, and so would "any target".
    cast_with(
        &mut game,
        command,
        &[DISCARD, BOLT],
        &[Target::Player(PlayerId::Two), Target::Player(PlayerId::Two)],
    );

    assert_eq!(game.players[1].life, 18, "two damage");
    assert!(
        game.players[1].hand.is_empty(),
        "and the card in their hand is gone",
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "into their graveyard",
    );
}

/// Two of four and never the same one twice.
#[test]
fn it_always_chooses_two_different_modes() {
    let (game, command) = staged(&[cards::SERRA_ANGEL], &[cards::SOL_RING]);

    let offered = casts(&game, command);
    assert!(!offered.is_empty(), "it is castable");
    for (modes, _) in &offered {
        assert_eq!(modes.len(), 2, "always two: {modes:?}");
        assert_ne!(modes[0], modes[1], "and never the same twice");
    }
}

/// A mode with nothing to point at is not among the pairs on offer.
#[test]
fn a_mode_with_no_target_is_not_offered() {
    let (game, command) = staged(&[], &[]);

    let offered = casts(&game, command);
    assert!(!offered.is_empty(), "the other modes still pair up");
    assert!(
        offered
            .iter()
            .all(|(modes, _)| !modes.contains(&mode(RETURN)) && !modes.contains(&mode(DESTROY))),
        "an empty graveyard and no artifact leave two modes: {offered:?}",
    );
}

/// "If at least one target is still legal, the spell will resolve but will
/// have no effect on any illegal targets." Cracking the artifact it named
/// saves the artifact and nothing else: the two damage still lands.
#[test]
fn one_answered_target_does_not_save_the_other() {
    let (mut game, command) = staged(&[], &[cards::BLACK_LOTUS]);
    let lotus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BLACK_LOTUS)
        .expect("it is here")
        .card
        .id;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: id, choices, ..
            } => {
                *id == command
                    && choices.modes() == [mode(DESTROY), mode(BOLT)]
                    && choices.iter_targets().copied().collect::<Vec<_>>()
                        == [Target::Permanent(lotus), Target::Player(PlayerId::Two)]
            }
            _ => false,
        })
        .expect("the Lotus and their face");
    game.apply(PlayerId::One, action).expect("it is cast");

    // They crack the Lotus for mana rather than let it be destroyed.
    game.priority = PlayerId::Two;
    let crack = Action::ActivateManaAbility {
        source: lotus,
        ability: mana_ability_for(&game, lotus, ManaColor::Green),
        color: ManaColor::Green,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::Two, crack)
        .expect("its own ability sacrifices it");
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "the mode that still had a target resolved",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::KOLAGHAN_S_COMMAND),
        "and the Command is spent",
    );
}

/// "Follow the instructions of the modes you chose in the order they are
/// printed": the creature comes back before the discard is made, so it is
/// among the cards that may be discarded.
#[test]
fn the_returned_card_is_there_to_be_discarded() {
    let (mut game, command) = staged(&[cards::SERRA_ANGEL], &[]);
    let angel = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::SERRA_ANGEL)
        .expect("it is in the graveyard")
        .id;
    assert!(game.players[0].hand.len() == 1, "only the Command is held");

    cast_with(
        &mut game,
        command,
        &[RETURN, DISCARD],
        &[Target::Card(angel), Target::Player(PlayerId::One)],
    );

    assert!(
        game.players[0].hand.is_empty(),
        "the Angel came back and was the only card there to discard",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "so it went straight back where it came from",
    );
}
