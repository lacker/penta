//! Worldspine Wurm: a fifteen-mana body nobody pays for, whose real text is
//! what happens when it dies and what stops that from repeating.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn wurm_card(game: &mut Game) -> CardInstance {
    game.build_zone(PlayerId::One, &[cards::WORLDSPINE_WURM])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card")
}

/// Answers every question either player is asked -- ordering two triggers is
/// the only one this card raises -- and passes until the stack is quiet.
fn settle(game: &mut Game) {
    for _ in 0..32 {
        let deciding = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find(|player| game.observe(*player).decision.is_some());
        if let Some(player) = deciding {
            let decision = game.observe(player).decision.expect("just checked");
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("answering with the offered options is legal");
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

fn tokens(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_trample(crate::card::TokenCharacteristics::creature(
                    &["Wurm"],
                    &[ManaColor::Green],
                    5,
                    5,
                )),
            )
        })
        .count()
}

/// The body, so a trigger test is not also the first check that a 15/15
/// tramples.
#[test]
fn it_is_a_fifteen_fifteen_with_trample() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == wurm)
        .expect("it is on the battlefield");
    assert_eq!(game.power(permanent), Some(15), "fifteen power");
    assert!(
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Trample),
        "and trample",
    );
}

/// Dying does both things at once: three Wurms arrive and the card itself
/// goes home rather than staying in a graveyard to be reanimated again.
#[test]
fn dying_makes_three_wurms_and_shuffles_itself_away() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);

    game.move_permanents_to_graveyard(&[wurm]);
    assert_eq!(
        game.pending_triggers.len(),
        2,
        "one look-back dies trigger and one graveyard-source shuffle trigger",
    );
    settle(&mut game);

    assert_eq!(tokens(&game), 3, "three tokens");
    assert!(
        game.players[0].graveyard.is_empty(),
        "and nothing is left in the graveyard: {:?}",
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        game.players[0].library.len(),
        1,
        "the Wurm is back in its owner's library",
    );
}

/// The dies ability belongs to the departing permanent, while the
/// from-anywhere ability belongs to the new graveyard card. If control was
/// stolen, those are controlled by different players and use different ids.
#[test]
fn a_stolen_wurm_has_one_trigger_from_each_side_of_the_transition() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == wurm)
        .expect("the Wurm is present")
        .controller = PlayerId::Two;

    game.move_permanents_to_graveyard(&[wurm]);

    assert_eq!(
        game.pending_triggers.len(),
        2,
        "exactly the two printed abilities"
    );
    let dies = game
        .pending_triggers
        .iter()
        .find(|trigger| trigger.text.contains("dies"))
        .expect("the battlefield look-back trigger");
    assert_eq!(dies.source.object, wurm, "its source is the old permanent");
    assert_eq!(
        dies.controller,
        PlayerId::Two,
        "its last controller controls it"
    );

    let shuffle = game
        .pending_triggers
        .iter()
        .find(|trigger| trigger.text.contains("from anywhere"))
        .expect("the graveyard arrival trigger");
    assert_ne!(
        shuffle.source.object, wurm,
        "its source is the new graveyard card"
    );
    assert_eq!(
        shuffle.controller,
        PlayerId::One,
        "the card's owner controls it"
    );
    assert_eq!(
        shuffle.context.trigger.object,
        Some(shuffle.source.object),
        "the triggering object is observed after the move",
    );
    assert_eq!(
        shuffle.context.trigger.zone_change_result,
        Some(shuffle.source.object),
        "the event records that same destination identity",
    );
}

/// The tokens are the printed ones: 5/5, green, and trampling.
#[test]
fn the_tokens_are_five_five_tramplers() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);
    game.move_permanents_to_graveyard(&[wurm]);
    settle(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                token_with_trample(crate::card::TokenCharacteristics::creature(
                    &["Wurm"],
                    &[ManaColor::Green],
                    5,
                    5,
                )),
            )
        })
        .expect("a token was made");
    assert_eq!(game.power(token), Some(5), "5/5");
    assert!(
        game.permanent_has_executable_keyword(token, KeywordAbility::Trample),
        "with trample",
    );
    assert_eq!(
        token.controller,
        PlayerId::One,
        "under the Wurm's controller"
    );
}

/// "From anywhere" reaches a discard, and the tokens do not: only a Wurm
/// that died makes them.
#[test]
fn a_discarded_wurm_shuffles_back_and_makes_nothing() {
    let mut game = staged();
    let card = wurm_card(&mut game);
    let wurm = card.id;
    game.players[0].hand.push(card);

    game.discard_cards(PlayerId::One, &[wurm]);
    settle(&mut game);

    assert_eq!(
        game.players[0].library.len(),
        1,
        "the discarded Wurm went to the library",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "rather than staying in the graveyard",
    );
    assert_eq!(tokens(&game), 0, "and dying is what makes Wurms, not going");
}

/// Another card put into the graveyard does not set it off: the trigger
/// names itself.
#[test]
fn somebody_elses_card_hitting_the_graveyard_does_nothing() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);

    game.move_permanents_to_graveyard(&[lions]);
    settle(&mut game);

    assert_eq!(tokens(&game), 0, "no Wurms");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == wurm),
        "and the Wurm is still standing there",
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "with the Lions where it fell",
    );
}

/// Its ruling: "the last ability is a triggered ability, not a replacement
/// ability. Players can respond to this ability." So the Wurm is in the
/// graveyard, and stays there, for as long as the trigger waits on the
/// stack -- which is the window a Soul-Guide Lantern or the like needs.
#[test]
fn the_shuffle_waits_on_the_stack_with_the_wurm_in_the_graveyard() {
    let mut game = staged();
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);

    game.move_permanents_to_graveyard(&[wurm]);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::WORLDSPINE_WURM),
        "a replacement would have moved it already; a trigger leaves it here",
    );
    assert!(
        !game.pending_triggers.is_empty() || !game.stack.is_empty(),
        "and the ability that will move it is waiting to be answered",
    );
    assert!(
        game.players[0].library.is_empty(),
        "nothing has been shuffled anywhere yet",
    );

    settle(&mut game);

    assert_eq!(
        game.players[0].library.len(),
        1,
        "once it resolves the Wurm goes back into the library",
    );
    assert!(game.players[0].graveyard.is_empty());
}

/// The ruling's own example, taken up: "players can respond to this ability,
/// for example, by trying to exile Worldspine Wurm from the graveyard before
/// it's shuffled into a library." A Scavenging Ooze does exactly that, and the
/// shuffle then resolves onto nothing. The three Wurms still arrive -- that
/// trigger belongs to the permanent that died and asks nothing of the card.
#[test]
fn a_wurm_exiled_before_the_shuffle_never_goes_home() {
    let mut game = staged();
    let vacuum = game
        .put_onto_battlefield(PlayerId::Two, cards::SCAVENGING_OOZE)
        .expect("cataloged");
    let wurm = game
        .put_onto_battlefield(PlayerId::One, cards::WORLDSPINE_WURM)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }

    game.move_permanents_to_graveyard(&[wurm]);
    let buried = game.players[PlayerId::One.index()]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::WORLDSPINE_WURM)
        .expect("it is in the graveyard while the trigger waits")
        .id;

    // The triggers are pending rather than on the stack; the active player
    // passing is what puts them there, and only then is there something to
    // respond to.
    game.priority = PlayerId::One;
    for _ in 0..8 {
        if !game.stack.is_empty() {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // The two triggers are ordered rather than chosen between, and
            // that decision wants every option.
            let options = decision.options.iter().map(|option| option.id).collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("ordering them is legal");
            continue;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        !game.stack.is_empty(),
        "both triggers are on the stack now, the shuffle among them",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::WORLDSPINE_WURM),
        "and the Wurm is still sitting in the graveyard",
    );

    // Their Ooze answers it in the window the trigger leaves open.
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Green, 1);
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == vacuum
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Card(buried)))
            }
            _ => false,
        })
        .expect("the Ooze can point at a card in a graveyard");
    game.apply(PlayerId::Two, action).expect("it activates");
    settle(&mut game);

    assert!(
        game.players[PlayerId::One.index()].library.is_empty(),
        "the shuffle resolved onto a card that was no longer there",
    );
    assert!(
        game.players[PlayerId::One.index()].graveyard.is_empty(),
        "and it did not come back to the graveyard either",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::WORLDSPINE_WURM),
        "it is where the Vacuum put it",
    );
    assert_eq!(
        tokens(&game),
        3,
        "and the three Wurms arrived regardless: that trigger asks nothing \
         of the card it came from",
    );
}
