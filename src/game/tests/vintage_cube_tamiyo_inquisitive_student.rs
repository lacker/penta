//! Tamiyo, Inquisitive Student: one blue mana for a blocker that draws, and
//! a planeswalker on the turn the deck does what it was built to do.

use super::*;

/// Her on the battlefield since last turn, with `graveyard` behind her.
fn staged(graveyard: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    for (index, definition) in graveyard.iter().enumerate() {
        game.players[0].graveyard.push(card(
            97_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    let tamiyo = game
        .put_onto_battlefield(PlayerId::One, cards::TAMIYO_INQUISITIVE_STUDENT)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, tamiyo)
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

/// The permanent she is, whichever face is up.
fn tamiyo_on_battlefield(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::TAMIYO_INQUISITIVE_STUDENT)
        })
        .expect("she is on the battlefield")
}

fn is_planeswalker(game: &Game) -> bool {
    game.permanent_types(tamiyo_on_battlefield(game))
        .is_some_and(|types| types.contains(CardType::Planeswalker))
}

fn clues(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Clue"))
        })
        .count()
}

/// Activates the back face's `index`th loyalty ability -- 0 is +2, 1 is -3,
/// 2 is -7 -- naming `wanted` wherever it asks for a target.
fn activate_loyalty(game: &mut Game, index: u8, wanted: Option<GameObjectId>) {
    let tamiyo = tamiyo_on_battlefield(game).card.id;
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
                *source == tamiyo
                    && *ability == AbilityId(index)
                    && wanted.is_none_or(|wanted| {
                        targets
                            .iter()
                            .any(|slot| slot.targets().contains(&Target::Card(wanted)))
                    })
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("loyalty ability {index} is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

/// A 0/3 flier that makes a Clue when she attacks.
#[test]
fn she_investigates_when_she_attacks() {
    let (mut game, tamiyo) = staged(&[]);
    assert!(game.has_flying(tamiyo_on_battlefield(&game)), "flying");
    assert_eq!(game.power(tamiyo_on_battlefield(&game)), Some(0));
    assert_eq!(game.toughness(tamiyo_on_battlefield(&game)), Some(3));

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(tamiyo, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);

    assert_eq!(clues(&game), 1, "one Clue for the attack");
}

/// The third draw of the turn turns her over; the second does not.
#[test]
fn the_third_draw_turns_her_over() {
    let (mut game, _tamiyo) = staged(&[]);

    game.draw_cards(PlayerId::One, 2);
    settle(&mut game);
    assert!(!is_planeswalker(&game), "two draws is not three");

    game.draw_cards(PlayerId::One, 1);
    settle(&mut game);

    assert!(is_planeswalker(&game), "the third draw turned her over");
    assert_eq!(
        tamiyo_on_battlefield(&game).counters(CounterKind::Loyalty),
        2,
        "and she came back with her printed loyalty",
    );
}

/// The count is per turn: two draws this turn and one the next does not
/// reach three.
#[test]
fn the_count_resets_with_the_turn() {
    let (mut game, _tamiyo) = staged(&[]);
    game.draw_cards(PlayerId::One, 2);
    settle(&mut game);

    for _ in 0..40 {
        if game.turn > 9 && game.active_player == PlayerId::One {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }
    // The draw step already took one this turn, so one more makes two.
    game.draw_cards(PlayerId::One, 1);
    settle(&mut game);

    assert!(!is_planeswalker(&game), "a new turn starts the count over");
}

/// Sets her loyalty, for the abilities she cannot pay for on arrival.
fn set_loyalty(game: &mut Game, loyalty: u16) {
    let tamiyo = tamiyo_on_battlefield(game).card.id;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tamiyo)
    {
        permanent.set_counters(CounterKind::Loyalty, loyalty);
    }
}

/// Turns her over and hands priority back with the planeswalker up.
fn transformed(graveyard: &[CardDefinitionId]) -> Game {
    let (mut game, _tamiyo) = staged(graveyard);
    game.draw_cards(PlayerId::One, 3);
    settle(&mut game);
    game.players[0].hand.clear();
    assert!(is_planeswalker(&game), "she is the planeswalker now");
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// +2 shrinks whatever attacks her controller, and keeps doing it on the
/// opponent's turn -- which is the turn it was played for.
#[test]
fn the_plus_two_shrinks_attackers_until_your_next_turn() {
    let mut game = transformed(&[]);
    activate_loyalty(&mut game, 0, None);
    assert_eq!(
        tamiyo_on_battlefield(&game).counters(CounterKind::Loyalty),
        4,
    );

    let bears = creature(97_500, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(bears_id, AttackDefender::Player(PlayerId::One));
    game.finish_declaring_attackers();
    settle(&mut game);

    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("the attacker is there");
    assert_eq!(game.power(attacker), Some(1), "-1/-0");
    assert_eq!(game.toughness(attacker), Some(2), "and toughness untouched");
}

/// −3 takes a spell back, and a green one pays a mana for the trouble.
#[test]
fn the_minus_three_returns_a_spell_and_a_green_one_pays() {
    let mut game = transformed(&[cards::GIANT_GROWTH]);
    set_loyalty(&mut game, 3);
    let growth = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::GIANT_GROWTH)
        .expect("it is in the graveyard")
        .id;

    activate_loyalty(&mut game, 1, Some(growth));

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::GIANT_GROWTH],
    );
    assert_eq!(
        game.players[0].mana_pool.total(),
        1,
        "a green card adds one mana of any colour",
    );
}

/// A card that is not green returns just the same, and pays nothing.
#[test]
fn a_blue_card_returns_without_the_rebate() {
    let mut game = transformed(&[cards::ANCESTRAL_RECALL]);
    set_loyalty(&mut game, 3);
    let recall = game.players[0]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::ANCESTRAL_RECALL)
        .expect("it is in the graveyard")
        .id;

    activate_loyalty(&mut game, 1, Some(recall));

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].mana_pool.total(), 0, "not green, no mana");
}

/// −7 draws half the library rounded up and leaves an emblem behind.
#[test]
fn the_ultimate_draws_half_the_library_and_makes_an_emblem() {
    let mut game = transformed(&[]);
    set_loyalty(&mut game, 7);
    game.players[0].library.truncate(9);
    let library = game.players[0].library.len();

    activate_loyalty(&mut game, 2, None);

    assert_eq!(
        game.players[0].hand.len(),
        library.div_ceil(2),
        "nine halves up to five"
    );
    assert_eq!(game.emblems.len(), 1, "and the emblem stays");
}

/// The Clue she made is a real Clue: two mana and itself for a card. That
/// card is a draw like any other, so cracking it can be the third of the
/// turn and turn her over.
#[test]
fn cracking_her_clue_can_be_the_draw_that_turns_her() {
    let (mut game, tamiyo) = staged(&[]);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(tamiyo, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(clues(&game), 1, "one Clue for the attack");

    game.draw_cards(PlayerId::One, 2);
    settle(&mut game);
    assert!(!is_planeswalker(&game), "two draws is not three");

    let clue = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Clue"))
        })
        .expect("the Clue is there")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    let hand = game.players[0].hand.len();
    let crack = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == clue))
        .expect("two mana and the Clue itself buy a card");
    game.apply(PlayerId::One, crack).expect("it activates");
    settle(&mut game);

    assert_eq!(clues(&game), 0, "the Clue sacrificed itself");
    assert_eq!(
        game.players[0].hand.len(),
        hand + 1,
        "and drew the card it promised",
    );
    assert!(
        is_planeswalker(&game),
        "which was the third draw of the turn",
    );
}

/// "You can activate one of Tamiyo, Seasoned Scholar's loyalty abilities the
/// turn she enters the battlefield. However, you may do so only during one of
/// your main phases when the stack is empty." She often arrives in the middle
/// of combat, which is exactly when she cannot be used.
#[test]
fn her_loyalty_abilities_wait_for_a_main_phase() {
    let mut game = transformed(&[cards::GIANT_GROWTH]);
    let tamiyo = tamiyo_on_battlefield(&game).card.id;
    let loyalties = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, ability: AbilityOrigin::Printed { .. }, .. }
                        if *source == tamiyo
                )
            })
            .count()
    };
    assert!(
        loyalties(&game) > 0,
        "your own main phase with an empty stack is when she works",
    );

    // The step she usually arrives in, on the turn she flipped.
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::One;
    assert_eq!(
        loyalties(&game),
        0,
        "combat is no time for a loyalty ability",
    );

    // And their turn is not yours, main phase or not.
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert_eq!(loyalties(&game), 0, "nor is their main phase");
}

/// "The back face of a transforming double-faced card usually has a color
/// indicator that defines its color." Hers is blue, which the face has no
/// mana cost to say any other way.
#[test]
fn the_planeswalker_side_is_still_blue() {
    let game = transformed(&[]);
    let colors = game.permanent_colors(tamiyo_on_battlefield(&game));

    assert!(
        ManaColor::Blue
            .color_index()
            .is_some_and(|index| colors[index]),
        "the colour indicator is what she is read from",
    );
    assert_eq!(
        colors.iter().filter(|painted| **painted).count(),
        1,
        "and blue is the whole of it",
    );
}

/// "The mana value of a transforming double-faced card is the mana value of
/// its front face, no matter which face is up." She is a one-drop as a
/// creature and a one-drop as a planeswalker.
#[test]
fn her_mana_value_is_the_front_faces_either_way() {
    let (game, _tamiyo) = staged(&[]);
    assert_eq!(
        game.permanent_mana_value(tamiyo_on_battlefield(&game)),
        1,
        "{{U}} on the front face",
    );

    let game = transformed(&[]);
    assert_eq!(
        game.permanent_mana_value(tamiyo_on_battlefield(&game)),
        1,
        "and the back face has no mana cost of its own to read",
    );
}

/// The body: a 0/3 flier, which is what lets a one-drop attack into a board
/// and come back with a Clue.
#[test]
fn she_flies() {
    let (game, _tamiyo) = staged(&[]);
    let tamiyo = tamiyo_on_battlefield(&game);

    assert!(game.has_flying(tamiyo), "flying");
    assert_eq!(
        (game.power(tamiyo), game.toughness(tamiyo)),
        (Some(0), Some(3))
    );
}

/// "When you draw your third card in a turn": any turn, theirs included, so
/// three draws on their turn turn her over just as readily.
#[test]
fn three_draws_on_their_turn_turn_her_over_too() {
    let (mut game, _tamiyo) = staged(&[]);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;

    game.draw_cards(PlayerId::One, 2);
    settle(&mut game);
    assert!(!is_planeswalker(&game), "two is not three on any turn");

    game.draw_cards(PlayerId::One, 1);
    settle(&mut game);

    assert!(
        is_planeswalker(&game),
        "and the third turns her over on their turn as well",
    );
}

/// "In every zone other than the battlefield, consider only the
/// characteristics of its front face." In the graveyard she is a creature
/// card, so a Metamorphosis Fanatic looking for a creature to raise finds
/// her -- and comes back on that front face rather than as a planeswalker
/// with no loyalty. A graveyard of Bolts is the control: nothing to raise.
#[test]
fn in_the_graveyard_she_is_a_creature_card() {
    let raise_from = |buried: &[CardDefinitionId]| -> Game {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        game.players[PlayerId::One.index()].graveyard.clear();
        for (index, definition) in buried.iter().enumerate() {
            game.players[PlayerId::One.index()].graveyard.push(card(
                97_500 + u32::try_from(index).expect("few cards"),
                *definition,
                PlayerId::One,
            ));
        }
        game.turns_started = [5, 5];
        game.turn = 9;
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        game.put_onto_battlefield(PlayerId::One, cards::METAMORPHOSIS_FANATIC)
            .expect("cataloged");
        drain_pending(&mut game);
        game.check_state_based_actions();
        game
    };

    // Control: nothing in the graveyard is a creature card, so nothing rises.
    let game = raise_from(&[cards::LIGHTNING_BOLT]);
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "the Bolt is not a creature card and stays where it is",
    );

    let game = raise_from(&[cards::TAMIYO_INQUISITIVE_STUDENT, cards::LIGHTNING_BOLT]);
    let raised = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::TAMIYO_INQUISITIVE_STUDENT)
        })
        .expect("a creature card is what the graveyard held, so she came back");
    assert!(
        !game
            .permanent_types(raised)
            .is_some_and(|types| types.contains(CardType::Planeswalker)),
        "and she came back on her front face",
    );
    assert_eq!(
        (game.power(raised), game.toughness(raised)),
        (Some(0), Some(3)),
        "the 0/3 body rather than a walker with no loyalty",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and the Bolt beside her was never a candidate",
    );
}
