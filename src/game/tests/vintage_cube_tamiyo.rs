//! Tamiyo, Collector of Tales: two prohibitions in one sentence, and a dig
//! that sorts by a name chosen before the cards are seen.

use super::*;

/// Tamiyo, with the loyalty she would have entered carrying. A permanent
/// built by hand has none, and a planeswalker at zero is binned by
/// state-based actions before anything can read its static ability.
fn tamiyo(id: u32) -> Permanent {
    let mut permanent = creature(id, cards::TAMIYO_COLLECTOR_OF_TALES, PlayerId::One);
    permanent.add_counters(CounterKind::Loyalty, 5);
    permanent
}

/// Resolves whatever is on the stack, answering nothing.
fn resolve(game: &mut Game) {
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// An opponent's discard spell takes nothing; the same spell cast by
/// Tamiyo's own controller still discards.
#[test]
fn tamiyo_stops_an_opponents_discard_but_not_your_own() {
    for caster in [PlayerId::Two, PlayerId::One] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.battlefield.push(tamiyo(98_000));
        game.players[0].hand.clear();
        for id in 98_001..98_005 {
            game.players[0]
                .hand
                .push(card(id, cards::GRIZZLY_BEARS, PlayerId::One));
        }
        let hymn = card(98_010, cards::HYMN_TO_TOURACH, caster);
        let hymn_id = hymn.id;
        game.players[caster.index()].hand.push(hymn);
        game.add_unrestricted_mana(caster, ManaColor::Black, 2);
        game.priority = caster;
        game.active_player = caster;
        let held = game.players[0].hand.len();

        let cast = game
            .legal_actions(caster)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == hymn_id
                        && choices
                            .iter_targets()
                            .any(|target| *target == Target::Player(PlayerId::One))
                }
                _ => false,
            })
            .expect("the Hymn can point at Tamiyo's controller");
        game.apply(caster, cast).expect("it is cast");
        resolve(&mut game);
        drain_pending(&mut game);

        let taken = held - game.players[0].hand.len();
        if caster == PlayerId::One {
            // The Hymn itself left their hand too, so two discards plus the
            // spell is three cards gone.
            assert_eq!(taken, 3, "your own spell still discards");
        } else {
            assert_eq!(taken, 0, "an opponent's spell takes nothing");
        }
    }
}

/// An opponent cannot make Tamiyo's controller sacrifice either.
#[test]
fn tamiyo_stops_an_opponents_sacrifice() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.push(tamiyo(98_010));
    game.battlefield
        .push(creature(98_011, cards::GRIZZLY_BEARS, PlayerId::One));

    assert!(
        !game.can_be_forced_to_sacrifice(PlayerId::One, PlayerId::Two),
        "an opponent cannot",
    );
    assert!(
        game.can_be_forced_to_sacrifice(PlayerId::One, PlayerId::One),
        "and their own spell still can",
    );
}

/// The +1 sorts the top four by a name chosen before they are seen: matches
/// to hand, everything else to the graveyard.
#[test]
fn the_plus_one_sorts_the_top_four_by_the_chosen_name() {
    let mut game = ready_game();
    game.battlefield.clear();
    let planeswalker = tamiyo(98_020);
    let tamiyo_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    game.players[0].hand.clear();
    game.players[0].library.clear();
    // Top four, pushed last-is-top: two Bolts and two Bears.
    for id in 98_021..98_023 {
        game.players[0]
            .library
            .push(card(id, cards::GRIZZLY_BEARS, PlayerId::One));
    }
    for id in 98_023..98_025 {
        game.players[0]
            .library
            .push(card(id, cards::LIGHTNING_BOLT, PlayerId::One));
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == tamiyo_id)
                && matches!(action, Action::ActivateAbility { ability, .. }
                    if game
                        .ability_for_origin(tamiyo_id, *ability)
                        .is_some_and(|ability| ability.text.starts_with("+1")))
        })
        .expect("the +1 is available");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);

    let naming = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the name is chosen first");
    let bolt = naming
        .options
        .iter()
        // Adventure cards can also contain this name in a combined catalog label.
        .find(|option| option.label == "Lightning Bolt")
        .expect("Lightning Bolt is a nonland name that can be chosen");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: naming.id,
            options: vec![bolt.id],
        },
    )
    .expect("naming a card is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .filter(|card| card.definition == cards::LIGHTNING_BOLT)
            .count(),
        2,
        "both cards with the chosen name reach the hand",
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::GRIZZLY_BEARS)
            .count(),
        2,
        "and the rest go to the graveyard, not back on the library",
    );
    assert!(
        game.players[0].library.is_empty(),
        "all four left the library",
    );
}

/// The minus three buys back any card in your graveyard, not only a
/// creature or a spell -- the target says "target card".
#[test]
fn the_minus_three_returns_any_card_from_your_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    let planeswalker = tamiyo(98_040);
    let tamiyo_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(98_041, cards::MOUNTAIN, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(98_042, cards::BLACK_LOTUS, PlayerId::One));
    let lotus = game.players[0].graveyard[1].id;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                ability,
                targets,
                ..
            } => {
                *source == tamiyo_id
                    && game
                        .ability_for_origin(tamiyo_id, *ability)
                        .is_some_and(|ability| ability.text.starts_with('\u{2212}'))
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Card(lotus))
            }
            _ => false,
        })
        .expect("the minus three can name the Lotus");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);

    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::BLACK_LOTUS],
        "an artifact is a card like any other",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == tamiyo_id)
            .expect("she is still there")
            .counters(CounterKind::Loyalty),
        2,
        "and it cost her three loyalty",
    );
}

/// Her ruling: at three loyalty the minus three is activatable, and she is
/// not among its targets -- she is on the battlefield while they are chosen,
/// not in the graveyard.
#[test]
fn she_cannot_buy_herself_back_with_her_own_minus_three() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut planeswalker = tamiyo(98_050);
    planeswalker.set_counters(CounterKind::Loyalty, 3);
    let tamiyo_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(98_051, cards::MOUNTAIN, PlayerId::One));

    let minus_three = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| match action {
                Action::ActivateAbility {
                    source, ability, ..
                } => {
                    *source == tamiyo_id
                        && game
                            .ability_for_origin(tamiyo_id, *ability)
                            .is_some_and(|ability| ability.text.starts_with('\u{2212}'))
                }
                _ => false,
            })
            .collect::<Vec<_>>()
    };
    let offers = minus_three(&game);
    assert_eq!(
        offers.len(),
        1,
        "three loyalty pays for it, and the Mountain is the only card to name",
    );
    assert!(
        !offers.iter().any(|action| matches!(action,
            Action::ActivateAbility { targets, .. }
                if targets
                    .iter()
                    .flat_map(crate::casting::TargetSelection::targets)
                    .any(|target| *target == Target::Card(tamiyo_id)
                        || *target == Target::Permanent(tamiyo_id)))),
        "and she is not one of them: she is on the battlefield, not in the graveyard",
    );
}

/// "Her ability affects sacrifices, but not any other ways permanents can
/// leave the battlefield. It won't stop a creature from dying due to lethal
/// damage... and it won't stop a permanent from being put into its owner's
/// graveyard due to the legend rule."
#[test]
fn she_stops_sacrifices_and_nothing_else() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.push(tamiyo(98_060));
    let bears = creature(98_061, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    // Their removal is damage, not a sacrifice.
    let bolt = card(98_062, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.players[1].mana_pool.red = 1;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(bolt_id, vec![Target::Permanent(bears_id)], Vec::new(), 0),
    )
    .expect("they may point a Bolt at it");
    resolve(&mut game);
    game.check_state_based_actions();
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "three damage kills a 2/2 with her standing there",
    );

    // And a second copy of her is the legend rule, not a sacrifice.
    game.battlefield.push(tamiyo(98_063));
    game.check_state_based_actions();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition
                == crate::game::ObjectKind::Card(cards::TAMIYO_COLLECTOR_OF_TALES))
            .count(),
        1,
        "the legend rule takes one of the two",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::TAMIYO_COLLECTOR_OF_TALES),
        "and it goes to the graveyard: a game rule is not a spell",
    );
}

/// "If a spell or ability your opponent controls reduces your maximum hand
/// size, her first ability won't stop you from discarding cards when the
/// game rules cause you to discard during your cleanup step." The ordinary
/// seven-card cleanup is that same game rule.
#[test]
fn the_cleanup_discard_is_a_game_rule_she_does_not_touch() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.push(tamiyo(98_070));
    game.players[0].hand.clear();
    for index in 0..9 {
        game.players[0]
            .hand
            .push(card(98_071 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.active_player = PlayerId::One;
    game.step = Step::Cleanup;

    game.cleanup();
    assert!(
        game.cleanup_pending,
        "the game rules are asking for a discard"
    );
    let discard = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DiscardCards { .. }))
        .expect("cleanup offers the required discard");
    game.apply(PlayerId::One, discard).expect("it is legal");

    assert_eq!(
        game.players[0].hand.len(),
        7,
        "the game rules trimmed the hand to seven all the same",
    );
}

/// "If that spell or ability gives you the option to sacrifice a permanent
/// or to discard a card, you can't take that option." A Desecration Demon
/// asks each combat and pays for the answer; with Tamiyo out there is no
/// answer to give, and without her the same board feeds it.
#[test]
fn an_optional_sacrifice_is_not_an_option_either() {
    for with_tamiyo in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        if with_tamiyo {
            game.battlefield.push(tamiyo(98_100));
        }
        let bears = creature(98_101, cards::GRIZZLY_BEARS, PlayerId::One);
        let bears_id = bears.card.id;
        game.battlefield.push(bears);
        let demon = creature(98_102, cards::DESECRATION_DEMON, PlayerId::Two);
        let demon_id = demon.card.id;
        game.battlefield.push(demon);
        game.turns_started = [5, 5];
        game.active_player = PlayerId::Two;
        game.priority = PlayerId::Two;
        game.step = Step::BeginningOfCombat;
        game.begin_step_triggers();

        // Feed the Demon whenever it asks, which is the whole question:
        // with Tamiyo out it never asks.
        let mut fed = false;
        for _ in 0..16 {
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .filter(|option| option.card.is_some_and(|(object, _)| object == bears_id))
                    .map(|option| option.id)
                    .take(decision.maximum)
                    .collect::<Vec<_>>();
                fed |= !options.is_empty();
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
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
        game.check_state_based_actions();

        let demon = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == demon_id)
            .expect("the Demon is still there");
        if with_tamiyo {
            assert!(!fed, "the option to sacrifice was never on the table");
            assert!(!demon.tapped, "so the Demon was not paid off");
            assert_eq!(demon.counters(CounterKind::PlusOnePlusOne), 0);
            assert!(
                game.battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == bears_id),
                "and the creature it wanted is still there",
            );
        } else {
            assert!(fed, "without her the option is a real one");
            assert!(demon.tapped, "and taking it keeps the Demon home");
            assert_eq!(demon.counters(CounterKind::PlusOnePlusOne), 1);
        }
    }
}

/// "Return target card from *your* graveyard": the other graveyard is no
/// part of it, however much is lying in it.
#[test]
fn the_minus_three_reaches_only_your_own_graveyard() {
    let mut game = ready_game();
    game.battlefield.clear();
    let planeswalker = tamiyo(98_100);
    let tamiyo_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(98_101, cards::BLACK_LOTUS, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(98_102, cards::SERRA_ANGEL, PlayerId::Two));
    let mine = game.players[0].graveyard[0].id;
    let theirs = game.players[1].graveyard[0].id;

    let named: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source,
                ability,
                targets,
                ..
            } if source == tamiyo_id
                && game
                    .ability_for_origin(tamiyo_id, ability)
                    .is_some_and(|ability| ability.text.starts_with('\u{2212}')) =>
            {
                Some(targets)
            }
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .collect();

    assert!(
        named.contains(&Target::Card(mine)),
        "your own Lotus is a target",
    );
    assert!(
        !named.contains(&Target::Card(theirs)),
        "and their Angel is not",
    );
}

/// "Choose a *nonland* card name": the +1 sorts for spells, so no land name
/// is offered -- a basic, a Tolarian Academy and a Celestial Colonnade
/// alike. Metadata-only records carry no type line for the filter to read
/// and so are not excluded, which is a property of those records rather
/// than of this clause.
#[test]
fn the_plus_one_will_not_name_a_land() {
    let mut game = ready_game();
    game.battlefield.clear();
    let planeswalker = tamiyo(98_200);
    let tamiyo_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for id in 98_201..98_205 {
        game.players[0]
            .library
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == tamiyo_id
                    && game
                        .ability_for_origin(tamiyo_id, *ability)
                        .is_some_and(|ability| ability.text.starts_with("+1")))
        })
        .expect("the +1 is available");
    game.apply(PlayerId::One, action).expect("it is activated");
    resolve(&mut game);

    let naming = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the name is chosen first");
    let named = |name: &str| naming.options.iter().any(|option| option.label == name);

    assert!(!named("Mountain"), "a basic land is no nonland card name");
    assert!(!named("Tolarian Academy"), "and neither is a nonbasic one");
    assert!(
        !named("Celestial Colonnade"),
        "not even one that turns into a creature",
    );
    assert!(
        named("Lightning Bolt"),
        "while the nonland names are all still there",
    );
}
