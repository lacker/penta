//! Esika's Chariot: four mana that brings its own crew, and copies one of
//! them every time it swings.

use super::*;

/// The Chariot on the battlefield with its enters trigger already answered.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let chariot = game
        .put_onto_battlefield(PlayerId::One, cards::ESIKA_S_CHARIOT)
        .expect("cataloged");
    drain_pending(&mut game);
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [4, 4];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, chariot)
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
                .map(|option| option.id)
                .take(decision.minimum.max(1))
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

fn cats(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                crate::card::TokenCharacteristics::creature(&["Cat"], &[ManaColor::Green], 2, 2),
            )
        })
        .collect()
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn is_creature(game: &Game, id: GameObjectId) -> bool {
    game.permanent_types(permanent(game, id))
        .is_some_and(|types| types.contains(CardType::Creature))
}

/// Two Cats on the way in, which are exactly enough power to crew it.
#[test]
fn it_brings_two_cats() {
    let (game, _chariot) = staged();

    let cats = cats(&game);
    assert_eq!(cats.len(), 2, "two of them");
    assert_eq!(game.power(cats[0]), Some(2), "each a 2/2");
}

/// Crew 4: the two Cats it came with are exactly four power between them.
#[test]
fn its_own_cats_crew_it() {
    let (mut game, chariot) = staged();
    assert!(!is_creature(&game, chariot), "a Vehicle until crewed");

    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("four power is on the battlefield");
    game.apply(PlayerId::One, crew).expect("it crews");
    settle(&mut game);

    assert!(is_creature(&game, chariot));
    assert_eq!(game.power(permanent(&game, chariot)), Some(4));
    assert!(
        cats(&game).iter().all(|cat| cat.tapped),
        "and both Cats tapped to do it",
    );
}

/// Attacking copies a token: another Cat, which is another two power for
/// next turn's crew.
#[test]
fn attacking_copies_one_of_them() {
    let (mut game, chariot) = staged();
    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("four power is on the battlefield");
    game.apply(PlayerId::One, crew).expect("it crews");
    settle(&mut game);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: chariot,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a crewed Chariot may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    settle(&mut game);

    assert_eq!(cats(&game).len(), 3, "the attack made a third Cat");
}

/// "Target token you control": the Chariot is not a token and neither is
/// anything else on an empty board, so only the Cats are on offer.
#[test]
fn only_tokens_are_legal_targets() {
    let (mut game, chariot) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let cats = cats(&game)
        .iter()
        .map(|cat| cat.card.id)
        .collect::<Vec<_>>();
    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("four power is on the battlefield");
    game.apply(PlayerId::One, crew).expect("it crews");
    settle(&mut game);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: chariot,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a crewed Chariot may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let mut offered = game
        .pending_decisions
        .first()
        .expect("the attack trigger is asking")
        .observation
        .options
        .iter()
        .filter_map(|option| option.card.map(|(id, _)| id))
        .collect::<Vec<_>>();
    offered.sort_unstable();
    let mut expected = cats;
    expected.sort_unstable();
    assert_eq!(offered, expected, "the Cats and nothing else");
}

/// Crew the Chariot by naming the given creatures, one at a time, and then
/// declining to tap anything more.
fn crew_with(game: &mut Game, chariot: GameObjectId, payers: &[GameObjectId]) {
    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("four power is on the battlefield");
    game.apply(PlayerId::One, crew).expect("it crews");
    for _ in 0..4 {
        let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        else {
            break;
        };
        let bear = decision.options.iter().find(|option| {
            option
                .card
                .as_ref()
                .is_some_and(|(id, _)| payers.contains(id))
        });
        let chosen = bear
            .or_else(|| decision.options.first())
            .expect("an option");
        let stopping = bear.is_none();
        game.apply(
            decision.player,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![chosen.id],
            },
        )
        .expect("the offered choice is legal");
        if stopping {
            break;
        }
    }
}

/// A copy is of the token, not of its situation: copying a Cat that is
/// tapped and attacking makes a Cat that is untapped and at home.
#[test]
fn the_copy_arrives_untapped_and_at_home() {
    let (mut game, chariot) = staged();
    // Bears to pay for crew, so both Cats are free to swing alongside.
    for id in [90_001, 90_002] {
        let mut bear = creature(id, cards::GRIZZLY_BEARS, PlayerId::One);
        bear.entered_controller_turn = 0;
        game.battlefield.push(bear);
    }
    let bears = [GameObjectId(90_001), GameObjectId(90_002)];

    crew_with(&mut game, chariot, &bears);
    settle(&mut game);
    assert!(
        cats(&game).iter().all(|cat| !cat.tapped),
        "the Bears paid, so the Cats are still standing",
    );

    let before = cats(&game)
        .iter()
        .map(|cat| cat.card.id)
        .collect::<Vec<_>>();
    let swinging = before[0];
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    for attacker in [chariot, swinging] {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .expect("a crewed Chariot and a settled Cat may both attack");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        permanent(&game, swinging).tapped,
        "the attacking Cat is tapped and attacking, which is what makes this worth asking",
    );

    let decision = game
        .pending_decisions
        .first()
        .expect("the attack trigger is asking")
        .observation
        .clone();
    let attacking_cat = decision
        .options
        .iter()
        .find(|option| option.card.as_ref().is_some_and(|(id, _)| *id == swinging))
        .expect("the Cat that is attacking is a token you control");
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![attacking_cat.id],
        },
    )
    .expect("targeting it is legal");
    settle(&mut game);

    let copy = cats(&game)
        .into_iter()
        .find(|cat| !before.contains(&cat.card.id))
        .expect("the trigger made a third Cat");
    assert!(!copy.tapped, "the copy is untapped");
    assert_eq!(
        copy.attack_defender, None,
        "and it is not attacking, however its original arrived",
    );
}

/// "The newly created token doesn't copy ... whether it has any counters on
/// it": a Cat grown by two comes back the 2/2 the Chariot printed.
#[test]
fn counters_on_the_original_are_not_copied() {
    let (mut game, chariot) = staged();
    let bears = [90_001, 90_002].map(|id| {
        let mut bear = creature(id, cards::GRIZZLY_BEARS, PlayerId::One);
        bear.entered_controller_turn = 0;
        let bear_id = bear.card.id;
        game.battlefield.push(bear);
        bear_id
    });
    let grown = cats(&game)[0].card.id;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == grown)
        .expect("a Cat is there")
        .add_counters(CounterKind::PlusOnePlusOne, 2);
    crew_with(&mut game, chariot, &bears);
    settle(&mut game);

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: chariot,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a crewed Chariot may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the attack trigger is asking");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.as_ref().is_some_and(|(id, _)| *id == grown))
        .expect("the grown Cat is a token you control")
        .id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("naming it is legal");
    settle(&mut game);

    let copy = cats(&game)
        .into_iter()
        .find(|cat| cat.card.id != grown)
        .expect("the trigger made another Cat");
    assert_eq!(
        copy.counters(CounterKind::PlusOnePlusOne),
        0,
        "the counters stayed with the Cat that had them",
    );
    assert_eq!(game.power(copy), Some(2), "so the copy is the printed 2/2");
}

/// "Target token you control": theirs is a token, and not one of yours.
#[test]
fn their_tokens_are_not_on_the_menu() {
    let (mut game, chariot) = staged();
    game.put_onto_battlefield(PlayerId::Two, cards::ESIKA_S_CHARIOT)
        .expect("cataloged");
    drain_pending(&mut game);
    let theirs = game
        .battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::Two && permanent.card.definition == ObjectKind::Token
        })
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    assert_eq!(theirs.len(), 2, "their Chariot brought Cats of its own");

    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == chariot => Some(targets),
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .collect::<Vec<_>>();
    assert!(
        theirs
            .iter()
            .all(|id| !offered.contains(&Target::Permanent(*id))),
        "their Cats are nobody's to copy but theirs",
    );
}

/// Attacks with an already-crewed Chariot, answering the copy trigger by
/// naming `wanted` rather than whatever is listed first.
fn attack_copying(game: &mut Game, chariot: GameObjectId, wanted: GameObjectId) {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: chariot,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a crewed Chariot may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    for _ in 0..24 {
        let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        else {
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            let priority = game.priority;
            if game.apply(priority, Action::PassPriority).is_err() {
                break;
            }
            continue;
        };
        let chosen = decision
            .options
            .iter()
            .find(|option| option.card.is_some_and(|(id, _)| id == wanted))
            .map_or_else(
                || {
                    decision
                        .options
                        .iter()
                        .map(|option| option.id)
                        .take(decision.minimum.max(1))
                        .collect::<Vec<_>>()
                },
                |option| vec![option.id],
            );
        game.apply(
            decision.player,
            Action::ChooseDecision {
                decision: decision.id,
                options: chosen,
            },
        )
        .expect("the offered choice is legal");
    }
    game.check_state_based_actions();
}

/// "If the original token is copying something else, the token you create
/// will use the copiable values of the original token." The Chariot makes
/// copy tokens itself, so the second attack can point at the one the first
/// attack made -- and what comes back is another 2/2 green Cat rather than
/// anything degraded.
#[test]
fn copying_a_copy_makes_another_cat() {
    let (mut game, chariot) = staged();
    let originals = cats(&game)
        .iter()
        .map(|cat| cat.card.id)
        .collect::<Vec<_>>();
    assert_eq!(originals.len(), 2, "the two it came with");

    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("four power is on the battlefield");
    game.apply(PlayerId::One, crew).expect("it crews");
    settle(&mut game);
    attack_copying(&mut game, chariot, originals[0]);

    let copy = cats(&game)
        .iter()
        .map(|cat| cat.card.id)
        .find(|id| !originals.contains(id))
        .expect("the first attack made a third Cat");

    // A fresh turn so the Chariot may attack again, crewed by the two Cats
    // that did not just come into being. The combat it was in has to be
    // cleared too, or it is still an attacker and cannot be declared twice.
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
        permanent.attacking = false;
    }
    game.turns_started = [5, 5];
    game.step = Step::PrecombatMain;
    game.attackers_declared = false;
    let crew = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == chariot),
        )
        .expect("there is plenty of power now");
    game.apply(PlayerId::One, crew).expect("it crews again");
    settle(&mut game);

    attack_copying(&mut game, chariot, copy);

    assert_eq!(cats(&game).len(), 4, "a copy of a copy is still a Cat");
    let newest = cats(&game)
        .iter()
        .map(|cat| cat.card.id)
        .find(|id| !originals.contains(id) && *id != copy)
        .expect("the second attack made a fourth");
    let cat = permanent(&game, newest);
    assert_eq!(
        (game.power(cat), game.toughness(cat)),
        (Some(2), Some(2)),
        "the copiable values it read were the Cat's own",
    );
    assert!(
        game.effective_subtypes(cat)
            .contains(crate::card::Subtype::named("Cat")),
        "and it is a Cat like the rest of them",
    );
    assert!(
        !cat.tapped,
        "arriving untapped and at home, as any copy does",
    );
}
