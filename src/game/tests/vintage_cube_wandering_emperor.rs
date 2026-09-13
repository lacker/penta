//! The Wandering Emperor: a planeswalker you cast on the other player's
//! turn, whose loyalty abilities are open the moment she lands.

use super::*;

/// Player One with the Emperor freshly arrived during Player Two's combat,
/// which is the window the card is built for.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started = [5, 5];
    game.turn = 10;
    let emperor = game
        .put_onto_battlefield(PlayerId::One, cards::THE_WANDERING_EMPEROR)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.priority = PlayerId::One;
    (game, emperor)
}

fn loyalty_action(game: &Game, emperor: GameObjectId, ability: u8) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability: id, .. },
                ..
            } => *source == emperor && *id == AbilityId(ability),
            _ => false,
        })
}

/// Whether this action activates that printed ability of hers at that
/// permanent.
fn targets_with(action: &Action, emperor: GameObjectId, ability: u8, target: GameObjectId) -> bool {
    match action {
        Action::ActivateAbility {
            source,
            ability: AbilityOrigin::Printed { ability: id, .. },
            targets,
            ..
        } => {
            *source == emperor
                && *id == AbilityId(ability)
                && targets
                    .iter()
                    .flat_map(crate::casting::TargetSelection::targets)
                    .any(|chosen| *chosen == Target::Permanent(target))
        }
        _ => false,
    }
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
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

fn tapped_creature(game: &mut Game, id: u32, controller: PlayerId) -> GameObjectId {
    let mut permanent = creature(id, cards::GRIZZLY_BEARS, controller);
    permanent.tapped = true;
    let object = permanent.card.id;
    game.battlefield.push(permanent);
    object
}

/// She entered this turn, so her loyalty abilities are open on the other
/// player's turn, in a step no sorcery could be cast in.
#[test]
fn her_loyalty_is_open_the_turn_she_lands() {
    let (game, emperor) = staged();

    assert!(
        loyalty_action(&game, emperor, 2).is_some(),
        "the plus is activatable in the opponent's end step",
    );
}

/// And closed once that turn is over: "this turn" is the turn itself, not
/// the controller's own turn count, so the opponent's next turn is too late.
#[test]
fn her_loyalty_closes_after_the_turn_she_lands() {
    let (mut game, emperor) = staged();
    game.turn += 1;

    assert!(
        loyalty_action(&game, emperor, 2).is_none(),
        "a later turn is an ordinary sorcery-speed window",
    );

    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    assert!(
        loyalty_action(&game, emperor, 2).is_some(),
        "and her own main phase still is one",
    );
}

/// The plus grows a creature and gives it first strike.
#[test]
fn the_plus_grows_a_creature() {
    let (mut game, emperor) = staged();
    let bears = creature(130_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let plus = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| targets_with(action, emperor, 2, bears_id))
        .expect("she may name a creature");
    game.apply(PlayerId::One, plus).expect("it activates");
    settle(&mut game);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("still there");
    assert_eq!(bears.counters(CounterKind::PlusOnePlusOne), 1);
    assert!(
        game.permanent_has_executable_keyword(bears, KeywordAbility::FirstStrike),
        "and it strikes first for the turn",
    );
}

/// The first minus makes a body that can block and still attack later.
#[test]
fn the_first_minus_makes_a_samurai() {
    let (mut game, emperor) = staged();

    let minus = loyalty_action(&game, emperor, 3).expect("she is fresh");
    game.apply(PlayerId::One, minus).expect("it activates");
    settle(&mut game);

    let samurai = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Samurai"))
        })
        .expect("a token arrived");
    assert_eq!(game.power(samurai), Some(2));
    assert_eq!(game.toughness(samurai), Some(2));
    assert!(game.permanent_has_executable_keyword(samurai, KeywordAbility::Vigilance));
}

/// The second minus answers a creature that has already tapped, and only
/// one that has.
#[test]
fn the_second_minus_exiles_a_tapped_creature() {
    let (mut game, emperor) = staged();
    let attacker = tapped_creature(&mut game, 130_200, PlayerId::Two);
    let untapped = creature(130_201, cards::GRIZZLY_BEARS, PlayerId::Two);
    let untapped_id = untapped.card.id;
    game.battlefield.push(untapped);

    let exile = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| targets_with(action, emperor, 4, attacker))
        .expect("a tapped creature is a legal target");
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| targets_with(action, emperor, 4, untapped_id)),
        "an untapped one is not",
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| targets_with(action, emperor, 2, untapped_id)),
        "though the plus may name it",
    );

    game.apply(PlayerId::One, exile).expect("it activates");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == attacker),
        "the tapped creature is gone",
    );
    assert_eq!(game.players[0].life, 22, "and two life came with it");
}

/// Flash: she is cast on their turn, which is the only reason the timing
/// permission beside it is worth anything.
#[test]
fn she_has_flash() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let emperor = card(130_300, cards::THE_WANDERING_EMPEROR, PlayerId::One);
    let emperor_id = emperor.id;
    game.players[0].hand.push(emperor);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == emperor_id)),
        "a planeswalker with flash is castable in their end step",
    );
}

/// "You may still only activate one of The Wandering Emperor's loyalty
/// abilities on the turn she entered the battlefield." The permission moves
/// when they may be used, not how many.
#[test]
fn instant_speed_is_still_one_ability_a_turn() {
    let (mut game, emperor) = staged();

    let samurai = loyalty_action(&game, emperor, 3).expect("she is fresh");
    game.apply(PlayerId::One, samurai).expect("it activates");
    settle(&mut game);

    assert!(
        loyalty_action(&game, emperor, 2).is_none(),
        "the plus is closed for the rest of the turn",
    );
    assert!(
        loyalty_action(&game, emperor, 3).is_none(),
        "and so is the one she just used",
    );
    assert!(
        loyalty_action(&game, emperor, 4).is_none(),
        "one loyalty ability a turn is one, whatever the timing permission says",
    );
}

/// "Up to one target creature": with nobody on the battlefield the plus is
/// still activatable, and it is loyalty gained for nothing else.
#[test]
fn the_plus_may_name_nobody() {
    let (mut game, emperor) = staged();
    let loyalty = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == emperor)
            .expect("she is there")
            .counters(CounterKind::Loyalty)
    };
    assert_eq!(loyalty(&game), 3, "her printed loyalty");

    let plus = loyalty_action(&game, emperor, 2)
        .expect("up to one means none is an answer, even on an empty board");
    game.apply(PlayerId::One, plus).expect("it activates");
    settle(&mut game);

    assert_eq!(loyalty(&game), 4, "and the counter went on regardless");
}

/// The minus names a tapped creature and says nothing about whose: your own
/// attacker, tapped from swinging, is as legal a target as theirs -- and the
/// two life is yours either way.
#[test]
fn the_second_minus_reaches_your_own_tapped_creature() {
    let (mut game, emperor) = staged();
    let mine = tapped_creature(&mut game, 130_400, PlayerId::One);
    let life = game.players[PlayerId::One.index()].life;

    let exile = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| targets_with(action, emperor, 4, mine))
        .expect("a tapped creature of your own is a legal target");
    game.apply(PlayerId::One, exile).expect("it activates");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == mine),
        "your own creature is what you exiled",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life + 2,
        "and the two life is the caster's, whoever the creature belonged to",
    );
}
