use super::*;

#[test]
fn generic_cost_reduction_counts_matching_cards_outside_the_battlefield() {
    let mut game = ready_game();
    let ghoultree = card(10_000, cards::GHOULTREE, PlayerId::One);
    let source = ghoultree.id;
    game.players[0].hand.push(ghoultree);
    game.players[0].graveyard.extend([
        card(10_001, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_002, cards::JUGGERNAUT, PlayerId::One),
        card(10_003, cards::SENGIR_VAMPIRE, PlayerId::One),
    ]);
    game.players[0]
        .graveyard
        .push(card(10_004, cards::BLACK_VISE, PlayerId::One));

    assert_eq!(
        game.spell_cost_reduction(cards::GHOULTREE, PlayerId::One, source),
        3,
        "Ghoultree reads creature cards in its controller's graveyard rather than only battlefield permanents",
    );
}

#[test]
fn mana_preview_uses_existing_pool_before_tapping_sources() {
    let mut game = ready_game();
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let vise = card(10_002, cards::BLACK_VISE, PlayerId::One);
    let mountain_id = mountain.card.id;
    let mox_id = mox.card.id;
    game.battlefield.extend([mox, mountain]);
    game.players[0].mana_pool.colorless = 1;
    game.players[0].hand.push(vise.clone());

    let action = cast_action(vise.id, Vec::new(), Vec::new(), 0);
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        Vec::<CardInstanceId>::new(),
        "the floating mana already pays Black Vise's generic cost"
    );

    game.players[0].mana_pool = ManaPool::default();
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mox_id],
        "the preview chooses a single flexible source without mutating the game"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .expect("mountain remains on the battlefield")
            .tapped
    );
}

#[test]
fn mana_preview_uses_the_selected_declarative_activated_ability_cost() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}, {T}: Draw a card.",
            &[
                AbilityCostDef::Mana(ManaCost::new(1, 0)),
                AbilityCostDef::TapSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];

    let mut game = ready_game();
    let tome = creature(10_000, cards::JAYEMDAE_TOME, PlayerId::One);
    let first_ring = creature(10_001, cards::SOL_RING, PlayerId::One);
    let second_ring = creature(10_002, cards::SOL_RING, PlayerId::One);
    let tome_id = tome.card.id;
    let first_ring_id = first_ring.card.id;
    let second_ring_id = second_ring.card.id;
    game.battlefield.extend([tome, first_ring, second_ring]);
    let action = Action::ActivateAbility {
        source: tome_id,
        ability: activated_ability_for(&game, tome_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![first_ring_id, second_ring_id],
        "the behavior-free Tome activation previews its printed four-mana cost",
    );
    assert!(game.battlefield.iter().all(|permanent| !permanent.tapped));

    let definition_id = CardDefinitionId::new(10_065);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mana preview tap-source test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_010);
    let mountain = CardInstanceId(10_011);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(mountain.0, cards::MOUNTAIN, PlayerId::One),
    ]);
    let action = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };

    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mountain],
        "a source needed for the activation's tap cost is avoided when another source can pay",
    );
}

#[test]
fn orcish_mechanics_can_sacrifice_an_artifact_to_damage_a_creature() {
    let mut game = ready_game();
    let mechanics = creature(10_000, cards::ORCISH_MECHANICS, PlayerId::One);
    let artifact = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let target = creature(10_002, cards::SU_CHI, PlayerId::Two);
    let mechanics_id = mechanics.card.id;
    let artifact_id = artifact.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![mechanics, artifact, target];

    let action = Action::ActivateAbility {
        source: mechanics_id,
        ability: activated_ability_for(&game, mechanics_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        cost_objects: vec![artifact_id],
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mechanics_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != artifact_id)
    );
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Permanent(target_id)]);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .damage,
        0
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .damage,
        2
    );
}

#[test]
fn iron_star_payment_can_use_untapped_mana_sources() {
    let mut game = ready_game();
    let first_mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let second_mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let second_mountain_id = second_mountain.card.id;
    game.battlefield.extend([
        first_mountain,
        second_mountain,
        creature(10_002, cards::IRON_STAR, PlayerId::One),
    ]);
    let bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2, "Iron Star's trigger is above the Bolt");
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 21);
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second_mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn optional_payment_uses_its_declared_payer() {
    static IF_PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    let mut game = ready_game();
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let effect = EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::mana(
            PlayerSetDef::Related(PlayerRelation::Opponent),
            ManaCost::new(1, 0),
        ),
        &IF_PAID,
    ));

    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &source,
        TriggerContext::empty(),
    );
    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the declared payer receives the choice");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 21);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn optional_life_payment_is_private_and_resumes_the_paid_branch() {
    static IF_PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    let mut game = ready_game();
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    let effect = EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::life(PlayerSetDef::One(PlayerRefDef::EffectController), 2),
        &IF_PAID,
    ));

    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &source,
        TriggerContext::empty(),
    );
    assert!(
        game.observe(PlayerId::Two).decision.is_none(),
        "the other seat cannot inspect a private payment choice"
    );
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the payer receives the life-payment choice");
    assert_eq!(decision.options[1].label, "Pay 2 life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 21);
}

#[test]
fn nested_choice_payment_preserves_its_binding_and_outer_sequence_tail() {
    static DESTROY_CHOSEN: EffectDef = EffectDef::Destroy {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
        can_regenerate: false,
    };
    static PAY_TO_DESTROY: EffectDef = EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::mana(
            PlayerSetDef::Related(PlayerRelation::You),
            ManaCost::new(1, 0),
        ),
        &DESTROY_CHOSEN,
    ));
    static CHOOSE_CREATURE: EffectDef = EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerSetDef::One(PlayerRefDef::EffectController),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &PAY_TO_DESTROY,
    });
    static CONTROLLED_CREATURES_IN_GRAVEYARD: ObjectQueryDef = ObjectQueryDef::owned_by(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Graveyard],
        PlayerSetDef::One(PlayerRefDef::EffectController),
    );
    static OUTER_EFFECTS: [EffectDef; 2] = [
        CHOOSE_CREATURE,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&CONTROLLED_CREATURES_IN_GRAVEYARD),
        },
    ];

    let mut game = ready_game();
    let chosen = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let chosen_id = chosen.card.id;
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    game.battlefield.extend([chosen, mountain]);
    let source = spell(10_002, cards::LIGHTNING_BOLT, PlayerId::One, 0);

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&OUTER_EFFECTS)),
        &source,
        TriggerContext::empty(),
    );

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the nested payment suspends the outer sequence");
    assert_eq!(game.players[0].life, 20, "the outer tail has not run");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == chosen_id),
        "the paid branch has not run"
    );

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the paid branch consumed the object binding"
    );
    assert_eq!(
        game.players[0].life, 21,
        "the outer tail ran after the chosen creature reached the graveyard"
    );
}

#[test]
fn chain_lightning_copy_payment_can_use_untapped_mountains() {
    let mut game = ready_game();
    let first = creature(10_000, cards::MOUNTAIN, PlayerId::Two);
    let second = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let first_id = first.card.id;
    let second_id = second.card.id;
    game.battlefield = vec![first, second];
    game.queue_chain_lightning_decision(
        PlayerId::Two,
        spell_with_targets(
            77,
            cards::CHAIN_LIGHTNING,
            PlayerId::One,
            vec![Target::Player(PlayerId::Two)],
            0,
        ),
    );
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let copy = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![copy],
        },
    )
    .unwrap();

    assert_eq!(game.players[1].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| [first_id, second_id].contains(&permanent.card.id))
            .all(|permanent| permanent.tapped)
    );
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
    assert!(game.stack[0].is_copy);
}

#[test]
fn chain_lightning_copy_payment_can_use_a_creature_dealt_lethal_damage() {
    let mut game = ready_game();
    game.turns_started[PlayerId::Two.index()] = 1;
    let birds = creature(10_000, cards::BIRDS_OF_PARADISE, PlayerId::Two);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let birds_id = birds.card.id;
    let mountain_id = mountain.card.id;
    game.battlefield = vec![birds, mountain];
    let chain = card(10_002, cards::CHAIN_LIGHTNING, PlayerId::One);
    let chain_id = chain.id;
    game.players[0].hand.push(chain);
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(chain_id, vec![Target::Permanent(birds_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == birds_id)
            .is_some_and(|permanent| permanent.damage == 3),
        "state-based actions wait while Chain Lightning asks whether to pay",
    );
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let copy = decision
        .options
        .iter()
        .find(|option| option.label.contains("your opponent"))
        .map(|option| option.id)
        .unwrap();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![copy],
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != birds_id),
        "Birds dies only after its mana ability pays for the copy",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped),
    );
    assert_eq!(game.players[1].mana_pool, ManaPool::default());
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].targets(), vec![Target::Player(PlayerId::One)]);
    assert!(game.stack[0].is_copy);
}

#[test]
fn goblin_grenade_requires_and_sacrifices_a_goblin() {
    let mut game = ready_game();
    let grenade = card(10_000, cards::GOBLIN_GRENADE, PlayerId::One);
    let goblin = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.players[0].hand.push(grenade.clone());
    game.players[0].mana_pool.red = 1;
    game.battlefield.push(goblin);
    let action = cast_action(
        grenade.id,
        vec![Target::Player(PlayerId::Two)],
        vec![goblin_id],
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != goblin_id)
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15);
}

#[test]
fn goblin_grenade_eats_exactly_one_of_two_identical_goblins() {
    let mut game = ready_game();
    let grenade = card(10_000, cards::GOBLIN_GRENADE, PlayerId::One);
    let first = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let first_id = first.card.id;
    let second_id = second.card.id;
    game.players[0].hand.push(grenade.clone());
    game.players[0].mana_pool.red = 1;
    game.battlefield.push(first);
    game.battlefield.push(second);

    // Each identical Goblin is its own separate cost, not one lumped choice.
    let casts: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == grenade.id))
        .collect();
    assert!(
        casts.iter().all(|action| matches!(
            action,
            Action::CastSpell { sacrifices, .. } if sacrifices.len() == 1
        )),
        "every Grenade cast sacrifices exactly one Goblin",
    );

    game.apply(
        PlayerId::One,
        cast_action(
            grenade.id,
            vec![Target::Player(PlayerId::Two)],
            vec![first_id],
            0,
        ),
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first_id),
        "the chosen Goblin is gone",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_id),
        "its twin stays on the battlefield",
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second_id),
        "resolving the Grenade does not take the twin either",
    );
}

#[test]
fn hypnotic_specter_discards_after_dealing_combat_damage() {
    let mut game = ready_game();
    let mut specter = creature(10_000, cards::HYPNOTIC_SPECTER, PlayerId::One);
    specter.attacking = true;
    game.battlefield.push(specter);
    game.players[1]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::Two));

    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert!(game.players[1].hand.is_empty());
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert!(game.events().iter().any(|event| {
        matches!(
            event,
            GameEvent::CardsDiscarded { player: PlayerId::Two, cards }
                if cards.len() == 1 && cards[0].1 == cards::MOUNTAIN
        )
    }));
}

#[test]
#[allow(clippy::too_many_lines)]
fn factory_animates_and_strip_mine_destroys_lands() {
    let mut game = ready_game();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let strip = creature(10_001, cards::STRIP_MINE, PlayerId::One);
    let opposing_factory = creature(10_002, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let factory_id = factory.card.id;
    let strip_id = strip.card.id;
    let opposing_id = opposing_factory.card.id;
    game.battlefield = vec![factory, strip, opposing_factory];
    game.players[0].mana_pool.colorless = 1;

    assert_eq!(
        activated_ability_for(&game, factory_id, 0),
        AbilityOrigin::Printed {
            definition: cards::MISHRA_S_FACTORY,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        }
    );
    assert_eq!(
        activated_ability_for(&game, factory_id, 1),
        AbilityOrigin::Printed {
            definition: cards::MISHRA_S_FACTORY,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(2),
        }
    );
    assert_eq!(
        activated_ability_for(&game, strip_id, 0),
        AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        }
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: factory_id,
            ability: activated_ability_for(&game, factory_id, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == factory_id)
            .and_then(|permanent| game.power(permanent)),
        Some(2)
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::ActivateAbility {
                source: factory_id,
                ability: AbilityOrigin::Printed {
                    definition: cards::MISHRA_S_FACTORY,
                    part: CardPartId::PRIMARY,
                    ability: crate::AbilityId(2),
                },
                targets: activated_targets(Target::Permanent(factory_id)),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
            })
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: strip_id,
            ability: activated_ability_for(&game, strip_id, 0),
            targets: activated_targets(Target::Permanent(opposing_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        })
    );
    assert_eq!(
        game.stack[0].targets(),
        vec![Target::Permanent(opposing_id)]
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != strip_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == opposing_id)
    );

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_id)
    );
}

#[test]
fn mishras_factory_can_use_its_own_mana_to_animate() {
    let mut game = ready_game();
    let factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    let factory_id = factory.card.id;
    game.battlefield = vec![factory];
    let animate = Action::ActivateAbility {
        source: factory_id,
        ability: activated_ability_for(&game, factory_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&animate));
    game.apply(PlayerId::One, animate).unwrap();
    drain_pending(&mut game);

    let factory = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == factory_id)
        .unwrap();
    assert!(factory.tapped);
    assert_eq!(game.power(factory), Some(2));
    assert_eq!(game.players[0].mana_pool.total(), 0);

    let shatter = card(10_001, cards::SHATTER, PlayerId::Two);
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.red = 2;
    game.priority = PlayerId::Two;
    assert!(game.legal_actions(PlayerId::Two).contains(&cast_action(
        shatter.id,
        vec![Target::Permanent(factory_id)],
        Vec::new(),
        0,
    )));
}

#[test]
fn an_animated_untapped_mishras_factory_can_block() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::GOBLINS_OF_THE_FLARG, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let factory = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let factory_id = factory.card.id;
    game.battlefield = vec![attacker, factory];
    attach_constant_resolved_characteristics(
        &mut game,
        factory_id,
        &TEST_MISHRAS_FACTORY_CHARACTERISTICS,
        ContinuousEffectExpiration::EndOfTurn,
    );
    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: factory_id,
                attacker: attacker_id,
            })
    );
}

#[test]
fn strip_mine_can_be_activated_in_response_to_strip_mine() {
    let mut game = ready_game();
    let first_strip = creature(10_000, cards::STRIP_MINE, PlayerId::One);
    let second_strip = creature(10_001, cards::STRIP_MINE, PlayerId::Two);
    let other_land = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let first_strip_id = first_strip.card.id;
    let second_strip_id = second_strip.card.id;
    let other_land_id = other_land.card.id;
    game.battlefield = vec![first_strip, second_strip, other_land];
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        Action::ActivateAbility {
            source: second_strip_id,
            ability: activated_ability_for(&game, second_strip_id, 0),
            targets: activated_targets(Target::Permanent(first_strip_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    let response = Action::ActivateAbility {
        source: first_strip_id,
        ability: activated_ability_for(&game, first_strip_id, 0),
        targets: activated_targets(Target::Permanent(other_land_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&response));
    game.apply(PlayerId::One, response).unwrap();
    assert_eq!(game.stack.len(), 2);

    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != other_land_id)
    );
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| ![first_strip_id, second_strip_id].contains(&permanent.card.id))
    );
}

#[test]
fn icatian_javelineers_cannot_activate_until_their_controller_turn() {
    let mut game = ready_game();
    let mut javeliners = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    javeliners.counters[CounterKind::Javelin.index()] = 1;
    javeliners.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let source = javeliners.card.id;
    game.battlefield = vec![javeliners];
    let action = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert_eq!(game.power(&game.battlefield[0]), Some(1));
    assert_eq!(game.toughness(&game.battlefield[0]), Some(1));

    assert!(!game.legal_actions(PlayerId::One).contains(&action));

    game.start_next_turn();
    game.priority = PlayerId::One;
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(!game.legal_actions(PlayerId::One).contains(&action));

    game.start_next_turn();
    game.priority = PlayerId::One;
    assert_eq!(game.active_player, PlayerId::One);
    assert!(game.legal_actions(PlayerId::One).contains(&action));
}

#[test]
fn icatian_javelineers_counter_cost_preserves_white_source_targeting() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let mut javelineers = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    javelineers.counters[CounterKind::Javelin.index()] = 1;
    let source = javelineers.card.id;
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    let knight_id = knight.card.id;
    game.battlefield = vec![javelineers, knight];

    let protected_target = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Permanent(knight_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&protected_target),
        "protection from white sees the activated ability's white source",
    );

    let player_target = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    game.apply(PlayerId::One, player_target).unwrap();
    let javelineers = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("paying the counter cost leaves the source on the battlefield");
    assert!(javelineers.tapped);
    assert_eq!(javelineers.counters(CounterKind::Javelin), 0);
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
    ));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].life, 19);
}

#[test]
fn an_unplannable_payment_reports_what_the_planner_saw() {
    let mut game = ready_game();
    let mut mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    mountain.tapped = true;
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    game.battlefield.extend([mountain, mox]);

    let cost = ManaCost {
        generic: 5,
        ..ManaCost::default()
    };
    let report = game.unplannable_payment(PlayerId::One, cost, 0, None, &ManaPaymentPurpose::Other);

    assert!(
        report.starts_with("a legal payment has a complete mana activation plan"),
        "the invariant still names itself first: {report}"
    );
    assert!(report.contains("cost {5} with x 0"), "{report}");
    assert!(
        report.contains("affordable per the gate: false"),
        "the report says whether the gate and the plan actually disagree: {report}"
    );
    assert!(
        report.contains("Mox Ruby produces [Red]"),
        "an untapped source is listed with what it makes: {report}"
    );
    assert!(
        !report.contains("Mountain"),
        "a tapped land offers no activation, so it is not a candidate: {report}"
    );
}
