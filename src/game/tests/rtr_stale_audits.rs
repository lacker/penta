//! Return to Ravnica cards unlocked by recent declarative engine support.

use super::*;

fn card_effect(game: &Game, definition: CardDefinitionId) -> EffectDef {
    game.catalog
        .get(definition)
        .expect("the card is cataloged")
        .rules
        .ability_clauses()[0]
        .declarative_effect()
        .expect("the card has a declarative effect")
}

#[test]
fn epic_experiment_uses_chosen_x_for_the_exiled_collection() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].library.extend([
        card(81_000, cards::DIVINE_RECKONING, PlayerId::One),
        card(81_001, cards::GRIZZLY_BEARS, PlayerId::One),
        card(81_002, cards::PONDER, PlayerId::One),
    ]);
    let experiment = spell(81_010, cards::EPIC_EXPERIMENT, PlayerId::One, 3);

    game.resolve_effect_def(
        ScopedEffect::primary(card_effect(&game, cards::EPIC_EXPERIMENT)),
        &experiment,
        TriggerContext::empty(),
    );

    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the qualifying spell is offered");
    assert!(offer.prompt.contains("Ponder"));
    assert!(
        !offer.prompt.contains("Divine Reckoning"),
        "the four-mana sorcery is above the chosen X",
    );
    assert!(
        !offer.prompt.contains("Grizzly Bears"),
        "a creature is not an instant or sorcery",
    );

    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    drain_pending(&mut game);

    assert!(game.players[0].exile.is_empty());
    let graveyard = game.players[0]
        .graveyard
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    assert!(graveyard.contains(&cards::PONDER));
    assert!(graveyard.contains(&cards::GRIZZLY_BEARS));
    assert!(graveyard.contains(&cards::DIVINE_RECKONING));
}

#[test]
fn essence_backlash_reads_the_countered_creature_spells_last_known_power() {
    let mut game = ready_game();
    let creature_spell = spell(81_100, cards::SERRA_ANGEL, PlayerId::Two, 0);
    let creature_spell_id = creature_spell.id;
    game.stack.push(creature_spell);
    let backlash = spell_with_targets(
        81_101,
        cards::ESSENCE_BACKLASH,
        PlayerId::One,
        vec![Target::Spell(creature_spell_id)],
        0,
    );

    game.resolve_effect_def(
        ScopedEffect::primary(card_effect(&game, cards::ESSENCE_BACKLASH)),
        &backlash,
        TriggerContext::empty(),
    );

    assert!(
        game.stack
            .iter()
            .all(|object| object.id != creature_spell_id),
        "the creature spell was countered",
    );
    assert_eq!(game.current_or_last_known_power(creature_spell_id), Some(4));
    assert_eq!(
        game.current_or_last_known_toughness(creature_spell_id),
        Some(4),
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        16,
        "Essence Backlash reads Serra Angel's four power after it leaves the stack",
    );
}

#[test]
fn grave_betrayal_returns_each_dead_opposing_creature_at_the_next_end_step() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::GRAVE_BETRAYAL)
        .expect("Grave Betrayal is cataloged");
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is cataloged");
    drain_pending(&mut game);

    game.destroy_permanents(&[bear, angel], true);
    drain_pending(&mut game);
    assert_eq!(
        game.installed_triggers.len(),
        2,
        "each death installed its own delayed trigger",
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .filter(|card| { matches!(card.definition, cards::GRIZZLY_BEARS | cards::SERRA_ANGEL) })
            .count()
            == 2,
        "the delayed triggers leave both dead cards in their graveyard until the end step",
    );

    game.step = Step::End;
    game.begin_step_triggers();
    assert_eq!(
        game.pending_triggers.len(),
        2,
        "both delayed triggers fired"
    );
    drain_pending(&mut game);

    for (definition, original_color) in [
        (cards::GRIZZLY_BEARS, ManaColor::Green),
        (cards::SERRA_ANGEL, ManaColor::White),
    ] {
        let returned = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("each dead creature returned");
        assert_eq!(returned.controller, PlayerId::One);
        assert_eq!(returned.counters(CounterKind::PlusOnePlusOne), 1);
        assert!(game.effective_subtypes(returned).contains(&"Zombie"));
        let colors = game.effective_colors(returned, &game.effective_rules(returned).unwrap());
        assert!(colors[ManaColor::Black.index()]);
        assert!(colors[original_color.index()]);
    }
}

#[test]
fn guild_feud_keeps_both_reveal_workflows_bound_across_sequence_steps() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].library.extend([
        card(81_300, cards::PONDER, PlayerId::One),
        card(81_301, cards::DIVINE_RECKONING, PlayerId::One),
        card(81_302, cards::GRIZZLY_BEARS, PlayerId::One),
    ]);
    game.players[PlayerId::Two.index()].library.clear();
    game.players[PlayerId::Two.index()].library.extend([
        card(81_310, cards::PONDER, PlayerId::Two),
        card(81_311, cards::DIVINE_RECKONING, PlayerId::Two),
        card(81_312, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let feud = spell_with_targets(
        81_320,
        cards::GUILD_FEUD,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );

    game.resolve_effect_def(
        ScopedEffect::primary(card_effect(&game, cards::GUILD_FEUD)),
        &feud,
        TriggerContext::empty(),
    );

    choose_decision_by_label(&mut game, PlayerId::Two, "Serra Angel");
    choose_decision_by_label(&mut game, PlayerId::One, "Grizzly Bears");
    drain_pending(&mut game);

    assert!(
        game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::Two && permanent.card.definition == cards::SERRA_ANGEL
        }),
        "the opponent's chosen creature entered and survived the fight",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "the controller's chosen creature entered, fought, and died",
    );
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(
            game.players[player.index()]
                .graveyard
                .iter()
                .filter(|card| {
                    matches!(card.definition, cards::PONDER | cards::DIVINE_RECKONING)
                })
                .count(),
            2,
            "that player's unchosen cards went to their graveyard",
        );
    }
}

#[test]
fn havoc_festival_statically_stops_life_gain_and_halves_the_active_player() {
    let mut game = ready_game();
    let festival = game
        .put_onto_battlefield(PlayerId::One, cards::HAVOC_FESTIVAL)
        .expect("Havoc Festival is cataloged");
    drain_pending(&mut game);

    game.players[0].life = 5;
    game.players[1].life = 7;
    game.gain_life(PlayerId::One, 3);
    game.gain_life(PlayerId::Two, 3);
    assert_eq!(game.players[0].life, 5);
    assert_eq!(game.players[1].life, 7);

    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 5, "only the active player loses life");
    assert_eq!(game.players[1].life, 3, "half of seven rounded up is four");

    game.destroy_permanents(&[festival], false);
    drain_pending(&mut game);
    game.gain_life(PlayerId::One, 3);
    game.gain_life(PlayerId::Two, 3);
    assert_eq!(game.players[0].life, 8);
    assert_eq!(game.players[1].life, 6);
}

#[test]
fn mercurial_chemister_binds_the_card_discarded_to_its_damage_amount() {
    let mut game = ready_game();
    let chemister = game
        .put_onto_battlefield(PlayerId::One, cards::MERCURIAL_CHEMISTER)
        .expect("Mercurial Chemister is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == chemister)
        .expect("the Chemister entered")
        .entered_controller_turn = 0;
    let discarded = card(81_200, cards::DIVINE_RECKONING, PlayerId::One);
    let discarded_id = discarded.id;
    game.players[0].hand.push(discarded);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    drain_pending(&mut game);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    cost_objects,
                    ..
                } if *source == chemister
                    && cost_objects.as_slice() == [discarded_id]
                    && targets.iter().any(|selection| {
                        selection.targets().contains(&Target::Player(PlayerId::Two))
                    })
            )
        })
        .expect("the discarded card can pay for the targeted damage ability");
    game.apply(PlayerId::One, action)
        .expect("the Chemister ability activates");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::DIVINE_RECKONING),
        "the chosen card was discarded as the activation cost",
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        16,
        "Divine Reckoning's mana value makes the Chemister deal four",
    );
}
