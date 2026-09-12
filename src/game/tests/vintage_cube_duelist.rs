//! Duelist of the Mind: a power that counts draws, and a crime that pays
//! once a turn.

use super::*;

/// Answers every pending decision, saying yes to any "may", then resolves
/// whatever is left on the stack.
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
}

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let duelist = game
        .put_onto_battlefield(PlayerId::One, cards::DUELIST_OF_THE_MIND)
        .expect("cataloged");
    drain_pending(&mut game);
    game.cards_drawn_this_turn = [0; 2];
    (game, duelist)
}

fn power_of(game: &Game, id: GameObjectId) -> Option<i16> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .and_then(|permanent| game.power(permanent))
}

/// Nothing drawn is a zero-power flier; every draw is another point.
#[test]
fn the_power_counts_the_cards_you_have_drawn_this_turn() {
    let (mut game, duelist) = staged();

    assert_eq!(power_of(&game, duelist), Some(0), "no draws yet");
    game.draw_cards(PlayerId::One, 3);
    drain_pending(&mut game);
    assert_eq!(power_of(&game, duelist), Some(3));

    // The opponent's draws are theirs, not yours.
    game.draw_cards(PlayerId::Two, 2);
    drain_pending(&mut game);
    assert_eq!(power_of(&game, duelist), Some(3), "still only your own");
}

/// Targeting an opponent's creature is a crime, and it pays out a draw and
/// a discard.
#[test]
fn targeting_an_opponents_creature_is_a_crime() {
    let (mut game, _duelist) = staged();
    let theirs = creature(90_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    let bolt = card(90_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let hand_before = game.players[0].hand.len();

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(theirs_id))
            }
            _ => false,
        })
        .expect("the Bolt can point at their creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        hand_before - 1,
        "the Bolt left, and the draw and discard cancel out",
    );
    assert_eq!(
        game.cards_drawn_this_turn[0], 1,
        "the crime paid out a draw"
    );
}

/// Targeting your own creature is not a crime.
#[test]
fn targeting_your_own_creature_is_not_a_crime() {
    let (mut game, _duelist) = staged();
    let mine = creature(90_010, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let bolt = card(90_011, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(mine_id))
            }
            _ => false,
        })
        .expect("the Bolt can point at your own creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.cards_drawn_this_turn[0], 0,
        "nothing of theirs was pointed at",
    );
}

/// The ability triggers only once each turn, however many crimes follow.
#[test]
fn the_ability_pays_out_only_once_each_turn() {
    let (mut game, _duelist) = staged();
    for (offset, id) in (90_020..90_023).enumerate() {
        let bolt = card(id, cards::LIGHTNING_BOLT, PlayerId::One);
        game.players[0].hand.push(bolt);
        let _ = offset;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);

    for _ in 0..3 {
        let cast =
            game.legal_actions(PlayerId::One)
                .into_iter()
                .find(|action| match action {
                    Action::CastSpell { card, choices, .. } => {
                        game.players[0].hand.iter().any(|held| {
                            held.id == *card && held.definition == cards::LIGHTNING_BOLT
                        }) && choices
                            .targets()
                            .iter()
                            .flat_map(crate::casting::TargetSelection::targets)
                            .any(|target| *target == Target::Player(PlayerId::Two))
                    }
                    _ => false,
                });
        let Some(cast) = cast else { break };
        game.apply(PlayerId::One, cast).expect("it is cast");
        settle(&mut game);
        drain_pending(&mut game);
    }

    assert_eq!(game.cards_drawn_this_turn[0], 1, "three crimes, one payout");
}

/// "At least one card in an opponent's graveyard": the crime need not be a
/// spell and need not touch anything they control on the battlefield.
/// Activating Scavenging Ooze for a card in their graveyard is enough.
#[test]
fn eating_a_card_from_their_graveyard_is_a_crime() {
    let (mut game, _duelist) = staged();
    let ooze = game
        .put_onto_battlefield(PlayerId::One, cards::SCAVENGING_OOZE)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[1].graveyard.clear();
    let theirs = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let theirs_id = theirs.id;
    game.players[1].graveyard.push(theirs);
    drain_pending(&mut game);
    game.cards_drawn_this_turn = [0; 2];
    let hand_before = game.players[0].hand.len();

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == ooze
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Card(theirs_id))
            }
            _ => false,
        })
        .expect("Scavenging Ooze can point at a card in their graveyard");
    game.apply(PlayerId::One, activate).expect("it activates");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.cards_drawn_this_turn[0], 1,
        "an ability aimed into their graveyard is a crime too",
    );
    assert_eq!(
        game.players[0].hand.len(),
        hand_before,
        "the draw and the discard cancel out",
    );
}

/// "The spell or ability that constituted a crime doesn't have to have
/// resolved yet or at all." Countering the Bolt takes the damage away and
/// leaves the crime where it was committed: on casting.
#[test]
fn a_countered_spell_was_still_a_crime() {
    let (mut game, _duelist) = staged();
    let theirs = creature(90_030, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    let bolt = card(90_031, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    let counterspell = card(90_032, cards::COUNTERSPELL, PlayerId::Two);
    let counterspell_id = counterspell.id;
    game.players[1].hand.push(counterspell);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(theirs_id))
            }
            _ => false,
        })
        .expect("the Bolt can point at their creature");
    game.apply(PlayerId::One, cast).expect("it is cast");
    game.apply(PlayerId::One, Action::PassPriority)
        .expect("they get a word in");
    let answer = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == counterspell_id))
        .expect("two blue answers it");
    game.apply(PlayerId::Two, answer).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs_id),
        "the Bears never took the damage",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "because the Bolt was countered",
    );
    assert_eq!(
        game.cards_drawn_this_turn[0], 1,
        "the crime was committed as it was cast, and paid out anyway",
    );
}

/// "At least one opponent": the player is a target like any other. A Bolt
/// pointed at their face is a crime with nothing of theirs on the board at
/// all.
#[test]
fn targeting_the_opponent_themselves_is_a_crime() {
    let (mut game, _duelist) = staged();
    let bolt = card(90_400, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the Bolt can point at them");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.cards_drawn_this_turn[0], 1,
        "naming the player is naming an opponent",
    );
    assert_eq!(game.players[1].life, 17, "and the Bolt still resolved");
}

/// Vigilance is the half the power clause makes worth having: the Duelist
/// attacks and stays up, so a board it swung into is a board it can still
/// block.
#[test]
fn it_attacks_without_tapping() {
    let (mut game, duelist) = staged();
    game.draw_cards(PlayerId::One, 2);
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;

    game.declare_attacker(duelist, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == duelist)
        .expect("she is attacking");
    assert!(!permanent.tapped, "vigilance leaves her up");
    assert_eq!(
        game.power(permanent),
        Some(2),
        "swinging for what she has drawn",
    );
}

/// "The ability that defines her power works in all zones, not just the
/// battlefield." No card in this catalog reads a card's power outside the
/// battlefield, so the clause is asked through the predicate such a card
/// would use: a Duelist lying in the graveyard is two power once two cards
/// have been drawn, and no power at all before that.
#[test]
fn her_power_is_defined_outside_the_battlefield_too() {
    let (mut game, duelist) = staged();
    let buried = game
        .build_zone(PlayerId::One, &[cards::DUELIST_OF_THE_MIND])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].graveyard.push(buried);
    let reads_two = |game: &Game| {
        let card = game.players[0]
            .graveyard
            .last()
            .expect("she is lying there");
        game.card_object_matches(
            ObjectPredicateDef::PowerAtLeast(2),
            card,
            ZoneKind::Graveyard,
            duelist,
        )
    };

    assert!(!reads_two(&game), "nothing drawn is no power at all");

    game.draw_cards(PlayerId::One, 2);
    drain_pending(&mut game);

    assert!(
        reads_two(&game),
        "and two cards drawn is two power, graveyard or not",
    );
}
