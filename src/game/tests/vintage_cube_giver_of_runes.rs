//! Giver of Runes: Mother of Runes who cannot save herself, and in exchange
//! answers colourless removal her mother could not.

use super::*;

/// Her on the battlefield since last turn, with `others` beside her.
fn staged(others: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let giver = game
        .put_onto_battlefield(PlayerId::One, cards::GIVER_OF_RUNES)
        .expect("cataloged");
    let mut ids = Vec::new();
    for definition in others {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, giver, ids)
}

/// The activation naming `wanted`, if it is offered at all.
fn activation(game: &Game, giver: GameObjectId, wanted: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == giver
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(wanted)))
            }
            _ => false,
        })
}

/// Runs the ability out, naming the quality labelled `label`.
fn protect(game: &mut Game, giver: GameObjectId, wanted: GameObjectId, label: &str) {
    let action = activation(game, giver, wanted).expect("the ability is activatable");
    game.apply(PlayerId::One, action).expect("it activates");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("a quality was asked for");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == label)
        .unwrap_or_else(|| panic!("{label} is offered: {:?}", decision.options))
        .id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the decision accepts what it offered");
    drain_pending(game);
}

fn protected_from(game: &Game, id: GameObjectId, color: ManaColor) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .is_some_and(|permanent| {
            game.permanent_has_executable_keyword(permanent, protection_keyword(color))
        })
}

/// She offers colourless alongside the five colours -- six qualities, where
/// her mother offers five.
#[test]
fn the_choice_includes_colorless() {
    let (mut game, giver, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];
    let action = activation(&game, giver, bears).expect("the ability is activatable");
    game.apply(PlayerId::One, action).expect("it activates");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("a quality was asked for");
    let labels = decision
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert_eq!(labels.len(), 6);
    assert!(labels.contains(&"Colorless".to_owned()));
    assert!(labels.contains(&"Black".to_owned()));
}

/// Naming a colour protects against that colour and nothing else.
#[test]
fn naming_a_colour_protects_from_it() {
    let (mut game, giver, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];

    protect(&mut game, giver, bears, "Black");

    assert!(protected_from(&game, bears, ManaColor::Black));
    assert!(!protected_from(&game, bears, ManaColor::Red));
    assert!(!protected_from(&game, bears, ManaColor::Colorless));
}

/// A Ballista player Two controls, loaded and ready to shoot.
fn opposing_ballista(game: &mut Game) -> GameObjectId {
    let ballista = game
        .put_onto_battlefield(PlayerId::Two, cards::WALKING_BALLISTA)
        .expect("cataloged");
    drain_pending(game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == ballista)
    {
        permanent.add_counters(CounterKind::PlusOnePlusOne, 2);
        permanent.entered_controller_turn = 0;
    }
    ballista
}

/// Whether the Ballista may point its shot at `wanted`.
fn ballista_can_shoot(game: &Game, ballista: GameObjectId, wanted: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::ActivateAbility { source, targets, .. }
                if source == ballista
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(wanted)))
        )
    })
}

/// Naming colourless takes the creature off a colourless source's target
/// list, which is the whole reason the option is there.
#[test]
fn naming_colorless_stops_a_colorless_source() {
    let (mut game, giver, others) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    let bears = others[0];
    let lions = others[1];
    let ballista = opposing_ballista(&mut game);
    // Her ability is done; hand the window to the other player so their
    // activations are the ones being enumerated.
    game.priority = PlayerId::Two;
    assert!(
        ballista_can_shoot(&game, ballista, bears),
        "it could shoot the creature before",
    );

    game.priority = PlayerId::One;
    protect(&mut game, giver, bears, "Colorless");
    assert!(protected_from(&game, bears, ManaColor::Colorless));
    game.priority = PlayerId::Two;

    assert!(
        !ballista_can_shoot(&game, ballista, bears),
        "a colourless source cannot point at it now",
    );
    assert!(
        ballista_can_shoot(&game, ballista, lions),
        "and everything else is still fair game",
    );
}

/// A coloured source is not colourless, so protection from colourless leaves
/// it alone.
#[test]
fn protection_from_colorless_does_not_stop_a_coloured_source() {
    let (mut game, giver, others) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = others[0];
    protect(&mut game, giver, bears, "Colorless");

    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);
    let protected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("the protected creature is there");

    assert!(!game.is_protected_from_object(protected, angel, false));
}

/// "Another": she is not a legal target for her own ability.
#[test]
fn she_cannot_protect_herself() {
    let (game, giver, _) = staged(&[cards::GRIZZLY_BEARS]);

    assert!(
        activation(&game, giver, giver).is_none(),
        "she may not name herself",
    );
}

/// Nor a creature an opponent controls.
#[test]
fn she_cannot_protect_their_creature() {
    let (mut game, giver, _) = staged(&[]);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(activation(&game, giver, theirs).is_none());
}
