//! Chandra, Torch of Defiance: an offer to cast the top card of your library,
//! and two damage when you turn it down.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    let chandra = game
        .put_onto_battlefield(PlayerId::One, cards::CHANDRA_TORCH_OF_DEFIANCE)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == chandra)
    {
        permanent.entered_controller_turn = 0;
    }
    drain_pending(&mut game);
    (game, chandra)
}

/// Puts one card on top of Player One's library and hands it back.
fn on_top(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let built = game
        .build_zone(PlayerId::One, &[definition])
        .expect("cataloged");
    let card = built.into_iter().next().expect("one card");
    let id = card.id;
    game.players[0].library.push(card);
    id
}

fn activate(game: &mut Game, chandra: GameObjectId, index: usize, targets: Vec<TargetSelection>) {
    let ability = activated_ability_for(game, chandra, index);
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: chandra,
            ability,
            targets,
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .expect("the loyalty ability activates");
    resolve_stack(game);
}

/// Passes priority until the stack is empty or somebody has to decide.
fn resolve_stack(game: &mut Game) {
    for _ in 0..16 {
        if !game.pending_decisions.is_empty() {
            return;
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

fn decline(game: &mut Game) {
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the offer is waiting");
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .expect("declining is always available");
    resolve_stack(game);
}

/// The card is exiled, the offer stands, and turning it down is what deals
/// the damage.
#[test]
fn declining_the_cast_shoots_each_opponent() {
    let (mut game, chandra) = staged();
    on_top(&mut game, cards::LIGHTNING_BOLT);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    activate(&mut game, chandra, 0, Vec::new());
    // Leaving the library mints a new object, so the card is recognized by
    // what it is rather than by the identity it had on top.
    assert!(
        exiled_bolt(&game),
        "the top card is exiled whatever happens next",
    );
    assert_eq!(game.pending_decisions.len(), 1, "the offer is standing");
    assert_eq!(game.players[1].life, 20, "nothing has been declined yet");

    decline(&mut game);
    assert_eq!(game.players[1].life, 18);
    assert!(exiled_bolt(&game), "a card left uncast stays in exile");
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { .. })),
        "and the permission to cast it went with the decline",
    );
}

fn exiled_bolt(game: &Game) -> bool {
    game.players[0]
        .exile
        .iter()
        .any(|card| card.definition == cards::LIGHTNING_BOLT)
}

/// Casting it is how the offer is answered, and the damage never happens.
#[test]
fn casting_the_exiled_card_answers_the_offer() {
    let (mut game, chandra) = staged();
    on_top(&mut game, cards::LIGHTNING_BOLT);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    activate(&mut game, chandra, 0, Vec::new());
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { .. }))
        .expect("the exiled card is castable from the offer");
    game.apply(PlayerId::One, cast)
        .expect("the offer accepts a cast");

    assert!(
        game.pending_decisions.is_empty(),
        "the cast took the offer away rather than leaving it to answer",
    );
    assert_eq!(game.stack.len(), 1, "the spell is on the stack");
    assert_eq!(game.players[1].life, 20, "the else branch never ran");
}

/// An unfunded card retains its manual offer; declining runs the else branch.
#[test]
fn an_unfunded_card_retains_the_manual_cast_offer() {
    let (mut game, chandra) = staged();
    on_top(&mut game, cards::LIGHTNING_BOLT);

    activate(&mut game, chandra, 0, Vec::new());

    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::BeginPayment)
    );
    assert_eq!(game.players[1].life, 20);
    let decision = game.pending_decisions[0].observation.id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision,
            options: vec![0],
        },
    )
    .unwrap();
    assert_eq!(game.players[1].life, 18, "declining deals the damage");
}

/// An empty library exiles nothing and offers nothing.
#[test]
fn an_empty_library_still_shoots() {
    let (mut game, chandra) = staged();

    activate(&mut game, chandra, 0, Vec::new());

    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[1].life, 18);
}

/// The mana ability is a loyalty ability, so it uses the stack.
#[test]
fn the_second_plus_one_makes_two_red() {
    let (mut game, chandra) = staged();

    activate(&mut game, chandra, 1, Vec::new());

    assert_eq!(game.players[0].mana_pool.red, 2);
}

/// Four damage kills anything the format cares about.
#[test]
fn the_minus_three_burns_a_creature() {
    let (mut game, chandra) = staged();
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(
        &mut game,
        chandra,
        2,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(bears),
        )],
    );
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "four damage on a 2/2",
    );
}

/// The emblem shoots for every spell its controller casts afterwards.
#[test]
fn the_emblem_shoots_at_every_spell() {
    let (mut game, chandra) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == chandra)
    {
        permanent.set_counters(CounterKind::Loyalty, 7);
    }

    activate(&mut game, chandra, 3, Vec::new());
    assert_eq!(game.emblems.len(), 1, "the emblem is in the command zone");

    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { choices, .. } => choices
                .iter_targets()
                .any(|target| *target == Target::Player(PlayerId::Two)),
            _ => false,
        })
        .expect("a bolt in hand with red to spend, pointed across the table");
    game.apply(PlayerId::One, cast)
        .expect("the bolt is castable");
    settle_emblem(&mut game);

    assert_eq!(
        game.players[1].life,
        20 - 5 - 3,
        "five from the emblem and three from the bolt",
    );
}

/// Answers the emblem's target decision with the opponent, then lets
/// everything resolve.
fn settle_emblem(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .map(|option| vec![option.id])
                .expect("the emblem can point at the opponent");
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
