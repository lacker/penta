//! Spells and permanents cataloged for the Vintage Cube pool.

use super::*;

/// Resolves whatever is on the stack, answering nothing.
fn resolve(game: &mut Game) {
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// The one activation offered for `source`, if there is one.
fn activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(
        |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
    )
}

#[test]
fn the_bombardment_throws_a_creature_for_one_damage() {
    let mut game = ready_game();
    let bombardment = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_BOMBARDMENT)
        .expect("cataloged");
    game.battlefield
        .push(creature(50_100, cards::GRIZZLY_BEARS, PlayerId::One));
    let before = game.players[PlayerId::Two.index()].life;

    assert!(
        activation(&game, bombardment).is_some(),
        "the sacrifice ability is offered once there is a creature",
    );
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == bombardment
                && targets.iter().any(|selection| {
                    selection.targets().contains(&Target::Player(PlayerId::Two))
                }))
        })
        .expect("the opponent is one of the offered targets");
    game.apply(PlayerId::One, action).expect("it is activated");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GRIZZLY_BEARS),
        "the creature is sacrificed as a cost",
    );
    resolve(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].life, before - 1);
}

#[test]
fn the_legendary_lands_count_what_they_name() {
    for (definition, color, counted, uncounted) in [
        (
            cards::GAEAS_CRADLE,
            ManaColor::Green,
            cards::GRIZZLY_BEARS,
            cards::BLACK_LOTUS,
        ),
        (
            cards::TOLARIAN_ACADEMY,
            ManaColor::Blue,
            cards::BLACK_LOTUS,
            cards::GRIZZLY_BEARS,
        ),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        let land = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        for instance in 0..3 {
            game.battlefield
                .push(creature(50_200 + instance, counted, PlayerId::One));
        }
        game.battlefield
            .push(creature(50_300, uncounted, PlayerId::One));

        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: land,
                ability: mana_ability_for(&game, land, color),
                color,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .expect("the land taps for mana");
        assert_eq!(
            game.players[PlayerId::One.index()].mana_pool.amount(color),
            3,
            "{definition:?} counts only what it names",
        );
    }
}

#[test]
fn time_warp_hands_the_extra_turn_to_the_player_it_targets() {
    let mut game = ready_game();
    let warp = card(50_700, cards::TIME_WARP, PlayerId::One);
    let warp_id = warp.id;
    game.players[PlayerId::One.index()].hand.push(warp);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == warp_id
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Player(PlayerId::Two))
                }))
        })
        .expect("Time Warp can hand the turn to the other player");
    game.apply(PlayerId::One, action).expect("it is cast");
    resolve(&mut game);

    assert_eq!(
        game.extra_turns,
        vec![PlayerId::Two],
        "the extra turn belongs to the player it targeted, not its caster",
    );
}

#[test]
fn the_soft_counters_ask_for_their_own_amount() {
    for (definition, color, tax) in [
        (cards::MANA_TITHE, ManaColor::White, 1),
        (cards::SPELL_PIERCE, ManaColor::Blue, 2),
        (cards::MISCALCULATION, ManaColor::Blue, 2),
    ] {
        let mut game = ready_game();
        let bolt = card(50_800, cards::LIGHTNING_BOLT, PlayerId::Two);
        let bolt_id = bolt.id;
        game.players[PlayerId::Two.index()].hand.push(bolt);
        game.players[PlayerId::Two.index()].mana_pool.red = 1;
        // Enough left over to pay the tax, so the choice is a real one.
        game.players[PlayerId::Two.index()].mana_pool.colorless = tax;
        game.priority = PlayerId::Two;
        game.apply(
            PlayerId::Two,
            cast_action(bolt_id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
        )
        .expect("the opponent casts something to counter");
        let on_stack = game.stack.last().expect("the spell is on the stack").id;
        game.apply(PlayerId::Two, Action::PassPriority).unwrap();

        let counter = card(50_801, definition, PlayerId::One);
        let counter_id = counter.id;
        game.players[PlayerId::One.index()].hand.push(counter);
        let pool = &mut game.players[PlayerId::One.index()].mana_pool;
        pool.add_color(color, 1);
        pool.colorless += 1;
        game.apply(
            PlayerId::One,
            cast_action(counter_id, vec![Target::Spell(on_stack)], Vec::new(), 0),
        )
        .unwrap_or_else(|error| panic!("{definition:?} answers a spell: {error}"));
        pass_priority_pair(&mut game);

        // Declining the tax is what counters the spell.
        let decision = game
            .observe(PlayerId::Two)
            .decision
            .unwrap_or_else(|| panic!("{definition:?} asks its controller to pay {tax}"));
        let decline = decision
            .options
            .iter()
            .find(|option| option.label != "Pay the cost")
            .unwrap_or_else(|| panic!("{definition:?} offers declining"))
            .id;
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decline],
            },
        )
        .expect("declining is allowed");
        assert!(
            game.players[PlayerId::Two.index()]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::LIGHTNING_BOLT),
            "{definition:?} counters what went unpaid",
        );
    }
}

/// The other half of the same choice: paying the tax lets the spell
/// through, and the counter goes to the graveyard having done nothing.
#[test]
fn the_soft_counters_let_a_paid_spell_resolve() {
    for (definition, color, tax) in [
        (cards::MANA_TITHE, ManaColor::White, 1),
        (cards::SPELL_PIERCE, ManaColor::Blue, 2),
        (cards::MISCALCULATION, ManaColor::Blue, 2),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        let bolt = card(50_820, cards::LIGHTNING_BOLT, PlayerId::Two);
        let bolt_id = bolt.id;
        game.players[PlayerId::Two.index()].hand.push(bolt);
        game.players[PlayerId::Two.index()].mana_pool.red = 1;
        game.players[PlayerId::Two.index()].mana_pool.colorless = tax;
        game.players[PlayerId::One.index()].life = 20;
        game.priority = PlayerId::Two;
        game.apply(
            PlayerId::Two,
            cast_action(bolt_id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
        )
        .expect("the opponent casts something to counter");
        let on_stack = game.stack.last().expect("the spell is on the stack").id;
        game.apply(PlayerId::Two, Action::PassPriority).unwrap();

        let counter = card(50_821, definition, PlayerId::One);
        let counter_id = counter.id;
        game.players[PlayerId::One.index()].hand.push(counter);
        let pool = &mut game.players[PlayerId::One.index()].mana_pool;
        pool.add_color(color, 1);
        pool.colorless += 1;
        game.apply(
            PlayerId::One,
            cast_action(counter_id, vec![Target::Spell(on_stack)], Vec::new(), 0),
        )
        .unwrap_or_else(|error| panic!("{definition:?} answers a spell: {error}"));
        pass_priority_pair(&mut game);

        let decision = game
            .observe(PlayerId::Two)
            .decision
            .unwrap_or_else(|| panic!("{definition:?} asks its controller to pay {tax}"));
        let pay = decision
            .options
            .iter()
            .find(|option| option.label == "Pay the cost")
            .unwrap_or_else(|| panic!("{definition:?} offers paying"))
            .id;
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![pay],
            },
        )
        .expect("paying is allowed");
        drain_pending(&mut game);

        assert_eq!(
            game.players[PlayerId::One.index()].life,
            17,
            "{definition:?} was paid, so the Bolt resolved",
        );
        assert!(
            game.players[PlayerId::One.index()]
                .graveyard
                .iter()
                .any(|card| card.definition == definition),
            "{definition:?} is in its own graveyard either way",
        );
    }
}

/// "Noncreature spell" is what separates the Pierce from the other two: a
/// creature spell is not a legal target for it, and is for them.
#[test]
fn spell_pierce_alone_cannot_name_a_creature_spell() {
    for (definition, color, names_creatures) in [
        (cards::MANA_TITHE, ManaColor::White, true),
        (cards::MISCALCULATION, ManaColor::Blue, true),
        (cards::SPELL_PIERCE, ManaColor::Blue, false),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        let bears = card(50_830, cards::GRIZZLY_BEARS, PlayerId::Two);
        let bears_id = bears.id;
        game.players[PlayerId::Two.index()].hand.push(bears);
        game.players[PlayerId::Two.index()].mana_pool.green = 1;
        game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
        // A creature spell wants its caster's own main phase.
        game.active_player = PlayerId::Two;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::Two;
        game.apply(
            PlayerId::Two,
            cast_action(bears_id, Vec::new(), Vec::new(), 0),
        )
        .expect("a creature spell goes on the stack");
        let on_stack = game.stack.last().expect("it is waiting").id;
        game.apply(PlayerId::Two, Action::PassPriority).unwrap();

        let counter = card(50_831, definition, PlayerId::One);
        let counter_id = counter.id;
        game.players[PlayerId::One.index()].hand.push(counter);
        let pool = &mut game.players[PlayerId::One.index()].mana_pool;
        pool.add_color(color, 1);
        pool.colorless += 1;

        let offered = game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if card == counter_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Spell(on_stack)))
        });
        assert_eq!(
            offered, names_creatures,
            "{definition:?} against a creature spell",
        );
    }
}

#[test]
fn mother_of_runes_protects_a_creature_from_the_color_she_names() {
    let mut game = ready_game();
    game.battlefield.clear();
    // She has been here since before this turn, so her tap ability is live.
    let mother_permanent = creature(50_899, cards::MOTHER_OF_RUNES, PlayerId::One);
    let mother = mother_permanent.card.id;
    game.battlefield.push(mother_permanent);
    let bears = creature(50_900, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == mother
                && targets.iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(bears_id))
                }))
        })
        .expect("she can target another creature you control");
    game.apply(PlayerId::One, action).expect("it is activated");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the color is chosen on resolution");
    let red = decision
        .options
        .iter()
        .find(|option| option.label.eq_ignore_ascii_case("red"))
        .expect("red is one of the colors")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![red],
        },
    )
    .expect("the color is chosen");

    // A red spell can no longer touch what she named.
    let bolt = card(50_901, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.priority = PlayerId::Two;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == bolt_id
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(bears_id))
                }))
        }),
        "the protected creature is not an offered target",
    );
}

/// Life: the lands stay lands, which is the half of the sentence that matters.
#[test]
fn life_animates_your_lands_without_taking_their_land_type_away() {
    let mut game = ready_game();
    game.battlefield.clear();
    let forest = creature(53_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    // Someone else's land is not "you control".
    let island = creature(53_001, cards::ISLAND, PlayerId::Two);
    let island_id = island.card.id;
    game.battlefield.push(island);

    let life = card(53_002, cards::LIFE_DEATH, PlayerId::One);
    let life_id = life.id;
    game.players[PlayerId::One.index()].hand.push(life);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == life_id && choices.play_option() == PlayOptionId::DEFAULT)
        })
        .expect("Life is the first half");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    let types = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| game.permanent_types(permanent))
            .expect("the land is still there")
    };
    assert!(types(forest_id).contains(crate::card::CardType::Creature));
    assert!(
        types(forest_id).contains(crate::card::CardType::Land),
        "they're still lands",
    );
    let forest = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == forest_id)
        .expect("the Forest is still there");
    assert_eq!(
        (game.power(forest), game.toughness(forest)),
        (Some(1), Some(1)),
    );
    assert!(
        !types(island_id).contains(crate::card::CardType::Creature),
        "only lands you control are animated",
    );
}

/// Death: the other half of the same card, reaching only into your own
/// graveyard and charging you the creature's mana value in life.
#[test]
fn death_reanimates_from_your_own_graveyard_for_its_mana_value_in_life() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.push(card(
        53_100,
        cards::SERRA_ANGEL,
        PlayerId::One,
    ));
    game.players[PlayerId::Two.index()].graveyard.push(card(
        53_101,
        cards::GRIZZLY_BEARS,
        PlayerId::Two,
    ));

    let death = card(53_102, cards::LIFE_DEATH, PlayerId::One);
    let death_id = death.id;
    game.players[PlayerId::One.index()].hand.push(death);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let life = game.players[PlayerId::One.index()].life;

    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == death_id && choices.play_option() == PlayOptionId(1) =>
            {
                Some(choices.targets().to_vec())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered.len(),
        1,
        "only the creature in your own graveyard is a legal target",
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == death_id && choices.play_option() == PlayOptionId(1))
        })
        .expect("Death is the second half");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    assert!(
        game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.definition == cards::SERRA_ANGEL
        }),
        "the angel comes back under your control",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        life - 5,
        "a five-drop costs five life",
    );
}

/// Rancor on a creature: bigger, trampling, and still there.
#[test]
fn rancor_grants_two_power_and_trample() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(54_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let rancor = card(54_001, cards::RANCOR, PlayerId::One);
    let rancor_id = rancor.id;
    game.players[PlayerId::One.index()].hand.push(rancor);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == rancor_id
                && choices.targets().iter().any(|selection| {
                    selection.targets().contains(&Target::Permanent(bears_id))
                }))
        })
        .expect("Rancor targets a creature");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("the creature is still there");
    assert_eq!(
        (game.power(bears), game.toughness(bears)),
        (Some(4), Some(2))
    );
    assert!(game.has_trample(bears));
}

/// The clause the card is remembered for. Whichever half of the pair is
/// answered, the Aura reaches the graveyard, and it is the graveyard object
/// -- a different object from the permanent that just left -- that comes back
/// to hand.
#[test]
fn rancor_returns_itself_to_hand_from_the_graveyard() {
    for kill_the_creature in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        let bears = creature(54_100, cards::GRIZZLY_BEARS, PlayerId::One);
        let bears_id = bears.card.id;
        game.battlefield.push(bears);
        let rancor = card(54_101, cards::RANCOR, PlayerId::One);
        let rancor_id = rancor.id;
        game.players[PlayerId::One.index()].hand.push(rancor);
        game.players[PlayerId::One.index()].mana_pool.green = 1;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == rancor_id
                    && choices.targets().iter().any(|selection| {
                        selection.targets().contains(&Target::Permanent(bears_id))
                    }))
            })
            .expect("Rancor targets a creature");
        game.apply(PlayerId::One, action).expect("it is cast");
        drain_pending(&mut game);

        // The Aura on the battlefield is a new object; the hand card's id is
        // not the permanent's.
        let aura = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::RANCOR)
            .expect("the Aura attached")
            .card
            .id;
        let doomed = if kill_the_creature { bears_id } else { aura };
        game.move_permanents_to_graveyard(&[doomed]);
        // Killing the creature leaves the Aura enchanting nothing; it takes
        // state-based actions to notice and send it after its host.
        game.check_state_based_actions();
        drain_pending(&mut game);

        assert!(
            game.players[PlayerId::One.index()]
                .hand
                .iter()
                .any(|card| card.definition == cards::RANCOR),
            "Rancor comes back whether the creature or the Aura was answered \
             (creature killed: {kill_the_creature})",
        );
        assert!(
            game.players[PlayerId::One.index()]
                .graveyard
                .iter()
                .all(|card| card.definition != cards::RANCOR),
            "and it does not stay in the graveyard as well",
        );
    }
}

/// "Power or toughness 2 or less" is a disjunction. A 4/1 qualifies on
/// toughness and a 2/3 on power; only a creature big in both directions is
/// safe, and a noncreature spell was never in question.
#[test]
fn stern_scolding_answers_a_spell_small_in_either_direction() {
    for (spell, counterable) in [
        (cards::GRIZZLY_BEARS, true),
        // A 4/1: too big to be caught by power, small enough by toughness.
        (cards::PHANTASMAL_FORCES, true),
        // A 2/3: the mirror of it.
        (cards::ERG_RAIDERS, true),
        (cards::SERRA_ANGEL, false),
        // Not a creature spell at all.
        (cards::LIGHTNING_BOLT, false),
    ] {
        // The active player casts the creature; the other one answers it.
        let mut game = ready_game();
        let cast = card(55_000, spell, PlayerId::One);
        let cast_id = cast.id;
        game.players[PlayerId::One.index()].hand.push(cast);
        let pool = &mut game.players[PlayerId::One.index()].mana_pool;
        pool.white = 5;
        pool.blue = 5;
        pool.black = 5;
        pool.red = 5;
        pool.green = 5;
        pool.colorless = 5;
        let cast_action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == cast_id))
            .unwrap_or_else(|| panic!("{spell:?} is castable"));
        game.apply(PlayerId::One, cast_action).expect("it is cast");
        let on_stack = game.stack.last().expect("it is on the stack").id;
        game.apply(PlayerId::One, Action::PassPriority).unwrap();

        let scolding = card(55_001, cards::STERN_SCOLDING, PlayerId::Two);
        let scolding_id = scolding.id;
        game.players[PlayerId::Two.index()].hand.push(scolding);
        game.players[PlayerId::Two.index()].mana_pool.blue = 1;

        let offered = |game: &Game| {
            game.legal_actions(PlayerId::Two)
                .into_iter()
                .find(|action| {
                    matches!(action, Action::CastSpell { card, choices, .. }
                    if *card == scolding_id
                        && choices.targets().iter().any(|selection| {
                            selection.targets().contains(&Target::Spell(on_stack))
                        }))
                })
        };
        let action = offered(&game);
        assert_eq!(
            action.is_some(),
            counterable,
            "{spell:?} should{} be a legal target",
            if counterable { "" } else { " not" },
        );

        let Some(action) = action else {
            continue;
        };
        game.apply(PlayerId::Two, action).expect("it is cast");
        drain_pending(&mut game);
        assert!(
            game.players[PlayerId::One.index()]
                .graveyard
                .iter()
                .any(|card| card.definition == spell),
            "{spell:?} is countered",
        );
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.definition != spell),
            "{spell:?} never reaches the battlefield",
        );
    }
}

/// The mana-value bound is a targeting restriction, not a resolution check,
/// so an unkicked Thirst is never offered the bigger creature at all. Paying
/// the kicker is a second, dearer cast of the same card that can.
#[test]
fn bloodchiefs_thirst_reaches_past_two_mana_only_when_kicked() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(78_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let angel = creature(78_001, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let thirst = card(78_002, cards::BLOODCHIEFS_THIRST, PlayerId::One);
    let thirst_id = thirst.id;
    game.players[0].hand.push(thirst);
    game.players[0].mana_pool.black = 1;

    let targets = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == thirst_id => {
                    choices.iter_targets().copied().next()
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(
        targets(&game),
        vec![Target::Permanent(bears_id)],
        "one black mana reaches the 2/2 and nothing else",
    );

    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 2;
    let mut kicked = targets(&game);
    kicked.sort_unstable();
    kicked.dedup();
    assert!(
        kicked.contains(&Target::Permanent(angel_id)),
        "four mana reaches the Angel",
    );
}

/// A kicked Thirst destroys what an unkicked one could not, and the extra
/// mana is really spent.
#[test]
fn bloodchiefs_thirst_kicked_destroys_a_large_creature() {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = creature(78_010, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let thirst = card(78_011, cards::BLOODCHIEFS_THIRST, PlayerId::One);
    let thirst_id = thirst.id;
    game.players[0].hand.push(thirst);
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == thirst_id
                    && choices.iter_targets().any(|target| *target == Target::Permanent(angel_id)))
        })
        .expect("the kicked cast is on offer");
    game.apply(PlayerId::One, action).expect("it is cast");
    resolve(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel_id),
        "the Angel is destroyed",
    );
    assert_eq!(
        game.players[0].mana_pool.total(),
        0,
        "the kicker is paid, not just declared",
    );
}

/// Cast normally it takes one creature, and its controller gets one basic
/// land back, tapped.
#[test]
fn winds_of_abandon_exiles_one_and_pays_a_land_for_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(93_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[1].library.clear();
    game.players[1]
        .library
        .push(card(93_001, cards::FOREST, PlayerId::Two));
    let winds = card(93_002, cards::WINDS_OF_ABANDON, PlayerId::One);
    let winds_id = winds.id;
    game.players[0].hand.push(winds);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == winds_id))
        .expect("two mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "the creature is gone",
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "exiled rather than destroyed, so nothing rebuilds from it",
    );
    let forest = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::FOREST)
        .expect("its controller found a basic land");
    assert_eq!(forest.controller, PlayerId::Two);
    assert!(forest.tapped, "which arrives tapped");
}

/// Overloaded it takes every creature you don't control, leaves your own
/// alone, and pays one land per creature it took.
#[test]
fn overloaded_winds_takes_their_board_and_leaves_yours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mine = creature(93_010, cards::SAVANNAH_LIONS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    for id in 93_011..93_014 {
        game.battlefield
            .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::Two));
    }
    game.players[1].library.clear();
    for id in 93_020..93_025 {
        game.players[1]
            .library
            .push(card(id, cards::FOREST, PlayerId::Two));
    }
    let winds = card(93_030, cards::WINDS_OF_ABANDON, PlayerId::One);
    let winds_id = winds.id;
    game.players[0].hand.push(winds);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == winds_id && choices.costs().alternative().is_some()
            }
            _ => false,
        })
        .expect("six mana overloads it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);

    // "For each creature exiled this way" is three, not five: the count
    // comes from what was taken rather than from the library.
    let search = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("their search asks which lands to take");
    assert_eq!(search.player, PlayerId::Two);
    assert_eq!(search.maximum, 3, "one land per creature exiled");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: search.id,
            options: search
                .options
                .iter()
                .take(3)
                .map(|option| option.id)
                .collect(),
        },
    )
    .expect("taking three is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .filter(|card| card.definition == cards::GRIZZLY_BEARS)
            .count(),
        3,
        "every creature they controlled is exiled",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == mine_id),
        "and your own is untouched",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::FOREST)
            .count(),
        3,
        "three lands came back, tapped",
    );
}

/// Four damage for one mana, at sorcery speed and only at creatures.
#[test]
fn flame_slash_kills_a_four_toughness_creature_and_cannot_go_upstairs() {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = creature(96_000, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let slash = card(96_001, cards::FLAME_SLASH, PlayerId::One);
    let slash_id = slash.id;
    game.players[0].hand.push(slash);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == slash_id => {
                choices.iter_targets().copied().next()
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![Target::Permanent(angel_id)],
        "the only legal target is the creature, never a player",
    );

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == slash_id))
        .expect("one red mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    resolve(&mut game);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel_id),
        "four damage kills a 4/4",
    );
}
