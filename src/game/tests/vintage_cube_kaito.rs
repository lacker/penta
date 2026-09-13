//! Kaito, Bane of Nightmares: a planeswalker that is a hexproof body on his
//! own turn and back to a planeswalker on theirs.

use super::*;

/// Kaito on the battlefield under Player One.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for definition in [cards::SERRA_ANGEL, cards::MOUNTAIN, cards::FOREST] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let kaito = game
        .put_onto_battlefield(PlayerId::One, cards::KAITO_BANE_OF_NIGHTMARES)
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
    (game, kaito)
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

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn loyalty_action(game: &Game, kaito: GameObjectId, ability: u8) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability: id, .. },
                ..
            } => *source == kaito && *id == AbilityId(ability),
            _ => false,
        })
}

/// On your turn he is a hexproof 3/4 Ninja; on theirs he is a planeswalker
/// again.
#[test]
fn he_is_a_creature_only_on_your_turn() {
    let (mut game, kaito) = staged();

    let him = permanent(&game, kaito);
    assert_eq!(game.power(him), Some(3));
    assert_eq!(game.toughness(him), Some(4));
    assert!(
        game.effective_subtypes(him)
            .contains(crate::card::Subtype::named("Ninja"))
    );
    assert!(game.permanent_has_executable_keyword(him, KeywordAbility::Hexproof));

    game.active_player = PlayerId::Two;

    let him = permanent(&game, kaito);
    assert_eq!(game.power(him), None, "not a creature on their turn");
    assert!(!game.permanent_has_executable_keyword(him, KeywordAbility::Hexproof));
}

/// And only while he has loyalty: an ultimate that empties him takes the
/// body with it.
#[test]
fn without_loyalty_he_is_not_a_creature() {
    let (mut game, kaito) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == kaito)
    {
        permanent.set_counters(CounterKind::Loyalty, 0);
    }

    assert_eq!(game.power(permanent(&game, kaito)), None);
}

/// The plus makes an emblem that pumps the Ninjas, himself included.
#[test]
fn the_emblem_pumps_ninjas() {
    let (mut game, kaito) = staged();

    let plus = loyalty_action(&game, kaito, 2).expect("the plus is activatable");
    game.apply(PlayerId::One, plus).expect("it activates");
    settle(&mut game);

    let him = permanent(&game, kaito);
    assert_eq!(game.power(him), Some(4), "he is a Ninja too");
    assert_eq!(game.toughness(him), Some(5));
}

/// The zero surveils two and draws only when an opponent has lost life.
#[test]
fn the_zero_draws_when_they_have_bled() {
    let (mut game, kaito) = staged();

    let zero = loyalty_action(&game, kaito, 3).expect("the zero is activatable");
    game.apply(PlayerId::One, zero).expect("it activates");
    settle(&mut game);
    assert!(
        game.players[0].hand.is_empty(),
        "nobody has lost life, so nothing was drawn",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == kaito)
        .expect("he is there")
        .activated_loyalty_this_turn = false;
    game.deal_damage(PlayerId::Two, 1);

    let zero = loyalty_action(&game, kaito, 3).expect("the zero is activatable again");
    game.apply(PlayerId::One, zero).expect("it activates");
    settle(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        1,
        "one opponent bled, so one card",
    );
}

/// The minus taps a creature and leaves two stun counters on it, so it stays
/// tapped through two untap steps.
#[test]
fn the_minus_stuns_a_creature() {
    let (mut game, kaito) = staged();
    let bears = creature(300_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let minus = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability: id, .. },
                targets,
                ..
            } => {
                *source == kaito
                    && *id == AbilityId(4)
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bears_id))
            }
            _ => false,
        })
        .expect("their creature is a legal target");
    game.apply(PlayerId::One, minus).expect("it activates");
    settle(&mut game);

    let stunned = permanent(&game, bears_id);
    assert!(stunned.tapped);
    assert_eq!(stunned.counters(CounterKind::Stun), 2);

    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert!(permanent(&game, bears_id).tapped, "one counter came off");
    assert_eq!(permanent(&game, bears_id).counters(CounterKind::Stun), 1);
}

/// Ninjutsu is how he is meant to arrive: an unblocked bear goes home and
/// Kaito takes its place, tapped and attacking the player it was attacking.
/// He arrives with his loyalty, so on your own turn he is at once the 3/4
/// Ninja his static clause promises -- and he connects for it.
#[test]
fn ninjutsu_brings_him_in_attacking_and_he_connects() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    let bears = creature(109_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let kaito = card(109_001, cards::KAITO_BANE_OF_NIGHTMARES, PlayerId::One);
    let kaito_id = kaito.id;
    game.players[PlayerId::One.index()].hand.push(kaito);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    // Blockers are in and none of them are on the bear, which is the window
    // ninjutsu opens in.
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::One;
    for permanent in &mut game.battlefield {
        if permanent.card.id == bears_id {
            permanent.attacking = true;
            permanent.tapped = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }
    let life = game.players[PlayerId::Two.index()].life;

    let ninjutsu = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == kaito_id))
        .expect("an unblocked attacker and three mana is the whole cost");
    game.apply(PlayerId::One, ninjutsu)
        .expect("the ability activates");
    settle(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "the bear he replaced is back in hand",
    );
    let arrived = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::KAITO_BANE_OF_NIGHTMARES)
        .expect("Kaito arrived");
    let kaito_on_board = arrived.card.id;
    assert!(arrived.tapped, "tapped, as ninjutsu puts him");
    assert!(arrived.attacking, "and attacking");
    assert_eq!(
        arrived.attack_defender,
        Some(AttackDefender::Player(PlayerId::Two)),
        "the player the bear was attacking",
    );
    assert_eq!(
        arrived.counters(CounterKind::Loyalty),
        4,
        "with his printed loyalty",
    );
    assert_eq!(
        (game.power(arrived), game.toughness(arrived)),
        (Some(3), Some(4)),
        "which on your own turn makes him the 3/4 his static clause promises",
    );
    assert!(
        game.permanent_has_executable_keyword(arrived, KeywordAbility::Hexproof),
        "and hexproof with it",
    );

    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    game.finish_declaring_blockers();
    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        life - 3,
        "a tapped attacker still deals its damage",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == kaito_on_board),
        "and he is still standing afterwards",
    );
}
