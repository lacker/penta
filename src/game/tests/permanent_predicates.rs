use super::*;

#[test]
fn permanent_predicate_distinguishes_battlefield_identity_from_cards_and_spells() {
    let mut game = ready_game();
    let source = GameObjectId(910_000);
    for definition in [cards::ORNITHOPTER, cards::SOL_RING, cards::LIGHTNING_BOLT] {
        let card = card(910_001, definition, PlayerId::One);
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            assert!(!game.card_object_matches(ObjectPredicateDef::Permanent, &card, zone, source));
            assert!(game.card_object_matches(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Permanent),
                &card,
                zone,
                source
            ));
        }
        let spell = game
            .printed_trigger_event_object(
                card.id,
                definition,
                PlayerId::One,
                &CharacteristicContext::Stack {
                    form: SpellForm::Part(CardPartId::PRIMARY),
                },
            )
            .unwrap();
        assert!(!game.trigger_object_matches(ObjectPredicateDef::Permanent, &spell, source, true));
    }
    for definition in [cards::ORNITHOPTER, cards::FOREST] {
        let id = game
            .put_onto_battlefield(PlayerId::One, definition)
            .unwrap();
        let object =
            game.trigger_event_object(game.battlefield.iter().find(|p| p.card.id == id).unwrap());
        assert!(game.trigger_object_matches(ObjectPredicateDef::Permanent, &object, source, false));
        game.return_permanent_to_hand(id);
        assert!(game.trigger_object_matches(ObjectPredicateDef::Permanent, &object, source, false));
        let successor = game.players[0].hand.last().unwrap();
        assert!(!game.card_object_matches(
            ObjectPredicateDef::Permanent,
            successor,
            ZoneKind::Hand,
            source
        ));
    }
    let token = token_permanent(
        910_002,
        crate::card::TokenCharacteristics::creature(&["Soldier"], &[], 1, 1),
        PlayerId::One,
    );
    let id = token.card.id;
    game.battlefield.push(token);
    let object = game.trigger_event_object(game.battlefield.last().unwrap());
    game.destroy_permanent_without_regeneration(id);
    assert!(game.trigger_object_matches(
        ObjectPredicateDef::All(&[ObjectPredicateDef::Permanent, ObjectPredicateDef::Token,]),
        &object,
        source,
        false
    ));
}

#[test]
fn permanent_predicate_composes_in_static_effects_in_both_engines() {
    for prepared in [false, true] {
        let mut game = ready_game();
        let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000902");
        let definition = CardDefinition::new(
            id,
            "Permanent predicate fixture",
            crate::card::sets::magic_2015::SET,
            CardRules::new_enchantment(mana_cost!("{1}")).with_abilities(
                &const {
                    [
                        AbilityDef::static_ability(
                            "Creature permanents get +1/+1.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::Permanent,
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                    ]),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                            },
                        ),
                        AbilityDef::static_ability(
                            "Nonpermanents get +5/+5.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Permanent),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::Any,
                                ),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(5),
                                    ValueDef::Constant(5),
                                ),
                            },
                        ),
                    ]
                },
            ),
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
        game.set_prepared_engine_enabled(prepared);
        game.put_onto_battlefield(PlayerId::One, id).unwrap();
        let creature = game
            .put_onto_battlefield(PlayerId::One, cards::ORNITHOPTER)
            .unwrap();
        let creature = game
            .battlefield
            .iter()
            .find(|p| p.card.id == creature)
            .unwrap();
        assert_eq!(
            (game.power(creature), game.toughness(creature)),
            (Some(1), Some(3))
        );
    }
}
