mod cavern_of_souls;

use super::*;
use crate::AbilityProgramDef;

fn acceptance_cast_action_for_card(game: &Game, player: PlayerId, spell: GameObjectId) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the acceptance spell has a legal cast action")
}

pub(super) fn acceptance_cast_action_targeting(
    game: &Game,
    player: PlayerId,
    spell: GameObjectId,
    target: Target,
) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell
                        && choices.iter_targets().copied().eq(std::iter::once(target))
            )
        })
        .expect("the acceptance spell can legally target the requested object")
}

pub(super) fn acceptance_attempt_counterspell(game: &mut Game, counterspell: GameObjectId) {
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let target = game
        .stack
        .last()
        .expect("a spell is waiting to be countered")
        .id;
    game.apply(
        PlayerId::Two,
        cast_action(counterspell, vec![Target::Spell(target)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(game);
}

#[test]
fn countering_acceptance_cards_report_complete_shared_implementations() {
    let game = ready_game();
    for definition in [
        cards::RED_ELEMENTAL_BLAST,
        cards::BLUE_ELEMENTAL_BLAST,
        cards::ABRUPT_DECAY,
        cards::LOXODON_SMITER,
        cards::CAVERN_OF_SOULS,
    ] {
        let definition = game.catalog.get(definition).unwrap();
        assert_eq!(
            definition.implementation_status(),
            crate::ImplementationStatus::Complete,
            "{} should be complete",
            definition.name,
        );
    }

    for definition in [cards::RED_ELEMENTAL_BLAST, cards::BLUE_ELEMENTAL_BLAST] {
        let definition = game.catalog.get(definition).unwrap();
        assert_eq!(definition.rules.special_behavior(), None);
        assert!(
            definition
                .rules
                .ability_clauses()
                .iter()
                .all(|ability| ability.declarative_effect().is_some()),
            "{} should not use a card-local resolver",
            definition.name,
        );
        let modal = definition
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.modal(),
                DeclarativeAbilityDef::Pregame(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
            .expect("an Elemental Blast has declarative modes");
        assert_eq!((modal.minimum, modal.maximum), (1, 1));
        assert!(!modal.may_repeat);
        assert_eq!(modal.modes.len(), 2);
        assert!(modal.modes.iter().all(|mode| {
            mode.declarative_effect().is_some()
                && matches!(mode.definition, DeclarativeAbilityDef::Spell(spell) if spell.modal().is_none())
        }));
        assert!(modal.modes.iter().any(|mode| matches!(
            mode.effect.definition,
            AbilityProgramDef::Effects(EffectDef::Counter { .. })
        )));
        assert!(modal.modes.iter().any(|mode| matches!(
            mode.effect.definition,
            AbilityProgramDef::Effects(EffectDef::Destroy { .. })
        )));
    }
}

#[test]
fn elemental_blast_modes_offer_only_the_matching_color_and_zone() {
    let mut game = ready_game();
    let blue_spell = spell(19_001, cards::PSIONIC_BLAST, PlayerId::Two, 0);
    let red_spell = spell(19_002, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let blue_spell_id = blue_spell.id;
    let red_spell_id = red_spell.id;
    game.stack.push(blue_spell);
    game.stack.push(red_spell);
    let blue_permanent = creature(19_003, cards::SERENDIB_EFREET, PlayerId::Two);
    let red_permanent = creature(19_004, cards::ATOG, PlayerId::Two);
    let blue_permanent_id = blue_permanent.card.id;
    let red_permanent_id = red_permanent.card.id;
    game.battlefield.extend([blue_permanent, red_permanent]);
    let red_blast = card(19_005, cards::RED_ELEMENTAL_BLAST, PlayerId::One);
    let blue_blast = card(19_006, cards::BLUE_ELEMENTAL_BLAST, PlayerId::One);
    game.players[0]
        .hand
        .extend([red_blast.clone(), blue_blast.clone()]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    let offered_targets = |definition| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == definition => {
                    assert_eq!(choices.modes().len(), 1, "one mode is locked in");
                    let targets = choices.iter_targets().copied().collect::<Vec<_>>();
                    assert_eq!(targets.len(), 1);
                    Some(targets[0])
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    let red_targets = offered_targets(red_blast.id);
    assert_eq!(red_targets.len(), 2);
    assert!(red_targets.contains(&Target::Spell(blue_spell_id)));
    assert!(red_targets.contains(&Target::Permanent(blue_permanent_id)));
    assert!(!red_targets.contains(&Target::Spell(red_spell_id)));
    assert!(!red_targets.contains(&Target::Permanent(red_permanent_id)));

    let blue_targets = offered_targets(blue_blast.id);
    assert_eq!(blue_targets.len(), 2);
    assert!(blue_targets.contains(&Target::Spell(red_spell_id)));
    assert!(blue_targets.contains(&Target::Permanent(red_permanent_id)));
    assert!(!blue_targets.contains(&Target::Spell(blue_spell_id)));
    assert!(!blue_targets.contains(&Target::Permanent(blue_permanent_id)));
}

fn assert_elemental_blast_counters_and_destroys(
    blast_definition: CardDefinitionId,
    mana: ManaColor,
    spell_definition: CardDefinitionId,
    permanent_definition: CardDefinitionId,
) {
    let mut game = ready_game();
    let target = spell(19_010, spell_definition, PlayerId::Two, 0);
    let target_id = target.id;
    game.stack.push(target);
    let blast = card(19_011, blast_definition, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.add_unrestricted_mana(PlayerId::One, mana, 1);
    let action =
        acceptance_cast_action_targeting(&game, PlayerId::One, blast.id, Target::Spell(target_id));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == spell_definition),
        "the matching spell was countered",
    );

    let mut game = ready_game();
    let target = creature(19_012, permanent_definition, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let blast = card(19_013, blast_definition, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.add_unrestricted_mana(PlayerId::One, mana, 1);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        blast.id,
        Target::Permanent(target_id),
    );
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != target_id),
        "the matching permanent was destroyed",
    );
}

#[test]
fn elemental_blast_modes_counter_spells_and_destroy_permanents() {
    assert_elemental_blast_counters_and_destroys(
        cards::RED_ELEMENTAL_BLAST,
        ManaColor::Red,
        cards::PSIONIC_BLAST,
        cards::SERENDIB_EFREET,
    );
    assert_elemental_blast_counters_and_destroys(
        cards::BLUE_ELEMENTAL_BLAST,
        ManaColor::Blue,
        cards::LIGHTNING_BOLT,
        cards::ATOG,
    );
}

#[test]
fn fork_retargets_an_elemental_blast_without_changing_its_mode() {
    let mut game = ready_game();
    let first = spell(19_020, cards::PSIONIC_BLAST, PlayerId::Two, 0);
    let second = spell(19_021, cards::COUNTERSPELL, PlayerId::Two, 0);
    let first_id = first.id;
    let second_id = second.id;
    game.stack.push(first);
    game.stack.push(second);
    let blue_permanent = creature(19_022, cards::SERENDIB_EFREET, PlayerId::Two);
    let blue_permanent_id = blue_permanent.card.id;
    game.battlefield.push(blue_permanent);
    let blast = card(19_023, cards::RED_ELEMENTAL_BLAST, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let action =
        acceptance_cast_action_targeting(&game, PlayerId::One, blast.id, Target::Spell(first_id));
    game.apply(PlayerId::One, action).unwrap();
    let original = game
        .stack
        .last()
        .expect("the Blast is on the stack")
        .clone();

    let replacement_choices = game.copy_target_choices(&original, PlayerId::One);
    assert!(replacement_choices.iter().all(|selections| {
        selections
            .iter()
            .flat_map(TargetSelection::targets)
            .all(|target| matches!(target, Target::Spell(_)))
    }));
    assert!(replacement_choices.iter().all(|selections| {
        !selections
            .iter()
            .flat_map(TargetSelection::targets)
            .any(|target| *target == Target::Permanent(blue_permanent_id))
    }));
    let replacement = replacement_choices
        .into_iter()
        .find(|selections| {
            selections
                .iter()
                .flat_map(TargetSelection::targets)
                .any(|target| *target == Target::Spell(second_id))
        })
        .expect("the counter mode may retarget another blue spell");
    game.push_copy(original.clone(), PlayerId::One, replacement);

    let copied = game.stack.last().expect("Fork's copy is on the stack");
    assert_eq!(
        copied.signature.as_ref().map(CastSignature::modes),
        original.signature.as_ref().map(CastSignature::modes),
    );
    assert_eq!(
        copied
            .ability
            .as_ref()
            .map(|ability| ability.mode_effects.as_slice()),
        original
            .ability
            .as_ref()
            .map(|ability| ability.mode_effects.as_slice()),
    );

    pass_priority_pair(&mut game);
    assert!(game.stack.iter().all(|object| object.id != second_id));
    assert!(game.stack.iter().any(|object| object.id == first_id));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == blue_permanent_id),
        "the locked counter mode cannot become the destroy mode",
    );
}

#[test]
fn abrupt_decay_targets_only_nonlands_with_mana_value_three_or_less() {
    let mut game = ready_game();
    let one = creature(19_030, cards::SOL_RING, PlayerId::Two);
    let three = creature(19_031, cards::SEDGE_TROLL, PlayerId::Two);
    let five = creature(19_032, cards::SERRA_ANGEL, PlayerId::Two);
    let land = creature(19_033, cards::MOUNTAIN, PlayerId::Two);
    let mut transformed_four = creature(19_034, cards::HUNTMASTER_OF_THE_FELLS, PlayerId::Two);
    transformed_four.presented = CardPartId(1);
    let one_id = one.card.id;
    let three_id = three.card.id;
    let five_id = five.card.id;
    let land_id = land.card.id;
    let transformed_four_id = transformed_four.card.id;
    assert_eq!(game.permanent_mana_value(&transformed_four), 4);
    game.battlefield
        .extend([one, three, five, land, transformed_four]);
    let decay = card(19_035, cards::ABRUPT_DECAY, PlayerId::One);
    game.players[0].hand.push(decay.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == decay.id => {
                choices.iter_targets().next().copied()
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&Target::Permanent(one_id)));
    assert!(targets.contains(&Target::Permanent(three_id)));
    assert!(!targets.contains(&Target::Permanent(five_id)));
    assert!(!targets.contains(&Target::Permanent(land_id)));
    assert!(!targets.contains(&Target::Permanent(transformed_four_id)));

    let action =
        acceptance_cast_action_targeting(&game, PlayerId::One, decay.id, Target::Permanent(one_id));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != one_id),
    );
}

#[test]
fn counterspell_can_target_abrupt_decay_but_cannot_stop_it() {
    let mut game = ready_game();
    let target = creature(19_040, cards::SOL_RING, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let decay = card(19_041, cards::ABRUPT_DECAY, PlayerId::One);
    let counterspell = card(19_042, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(decay.clone());
    game.players[1].hand.push(counterspell.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        decay.id,
        Target::Permanent(target_id),
    );
    game.apply(PlayerId::One, action).unwrap();
    assert!(!game.observe(PlayerId::Two).stack[0].counterable);
    acceptance_attempt_counterspell(&mut game, counterspell.id);

    assert_eq!(game.stack.len(), 1, "Abrupt Decay remains on the stack");
    assert_eq!(game.stack[0].card.definition, cards::ABRUPT_DECAY);
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != target_id),
        "Abrupt Decay still resolves",
    );
}

#[test]
fn a_failed_mana_drain_still_records_abrupt_decays_mana_value() {
    let mut game = ready_game();
    let target = creature(19_043, cards::SOL_RING, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let decay = card(19_044, cards::ABRUPT_DECAY, PlayerId::One);
    let mana_drain = card(19_045, cards::MANA_DRAIN, PlayerId::Two);
    game.players[0].hand.push(decay.clone());
    game.players[1].hand.push(mana_drain.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        decay.id,
        Target::Permanent(target_id),
    );
    game.apply(PlayerId::One, action).unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let decay_on_stack = game.stack.last().unwrap().id;
    game.apply(
        PlayerId::Two,
        cast_action(
            mana_drain.id,
            vec![Target::Spell(decay_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.stack.len(), 1, "Abrupt Decay was not countered");
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != target_id),
        "Mana Drain's other effect does not stop Abrupt Decay resolving",
    );
}

#[test]
fn counterspell_can_target_loxodon_smiter_but_the_smiter_resolves() {
    let mut game = ready_game();
    let smiter = card(19_050, cards::LOXODON_SMITER, PlayerId::One);
    let counterspell = card(19_051, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(smiter.clone());
    game.players[1].hand.push(counterspell.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, smiter.id);
    game.apply(PlayerId::One, action).unwrap();
    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert_eq!(game.stack.len(), 1, "the Smiter was not countered");
    pass_priority_pair(&mut game);

    let smiter = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LOXODON_SMITER)
        .expect("Loxodon Smiter resolved");
    assert_eq!(game.power(smiter), Some(4));
    assert_eq!(game.toughness(smiter), Some(4));
}

#[test]
fn loxodon_smiter_replaces_an_opponent_caused_hand_to_graveyard_move() {
    let game = ready_game();
    let smiter = game.catalog.get(cards::LOXODON_SMITER).unwrap();
    let replacement = smiter
        .rules
        .ability_clauses()
        .iter()
        .find_map(|ability| match ability.definition {
            DeclarativeAbilityDef::Replacement(replacement) => Some((ability, replacement)),
            _ => None,
        })
        .expect("Loxodon Smiter has a replacement ability");

    assert_eq!(replacement.1.source_zones, [ZoneKind::Hand]);
    assert_eq!(
        replacement.1.event,
        ReplacementEventDef::WouldMove {
            from: Some(ZoneKind::Hand),
            to: ZoneKind::Graveyard,
            cause: ZoneMoveCauseDef::EffectControlledBy(PlayerRelation::Opponent),
        }
    );
    assert_eq!(
        replacement.0.effect.definition,
        AbilityProgramDef::Replacement(ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield,))
    );
}

#[test]
fn loxodon_smiter_zone_move_replacement_checks_the_cause_controller() {
    for (cause, enters) in [
        (
            ZoneMoveCause::Effect {
                controller: PlayerId::Two,
            },
            true,
        ),
        (
            ZoneMoveCause::Effect {
                controller: PlayerId::One,
            },
            false,
        ),
        (ZoneMoveCause::Rules, false),
    ] {
        let mut game = ready_game();
        let smiter = card(19_060, cards::LOXODON_SMITER, PlayerId::One);
        game.players[0].hand.push(smiter.clone());
        game.discard_cards_with_cause(PlayerId::One, &[smiter.id], cause);

        assert_eq!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::LOXODON_SMITER),
            enters,
        );
        assert_eq!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::LOXODON_SMITER),
            !enters,
        );
        assert!(
            game.events.iter().any(|event| {
                matches!(
                    event,
                    GameEvent::CardsDiscarded { player: PlayerId::One, cards }
                        if cards.iter().any(|(_, definition)| *definition == cards::LOXODON_SMITER)
                )
            }),
            "the replacement changes the destination, not whether it was discarded"
        );
    }
}

#[test]
fn general_effect_zone_moves_consult_would_move_replacements() {
    let mut game = ready_game();
    let smiter = card(19_061, cards::LOXODON_SMITER, PlayerId::One);
    game.players[0].hand.push(smiter.clone());

    game.move_target_to_zone(
        Target::Card(smiter.id),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::Two,
        },
        None,
        ZonePlacement::Top,
    );

    assert!(game.players[0].graveyard.is_empty());
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::One && permanent.card.definition == cards::LOXODON_SMITER
    }));
}

#[test]
fn a_smiters_replaced_discard_still_runs_battlefield_entry_replacements() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(19_062, cards::BLIND_OBEDIENCE, PlayerId::One));
    let smiter = card(19_063, cards::LOXODON_SMITER, PlayerId::Two);
    game.players[1].hand.push(smiter.clone());

    game.discard_cards_with_cause(
        PlayerId::Two,
        &[smiter.id],
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
    );

    let smiter = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LOXODON_SMITER)
        .expect("the replaced discard became a battlefield entry");
    assert!(
        smiter.tapped,
        "Blind Obedience modifies the replacement's battlefield destination"
    );
}

#[test]
fn opponent_spells_and_abilities_put_a_discarded_smiter_onto_the_battlefield() {
    let mut game = ready_game();
    let hymn = card(19_070, cards::HYMN_TO_TOURACH, PlayerId::One);
    let smiter = card(19_071, cards::LOXODON_SMITER, PlayerId::Two);
    game.players[0].hand.push(hymn.clone());
    game.players[1].hand.push(smiter);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        hymn.id,
        Target::Player(PlayerId::Two),
    );
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::Two && permanent.card.definition == cards::LOXODON_SMITER
    }));
    assert!(game.players[1].graveyard.is_empty());

    let mut game = ready_game();
    let mut specter = creature(19_072, cards::HYPNOTIC_SPECTER, PlayerId::One);
    specter.attacking = true;
    game.battlefield.push(specter);
    game.players[1]
        .hand
        .push(card(19_073, cards::LOXODON_SMITER, PlayerId::Two));
    game.deal_combat_damage();
    drain_pending(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::Two && permanent.card.definition == cards::LOXODON_SMITER
    }));
    assert!(game.players[1].graveyard.is_empty());
}
