use super::*;
use crate::poc::{self, cards};
use crate::{
    AbilityTargetDef, AbilityTargetPredicate, AdditionalCostDef, AdditionalCostId,
    AlternativeCostDef, AlternativeCostId, CardComposition, CardDefinition, CardEffectStatus,
    CardInstanceId, CardPart, CardPartId, CardPrinting, CardRules, CardStructure, CastChoices,
    DoubleFacedKind, LandEntry, ManaRestrictionDef, ManaSpendEffectDef, ModeDef, ModeSetDef,
    PlayOptionDef, PlayOptionId, PlayerRelation, SpellForm, StackObjectId, TargetPredicate,
    TargetSelection, TargetSlotDef, TargetSlotId,
};

fn ready_game() -> Game {
    let deck = poc::mono_red_atog();
    let mut game = Game::new(poc::catalog().unwrap(), [deck.clone(), deck], 0).unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.stack.clear();
    game.pending_decisions.clear();
    game.pending_combat_attackers.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.exile.clear();
        player.life = i16::from(rules::STARTING_LIFE);
        player.mana_pool = ManaPool::default();
        player.mana.clear();
    }
    game
}

fn card(id: u32, definition: CardDefinitionId, owner: PlayerId) -> CardInstance {
    CardInstance {
        id: CardInstanceId(id),
        definition,
        owner,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(id)]),
        characteristics: CharacteristicSource::Card(definition),
    }
}

fn creature(id: u32, definition: CardDefinitionId, controller: PlayerId) -> Permanent {
    Permanent {
        card: card(id, definition, controller),
        presented: CardPartId::PRIMARY,
        controller,
        tapped: false,
        entered_controller_turn: 0,
        damage: 0,
        power_bonus: 0,
        toughness_bonus: 0,
        attacking: false,
        blocking: None,
        chosen_player: None,
        destroy_at_end: false,
        flying_until_end: false,
        factory_animated: false,
        dragon_whelp_activations: 0,
        plus_one_counters: 0,
        javelin_counters: 0,
        dealt_deathtouch_damage: false,
        exile_instead_of_dying: false,
        combat_damage_assignment: Vec::new(),
        copied_behavior: None,
        regeneration_shields: 0,
        trample_until_end: false,
        berserked: false,
        attacked_this_turn: false,
        forestwalk_until_upkeep_of: None,
    }
}

fn cast_choices(targets: Vec<Target>, x: u16) -> CastChoices {
    let choices = CastChoices::default().with_x(x);
    if targets.is_empty() {
        choices
    } else {
        choices.with_targets(vec![TargetSelection::new(TargetSlotId(0), targets)])
    }
}

fn cast_action(
    card: GameObjectId,
    targets: Vec<Target>,
    sacrifices: Vec<GameObjectId>,
    x: u16,
) -> Action {
    Action::CastSpell {
        card,
        choices: cast_choices(targets, x),
        sacrifices,
    }
}

fn synchronize_single_part_definition(definition: &mut CardDefinition) {
    let composition = CardComposition::single(definition.name.clone(), definition.rules);
    definition.parts = composition.parts;
    definition.structure = composition.structure;
    definition.play_options = composition.play_options;
}

fn spell(id: u32, definition: CardDefinitionId, controller: PlayerId, x: u16) -> StackObject {
    StackObject {
        id: StackObjectId(id),
        kind: StackObjectKind::Spell,
        card: card(id, definition, controller),
        source: None,
        ability: None,
        ability_text: None,
        controller,
        signature: Some(CastSignature::from_validated_choices(
            SpellForm::Part(CardPartId::PRIMARY),
            cast_choices(Vec::new(), x),
        )),
        ability_targets: Vec::new(),
        ability_target_selections: Vec::new(),
        triggered_target_defs: &[],
        chosen_permanents: Vec::new(),
        triggered_effect: None,
        trigger_context: None,
        applied_effects: Vec::new(),
        is_copy: false,
    }
}

fn spell_with_targets(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    targets: Vec<Target>,
    x: u16,
) -> StackObject {
    let mut object = spell(id, definition, controller, x);
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(targets, x),
    ));
    object
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

#[test]
fn a_physical_card_gets_new_object_identity_in_each_cast_zone() {
    let mut game = ready_game();
    let card = card(10_000, cards::TRISKELION, PlayerId::One);
    let hand_id = card.id;
    let physical = backing_cards(&card.backing);
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(hand_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let spell_id = game.stack[0].id;
    assert_ne!(spell_id, hand_id);
    assert_eq!(backing_cards(&game.stack[0].card.backing), physical);

    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    assert_ne!(permanent.card.id, spell_id);
    assert_ne!(permanent.card.id, hand_id);
    assert_eq!(backing_cards(&permanent.card.backing), physical);
}

#[test]
fn a_forked_spell_has_new_identity_and_no_physical_backing() {
    let mut game = ready_game();
    let original = spell(77, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let original_id = original.id;

    game.push_copy(original, PlayerId::One, Vec::new());

    let copied = game.stack.last().unwrap();
    assert_ne!(copied.id, original_id);
    assert_eq!(copied.card.backing, ObjectBacking::None);
    assert_eq!(
        copied.card.characteristics,
        CharacteristicSource::Copy(cards::LIGHTNING_BOLT)
    );
    assert_eq!(copied.card.owner, PlayerId::One);
    assert!(copied.is_copy);
}

#[test]
fn physical_card_metadata_is_separate_from_live_objects() {
    let game = ready_game();
    let physical = game.physical_cards[0].clone();
    assert_eq!(
        game.physical_card_definition(physical.id),
        Some(physical.definition)
    );
    assert_eq!(game.physical_card_owner(physical.id), Some(physical.owner));
}

#[test]
fn spell_events_keep_stack_identity_and_definition_after_the_card_moves() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let hand_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.players[0].mana_pool.red = 1;
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        cast_action(hand_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    let stack_id = game.stack[0].id;
    assert_ne!(stack_id, hand_id);
    assert!(game.events[event_start..].contains(&GameEvent::SpellCast {
        player: PlayerId::One,
        card: stack_id,
        definition: cards::LIGHTNING_BOLT,
        targets: vec![Target::Player(PlayerId::Two)],
    }));

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::SpellResolved {
            card: stack_id,
            definition: cards::LIGHTNING_BOLT,
        })
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT && card.id != stack_id),
        "the event still names the former stack object after the card became a new object",
    );
}

#[test]
fn ability_events_distinguish_the_stack_object_from_a_source_that_left_play() {
    let mut game = ready_game();
    let strip = creature(10_000, cards::STRIP_MINE, PlayerId::One);
    let target = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let source_id = strip.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![strip, target];
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: source_id,
            target: Some(Target::Permanent(target_id)),
            sacrifice: Some(source_id),
        },
    )
    .unwrap();
    let ability_id = game.stack[0].id;
    assert_ne!(ability_id, source_id);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id),
        "the source has already left play when its activation is logged",
    );
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityActivated {
            player: PlayerId::One,
            object: ability_id,
            source: source_id,
            definition: cards::STRIP_MINE,
            chosen_permanents: vec![target_id],
        })
    );

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityResolved {
            object: ability_id,
            source: source_id,
            definition: cards::STRIP_MINE,
        })
    );
}

#[test]
fn recall_charges_two_generic_mana_for_each_x() {
    let cost = CardBehavior::Recall.mana_cost();
    assert!(can_pay(
        ManaPool {
            blue: 1,
            colorless: 6,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
    assert!(!can_pay(
        ManaPool {
            blue: 1,
            colorless: 5,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
}

#[test]
fn white_red_hybrid_symbols_accept_either_color_but_not_colorless() {
    let cost = ManaCost::white_red_hybrid(3);
    assert!(can_pay(
        ManaPool {
            white: 2,
            red: 1,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(can_pay(
        ManaPool {
            red: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(!can_pay(
        ManaPool {
            colorless: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));

    let mut pool = ManaPool {
        white: 2,
        red: 1,
        ..ManaPool::default()
    };
    pay_cost(&mut pool, cost, 0);
    assert_eq!(pool, ManaPool::default());
}

#[test]
fn declarative_mana_production_drives_generic_mana_sources() {
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test dual land",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(CardKind::Land, ManaCost::default(), "Tap: Add U or R.")
        .produces([false, true, false, true, false, false]);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    assert_eq!(
        game.mana_colors(&game.battlefield[0]),
        vec![ManaColor::Blue, ManaColor::Red]
    );
    game.activate_mana_source(PlayerId::One, CardInstanceId(10_000), ManaColor::Blue);
    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn declarative_land_entry_handles_check_tapped_and_shock_lands() {
    let check_id = CardDefinitionId(10_000);
    let gate_id = CardDefinitionId(10_001);
    let shock_id = CardDefinitionId(10_002);
    let mut check = CardDefinition::new(
        check_id,
        "Test check land",
        CardSet::Magic2013,
        false,
        CardBehavior::Unsupported,
    );
    check.rules = CardRules::new(CardKind::Land, ManaCost::default(), "").land_entry(
        LandEntry::TappedUnlessControlsLandType([true, false, false, false, false]),
    );
    synchronize_single_part_definition(&mut check);
    let mut gate = CardDefinition::new(
        gate_id,
        "Test gate",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    gate.rules =
        CardRules::new(CardKind::Land, ManaCost::default(), "").land_entry(LandEntry::Tapped);
    synchronize_single_part_definition(&mut gate);
    let mut shock = CardDefinition::new(
        shock_id,
        "Test shock land",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    shock.rules = CardRules::new(CardKind::Land, ManaCost::default(), "")
        .land_types([true, false, true, false, false])
        .land_entry(LandEntry::PayLifeOrTapped(2));
    synchronize_single_part_definition(&mut shock);

    let plains = CardDefinition::new(
        cards::PLAINS,
        "Plains",
        CardSet::Alpha,
        true,
        CardBehavior::Plains,
    );
    let mut test_game = ready_game();
    test_game.catalog = CardCatalog::new([check, gate, shock, plains]).unwrap();
    test_game
        .battlefield
        .push(creature(9_999, cards::PLAINS, PlayerId::One));

    for (instance, definition) in [(10_000, check_id), (10_001, gate_id), (10_002, shock_id)] {
        test_game.players[0]
            .hand
            .push(card(instance, definition, PlayerId::One));
        test_game.play_land(
            PlayerId::One,
            CardInstanceId(instance),
            PlayOptionId::DEFAULT,
        );
    }

    assert!(!test_game.battlefield[1].tapped);
    assert!(test_game.battlefield[2].tapped);
    assert!(test_game.battlefield[3].tapped);
    assert_eq!(test_game.players[0].life, 20);
}

#[test]
fn a_land_play_option_locks_the_presented_part_on_the_permanent() {
    let definition_id = CardDefinitionId(10_100);
    let land_part = CardPartId(1);
    let land_option = PlayOptionId(1);
    let front_rules = CardRules::new(CardKind::Sorcery, ManaCost::new(1, 0), "Test front");
    let land_rules = CardRules::new(CardKind::Land, ManaCost::default(), "Test back")
        .land_entry(LandEntry::Tapped);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(land_part, "Test back", land_rules).without_mana_cost(),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: land_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules.mana_cost,
            CardEffectStatus::MetadataOnly,
        ),
        PlayOptionDef::play_land(
            land_option,
            "Play Test back",
            land_part,
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_100, definition_id, PlayerId::One);
    let action = Action::PlayLand {
        card: card.id,
        option: land_option,
    };
    game.players[0].hand.push(card);

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.battlefield[0].presented, land_part);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn a_modal_spell_resolves_by_its_locked_part_instead_of_the_canonical_front() {
    let definition_id = CardDefinitionId(10_150);
    let creature_part = CardPartId(1);
    let creature_option = PlayOptionId(1);
    let front_rules = CardRules::new(CardKind::Instant, ManaCost::new(1, 1), "Test front");
    let creature_rules = CardRules::new(
        CardKind::Creature,
        ManaCost::new(0, 0),
        "Test creature back",
    )
    .creature(3, 4)
    .flying();
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal spell",
        CardSet::Magic2014,
        false,
        CardBehavior::LightningBolt,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(creature_part, "Test creature back", creature_rules),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: creature_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules.mana_cost,
            CardEffectStatus::MetadataOnly,
        ),
        PlayOptionDef::cast(
            creature_option,
            "Cast Test creature back",
            SpellForm::Part(creature_part),
            creature_rules.mana_cost,
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_150, definition_id, PlayerId::One);
    let hand_id = card.id;
    game.players[0].hand.push(card);
    let action = Action::CastSpell {
        card: hand_id,
        choices: CastChoices::new(creature_option),
        sacrifices: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    let spell_id = game.stack[0].id;
    pass_priority_pair(&mut game);

    let permanent = &game.battlefield[0];
    assert_ne!(permanent.card.id, spell_id);
    assert_eq!(permanent.presented, creature_part);
    assert_eq!(game.power(permanent), Some(3));
    assert_eq!(game.toughness(permanent), Some(4));
    assert!(game.has_flying(permanent));
}

#[test]
fn changing_a_permanents_presented_face_keeps_its_object_identity() {
    let definition_id = CardDefinitionId(10_101);
    let back = CardPartId(1);
    let front_rules =
        CardRules::new(CardKind::Creature, ManaCost::new(2, 0), "Front-face rules.").creature(2, 2);
    let back_rules = CardRules::new(CardKind::Creature, ManaCost::default(), "Back-face rules.")
        .creature(4, 5)
        .flying()
        .trample();
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Werewolf",
        CardSet::Innistrad,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test Werewolf", front_rules),
        CardPart::new(back, "Test Ravager", back_rules).without_mana_cost(),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back,
        kind: DoubleFacedKind::Transforming,
    };
    definition.play_options = vec![PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Cast Test Werewolf",
        SpellForm::Part(CardPartId::PRIMARY),
        front_rules.mana_cost,
        CardEffectStatus::MetadataOnly,
    )];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let permanent_id = GameObjectId(10_101);
    game.battlefield
        .push(creature(permanent_id.0, definition_id, PlayerId::One));

    let front = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(front.id, permanent_id);
    assert_eq!(front.presented, CardPartId::PRIMARY);
    assert_eq!(
        (front.power, front.toughness, front.flying),
        (Some(2), Some(2), false)
    );

    game.battlefield[0].presented = back;

    let transformed = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(transformed.id, permanent_id);
    assert_eq!(transformed.presented, back);
    assert_eq!(
        (transformed.power, transformed.toughness, transformed.flying),
        (Some(4), Some(5), true),
    );
    assert!(game.has_trample(&game.battlefield[0]));

    game.return_permanent_to_hand(permanent_id);
    let returned_id = game.players[0].hand[0].id;
    assert_ne!(returned_id, permanent_id);
}

#[test]
fn city_in_a_bottle_uses_canonical_origin_even_when_a_reprint_exists() {
    let city = CardDefinition::new(
        CardDefinitionId(10_000),
        "City in a Bottle",
        CardSet::ArabianNights,
        false,
        CardBehavior::CityInABottle,
    );
    let kird_ape = CardDefinition::new(
        CardDefinitionId(10_001),
        "Kird Ape",
        CardSet::ArabianNights,
        false,
        CardBehavior::KirdApe,
    );
    let mut game = ready_game();
    game.catalog = CardCatalog::with_additional_printings(
        [city, kird_ape],
        [CardPrinting::new(
            CardDefinitionId(10_001),
            CardSet::Magic2014,
        )],
    )
    .unwrap();
    game.battlefield
        .push(creature(10_000, CardDefinitionId(10_000), PlayerId::One));
    game.battlefield
        .push(creature(10_001, CardDefinitionId(10_001), PlayerId::Two));

    game.handle_upkeep_triggers();

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(
        game.battlefield[0].card.definition,
        CardDefinitionId(10_000)
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        CardDefinitionId(10_001)
    );
}

#[test]
fn metadata_only_noncreature_spells_are_hidden_but_baseline_cards_remain_playable() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::DOOM_BLADE, PlayerId::One),
        card(10_001, crate::card::cards::PITHING_NEEDLE, PlayerId::One),
        card(10_002, crate::card::cards::DOMRI_RADE, PlayerId::One),
        card(10_003, crate::card::cards::LOXODON_SMITER, PlayerId::One),
        card(10_004, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        card(10_005, crate::card::cards::IZZET_CHARM, PlayerId::One),
        card(10_006, crate::card::cards::TURN_BURN, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        white: 4,
        blue: 4,
        black: 4,
        red: 4,
        green: 4,
        colorless: 4,
    };

    let actions = game.legal_actions(PlayerId::One);
    let cast_cards = actions
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(*card),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(cast_cards, vec![CardInstanceId(10_003)]);
    assert!(actions.contains(&Action::PlayLand {
        card: CardInstanceId(10_004),
        option: PlayOptionId::DEFAULT,
    }));
}

#[test]
#[allow(clippy::too_many_lines)]
fn cast_validation_rejects_unrecognized_structured_choices() {
    let definition_id = CardDefinitionId(10_200);
    let option_id = PlayOptionId(7);
    let implemented_mode = ModeId(2);
    let metadata_mode = ModeId(3);
    let slot_id = TargetSlotId(5);
    let alternative_id = AlternativeCostId(11);
    let additional_id = AdditionalCostId(13);
    let mut definition = CardDefinition::new(
        definition_id,
        "Structured Bolt",
        CardSet::Alpha,
        false,
        CardBehavior::LightningBolt,
    );
    let mut option = PlayOptionDef::cast(
        option_id,
        "Cast Structured Bolt",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(0, 1),
        CardEffectStatus::Implemented,
    )
    .with_modes(ModeSetDef {
        minimum: 1,
        maximum: 2,
        may_repeat: false,
        modes: vec![
            ModeDef {
                id: implemented_mode,
                label: "Target a player".into(),
                targets: vec![TargetSlotDef::exactly_one(
                    slot_id,
                    "target player",
                    TargetPredicate::Player,
                )],
                effect_status: CardEffectStatus::Implemented,
            },
            ModeDef {
                id: metadata_mode,
                label: "Not implemented".into(),
                targets: Vec::new(),
                effect_status: CardEffectStatus::MetadataOnly,
            },
        ],
    });
    option.alternative_costs = vec![AlternativeCostDef {
        id: alternative_id,
        label: "Alternative cost".into(),
        mana_cost: ManaCost::new(1, 0),
    }];
    option.additional_costs = vec![AdditionalCostDef {
        id: additional_id,
        label: "Additional cost".into(),
        mana_cost: Some(ManaCost::new(2, 0)),
    }];
    definition.play_options = vec![option];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_200, definition_id, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 20;

    let valid = CastChoices::new(option_id)
        .with_modes(vec![implemented_mode])
        .with_costs(CostConfiguration::new(
            Some(alternative_id),
            vec![additional_id],
        ))
        .with_targets(vec![TargetSelection::single(
            slot_id,
            Target::Player(PlayerId::Two),
        )]);
    let (signature, cost, _) = game
        .validated_cast_signature(PlayerId::One, card_id, &valid)
        .expect("all structured choices are recognized and payable");
    assert_eq!(signature.play_option(), option_id);
    assert_eq!(signature.form(), &SpellForm::Part(CardPartId::PRIMARY));
    assert_eq!(signature.modes(), &[implemented_mode]);
    assert_eq!(signature.costs(), valid.costs());
    assert_eq!(signature.targets(), valid.targets());
    assert_eq!(cost, ManaCost::new(3, 0));

    let invalid = [
        CastChoices::new(PlayOptionId(99)),
        CastChoices::new(option_id),
        CastChoices::new(option_id).with_modes(vec![metadata_mode]),
        CastChoices::new(option_id).with_modes(vec![implemented_mode, implemented_mode]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(
                Some(AlternativeCostId(99)),
                Vec::new(),
            )),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(None, vec![AdditionalCostId(99)])),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_x(1),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                TargetSlotId(99),
                Target::Player(PlayerId::Two),
            )]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                slot_id,
                Target::Permanent(GameObjectId(99_999)),
            )]),
    ];
    for choices in invalid {
        assert!(
            game.validated_cast_signature(PlayerId::One, card_id, &choices)
                .is_none(),
            "invalid structured choices were accepted: {choices:?}",
        );
    }
}

#[test]
fn declarative_dual_lands_cast_and_resolve_a_hybrid_creature() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        creature(10_001, crate::card::cards::SACRED_FOUNDRY, PlayerId::One),
        creature(10_002, crate::card::cards::SUNPETAL_GROVE, PlayerId::One),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::BOROS_RECKONER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("three declarative dual lands can pay {R/W}{R/W}{R/W}");
    assert_eq!(game.mana_sources_for_action(PlayerId::One, &cast).len(), 3);

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    pass_priority_pair(&mut game);

    let reckoner = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::BOROS_RECKONER)
        .unwrap();
    assert_eq!(game.power(reckoner), Some(3));
    assert_eq!(game.toughness(reckoner), Some(3));
}

#[test]
fn flexible_mana_plan_reserves_the_only_green_source_for_a_multicolor_spell() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::TEMPLE_GARDEN, PlayerId::One),
        creature(10_001, crate::card::cards::GODLESS_SHRINE, PlayerId::One),
        creature(
            10_002,
            crate::card::cards::ENCROACHING_WASTES,
            PlayerId::One,
        ),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::LOXODON_SMITER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("Godless Shrine can make white while Temple Garden makes green");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &cast),
        vec![
            CardInstanceId(10_001),
            CardInstanceId(10_000),
            CardInstanceId(10_002),
        ],
    );

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn metadata_only_flash_creatures_keep_their_printed_cast_timing() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.step = Step::End;
    game.players[0].mana_pool = ManaPool {
        white: 1,
        colorless: 3,
        ..ManaPool::default()
    };
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::RESTORATION_ANGEL, PlayerId::One),
        card(10_001, crate::card::cards::LOXODON_SMITER, PlayerId::One),
    ]);

    let cast_cards = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(card),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(cast_cards, vec![CardInstanceId(10_000)]);
}

#[test]
fn city_of_brass_produces_any_color_then_uses_the_stack_for_damage() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));

    game.activate_mana_source(PlayerId::One, CardInstanceId(10_000), ManaColor::Blue);

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].life, 20);
    assert!(game.stack.is_empty());
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 19);
}

#[test]
fn trigger_placement_preserves_the_nonactive_players_priority() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::Two));

    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    assert_eq!(game.priority, PlayerId::Two);
    game.apply(
        PlayerId::Two,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            color: ManaColor::Blue,
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.priority, PlayerId::Two);
    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn ankh_trigger_can_be_answered_by_bolt_before_it_resolves() {
    let mut game = ready_game();
    game.players[0].life = 2;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::Two));
    let mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0]
        .hand
        .extend([mountain.clone(), bolt.clone()]);

    let play_land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play_land).unwrap();

    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    assert_eq!(game.stack[0].ability, Some(crate::AbilityId::PRIMARY));

    let cast_bolt = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast_bolt));
    game.apply(PlayerId::One, cast_bolt).unwrap();
    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);

    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1, "Ankh never got to resolve");
}

#[test]
fn city_trigger_can_be_answered_when_mana_was_floated_first() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    let city = creature(10_000, cards::CITY_OF_BRASS, PlayerId::One);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.battlefield.push(city);
    game.players[0].hand.push(bolt.clone());

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            color: ManaColor::Red,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.players[0].life, 1);

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
}

#[test]
fn city_trigger_is_above_a_spell_when_city_pays_during_casting() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());

    let cast = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack[0].kind, StackObjectKind::Spell);
    assert_eq!(game.stack[1].kind, StackObjectKind::TriggeredAbility);
    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[1].life, 3, "Bolt never resolved");
}

#[test]
fn a_resolving_tap_effect_uses_the_same_city_trigger_path() {
    let mut game = ready_game();
    game.players[0].mana_pool.colorless = 1;
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::Two),
    ]);
    let activation = Action::ActivateAbility {
        source: CardInstanceId(10_000),
        target: Some(Target::Permanent(CardInstanceId(10_001))),
        sacrifice: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield[1].tapped);
    assert_eq!(game.players[1].life, 20);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_001)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn controller_chooses_resolution_order_for_simultaneous_triggers() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::One),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .unwrap();
    game.apply(PlayerId::One, play).unwrap();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerOrder);
    assert_eq!(
        decision.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    assert!(decision.options.iter().all(|option| {
        option
            .ability_text
            .as_deref()
            .is_some_and(|text| text.contains("Whenever a land enters"))
    }));
    let first = decision.options[0].id;
    let second = decision.options[1].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![second, first],
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack.last().unwrap().source,
        Some(CardInstanceId(10_001))
    );
    assert!(game.stack.iter().all(|object| {
        object.ability == Some(crate::AbilityId::PRIMARY) && object.ability_text.is_some()
    }));
}

#[test]
fn targeted_trigger_chooses_public_targets_while_being_put_on_stack() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(7),
        "target creature an opponent controls",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Creature,
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )];
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::Two),
    ]);
    game.capture_trigger(TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_000),
            ability: crate::AbilityId::PRIMARY,
        },
        definition: cards::ANKH_OF_MISHRA,
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal 2 damage to target creature an opponent controls.",
        target_defs: &TARGETS,
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(7)),
            amount: ValueDef::Constant(2),
        },
        context: TriggerContext {
            object: None,
            player: None,
            amount: None,
        },
    });
    game.finish_rules_procedure();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerPlacement);
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    assert_eq!(decision.options.len(), 1);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(
        game.stack[0].targets(),
        vec![Target::Permanent(CardInstanceId(10_001))]
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield[1].damage, 2);
}

#[test]
fn su_chi_mana_and_source_power_use_ordinary_stack_and_lki() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SU_CHI, PlayerId::One));
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.players[0].mana_pool.colorless, 0);
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.colorless, 4);

    let mut game = ready_game();
    let mut source = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::One);
    source.power_bonus = 3;
    game.battlefield.push(source);
    game.capture_trigger(TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_010),
            ability: crate::AbilityId::PRIMARY,
        },
        definition: cards::SAVANNAH_LIONS,
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal damage equal to this creature's power.",
        target_defs: &[],
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        },
        context: TriggerContext {
            object: Some(CardInstanceId(10_010)),
            player: Some(PlayerId::One),
            amount: None,
        },
    });
    game.destroy_permanent(CardInstanceId(10_010));
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15, "last known power was five");
}

#[test]
fn workshop_mana_is_three_individual_restricted_values() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MISHRA_S_WORKSHOP, PlayerId::One));
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            color: ManaColor::Colorless,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 3);
    assert_eq!(game.players[0].mana.len(), 3);
    assert!(game.players[0].mana.iter().all(|mana| {
        mana.color == ManaColor::Colorless
            && mana.source
                == Some(ManaSource {
                    object: CardInstanceId(10_000),
                    ability: crate::AbilityId::PRIMARY,
                })
            && mana.restrictions == [ManaRestrictionDef::CastSpell(ObjectPredicateDef::Artifact)]
    }));
}

#[test]
fn explicitly_tagged_triggered_mana_ability_resolves_without_the_stack() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_mana(
        crate::AbilityId::PRIMARY,
        "Whenever this becomes tapped, add {C}.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
        EffectDef::AddMana(AddManaEffectDef::one(ManaKindDef::Colorless)),
    )];
    let definition_id = CardDefinitionId(10_050);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test triggered mana source",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new(
        CardKind::Artifact,
        ManaCost::new(0, 0),
        "Whenever this becomes tapped, add {C}.",
    )
    .with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_050, definition_id, PlayerId::One));

    let _ = game.tap_permanent(CardInstanceId(10_050));

    assert_eq!(game.players[0].mana_pool.colorless, 1);
    assert_eq!(game.players[0].mana.len(), 1);
    assert!(game.pending_triggers.is_empty());
    assert!(game.stack.is_empty());
}

#[test]
fn a_mana_spend_rider_attaches_to_the_paid_spell_with_its_source() {
    static RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
        crate::AppliedEffectDef::CannotBeCountered,
    )];
    let mut object = spell(77, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    let mana = Mana::from_ability(
        ManaColor::White,
        ManaSource {
            object: CardInstanceId(10_000),
            ability: crate::AbilityId(1),
        },
        &[],
        &RIDERS,
    );

    Game::apply_spent_mana_to_spell(&mut object, &[mana]);

    assert_eq!(object.applied_effects.len(), 1);
    assert_eq!(object.applied_effects[0].source, mana.source);
    assert_eq!(
        object.applied_effects[0].effect,
        crate::AppliedEffectDef::CannotBeCountered
    );
}

#[test]
fn crusade_buffs_white_creatures() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CRUSADE, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));

    assert_eq!(game.power(&game.battlefield[1]), Some(3));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(2));
}

#[test]
fn demonic_tutor_exposes_a_library_choice_then_shuffles() {
    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::JUZAM_DJINN, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let option = decision
        .options
        .iter()
        .find(|option| option.card == Some((CardInstanceId(10_001), cards::JUZAM_DJINN)))
        .unwrap();
    let choice = Action::ChooseDecision {
        decision: decision.id,
        options: vec![option.id],
    };
    game.apply(PlayerId::One, choice).unwrap();

    assert_eq!(game.players[0].hand[0].definition, cards::JUZAM_DJINN);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn a_search_may_fail_to_find_even_with_a_full_library() {
    // CR 701.19c: searching a hidden zone never obliges the searcher to find.
    // This is not cancelling the spell -- Demonic Tutor resolved, the search
    // happened, and it turned up nothing on purpose.
    let mut game = ready_game();
    for (index, definition) in [cards::JUZAM_DJINN, cards::BLACK_LOTUS]
        .into_iter()
        .enumerate()
    {
        let id = 10_001 + u32::try_from(index).unwrap();
        game.players[0]
            .library
            .push(card(id, definition, PlayerId::One));
    }
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 0, "a search is never compulsory");
    assert_eq!(decision.maximum, 1);
    assert!(
        !decision.cancellable,
        "failing to find is a resolution, not a way out of the spell"
    );

    let library_before = game.players[0].library.len();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("failing to find is a legal resolution");

    assert!(game.players[0].hand.is_empty(), "nothing was found");
    assert_eq!(
        game.players[0].library.len(),
        library_before,
        "and nothing left the library"
    );
    assert!(game.pending_decisions.is_empty(), "the search is over");
}

#[test]
fn the_handcrafted_policy_still_finds_when_it_may_decline() {
    // Failing to find became legal, and the policy takes `minimum` options by
    // default -- which is now zero. Left alone it would tutor for nothing
    // every single time, quietly turning Demonic Tutor into a blank.
    use crate::{HandcraftedPolicy, Policy};

    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::BLACK_LOTUS, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let mut policy = HandcraftedPolicy::new(poc::catalog().unwrap());
    let action = policy
        .choose_action(&game.observe(PlayerId::One))
        .expect("the policy answers the search");
    let Action::ChooseDecision { options, .. } = &action else {
        panic!("expected a decision, got {action:?}");
    };
    assert_eq!(options.len(), 1, "the policy searched and found a card");

    game.apply(PlayerId::One, action.clone()).expect("legal");
    assert_eq!(game.players[0].hand.len(), 1, "the card reached hand");
}

#[test]
fn a_search_shuffles_even_when_it_finds_nothing() {
    // Otherwise a player learns their own library order for free: tutor, fail
    // to find, and the top of the deck is whatever it already was.
    let mut game = ready_game();
    let before: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert!(
        before.len() > 10,
        "the deck's library is long enough for a shuffle to be observable"
    );

    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("failing to find is legal");

    let after: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert_eq!(
        before.len(),
        after.len(),
        "a failed search moves no cards, it only shuffles"
    );
    assert_ne!(
        before, after,
        "the library was shuffled despite finding nothing"
    );
}

#[test]
fn a_tutor_with_nothing_to_find_leaves_a_legal_action() {
    // An empty library used to produce a decision asking for exactly one of
    // zero options, and not cancellable. `is_legal` rejects a ChooseDecision
    // carrying fewer than `minimum` options, so no legal action existed and
    // the game deadlocked -- every policy stalls, having nothing to return.
    let mut game = ready_game();
    game.players[0].library.clear();
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    game.resolve_spell_effect(&tutor, CardBehavior::DemonicTutor);

    let observation = game.observe(PlayerId::One);
    if let Some(decision) = observation.decision.as_ref() {
        assert!(
            decision.minimum <= decision.options.len(),
            "a decision must never ask for more than it offers: \
             minimum={} options={}",
            decision.minimum,
            decision.options.len(),
        );
    }
    assert!(
        !observation.legal_actions.is_empty(),
        "an empty library must still leave the player something to do"
    );

    // The player resolves it by finding nothing, and the game moves on.
    let decision = observation.decision.expect("the tutor still asks");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("choosing nothing from nothing is legal");

    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].hand.is_empty(), "nothing was found");
}

#[test]
fn armageddon_destroys_every_land_but_not_creatures() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_001, cards::SWAMP, PlayerId::Two),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    let armageddon = spell(10_003, cards::ARMAGEDDON, PlayerId::One, 0);

    game.resolve_spell_effect(&armageddon, CardBehavior::Armageddon);

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SAVANNAH_LIONS);
}

#[test]
fn recall_uses_cancellable_cost_and_return_decisions() {
    let mut game = ready_game();
    game.players[0].hand.extend([
        card(10_000, cards::RECALL, PlayerId::One),
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_002, cards::BALANCE, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        colorless: 4,
        ..ManaPool::default()
    };

    game.cast_spell(
        PlayerId::One,
        CardInstanceId(10_000),
        cast_choices(Vec::new(), 2),
        &[],
    );
    let cost_decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(cost_decision.cancellable);
    assert_eq!(cost_decision.minimum, 2);
    let cost_action = Action::ChooseDecision {
        decision: cost_decision.id,
        options: cost_decision
            .options
            .iter()
            .take(cost_decision.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, cost_action).unwrap();
    assert_eq!(game.players[0].graveyard.len(), 2);

    pass_priority_pair(&mut game);
    let return_decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(!return_decision.cancellable);
    assert_eq!(return_decision.minimum, 2);
    let return_action = Action::ChooseDecision {
        decision: return_decision.id,
        options: return_decision
            .options
            .iter()
            .take(return_decision.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, return_action).unwrap();

    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].exile[0].definition, cards::RECALL);
}

#[test]
fn balance_requests_public_sacrifices_and_private_discards() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::PLAINS, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::SWAMP, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_004, cards::BALANCE, PlayerId::One),
    ]);
    game.players[1]
        .hand
        .push(card(10_005, cards::TERROR, PlayerId::Two));

    game.resolve_balance();
    assert_eq!(
        game.observe(PlayerId::Two).decision.unwrap().visibility,
        DecisionVisibility::Public
    );
    let decision_player = game.decision_player().unwrap();
    let pending_actions = game.legal_actions(decision_player);
    assert_eq!(pending_actions.len(), 2);
    assert!(matches!(
        &pending_actions[1],
        Action::ChooseDecision {
            decision: _,
            options
        } if options.is_empty()
    ));
    while let Some(player) = game.decision_player() {
        let Some(decision) = game.observe(player).decision else {
            break;
        };
        let action = Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        };
        game.apply(player, action).unwrap();
    }

    let land_counts = [PlayerId::One, PlayerId::Two].map(|player| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && game.permanent_kind(permanent) == Some(CardKind::Land)
            })
            .count()
    });
    assert_eq!(land_counts, [1, 1]);
    assert_eq!(game.players[0].hand.len(), game.players[1].hand.len());
}

#[test]
fn time_vault_can_untap_by_skipping_the_controllers_next_turn() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::TIME_VAULT, PlayerId::Two);
    vault.tapped = true;
    game.battlefield.push(vault);

    game.start_next_turn();
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let untap = Action::ChooseDecision {
        decision: decision.id,
        options: vec![1],
    };
    game.apply(PlayerId::Two, untap).unwrap();
    assert!(!game.battlefield[0].tapped);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
}

#[test]
fn sylvan_library_tracks_drawn_cards_and_resolves_each_choice() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    assert_eq!(game.players[0].hand.len(), 3);
    for mode in [1, 0] {
        let selection = game.observe(PlayerId::One).decision.unwrap();
        let select = Action::ChooseDecision {
            decision: selection.id,
            options: vec![selection.options[0].id],
        };
        game.apply(PlayerId::One, select).unwrap();
        let decision = game.observe(PlayerId::One).decision.unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![mode],
            },
        )
        .unwrap();
    }

    assert_eq!(game.players[0].life, 16);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn mana_vault_stays_tapped_and_can_be_paid_to_untap_at_upkeep() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 10_001..10_005 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.prompt, "Mana Vault would remain tapped");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::MANA_VAULT)
            .unwrap()
            .tapped
    );
}

#[test]
fn multiple_mana_vault_upkeep_choices_do_not_reuse_stale_mana() {
    let mut game = ready_game();
    for id in 10_000..10_002 {
        let mut vault = creature(id, cards::MANA_VAULT, PlayerId::One);
        vault.tapped = true;
        game.battlefield.push(vault);
    }
    for id in 10_002..10_006 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    let first = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: vec![1],
        },
    )
    .unwrap();

    let second = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(second.prompt, "Mana Vault would remain tapped");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: second.id,
            options: vec![1],
        },
    )
    .unwrap();

    let vaults: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MANA_VAULT)
        .map(|permanent| permanent.tapped)
        .collect();
    assert_eq!(vaults, vec![false, true]);
}

#[test]
fn tapped_mana_vault_deals_one_at_the_draw_step() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    game.step = Step::Upkeep;

    game.advance_step();

    assert_eq!(game.players[0].life, 19);
    assert_eq!(game.step, Step::Draw);
}

#[test]
fn juggernaut_must_attack_if_able() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let juggernaut = creature(10_000, cards::JUGGERNAUT, PlayerId::One);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);

    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::FinishDeclaringAttackers));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: juggernaut_id,
    }));

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: juggernaut_id,
        },
    )
    .unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers)
    );
}

#[test]
fn triskelion_enters_with_counters_and_spends_one_to_deal_damage() {
    let mut game = ready_game();
    let triskelion = card(10_000, cards::TRISKELION, PlayerId::One);
    let triskelion_id = triskelion.id;
    game.players[0].hand.push(triskelion);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(triskelion_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    let permanent_id = permanent.card.id;
    assert_eq!(game.power(permanent), Some(4));
    assert_eq!(game.toughness(permanent), Some(4));

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: permanent_id,
            target: Some(Target::Player(PlayerId::Two)),
            sacrifice: None,
        },
    )
    .unwrap();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == permanent_id)
        .unwrap();
    assert_eq!(game.power(permanent), Some(3));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn tundras_pay_counterspells_double_blue_cost() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_001, cards::COUNTERSPELL, PlayerId::One);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[0].hand.push(counterspell.clone());
    game.battlefield
        .push(creature(10_002, cards::TUNDRA, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::TUNDRA, PlayerId::One));
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let bolt_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(bolt_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[0].graveyard[0].definition, cards::COUNTERSPELL);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::LIGHTNING_BOLT
    );
}

#[test]
fn swords_exiles_a_creature_and_grants_life_equal_to_power() {
    let mut game = ready_game();
    let serra = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let serra_id = serra.card.id;
    game.battlefield.push(serra);
    let swords = card(10_001, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_action(swords.id, vec![Target::Permanent(serra_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[1].life, 24);
    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
}

#[test]
fn swords_cannot_target_order_of_the_ebon_hand() {
    let mut game = ready_game();
    let order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::Two);
    let order_id = order.card.id;
    game.battlefield.push(order);
    let swords = card(10_001, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;

    let swords_action = cast_action(swords.id, vec![Target::Permanent(order_id)], Vec::new(), 0);
    assert!(!game.legal_actions(PlayerId::One).contains(&swords_action));
}

#[test]
fn protection_from_white_prevents_white_blockers() {
    let mut game = ready_game();
    let mut order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::One);
    order.attacking = true;
    let lion = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    game.battlefield = vec![order, lion];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn ancestral_recall_draws_three_and_time_walk_queues_an_extra_turn() {
    let mut game = ready_game();
    let ancestral = card(10_000, cards::ANCESTRAL_RECALL, PlayerId::One);
    game.players[0].hand.push(ancestral.clone());
    game.players[0].mana_pool.blue = 1;
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        cast_action(
            ancestral.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before - 1 + 3);

    let time_walk = card(10_001, cards::TIME_WALK, PlayerId::One);
    game.players[0].hand.push(time_walk.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(time_walk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.observe(PlayerId::One).active_turn, 2);
}

#[test]
fn serra_angel_attacks_without_tapping() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let serra = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let serra_id = serra.card.id;
    game.battlefield.push(serra);

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker { attacker: serra_id },
    )
    .unwrap();

    let serra = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == serra_id)
        .unwrap();
    assert!(serra.attacking);
    assert!(!serra.tapped);
}

#[test]
fn ivory_tower_and_jayemdae_tome_provide_control_card_advantage() {
    let mut game = ready_game();
    game.players[0].life = 10;
    for id in 10_000..10_006 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.battlefield
        .push(creature(10_010, cards::IVORY_TOWER, PlayerId::One));
    let tome = creature(10_011, cards::JAYEMDAE_TOME, PlayerId::One);
    let tome_id = tome.card.id;
    game.battlefield.push(tome);
    game.players[0].mana_pool.colorless = 4;

    game.handle_upkeep_triggers();
    assert_eq!(game.players[0].life, 12);
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: tome_id,
            target: None,
            sacrifice: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);
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
    let fork = card(10_000, cards::FORK, PlayerId::One);
    game.players[0].hand.push(fork.clone());
    game.players[0].mana_pool.red = 2;
    game.stack.push(spell_with_targets(
        77,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
        vec![Target::Player(PlayerId::One)],
        0,
    ));

    game.apply(
        PlayerId::One,
        cast_action(
            fork.id,
            vec![Target::Spell(StackObjectId(77))],
            Vec::new(),
            0,
        ),
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
fn fork_copies_a_targetless_spell_immediately_and_preserves_its_signature() {
    let mut game = ready_game();
    let original = spell(77, cards::DARK_RITUAL, PlayerId::Two, 0);
    let signature = original.signature.clone().unwrap();

    game.queue_fork_decision(PlayerId::One, original);

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
    game.queue_fork_decision(
        PlayerId::One,
        spell_with_targets(77, cards::SHATTER, PlayerId::Two, vec![stale_target], 0),
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
    let mut factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::Two);
    factory.factory_animated = true;
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    let mut turn = spell(77, crate::card::cards::TURN_BURN, PlayerId::One, 0);
    turn.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::new(PlayOptionId::DEFAULT).with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(factory_id),
        )]),
    ));

    assert!(!game.spell_fizzles(&turn));
    game.battlefield[0].factory_animated = false;
    assert!(game.spell_fizzles(&turn));
}

#[test]
fn black_lotus_sacrifices_for_three_red_mana() {
    let mut game = ready_game();
    let lotus = creature(10_000, cards::BLACK_LOTUS, PlayerId::One);
    let lotus_id = lotus.card.id;
    game.battlefield.push(lotus);
    let action = Action::ActivateManaAbility {
        source: lotus_id,
        color: ManaColor::Red,
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
        target: Some(Target::Permanent(target_id)),
        sacrifice: Some(artifact_id),
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
    let mountain = creature(10_000, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);
    game.queue_iron_star_decision(PlayerId::One);
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
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
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

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: factory_id,
            target: None,
            sacrifice: None,
        },
    )
    .unwrap();
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == factory_id)
            .and_then(|permanent| game.power(permanent)),
        Some(2)
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: strip_id,
            target: Some(Target::Permanent(opposing_id)),
            sacrifice: Some(strip_id),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
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
        target: None,
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&animate));
    game.apply(PlayerId::One, animate).unwrap();

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
    let mut factory = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::Two);
    factory.factory_animated = true;
    let factory_id = factory.card.id;
    game.battlefield = vec![attacker, factory];
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
            target: Some(Target::Permanent(first_strip_id)),
            sacrifice: Some(second_strip_id),
        },
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    let response = Action::ActivateAbility {
        source: first_strip_id,
        target: Some(Target::Permanent(other_land_id)),
        sacrifice: Some(first_strip_id),
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
fn chaos_orb_uses_the_documented_deterministic_success_rule() {
    let mut game = ready_game();
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;
    let action = Action::ActivateAbility {
        source: orb_id,
        target: Some(Target::Permanent(target_id)),
        sacrifice: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
    assert_eq!(game.stack[0].chosen_permanents, vec![target_id]);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
    pass_priority_pair(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn chaos_orb_can_be_activated_the_turn_it_enters_using_untapped_mana() {
    let mut game = ready_game();
    let mut orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let mut mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let target = creature(10_002, cards::BLACK_VISE, PlayerId::Two);
    orb.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    mountain.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let orb_id = orb.card.id;
    let mountain_id = mountain.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, mountain, target];
    let action = Action::ActivateAbility {
        source: orb_id,
        target: Some(Target::Permanent(target_id)),
        sacrifice: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn icatian_javelineers_cannot_activate_until_their_controller_turn() {
    let mut game = ready_game();
    let mut javeliners = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    javeliners.javelin_counters = 1;
    javeliners.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let action = Action::ActivateAbility {
        source: javeliners.card.id,
        target: Some(Target::Player(PlayerId::Two)),
        sacrifice: None,
    };
    game.battlefield = vec![javeliners];
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
fn removing_chaos_orb_in_response_nullifies_its_flip() {
    let mut game = ready_game();
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let shatter = card(10_002, cards::SHATTER, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: orb_id,
            target: Some(Target::Permanent(target_id)),
            sacrifice: None,
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::Two,
        cast_action(shatter.id, vec![Target::Permanent(orb_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != orb_id)
    );

    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
}

#[test]
fn goblin_king_buffs_other_goblins_and_grants_mountainwalk() {
    let mut game = ready_game();
    let king = creature(10_000, cards::GOBLIN_KING, PlayerId::One);
    let mut flarg = creature(10_001, cards::GOBLINS_OF_THE_FLARG, PlayerId::One);
    flarg.attacking = true;
    let mountain = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let blocker = creature(10_003, cards::IRONCLAW_ORCS, PlayerId::Two);
    let flarg_id = flarg.card.id;
    game.battlefield = vec![king, flarg, mountain, blocker];
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    let flarg = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == flarg_id)
        .unwrap();
    assert_eq!(game.power(flarg), Some(2));
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .all(|action| !matches!(
                action,
                Action::DeclareBlocker { attacker, .. } if *attacker == flarg_id
            ))
    );
}

#[test]
fn erhnam_djinn_upkeep_targets_a_creature_for_forestwalk() {
    let mut game = ready_game();
    let erhnam = creature(10_000, cards::ERHNAM_DJINN, PlayerId::One);
    let target = creature(10_001, cards::JUZAM_DJINN, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield = vec![erhnam, target];
    game.turn = 2;
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.prompt,
        "Erhnam Djinn: choose a creature for forestwalk"
    );
    assert_eq!(decision.options.len(), 1);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target_id.0],
        },
    )
    .unwrap();

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .unwrap()
            .forestwalk_until_upkeep_of,
        Some(PlayerId::One)
    );
}

#[test]
fn wheel_discards_both_hands_and_draws_seven() {
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::One));
    game.players[1]
        .hand
        .push(card(10_002, cards::MOUNTAIN, PlayerId::Two));
    game.players[0].mana_pool.red = 3;

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), 7);
    assert_eq!(game.players[1].hand.len(), 7);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| backing_cards(&card.backing) == vec![PhysicalCardId(10_001)])
    );
}

#[test]
fn cleanup_without_a_discard_advances_without_priority() {
    let mut game = ready_game();
    game.step = Step::End;
    let first_turn = game.turn;

    pass_priority_pair(&mut game);

    assert_eq!(game.turn, first_turn + 1);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.observe(PlayerId::One).active_turn, 1);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn cleanup_discard_advances_directly_to_the_next_upkeep() {
    let mut game = ready_game();
    game.step = Step::End;
    for id in 10_000..10_008 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }

    pass_priority_pair(&mut game);
    assert_eq!(game.step, Step::Cleanup);
    let discard = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DiscardCards { .. }))
        .unwrap();
    game.apply(PlayerId::One, discard).unwrap();

    assert_eq!(game.turn, 2);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn attacker_controller_assigns_damage_freely_across_multiple_blockers() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let mut first_blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    first_blocker.blocking = Some(attacker.card.id);
    let mut second_blocker = creature(10_002, cards::ATOG, PlayerId::Two);
    second_blocker.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let first_id = first_blocker.card.id;
    let second_id = second_blocker.card.id;
    game.battlefield = vec![attacker, first_blocker, second_blocker];
    game.begin_combat_damage_assignment();

    let assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: vec![
            CombatDamageAssignment {
                recipient: Target::Permanent(first_id),
                amount: 1,
            },
            CombatDamageAssignment {
                recipient: Target::Permanent(second_id),
                amount: 3,
            },
        ],
    };
    assert!(game.legal_actions(PlayerId::One).contains(&assignment));
    game.apply(PlayerId::One, assignment).unwrap();

    let first = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == first_id)
        .unwrap();
    assert_eq!(first.damage, 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second_id)
    );
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(attacker.damage, 2);
}

#[test]
fn a_single_blocker_needs_no_damage_assignment() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker.card.id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;
    game.begin_combat_damage_assignment();

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { .. })),
        "one blocker leaves nothing worth deciding",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the blocker still takes lethal damage",
    );
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "a 6/1 trampler over a 1/2 blocker spills the remaining 4",
    );
}

#[test]
fn trample_requires_lethal_assignment_before_player_damage() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let mut first = creature(10_001, cards::ATOG, PlayerId::Two);
    first.blocking = Some(attacker.card.id);
    let mut second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::Two);
    second.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let (first_id, second_id) = (first.card.id, second.card.id);
    game.battlefield = vec![attacker, first, second];
    game.begin_combat_damage_assignment();

    let mut recipients = [Target::Permanent(first_id), Target::Permanent(second_id)];
    recipients.sort_unstable();
    let assignment = |to_first: u16, to_second: u16, to_player: u16| {
        let mut assignments: Vec<_> = recipients
            .iter()
            .copied()
            .zip([to_first, to_second])
            .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
            .collect();
        assignments.push(CombatDamageAssignment {
            recipient: Target::Player(PlayerId::Two),
            amount: to_player,
        });
        Action::AssignCombatDamage {
            attacker: attacker_id,
            assignments,
        }
    };
    let actions = game.legal_actions(PlayerId::One);
    let lethal: Vec<u16> = recipients
        .iter()
        .map(|target| match target {
            Target::Permanent(id) => game.lethal_damage(*id),
            _ => 0,
        })
        .collect();
    let spare = 6 - lethal[0] - lethal[1];

    assert!(
        actions.contains(&assignment(lethal[0], lethal[1], spare)),
        "lethal to both blockers then trample over is legal",
    );
    assert!(
        !actions.contains(&assignment(lethal[0] - 1, lethal[1], spare + 1)),
        "trample cannot spill while a blocker is short of lethal",
    );
}

#[test]
fn damage_cannot_be_dribbled_across_several_blockers_at_once() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield = vec![attacker];
    let mut ids = Vec::new();
    for index in 0..3 {
        let mut blocker = creature(10_001 + index, cards::ATOG, PlayerId::Two);
        blocker.blocking = Some(attacker_id);
        ids.push(blocker.card.id);
        game.battlefield.push(blocker);
    }
    ids.sort_unstable();
    game.begin_combat_damage_assignment();

    let assignment = |amounts: [u16; 3]| Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: ids
            .iter()
            .copied()
            .zip(amounts)
            .map(|(id, amount)| CombatDamageAssignment {
                recipient: Target::Permanent(id),
                amount,
            })
            .collect(),
    };
    let actions = game.legal_actions(PlayerId::One);

    // Su-Chi is 4/4 into three 1/2 blockers, so it can kill two of them.
    assert!(
        actions.contains(&assignment([2, 2, 0])),
        "killing two blockers outright is legal",
    );
    assert!(
        !actions.contains(&assignment([1, 1, 2])),
        "only the blocker at the front of the order may be left short of lethal",
    );
}

#[test]
fn green_creatures_get_their_land_bonuses_and_llanowar_elves_make_green() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::TAIGA, PlayerId::One),
        creature(10_001, cards::KIRD_APE, PlayerId::One),
        creature(10_002, cards::LLANOWAR_ELVES, PlayerId::One),
    ]);
    assert_eq!(game.power(&game.battlefield[1]), Some(2));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(3));
    assert_eq!(
        game.mana_colors(&game.battlefield[2]),
        vec![ManaColor::Green]
    );
}

#[test]
fn copy_artifact_copies_an_artifact_creature() {
    let mut game = ready_game();
    let source = creature(10_000, cards::TETRAVUS, PlayerId::Two);
    game.battlefield.push(source);
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let action = cast_action(
        copy.id,
        vec![Target::Permanent(CardInstanceId(10_000))],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .unwrap();
    assert_eq!(
        game.effective_behavior(copied),
        Some(CardBehavior::Tetravus)
    );
    assert_eq!(copied.presented, CardPartId::PRIMARY);
    assert_eq!(
        game.effective_rules(copied),
        Some(CardBehavior::Tetravus.rules())
    );
    assert_eq!(game.power(copied), Some(4));
    assert!(game.has_flying(copied));
}

#[test]
fn dust_to_dust_exiles_two_artifacts_and_hurkyls_recall_returns_them() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let dust = spell(10_002, cards::DUST_TO_DUST, PlayerId::One, 0);
    dust_to_dust_targets(&mut game, dust);
    assert_eq!(game.players[0].exile.len(), 0);
    assert_eq!(game.players[1].exile.len(), 2);

    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let recall = spell_with_targets(
        10_002,
        cards::HURKYLS_RECALL,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    game.resolve_spell_effect(&recall, CardBehavior::HurkylsRecall);
    assert_eq!(game.players[1].hand.len(), 2);
    assert!(game.battlefield.is_empty());
}

fn dust_to_dust_targets(game: &mut Game, mut spell: StackObject) {
    spell.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(
            vec![
                Target::Permanent(CardInstanceId(10_000)),
                Target::Permanent(CardInstanceId(10_001)),
            ],
            0,
        ),
    ));
    game.resolve_spell_effect(&spell, CardBehavior::DustToDust);
}

#[test]
fn regeneration_shields_stop_destroy_but_not_wrath() {
    let mut game = ready_game();
    let mut troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    troll.regeneration_shields = 1;
    game.battlefield.push(troll);
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.battlefield[0].regeneration_shields, 0);

    let wrath = spell(10_001, cards::WRATH_OF_GOD, PlayerId::Two, 0);
    game.resolve_spell_effect(&wrath, CardBehavior::WrathOfGod);
    assert!(game.battlefield.is_empty());
}

#[test]
fn moat_prevents_nonfliers_and_argothian_pixies_dodge_artifact_blockers() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.battlefield
        .push(creature(10_000, cards::MOAT, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERENDIB_EFREET, PlayerId::One));
    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_001)
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_002)
    }));

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    let mut pixies = creature(10_003, cards::ARGOTHIAN_PIXIES, PlayerId::One);
    pixies.attacking = true;
    game.battlefield.push(pixies);
    game.battlefield
        .push(creature(10_004, cards::SU_CHI, PlayerId::Two));
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_004),
                attacker: CardInstanceId(10_003),
            })
    );
}

#[test]
fn firebreathing_is_offered_while_the_mana_is_still_in_the_land() {
    for definition in [
        cards::DRAGON_WHELP,
        cards::GOBLIN_BALLOON_BRIGADE,
        cards::GRANITE_GARGOYLE,
    ] {
        let mut game = ready_game();
        let source = creature(10_000, definition, PlayerId::One);
        let source_id = source.card.id;
        game.battlefield.push(source);
        game.battlefield
            .push(creature(10_001, cards::MOUNTAIN, PlayerId::One));
        assert_eq!(game.players[0].mana_pool.red, 0);

        let activation = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, target: None, .. }
                    if *source == source_id)
            })
            .expect("the ability is offered with an untapped Mountain and an empty pool");

        let before = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| game.power(permanent));
        game.apply(PlayerId::One, activation).unwrap();
        while !game.stack.is_empty() {
            game.apply(PlayerId::One, Action::PassPriority).unwrap();
            game.apply(PlayerId::Two, Action::PassPriority).unwrap();
        }

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == cards::MOUNTAIN && permanent.tapped),
            "activating tapped the land for you",
        );
        assert_eq!(
            game.players[0].mana_pool.red, 0,
            "and spent exactly the red it produced",
        );
        let after = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| game.power(permanent));
        if definition == cards::DRAGON_WHELP {
            assert_eq!(
                after,
                before.map(|power| power.map(|value| value + 1)),
                "Dragon Whelp grew",
            );
        }
    }
}

#[test]
fn doom_blade_destroys_a_nonblack_creature_but_not_a_black_one() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(10_002, cards::JUZAM_DJINN, PlayerId::Two),
    ]);

    let named: Vec<_> = game
        .legal_target_lists(CardBehavior::DoomBlade, 0, PlayerId::One, None)
        .into_iter()
        .filter_map(|choice| match choice.first() {
            Some(Target::Permanent(id)) => Some(*id),
            _ => None,
        })
        .collect();
    assert!(
        named.contains(&CardInstanceId(10_001)),
        "the white creature is a legal target"
    );
    assert!(
        !named.contains(&CardInstanceId(10_002)),
        "Juzam Djinn is black, so Doom Blade cannot touch it"
    );

    let cast = spell_with_targets(
        10_003,
        cards::DOOM_BLADE,
        PlayerId::One,
        vec![Target::Permanent(CardInstanceId(10_001))],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::DoomBlade);

    assert_eq!(game.battlefield.len(), 1, "only the black creature is left");
    assert_eq!(
        game.battlefield[0].card.definition,
        cards::JUZAM_DJINN,
        "and it is the one Doom Blade could not target"
    );
}

#[test]
fn negate_and_essence_scatter_split_the_stack_by_card_kind() {
    let mut game = ready_game();
    // A creature spell and a noncreature spell, both waiting to resolve.
    game.stack
        .push(spell(10_001, cards::SAVANNAH_LIONS, PlayerId::Two, 0));
    game.stack
        .push(spell(10_002, cards::LIGHTNING_BOLT, PlayerId::Two, 0));

    let spells_hit = |game: &Game, behavior| -> Vec<StackObjectId> {
        game.legal_target_lists(behavior, 0, PlayerId::One, None)
            .into_iter()
            .filter_map(|choice| match choice.first() {
                Some(Target::Spell(id)) => Some(*id),
                _ => None,
            })
            .collect()
    };

    let scatter = spells_hit(&game, CardBehavior::EssenceScatter);
    assert_eq!(
        scatter,
        vec![StackObjectId(10_001)],
        "Essence Scatter sees only the creature spell"
    );
    let negate = spells_hit(&game, CardBehavior::Negate);
    assert_eq!(
        negate,
        vec![StackObjectId(10_002)],
        "Negate sees only the noncreature spell"
    );

    let counter = spell_with_targets(
        10_003,
        cards::NEGATE,
        PlayerId::One,
        vec![Target::Spell(StackObjectId(10_002))],
        0,
    );
    game.resolve_spell_effect(&counter, CardBehavior::Negate);
    assert!(
        !game
            .stack
            .iter()
            .any(|object| object.id == StackObjectId(10_002)),
        "the countered spell left the stack"
    );
    assert!(
        game.stack
            .iter()
            .any(|object| object.id == StackObjectId(10_001)),
        "and the creature spell is untouched"
    );
}

#[test]
fn dispel_counters_only_instants() {
    let mut game = ready_game();
    game.stack
        .push(spell(10_001, cards::LIGHTNING_BOLT, PlayerId::Two, 0)); // instant
    game.stack
        .push(spell(10_002, cards::ARMAGEDDON, PlayerId::Two, 0)); // sorcery

    let hit: Vec<_> = game
        .legal_target_lists(CardBehavior::Dispel, 0, PlayerId::One, None)
        .into_iter()
        .filter_map(|choice| match choice.first() {
            Some(Target::Spell(id)) => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(
        hit,
        vec![StackObjectId(10_001)],
        "Dispel sees the instant and not the sorcery"
    );
}

#[test]
fn ultimate_price_spares_multicolored_creatures() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two), // mono-white
        creature(10_002, cards::SERENDIB_EFREET, PlayerId::Two), // mono-blue
    ]);

    let named: Vec<_> = game
        .legal_target_lists(CardBehavior::UltimatePrice, 0, PlayerId::One, None)
        .into_iter()
        .filter_map(|choice| match choice.first() {
            Some(Target::Permanent(id)) => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(named.len(), 2, "both monocolored creatures are targetable");
}

#[test]
fn sign_in_blood_draws_two_and_costs_two_life_without_dealing_damage() {
    let mut game = ready_game();
    let before_hand = game.players[0].hand.len();
    let before_life = game.players[0].life;

    let cast = spell_with_targets(
        10_000,
        cards::SIGN_IN_BLOOD,
        PlayerId::One,
        vec![Target::Player(PlayerId::One)],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::SignInBlood);

    assert_eq!(game.players[0].hand.len(), before_hand + 2);
    assert_eq!(game.players[0].life, before_life - 2);
    // Losing life is not being dealt damage: nothing that triggers on damage
    // may see this, so it must not be logged as damage either.
    assert!(
        game.events
            .iter()
            .any(|event| matches!(event, GameEvent::LifeLost { amount: 2, .. })),
        "the loss is recorded as life loss"
    );
    assert!(
        !game
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::DamageDealt { .. })),
        "and never as damage"
    );
}

#[test]
fn dissipate_exiles_the_spell_it_counters() {
    let mut game = ready_game();
    game.stack
        .push(spell(10_001, cards::LIGHTNING_BOLT, PlayerId::Two, 0));

    let cast = spell_with_targets(
        10_002,
        cards::DISSIPATE,
        PlayerId::One,
        vec![Target::Spell(StackObjectId(10_001))],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::Dissipate);

    assert!(game.stack.is_empty(), "the spell left the stack");
    assert!(
        game.players[1].graveyard.is_empty(),
        "a Dissipated spell does not reach the graveyard"
    );
    assert_eq!(
        game.players[1].exile.len(),
        1,
        "it is exiled instead, so it cannot be rebought"
    );
}

#[test]
fn duress_takes_a_noncreature_nonland_card_of_the_casters_choosing() {
    let mut game = ready_game();
    game.players[1].hand.extend([
        card(10_001, cards::SAVANNAH_LIONS, PlayerId::Two), // creature: off limits
        card(10_002, cards::MOUNTAIN, PlayerId::Two),       // land: off limits
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two), // fair game
    ]);

    let cast = spell_with_targets(
        10_000,
        cards::DURESS,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::Duress);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses");
    assert_eq!(
        decision.options.len(),
        1,
        "only the instant is a legal choice"
    );
    // The hand is revealed, so the choice is public rather than hidden.
    assert_eq!(decision.visibility, DecisionVisibility::Public);

    let choice = decision.options[0].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![choice],
        },
    )
    .expect("choosing the revealed card is legal");

    assert_eq!(game.players[1].hand.len(), 2, "one card was discarded");
    assert!(
        !game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and it was the one the caster named"
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
}

#[test]
fn mulch_keeps_the_lands_and_bins_the_rest() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::MOUNTAIN, PlayerId::One),
        card(10_002, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_003, cards::MOUNTAIN, PlayerId::One),
        card(10_004, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_005, cards::BLACK_LOTUS, PlayerId::One), // fifth card is untouched
    ]);
    let before_hand = game.players[0].hand.len();

    let cast = spell(10_000, cards::MULCH, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::Mulch);

    assert_eq!(
        game.players[0].hand.len(),
        before_hand + 2,
        "two lands kept"
    );
    assert_eq!(game.players[0].graveyard.len(), 2, "two nonlands binned");
    assert_eq!(
        game.players[0].library.len(),
        1,
        "only the top four were revealed"
    );
}

#[test]
fn grisly_salvage_may_keep_one_creature_or_land() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One), // not eligible
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One), // creature
        card(10_003, cards::MOUNTAIN, PlayerId::One),       // land
        card(10_004, cards::BLACK_LOTUS, PlayerId::One),    // not eligible
        card(10_005, cards::COUNTERSPELL, PlayerId::One),   // not eligible
    ]);

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::GrislySalvage);

    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    assert_eq!(decision.options.len(), 2, "the creature and the land");
    assert_eq!(
        decision.minimum, 0,
        "'you may' means keeping nothing is legal"
    );

    let keep = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == CardInstanceId(10_003))
        })
        .expect("the land is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![keep],
        },
    )
    .unwrap();

    // A zone change mints a new object id, so the card is identified by what
    // it is rather than by the id it had in the library.
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.players[0].hand[0].definition,
        cards::MOUNTAIN,
        "the chosen land reached hand"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        4,
        "the other four are binned"
    );
    assert!(game.players[0].library.is_empty());
}

#[test]
fn grisly_salvage_can_decline_and_bin_everything() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0]
        .library
        .extend((0..5).map(|i| card(10_100 + i, cards::SAVANNAH_LIONS, PlayerId::One)));

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    game.resolve_spell_effect(&cast, CardBehavior::GrislySalvage);
    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("declining is legal");

    assert!(game.players[0].hand.is_empty(), "nothing was kept");
    assert_eq!(
        game.players[0].graveyard.len(),
        5,
        "and no revealed card was lost on the way"
    );
}

#[test]
fn putrefy_kills_a_creature_or_an_artifact_without_regeneration() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(10_002, cards::BLACK_LOTUS, PlayerId::Two), // artifact
        creature(10_003, cards::MOUNTAIN, PlayerId::Two),    // land: neither
    ]);

    let named: Vec<_> = game
        .legal_target_lists(CardBehavior::Putrefy, 0, PlayerId::One, None)
        .into_iter()
        .filter_map(|choice| match choice.first() {
            Some(Target::Permanent(id)) => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(
        named.len(),
        2,
        "the creature and the artifact, not the land"
    );

    let cast = spell_with_targets(
        10_004,
        cards::PUTREFY,
        PlayerId::One,
        vec![Target::Permanent(CardInstanceId(10_002))],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::Putrefy);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(10_002)),
        "the artifact was destroyed"
    );
}

#[test]
fn warleaders_helix_burns_and_gains_in_one_resolution() {
    let mut game = ready_game();
    let before = game.players[0].life;

    let cast = spell_with_targets(
        10_000,
        cards::WARLEADERS_HELIX,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    game.resolve_spell_effect(&cast, CardBehavior::WarleadersHelix);

    assert_eq!(game.players[1].life, 16, "four damage to the opponent");
    assert_eq!(game.players[0].life, before + 4, "and four life to you");
}

#[test]
fn sphinxs_revelation_scales_life_and_cards_with_x() {
    let mut game = ready_game();
    let before_life = game.players[0].life;
    let before_hand = game.players[0].hand.len();

    let cast = spell(10_000, cards::SPHINXS_REVELATION, PlayerId::One, 3);
    game.resolve_spell_effect(&cast, CardBehavior::SphinxsRevelation);

    assert_eq!(game.players[0].life, before_life + 3);
    assert_eq!(game.players[0].hand.len(), before_hand + 3);
}

#[test]
fn the_mana_creatures_tap_for_their_colour() {
    // Their whole printed text is a mana ability the engine already models,
    // so they are complete rather than staged.
    for (definition, expected) in [
        (cards::AVACYNS_PILGRIM, ManaColor::White),
        (cards::ELVISH_MYSTIC, ManaColor::Green),
    ] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_001, definition, PlayerId::One));
        assert!(
            game.legal_actions(PlayerId::One)
                .iter()
                .any(|action| matches!(
                    action,
                    Action::ActivateManaAbility { color, .. } if *color == expected
                )),
            "{definition:?} taps for {expected:?}"
        );
    }
}

#[test]
fn deathtouch_kills_whatever_it_touches_and_lifelink_pays_its_controller() {
    // Vampire Nighthawk is a 2/3 flying deathtouch lifelink. Before these
    // keywords were read, it was a 2/3 flier and nothing more.
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = Some(CardInstanceId(10_001));
    game.battlefield.extend([hawk, wall]);
    let before_life = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two deathtouch damage is lethal to a 4/4"
    );
    assert_eq!(
        game.players[0].life,
        before_life + 2,
        "and lifelink paid its controller for the damage dealt"
    );
}

#[test]
fn lifelink_pays_for_damage_to_a_player_too() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    game.battlefield.push(hawk);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert_eq!(game.players[1].life, 18, "unblocked, it hits for two");
    assert_eq!(game.players[0].life, before + 2, "and gains that much");
}

#[test]
fn an_ordinary_creature_does_not_gain_life_or_kill_through_toughness() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One); // 2/1 vanilla
    lions.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = Some(CardInstanceId(10_001));
    game.battlefield.extend([lions, wall]);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two ordinary damage does not kill a 4/4"
    );
    assert_eq!(game.players[0].life, before, "and gains nobody any life");
}

#[test]
fn reach_blocks_fliers_without_flying() {
    // Ruric Thar has reach; a plain ground creature does not.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut flier = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    flier.attacking = true;
    game.battlefield.push(flier);
    game.battlefield.push(creature(
        10_002,
        cards::RURIC_THAR_THE_UNBOWED,
        PlayerId::Two,
    ));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two));

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "reach can block a flier"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a ground creature still cannot"
    );
}

#[test]
fn intimidate_only_lets_artifacts_and_matching_colours_block() {
    // Lifebane Zombie is black; only black or artifact creatures may block it.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut zombie = creature(10_001, cards::LIFEBANE_ZOMBIE, PlayerId::One);
    zombie.attacking = true;
    game.battlefield.push(zombie);
    game.battlefield
        .push(creature(10_002, cards::JUZAM_DJINN, PlayerId::Two)); // black
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two)); // white

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "a black creature shares a colour and may block"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a white creature may not"
    );
}

#[test]
fn hexproof_stops_opponents_targeting_but_not_its_controller() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::SIGARDA_HOST_OF_HERONS,
        PlayerId::Two,
    ));

    let opponent_targets = game.legal_target_lists(CardBehavior::Terror, 0, PlayerId::One, None);
    assert!(
        opponent_targets.is_empty(),
        "an opponent cannot target hexproof"
    );

    let own_targets = game.legal_target_lists(CardBehavior::Terror, 0, PlayerId::Two, None);
    assert_eq!(
        own_targets.len(),
        1,
        "its own controller still can, hexproof only stops opponents"
    );
}

#[test]
fn undying_returns_the_creature_once_with_a_counter() {
    // Strangleroot Geist is a 2/1 with haste and undying.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One));

    game.destroy_permanent(CardInstanceId(10_001));

    assert_eq!(
        game.battlefield.len(),
        1,
        "it came back rather than staying dead"
    );
    let returned = &game.battlefield[0];
    assert_eq!(returned.plus_one_counters, 1, "with a +1/+1 counter");
    assert_ne!(
        returned.card.id,
        CardInstanceId(10_001),
        "and as a new object, because it really did change zones"
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "the card left the graveyard on its way back"
    );

    // Second death: it has a counter now, so undying does not apply.
    let second = returned.card.id;
    game.destroy_permanent(second);
    assert!(game.battlefield.is_empty(), "it stays dead the second time");
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn undying_returns_it_to_its_owner_not_whoever_killed_it() {
    let mut game = ready_game();
    let mut geist = creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One);
    // Someone else has taken control of it.
    geist.controller = PlayerId::Two;
    game.battlefield.push(geist);

    game.destroy_permanent(CardInstanceId(10_001));

    assert_eq!(
        game.battlefield[0].controller,
        PlayerId::One,
        "undying returns it under its owner's control"
    );
}

#[test]
fn a_plus_one_counter_boosts_stats_whatever_put_it_there() {
    // Strangleroot Geist is a 2/1; undying brings it back as a 3/2. Before
    // +1/+1 counters and javelin counters were separated, the stat bonus was
    // allowlisted to three named cards and this counter did nothing.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One));
    game.destroy_permanent(CardInstanceId(10_001));

    let returned = &game.battlefield[0];
    assert_eq!(game.power(returned), Some(3), "2/1 plus a counter is 3/2");
    assert_eq!(game.toughness(returned), Some(2));
}

#[test]
fn a_javelin_counter_is_not_a_plus_one_counter() {
    // Icatian Javelineers enters with a javelin counter and stays a 1/1.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::ICATIAN_JAVELINEERS, PlayerId::One));
    let javelineers = &game.battlefield[0];
    assert_eq!(
        game.power(javelineers),
        Some(1),
        "its counter is ammunition, not a stat boost"
    );
    assert_eq!(game.toughness(javelineers), Some(1));
}

#[test]
fn protection_reads_the_printed_colours_not_a_list_of_card_names() {
    // Blood Baron of Vizkopa has protection from white and from black. Its
    // data was already in the catalog; the engine simply never looked at it.
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::BLOOD_BARON_OF_VIZKOPA,
        PlayerId::Two,
    ));

    // Swords to Plowshares is white, Terror is black, Lightning Bolt is red.
    for (behavior, blocked) in [
        (CardBehavior::SwordsToPlowshares, true),
        (CardBehavior::Terror, true),
        (CardBehavior::LightningBolt, false),
    ] {
        let targets = game.legal_target_lists(behavior, 0, PlayerId::One, None);
        let names_baron = targets.iter().any(|choice| {
            matches!(choice.first(), Some(Target::Permanent(id)) if *id == CardInstanceId(10_001))
        });
        assert_eq!(
            names_baron,
            !blocked,
            "{behavior:?} targeting Blood Baron should be {}",
            if blocked { "blocked" } else { "allowed" }
        );
    }
}

#[test]
fn the_old_school_knights_keep_their_protection() {
    // These four used to be named in the engine directly. Moving to printed
    // data must not change what they do.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::WHITE_KNIGHT, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::BLACK_KNIGHT, PlayerId::Two));

    let hit_by = |game: &Game, behavior| -> Vec<CardInstanceId> {
        game.legal_target_lists(behavior, 0, PlayerId::One, None)
            .into_iter()
            .filter_map(|choice| match choice.first() {
                Some(Target::Permanent(id)) => Some(*id),
                _ => None,
            })
            .collect()
    };

    // Terror is black and cannot touch White Knight. It could not touch Black
    // Knight either, but only because Black Knight is black, so the white
    // Swords to Plowshares is what shows protection working the other way.
    let by_black = hit_by(&game, CardBehavior::Terror);
    assert!(
        !by_black.contains(&CardInstanceId(10_001)),
        "White Knight has protection from black"
    );

    let by_white = hit_by(&game, CardBehavior::SwordsToPlowshares);
    assert!(
        !by_white.contains(&CardInstanceId(10_002)),
        "Black Knight has protection from white"
    );
    assert!(
        by_white.contains(&CardInstanceId(10_001)),
        "White Knight has no protection from white"
    );
}

#[test]
fn blood_baron_of_vizkopa_ascends_at_thirty_life() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::BLOOD_BARON_OF_VIZKOPA,
        PlayerId::One,
    ));
    let baron = game.battlefield.last().unwrap().clone();

    // Neither half of the condition alone is enough.
    game.players[0].life = 30;
    game.players[1].life = 11;
    assert_eq!(game.power(&baron), Some(4));
    assert!(!game.has_flying(&baron));

    game.players[0].life = 29;
    game.players[1].life = 10;
    assert_eq!(game.power(&baron), Some(4));

    game.players[0].life = 30;
    game.players[1].life = 10;
    assert_eq!(game.power(&baron), Some(10));
    assert_eq!(game.toughness(&baron), Some(10));
    assert!(game.has_flying(&baron));
}

#[test]
fn pillar_of_flame_exiles_what_it_kills() {
    let mut game = ready_game();
    // Savannah Lions is 2/1, so two damage is lethal.
    let lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lion_id = lion.card.id;
    game.battlefield.push(lion);
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(pillar.id, vec![Target::Permanent(lion_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert!(
        game.players[1].graveyard.is_empty(),
        "the lion never reaches the graveyard"
    );
    assert_eq!(game.players[1].exile[0].definition, cards::SAVANNAH_LIONS);
}

#[test]
fn pillar_of_flame_exiles_a_survivor_that_dies_later_this_turn() {
    let mut game = ready_game();
    // Serra Angel is 4/4: two damage leaves it alive, but the replacement
    // lasts the turn, so a later Lightning Bolt exiles it anyway.
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        cast_action(pillar.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield.len(), 1, "four damage is not lethal");

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
}

#[test]
fn pillar_of_flame_can_burn_a_player() {
    let mut game = ready_game();
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(
            pillar.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
}

#[test]
fn supreme_verdict_destroys_every_creature() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::Two));
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

    assert!(game.battlefield.is_empty(), "both sides are swept");
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

/// Plays a shock land and answers its enters-untapped question.
fn play_shock_land(game: &mut Game, definition: CardDefinitionId, pay: bool) {
    game.players[0]
        .hand
        .push(card(10_500, definition, PlayerId::One));
    game.play_land(PlayerId::One, CardInstanceId(10_500), PlayOptionId::DEFAULT);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let option = u32::from(pay);
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
fn a_shock_land_can_be_paid_for_or_left_tapped() {
    for (pay, tapped, life) in [(true, false, 18), (false, true, 20)] {
        let mut game = ready_game();
        play_shock_land(&mut game, cards::HALLOWED_FOUNTAIN, pay);

        assert_eq!(game.battlefield[0].tapped, tapped);
        assert_eq!(game.players[0].life, life);
        assert!(game.pending_decisions.is_empty());
    }
}

#[test]
fn a_shock_land_asks_nothing_when_the_life_is_not_there() {
    // You may pay life down to zero, but you cannot pay more than you have.
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[0]
        .hand
        .push(card(10_500, cards::STEAM_VENTS, PlayerId::One));
    game.play_land(PlayerId::One, CardInstanceId(10_500), PlayOptionId::DEFAULT);

    assert!(
        game.pending_decisions.is_empty(),
        "no prompt whose only real answer is no"
    );
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].life, 1);
}

#[test]
fn paying_for_a_shock_land_at_exactly_two_life_loses_the_game() {
    let mut game = ready_game();
    game.players[0].life = 2;
    play_shock_land(&mut game, cards::TEMPLE_GARDEN, true);

    assert_eq!(game.players[0].life, 0);
    assert!(matches!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            ..
        })
    ));
}

/// Puts `library` on top of player one's library, top card first.
fn stack_library(game: &mut Game, library: &[(u32, CardDefinitionId)]) {
    for (instance, definition) in library.iter().rev() {
        game.players[0]
            .library
            .insert(0, card(*instance, *definition, PlayerId::One));
    }
}

#[test]
fn augur_of_bolas_digs_three_deep_when_it_enters() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (11_000, cards::SAVANNAH_LIONS),
            (11_001, cards::LIGHTNING_BOLT),
            (11_002, cards::SERRA_ANGEL),
            (11_003, cards::JUZAM_DJINN),
        ],
    );
    let augur = card(11_100, cards::AUGUR_OF_BOLAS, PlayerId::One);
    game.players[0].hand.push(augur.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(augur.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // Only the Bolt among the top three is an instant or sorcery.
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(_, definition)| definition))
        .collect();
    assert_eq!(offered, vec![cards::LIGHTNING_BOLT]);

    let bolt = decision.options.iter().find(|o| o.card.is_some()).unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![bolt.id],
        },
    )
    .unwrap();

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::LIGHTNING_BOLT)
    );
    // The other two went to the bottom, leaving the fourth card on top.
    let library: Vec<_> = game.players[0]
        .library
        .iter()
        .map(|c| c.definition)
        .collect();
    assert_eq!(
        library,
        vec![
            cards::JUZAM_DJINN,
            cards::SAVANNAH_LIONS,
            cards::SERRA_ANGEL
        ]
    );
}

#[test]
fn augur_of_bolas_may_decline_and_bottom_all_three() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (11_000, cards::LIGHTNING_BOLT),
            (11_001, cards::SERRA_ANGEL),
            (11_002, cards::JUZAM_DJINN),
            (11_003, cards::SAVANNAH_LIONS),
        ],
    );
    let augur = card(11_100, cards::AUGUR_OF_BOLAS, PlayerId::One);
    game.players[0].hand.push(augur.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(augur.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 0, "revealing is optional");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .unwrap();

    assert!(game.players[0].hand.is_empty());
    let library: Vec<_> = game.players[0]
        .library
        .iter()
        .map(|c| c.definition)
        .collect();
    assert_eq!(
        library,
        vec![
            cards::SAVANNAH_LIONS,
            cards::LIGHTNING_BOLT,
            cards::SERRA_ANGEL,
            cards::JUZAM_DJINN
        ]
    );
}
