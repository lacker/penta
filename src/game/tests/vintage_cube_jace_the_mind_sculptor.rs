//! Jace, the Mind Sculptor: four abilities, three of which decide games.

use super::*;

/// Player One with a Jace out since last turn, `library` stacked on their
/// own so the last entry is on top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    if !library.is_empty() {
        game.players[0].library.clear();
        for definition in library {
            let card = game
                .build_zone(PlayerId::One, &[*definition])
                .expect("cataloged")
                .into_iter()
                .next()
                .expect("one card");
            game.players[0].library.push(card);
        }
    }
    let jace = game
        .put_onto_battlefield(PlayerId::One, cards::JACE_THE_MIND_SCULPTOR)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, jace)
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

/// Activates the loyalty ability at `index`, aimed at `target` when it takes
/// one, and lets the stack settle.
fn activate(game: &mut Game, jace: GameObjectId, index: u8, target: Option<Target>) {
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
                *source == jace
                    && *ability == AbilityId(index)
                    && target.is_none_or(|wanted| {
                        targets
                            .iter()
                            .any(|selection| selection.targets().contains(&wanted))
                    })
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("loyalty ability {index} is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

/// Answers whatever is pending with the option naming `wanted`, or with none
/// when nothing is named.
fn answer(game: &mut Game, wanted: &[CardDefinitionId]) {
    let seat = deciding(game).expect("somebody is being asked");
    let decision = game.observe(seat).decision.expect("just checked");
    let options = wanted
        .iter()
        .map(|definition| {
            decision
                .options
                .iter()
                .find(|option| {
                    option
                        .card
                        .is_some_and(|(_, found)| found.card_definition() == Some(*definition))
                })
                .unwrap_or_else(|| panic!("{definition:?} is offered"))
                .id
        })
        .collect();
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("the answer is legal");
    settle(game);
}

fn loyalty(game: &Game, jace: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == jace)
        .expect("he is on the battlefield")
        .counters(CounterKind::Loyalty)
}

/// He enters with three, and the plus takes him to five.
#[test]
fn the_fateseal_costs_him_two_upward() {
    let (mut game, jace) = staged(&[cards::MOUNTAIN, cards::SERRA_ANGEL]);
    assert_eq!(loyalty(&game, jace), 3, "three to start");

    activate(&mut game, jace, 0, Some(Target::Player(PlayerId::One)));
    answer(&mut game, &[]);

    assert_eq!(loyalty(&game, jace), 5, "and five after");
}

/// "You may put that card on the bottom": taking it buries the top card.
#[test]
fn the_fateseal_can_bottom_the_top_card() {
    let (mut game, jace) = staged(&[cards::MOUNTAIN, cards::SERRA_ANGEL]);

    activate(&mut game, jace, 0, Some(Target::Player(PlayerId::One)));
    answer(&mut game, &[cards::SERRA_ANGEL]);

    assert_eq!(
        game.players[0].library.last().map(|card| card.definition),
        Some(cards::MOUNTAIN),
        "the Angel went to the bottom and the Mountain is on top",
    );
    assert_eq!(game.players[0].library.len(), 2, "still two cards");
}

/// "You may": leaving it alone puts it back on top.
#[test]
fn the_fateseal_may_leave_it_on_top() {
    let (mut game, jace) = staged(&[cards::MOUNTAIN, cards::SERRA_ANGEL]);

    activate(&mut game, jace, 0, Some(Target::Player(PlayerId::One)));
    answer(&mut game, &[]);

    assert_eq!(
        game.players[0].library.last().map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "the Angel stayed where it was",
    );
}

/// It aims at a player, so their library is what gets looked at.
#[test]
fn the_fateseal_can_point_at_them() {
    let (mut game, jace) = staged(&[]);
    let before = game.players[1].library.len();

    activate(&mut game, jace, 0, Some(Target::Player(PlayerId::Two)));

    let seat = deciding(&game).expect("the looking is offered");
    assert_eq!(
        seat,
        PlayerId::One,
        "and it is Jace's controller who looks, not the player looked at",
    );
    answer(&mut game, &[]);
    assert_eq!(
        game.players[1].library.len(),
        before,
        "their library is the same size either way",
    );
}

/// The zero is Brainstorm: three in, two back, and the loyalty unchanged.
#[test]
fn the_zero_draws_three_and_puts_two_back() {
    let (mut game, jace) = staged(&[]);
    let before = game.players[0].library.len();

    activate(&mut game, jace, 1, None);
    let seat = deciding(&game).expect("it asks which two go back");
    let held = game.observe(seat).decision.expect("just checked").options[..2]
        .iter()
        .map(|option| option.id)
        .collect::<Vec<_>>();
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: game.observe(seat).decision.expect("just checked").id,
            options: held,
        },
    )
    .expect("the answer is legal");
    settle(&mut game);

    assert_eq!(game.players[0].hand.len(), 1, "three drawn, two put back");
    assert_eq!(
        game.players[0].library.len(),
        before - 1,
        "one card of real advantage",
    );
    assert_eq!(loyalty(&game, jace), 3, "and the loyalty is untouched");
}

/// The minus bounces a creature, and costs him one.
#[test]
fn the_minus_one_bounces_a_creature() {
    let (mut game, jace) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    activate(&mut game, jace, 2, Some(Target::Permanent(bears)));

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears),
        "the Bears left the battlefield",
    );
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "to its owner's hand, not yours",
    );
    assert_eq!(loyalty(&game, jace), 2, "and it cost him one");
}

/// The ultimate empties a library and leaves them the hand they were
/// holding, shuffled in.
#[test]
fn the_ultimate_exiles_a_library_and_shuffles_the_hand_in() {
    let (mut game, jace) = staged(&[]);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == jace)
    {
        permanent.set_counters(CounterKind::Loyalty, 12);
    }
    let library = game.players[1].library.len();
    let held = game
        .build_zone(PlayerId::Two, &[cards::SERRA_ANGEL, cards::MOUNTAIN])
        .expect("cataloged");
    let hand = held.len();
    game.players[1].hand.extend(held);

    activate(&mut game, jace, 3, Some(Target::Player(PlayerId::Two)));

    assert_eq!(
        game.players[1].exile.len(),
        library,
        "their whole library is in exile",
    );
    assert!(game.players[1].hand.is_empty(), "their hand went in");
    assert_eq!(
        game.players[1].library.len(),
        hand,
        "and their library is what the hand was",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == jace),
        "twelve loyalty is all of it, so he goes to the graveyard",
    );
}

/// He is not free: a loyalty ability is once a turn at sorcery speed.
#[test]
fn only_one_loyalty_ability_each_turn() {
    let (mut game, jace) = staged(&[]);

    activate(&mut game, jace, 0, Some(Target::Player(PlayerId::One)));
    answer(&mut game, &[]);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == jace)
        }),
        "he has already gone this turn",
    );
}
