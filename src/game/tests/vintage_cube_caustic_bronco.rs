//! Caustic Bronco: a two-drop that draws an extra card every attack, and a
//! saddle that decides who pays for it.

use super::*;

/// The Bronco and `friends` on the battlefield, with `library` on top.
fn staged(friends: &[CardDefinitionId], library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].library.clear();
    // The library is a stack, so the last pushed is the top card.
    for (index, definition) in library.iter().rev().enumerate() {
        game.players[PlayerId::One.index()].library.push(card(
            92_000 + u32::try_from(index).expect("few cards"),
            *definition,
            PlayerId::One,
        ));
    }
    for (index, definition) in friends.iter().enumerate() {
        game.battlefield.push(creature(
            92_100 + u32::try_from(index).expect("few creatures"),
            *definition,
            PlayerId::One,
        ));
    }
    let bronco = game
        .put_onto_battlefield(PlayerId::One, cards::CAUSTIC_BRONCO)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, bronco)
}

fn saddle_action(game: &Game, bronco: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == bronco),
    )
}

/// Answers everything waiting by taking the first option offered.
fn settle(game: &mut Game) {
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
}

fn attack(game: &mut Game, bronco: GameObjectId) {
    game.step = Step::DeclareAttackers;
    game.declare_attacker(bronco, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(game);
}

fn saddled(game: &Game, bronco: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == bronco)
        .expect("it is there")
        .saddled
}

/// Unsaddled, the card it reveals costs you its mana value.
#[test]
fn attacking_unsaddled_costs_you_the_mana_value() {
    let (mut game, bronco) = staged(&[], &[cards::SERRA_ANGEL]);
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    attack(&mut game, bronco);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL],
        "the top card is revealed into your hand",
    );
    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        [before[0] - 5, before[1]],
        "a five-mana Angel costs you five",
    );
}

/// Saddled, the same reveal costs them instead.
#[test]
fn saddling_turns_the_drain_around() {
    let (mut game, bronco) = staged(
        &[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS],
        &[cards::SERRA_ANGEL],
    );
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    let saddle = saddle_action(&game, bronco).expect("two bears are four power");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);
    assert!(saddled(&game, bronco));

    attack(&mut game, bronco);

    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        [before[0], before[1] - 5],
        "saddled, the Angel costs them five",
    );
}

/// The saddle taps what pays for it, and only other creatures may.
#[test]
fn saddling_taps_the_creatures_that_paid() {
    let (mut game, bronco) = staged(&[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS], &[]);

    let saddle = saddle_action(&game, bronco).expect("two bears are four power");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
            .any(|permanent| permanent.tapped),
        "something was tapped to pay",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == bronco)
            .expect("it is there")
            .tapped,
        "the Mount itself is not what pays",
    );
}

/// Not enough power on the board is not an offer.
#[test]
fn one_small_creature_cannot_pay() {
    let (game, bronco) = staged(&[cards::SAVANNAH_LIONS], &[]);

    assert!(
        saddle_action(&game, bronco).is_none(),
        "two power is not three",
    );
}

/// Saddling is sorcery speed, and the saddle ends with the turn.
#[test]
fn it_saddles_only_as_a_sorcery_and_only_for_the_turn() {
    let (mut game, bronco) = staged(&[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS], &[]);
    game.step = Step::Upkeep;
    assert!(
        saddle_action(&game, bronco).is_none(),
        "an upkeep is not a main phase",
    );

    game.step = Step::PrecombatMain;
    let saddle = saddle_action(&game, bronco).expect("a main phase is");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);
    assert!(saddled(&game, bronco));

    game.cleanup();
    assert!(!saddled(&game, bronco), "the saddle ends with the turn");
}

/// "If the revealed card doesn't have a mana cost (because it's a land card,
/// for example), its mana value is 0." The card still comes to hand; nobody
/// pays for it.
#[test]
fn a_land_off_the_top_costs_nobody_anything() {
    let (mut game, bronco) = staged(&[], &[cards::MOUNTAIN]);
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    attack(&mut game, bronco);

    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        before,
        "a land is mana value nought either way round",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "and it is in hand all the same",
    );
}

/// "If Caustic Bronco isn't on the battlefield as its triggered ability
/// resolves, use whether it was saddled or not before it left." Killing it
/// with the trigger on the stack does not turn the drain back around.
#[test]
fn a_dead_bronco_is_remembered_as_saddled() {
    let (mut game, bronco) = staged(
        &[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS],
        &[cards::SERRA_ANGEL],
    );
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];
    let saddle = saddle_action(&game, bronco).expect("two bears are four power");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(bronco, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    // The trigger is on the stack; the Bronco is answered before it resolves.
    game.move_permanents_to_graveyard(&[bronco]);
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bronco),
        "it is gone",
    );
    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        [before[0], before[1] - 5],
        "and it was saddled when it left, so they still pay",
    );
}

/// "If the revealed card's mana cost includes {X}, X is 0 for the purpose of
/// determining its mana value." A Walking Ballista is printed {X}{X}, which
/// off the top of a library is nothing at all.
#[test]
fn an_x_in_the_revealed_cost_counts_as_zero() {
    let (mut game, bronco) = staged(&[], &[cards::WALKING_BALLISTA]);
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];

    attack(&mut game, bronco);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::WALKING_BALLISTA],
        "it still comes to hand",
    );
    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        before,
        "and an unpaid X is worth nothing to either of you",
    );
}

/// "You may activate a permanent's saddle ability even if that permanent is
/// already saddled." It buys nothing, but nothing stops it either.
#[test]
fn an_already_saddled_mount_may_be_saddled_again() {
    let (mut game, bronco) = staged(&[cards::GRIZZLY_BEARS; 4], &[]);

    let saddle = saddle_action(&game, bronco).expect("eight power is enough");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);
    assert!(saddled(&game, bronco));

    assert!(
        saddle_action(&game, bronco).is_some(),
        "the bears still standing may pay for it a second time",
    );
}

/// "Tap any number of other untapped creatures you control": a creature that
/// is already tapped has nothing left to give.
#[test]
fn tapped_creatures_cannot_pay_the_saddle() {
    let (mut game, bronco) = staged(&[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS], &[]);
    for permanent in &mut game.battlefield {
        if permanent.card.definition == cards::GRIZZLY_BEARS {
            permanent.tapped = true;
        }
    }

    assert!(
        saddle_action(&game, bronco).is_none(),
        "four power that is already tapped is no power at all",
    );
}

/// "A split card's characteristics are a combination of its two halves while
/// it is not on the stack." The card the Bronco turns over is in a library,
/// so Life // Death is worth {G} and {1}{B} together: three, not one.
#[test]
fn a_split_card_off_the_top_costs_both_halves() {
    let (mut game, bronco) = staged(&[], &[cards::LIFE_DEATH]);
    let before = game.players[PlayerId::One.index()].life;

    attack(&mut game, bronco);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIFE_DEATH],
        "the one card it revealed",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before - 3,
        "one for the Life half and two for the Death half",
    );
}

/// "While in any zone other than the stack or the battlefield, a Room card's
/// characteristics are a combination of its two doors." Off the top of a
/// library that is {1}{R} and {3}{U}{U} together: seven, not two.
#[test]
fn a_room_card_off_the_top_costs_both_doors() {
    let (mut game, bronco) = staged(&[], &[cards::ROARING_FURNACE]);
    let before = game.players[PlayerId::One.index()].life;

    attack(&mut game, bronco);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before - 7,
        "two for the Furnace and five for the Sauna",
    );
}

/// "'Saddled' isn't an ability that a creature has. It's just something true
/// about that creature. It won't stop being saddled until the turn ends or
/// it leaves the battlefield." A Bronco bounced and replayed the same turn
/// is a new object with nothing true about it, so the drain turns back
/// around.
#[test]
fn leaving_the_battlefield_unsaddles_it() {
    let (mut game, bronco) = staged(
        &[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS],
        &[cards::SERRA_ANGEL],
    );
    let before = [
        game.players[PlayerId::One.index()].life,
        game.players[PlayerId::Two.index()].life,
    ];
    let saddle = saddle_action(&game, bronco).expect("two bears are four power");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);
    assert!(saddled(&game, bronco), "saddled for the turn");

    game.return_permanent_to_hand(bronco);
    drain_pending(&mut game);
    let replayed = game
        .put_onto_battlefield(PlayerId::One, cards::CAUSTIC_BRONCO)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    assert!(
        !saddled(&game, replayed),
        "what came back is a new object and nothing is true about it",
    );

    attack(&mut game, replayed);

    assert_eq!(
        [
            game.players[PlayerId::One.index()].life,
            game.players[PlayerId::Two.index()].life,
        ],
        [before[0] - 5, before[1]],
        "so the Angel costs its own controller five again",
    );
}

/// "If a permanent becomes a copy of a saddled Mount, the copy won't be
/// saddled." Being saddled is not a copiable value, so a Clone that arrives
/// as a saddled Bronco arrives unsaddled and drains its own controller.
#[test]
fn a_copy_of_a_saddled_bronco_is_not_saddled() {
    let (mut game, bronco) = staged(
        &[cards::GRIZZLY_BEARS, cards::GRIZZLY_BEARS],
        &[cards::SERRA_ANGEL, cards::SERRA_ANGEL],
    );
    let saddle = saddle_action(&game, bronco).expect("two bears are four power");
    game.apply(PlayerId::One, saddle).expect("it activates");
    settle(&mut game);
    assert!(saddled(&game, bronco), "the original is saddled");

    let held = card(92_500, cards::CLONE, PlayerId::One);
    let clone = held.id;
    game.players[PlayerId::One.index()].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == clone))
        .expect("four mana casts the Clone");
    game.apply(PlayerId::One, cast).expect("it is cast");
    // Two Bears are on the board beside the Bronco, so the copy is named
    // rather than taken as whatever the entry replacement offered first.
    for _ in 0..16 {
        let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        else {
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            if game.apply(game.priority, Action::PassPriority).is_err() {
                break;
            }
            continue;
        };
        let wanted = decision
            .options
            .iter()
            .find(|option| option.label == "Enter as a copy of Caustic Bronco")
            .or_else(|| decision.options.first());
        let options = wanted.map(|option| vec![option.id]).unwrap_or_default();
        game.apply(
            decision.player,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .expect("the decision accepts what it offered");
    }
    settle(&mut game);
    drain_pending(&mut game);
    game.check_state_based_actions();
    // The Clone changes zones on its way in, so what stands there is a new
    // object: it is found by what it copied rather than by the card's id.
    let copy = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.id != bronco
                && Game::effective_rules_source(permanent)
                    == ObjectCharacteristics::card(cards::CAUSTIC_BRONCO, CardPartId::PRIMARY)
        })
        .expect("the Clone arrived as a second Bronco");

    assert!(
        !copy.saddled,
        "and copied nothing about the original being saddled",
    );
    assert!(
        saddled(&game, bronco),
        "while the original is saddled still",
    );
}

/// "Creatures with saddle can attack or block as normal even if they aren't
/// saddled." The attacking half is all over this file; the blocking half is
/// the point of the ruling -- a Mount is not a Vehicle and needs nobody in
/// the seat to stand in the way. Its trigger watches attacking, so blocking
/// reveals nothing and costs nobody a point.
#[test]
fn an_unsaddled_bronco_blocks_and_reveals_nothing() {
    let (mut game, bronco) = staged(&[], &[cards::SERRA_ANGEL]);
    assert!(!saddled(&game, bronco), "nobody is riding it");
    let attacker = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let library = game.players[PlayerId::One.index()].library.len();

    // Their turn, their bear, and the Bronco steps in front of it.
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.declare_attacker(attacker, AttackDefender::Player(PlayerId::One));
    game.finish_declaring_attackers();
    game.step = Step::DeclareBlockers;
    game.declare_blocker(bronco, attacker);
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    game.check_state_based_actions();

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        20,
        "the bear was blocked, and no trigger drained anybody",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        20,
        "least of all them",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library,
        "and nothing was revealed: the trigger watches attacking",
    );
    assert!(
        game.players[PlayerId::One.index()].hand.is_empty(),
        "so the Angel is still on top rather than in hand",
    );

    // Two power each way: the bear dies and so does the Bronco.
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attacker),
        "a 2/2 blocked by a 2/2 trades",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != bronco),
        "and the Bronco went with it, having blocked like any creature",
    );
}
