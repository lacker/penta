//! Proposed spells and committed spells must agree on characteristics and cost.
use super::*;

fn setup(
    prepared: bool,
    definition: CardDefinitionId,
    modifier: CardDefinitionId,
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, modifier).unwrap();
    let held = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = held.id;
    game.players[0].hand.push(held);
    (game, id)
}

fn cast_option(game: &Game, id: GameObjectId, option: PlayOptionId) -> Option<Action> {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::CastSpell { card, choices, .. } if *card == id && choices.play_option() == option)
    })
}

#[test]
fn adventure_discount_agrees_in_enumeration_preview_validation_and_stack() {
    for prepared in [false, true] {
        let (mut game, id) = setup(
            prepared,
            cards::BONECRUSHER_GIANT,
            cards::GOBLIN_ELECTROMANCER,
        );
        let mountain = game
            .put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
            .unwrap();
        let before = checkpoint_fixture(&game, PlayerId::One);
        let action = cast_option(&game, id, PlayOptionId(1)).expect("Stomp costs only R");
        assert!(cast_option(&game, id, PlayOptionId::DEFAULT).is_none());
        assert_eq!(
            game.mana_sources_for_action(PlayerId::One, &action),
            vec![mountain]
        );
        assert_eq!(
            checkpoint_fixture(&game, PlayerId::One),
            before,
            "queries are read-only"
        );
        game.apply(PlayerId::One, action).unwrap();
        let spell = game
            .stack
            .iter()
            .find(|o| o.kind == StackObjectKind::Spell)
            .unwrap();
        let view = game.stack_trigger_event_object(spell).unwrap();
        assert!(view.types.contains(CardType::Instant));
        assert!(!view.types.contains(CardType::Creature));
        assert_eq!(
            view.mana_value, 2,
            "the discount does not change mana value"
        );
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == mountain)
                .unwrap()
                .tapped
        );
    }
}

#[test]
fn adventure_pays_noncreature_tax_and_stale_discount_is_rejected() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::BONECRUSHER_GIANT, cards::THORN_OF_AMETHYST);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        assert!(
            cast_option(&game, id, PlayOptionId(1)).is_none(),
            "Stomp owes Thorn's tax"
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let action = cast_option(&game, id, PlayOptionId(1)).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());

        let (mut game, id) = setup(
            prepared,
            cards::BONECRUSHER_GIANT,
            cards::GOBLIN_ELECTROMANCER,
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        let action = cast_option(&game, id, PlayOptionId(1)).unwrap();
        let source = game.battlefield[0].card.id;
        game.destroy_permanent(source);
        let before = checkpoint_fixture(&game, PlayerId::One);
        assert!(game.apply(PlayerId::One, action).is_err());
        assert_eq!(checkpoint_fixture(&game, PlayerId::One), before);
    }
}

#[test]
fn bestow_costs_match_an_aura_instead_of_a_creature() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::SPRINGHEART_NANTUKO, cards::PLANAR_GATE);
        game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        let casts = game.legal_actions(PlayerId::One);
        assert!(
            casts
                .iter()
                .any(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
        );
        assert!(!casts.iter().any(|a| matches!(a, Action::CastSpell { card, choices, .. } if *card == id && choices.costs().alternative().is_some())), "Planar Gate cannot discount bestow");
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let bestow = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { card, choices, .. } if *card == id && choices.costs().alternative().is_some())).unwrap();
        game.apply(PlayerId::One, bestow).unwrap();
        let spell = game
            .stack
            .iter()
            .find(|o| o.kind == StackObjectKind::Spell)
            .unwrap();
        let view = game.stack_trigger_event_object(spell).unwrap();
        assert_eq!(
            view.subtypes,
            crate::card::SubtypeSet::from_names(&["Aura"])
        );
        assert_eq!((view.power, view.toughness), (None, None));
        assert!(!view.types.contains(CardType::Creature));
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn face_down_cast_does_not_receive_printed_angel_discount() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::EXALTED_ANGEL, cards::HERALD_OF_WAR);
        game.battlefield[0].add_counters(CounterKind::PlusOnePlusOne, 3);
        assert!(
            cast_option(&game, id, PlayOptionId::DEFAULT).is_none(),
            "face-down spell is not an Angel"
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
        let action = cast_option(&game, id, PlayOptionId::DEFAULT).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        let spell = game
            .stack
            .iter()
            .find(|o| o.kind == StackObjectKind::Spell)
            .unwrap();
        let view = game.stack_trigger_event_object(spell).unwrap();
        assert!(view.subtypes.is_empty());
        assert_eq!(view.mana_value, 0);
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

fn modifier_fixture(game: &mut Game, effect: EffectDef) {
    rules_fixture(
        game,
        &CardRules::new_enchantment(mana_cost!("{0}"))
            .with_ability(AbilityDef::static_ability("Spell cost modifier.", effect)),
    );
}

fn rules_fixture(game: &mut Game, rules: &CardRules) -> GameObjectId {
    let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000903");
    let definition = CardDefinition::new(
        id,
        "Spell view modifier fixture",
        crate::card::sets::bloomburrow::SET,
        *rules,
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
    game.put_onto_battlefield(PlayerId::One, id).unwrap()
}

#[test]
fn chosen_x_controls_discounts_without_excluding_larger_legal_choices() {
    for prepared in [false, true] {
        for (predicate, expected_max) in [
            (ObjectPredicateDef::ManaValueAtMost(2), 1),
            (
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(2)),
                2,
            ),
        ] {
            let (mut game, id) = setup(prepared, cards::WALKING_BALLISTA, cards::MOUNTAIN);
            modifier_fixture(
                &mut game,
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    predicate,
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            );
            game.battlefield
                .retain(|p| p.card.definition != cards::MOUNTAIN);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
            let action = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .filter(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
                .max_by_key(|a| match a {
                    Action::CastSpell { choices, .. } => choices.x(),
                    _ => 0,
                })
                .unwrap();
            let Action::CastSpell { choices, .. } = &action else {
                unreachable!()
            };
            assert_eq!(choices.x(), expected_max);
            game.apply(PlayerId::One, action).unwrap();
            let spell = game
                .stack
                .iter()
                .find(|o| o.kind == StackObjectKind::Spell)
                .unwrap();
            assert_eq!(
                game.stack_trigger_event_object(spell).unwrap().mana_value,
                expected_max * 2
            );
        }
    }
}

#[test]
fn alternative_cost_uses_adventure_form_and_keeps_origin_zone_restriction() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::BONECRUSHER_GIANT, cards::MOUNTAIN);
        game.battlefield.clear();
        modifier_fixture(
            &mut game,
            EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                spell: ObjectPredicateDef::HasType(CardType::Instant),
                caster: PlayerRelation::You,
                zones: &[ZoneKind::Hand],
                costs: &[CostDef::Mana(mana_cost!("{0}"))],
            }),
        );
        let action =
            cast_option(&game, id, PlayOptionId(1)).expect("Stomp has the free alternative");
        assert!(cast_option(&game, id, PlayOptionId::DEFAULT).is_none());
        let mut from_exile = game.clone();
        let held = from_exile.players[0].hand.pop().unwrap();
        from_exile.players[0].exile.push(held);
        from_exile.permit_conditional_cast_while_exiled(id, PlayerId::One);
        assert!(
            cast_option(&from_exile, id, PlayOptionId(1)).is_none(),
            "the hand-only offer cannot follow the card to exile"
        );
        game.apply(PlayerId::One, action).unwrap();
    }
}

#[test]
fn an_x_dependent_tax_does_not_hide_larger_affordable_casts() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::WALKING_BALLISTA, cards::MOUNTAIN);
        game.battlefield.clear();
        modifier_fixture(
            &mut game,
            EffectDef::ModifyCost(CostModificationDef::increase_spell(
                ObjectPredicateDef::ManaValueAtMost(0),
                PlayerRelation::You,
                mana_cost!("{2}"),
            )),
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { card, choices, .. } if *card == id && choices.x() == 1)).expect("X = 1 removes the tax");
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn payment_locks_discount_but_rechecks_the_source_of_storm() {
    for prepared in [false, true] {
        let (mut game, id) = setup(prepared, cards::ALTARS_REAP, cards::MOUNTAIN);
        game.battlefield.clear();
        let source = rules_fixture(
            &mut game,
            &CardRules::new_creature(mana_cost!("{0}"), &[], 1, 1).with_abilities(
                &const {
                    [
                        AbilityDef::static_ability(
                            "Instant spells cost less.",
                            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                                ObjectPredicateDef::HasType(CardType::Instant),
                                PlayerRelation::You,
                                ValueDef::Constant(1),
                            )),
                        ),
                        AbilityDef::static_ability(
                            "Instant spells have storm.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::matching_objects(
                                    ObjectPredicateDef::HasType(CardType::Instant),
                                    &[ZoneKind::Stack],
                                    PlayerRelation::You,
                                ),
                                effect: AppliedEffectDef::add_ability(
                                    &const { abilities::storm() },
                                ),
                            },
                        ),
                    ]
                },
            ),
        );
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { card, sacrifices, .. } if *card == id && sacrifices.contains(&source))).expect("the source discounts the spell before it is sacrificed");
        game.apply(PlayerId::One, action).unwrap();
        assert!(!game.battlefield.iter().any(|p| p.card.id == source));
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
        assert_eq!(
            game.stack.len(),
            1,
            "the storm grant is gone when casting completes"
        );
        assert_eq!(game.stack[0].kind, StackObjectKind::Spell);
    }
}
