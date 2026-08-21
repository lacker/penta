//! Legion Extruder: two mana that answers something on the way in, then
//! turns every spent artifact into a 3/3.

use super::*;

/// Player One holding an Extruder with two mana up, and `board` already on
/// the battlefield under them.
fn staged(board: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in board {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::LEGION_EXTRUDER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let extruder = card.id;
    game.players[0].hand.push(card);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, extruder)
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

/// Casts the Extruder with its trigger pointed at `target`, and lets it
/// resolve.
fn cast_at(game: &mut Game, extruder: GameObjectId, target: Target) -> GameObjectId {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == extruder))
        .expect("two mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
    // Entering asks what the damage points at; the artifact is already on
    // the battlefield by then.
    if let Some(seat) = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.player)
    {
        let decision = game.observe(seat).decision.expect("just checked");
        let option = decision
            .options
            .iter()
            .find(|option| match target {
                // A player option carries no card, so the seat is told by
                // the label the observation prints for it.
                Target::Player(PlayerId::Two) => {
                    option.card.is_none() && option.label.contains("opponent")
                }
                Target::Player(PlayerId::One) => {
                    option.card.is_none() && !option.label.contains("opponent")
                }
                Target::Permanent(id) => option.card.is_some_and(|(found, _)| found == id),
                _ => false,
            })
            .unwrap_or_else(|| panic!("{target:?} is a legal target"))
            .id;
        game.apply(
            seat,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .expect("the answer is legal");
        settle(game);
    }
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LEGION_EXTRUDER)
        .expect("it resolved")
        .card
        .id
}

/// Every way the Extruder could make a Golem right now.
fn golem_activations(game: &Game, extruder: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == extruder),
        )
        .collect()
}

fn golems(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(permanent, tokens::artifact_creature(&["Golem"], &[], 3, 3))
        })
        .count()
}

/// The entry trigger shoots two damage at a player.
#[test]
fn entering_deals_two_damage_to_a_player() {
    let (mut game, extruder) = staged(&[]);

    cast_at(&mut game, extruder, Target::Player(PlayerId::Two));

    assert_eq!(game.players[1].life, 18, "two damage across the table");
    assert_eq!(game.players[0].life, 20, "and none of it your own");
}

/// "Any target" reaches a creature too, and two damage kills a two-toughness
/// one.
#[test]
fn entering_can_shoot_a_creature_instead() {
    let (mut game, extruder) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    cast_at(&mut game, extruder, Target::Permanent(bears));

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "a 2/2 does not survive two damage",
    );
    assert_eq!(game.players[1].life, 20, "and no player was hurt");
}

/// Two mana, a tap, and a spare artifact make a 3/3.
#[test]
fn it_turns_a_spare_artifact_into_a_golem() {
    let (mut game, extruder) = staged(&[cards::SOL_RING]);
    let body = cast_at(&mut game, extruder, Target::Player(PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let action = golem_activations(&game, body)
        .into_iter()
        .next()
        .expect("a spare artifact pays for it");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(&mut game);

    assert_eq!(golems(&game), 1, "one Golem");
    let golem = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(permanent, tokens::artifact_creature(&["Golem"], &[], 3, 3))
        })
        .expect("it is here");
    assert_eq!(
        (game.power(golem), game.toughness(golem)),
        (Some(3), Some(3))
    );
    assert!(
        game.permanent_types(golem)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "an artifact creature",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SOL_RING),
        "and the Sol Ring paid for it",
    );
}

/// "Another artifact": the Extruder may not eat itself.
#[test]
fn it_cannot_sacrifice_itself() {
    let (mut game, extruder) = staged(&[]);
    let body = cast_at(&mut game, extruder, Target::Player(PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    assert!(
        golem_activations(&game, body).is_empty(),
        "with nothing else to spend there is nothing to spend",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == body),
        "and it is still here",
    );
}

/// An artifact is what it eats: a creature that is not one will not do.
#[test]
fn a_nonartifact_permanent_is_not_a_legal_sacrifice() {
    let (mut game, extruder) = staged(&[cards::GRIZZLY_BEARS]);
    let body = cast_at(&mut game, extruder, Target::Player(PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    assert!(
        golem_activations(&game, body).is_empty(),
        "a Grizzly Bears is not an artifact",
    );
}

/// Theirs is not yours: the sacrifice comes off your own battlefield.
#[test]
fn it_cannot_eat_an_artifact_they_control() {
    let (mut game, extruder) = staged(&[]);
    game.put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    let body = cast_at(&mut game, extruder, Target::Player(PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    assert!(
        golem_activations(&game, body).is_empty(),
        "you sacrifice permanents you control",
    );
}
