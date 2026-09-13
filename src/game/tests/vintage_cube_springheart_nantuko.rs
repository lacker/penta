//! Springheart Nantuko: a 1/1 for two, or four mana to bestow it onto
//! something worth copying -- and then every land is another one of that.

use super::*;

/// Player One with the Nantuko in hand, a creature out, a land to play, and
/// `mana` green available.
fn staged(mana: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let bears = creature(92_500, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let mut drawn = game
        .build_zone(PlayerId::One, &[cards::SPRINGHEART_NANTUKO, cards::FOREST])
        .expect("cataloged")
        .into_iter();
    let nantuko = drawn.next().expect("the Nantuko first");
    let forest = drawn.next().expect("the Forest second");
    let nantuko_id = nantuko.id;
    game.players[0].hand.push(nantuko);
    game.players[0].hand.push(forest);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.players[0].lands_played_this_turn = 0;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, mana);
    drain_pending(&mut game);
    (game, nantuko_id, bears_id)
}

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if !game.pending_decisions.is_empty() {
            return;
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

/// Casts the Nantuko onto `host` with its bestow cost.
fn bestow_onto(game: &mut Game, nantuko: GameObjectId, host: GameObjectId) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == nantuko
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(host))
            }
            _ => false,
        })
        .expect("four mana bestows it onto the creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(game);
}

/// Plays the land in hand and answers the landfall trigger, paying when
/// `pay` says so.
fn play_a_land(game: &mut Game, pay: bool) {
    let land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { .. }))
        .expect("a land in hand and a land drop left");
    game.apply(PlayerId::One, land).expect("the land is played");
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let wanted = if pay { "Pay the cost" } else { "Decline" };
            let options = decision
                .options
                .iter()
                .find(|option| option.label == wanted)
                .map(|option| vec![option.id])
                .expect("both answers are offered");
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offer accepts either answer");
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

fn tokens(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect()
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// The Nantuko as it stands on the battlefield. A card that resolves takes a
/// new identity, so the one in hand cannot be looked up by its old id.
fn nantuko_on_battlefield(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SPRINGHEART_NANTUKO)
        .expect("the Nantuko resolved")
}

/// Bestowed, it is an Aura that makes its host bigger rather than a creature
/// of its own.
#[test]
fn bestowing_it_makes_the_host_bigger() {
    let (mut game, nantuko, bears) = staged(4);

    bestow_onto(&mut game, nantuko, bears);

    assert_eq!(game.power(permanent(&game, bears)), Some(3), "2/2 plus one");
    assert_eq!(game.toughness(permanent(&game, bears)), Some(3));
    assert_eq!(
        nantuko_on_battlefield(&game).attached_to,
        Some(bears),
        "and the Nantuko is wearing it",
    );
}

/// Attached and paid for, a land copies whatever it is wearing.
#[test]
fn a_land_copies_the_host_when_you_pay() {
    let (mut game, nantuko, bears) = staged(6);
    bestow_onto(&mut game, nantuko, bears);

    play_a_land(&mut game, true);

    let made = tokens(&game);
    assert_eq!(made.len(), 1, "one token");
    assert_eq!(
        made[0].card.definition,
        ObjectKind::Token,
        "and it is a token",
    );
    assert_eq!(game.power(made[0]), Some(2), "a copy of the 2/2 host");
    assert_eq!(game.toughness(made[0]), Some(2));
}

/// Declining is the other half of the same clause, and it still leaves an
/// Insect.
#[test]
fn declining_leaves_an_insect_instead() {
    let (mut game, nantuko, bears) = staged(6);
    bestow_onto(&mut game, nantuko, bears);

    play_a_land(&mut game, false);

    let made = tokens(&game);
    assert_eq!(made.len(), 1, "one token either way");
    assert_eq!(game.power(made[0]), Some(1), "a 1/1 Insect");
    assert!(
        game.effective_subtypes(made[0])
            .contains(crate::card::Subtype::named("Insect")),
        "and it is an Insect",
    );
}

/// Cast as a creature it is attached to nothing, so nobody is asked to pay
/// and the land makes an Insect.
#[test]
fn as_a_creature_it_never_offers_the_payment() {
    let (mut game, nantuko, _) = staged(6);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == nantuko && choices.iter_targets().count() == 0
            }
            _ => false,
        })
        .expect("two mana casts it as a creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);

    play_a_land(&mut game, true);

    let made = tokens(&game);
    assert_eq!(made.len(), 1, "an Insect and nothing else");
    assert_eq!(game.power(made[0]), Some(1));
    assert_eq!(
        nantuko_on_battlefield(&game).attached_to,
        None,
        "it was never an Aura",
    );
}

/// "Unlike other Auras, an Aura with bestow isn't put into its owner's
/// graveyard if it becomes unattached. The effect making it an Aura ends and
/// it remains on the battlefield as an enchantment creature." Killing the
/// host does not kill what it was wearing.
#[test]
fn losing_the_host_leaves_a_creature_behind() {
    let (mut game, nantuko, bears) = staged(4);
    bestow_onto(&mut game, nantuko, bears);

    game.move_permanents_to_graveyard(&[bears]);
    drain_pending(&mut game);
    game.check_state_based_actions();

    let standing = nantuko_on_battlefield(&game);
    assert_eq!(standing.attached_to, None, "it has nothing to wear now");
    assert!(
        game.permanent_types(standing)
            .is_some_and(super::CardTypeSet::is_creature),
        "and being unattached is what makes it a creature again",
    );
    assert_eq!(
        (game.power(standing), game.toughness(standing)),
        (Some(1), Some(1)),
        "a 1/1, the body it was printed with",
    );
    assert!(
        !game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SPRINGHEART_NANTUKO),
        "an ordinary Aura would have gone to the graveyard; this one does not",
    );
}

/// And a creature is what a land finds: with nothing attached the landfall
/// makes an Insect rather than asking for {1}{G}.
#[test]
fn the_survivor_offers_no_payment() {
    let (mut game, nantuko, bears) = staged(6);
    bestow_onto(&mut game, nantuko, bears);
    game.move_permanents_to_graveyard(&[bears]);
    drain_pending(&mut game);
    game.check_state_based_actions();

    play_a_land(&mut game, false);

    let made = tokens(&game);
    assert_eq!(made.len(), 1, "one token");
    assert_eq!(game.power(made[0]), Some(1), "and it is the Insect");
}

/// "Unlike other Aura spells, an Aura spell with bestow isn't countered if
/// its target is illegal as it begins to resolve. It returns to being an
/// enchantment creature spell and enters as an enchantment creature." Killing
/// the host in response does not answer the Nantuko.
#[test]
fn a_dead_target_does_not_counter_the_bestow_spell() {
    let (mut game, nantuko, bears) = staged(4);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == nantuko
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bears))
            }
            _ => false,
        })
        .expect("four mana bestows it onto the Bears");
    game.apply(PlayerId::One, cast).expect("it is cast");

    // In response, with the spell still on the stack.
    game.move_permanents_to_graveyard(&[bears]);
    drain_pending(&mut game);
    settle(&mut game);
    game.check_state_based_actions();

    let standing = nantuko_on_battlefield(&game);
    assert_eq!(standing.attached_to, None, "there was nothing left to wear");
    assert!(
        game.permanent_types(standing)
            .is_some_and(super::CardTypeSet::is_creature),
        "so it resolved as the creature it is printed as",
    );
    assert_eq!(
        (game.power(standing), game.toughness(standing)),
        (Some(1), Some(1)),
    );
}
