//! Goldspan Dragon: he pays for himself, whether he is attacking or being
//! answered, and the Treasures he leaves are worth twice what they say.

use super::*;

/// The Dragon on the battlefield, ready to attack.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let dragon = game
        .put_onto_battlefield(PlayerId::One, cards::GOLDSPAN_DRAGON)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, dragon)
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

fn treasures(game: &Game) -> Vec<GameObjectId> {
    game.battlefield
        .iter()
        .filter(|permanent| game.effective_subtypes(permanent).contains(&"Treasure"))
        .map(|permanent| permanent.card.id)
        .collect()
}

/// He is hasty, so the attack that pays for him happens the turn he lands.
#[test]
fn he_flies_and_attacks_the_turn_he_lands() {
    let (game, dragon) = staged();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == dragon)
        .expect("he is there");

    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste));
}

/// Attacking makes a Treasure.
#[test]
fn attacking_makes_a_treasure() {
    let (mut game, dragon) = staged();
    game.step = Step::DeclareAttackers;
    game.declare_attacker(dragon, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);

    assert_eq!(treasures(&game).len(), 1);
}

/// So does being targeted, whatever the spell does about it.
#[test]
fn being_targeted_makes_a_treasure() {
    let (mut game, dragon) = staged();
    let bolt = card(180_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(dragon))
            }
            _ => false,
        })
        .expect("the Dragon is a legal target");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        treasures(&game).len(),
        1,
        "the trigger is the targeting, not what the spell did",
    );
}

/// A Treasure of his is worth two mana of one colour rather than one of any.
#[test]
fn his_treasures_are_worth_two() {
    let (mut game, dragon) = staged();
    game.step = Step::DeclareAttackers;
    game.declare_attacker(dragon, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    let treasure = treasures(&game)[0];

    let doubled = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            // The granted ability, not the Treasure's own: both make blue,
            // and only one of them makes two of it.
            Action::ActivateManaAbility {
                source,
                ability:
                    AbilityOrigin::Granted {
                        source: granter, ..
                    },
                color,
                ..
            } => *source == treasure && *granter == dragon && *color == ManaColor::Blue,
            _ => false,
        })
        .expect("his grant reaches the Treasure he just made");
    game.apply(PlayerId::One, doubled).expect("it activates");

    assert_eq!(
        game.players[0].mana.len(),
        2,
        "two mana of the one colour chosen",
    );
    assert!(treasures(&game).is_empty(), "and the Treasure is spent");
}

/// The grant reaches Treasures however they arrived, and stops when he does.
#[test]
fn the_grant_covers_every_treasure_and_ends_with_him() {
    let (mut game, dragon) = staged();
    game.create_token(PlayerId::One, tokens::treasure());
    drain_pending(&mut game);

    let outside = treasures(&game)[0];
    assert!(
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == outside)
        }),
        "a Treasure he did not make still has the ability",
    );

    game.battlefield
        .retain(|permanent| permanent.card.id != dragon);
    game.check_state_based_actions();

    let activations = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == outside)
        })
        .count();
    assert_eq!(
        activations, 5,
        "with him gone only the Treasure's own five colours remain",
    );
}

/// "An ability that triggers when a creature becomes the target of a spell
/// resolves before the spell that caused it to trigger. Such an ability
/// resolves even if that spell is countered." The Treasure is on the
/// battlefield while their Bolt is still on the stack, and it stays there
/// when the Bolt never resolves.
#[test]
fn the_treasure_arrives_before_the_spell_and_outlives_a_counter() {
    let (mut game, dragon) = staged();
    let bolt = card(180_500, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    let counter = card(180_501, cards::COUNTERSPELL, PlayerId::One);
    let counter_id = counter.id;
    game.players[PlayerId::One.index()].hand.push(counter);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    game.priority = PlayerId::Two;

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(dragon))
            }
            _ => false,
        })
        .expect("the Dragon is a legal target");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    // The Treasure trigger goes on the stack above the Bolt as it is cast,
    // so the Bolt is the object underneath it.
    let bolt_spell = game
        .stack
        .iter()
        .next()
        .expect("the Bolt is on the stack")
        .id;

    // The Treasure trigger is above the Bolt, so it resolves first.
    for _ in 0..8 {
        if !treasures(&game).is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    assert_eq!(
        treasures(&game).len(),
        1,
        "the Treasure is here while their Bolt is still waiting",
    );
    assert!(
        game.stack.iter().any(|spell| spell.id == bolt_spell),
        "and the Bolt has not resolved",
    );

    game.priority = PlayerId::One;
    let answer = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == counter_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(bolt_spell))
            }
            _ => false,
        })
        .expect("a Counterspell answers it");
    game.apply(PlayerId::One, answer).expect("it is cast");
    settle(&mut game);

    assert_eq!(
        treasures(&game).len(),
        1,
        "countering the spell takes nothing back",
    );
    let dragon = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == dragon)
        .expect("he is untouched");
    assert_eq!(dragon.damage, 0, "and the Dragon took no damage at all");
}

/// "Becomes the target of a spell" does not say whose. Pointing your own
/// Giant Growth at him is a Treasure, which is a thing you do on purpose.
#[test]
fn your_own_spell_targeting_him_pays_out_too() {
    let (mut game, dragon) = staged();
    let growth = card(180_400, cards::GIANT_GROWTH, PlayerId::One);
    let growth_id = growth.id;
    game.players[0].hand.push(growth);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == growth_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(dragon))
            }
            _ => false,
        })
        .expect("he is a legal target for your own spell");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);

    assert_eq!(treasures(&game).len(), 1, "a spell of yours is a spell");
}

/// The object predicate says "spell", so an ability choosing the Dragon is
/// deliberately outside this trigger even though it uses the same target event.
#[test]
fn an_ability_targeting_him_does_not_make_a_treasure() {
    let (mut game, dragon) = staged();
    let sorcerer = game
        .put_onto_battlefield(PlayerId::Two, cards::PRODIGAL_SORCERER)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sorcerer)
        .expect("it is on the battlefield")
        .entered_controller_turn = 0;
    drain_pending(&mut game);
    game.priority = PlayerId::Two;

    let activation = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == sorcerer
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(dragon)))
            }
            _ => false,
        })
        .expect("the Sorcerer can target the Dragon");
    game.apply(PlayerId::Two, activation)
        .expect("the ability activates");
    settle(&mut game);

    assert!(treasures(&game).is_empty(), "an ability is not a spell");
}

/// "Players can cast spells and activate abilities after the triggered
/// ability resolves but before the spell that caused it to trigger does."
/// The Treasure is the point of that window: it is on the battlefield, and
/// worth its doubled two, while their Bolt is still waiting to resolve.
#[test]
fn the_treasure_may_be_spent_while_the_spell_still_waits() {
    let (mut game, dragon) = staged();
    let bolt = card(180_500, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(dragon))
            }
            _ => false,
        })
        .expect("the Dragon is a legal target");
    game.apply(PlayerId::Two, cast).expect("it is cast");

    // Only the trigger resolves; the Bolt is left where it is.
    for _ in 0..8 {
        if game.pending_triggers.is_empty() && game.stack.len() == 1 {
            break;
        }
        if game.apply(game.priority, Action::PassPriority).is_err() {
            break;
        }
    }
    assert_eq!(game.stack.len(), 1, "their Bolt is still on the stack");
    let treasure = *treasures(&game).first().expect("the Treasure arrived");

    game.priority = PlayerId::One;
    let doubled = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateManaAbility {
                source,
                ability:
                    AbilityOrigin::Granted {
                        source: granter, ..
                    },
                ..
            } => *source == treasure && *granter == dragon,
            _ => false,
        })
        .expect("his Treasure is spendable where it stands");
    game.apply(PlayerId::One, doubled)
        .expect("it is sacrificed");

    // A mana ability uses no stack, so nothing has been allowed to resolve
    // between announcing it and reading the pool.
    assert_eq!(
        game.players[0].mana.len(),
        2,
        "two mana, paid for by the spell that was aimed at him",
    );
    assert_eq!(game.stack.len(), 1, "and their Bolt has still not resolved");
}

/// "*Treasures you control* have..." -- a Treasure across the table is
/// theirs, and his grant never reaches it.
#[test]
fn their_treasures_are_not_granted_the_second_mana() {
    let (mut game, dragon) = staged();
    game.create_token(PlayerId::Two, tokens::treasure());
    drain_pending(&mut game);
    // Their seat needs priority, or its offered actions are empty and every
    // question asked of them answers itself.
    game.priority = PlayerId::Two;
    let theirs = *treasures(&game)
        .first()
        .expect("their Treasure is on the battlefield");
    assert!(
        !game.legal_actions(PlayerId::Two).is_empty(),
        "the seat is live, so what it is not offered means something",
    );

    let granted = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .any(|action| match action {
            Action::ActivateManaAbility {
                source,
                ability:
                    AbilityOrigin::Granted {
                        source: granter, ..
                    },
                ..
            } => source == theirs && granter == dragon,
            _ => false,
        });
    assert!(!granted, "his grant reaches only the Treasures he controls");

    let own = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == theirs
            )
        })
        .expect("they may still spend their own Treasure");
    game.apply(PlayerId::Two, own).expect("it is sacrificed");
    drain_pending(&mut game);

    assert_eq!(
        game.players[1].mana.len(),
        1,
        "one mana, which is what a Treasure is printed with",
    );
}
