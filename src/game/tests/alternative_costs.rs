use super::*;
use crate::AbilityProgramDef;

fn alternative_cast_action(
    game: &Game,
    player: PlayerId,
    source: GameObjectId,
    alternative: AlternativeCostId,
) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == source
                        && choices.costs().alternative() == Some(alternative)
            )
        })
        .expect("the requested alternative cast is legal")
}

fn grant_card_cost_flashback(game: &mut Game, object: GameObjectId) {
    game.nonbattlefield_ability_grants
        .push(NonbattlefieldAbilityGrant {
            object,
            ability: CARD_COST_FLASHBACK,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            source: None,
        });
}

#[test]
fn snapcaster_grants_an_ordinary_card_cost_flashback_ability() {
    let catalog = poc::catalog().unwrap();
    let snapcaster = catalog.get(cards::SNAPCASTER_MAGE).unwrap();
    let trigger = snapcaster.rules.ability(AbilityId(1)).unwrap();
    let AbilityProgramDef::Effects(EffectDef::Apply {
        effect:
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(granted),
            )),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        ..
    }) = trigger.effect.definition
    else {
        panic!("Snapcaster's trigger should use the generic ability-grant effect")
    };
    assert!(matches!(
        granted.definition,
        DeclarativeAbilityDef::AlternativeCast(alternative)
            if alternative.kind == AlternativeCastKindDef::Flashback
                && alternative.mana_cost == AlternativeCastManaCostDef::ThisCardManaCost
    ));
}

#[test]
fn think_twice_can_be_flashed_back_and_is_exiled_after_resolving() {
    let mut game = ready_game();
    let spell = card(20_000, cards::THINK_TWICE, PlayerId::One);
    let graveyard_id = spell.id;
    game.players[0].graveyard.push(spell);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    let library_before = game.players[0].library.len();

    let action = alternative_cast_action(&game, PlayerId::One, graveyard_id, AlternativeCostId(1));
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.players[0].graveyard.is_empty());
    assert!(game.stack.last().unwrap().cast_via_flashback);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), library_before - 1);
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::THINK_TWICE);
}

#[test]
fn flashback_exiles_a_spell_that_is_countered_or_fizzles() {
    // Countered.
    let mut game = ready_game();
    let think_twice = card(20_000, cards::THINK_TWICE, PlayerId::One);
    let counterspell = card(20_001, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].graveyard.push(think_twice.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.players[1].hand.push(counterspell.clone());
    game.players[1].mana_pool.blue = 2;
    let flashback =
        alternative_cast_action(&game, PlayerId::One, think_twice.id, AlternativeCostId(1));
    game.apply(PlayerId::One, flashback).unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let flashed_back_spell = game.stack.last().unwrap().id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(flashed_back_spell)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::THINK_TWICE)
    );

    // All targets illegal on resolution.
    let mut game = ready_game();
    let enchantment = creature(20_010, cards::ENERGY_FLUX, PlayerId::Two);
    let enchantment_id = enchantment.card.id;
    game.battlefield.push(enchantment);
    let ray = card(20_011, cards::RAY_OF_REVELATION, PlayerId::One);
    game.players[0].graveyard.push(ray.clone());
    game.players[0].mana_pool.green = 1;
    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == ray.id
                        && choices.costs().alternative() == Some(AlternativeCostId(1))
                        && choices.iter_targets().copied().eq([Target::Permanent(enchantment_id)])
            )
        })
        .expect("Ray can target the enchantment from the graveyard");
    game.apply(PlayerId::One, flashback).unwrap();
    game.destroy_permanent(enchantment_id);
    pass_priority_pair(&mut game);
    assert!(game
        .events
        .iter()
        .any(|event| matches!(event, GameEvent::SpellFizzled { definition, .. } if *definition == cards::RAY_OF_REVELATION)));
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::RAY_OF_REVELATION)
    );
}

#[test]
fn put_onto_battlefield_keeps_the_object_prospective_during_replacements() {
    let mut game = ready_game();
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::TEMPLE_GARDEN)
        .expect("Temple Garden is in the catalog");

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the setup entry still applies the shock-land replacement");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "the reserved object ID is not public before its replacement finishes"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .unwrap();

    let garden = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the reserved setup identity commits after the choice");
    assert!(garden.tapped);
    assert_eq!(garden.card.definition, cards::TEMPLE_GARDEN);
}

#[test]
fn put_onto_battlefield_runs_the_entry_trigger() {
    // Thragtusk gains 5 life when it enters, so a board set up this way is a
    // real entry rather than a permanent appearing out of nowhere.
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::THRAGTUSK)
        .expect("Thragtusk is in the catalog");
    for _ in 0..6 {
        if game.players[0].life > 20 {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert_eq!(game.players[0].life, 25);
}

#[test]
fn snapcaster_grants_a_second_flashback_cost_until_cleanup() {
    let mut game = ready_game();
    let think_twice = card(20_000, cards::THINK_TWICE, PlayerId::One);
    let graveyard_id = think_twice.id;
    game.players[0].graveyard.push(think_twice);
    let snapcaster = card(20_001, cards::SNAPCASTER_MAGE, PlayerId::One);
    game.players[0].hand.push(snapcaster.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(snapcaster.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Snapcaster asks for its graveyard target");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1, "Snapcaster's ETB is on the stack");
    pass_priority_pair(&mut game);

    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 5;
    let flashback_ids = game
        .legal_actions(PlayerId::One)
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if *card == graveyard_id => {
                choices.costs().alternative()
            }
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(flashback_ids.len(), 2, "printed and granted costs coexist");
    assert!(flashback_ids.contains(&AlternativeCostId(1)));

    game.finish_cleanup();
    let after_cleanup = game
        .legal_actions(PlayerId::One)
        .iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == graveyard_id))
        .count();
    assert_eq!(after_cleanup, 1, "only printed flashback remains");
}

#[test]
fn snapcaster_granted_flashback_uses_the_card_mana_cost_and_exiles_on_resolution() {
    let mut game = ready_game();
    let mortars = card(20_000, cards::MIZZIUM_MORTARS, PlayerId::One);
    let graveyard_id = mortars.id;
    game.players[0].graveyard.push(mortars);
    let snapcaster = card(20_001, cards::SNAPCASTER_MAGE, PlayerId::One);
    game.players[0].hand.push(snapcaster.clone());
    let target = creature(20_002, cards::DESECRATION_DEMON, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(snapcaster.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 1;
    let actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == graveyard_id))
        .collect::<Vec<_>>();
    assert_eq!(
        actions.len(),
        1,
        "the overload alternative is unavailable from the graveyard"
    );
    let Action::CastSpell { choices, .. } = &actions[0] else {
        unreachable!("the filtered action is a cast")
    };
    assert_ne!(
        choices.costs().alternative(),
        Some(AlternativeCostId(1)),
        "the affordable action is Snapcaster's synthetic flashback cost",
    );
    game.apply(PlayerId::One, actions.into_iter().next().unwrap())
        .unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(game.stack.last().unwrap().cast_via_flashback);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .expect("Desecration Demon survives four damage")
            .damage,
        4,
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::MIZZIUM_MORTARS)
    );
}

#[test]
fn snapcaster_flashback_does_not_bypass_a_from_hand_only_fuse_option() {
    let mut game = ready_game();
    let catalog = poc::catalog().unwrap();
    let mut turn_burn = catalog.get(cards::TURN_BURN).unwrap().clone();
    for option in &mut turn_burn.play_options {
        option.effect_status = CardEffectStatus::Implemented;
    }
    let lions = catalog.get(cards::SAVANNAH_LIONS).unwrap().clone();
    game.catalog = CardCatalog::new([turn_burn, lions]).unwrap();
    let split = card(20_000, cards::TURN_BURN, PlayerId::One);
    let split_id = split.id;
    game.players[0].graveyard.push(split);
    grant_card_cost_flashback(&mut game, split_id);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 3;
    let target = creature(20_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);

    let graveyard_options = game
        .legal_actions(PlayerId::One)
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if *card == split_id => {
                Some(choices.play_option())
            }
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    assert!(graveyard_options.contains(&PlayOptionId::DEFAULT));
    assert!(graveyard_options.contains(&PlayOptionId(1)));
    assert!(!graveyard_options.contains(&PlayOptionId(2)));

    let forged_fuse = Action::CastSpell {
        card: split_id,
        choices: CastChoices::new(PlayOptionId(2))
            .with_costs(CostConfiguration::new(
                Some(AlternativeCostId(u8::MAX)),
                Vec::new(),
            ))
            .with_targets(vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(target_id)),
                TargetSelection::single(TargetSlotId(1), Target::Player(PlayerId::Two)),
            ]),
        sacrifices: Vec::new(),
    };
    assert!(!game.is_legal_action(PlayerId::One, &forged_fuse));
}

#[test]
fn snapcaster_granted_recall_offers_x_under_the_flashback_cost() {
    let mut game = ready_game();
    let recall = card(20_000, cards::RECALL, PlayerId::One);
    let recall_id = recall.id;
    game.players[0].graveyard.push(recall);
    grant_card_cost_flashback(&mut game, recall_id);
    game.players[0].hand.extend([
        card(20_001, cards::MOUNTAIN, PlayerId::One),
        card(20_002, cards::MOUNTAIN, PlayerId::One),
    ]);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 4;

    assert!(game.legal_actions(PlayerId::One).iter().any(|action| {
        matches!(
            action,
            Action::CastSpell { card, choices, .. }
                if *card == recall_id
                    && choices.x() == 2
                    && choices.costs().alternative() == Some(AlternativeCostId(u8::MAX))
        )
    }));
}

#[test]
fn snapcaster_flashback_cannot_be_combined_with_overload() {
    let mut game = ready_game();
    let mortars = card(20_000, cards::MIZZIUM_MORTARS, PlayerId::One);
    let mortars_id = mortars.id;
    game.players[0].graveyard.push(mortars);
    grant_card_cost_flashback(&mut game, mortars_id);
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.colorless = 3;
    let target = creature(20_001, cards::SERRA_ANGEL, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);

    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == mortars_id))
        .collect::<Vec<_>>();
    assert_eq!(casts.len(), 1);
    let Action::CastSpell { choices, .. } = &casts[0] else {
        unreachable!("the filtered action is a cast")
    };
    assert_eq!(
        choices.costs().alternative(),
        Some(AlternativeCostId(u8::MAX)),
        "only the synthetic flashback alternative is offered from the graveyard",
    );
    assert_eq!(
        choices.iter_targets().copied().collect::<Vec<_>>(),
        [Target::Permanent(target_id)],
        "flashback retains the ordinary targeted spell rather than the overloaded form",
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn incomplete_alternative_cast_clauses_do_not_enable_or_transform_their_costs() {
    let definition_id = CardDefinitionId::new(20_100);
    let flashback = AlternativeCostId(1);
    let overload = AlternativeCostId(2);
    let targets = Box::leak(
        vec![AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )]
        .into_boxed_slice(),
    );
    let abilities = Box::leak(
        vec![
            AbilityDef::spell_with_targets(
                "Test spell deals 1 damage to target opponent.",
                targets,
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::alternative_cast(
                ManaCost::default(),
                AlternativeCastKindDef::Flashback,
                None,
                EffectDef::None,
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Test-only incomplete flashback.",
            )),
            AbilityDef::alternative_cast(
                ManaCost::default(),
                AlternativeCastKindDef::Overload,
                Some("Test spell deals 1 damage to each opponent."),
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Test-only incomplete overload.",
            )),
        ]
        .into_boxed_slice(),
    );
    let mut definition = CardDefinition::new(
        definition_id,
        "Incomplete Alternatives",
        CardSet::ReturnToRavnica,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_sorcery(ManaCost::new(1, 0)).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let hand = card(20_100, definition_id, PlayerId::One);
    let graveyard = card(20_101, definition_id, PlayerId::One);
    game.players[0].hand.push(hand.clone());
    game.players[0].graveyard.push(graveyard.clone());
    game.players[0].mana_pool.colorless = 1;

    let hand_casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == hand.id))
        .collect::<Vec<_>>();
    assert_eq!(hand_casts.len(), 1, "only the normal cast is offered");
    let Action::CastSpell { choices, .. } = &hand_casts[0] else {
        unreachable!("the filtered action is a cast")
    };
    assert_eq!(choices.costs().alternative(), None);
    assert_eq!(
        choices.iter_targets().copied().collect::<Vec<_>>(),
        [Target::Player(PlayerId::Two)],
    );
    assert!(
        game.legal_actions(PlayerId::One).iter().all(
            |action| !matches!(action, Action::CastSpell { card, .. } if *card == graveyard.id)
        ),
        "incomplete flashback does not grant graveyard casting permission",
    );

    for alternative in [flashback, overload] {
        let forged = Action::CastSpell {
            card: hand.id,
            choices: CastChoices::default()
                .with_costs(CostConfiguration::new(Some(alternative), Vec::new()))
                .with_targets(vec![TargetSelection::single(
                    TargetSlotId(0),
                    Target::Player(PlayerId::Two),
                )]),
            sacrifices: Vec::new(),
        };
        assert!(
            !game.is_legal_action(PlayerId::One, &forged),
            "an incomplete alternative cannot be paid as a generic cost or transform the spell",
        );
    }
}

#[test]
fn unburial_rites_reanimates_its_target_and_exiles_itself() {
    let mut game = ready_game();
    let rites = card(20_000, cards::UNBURIAL_RITES, PlayerId::One);
    let creature_card = card(20_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let creature_id = creature_card.id;
    game.players[0]
        .graveyard
        .extend([rites.clone(), creature_card]);
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 3;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == rites.id
                        && choices.costs().alternative() == Some(AlternativeCostId(1))
                        && choices.iter_targets().copied().eq([Target::Card(creature_id)])
            )
        })
        .expect("Unburial Rites can flash back targeting your creature card");
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::UNBURIAL_RITES)
    );
}

#[test]
fn ghor_clan_rampager_uses_one_shared_bloodrush_effect() {
    let catalog = poc::catalog().unwrap();
    let rampager = catalog.get(cards::GHOR_CLAN_RAMPAGER).unwrap();
    let bloodrush = rampager.rules.ability(AbilityId(1)).unwrap();
    let DeclarativeAbilityDef::Activated(definition) = bloodrush.definition else {
        panic!("Bloodrush should be an activated ability")
    };

    assert_eq!(definition.source_zones, [ZoneKind::Hand]);
    assert_eq!(
        definition.costs.as_slice(),
        [
            AbilityCostDef::Mana(mana_cost!("{R}{G}")),
            AbilityCostDef::DiscardSource,
        ],
    );
    let AbilityProgramDef::Effects(EffectDef::Apply {
        recipient,
        effect: AppliedEffectDef::Composite(components),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    }) = bloodrush.effect.definition
    else {
        panic!("Rampager should apply one composite effect until end of turn")
    };
    assert_eq!(recipient.legal_target(), Some(TargetIndex::PRIMARY));
    assert!(matches!(
        components,
        [
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Modify {
                    power: ValueDef::Constant(4),
                    toughness: ValueDef::Constant(4),
                },
            )),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )),
        ] if ability.definition == DeclarativeAbilityDef::Keyword(KeywordAbility::Trample)
    ));
}

#[test]
fn bloodrush_can_use_mana_restricted_to_activating_creature_abilities() {
    static RESTRICTIONS: [ManaRestrictionDef; 1] = [ManaRestrictionDef::ActivateAbility(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

    let mut game = ready_game();
    let mut attacker = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let rampager = card(20_001, cards::GHOR_CLAN_RAMPAGER, PlayerId::One);
    let rampager_id = rampager.id;
    game.players[0].hand.push(rampager);

    let mana_source = ManaSource {
        object: CardInstanceId(20_002),
        ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
    };
    game.add_mana(
        PlayerId::One,
        [
            Mana::from_ability(ManaColor::Red, mana_source, &RESTRICTIONS, &[]),
            Mana::unrestricted(ManaColor::Red),
            Mana::from_ability(ManaColor::Green, mana_source, &RESTRICTIONS, &[]),
        ],
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == rampager_id
                        && targets.iter().flat_map(TargetSelection::targets).copied()
                            .eq([Target::Permanent(attacker_id)])
            )
        })
        .expect("creature-ability-restricted mana can pay for Bloodrush");

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].mana_pool.green, 0);
    assert_eq!(
        game.players[0].mana,
        vec![Mana::unrestricted(ManaColor::Red)],
        "purpose-aware payment prefers the eligible restricted units",
    );
}

#[test]
fn bloodrush_discards_its_source_and_pumps_an_attacker_until_cleanup() {
    let mut game = ready_game();
    let mut attacker = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let rampager = card(20_001, cards::GHOR_CLAN_RAMPAGER, PlayerId::One);
    let rampager_id = rampager.id;
    game.players[0].hand.push(rampager);
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == rampager_id
                        && targets.iter().flat_map(TargetSelection::targets).copied()
                            .eq([Target::Permanent(attacker_id)])
            )
        })
        .expect("Bloodrush is available from hand");
    game.apply(PlayerId::One, action).unwrap();

    assert!(game.players[0].hand.is_empty());
    assert_eq!(
        game.players[0].graveyard[0].definition,
        cards::GHOR_CLAN_RAMPAGER
    );
    assert_ne!(game.players[0].graveyard[0].id, rampager_id);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardsDiscarded { player: PlayerId::One, cards }
            if cards.iter().any(|(_, definition)| *definition == cards::GHOR_CLAN_RAMPAGER)
    )));
    pass_priority_pair(&mut game);

    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(game.power(attacker), Some(6));
    assert_eq!(game.toughness(attacker), Some(5));
    assert!(game.permanent_has_executable_keyword(attacker, KeywordAbility::Trample));

    game.finish_cleanup();
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(game.power(attacker), Some(2));
    assert_eq!(game.toughness(attacker), Some(1));
    assert!(!game.permanent_has_executable_keyword(attacker, KeywordAbility::Trample));
}

#[test]
fn bloodrush_uses_real_mana_sources_and_tramples_over_a_blocker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = true;
    let mut attacker = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(20_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    let mountain = creature(20_002, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    let forest = creature(20_003, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield
        .extend([attacker, blocker, mountain, forest]);
    game.combat_blocked_attackers.push(attacker_id);
    let rampager = card(20_004, cards::GHOR_CLAN_RAMPAGER, PlayerId::One);
    let rampager_id = rampager.id;
    game.players[0].hand.push(rampager);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == rampager_id
                        && targets.iter().flat_map(TargetSelection::targets).copied()
                            .eq([Target::Permanent(attacker_id)])
            )
        })
        .expect("the lands make Bloodrush payable");
    let mana_sources = game
        .mana_sources_for_action(PlayerId::One, &action)
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(mana_sources, [mountain_id, forest_id].into_iter().collect());
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| [mountain_id, forest_id].contains(&permanent.card.id))
            .all(|permanent| permanent.tapped)
    );
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    take_default_combat_assignment(&mut game);

    assert_eq!(
        game.players[1].life, 15,
        "five damage tramples over the 2/1 blocker"
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id)
    );
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(attacker.damage, 2);
}

#[test]
fn bloodrush_rechecks_that_its_target_is_still_attacking() {
    let mut game = ready_game();
    let mut attacker = creature(20_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let rampager = card(20_001, cards::GHOR_CLAN_RAMPAGER, PlayerId::One);
    let rampager_id = rampager.id;
    game.players[0].hand.push(rampager);
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == rampager_id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let ability_object = game.stack.last().unwrap().id;
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GHOR_CLAN_RAMPAGER)
    );
    game.battlefield[0].attacking = false;
    pass_priority_pair(&mut game);

    let attacker = &game.battlefield[0];
    assert_eq!(attacker.card.id, attacker_id);
    assert_eq!(game.power(attacker), Some(2));
    assert_eq!(game.toughness(attacker), Some(1));
    assert!(!game.permanent_has_executable_keyword(attacker, KeywordAbility::Trample));
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::AbilityFizzled { object, source, presentation }
            if *object == ability_object
                && *source == rampager_id
                && *presentation == ObjectCharacteristics::card(
                    cards::GHOR_CLAN_RAMPAGER,
                    CardPartId::PRIMARY,
                )
    )));
    assert!(!game.events.iter().any(|event| matches!(
        event,
        GameEvent::AbilityResolved { object, .. } if *object == ability_object
    )));
}
