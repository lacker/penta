use super::*;
use crate::AbilityProgramDef;

#[test]
fn supreme_verdict_destroys_every_creature() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::Two));
    let sol_ring = creature(10_003, cards::SOL_RING, PlayerId::Two);
    let sol_ring_id = sol_ring.card.id;
    game.battlefield.push(sol_ring);
    let verdict = card(10_002, cards::SUPREME_VERDICT, PlayerId::One);
    game.players[0].hand.push(verdict.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.blue = 3;

    game.apply(
        PlayerId::One,
        cast_action(verdict.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1, "only creatures are swept");
    assert_eq!(game.battlefield[0].card.id, sol_ring_id);
    assert_eq!(
        game.players[0].graveyard[0].definition,
        cards::SAVANNAH_LIONS
    );
    assert_eq!(game.players[1].graveyard[0].definition, cards::SERRA_ANGEL);
}

#[test]
fn supreme_verdict_does_not_stop_regeneration() {
    // Wrath of God says "they can't be regenerated". The Verdict does not.
    let mut game = ready_game();
    let mut troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    troll.regeneration_shields = 1;
    game.battlefield.push(troll);
    let verdict = card(10_002, cards::SUPREME_VERDICT, PlayerId::One);
    game.players[0].hand.push(verdict.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.blue = 3;

    game.apply(
        PlayerId::One,
        cast_action(verdict.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1, "the shield saves the troll");
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.battlefield[0].regeneration_shields, 0);
    assert!(game.players[1].graveyard.is_empty());
}

#[test]
fn a_counterspell_may_target_supreme_verdict_and_accomplish_nothing() {
    // Can't be countered is not can't be targeted: the Counterspell is a legal
    // play, resolves, and simply fails to do its job.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    let verdict = card(10_002, cards::SUPREME_VERDICT, PlayerId::One);
    let counterspell = card(10_003, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(verdict.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.blue = 3;
    game.players[1].hand.push(counterspell.clone());
    game.players[1].mana_pool.blue = 2;

    game.apply(
        PlayerId::One,
        cast_action(verdict.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let verdict_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(verdict_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    // Once for the Counterspell, once for the Verdict underneath it.
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert!(game.battlefield.is_empty(), "the sweep still happened");
    assert_eq!(game.players[1].graveyard[0].definition, cards::COUNTERSPELL);
    assert_eq!(
        game.players[0].graveyard[0].definition,
        cards::SUPREME_VERDICT,
        "the Verdict resolved rather than being countered"
    );
}

#[test]
fn mizzium_mortars_normally_targets_one_opposing_creature() {
    let mut game = ready_game();
    let friendly = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let friendly_id = friendly.card.id;
    let opposing = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let opposing_id = opposing.card.id;
    let hexproof = creature(10_002, cards::SIGARDA_HOST_OF_HERONS, PlayerId::Two);
    let hexproof_id = hexproof.card.id;
    game.battlefield.extend([friendly, opposing, hexproof]);
    let mortars = card(10_003, cards::MIZZIUM_MORTARS, PlayerId::One);
    game.players[0].hand.push(mortars.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 1;

    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == mortars.id))
        .collect::<Vec<_>>();
    assert_eq!(casts.len(), 1, "only the ordinary affordable cast is legal");
    let Action::CastSpell { choices, .. } = &casts[0] else {
        unreachable!("the filtered action is a spell cast")
    };
    assert_eq!(choices.costs().alternative(), None);
    assert_eq!(
        choices.iter_targets().copied().collect::<Vec<_>>(),
        [Target::Permanent(opposing_id)]
    );
    assert!(!choices.iter_targets().any(|target| {
        matches!(target, Target::Permanent(id) if *id == friendly_id || *id == hexproof_id)
    }));

    game.apply(PlayerId::One, casts.into_iter().next().unwrap())
        .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_id),
        "four damage destroys Serra Angel"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == friendly_id),
        "the friendly creature is not a legal normal target"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == hexproof_id),
        "the hexproof creature was not targeted"
    );
}

#[test]
fn overloaded_mizzium_mortars_is_targetless_and_hits_hexproof_opposing_creatures() {
    let mut game = ready_game();
    let friendly = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let friendly_id = friendly.card.id;
    let opposing = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let opposing_id = opposing.card.id;
    let hexproof = creature(10_002, cards::SIGARDA_HOST_OF_HERONS, PlayerId::Two);
    let hexproof_id = hexproof.card.id;
    let mut protected = creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two);
    protected
        .temporary_keywords
        .push(protection_keyword(ManaColor::Red));
    let protected_id = protected.card.id;
    game.battlefield
        .extend([friendly, opposing, hexproof, protected]);
    let mortars = card(10_004, cards::MIZZIUM_MORTARS, PlayerId::One);
    game.players[0].hand.push(mortars.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.colorless = 3;

    let overload = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == mortars.id
                        && choices.costs().alternative() == Some(AlternativeCostId(1))
            )
        })
        .expect("the overload cost is payable");
    let Action::CastSpell { choices, .. } = &overload else {
        unreachable!("the filtered action is a spell cast")
    };
    assert!(
        choices.targets().is_empty(),
        "overload removes every target"
    );

    game.apply(PlayerId::One, overload).unwrap();
    assert_eq!(
        game.observe(PlayerId::One)
            .stack
            .last()
            .and_then(|object| object.ability_text.as_deref()),
        Some("Mizzium Mortars deals 4 damage to each creature you don't control."),
        "the stack presents the transformed spell instruction rather than only the reminder text",
    );
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != opposing_id),
        "the opposing 4/4 takes lethal damage"
    );
    let sigarda = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == hexproof_id)
        .expect("four damage does not destroy the 5/5");
    assert_eq!(sigarda.damage, 4, "a targetless sweep ignores hexproof");
    let friendly = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == friendly_id)
        .expect("Mizzium Mortars only affects creatures you don't control");
    assert_eq!(friendly.damage, 0);
    let protected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == protected_id)
        .expect("protection from red prevents Mizzium Mortars' damage");
    assert_eq!(protected.damage, 0);
}

#[test]
fn overloaded_mizzium_mortars_resolves_with_no_matching_creatures() {
    let mut game = ready_game();
    let mortars = card(10_000, cards::MIZZIUM_MORTARS, PlayerId::One);
    game.players[0].hand.push(mortars.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.colorless = 3;
    let choices = CastChoices::default().with_costs(CostConfiguration::new(
        Some(AlternativeCostId(1)),
        Vec::new(),
    ));

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: mortars.id,
            choices,
            sacrifices: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MIZZIUM_MORTARS),
        "the targetless spell resolves rather than fizzling"
    );
}

#[test]
fn overloaded_mizzium_mortars_matches_creatures_when_it_resolves() {
    let mut game = ready_game();
    let mortars = card(10_000, cards::MIZZIUM_MORTARS, PlayerId::One);
    game.players[0].hand.push(mortars.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].mana_pool.colorless = 3;
    let choices = CastChoices::default().with_costs(CostConfiguration::new(
        Some(AlternativeCostId(1)),
        Vec::new(),
    ));
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: mortars.id,
            choices,
            sacrifices: Vec::new(),
        },
    )
    .unwrap();
    let late_creature = creature(10_001, cards::SIGARDA_HOST_OF_HERONS, PlayerId::Two);
    let late_creature_id = late_creature.card.id;
    game.battlefield.push(late_creature);
    pass_priority_pair(&mut game);

    let late_creature = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == late_creature_id)
        .expect("the 5/5 survives four damage");
    assert_eq!(
        late_creature.damage, 4,
        "the targetless overload effect determines its recipients at resolution",
    );
}

#[test]
fn counterflux_normally_counters_one_opposing_spell_and_cannot_be_countered() {
    let mut game = ready_game();
    let threat = spell(10_000, cards::SERRA_ANGEL, PlayerId::Two, 0);
    let threat_id = threat.id;
    game.stack.push(threat);
    let counterflux = card(10_001, cards::COUNTERFLUX, PlayerId::One);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(counterflux.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.red = 1;
    game.players[1].hand.push(counterspell.clone());
    game.players[1].mana_pool.blue = 2;

    game.apply(
        PlayerId::One,
        cast_action(
            counterflux.id,
            vec![Target::Spell(threat_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let counterflux_on_stack = game.stack.last().unwrap().id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(counterflux_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();

    pass_priority_pair(&mut game);
    assert!(
        game.stack
            .iter()
            .any(|object| object.id == counterflux_on_stack),
        "Counterspell resolves but cannot counter Counterflux"
    );
    pass_priority_pair(&mut game);

    assert!(
        game.stack.is_empty(),
        "Counterflux counters the original spell"
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL)
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::COUNTERSPELL)
    );
}

#[test]
fn counterflux_uses_not_you_for_both_casting_modes() {
    let catalog = poc::catalog().unwrap();
    let counterflux = catalog.get(cards::COUNTERFLUX).unwrap();

    let normal = counterflux.rules.ability(AbilityId(1)).unwrap();
    let DeclarativeAbilityDef::Spell(normal) = normal.definition else {
        panic!("Counterflux's normal instruction should be a spell ability")
    };
    assert!(matches!(
        normal.targets()[0].predicate,
        AbilityTargetPredicate::Object {
            controller: Some(PlayerRelation::NotYou),
            ..
        }
    ));

    let overload = counterflux.rules.ability(AbilityId(2)).unwrap();
    assert!(matches!(
        overload.effect.definition,
        AbilityProgramDef::Effects(EffectDef::Counter {
            object,
            ..
        }) if object.object_query().is_some_and(|query| {
            query.related_player == Some(PlayerSetDef::Related(PlayerRelation::NotYou))
                && query.controller.is_none()
                && query.owner.is_none()
        })
    ));
}

#[test]
fn a_non_executable_cannot_be_countered_clause_does_not_change_gameplay() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
        },
    )
    .with_source_zones(&[ZoneKind::Stack])
    .with_coverage(AbilityCoverageDef::metadata_only(
        "Test-only incomplete clause.",
    ))];
    let definition_id = CardDefinitionId::new(20_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Incomplete uncounterable spell",
        CardSet::ReturnToRavnica,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);
    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.stack
        .push(spell(20_000, definition_id, PlayerId::One, 0));

    assert!(game.can_be_countered(&game.stack[0]));
}

#[test]
fn a_composite_static_clause_can_make_its_source_uncounterable() {
    static COMPONENTS: [AppliedEffectDef; 1] =
        [AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Composite(&COMPONENTS),
        },
    )
    .with_source_zones(&[ZoneKind::Stack])];
    let definition_id = CardDefinitionId::new(20_001);
    let mut definition = CardDefinition::new(
        definition_id,
        "Composite uncounterable spell",
        CardSet::ReturnToRavnica,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);
    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.stack
        .push(spell(20_001, definition_id, PlayerId::One, 0));

    assert!(!game.can_be_countered(&game.stack[0]));
}

#[test]
fn a_composite_mana_spend_effect_can_make_a_spell_uncounterable() {
    static COMPONENTS: [AppliedEffectDef; 1] =
        [AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)];
    let mut object = spell(20_002, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    object.applied_effects.push(AppliedStackEffect {
        source: None,
        granting: None,
        effect: AppliedEffectDef::Composite(&COMPONENTS),
    });
    let game = ready_game();

    assert!(!game.can_be_countered(&object));
}

#[test]
fn overload_does_not_silently_discard_selected_modal_effects() {
    static MODES: [AbilityDef; 1] = [AbilityDef::spell(
        "Draw a card.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::choose_one_spell("Choose one.", &MODES),
        abilities::overload(
            mana_cost!("{0}"),
            "Draw two cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ];

    let definition_id = CardDefinitionId::new(20_003);
    let mut definition = CardDefinition::new(
        definition_id,
        "Modal overload test",
        CardSet::ReturnToRavnica,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);
    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(20_003, definition_id, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);

    assert!(game.legal_actions(PlayerId::One).iter().all(|action| {
        !matches!(
            action,
            Action::CastSpell { card, choices, .. }
                if *card == card_id
                    && choices.costs().alternative() == Some(AlternativeCostId(1))
        )
    }));

    let forged = CastChoices::default()
        .with_modes(vec![ModeId(0)])
        .with_costs(CostConfiguration::new(
            Some(AlternativeCostId(1)),
            Vec::new(),
        ));
    assert!(
        game.validated_cast_signature(PlayerId::One, card_id, &forged, &[])
            .is_none(),
        "validation must reject overload when its selected mode effects would be dropped",
    );
}

#[test]
fn overloaded_counterflux_is_targetless_and_counters_each_opposing_spell() {
    let mut game = ready_game();
    let friendly_spell = spell(10_000, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    let friendly_id = friendly_spell.id;
    game.stack.push(friendly_spell);
    let mut opposing_ability = spell(10_004, cards::SAVANNAH_LIONS, PlayerId::Two, 0);
    opposing_ability.kind = StackObjectKind::ActivatedAbility;
    opposing_ability.source = Some(GameObjectId(99_000));
    opposing_ability.signature = None;
    let opposing_ability_id = opposing_ability.id;
    game.stack.push(opposing_ability);
    let uncounterable = spell(10_005, cards::COUNTERFLUX, PlayerId::Two, 0);
    let uncounterable_id = uncounterable.id;
    game.stack.push(uncounterable);
    game.stack
        .push(spell(10_001, cards::SERRA_ANGEL, PlayerId::Two, 0));
    game.stack
        .push(spell(10_002, cards::TRISKELION, PlayerId::Two, 0));
    let counterflux = card(10_003, cards::COUNTERFLUX, PlayerId::One);
    game.players[0].hand.push(counterflux.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 1;

    let overload = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == counterflux.id
                        && choices.costs().alternative() == Some(AlternativeCostId(2))
            )
        })
        .expect("the overload cost is payable");
    let Action::CastSpell { choices, .. } = &overload else {
        unreachable!("the filtered action is a spell cast")
    };
    assert!(choices.targets().is_empty());

    game.apply(PlayerId::One, overload).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.stack.len(), 3);
    assert!(
        [friendly_id, opposing_ability_id, uncounterable_id]
            .into_iter()
            .all(|id| game.stack.iter().any(|object| object.id == id)),
        "your spell, an opposing ability, and an uncounterable opposing spell all remain",
    );
    assert_eq!(
        game.players[1]
            .graveyard
            .iter()
            .filter(|card| matches!(card.definition, cards::SERRA_ANGEL | cards::TRISKELION))
            .count(),
        2,
        "each opposing spell is countered"
    );
}
