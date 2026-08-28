//! Liliana of the Veil: a symmetrical discard and an edict, on a body of
//! loyalty.

use super::*;

/// Liliana on the battlefield at `loyalty`, with `mine` and `theirs` in the
/// two hands.
fn staged(
    loyalty: u16,
    mine: &[CardDefinitionId],
    theirs: &[CardDefinitionId],
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let liliana = game
        .put_onto_battlefield(PlayerId::One, cards::LILIANA_OF_THE_VEIL)
        .expect("cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liliana)
    {
        permanent.set_counters(CounterKind::Loyalty, loyalty);
    }
    for (player, cards) in [(PlayerId::One, mine), (PlayerId::Two, theirs)] {
        for (index, definition) in cards.iter().enumerate() {
            let id =
                213_000 + u32::try_from(player.index() * 50 + index).expect("a handful of cards");
            game.players[player.index()]
                .hand
                .push(card(id, *definition, player));
        }
    }
    drain_pending(&mut game);
    game.turn = 4;
    game.turns_started = [4, 3];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, liliana)
}

/// Activates the loyalty ability at `index`, aimed at `target` when it wants
/// one, and answers whatever it asks by taking the first thing offered.
fn activate(game: &mut Game, liliana: GameObjectId, index: u8, target: Option<PlayerId>) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } => {
                *source == liliana
                    && *ability == AbilityId(index)
                    && target.is_none_or(|player| {
                        targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|chosen| *chosen == Target::Player(player))
                    })
            }
            _ => false,
        })
        .expect("the loyalty ability is offered");
    game.apply(PlayerId::One, action).expect("it activates");
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
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

/// "Each player": the plus one costs its controller a card too.
#[test]
fn the_plus_one_empties_a_card_from_each_hand() {
    let (mut game, liliana) = staged(3, &[cards::MOUNTAIN], &[cards::ISLAND, cards::FOREST]);

    activate(&mut game, liliana, 0, None);

    assert!(
        game.players[0].hand.is_empty(),
        "your own hand pays the same card",
    );
    assert_eq!(game.players[1].hand.len(), 1, "and so does theirs");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == liliana)
            .expect("she is still there")
            .counters(CounterKind::Loyalty),
        4,
        "and she grew by one",
    );
}

/// "You can activate Liliana's first ability even if some or all players will
/// be unable to discard a card": an empty hand discards nothing and the
/// ability resolves anyway.
#[test]
fn the_plus_one_works_with_nothing_to_discard() {
    let (mut game, liliana) = staged(3, &[], &[]);

    activate(&mut game, liliana, 0, None);

    assert!(game.players[0].hand.is_empty());
    assert!(game.players[1].hand.is_empty());
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == liliana)
            .expect("she is still there")
            .counters(CounterKind::Loyalty),
        4,
        "the loyalty went up either way",
    );
}

/// The minus two is an edict: the player it names picks which creature goes.
#[test]
fn the_minus_two_makes_them_sacrifice_a_creature() {
    let (mut game, liliana) = staged(3, &[], &[]);
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(&mut game, liliana, 1, Some(PlayerId::Two));

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.controller == PlayerId::Two),
        "the only creature they had is gone",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == mine),
        "and yours is untouched: the edict names a player, not a creature",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == liliana)
            .expect("she is still there")
            .counters(CounterKind::Loyalty),
        1,
        "two loyalty paid for it",
    );
}

/// A player with no creature sacrifices nothing, and the ability is still
/// activatable at them.
#[test]
fn the_minus_two_finds_nothing_on_an_empty_board() {
    let (mut game, liliana) = staged(3, &[], &[]);

    activate(&mut game, liliana, 1, Some(PlayerId::Two));

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == liliana)
            .expect("she is still there")
            .counters(CounterKind::Loyalty),
        1,
        "the loyalty was spent on nothing",
    );
    assert!(game.players[1].graveyard.is_empty());
}

/// The ultimate is a two-actor workflow: Liliana's controller partitions the
/// targeted player's permanents, then that player chooses which pile to lose.
#[test]
fn the_ultimate_passes_control_between_partition_and_choice() {
    let (mut game, liliana) = staged(6, &[], &[]);
    let permanents = [
        cards::SAVANNAH_LIONS,
        cards::GRIZZLY_BEARS,
        cards::WALKING_CORPSE,
        cards::ORNITHOPTER,
    ]
    .into_iter()
    .map(|definition| {
        game.put_onto_battlefield(PlayerId::Two, definition)
            .expect("cataloged")
    })
    .collect::<Vec<_>>();
    drain_pending(&mut game);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } => {
                *source == liliana
                    && *ability == AbilityId(2)
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the ultimate is offered");
    game.apply(PlayerId::One, action).expect("it activates");
    pass_priority_pair(&mut game);

    let partition = game
        .observe(PlayerId::One)
        .decision
        .expect("Liliana's controller partitions the permanents");
    assert_eq!(partition.player, PlayerId::One);
    let first = partition.options[..2]
        .iter()
        .map(|option| option.id)
        .collect::<Vec<_>>();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: partition.id,
            options: first,
        },
    )
    .expect("the partition is legal");

    let choice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the targeted player chooses a pile");
    assert_eq!(choice.player, PlayerId::Two);
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![0],
        },
    )
    .expect("the pile choice is legal");
    game.check_state_based_actions();

    for permanent in &permanents[..2] {
        assert!(
            !game
                .battlefield
                .iter()
                .any(|candidate| candidate.card.id == *permanent),
            "the chosen first pile is sacrificed",
        );
    }
    for permanent in &permanents[2..] {
        assert!(
            game.battlefield
                .iter()
                .any(|candidate| candidate.card.id == *permanent),
            "the unchosen pile remains",
        );
    }
}

/// "A pile can be empty. If the player chooses an empty pile, no permanents
/// will be sacrificed." Putting the whole board in one pile leaves the other
/// one empty, and that is the one they take.
#[test]
fn an_empty_pile_is_a_pile_they_may_choose() {
    let (mut game, liliana) = staged(6, &[], &[]);
    let mut theirs = Vec::new();
    for definition in [cards::SAVANNAH_LIONS, cards::SERRA_ANGEL, cards::FOREST] {
        theirs.push(
            game.put_onto_battlefield(PlayerId::Two, definition)
                .expect("cataloged"),
        );
    }
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let ultimate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } => {
                *source == liliana
                    && *ability == AbilityId(2)
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("six loyalty pays for the ultimate");
    game.apply(PlayerId::One, ultimate).expect("it activates");
    while game.pending_decisions.is_empty() && !game.stack.is_empty() {
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("nothing to answer yet");
    }

    let split = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the split is offered to Liliana's controller");
    let everything = split
        .options
        .iter()
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(everything.len(), 3, "their whole board is on offer");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: split.id,
            options: everything,
        },
    )
    .expect("one pile may hold all of it");

    let choice = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the victim chooses a pile");
    assert_eq!(choice.player, PlayerId::Two);
    assert_eq!(choice.options.len(), 2, "an empty pile is still a pile");
    let empty = choice
        .options
        .iter()
        .find(|option| option.members.is_empty())
        .expect("one of them holds nothing")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![empty],
        },
    )
    .expect("taking the empty pile is legal");
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        theirs.iter().all(|id| game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == *id)),
        "they sacrificed nothing at all",
    );
    assert!(
        game.players[1].graveyard.is_empty(),
        "and nothing reached their graveyard",
    );
}
