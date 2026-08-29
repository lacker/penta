use super::*;

#[test]
fn earthquake_checks_flying_on_resolution() {
    let mut game = ready_game();
    let earthquake = card(10_000, cards::EARTHQUAKE, PlayerId::One);
    let ground_creature = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let granted_flying = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let printed_flying = creature(10_003, cards::SERRA_ANGEL, PlayerId::Two);
    let noncreature = creature(10_004, cards::SOL_RING, PlayerId::Two);
    let doomed_creature = creature(10_005, cards::SAVANNAH_LIONS, PlayerId::Two);
    game.players[0].hand.push(earthquake.clone());
    game.players[0].mana_pool.red = 4;
    game.battlefield.extend([
        ground_creature,
        granted_flying,
        printed_flying,
        noncreature,
        doomed_creature,
    ]);

    let cast = cast_action(earthquake.id, Vec::new(), Vec::new(), 3);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == CardInstanceId(10_002))
        .unwrap()
        .temporary_keywords
        .push(KeywordAbility::Flying);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 17);
    assert_eq!(game.players[1].life, 17);
    let damage = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| permanent.damage)
    };
    assert_eq!(damage(10_001), Some(3), "a ground creature takes X");
    assert_eq!(
        damage(10_002),
        Some(0),
        "flying gained before resolution excludes the creature"
    );
    assert_eq!(damage(10_003), Some(0), "a printed flier is excluded");
    assert_eq!(damage(10_004), Some(0), "a noncreature is excluded");
    assert_eq!(
        damage(10_005),
        None,
        "lethal damage destroys a ground creature"
    );
}

#[test]
fn earthquake_that_is_lethal_to_each_player_draws_the_game() {
    let mut game = ready_game();
    let earthquake = card(10_000, cards::EARTHQUAKE, PlayerId::One);
    game.players[0].hand.push(earthquake.clone());
    game.players[0].mana_pool.red = 4;
    game.players[0].life = 3;
    game.players[1].life = 3;

    game.apply(
        PlayerId::One,
        cast_action(earthquake.id, Vec::new(), Vec::new(), 3),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.result, Some(GameResult::Draw));
}

#[test]
fn fireball_pays_for_multiple_targets_and_divides_x_evenly() {
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    let creature = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let creature_id = creature.card.id;
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;
    game.battlefield.push(creature);

    let action = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(creature_id),
        ],
        Vec::new(),
        4,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature_id)
            .unwrap()
            .damage,
        2
    );
}

#[test]
fn channel_pays_for_a_fireball_in_one_cast() {
    // The reason Channel is a card: the life is spendable while paying for
    // the spell, so the X the engine offers has to count it.
    let mut game = ready_game();
    resolve_channel(&mut game);
    game.battlefield
        .push(creature(10_000, cards::MOUNTAIN, PlayerId::One));
    let fireball = card(10_001, cards::FIREBALL, PlayerId::One);
    game.players[0].hand.push(fireball.clone());

    let biggest = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. }
                if card == fireball.id
                    && choices
                        .iter_targets()
                        .copied()
                        .eq(std::iter::once(Target::Player(PlayerId::Two))) =>
            {
                Some(choices.x())
            }
            _ => None,
        })
        .max()
        .expect("Fireball can be cast");
    assert_eq!(biggest, 20, "Channel may spend the last point of life");

    game.apply(
        PlayerId::One,
        cast_action(
            fireball.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            12,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 8, "twelve life became twelve mana");
    assert_eq!(game.players[1].life, 8, "and all twelve landed");
}

#[test]
fn channel_and_pay_life_mana_share_life_left_after_the_spell_cost() {
    let mut game = ready_game();
    game.battlefield.clear();
    resolve_channel(&mut game);
    let confluence = game
        .put_onto_battlefield(PlayerId::One, cards::MANA_CONFLUENCE)
        .expect("cataloged");
    let deluge = card(10_050, cards::TOXIC_DELUGE, PlayerId::One);
    let deluge_id = deluge.id;
    game.players[PlayerId::One.index()].hand.push(deluge);

    game.players[PlayerId::One.index()].life = 5;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == deluge_id && choices.x() == 1)
        })
        .expect("one spell life, one Confluence life, and two Channel life fit in five");

    game.players[PlayerId::One.index()].life = 3;
    assert!(
        !game.is_legal_action(PlayerId::One, &cast),
        "Channel and Mana Confluence cannot both claim life reserved by the spell",
    );
    assert!(game.apply(PlayerId::One, cast.clone()).is_err());
    assert_eq!(game.players[PlayerId::One.index()].life, 3);
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.id == deluge_id)
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == confluence)
            .is_some_and(|permanent| !permanent.tapped)
    );

    game.players[PlayerId::One.index()].life = 5;
    game.apply(PlayerId::One, cast)
        .expect("the aggregate life budget and execution agree at five");
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn channel_does_not_pay_a_coloured_symbol() {
    // Channel makes {C}. It can cover the generic half of a cost and nothing
    // else, so a spell whose coloured symbol is unpayable stays unpayable
    // however much life is left.
    let mut game = ready_game();
    resolve_channel(&mut game);
    game.battlefield
        .push(creature(10_000, cards::MOUNTAIN, PlayerId::One));
    let counterspell = card(10_001, cards::COUNTERSPELL, PlayerId::One);
    game.players[0].hand.push(counterspell.clone());

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == counterspell.id)
        ),
        "two blue is not something life can buy"
    );
    assert_eq!(game.players[0].life, 20, "and nothing was paid trying");
}

#[test]
fn channel_pays_a_true_colorless_symbol_when_the_spell_is_applied() {
    let definition_id = CardDefinitionId::new(59_900);
    let mut definition = CardDefinition::new(
        definition_id,
        "True colorless Channel test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(mana_cost!("{C}"));
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the test definition is valid");
    resolve_channel(&mut game);
    game.players[PlayerId::One.index()].life = 3;
    let spell = card(10_002, definition_id, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("Channel advertises the spell because its mana can pay {C}");
    game.apply(PlayerId::One, action)
        .expect("the advertised true-colorless Channel payment applies");

    assert_eq!(game.players[PlayerId::One.index()].life, 2);
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default(),
        "the one life became one {{C}} and that mana was spent",
    );
    assert!(
        game.players[PlayerId::One.index()].mana.is_empty(),
        "the attributed Channel mana was consumed",
    );
    assert_eq!(game.stack.len(), 1, "the paid-for spell is on the stack");
}

#[test]
fn fireball_may_be_cast_with_no_targets_at_all() {
    // "Any number of targets" includes none. It is a bad play, but it is a
    // legal one, and a spell that insists on a target is a different card.
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;

    let action = cast_action(fireball.id, Vec::new(), Vec::new(), 5);
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life, 20,
        "nothing to divide the damage among"
    );
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FIREBALL),
        "and it still resolved rather than fizzling"
    );
}

#[test]
fn fireball_keeps_dividing_by_the_targets_it_was_cast_with() {
    // A target that vanishes does not make the survivor's share larger: the
    // division is fixed by how many targets Fireball was aimed at.
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 7;
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));

    game.apply(
        PlayerId::One,
        cast_action(
            fireball.id,
            vec![
                Target::Player(PlayerId::Two),
                Target::Permanent(GameObjectId(10_001)),
            ],
            Vec::new(),
            5,
        ),
    )
    .unwrap();
    game.battlefield
        .retain(|permanent| permanent.card.id != GameObjectId(10_001));
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life, 18,
        "two each, and the fifth point is lost to the rounding"
    );
}

#[test]
fn fireball_cannot_spread_further_than_the_extra_cost_allows() {
    // Six red pays {R} plus X=4 with one extra target. A third target costs
    // another {1}, which is one more than the pool has, so that spread is not
    // a legal action at all rather than a cast that underpays.
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    let first = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let second = creature(10_002, cards::JUGGERNAUT, PlayerId::Two);
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;
    game.battlefield.push(first);
    game.battlefield.push(second);

    let two_targets = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(GameObjectId(10_001)),
        ],
        Vec::new(),
        4,
    );
    let three_targets = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(GameObjectId(10_001)),
            Target::Permanent(GameObjectId(10_002)),
        ],
        Vec::new(),
        4,
    );
    let legal = game.legal_actions(PlayerId::One);
    assert!(
        legal.contains(&two_targets),
        "one red, four for X, one for the extra target"
    );
    assert!(
        !legal.contains(&three_targets),
        "the second extra target would need a seventh mana"
    );
    assert!(
        game.apply(PlayerId::One, three_targets).is_err(),
        "and submitting it directly is refused too"
    );
}

#[test]
fn fireball_x_three_can_hit_three_targets_for_six_mana() {
    let mut game = ready_game();
    let fireball = card(10_000, cards::FIREBALL, PlayerId::One);
    let first_creature = creature(10_001, cards::SU_CHI, PlayerId::Two);
    let first_creature_id = first_creature.card.id;
    let second_creature = creature(10_002, cards::JUGGERNAUT, PlayerId::Two);
    let second_creature_id = second_creature.card.id;
    game.players[0].hand.push(fireball.clone());
    game.players[0].mana_pool.red = 6;
    game.battlefield.push(first_creature);
    game.battlefield.push(second_creature);

    let action = cast_action(
        fireball.id,
        vec![
            Target::Player(PlayerId::Two),
            Target::Permanent(first_creature_id),
            Target::Permanent(second_creature_id),
        ],
        Vec::new(),
        3,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 19);
    for creature_id in [first_creature_id, second_creature_id] {
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == creature_id)
                .unwrap()
                .damage,
            1
        );
    }
}

#[test]
fn fork_controller_can_retarget_the_copied_spell() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let fork = card(10_001, cards::FORK, PlayerId::One);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[0].hand.push(fork.clone());
    game.players[0].mana_pool.red = 2;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let original = game.stack[0].id;

    game.apply(
        PlayerId::One,
        cast_action(fork.id, vec![Target::Spell(original)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let retarget = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![retarget],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[1].life, 17);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
}

#[test]
fn copied_spell_freezes_retargeted_ability_payload() {
    let mut game = ready_game();
    let shatter = card(10_000, cards::SHATTER, PlayerId::Two);
    let original_target = creature(10_001, cards::SOL_RING, PlayerId::One);
    let replacement_target = creature(10_002, cards::ANKH_OF_MISHRA, PlayerId::One);
    let original_target_id = original_target.card.id;
    let replacement_target_id = replacement_target.card.id;
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.colorless = 1;
    game.players[1].mana_pool.red = 1;
    game.battlefield
        .extend([original_target, replacement_target]);
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(
            shatter.id,
            vec![Target::Permanent(original_target_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let original = game.stack[0].clone();
    let replacement_targets = vec![TargetSelection::single(
        TargetSlotId(0),
        Target::Permanent(replacement_target_id),
    )];

    game.push_copy(original, PlayerId::One, replacement_targets.clone());

    let copy = game.stack.last().expect("the copied spell is on the stack");
    assert_eq!(
        copy.signature.as_ref().map(CastSignature::targets),
        Some(replacement_targets.as_slice()),
    );
    assert_eq!(
        copy.ability
            .as_ref()
            .map(|ability| ability.targets.as_slice()),
        Some(replacement_targets.as_slice()),
        "the executable payload must use the copy's replacement targets",
    );

    game.destroy_permanent(original_target_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != replacement_target_id),
        "the copy must not fizzle because its original target became illegal",
    );
}

#[test]
fn fork_copies_a_targetless_spell_immediately_and_preserves_its_signature() {
    let mut game = ready_game();
    let original = spell(77, cards::DARK_RITUAL, PlayerId::Two, 0);
    let signature = original.signature.clone().unwrap();

    game.queue_copy_decision_chain(
        PlayerId::One,
        original,
        Some(ColorSet::from_colors(&[ManaColor::Red])),
        true,
        "the copy",
        1,
    );

    assert!(game.pending_decisions.is_empty());
    let copied = game.stack.last().expect("the targetless copy is immediate");
    assert!(copied.is_copy);
    assert_eq!(copied.controller, PlayerId::One);
    assert_eq!(copied.card.backing, ObjectBacking::None);
    assert_eq!(copied.signature.as_ref(), Some(&signature));
}

#[test]
fn fork_can_keep_an_original_target_that_has_become_illegal() {
    let mut game = ready_game();
    let stale_target = Target::Permanent(CardInstanceId(99_999));
    game.queue_copy_decision_chain(
        PlayerId::One,
        spell_with_targets(77, cards::SHATTER, PlayerId::Two, vec![stale_target], 0),
        Some(ColorSet::from_colors(&[ManaColor::Red])),
        true,
        "the copy",
        1,
    );
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Keep original targets")
    );
}

#[test]
fn structured_target_predicates_are_rechecked_when_the_spell_resolves() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    let animation_timestamp = attach_constant_resolved_characteristics(
        &mut game,
        factory_id,
        &TEST_MISHRAS_FACTORY_CHARACTERISTICS,
        ContinuousEffectExpiration::EndOfTurn,
    );
    let mut turn = spell(77, crate::card::cards::TURN_BURN, PlayerId::One, 0);
    turn.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::new(PlayOptionId::DEFAULT).with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(factory_id),
        )]),
    ));

    assert!(!game.spell_fizzles(&turn));
    game.battlefield[0]
        .resolved_continuous_effects
        .retain(|effect| effect.timestamp != animation_timestamp);
    assert!(game.spell_fizzles(&turn));
}

pub(super) fn game_with_test_fused_split(
    definition_id: CardDefinitionId,
    first: &CardRules,
    second: &CardRules,
) -> (Game, PlayOptionId, Vec<CardPartId>) {
    let mut definition = CardDefinition::new(
        definition_id,
        "First Half // Second Half",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = *first;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "First Half", *first),
        CardPart::new(CardPartId(1), "Second Half", *second),
    ];
    let combined = PlayOptionId(2);
    let parts = vec![CardPartId::PRIMARY, CardPartId(1)];
    definition.structure = CardStructure::Split {
        parts: parts.clone(),
        fused: Some(combined),
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "First Half",
            SpellForm::Part(CardPartId::PRIMARY),
            ManaCost::default(),
            CardEffectStatus::Implemented,
        ),
        PlayOptionDef::cast(
            PlayOptionId(1),
            "Second Half",
            SpellForm::Part(CardPartId(1)),
            ManaCost::default(),
            CardEffectStatus::Implemented,
        ),
        PlayOptionDef::cast(
            combined,
            "Fuse",
            SpellForm::Combined(parts.clone()),
            ManaCost::default(),
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    (game, combined, parts)
}

#[test]
fn combined_spell_trigger_and_target_characteristics_union_parts() {
    let definition_id = CardDefinitionId::new(10_066);
    let instant = CardRules::new_instant(ManaCost::default()).with_subtypes(&["Arcane"]);
    let sorcery = CardRules::new_sorcery(ManaCost::default()).with_subtypes(&["Lesson"]);
    let (mut game, combined, parts) = game_with_test_fused_split(definition_id, &instant, &sorcery);
    let mut object = spell(77, definition_id, PlayerId::One, 0);
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Combined(parts.clone()),
        CastChoices::new(combined),
    ));

    let trigger_object = game
        .stack_trigger_event_object(&object)
        .expect("a fused spell has trigger characteristics");
    assert!(trigger_object.types.contains(CardType::Instant));
    assert!(trigger_object.types.contains(CardType::Sorcery));
    assert_eq!(trigger_object.subtypes.as_ref(), &["Arcane", "Lesson"]);
    let event = CommittedTriggerEvent::SpellCast {
        object: trigger_object,
    };
    for predicate in [
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
        ObjectPredicateDef::Subtype("Arcane"),
        ObjectPredicateDef::Subtype("Lesson"),
    ] {
        assert!(game.trigger_event_matches_for_controller(
            TriggerEventDef::SpellCast(predicate),
            &event,
            GameObjectId(99_999),
            None,
        ));
    }

    game.stack.push(object);
    for predicate in [
        ObjectPredicateDef::HasType(CardType::Sorcery),
        ObjectPredicateDef::Subtype("Lesson"),
    ] {
        assert_eq!(
            game.ability_targets_matching(
                AbilityTargetPredicate::Object {
                    object: predicate,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
                PlayerId::One,
                GameObjectId(99_999),
                TriggerContext::empty(),
            ),
            vec![Target::Spell(GameObjectId(77))],
        );
    }
}

#[test]
fn split_card_target_characteristics_union_parts_outside_the_stack() {
    let definition_id = CardDefinitionId::new(10_067);
    let instant = CardRules::new_instant(ManaCost::default()).with_subtypes(&["Arcane"]);
    let sorcery = CardRules::new_sorcery(ManaCost::default()).with_subtypes(&["Lesson"]);
    let (mut game, _, _) = game_with_test_fused_split(definition_id, &instant, &sorcery);
    game.players[0]
        .graveyard
        .push(card(78, definition_id, PlayerId::One));

    for predicate in [
        ObjectPredicateDef::HasType(CardType::Sorcery),
        ObjectPredicateDef::Subtype("Lesson"),
    ] {
        assert_eq!(
            game.ability_targets_matching(
                AbilityTargetPredicate::Object {
                    object: predicate,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
                PlayerId::One,
                GameObjectId(99_999),
                TriggerContext::empty(),
            ),
            vec![Target::Card(GameObjectId(78))],
        );
    }
}

#[test]
fn animated_factory_keeps_types_and_last_known_stats_under_blood_moon() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let blood_moon = creature(10_001, cards::BLOOD_MOON, PlayerId::Two);
    game.battlefield = vec![factory, blood_moon];
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &TEST_MISHRAS_FACTORY_CHARACTERISTICS,
        ContinuousEffectExpiration::EndOfTurn,
    );

    let snapshot = game.battlefield_exit_snapshot(&game.battlefield[0]);
    assert_eq!(snapshot.last_known.power, Some(2));
    assert_eq!(snapshot.last_known.toughness, Some(2));
    // Blood Moon sets the land subtype and removes the printed abilities, but
    // Assembly-Worker is a creature type the animation grants, so it survives
    // alongside the Mountain that replaced the land types.
    assert_eq!(
        snapshot.object.subtypes.as_ref(),
        &["Mountain", "Assembly-Worker"]
    );
    for card_type in [CardType::Land, CardType::Creature, CardType::Artifact] {
        assert!(snapshot.object.types.contains(card_type));
    }

    let event = CommittedTriggerEvent::ZoneChanged {
        before: Some(snapshot.object),
        after: None,
        from: ZoneKind::Battlefield,
        to: ZoneKind::Graveyard,
        damage_sources: Vec::new(),
    };
    for card_type in [CardType::Land, CardType::Creature, CardType::Artifact] {
        assert!(game.trigger_event_matches_for_controller(
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(card_type),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard)
            ),
            &event,
            GameObjectId(99_999),
            None,
        ));
    }
}

#[test]
fn black_lotus_sacrifices_for_three_red_mana() {
    let mut game = ready_game();
    let lotus = creature(10_000, cards::BLACK_LOTUS, PlayerId::One);
    let lotus_id = lotus.card.id;
    game.battlefield.push(lotus);
    let action = Action::ActivateManaAbility {
        source: lotus_id,
        ability: mana_ability_for(&game, lotus_id, ManaColor::Red),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.players[0].mana_pool.red, 3);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != lotus_id)
    );
    let graveyard_lotus = game.players[0].graveyard.last().unwrap();
    assert_ne!(graveyard_lotus.id, lotus_id);
    assert_eq!(
        backing_cards(&graveyard_lotus.backing),
        vec![PhysicalCardId(10_000)]
    );
}

#[test]
fn the_legend_rule_keeps_one_pendelhaven_per_player() {
    let mut game = ready_game();
    let mut old_haven = creature(10_000, cards::PENDELHAVEN, PlayerId::One);
    old_haven.tapped = true;
    game.battlefield.push(old_haven);
    game.players[0]
        .hand
        .push(card(10_001, cards::PENDELHAVEN, PlayerId::One));
    // The opponent's own Pendelhaven is unaffected: the rule is per player.
    game.battlefield
        .push(creature(10_002, cards::PENDELHAVEN, PlayerId::Two));

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: CardInstanceId(10_001),
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let mine: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.definition == cards::PENDELHAVEN
        })
        .collect();
    assert_eq!(mine.len(), 1, "only one Pendelhaven survives");
    assert_eq!(
        backing_cards(&mine[0].card.backing),
        vec![PhysicalCardId(10_001)],
        "the untapped newcomer is kept over the tapped original",
    );
    assert!(!mine[0].tapped, "the survivor is the untapped one");
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the extra copy went to the graveyard",
    );
    assert!(
        game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::Two && permanent.card.definition == cards::PENDELHAVEN
        }),
        "the opponent keeps theirs",
    );
}

#[test]
fn black_vise_needs_no_target_and_still_squeezes_the_opponent() {
    let mut game = ready_game();
    let vise = card(10_000, cards::BLACK_VISE, PlayerId::One);
    game.players[0].hand.push(vise.clone());
    game.players[0].mana_pool.colorless = 1;

    // With two players "choose an opponent" has one answer, so the cast
    // carries no target and offers the player nothing to pick.
    let cast = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == vise.id))
        .collect();
    assert_eq!(casts, vec![cast.clone()], "exactly one way to cast it");

    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    let resolved = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BLACK_VISE)
        .expect("Black Vise resolved onto the battlefield");
    assert_eq!(
        resolved.chosen_player,
        Some(PlayerId::Two),
        "the opponent is implied rather than chosen",
    );

    // Six cards in hand is two beyond four, so their upkeep costs 2 life.
    for index in 0..6 {
        game.players[1]
            .hand
            .push(card(20_000 + index, cards::MOUNTAIN, PlayerId::Two));
    }
    let before = game.players[1].life;
    game.turn = 2;
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(game.players[1].life, before - 2);
}

#[test]
fn mox_ruby_can_pay_black_vises_generic_cost() {
    let mut game = ready_game();
    let mox = creature(10_000, cards::MOX_RUBY, PlayerId::One);
    let vise = card(10_001, cards::BLACK_VISE, PlayerId::One);
    let mox_id = mox.card.id;
    game.battlefield.push(mox);
    game.players[0].hand.push(vise.clone());

    let cast_vise = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast_vise));
    game.apply(PlayerId::One, cast_vise).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mox_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}
