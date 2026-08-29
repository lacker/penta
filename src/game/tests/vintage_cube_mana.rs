//! Mana sources cataloged for the Vintage Cube pool.

use super::*;

/// The Halfling's coloured mana is Cavern of Souls narrowed to a different
/// question: not a creature type, but whether the spell is legendary.
#[test]
fn the_halflings_colored_mana_only_pays_for_legendary_spells() {
    for (spell, castable) in [(cards::TIFA_LOCKHART, true), (cards::GRIZZLY_BEARS, false)] {
        let mut game = ready_game();
        game.battlefield.clear();
        let halfling = creature(74_000, cards::DELIGHTED_HALFLING, PlayerId::One);
        let halfling_id = halfling.card.id;
        game.battlefield.push(halfling);
        let spell_card = card(74_001, spell, PlayerId::One);
        let spell_id = spell_card.id;
        game.players[0].hand.push(spell_card);

        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: halfling_id,
                ability: mana_ability_for(&game, halfling_id, ManaColor::Green),
                color: ManaColor::Green,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .expect("it taps for a colour");
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

        assert_eq!(
            game.legal_actions(PlayerId::One).iter().any(
                |action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id)
            ),
            castable,
            "{spell:?} should{} be castable on Halfling mana",
            if castable { "" } else { " not" },
        );
    }
}

/// And what it does pay for cannot be countered.
#[test]
fn a_legendary_spell_paid_with_halfling_mana_cannot_be_countered() {
    let mut game = ready_game();
    game.battlefield.clear();
    let halfling = creature(74_100, cards::DELIGHTED_HALFLING, PlayerId::One);
    let halfling_id = halfling.card.id;
    game.battlefield.push(halfling);
    let tifa = card(74_101, cards::TIFA_LOCKHART, PlayerId::One);
    let tifa_id = tifa.id;
    game.players[0].hand.push(tifa);
    let counterspell = card(74_102, cards::COUNTERSPELL, PlayerId::Two);
    let counterspell_id = counterspell.id;
    game.players[1].hand.push(counterspell);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: halfling_id,
            ability: mana_ability_for(&game, halfling_id, ManaColor::Green),
            color: ManaColor::Green,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for green");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tifa_id))
        .expect("a legendary spell is castable on this mana");
    game.apply(PlayerId::One, cast).expect("it is cast");
    assert!(
        game.stack
            .last()
            .expect("Tifa is on the stack")
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect
                        .source
                        .is_some_and(|source| source.object == halfling_id)
            }),
        "the rider rides on the spell, not on the Halfling",
    );

    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let target = game.stack.last().expect("still there").id;
    game.apply(
        PlayerId::Two,
        cast_action(counterspell_id, vec![Target::Spell(target)], Vec::new(), 0),
    )
    .expect("the counterspell is castable");
    pass_priority_pair(&mut game);
    assert_eq!(
        game.stack.len(),
        1,
        "the counterspell resolved and left her on the stack",
    );
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::TIFA_LOCKHART),
        "and she resolves anyway",
    );
}

/// The colourless half carries neither the restriction nor the rider.
#[test]
fn the_halflings_colorless_mana_is_ordinary() {
    let mut game = ready_game();
    game.battlefield.clear();
    let halfling = creature(74_200, cards::DELIGHTED_HALFLING, PlayerId::One);
    let halfling_id = halfling.card.id;
    game.battlefield.push(halfling);
    let bears = card(74_201, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    game.players[0].hand.push(bears);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: halfling_id,
            ability: mana_ability_for(&game, halfling_id, ManaColor::Colorless),
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for colourless");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == bears_id)),
        "a nonlegendary creature can be cast on the colourless mana",
    );
}

/// The life is a cost, not a trigger. City of Brass pays when it becomes
/// tapped, by anyone; the Confluence pays only when its own ability is
/// activated, and cannot be activated at all with no life to spare.
#[test]
fn mana_confluence_charges_a_life_as_a_cost_of_its_own_ability() {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        let land = game
            .put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
            .expect("cataloged");
        game.players[PlayerId::One.index()].life = 20;

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
        .unwrap_or_else(|error| panic!("it makes {color:?}: {error}"));
        assert_eq!(
            game.players[PlayerId::One.index()].mana_pool.amount(color),
            1
        );
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            19,
            "one life for one mana",
        );
    }
}

#[test]
fn mana_confluence_life_costs_are_aggregated_for_ordinary_autopayment() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
        .expect("cataloged");
    let bears = card(74_500, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.id;
    game.players[PlayerId::One.index()].hand.push(bears);

    game.players[PlayerId::One.index()].life = 2;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bears_id))
        .expect("two life can pay both Mana Confluence activation costs");

    game.players[PlayerId::One.index()].life = 1;
    assert!(
        !game.is_legal_action(PlayerId::One, &cast),
        "one life cannot pay two separate Mana Confluence activation costs",
    );
    assert!(
        game.apply(PlayerId::One, cast).is_err(),
        "ordinary autopayment also rejects the aggregate life overcommit",
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.id == bears_id),
        "rejecting the stale cast leaves the spell in hand",
    );
}

#[test]
fn ordinary_autopayment_cannot_sacrifice_one_shared_cost_object_twice() {
    static SACRIFICE_A_CREATURE: [AbilityCostDef; 1] = [AbilityCostDef::SacrificePermanent {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        controller: PlayerRelation::You,
    }];

    let source_definition = CardDefinitionId::new(50_003);
    let mut definition = CardDefinition::new(
        source_definition,
        "Shared sacrifice mana source",
        CardSet::FutureSight,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_mana(
            "Sacrifice a creature: Add {G}.",
            &SACRIFICE_A_CREATURE,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ));
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the mana-source fixture is valid");
    game.battlefield.clear();
    game.battlefield.extend([
        creature(74_600, source_definition, PlayerId::One),
        creature(74_601, source_definition, PlayerId::One),
        creature(74_602, cards::GRIZZLY_BEARS, PlayerId::One),
    ]);
    let spell = card(74_603, cards::GRIZZLY_BEARS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == spell_id)),
        "two mana abilities cannot both reserve the one creature they share as a sacrifice cost",
    );
}

/// Tapped by something else, it costs nothing -- the difference from City of
/// Brass, which would pay either way.
#[test]
fn mana_confluence_costs_nothing_when_something_else_taps_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    let land = game
        .put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
        .expect("cataloged");
    game.players[PlayerId::One.index()].life = 20;

    game.tap_permanent(land);
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.total(), 0);
}

/// "Add X mana in any combination of {U} and/or {R}" is offered once per
/// division of X, not once per colour: a Vivi with two power can make two
/// blue, two red, or one of each.
#[test]
fn vivi_offers_every_division_of_its_power() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut vivi = creature(75_000, cards::VIVI_ORNITIER, PlayerId::One);
    vivi.add_counters(CounterKind::PlusOnePlusOne, 2);
    let vivi_id = vivi.card.id;
    game.battlefield.push(vivi);

    let divisions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source,
                combination: Some(division),
                ..
            } if source == vivi_id => {
                Some((division.get(ManaColor::Blue), division.get(ManaColor::Red)))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(divisions, vec![(0, 2), (1, 1), (2, 0)]);
}

/// The division the action names is the mana the pool receives, so one blue
/// and one red pays a cost that neither two blue nor two red could.
#[test]
fn vivi_pays_a_two_colored_cost_with_one_activation() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut vivi = creature(75_010, cards::VIVI_ORNITIER, PlayerId::One);
    vivi.add_counters(CounterKind::PlusOnePlusOne, 2);
    let vivi_id = vivi.card.id;
    game.battlefield.push(vivi);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, combination: Some(division), .. }
                    if *source == vivi_id
                        && division.get(ManaColor::Blue) == 1
                        && division.get(ManaColor::Red) == 1
            )
        })
        .expect("one of each is a division of two");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].mana_pool.red, 1);
}

/// "Activate only during your turn and only once each turn" gates a mana
/// ability the way it gates any other, even though mana abilities are
/// enumerated on their own path.
#[test]
fn vivi_makes_mana_once_a_turn_and_only_on_its_own() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut vivi = creature(75_020, cards::VIVI_ORNITIER, PlayerId::One);
    vivi.add_counters(CounterKind::PlusOnePlusOne, 1);
    let vivi_id = vivi.card.id;
    game.battlefield.push(vivi);

    let offers = |game: &Game, player| {
        game.legal_actions(player)
            .into_iter()
            .any(|action| matches!(action, Action::ActivateManaAbility { source, .. } if source == vivi_id))
    };

    assert!(
        offers(&game, PlayerId::One),
        "its controller's turn is open"
    );
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == vivi_id))
        .expect("the ability is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);
    assert!(!offers(&game, PlayerId::One), "once each turn means once");

    game.active_player = PlayerId::Two;
    assert!(
        !offers(&game, PlayerId::One),
        "the window closes on the opponent's turn"
    );
}

/// The trigger grows Vivi before it burns, so the mana ability the same turn
/// reads the larger power.
#[test]
fn vivi_grows_and_burns_on_a_noncreature_spell() {
    let mut game = ready_game();
    game.battlefield.clear();
    let vivi = creature(75_030, cards::VIVI_ORNITIER, PlayerId::One);
    let vivi_id = vivi.card.id;
    game.battlefield.push(vivi);
    game.players[0]
        .hand
        .push(card(75_031, cards::LIGHTNING_BOLT, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { .. }))
        .expect("the Bolt is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    let vivi = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vivi_id)
        .expect("Vivi survives its own trigger");
    assert_eq!(vivi.counters(CounterKind::PlusOnePlusOne), 1);
    assert!(game.players[1].life < 20, "the opponent takes the damage");
}

/// The five Moxen are one card printed five ways: a free artifact that taps
/// for exactly one colour. What is worth checking per member is which.
#[test]
fn each_mox_taps_for_its_own_color_and_no_other() {
    for (definition, color) in [
        (cards::MOX_EMERALD, ManaColor::Green),
        (cards::MOX_JET, ManaColor::Black),
        (cards::MOX_PEARL, ManaColor::White),
        (cards::MOX_RUBY, ManaColor::Red),
        (cards::MOX_SAPPHIRE, ManaColor::Blue),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        let mox = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        drain_pending(&mut game);
        game.turns_started = [5, 5];
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;

        let offered = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == mox)
            .map(|permanent| {
                game.mana_ability_activations(permanent)
                    .into_iter()
                    .map(|activation| activation.color)
                    .collect::<Vec<_>>()
            })
            .expect("it is on the battlefield");
        assert_eq!(
            offered,
            vec![color],
            "{definition:?} makes {color:?} and nothing else",
        );

        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: mox,
                ability: mana_ability_for(&game, mox, color),
                color,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .unwrap_or_else(|error| panic!("{definition:?} taps for {color:?}: {error}"));

        assert_eq!(game.players[0].mana_pool.amount(color), 1);
        assert_eq!(
            game.players[0].mana_pool.total(),
            1,
            "one mana, and no life or damage on the way",
        );
        assert_eq!(game.players[0].life, 20);
    }
}

/// A mana creature's tap is a creature's tap: it waits for the turn after it
/// arrives (CR 302.6), unlike the artifacts beside it in the same slot. The
/// Elves make green and the Birds make whatever you name.
#[test]
fn a_mana_creature_waits_a_turn_and_then_makes_its_mana() {
    for (definition, colors) in [
        (cards::LLANOWAR_ELVES, &[ManaColor::Green][..]),
        (cards::BIRDS_OF_PARADISE, &ManaColor::COLORS[..]),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.turns_started = [5, 5];
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        let creature = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        drain_pending(&mut game);

        let offered = |game: &Game| {
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == creature)
                .map(|permanent| {
                    game.mana_ability_activations(permanent)
                        .into_iter()
                        .map(|activation| activation.color)
                        .collect::<Vec<_>>()
                })
                .expect("it is on the battlefield")
        };
        assert!(
            offered(&game).is_empty(),
            "{definition:?} arrived this turn, so its tap is not on offer",
        );

        // The same permanent, one turn older.
        if let Some(permanent) = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == creature)
        {
            permanent.entered_controller_turn = 0;
        }
        let mut made = offered(&game);
        made.sort_unstable();
        let mut expected = colors.to_vec();
        expected.sort_unstable();
        assert_eq!(made, expected, "{definition:?} makes what it prints");

        let color = colors[0];
        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: creature,
                ability: mana_ability_for(&game, creature, color),
                color,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .unwrap_or_else(|error| panic!("{definition:?} taps for {color:?}: {error}"));
        assert_eq!(game.players[0].mana_pool.amount(color), 1);
    }
}
