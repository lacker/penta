use super::*;

fn acceptance_play_cavern_choosing(game: &mut Game, creature_type: &str) -> GameObjectId {
    let cavern = card(19_000, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0].hand.push(cavern.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::CAVERN_OF_SOULS),
        "an as-enters choice finishes before Cavern is on the battlefield",
    );
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Cavern asks for a creature type");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == creature_type)
        .unwrap_or_else(|| panic!("{creature_type} is an available creature type"))
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
    let permanent = game
        .observe(PlayerId::One)
        .battlefield
        .into_iter()
        .find(|candidate| {
            candidate.characteristics.card_definition() == Some(cards::CAVERN_OF_SOULS)
        })
        .expect("Cavern entered after its creature type was chosen");
    assert_eq!(
        permanent.chosen_creature_type.as_deref(),
        Some(creature_type)
    );
    permanent.id
}

#[test]
fn cavern_of_souls_requires_and_records_a_creature_type_choice() {
    let mut game = ready_game();
    let cavern = card(19_080, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0].hand.push(cavern.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry replacement creates a mandatory choice");
    assert_eq!(decision.prompt, "Choose a creature type");
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Angel")
    );
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Time Lord"),
        "Cavern can name a legal type absent from the loaded card catalog",
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label != "Artifact")
    );
    assert!(
        game.observe(PlayerId::One).battlefield.is_empty(),
        "the replacement choice is made before Cavern enters",
    );
    let angel = decision
        .options
        .iter()
        .find(|option| option.label == "Angel")
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel],
        },
    )
    .unwrap();
    let observed = game.observe(PlayerId::Two);
    let cavern = observed
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.characteristics.card_definition() == Some(cards::CAVERN_OF_SOULS)
        })
        .expect("Cavern enters after the choice");
    assert_eq!(cavern.chosen_creature_type.as_deref(), Some("Angel"));
}

#[test]
fn cavern_choices_ignore_noncreature_subtypes_on_creature_cards() {
    let definition_id = CardDefinitionId(19_085);
    let mut equipment_creature = CardDefinition::new(
        definition_id,
        "Test equipment creature",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    equipment_creature.rules =
        CardRules::new_artifact_creature(ManaCost::new(1, 0), &["Equipment", "Rabbit"], 1, 1);
    synchronize_single_part_definition(&mut equipment_creature);

    let mut game = ready_game();
    let cavern_definition = game
        .catalog
        .get(cards::CAVERN_OF_SOULS)
        .expect("the acceptance catalog contains Cavern")
        .clone();
    game.catalog = CardCatalog::new([cavern_definition, equipment_creature]).unwrap();
    let cavern = card(19_086, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0]
        .hand
        .extend([cavern.clone(), card(19_087, definition_id, PlayerId::One)]);

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Cavern asks for a creature type");
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Rabbit")
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label != "Equipment"),
        "artifact subtypes do not become legal creature-type choices",
    );
}

#[test]
fn cavern_colored_mana_cannot_pay_for_a_nonmatching_creature() {
    let mut game = ready_game();
    let creature = card(19_090, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();

    assert!(
        game.legal_actions(PlayerId::One).iter().all(
            |action| !matches!(action, Action::CastSpell { card, .. } if *card == creature.id)
        )
    );
    assert!(
        game.apply(
            PlayerId::One,
            cast_action(creature.id, Vec::new(), Vec::new(), 0),
        )
        .is_err(),
        "restricted mana cannot be forced through validation",
    );
}

#[test]
fn cavern_mana_spent_on_a_matching_creature_makes_it_uncounterable() {
    let mut game = ready_game();
    let creature = card(19_100, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    let counterspell = card(19_101, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(creature.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );
    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert_eq!(game.stack.len(), 1, "the Cavern-paid creature remains");
    pass_priority_pair(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.card.definition == cards::ICATIAN_JAVELINEERS
            && permanent.controller == PlayerId::One
    }));
}

#[test]
fn generic_payment_prefers_eligible_cavern_mana_with_a_spell_rider() {
    let mut game = ready_game();
    let angel = card(19_105, cards::RESTORATION_ANGEL, PlayerId::One);
    game.players[0].hand.push(angel.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::Blue);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::Blue,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, angel.id);
    game.apply(PlayerId::One, action).unwrap();

    let spell = game
        .stack
        .last()
        .expect("Restoration Angel is on the stack");
    assert!(spell.applied_effects.iter().any(|effect| {
        effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
            && effect.source.is_some_and(|source| source.object == cavern)
    }));
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert_eq!(game.players[0].mana_pool.colorless, 1);
}

#[test]
fn cavern_mana_keeps_its_chosen_type_and_rider_after_cavern_leaves() {
    let mut game = ready_game();
    let creature = card(19_105, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();
    game.destroy_permanent_without_regeneration(cavern);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );
}

#[test]
fn automatic_payment_uses_cavern_when_its_rider_benefits_the_spell() {
    let mut game = ready_game();
    let goblin = card(19_106, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let counterspell = card(19_107, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(goblin.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Goblin");
    game.battlefield
        .push(creature(19_108, cards::MOUNTAIN, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, goblin.id);
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == cavern)
            .is_some_and(|permanent| permanent.tapped),
        "the automatic payment plan taps Cavern for the beneficial rider",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.card.definition == cards::MOUNTAIN && permanent.tapped
            })
            .count(),
        0,
    );
    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );

    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert_eq!(game.stack.len(), 1, "the Cavern-paid Goblin remains");
}

#[test]
fn cavern_does_not_protect_a_creature_paid_for_with_other_mana() {
    for chosen_type in ["Angel", "Soldier"] {
        let mut game = ready_game();
        let creature = card(19_110, cards::ICATIAN_JAVELINEERS, PlayerId::One);
        let counterspell = card(19_111, cards::COUNTERSPELL, PlayerId::Two);
        game.players[0].hand.push(creature.clone());
        game.players[1].hand.push(counterspell.clone());
        let _cavern = acceptance_play_cavern_choosing(&mut game, chosen_type);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

        let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
        game.apply(PlayerId::One, action).unwrap();
        assert!(game.stack.last().unwrap().applied_effects.is_empty());
        acceptance_attempt_counterspell(&mut game, counterspell.id);
        assert!(game.stack.is_empty(), "Counterspell counters the creature");
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| { permanent.card.definition != cards::ICATIAN_JAVELINEERS })
        );
    }
}

#[test]
fn caverns_colorless_mana_is_unrestricted_and_has_no_countering_rider() {
    let mut game = ready_game();
    let sol_ring = card(19_120, cards::SOL_RING, PlayerId::One);
    let counterspell = card(19_121, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(sol_ring.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, sol_ring.id);
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.stack.last().unwrap().applied_effects.is_empty());
    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
    );
}
