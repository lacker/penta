//! Sorin of House Markov: a lifelinking two-drop that turns into a
//! planeswalker on the turn he drinks enough.

use super::*;

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
                .find(|option| option.label != "Decline")
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
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
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Sorin on the battlefield since last turn, on his front face.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let sorin = game
        .put_onto_battlefield(PlayerId::One, cards::SORIN_OF_HOUSE_MARKOV)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [4, 4];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[0].life = 20;
    (game, sorin)
}

/// Sorin after he has turned over, with `loyalty` counters on him.
fn as_planeswalker(loyalty: u16) -> (Game, GameObjectId) {
    let (mut game, _sorin) = staged();
    game.gain_life(PlayerId::One, 3);
    postcombat_main(&mut game);
    let neonate = permanent_named(&game, "Sorin, Ravenous Neonate")
        .expect("he turned over")
        .card
        .id;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == neonate)
        .expect("he is there")
        .set_counters(CounterKind::Loyalty, loyalty);
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, neonate)
}

fn postcombat_main(game: &mut Game) {
    game.step = Step::PostcombatMain;
    game.begin_step_triggers();
    settle(game);
}

fn permanent_named<'a>(game: &'a Game, name: &str) -> Option<&'a Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| game.effective_permanent_name(permanent).as_deref() == Some(name))
}

fn activate(game: &mut Game, sorin: GameObjectId, index: u8, victim: Option<GameObjectId>) {
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
                *source == sorin
                    && *ability == AbilityId(index)
                    && victim.is_none_or(|victim| {
                        targets.iter().any(|selection| {
                            selection.targets().iter().any(
                                |target| matches!(target, Target::Permanent(id) if *id == victim),
                            )
                        })
                    })
            }
            _ => false,
        })
        .expect("the loyalty ability is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(game);
}

/// Three life in a turn turns him over in the postcombat main phase, and he
/// comes back as a planeswalker with his printed loyalty.
#[test]
fn three_life_turns_him_over() {
    let (mut game, sorin) = staged();
    game.gain_life(PlayerId::One, 3);

    postcombat_main(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != sorin),
        "the creature he was is gone",
    );
    let neonate = permanent_named(&game, "Sorin, Ravenous Neonate").expect("he came back");
    assert_eq!(neonate.counters(CounterKind::Loyalty), 3);
    assert_eq!(neonate.controller, PlayerId::One);
}

/// Two is not three, and the trigger checks again as it resolves.
#[test]
fn two_life_leaves_him_a_creature() {
    let (mut game, sorin) = staged();
    game.gain_life(PlayerId::One, 2);

    postcombat_main(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == sorin),
        "he is still the Noble",
    );
}

/// The tally is what was gained rather than where the life total ended up:
/// gaining three and losing three still turns him over.
#[test]
fn losing_the_life_again_does_not_stop_him() {
    let (mut game, _sorin) = staged();
    game.gain_life(PlayerId::One, 3);
    game.players[0].life -= 5;

    postcombat_main(&mut game);

    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_some(),
        "what the clause reads is the gaining",
    );
}

/// His plus makes a Food, which is the other way he gets to three.
#[test]
fn the_plus_two_makes_food() {
    let (mut game, sorin) = as_planeswalker(3);

    activate(&mut game, sorin, 1, None);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| is_token_with(permanent, tokens::food())),
        "a Food token is there",
    );
}

/// The minus one spends the same tally the front face read.
#[test]
fn the_minus_one_burns_for_the_life_you_gained() {
    let (mut game, sorin) = as_planeswalker(5);
    game.gain_life(PlayerId::One, 4);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(&mut game, sorin, 2, Some(bears));

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != bears),
        "four damage killed the Bear",
    );
}

/// The ultimate takes a creature, makes it a Vampire, and gives it a
/// lifelink counter when you have another white permanent.
#[test]
fn the_ultimate_takes_a_creature_and_marks_it() {
    let (mut game, sorin) = as_planeswalker(6);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(&mut game, sorin, 3, Some(bears));

    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is still on the battlefield");
    assert_eq!(stolen.controller, PlayerId::One, "under your control now");
    let subtypes = game.effective_subtypes(stolen);
    assert!(
        subtypes.contains(crate::card::Subtype::named("Bear")),
        "it is still a Bear"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Vampire")),
        "and a Vampire as well"
    );
    assert_eq!(stolen.counters(CounterKind::Lifelink), 1);
}

/// "Other than that creature or Sorin": a white creature you just took does
/// not count as the white permanent that pays for its own counter, and
/// neither does Sorin, who is white himself.
#[test]
fn the_counter_needs_a_third_white_permanent() {
    let (mut game, sorin) = as_planeswalker(6);
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(&mut game, sorin, 3, Some(angel));

    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel)
        .expect("it is still on the battlefield");
    assert_eq!(stolen.controller, PlayerId::One, "you took it all the same");
    assert_eq!(
        stolen.counters(CounterKind::Lifelink),
        0,
        "the only white permanents are Sorin and the creature itself",
    );
}

/// "Sorin doesn't need to have been on the battlefield when you gained the
/// life." The tally is the turn's, not his: three gained before he lands
/// still turns him over in the postcombat main phase.
#[test]
fn life_gained_before_he_arrived_still_counts() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started = [4, 4];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[0].life = 20;

    // Three life in the upkeep, and Sorin cast afterwards.
    game.gain_life(PlayerId::One, 3);
    let sorin = game
        .put_onto_battlefield(PlayerId::One, cards::SORIN_OF_HOUSE_MARKOV)
        .expect("cataloged");
    drain_pending(&mut game);

    postcombat_main(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != sorin),
        "the creature is gone",
    );
    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_some(),
        "he read a tally kept by the turn rather than by himself",
    );
}

/// Extort is the other half of the front face: casting anything offers the
/// {W/B}, and paying it drains them for one and gains you one -- which is a
/// third of what turns him over.
#[test]
fn extort_drains_them_and_feeds_his_own_tally() {
    let (mut game, _sorin) = staged();
    let bolt = card(96_900, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    let theirs = game.players[PlayerId::Two.index()].life;
    let mine = game.players[PlayerId::One.index()].life;

    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("a Bolt at their face");
    settle(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        mine + 1,
        "the extort payment gains you what it drains",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        theirs - 4,
        "three from the Bolt and one from the extort",
    );

    // One life gained is not three, so the postcombat main leaves him a
    // creature.
    postcombat_main(&mut game);
    assert!(
        permanent_named(&game, "Sorin of House Markov").is_some(),
        "one is not three",
    );

    // Two more, however they arrive, are enough.
    game.gain_life(PlayerId::One, 2);
    postcombat_main(&mut game);
    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_some(),
        "and the extort's own life counted toward the three",
    );
}

/// Lifelink is printed on the front face: four damage across the table is
/// four life, which is more than the three he asks for.
#[test]
fn his_lifelink_turns_him_over_by_itself() {
    let (mut game, sorin) = staged();
    // A 1/4 hits for one, so give him something to hit with: his own body
    // is the source, and the life it gains is what the trigger counts.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sorin)
        .expect("he is there")
        .add_counters(CounterKind::PlusOnePlusOne, 3);
    let mine = game.players[PlayerId::One.index()].life;
    let theirs = game.players[PlayerId::Two.index()].life;

    game.step = Step::DeclareAttackers;
    game.declare_attacker(sorin, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        theirs - 4,
        "a 4/7 connects for four",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        mine + 4,
        "and lifelink gains that much",
    );

    postcombat_main(&mut game);
    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_some(),
        "four is three or more, so he turns over in his own postcombat main",
    );
}

/// "If you haven't gained life so far this turn as your postcombat main
/// phase begins, the ability won't trigger at all. It's not possible to gain
/// life during your postcombat main phase in time for the ability to
/// trigger." The clause is read as the step opens: five life gained a moment
/// later is five life too late, and he stays a Noble for the rest of the
/// turn.
#[test]
fn life_gained_after_the_step_opens_is_too_late() {
    let (mut game, sorin) = staged();

    // The step begins with nothing gained, so nothing triggers.
    postcombat_main(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == sorin),
        "no life, no trigger",
    );

    game.gain_life(PlayerId::One, 5);
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == sorin),
        "and the gaining afterwards has no step left to be read in",
    );

    // Even walking the rest of the turn out finds no second chance: the
    // trigger is the beginning of that step and there is only one of them.
    for _ in 0..40 {
        if game.step == Step::Cleanup {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }
    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_none(),
        "he is a Noble to the end of it",
    );
}

/// "The control effect created by Sorin, Ravenous Neonate's last ability
/// lasts indefinitely. It doesn't wear off during the cleanup step or if
/// Sorin, Ravenous Neonate leaves the battlefield." What he takes is taken:
/// answering him afterwards buys the creature back for nobody.
#[test]
fn what_the_ultimate_takes_outlives_him() {
    let (mut game, sorin) = as_planeswalker(6);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);

    activate(&mut game, sorin, 3, Some(bears));
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == bears)
            .expect("it is on the battlefield")
            .controller,
        PlayerId::One,
        "taken",
    );

    game.move_permanents_to_graveyard(&[sorin]);
    game.check_state_based_actions();
    settle(&mut game);
    assert!(
        permanent_named(&game, "Sorin, Ravenous Neonate").is_none(),
        "and then he is answered",
    );

    // Round the turn, which is where a lesser control effect would end.
    let turn = game.turn;
    for _ in 0..80 {
        if game.turn > turn + 1 {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }

    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("the Bears are still on the battlefield");
    assert_eq!(
        stolen.controller,
        PlayerId::One,
        "the creature he took is still yours, with him in the graveyard",
    );
    assert!(
        game.effective_subtypes(stolen)
            .contains(crate::card::Subtype::named("Vampire")),
        "and still the Vampire he made of it",
    );
}
