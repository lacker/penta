use super::*;

static CARD_COST_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is in the built-in catalog"))
}

fn unrestricted(color: ManaColor, amount: usize) -> Vec<Mana> {
    (0..amount).map(|_| Mana::unrestricted(color)).collect()
}

#[test]
fn rift_bolt_suspends_and_the_last_counter_creates_a_cast_trigger() {
    let mut game = ready_game();
    let rift = card(90_000, definition(&game, "Rift Bolt"), PlayerId::One);
    let old_id = rift.id;
    game.players[0].hand.push(rift);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Red, 1));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::Suspend { card, .. } if *card == old_id))
        .expect("Rift Bolt may be suspended for red");
    game.apply(PlayerId::One, action).unwrap();

    let exiled = game.players[0].exile[0].id;
    assert_ne!(
        exiled, old_id,
        "the hand-to-exile move creates a new object"
    );
    assert_eq!(
        game.players[0].exile[0]
            .counters
            .count(CounterKind::named("time")),
        1
    );
    assert!(game.is_suspended(exiled));

    game.remove_counters_from_object(Target::Card(exiled), CounterKind::named("time"), 1);
    assert_eq!(game.pending_triggers.len(), 1);
    assert!(game.pending_triggers[0].text.contains("last time counter"));
}

#[test]
fn ancestral_vision_has_no_normal_hand_cast_but_can_be_suspended() {
    let mut game = ready_game();
    let vision = card(90_010, definition(&game, "Ancestral Vision"), PlayerId::One);
    let id = vision.id;
    game.players[0].hand.push(vision);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Blue, 1));

    let actions = game.legal_actions(PlayerId::One);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    );
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::Suspend { card, .. } if *card == id))
    );
}

#[test]
fn clockspinning_reaches_a_suspended_card_but_not_an_arbitrary_exiled_card() {
    let mut game = ready_game();
    let clock = card(90_020, definition(&game, "Clockspinning"), PlayerId::One);
    let clock_id = clock.id;
    game.players[0].hand.push(clock);

    let mut suspended = card(90_021, definition(&game, "Rift Bolt"), PlayerId::One);
    suspended.counters.add(CounterKind::named("time"), 1);
    let suspended_id = suspended.id;
    game.players[0].exile.push(suspended);
    let mut ordinary = card(90_022, definition(&game, "Lightning Bolt"), PlayerId::One);
    ordinary.counters.add(CounterKind::named("time"), 1);
    let ordinary_id = ordinary.id;
    game.players[0].exile.push(ordinary);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Blue, 1));

    let targeted = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == clock_id => Some(
                choices
                    .targets()
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    assert!(targeted.contains(&Target::Card(suspended_id)));
    assert!(!targeted.contains(&Target::Card(ordinary_id)));
}

#[test]
fn clockspinning_chooses_the_actual_counter_kind_on_resolution() {
    let mut game = ready_game();
    let clock = card(90_023, definition(&game, "Clockspinning"), PlayerId::One);
    let clock_id = clock.id;
    game.players[0].hand.push(clock);
    let mut target = creature(90_024, cards::SAVANNAH_LIONS, PlayerId::One);
    let target_id = target.card.id;
    target.add_counters(CounterKind::named("time"), 1);
    target.add_counters(CounterKind::named("charge"), 2);
    game.battlefield.push(target);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Blue, 1));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == clock_id
                    && choices
                        .targets()
                        .iter()
                        .any(|selection| selection.targets() == [Target::Permanent(target_id)])
            }
            _ => false,
        })
        .expect("Clockspinning may choose its add-another mode and the countered permanent");
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Choose a charge counter");
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Put another of the chosen counter",
    );

    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target_id)
        .unwrap();
    assert_eq!(target.counters(CounterKind::named("charge")), 3);
    assert_eq!(target.counters(CounterKind::named("time")), 1);
}

#[test]
fn venser_diffusion_excludes_land_permanents_but_includes_suspended_cards() {
    let mut game = ready_game();
    let diffusion = card(
        90_025,
        definition(&game, "Venser's Diffusion"),
        PlayerId::One,
    );
    let diffusion_id = diffusion.id;
    game.players[0].hand.push(diffusion);
    let land = creature(90_026, cards::MOUNTAIN, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);
    let mut suspended = card(90_027, definition(&game, "Rift Bolt"), PlayerId::Two);
    suspended.counters.add(CounterKind::named("time"), 1);
    let suspended_id = suspended.id;
    game.players[1].exile.push(suspended);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Blue, 3));

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == diffusion_id => Some(
                choices
                    .targets()
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    assert!(!targets.contains(&Target::Permanent(land_id)));
    assert!(targets.contains(&Target::Card(suspended_id)));

    let bounce = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == diffusion_id
                    && choices
                        .iter_targets()
                        .copied()
                        .eq([Target::Card(suspended_id)])
            }
            _ => false,
        })
        .expect("the suspended card is a legal Diffusion target");
    game.apply(PlayerId::One, bounce).unwrap();
    pass_priority_pair(&mut game);
    assert!(game.players[1].exile.is_empty());
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == definition(&game, "Rift Bolt"))
    );
}

#[test]
fn fungal_behemoth_suspend_x_starts_at_one_and_uses_the_chosen_x() {
    let mut game = ready_game();
    let behemoth = card(90_034, definition(&game, "Fungal Behemoth"), PlayerId::One);
    let id = behemoth.id;
    game.players[0].hand.push(behemoth);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Green, 5));

    let xs = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::Suspend { card, x, .. } if card == id => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(xs, vec![1, 2, 3]);

    let suspend_three = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::Suspend { card, x: 3, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, suspend_three).unwrap();
    assert_eq!(
        game.players[0].exile[0]
            .counters
            .count(CounterKind::named("time")),
        3
    );
}

#[test]
fn deep_sea_kraken_triggers_from_exile_for_an_opponents_spell() {
    let mut game = ready_game();
    let mut kraken = card(90_035, definition(&game, "Deep-Sea Kraken"), PlayerId::One);
    kraken.counters.add(CounterKind::named("time"), 2);
    game.players[0].exile.push(kraken);
    let bolt = card(90_036, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_mana(PlayerId::Two, unrestricted(ManaColor::Red, 1));
    game.priority = PlayerId::Two;

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt_id))
        .unwrap();
    game.apply(PlayerId::Two, cast).unwrap();
    drain_pending(&mut game);
    let kraken = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == definition(&game, "Deep-Sea Kraken"))
        .unwrap();
    assert_eq!(kraken.counters.count(CounterKind::named("time")), 1);
}

#[test]
fn epochrasite_dies_into_exile_with_a_granted_suspend_ability() {
    let mut game = ready_game();
    let epoch = creature(90_037, definition(&game, "Epochrasite"), PlayerId::One);
    let id = epoch.card.id;
    game.battlefield.push(epoch);

    game.sacrifice_permanent(id);
    drain_pending(&mut game);

    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == definition(&game, "Epochrasite"))
        .expect("the death trigger exiles Epochrasite");
    assert_eq!(exiled.counters.count(CounterKind::named("time")), 3);
    assert!(game.is_suspended(exiled.id));
}

#[test]
fn arc_blade_flashback_still_resolves_into_suspend_with_time_counters() {
    let mut game = ready_game();
    let blade = card(90_038, definition(&game, "Arc Blade"), PlayerId::One);
    let id = blade.id;
    game.players[0].graveyard.push(blade);
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object: id,
            ability: CARD_COST_FLASHBACK,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        });
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Red, 5));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == id
                    && choices.costs().alternative().is_some()
                    && choices
                        .iter_targets()
                        .copied()
                        .eq([Target::Player(PlayerId::Two)])
            }
            _ => false,
        })
        .expect("Snapcaster-style flashback offers Arc Blade from the graveyard");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == definition(&game, "Arc Blade"))
        .expect("Arc Blade's own destination remains exile under flashback");
    assert_eq!(exiled.counters.count(CounterKind::named("time")), 3);
    assert!(game.is_suspended(exiled.id));
}

#[test]
fn durkwood_baloth_cast_from_suspend_enters_with_haste() {
    let mut game = ready_game();
    let mut baloth = card(90_039, definition(&game, "Durkwood Baloth"), PlayerId::One);
    baloth.counters.add(CounterKind::named("time"), 1);
    let id = baloth.id;
    game.players[0].exile.push(baloth);

    game.remove_counters_from_object(Target::Card(id), CounterKind::named("time"), 1);
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    assert!(
        !game.pending_decisions.is_empty(),
        "the last-counter trigger offers the cast"
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::ChooseDecision { .. })),
        "Suspend requires the cast when the card can be cast"
    );
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("the suspended Baloth may be cast for free");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let baloth = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition(&game, "Durkwood Baloth"))
        .expect("the Baloth resolves as a permanent");
    assert!(game.permanent_has_executable_keyword(baloth, KeywordAbility::Haste));
    game.finish_cleanup();
    let baloth = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition(&game, "Durkwood Baloth"))
        .unwrap();
    assert!(
        game.permanent_has_executable_keyword(baloth, KeywordAbility::Haste),
        "Suspend's haste lasts beyond cleanup until control is lost"
    );
}

#[test]
fn suspend_leaves_an_uncastable_card_in_exile() {
    let mut game = ready_game();
    let mut diffusion = card(
        90_042,
        definition(&game, "Venser's Diffusion"),
        PlayerId::One,
    );
    diffusion.counters.add(CounterKind::named("time"), 1);
    let id = diffusion.id;
    game.players[0].exile.push(diffusion);
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object: id,
            ability: abilities::GRANTED_SUSPEND,
            expiration: ContinuousEffectExpiration::Never,
            source: None,
        });

    game.remove_counters_from_object(Target::Card(id), CounterKind::named("time"), 1);
    drain_pending(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].exile.iter().any(|card| card.id == id));
    assert!(
        !game.stack.iter().any(|object| object.source == Some(id)),
        "a targetless Venser's Diffusion cannot be cast"
    );
}

#[test]
fn timebug_uses_control_for_permanents_and_ownership_for_suspended_cards() {
    let mut game = ready_game();
    let timebug = creature(90_028, definition(&game, "Jhoira's Timebug"), PlayerId::One);
    let timebug_id = timebug.card.id;
    game.battlefield.push(timebug);
    let mut stolen = creature(90_029, cards::SAVANNAH_LIONS, PlayerId::Two);
    stolen.controller = PlayerId::One;
    let stolen_id = stolen.card.id;
    game.battlefield.push(stolen);
    let mut ours = card(90_032, definition(&game, "Rift Bolt"), PlayerId::One);
    ours.counters.add(CounterKind::named("time"), 1);
    let ours_id = ours.id;
    game.players[0].exile.push(ours);
    let mut theirs = card(90_033, definition(&game, "Rift Bolt"), PlayerId::Two);
    theirs.counters.add(CounterKind::named("time"), 1);
    let theirs_id = theirs.id;
    game.players[1].exile.push(theirs);

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == timebug_id => Some(
                targets
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    assert!(targets.contains(&Target::Permanent(stolen_id)));
    assert!(targets.contains(&Target::Card(ours_id)));
    assert!(!targets.contains(&Target::Card(theirs_id)));
}

#[test]
fn timebug_chooses_the_time_counter_operation_on_resolution() {
    let mut game = ready_game();
    let timebug = creature(90_043, definition(&game, "Jhoira's Timebug"), PlayerId::One);
    let timebug_id = timebug.card.id;
    game.battlefield.push(timebug);
    let mut target = creature(90_044, cards::SAVANNAH_LIONS, PlayerId::One);
    let target_id = target.card.id;
    target.add_counters(CounterKind::named("time"), 1);
    target.add_counters(CounterKind::named("charge"), 2);
    game.battlefield.push(target);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == timebug_id
                    && targets
                        .iter()
                        .any(|selection| selection.targets() == [Target::Permanent(target_id)])
            }
            _ => false,
        })
        .expect("Timebug may target the controlled permanent");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .pending_decisions
        .first()
        .expect("Timebug asks on resolution");
    assert!(
        decision
            .observation
            .options
            .iter()
            .any(|option| option.label == "Put another time counter")
    );
    assert!(
        !decision
            .observation
            .options
            .iter()
            .any(|option| option.label.contains("charge"))
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Put another time counter");

    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target_id)
        .unwrap();
    assert_eq!(target.counters(CounterKind::named("time")), 2);
    assert_eq!(target.counters(CounterKind::named("charge")), 2);
}

#[test]
fn jhoira_exiles_the_paid_card_and_grants_suspend_to_its_successor() {
    let mut game = ready_game();
    let jhoira = creature(
        90_030,
        definition(&game, "Jhoira of the Ghitu"),
        PlayerId::One,
    );
    let jhoira_id = jhoira.card.id;
    game.battlefield.push(jhoira);
    let bolt = card(90_031, definition(&game, "Lightning Bolt"), PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_mana(PlayerId::One, unrestricted(ManaColor::Blue, 2));

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, cost_objects, .. }
                    if *source == jhoira_id && cost_objects == &[bolt_id]
            )
        })
        .expect("Jhoira may exile the nonland card");
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == jhoira_id)
            .is_some_and(|permanent| permanent.tapped),
        "Jhoira is tapped as part of the activation cost"
    );
    pass_priority_pair(&mut game);

    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == definition(&game, "Lightning Bolt"))
        .expect("the paid card remains in exile");
    assert_eq!(exiled.counters.count(CounterKind::named("time")), 4);
    let exiled_id = exiled.id;
    assert!(game.is_suspended(exiled_id));
    game.finish_cleanup();
    assert!(
        game.is_suspended(exiled_id),
        "a permanent nonbattlefield ability grant lasts for the card incarnation"
    );
}

#[test]
fn greater_gargadon_activates_from_exile_and_spends_a_permanent() {
    let mut game = ready_game();
    let mut gargadon = card(90_040, definition(&game, "Greater Gargadon"), PlayerId::One);
    gargadon.counters.add(CounterKind::named("time"), 2);
    let gargadon_id = gargadon.id;
    game.players[0].exile.push(gargadon);
    let fodder = creature(90_041, cards::SAVANNAH_LIONS, PlayerId::One);
    let fodder_id = fodder.card.id;
    game.battlefield.push(fodder);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, cost_objects, .. }
                    if *source == gargadon_id && cost_objects == &[fodder_id]
            )
        })
        .expect("the suspended Gargadon offers its exile activation");
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fodder_id)
    );
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[0].exile[0]
            .counters
            .count(CounterKind::named("time")),
        1
    );
}
