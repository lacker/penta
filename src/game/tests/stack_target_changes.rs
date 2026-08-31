//! Stack target-changing effects and the cards that exercise their distinct
//! target and cost shapes.

use super::*;

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is cataloged"))
}

fn choose_stack_targets(game: &mut Game, wanted: &[Target]) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("a target-change decision is pending");
    let option = match &game
        .pending_decisions
        .first()
        .expect("the target-change continuation is retained")
        .continuation
    {
        DecisionContinuation::ChangeStackTargets { target_lists, .. } => target_lists
            .iter()
            .position(|targets| flatten_target_selections(targets) == wanted)
            .and_then(|index| u32::try_from(index).ok())
            .expect("the requested replacement targets are legal"),
        continuation => panic!("unexpected continuation: {continuation:?}"),
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
}

fn choose_copy_targets(game: &mut Game, wanted: &[Target]) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("a copy-target decision is pending");
    let option = match &game.pending_decisions[0].continuation {
        DecisionContinuation::CopyStackObject { target_lists, .. } => target_lists
            .iter()
            .position(|targets| flatten_target_selections(targets) == wanted)
            .and_then(|index| u32::try_from(index).ok())
            .expect("the requested copy targets are legal"),
        continuation => panic!("unexpected continuation: {continuation:?}"),
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
}

#[test]
fn redirect_changes_the_existing_spells_target() {
    let mut game = ready_game();
    let first = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bolt = card(170_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let redirect = card(170_001, definition(&game, "Redirect"), PlayerId::One);
    game.players[0]
        .hand
        .extend([bolt.clone(), redirect.clone()]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Permanent(first)], Vec::new(), 0),
    )
    .unwrap();
    let bolt_on_stack = game.stack.last().expect("the Bolt is cast").id;
    game.apply(
        PlayerId::One,
        cast_action(
            redirect.id,
            vec![Target::Spell(bolt_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_stack_targets(&mut game, &[Target::Permanent(second)]);

    let bolt = game
        .stack
        .iter()
        .find(|object| object.id == bolt_on_stack)
        .expect("Redirect changes the Bolt in place");
    assert_eq!(bolt.declared_targets(), [Target::Permanent(second)]);

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == first),
        "the former target is untouched",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second),
        "the new target takes the Bolt's damage",
    );
}

#[test]
fn single_target_stack_predicates_reject_zero_and_two_target_spells() {
    let mut game = ready_game();
    let first = GameObjectId(170_010);
    let second = GameObjectId(170_011);
    let targetless = spell(170_012, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let one_target = spell_with_targets(
        170_013,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(first)],
        0,
    );
    let two_targets = spell_with_targets(
        170_014,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(first), Target::Permanent(second)],
        0,
    );
    let (targetless_id, one_target_id, two_targets_id) =
        (targetless.id, one_target.id, two_targets.id);
    game.stack.push(targetless);
    game.stack.push(one_target);
    game.stack.push(two_targets);
    let bend = card(170_015, definition(&game, "Bolt Bend"), PlayerId::One);
    game.players[0].hand.push(bend.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == bend.id)),
        "Bolt Bend still costs four without the qualifying creature",
    );
    game.battlefield
        .push(creature(170_009, cards::SERRA_ANGEL, PlayerId::One));

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == bend.id => {
                choices.iter_targets().copied().next()
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(targets.contains(&Target::Spell(one_target_id)));
    assert!(!targets.contains(&Target::Spell(targetless_id)));
    assert!(!targets.contains(&Target::Spell(two_targets_id)));
}

#[test]
fn misdirection_exiles_another_blue_card_for_its_alternative_cost() {
    let mut game = ready_game();
    let target = GameObjectId(170_016);
    let bolt = spell_with_targets(
        170_017,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(target)],
        0,
    );
    let bolt_id = bolt.id;
    game.stack.push(bolt);
    let misdirection = card(170_018, definition(&game, "Misdirection"), PlayerId::One);
    let blue_card = card(170_019, cards::COUNTERSPELL, PlayerId::One);
    game.players[0]
        .hand
        .extend([misdirection.clone(), blue_card.clone()]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } if *card == misdirection.id
                    && choices.costs().alternative().is_some()
                    && choices.iter_targets().copied().eq([Target::Spell(bolt_id)])
                    && sacrifices == &[blue_card.id]
            )
        })
        .expect("Misdirection can exile the other blue card instead of paying mana");
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::COUNTERSPELL),
        "the exiled card is a new zone object with Counterspell's definition",
    );
}

#[test]
fn ricochet_trap_offers_its_red_cost_after_an_opponent_casts_blue() {
    let mut game = ready_game();
    let blue_spell = card(170_023, cards::ANCESTRAL_RECALL, PlayerId::Two);
    let trap = card(170_024, definition(&game, "Ricochet Trap"), PlayerId::One);
    game.players[1].hand.push(blue_spell.clone());
    game.players[0].hand.push(trap.clone());
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(
            blue_spell.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let blue_spell_id = game.stack.last().expect("the blue spell is cast").id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    assert!(game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(
            action,
            Action::CastSpell { card, choices, .. }
                if card == trap.id
                    && choices.costs().alternative().is_some()
                    && choices.iter_targets().copied().eq([Target::Spell(blue_spell_id)])
        )
    }));
}

#[test]
fn return_the_favor_charges_each_selected_modes_declared_cost() {
    let mut game = ready_game();
    let target = GameObjectId(170_020);
    let bolt = spell_with_targets(
        170_021,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Permanent(target)],
        0,
    );
    game.stack.push(bolt);
    let favor = card(
        170_022,
        definition(&game, "Return the Favor"),
        PlayerId::One,
    );
    game.players[0].hand.push(favor.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let selected_mode_counts = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == favor.id => {
                    Some(choices.modes().len())
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let one_extra = selected_mode_counts(&game);
    assert!(one_extra.contains(&1));
    assert!(!one_extra.contains(&2));

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(selected_mode_counts(&game).contains(&2));
}

#[test]
fn reflecting_mirror_prices_and_changes_a_spell_targeting_you() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mirror = game
        .put_onto_battlefield(PlayerId::One, definition(&game, "Reflecting Mirror"))
        .expect("Reflecting Mirror is cataloged");
    drain_pending(&mut game);

    let aimed_elsewhere = spell_with_targets(
        170_026,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    let aimed_at_you = spell_with_targets(
        170_027,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::One)],
        0,
    );
    let aimed_at_you_id = aimed_at_you.id;
    game.stack.push(aimed_elsewhere);
    game.stack.push(aimed_at_you);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let mirror_targets = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } if source == mirror => flatten_target_selections(&targets).first().copied(),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert!(
        mirror_targets(&game).is_empty(),
        "a mana-value-one spell costs two generic mana to retarget",
    );

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == mirror
                        && flatten_target_selections(targets)
                            == [Target::Spell(aimed_at_you_id)]
            )
        })
        .expect("only the single-target spell aimed at you is offered");
    game.apply(PlayerId::One, activation)
        .expect("the target-priced activation is payable");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mirror)
            .expect("Reflecting Mirror remains on the battlefield")
            .tapped,
        "the Mirror taps as part of the activation cost",
    );

    pass_priority_pair(&mut game);
    choose_stack_targets(&mut game, &[Target::Player(PlayerId::Two)]);
    assert_eq!(
        game.stack
            .iter()
            .find(|object| object.id == aimed_at_you_id)
            .expect("the Bolt remains on the stack")
            .declared_targets(),
        [Target::Player(PlayerId::Two)],
    );
}

#[test]
fn counterspells_still_reject_abilities_on_the_shared_stack_target_path() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let sorcerer = game
        .put_onto_battlefield(PlayerId::One, cards::PRODIGAL_SORCERER)
        .expect("cataloged");
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sorcerer)
        .expect("the Sorcerer is present")
        .entered_controller_turn = 0;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: sorcerer,
            ability: activated_ability_for(&game, sorcerer, 0),
            targets: activated_targets(Target::Permanent(target)),
            cost_objects: Vec::new(),
            mana_payment: None,
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    let ability = game.stack.last().expect("the ping is activated").id;
    let counterspell = card(170_029, cards::COUNTERSPELL, PlayerId::One);
    game.players[0].hand.push(counterspell.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);

    assert!(matches!(
        game.apply(
            PlayerId::One,
            cast_action(counterspell.id, vec![Target::Spell(ability)], Vec::new(), 0,),
        ),
        Err(ActionError::NotLegal { .. })
    ));
}

#[test]
fn venser_flashes_in_and_returns_a_spell_to_its_owners_hand() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let bolt = spell_with_targets(
        170_050,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::One)],
        0,
    );
    let bolt_on_stack = bolt.id;
    game.stack.push(bolt);
    let venser_definition = definition(&game, "Venser, Shaper Savant");
    let venser = card(170_051, venser_definition, PlayerId::One);
    let venser_id = venser.id;
    game.players[0].hand.push(venser);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);
    game.priority = PlayerId::One;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == venser_id))
        .expect("flash permits Venser to be cast over another spell");
    game.apply(PlayerId::One, cast).expect("Venser is cast");
    pass_priority_pair(&mut game);

    let pending = game
        .pending_decisions
        .first()
        .expect("Venser's enter trigger asks for a target");
    let target_index = match &pending.continuation {
        DecisionContinuation::TriggerPlacement { candidates, .. } => candidates
            .iter()
            .position(|candidate| *candidate == Target::Spell(bolt_on_stack))
            .expect("the spell is an offered target"),
        other => panic!("expected trigger placement, found {other:?}"),
    };
    let decision = pending.observation.clone();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[target_index].id],
        },
    )
    .expect("the spell is chosen");
    pass_priority_pair(&mut game);

    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the spell returned to its owner's hand instead of resolving",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == venser_definition),
        "Venser remains on the battlefield",
    );
}

#[test]
fn reroute_changes_an_activated_ability_target_then_draws() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let sorcerer = game
        .put_onto_battlefield(PlayerId::One, cards::PRODIGAL_SORCERER)
        .expect("cataloged");
    let first = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == sorcerer)
        .expect("the Sorcerer is present")
        .entered_controller_turn = 0;

    let ping = Action::ActivateAbility {
        source: sorcerer,
        ability: activated_ability_for(&game, sorcerer, 0),
        targets: activated_targets(Target::Permanent(first)),
        cost_objects: Vec::new(),
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    game.apply(PlayerId::One, ping).unwrap();
    let ping_on_stack = game.stack.last().expect("the ping is activated").id;
    let reroute = card(170_030, definition(&game, "Reroute"), PlayerId::One);
    game.players[0].hand.push(reroute.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.apply(
        PlayerId::One,
        cast_action(
            reroute.id,
            vec![Target::Spell(ping_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_stack_targets(&mut game, &[Target::Permanent(second)]);

    assert_eq!(
        game.players[0].hand.len(),
        1,
        "Reroute draws after changing the target",
    );
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == first)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second)
    );
}

#[test]
fn spellskite_replaces_one_target_with_itself_when_legal() {
    let mut game = ready_game();
    let spellskite = game
        .put_onto_battlefield(PlayerId::One, definition(&game, "Spellskite"))
        .expect("cataloged");
    let protected = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bolt = card(170_035, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Permanent(protected)], Vec::new(), 0),
    )
    .unwrap();
    let bolt_on_stack = game.stack.last().expect("the Bolt is cast").id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == spellskite
                        && flatten_target_selections(targets) == [Target::Spell(bolt_on_stack)]
            )
        })
        .expect("two life can pay for Spellskite's Phyrexian blue activation");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);
    choose_stack_targets(&mut game, &[Target::Permanent(spellskite)]);
    pass_until_decision(&mut game);

    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == protected)
    );
    let spellskite = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == spellskite)
        .expect("three damage does not destroy the 0/4");
    assert_eq!(spellskite.damage, 3);
}

#[test]
fn wild_ricochet_retargets_the_original_then_the_copy_independently() {
    let mut game = ready_game();
    let first = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);
    let bolt = card(170_040, cards::LIGHTNING_BOLT, PlayerId::One);
    let ricochet = card(170_041, definition(&game, "Wild Ricochet"), PlayerId::One);
    game.players[0]
        .hand
        .extend([bolt.clone(), ricochet.clone()]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Permanent(first)], Vec::new(), 0),
    )
    .unwrap();
    let original = game.stack.last().expect("the Bolt is cast").id;
    game.apply(
        PlayerId::One,
        cast_action(ricochet.id, vec![Target::Spell(original)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_stack_targets(&mut game, &[Target::Permanent(second)]);
    choose_copy_targets(&mut game, &[Target::Permanent(first)]);

    let bolts = game
        .stack
        .iter()
        .filter(|object| object.card.definition == cards::LIGHTNING_BOLT)
        .map(|object| (object.is_copy, object.declared_targets()))
        .collect::<Vec<_>>();
    assert!(bolts.contains(&(false, vec![Target::Permanent(second)])));
    assert!(bolts.contains(&(true, vec![Target::Permanent(first)])));

    pass_until_decision(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| { permanent.card.id != first && permanent.card.id != second }),
        "survivors: {:?}",
        game.battlefield
            .iter()
            .map(|permanent| (permanent.card.id, permanent.damage))
            .collect::<Vec<_>>(),
    );
}
