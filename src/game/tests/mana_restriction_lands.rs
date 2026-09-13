use super::*;
use crate::card::{AbilityPredicateDef, CreatureTypeSetDef};

fn put(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    game.put_onto_battlefield(PlayerId::One, definition)
        .unwrap()
}

fn tap(game: &mut Game, source: GameObjectId, color: ManaColor) -> Mana {
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateManaAbility { source: id, color: c, .. } if *id == source && *c == color)
    }).expect("the requested mana is offered");
    game.apply(PlayerId::One, action).unwrap();
    *game.players[0].mana.last().unwrap()
}

fn cast_purpose(definition: CardDefinitionId) -> ManaPaymentPurpose {
    ManaPaymentPurpose::Spell {
        object: GameObjectId(900_000),
        definition,
        controller: PlayerId::One,
        commander_owner: None,
        form: SpellForm::Part(CardPartId::PRIMARY),
        reserved_life_payment: 0,
    }
}

fn ability_purpose(source: GameObjectId) -> ManaPaymentPurpose {
    ManaPaymentPurpose::Ability {
        source,
        taps_source: false,
        leaves_source: false,
    }
}

fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("the activated ability is offered");
    game.apply(PlayerId::One, action).unwrap();
}

fn cast(game: &mut Game, object: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == object))
        .expect("the spell is castable");
    game.apply(PlayerId::One, action).unwrap();
}

#[test]
fn creature_only_mana_cannot_pay_for_abilities_or_resolving_payments() {
    for definition in [cards::ANCIENT_ZIGGURAT, cards::ABUNDANT_COUNTRYSIDE] {
        let mut game = ready_game();
        let land = put(&mut game, definition);
        let goblin = put(&mut game, cards::GOBLIN_BALLOON_BRIGADE);
        let mana = tap(&mut game, land, ManaColor::Red);
        assert!(game.mana_can_pay_for(mana, &cast_purpose(cards::GOBLIN_BALLOON_BRIGADE)));
        assert!(!game.mana_can_pay_for(mana, &cast_purpose(cards::LIGHTNING_BOLT)));
        assert!(!game.mana_can_pay_for(mana, &ability_purpose(goblin)));
        assert!(!game.mana_can_pay_for(
            mana,
            &ManaPaymentPurpose::Payment {
                source: goblin,
                label: None,
                snow: false
            }
        ));
        assert!(!game.mana_can_pay_for(mana, &ManaPaymentPurpose::Other));
        assert!(
            !game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == goblin))
        );
    }
}

#[test]
fn courtyard_plans_creature_casts_and_battlefield_activations() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let courtyard = put(&mut game, cards::SECLUDED_COURTYARD);
        choose_decision_by_label(&mut game, PlayerId::One, "Goblin");
        let goblin = card(900_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
        game.players[0].hand.push(goblin.clone());
        cast(&mut game, goblin.id);
        drain_pending(&mut game);
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == courtyard)
                .unwrap()
                .tapped
        );
        let goblin = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::GOBLIN_BALLOON_BRIGADE)
            .unwrap()
            .card
            .id;
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == courtyard)
            .unwrap()
            .tapped = false;
        activate(&mut game, goblin);
        drain_pending(&mut game);
        let goblin = game
            .battlefield
            .iter()
            .find(|p| p.card.id == goblin)
            .unwrap();
        assert!(game.permanent_has_executable_keyword(goblin, KeywordAbility::Flying));
        assert_eq!(game.players[0].mana_pool.total(), 0);
    }
}

#[test]
fn courtyard_mana_uses_creature_sources_in_other_zones_and_rejects_resolution() {
    let mut game = ready_game();
    let courtyard = put(&mut game, cards::SECLUDED_COURTYARD);
    choose_decision_by_label(&mut game, PlayerId::One, "Goblin");
    let mana = tap(&mut game, courtyard, ManaColor::Red);
    let goblin = card(900_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let human = card(900_003, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    for zone in [ZoneKind::Hand, ZoneKind::Graveyard, ZoneKind::Exile] {
        let cards = match zone {
            ZoneKind::Hand => &mut game.players[0].hand,
            ZoneKind::Graveyard => &mut game.players[0].graveyard,
            ZoneKind::Exile => &mut game.players[0].exile,
            _ => unreachable!(),
        };
        cards.extend([goblin.clone(), human.clone()]);
        assert!(game.mana_can_pay_for(mana, &ability_purpose(goblin.id)));
        assert!(!game.mana_can_pay_for(mana, &ability_purpose(human.id)));
        game.players[0].hand.clear();
        game.players[0].graveyard.clear();
        game.players[0].exile.clear();
    }
    let goblin = put(&mut game, cards::GOBLIN_BALLOON_BRIGADE);
    assert!(!game.mana_can_pay_for(
        mana,
        &ManaPaymentPurpose::Payment {
            source: goblin,
            label: None,
            snow: false
        }
    ));
    assert!(!game.mana_can_pay_for(mana, &ability_purpose(courtyard)));
    // Test the leaf independently of the disjunction too.
    let activation_only = Mana {
        restrictions: &[ManaRestrictionDef::ActivateAbility(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        ..mana
    };
    assert!(!game.mana_can_pay_for(
        activation_only,
        &ManaPaymentPurpose::Payment {
            source: goblin,
            label: None,
            snow: false
        }
    ));
    let empty = Mana {
        restrictions: &[ManaRestrictionDef::AnyOf(&[])],
        ..mana
    };
    assert!(!game.mana_can_pay_for(empty, &ability_purpose(goblin)));
    let conjunction = Mana {
        restrictions: &[
            ManaRestrictionDef::AnyOf(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::HasType(CardType::Creature)),
                ManaRestrictionDef::ActivateAbility(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ]),
            ManaRestrictionDef::CannotCastSpell(ObjectPredicateDef::HasType(CardType::Creature)),
        ],
        ..mana
    };
    assert!(game.mana_can_pay_for(conjunction, &ability_purpose(goblin)));
    assert!(!game.mana_can_pay_for(conjunction, &cast_purpose(cards::GOBLIN_BALLOON_BRIGADE)));
}

#[test]
fn courtyard_choice_survives_source_return_and_checkpoint() {
    let mut game = ready_game();
    let courtyard = put(&mut game, cards::SECLUDED_COURTYARD);
    choose_decision_by_label(&mut game, PlayerId::One, "Goblin");
    let mana = tap(&mut game, courtyard, ManaColor::Red);
    game.return_permanent_to_hand(courtyard);
    let returning = game.players[0].hand.pop().unwrap();
    game.put_card_onto_battlefield_from(
        returning,
        ZoneKind::Hand,
        BattlefieldArrival::under(PlayerId::One),
        None,
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Human");
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    assert_eq!(rebuilt.players[0].mana[0], mana);
    assert!(rebuilt.mana_can_pay_for(mana, &cast_purpose(cards::GOBLIN_BALLOON_BRIGADE)));
    assert!(!rebuilt.mana_can_pay_for(mana, &cast_purpose(cards::ICATIAN_JAVELINEERS)));
    let new_land = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.definition == cards::SECLUDED_COURTYARD)
        .unwrap();
    new_land.chosen_creature_type_binding = None;
    let new_id = new_land.card.id;
    let unbound = tap(&mut game, new_id, ManaColor::White);
    assert!(!game.mana_can_pay_for(unbound, &cast_purpose(cards::ICATIAN_JAVELINEERS)));
}

#[test]
fn beacon_offers_only_distinct_color_pairs_and_keeps_restrictions_on_each_unit() {
    let mut game = ready_game();
    let beacon = put(&mut game, cards::INTERPLANAR_BEACON);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let actions = game.legal_actions(PlayerId::One).into_iter().filter(|a| matches!(a, Action::ActivateManaAbility { source, combination: Some(_), .. } if *source == beacon)).collect::<Vec<_>>();
    assert_eq!(actions.len(), 10);
    for action in actions {
        let mut branch = game.clone();
        let Action::ActivateManaAbility {
            combination: Some(split),
            ..
        } = &action
        else {
            unreachable!()
        };
        assert_eq!(split.total(), 2);
        assert_eq!(split.iter().count(), 2);
        assert_eq!(split.get(ManaColor::Colorless), 0);
        branch.apply(PlayerId::One, action).unwrap();
        assert_eq!(branch.players[0].mana_pool.total(), 2);
        for mana in &branch.players[0].mana {
            assert!(branch.mana_can_pay_for(*mana, &cast_purpose(cards::NICOL_BOLAS_PLANESWALKER)));
            assert!(!branch.mana_can_pay_for(*mana, &cast_purpose(cards::SERRA_ANGEL)));
            assert!(!branch.mana_can_pay_for(*mana, &ManaPaymentPurpose::Other));
        }
        let (wire, hidden) = checkpoint_fixture(&branch, PlayerId::One);
        let rebuilt = Game::from_observation_checkpoint(
            branch.catalog.clone(),
            branch.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        assert_eq!(rebuilt.players[0].mana, branch.players[0].mana);
    }
}

#[test]
fn beacon_life_trigger_does_not_require_spending_its_mana() {
    let mut game = ready_game();
    put(&mut game, cards::INTERPLANAR_BEACON);
    let walker = card(900_004, cards::NICOL_BOLAS_PLANESWALKER, PlayerId::One);
    game.players[0].hand.push(walker.clone());
    for color in ManaColor::ALL {
        game.add_unrestricted_mana(PlayerId::One, color, 8);
    }
    cast(&mut game, walker.id);
    assert_eq!(game.players[0].life, 20);
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 21);
}

#[test]
fn sliver_hive_checks_control_only_when_activating() {
    let mut game = ready_game();
    let hive = put(&mut game, cards::SLIVER_HIVE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == hive))
    );
    // Countryside's changeling is a Sliver and therefore opens Hive's gate.
    let countryside = put(&mut game, cards::ABUNDANT_COUNTRYSIDE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
    activate(&mut game, countryside);
    drain_pending(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == ObjectKind::Token)
        .unwrap()
        .card
        .id;
    activate(&mut game, hive);
    game.destroy_permanent_without_regeneration(token);
    drain_pending(&mut game);
    let sliver = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == ObjectKind::Token)
        .unwrap();
    assert_eq!(&*game.effective_subtypes(sliver), &["Sliver"]);
    assert_eq!(
        (game.power(sliver), game.toughness(sliver)),
        (Some(1), Some(1))
    );
    assert_eq!(
        game.effective_colors(sliver, &game.effective_rules(sliver).unwrap()),
        [false; 5]
    );
}

#[test]
fn haven_returns_dragon_creatures_or_ugin_planeswalkers_from_own_graveyard() {
    for definition in [cards::SHIVAN_DRAGON, cards::UGIN_EYE_OF_THE_STORMS] {
        let mut game = ready_game();
        let haven = put(&mut game, cards::HAVEN_OF_THE_SPIRIT_DRAGON);
        let target = card(900_005, definition, PlayerId::One);
        game.players[0].graveyard.extend([
            target.clone(),
            card(900_006, cards::NICOL_BOLAS_PLANESWALKER, PlayerId::One),
        ]);
        game.players[1]
            .graveyard
            .push(card(900_007, definition, PlayerId::Two));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let actions = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == haven))
            .collect::<Vec<_>>();
        assert!(!actions.is_empty());
        for action in &actions {
            let Action::ActivateAbility { targets, .. } = action else {
                unreachable!()
            };
            assert_eq!(
                targets,
                &[TargetSelection::new(
                    TargetSlotId(0),
                    vec![Target::Card(target.id)]
                )]
            );
        }
        game.apply(PlayerId::One, actions[0].clone()).unwrap();
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::HAVEN_OF_THE_SPIRIT_DRAGON)
        );
        drain_pending(&mut game);
        assert!(
            game.players[0]
                .hand
                .iter()
                .any(|c| c.definition == definition)
        );
    }
}

fn apply_to(game: &mut Game, target: GameObjectId, effect: AppliedEffectDef) {
    let object = spell_with_targets(
        900_100,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(target)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        }),
        &object,
        TriggerContext::empty(),
    );
}

#[test]
fn countryside_changeling_survives_ability_loss_but_later_types_override_it() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let land = put(&mut game, cards::ABUNDANT_COUNTRYSIDE);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
        activate(&mut game, land);
        drain_pending(&mut game);
        let token = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == ObjectKind::Token)
            .unwrap()
            .card
            .id;
        apply_to(
            &mut game,
            token,
            AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        );
        let p = game
            .battlefield
            .iter()
            .find(|p| p.card.id == token)
            .unwrap();
        assert!(!game.permanent_has_executable_keyword(p, KeywordAbility::Changeling));
        for subtype in ["Sliver", "Dragon", "Time Lord"] {
            assert!(game.effective_subtypes(p).contains(&subtype));
        }
        apply_to(
            &mut game,
            token,
            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Frog"])),
        );
        let p = game
            .battlefield
            .iter()
            .find(|p| p.card.id == token)
            .unwrap();
        assert_eq!(&*game.effective_subtypes(p), &["Frog"]);
    }
}

fn add_fixture(game: &mut Game, rules: CardRules) -> CardDefinitionId {
    let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000901");
    let definition = CardDefinition::new(
        id,
        "Restriction fixture",
        crate::card::sets::magic_2015::SET,
        rules,
    );
    game.catalog = CardCatalog::new(
        game.catalog
            .definitions()
            .into_iter()
            .cloned()
            .chain([definition]),
    )
    .unwrap();
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    id
}

#[test]
fn subtype_spell_restrictions_distinguish_kindred_from_creature_spells() {
    let mut game = ready_game();
    let kindred = add_fixture(
        &mut game,
        CardRules::new_instant(mana_cost!("{R}"))
            .with_type(CardType::Kindred)
            .with_subtypes(&["Sliver", "Dragon"]),
    );
    let hive = put(&mut game, cards::SLIVER_HIVE);
    let haven = put(&mut game, cards::HAVEN_OF_THE_SPIRIT_DRAGON);
    let hive_mana = tap(&mut game, hive, ManaColor::Red);
    let haven_mana = tap(&mut game, haven, ManaColor::Red);
    assert!(game.mana_can_pay_for(hive_mana, &cast_purpose(kindred)));
    assert!(!game.mana_can_pay_for(haven_mana, &cast_purpose(kindred)));
    assert!(!game.mana_can_pay_for(hive_mana, &cast_purpose(cards::SHIVAN_DRAGON)));
    assert!(game.mana_can_pay_for(haven_mana, &cast_purpose(cards::SHIVAN_DRAGON)));
    assert!(!game.mana_can_pay_for(haven_mana, &cast_purpose(cards::UGIN_EYE_OF_THE_STORMS)));
}

#[test]
fn courtyard_pays_for_an_activated_creature_ability_from_hand() {
    for prepared in [false, true] {
        let mut game = ready_game();
        let id = add_fixture(
            &mut game,
            CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
                AbilityDef::activated(
                    "{R}, Discard this card: You gain 1 life.",
                    &[CostDef::Mana(mana_cost!("{R}")), CostDef::DiscardSource],
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                )
                .with_source_zones(&[ZoneKind::Hand]),
            ),
        );
        game.set_prepared_engine_enabled(prepared);
        let courtyard = put(&mut game, cards::SECLUDED_COURTYARD);
        choose_decision_by_label(&mut game, PlayerId::One, "Goblin");
        let source = card(900_010, id, PlayerId::One);
        game.players[0].hand.push(source.clone());
        activate(&mut game, source.id);
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, 21);
        assert!(game.players[0].graveyard.iter().any(|c| c.definition == id));
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == courtyard)
                .unwrap()
                .tapped
        );
    }
}

#[test]
fn changeling_defines_creature_types_in_every_card_zone() {
    for prepared in [false, true] {
        let mut game = ready_game();
        let id = add_fixture(
            &mut game,
            CardRules::new_creature(mana_cost!("{1}"), &["Shapeshifter"], 1, 1)
                .with_ability(abilities::changeling()),
        );
        game.set_prepared_engine_enabled(prepared);
        for context in [
            CharacteristicContext::Library,
            CharacteristicContext::Hand,
            CharacteristicContext::Graveyard,
            CharacteristicContext::Exile,
            CharacteristicContext::Command,
            CharacteristicContext::Stack {
                form: SpellForm::Part(CardPartId::PRIMARY),
            },
        ] {
            let object = game
                .printed_trigger_event_object(GameObjectId(900_020), id, PlayerId::One, &context)
                .unwrap();
            for subtype in ["Dragon", "Sliver", "Time Lord"] {
                assert!(object.subtypes.contains(&subtype), "{context:?}");
            }
        }
        let permanent = put(&mut game, id);
        let haven = put(&mut game, cards::HAVEN_OF_THE_SPIRIT_DRAGON);
        let mana = tap(&mut game, haven, ManaColor::White);
        assert!(game.mana_can_pay_for(mana, &cast_purpose(id)));
        let object = game
            .battlefield
            .iter()
            .find(|p| p.card.id == permanent)
            .unwrap();
        let copied = Game::copiable_characteristics(object);
        game.create_token_copy(PlayerId::One, copied, None, CardPartId::PRIMARY);
        drain_pending(&mut game);
        let copy = game
            .battlefield
            .iter()
            .find(|p| p.card.definition.is_token())
            .unwrap();
        assert!(game.effective_subtypes(copy).contains(&"Dragon"));
        assert!(game.permanent_has_executable_keyword(copy, KeywordAbility::Changeling));
    }
}

#[test]
fn countryside_token_and_changeling_reconstruct_from_checkpoint() {
    let mut game = ready_game();
    let land = put(&mut game, cards::ABUNDANT_COUNTRYSIDE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
    activate(&mut game, land);
    drain_pending(&mut game);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    let token = rebuilt
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(rebuilt.permanent_has_executable_keyword(token, KeywordAbility::Changeling));
    assert!(rebuilt.effective_subtypes(token).contains(&"Sliver"));
}

#[test]
fn kindred_permanents_and_copy_added_changeling_keep_creature_types() {
    let mut game = ready_game();
    let kindred = add_fixture(
        &mut game,
        CardRules::new_enchantment(mana_cost!("{1}"))
            .with_type(CardType::Kindred)
            .with_subtypes(&["Shapeshifter"])
            .with_ability(abilities::changeling()),
    );
    let source = put(&mut game, kindred);
    let p = game
        .battlefield
        .iter()
        .find(|p| p.card.id == source)
        .unwrap();
    assert!(game.effective_subtypes(p).contains(&"Sliver"));
    let mut copied = copied_characteristics(cards::SERRA_ANGEL);
    copied.added_abilities.push(CopiableAbility {
        origin: AbilityOrigin::Printed {
            definition: kindred,
            part: CardPartId::PRIMARY,
            ability: AbilityId(0),
        },
        definition: abilities::changeling(),
    });
    game.create_token_copy(PlayerId::One, copied, None, CardPartId::PRIMARY);
    drain_pending(&mut game);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(game.effective_subtypes(copy).contains(&"Sliver"));
}
