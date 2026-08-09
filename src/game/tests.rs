use super::*;
use crate::card::abilities;
use crate::mana_cost;
use crate::poc::{self, cards};
use crate::{
    AbilityTargetDef, AbilityTargetPredicate, AdditionalCostDef, AdditionalCostId,
    AlternativeCastManaCostDef, AlternativeCostDef, AlternativeCostId, CardComposition,
    CardDefinition, CardEffectStatus, CardInstanceId, CardPart, CardPartId, CardPrinting,
    CardRules, CardStructure, CastChoices, DoubleFacedKind, LandEntry, ManaSpendEffectDef, ModeDef,
    ModeSetDef, PlayOptionDef, PlayOptionId, PlayerRelation, SpellForm, StackObjectId,
    TargetPredicate, TargetSelection, TargetSlotDef, TargetSlotId,
};

static TEST_FLYING_ABILITY: [AbilityDef; 1] = [abilities::flying()];
static TEST_FLYING_TRAMPLE_ABILITIES: [AbilityDef; 2] = [abilities::flying(), abilities::trample()];
static CARD_COST_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();

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
    game.combat_damage_stage = CombatDamageStage::NotStarted;
    game.combat_blocked_attackers.clear();
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
    Permanent::entering(
        card(id, definition, controller),
        CardPartId::PRIMARY,
        controller,
        0,
    )
}

fn copied_characteristics(definition: CardDefinitionId) -> CopiableCharacteristics {
    CopiableCharacteristics {
        base: (definition, CardPartId::PRIMARY),
        added_types: CardTypeSet::empty(),
        added_abilities: Vec::new(),
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

fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

const fn primary_ability(definition: CardDefinitionId) -> AbilityOrigin {
    AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: crate::AbilityId::PRIMARY,
    }
}

fn mana_ability_for(game: &Game, source: GameObjectId, color: ManaColor) -> AbilityOrigin {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .into_iter()
        .flat_map(|permanent| game.mana_ability_activations(permanent))
        .find(|activation| activation.color == color)
        .expect("source has an effective mana ability for the requested color")
        .ability
}

fn activated_ability_for(game: &Game, source: GameObjectId, index: usize) -> AbilityOrigin {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("source is on the battlefield");
    game.activated_ability_origin(permanent, index)
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
        controller,
        signature: Some(CastSignature::from_validated_choices(
            SpellForm::Part(CardPartId::PRIMARY),
            cast_choices(Vec::new(), x),
        )),
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        text_changes: Vec::new(),
        cast_via_flashback: false,
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

fn choose_decision_by_label(game: &mut Game, player: PlayerId, label: &str) {
    let decision = game
        .observe(player)
        .decision
        .expect("the expected choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == label)
        .unwrap_or_else(|| panic!("decision does not offer {label}"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the named decision option is legal");
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
            ability: activated_ability_for(&game, source_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    let ability_id = game.stack[0].id;
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        })
    );
    assert_eq!(
        game.stack[0].ability_text(),
        Some("{T}, Sacrifice this land: Destroy target land.")
    );
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
    let cost = CardBehavior::Recall
        .mana_cost()
        .expect("Recall has a printed mana cost");
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
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}: Add {U} or {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
    )];
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test dual land",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_land(&[]).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    let activations = game.mana_ability_activations(&game.battlefield[0]);
    assert_eq!(
        activations
            .iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue, ManaColor::Red]
    );
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
    );
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
    check.rules = CardRules::new_land(&[]).land_entry(LandEntry::TappedUnlessControlsLandType([
        true, false, false, false, false,
    ]));
    synchronize_single_part_definition(&mut check);
    let mut gate = CardDefinition::new(
        gate_id,
        "Test gate",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    gate.rules = CardRules::new_land(&[]).land_entry(LandEntry::Tapped);
    synchronize_single_part_definition(&mut gate);
    let mut shock = CardDefinition::new(
        shock_id,
        "Test shock land",
        CardSet::Gatecrash,
        false,
        CardBehavior::Unsupported,
    );
    shock.rules =
        CardRules::new_land(&["Plains", "Swamp"]).land_entry(LandEntry::PayLifeOrTapped(2));
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
    let front_rules = CardRules::new_sorcery(ManaCost::new(1, 0));
    let land_rules = CardRules::new_land(&[]).land_entry(LandEntry::Tapped);
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
        CardPart::new(land_part, "Test back", land_rules),
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
            front_rules
                .mana_cost()
                .expect("the front has a printed mana cost"),
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
    let front_rules = CardRules::new_instant(ManaCost::new(1, 1));
    let creature_rules = CardRules::new_creature(ManaCost::new(0, 0), &[], 3, 4)
        .with_abilities(&TEST_FLYING_ABILITY);
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
            front_rules
                .mana_cost()
                .expect("the front has a printed mana cost"),
            CardEffectStatus::MetadataOnly,
        ),
        PlayOptionDef::cast(
            creature_option,
            "Cast Test creature back",
            SpellForm::Part(creature_part),
            creature_rules
                .mana_cost()
                .expect("the modal back has a printed mana cost"),
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
    let front_rules = CardRules::new_creature(ManaCost::new(2, 0), &[], 2, 2);
    let back_rules = CardRules::new_creature_without_mana_cost(&[], 4, 5)
        .with_abilities(&TEST_FLYING_TRAMPLE_ABILITIES);
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
        CardPart::new(back, "Test Ravager", back_rules),
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
        front_rules
            .mana_cost()
            .expect("the front has a printed mana cost"),
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

    // Izzet Charm is only partial, and the play gate follows the modes that
    // do work: its loot mode needs no target and is castable on an empty board.
    assert_eq!(
        cast_cards,
        vec![CardInstanceId(10_003), CardInstanceId(10_005)]
    );
    assert!(actions.contains(&Action::PlayLand {
        card: CardInstanceId(10_004),
        option: PlayOptionId::DEFAULT,
    }));
}

#[test]
fn energy_flux_cannot_be_cast_as_an_inert_enchantment() {
    let mut game = ready_game();
    let flux = card(10_000, crate::card::cards::ENERGY_FLUX, PlayerId::One);
    let flux_id = flux.id;
    game.players[0].hand.push(flux);
    game.players[0].mana_pool.colorless = 2;
    game.players[0].mana_pool.blue = 1;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == flux_id))
    );

    let result = game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: flux_id,
            choices: CastChoices::default(),
            sacrifices: Vec::new(),
        },
    );
    assert!(result.is_err());
    assert!(game.stack.is_empty());
    assert!(game.players[0].hand.iter().any(|card| card.id == flux_id));
}

#[test]
#[allow(clippy::too_many_lines)]
fn cast_validation_rejects_unrecognized_structured_choices() {
    let definition_id = CardDefinitionId(10_200);
    let option_id = PlayOptionId(7);
    let implemented_mode = ModeId(0);
    let metadata_mode = ModeId(1);
    let second_implemented_mode = ModeId(2);
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
    definition.rules = CardRules::new_instant(ManaCost::new(0, 1));
    synchronize_single_part_definition(&mut definition);
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
            ModeDef {
                id: second_implemented_mode,
                label: "Second implemented mode".into(),
                targets: Vec::new(),
                effect_status: CardEffectStatus::Implemented,
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
    let (signature, cost, _, _) = game
        .validated_cast_signature(PlayerId::One, card_id, &valid)
        .expect("all structured choices are recognized and payable");
    assert_eq!(signature.play_option(), option_id);
    assert_eq!(signature.form(), &SpellForm::Part(CardPartId::PRIMARY));
    assert_eq!(signature.modes(), &[implemented_mode]);
    assert_eq!(signature.costs(), valid.costs());
    assert_eq!(signature.targets(), valid.targets());
    assert_eq!(cost, ManaCost::new(3, 0));

    let canonical_modes = CastChoices::new(option_id)
        .with_modes(vec![implemented_mode, second_implemented_mode])
        .with_costs(CostConfiguration::new(
            Some(alternative_id),
            vec![additional_id],
        ))
        .with_targets(vec![TargetSelection::single(
            slot_id,
            Target::Player(PlayerId::Two),
        )]);
    assert!(
        game.validated_cast_signature(PlayerId::One, card_id, &canonical_modes)
            .is_some(),
        "distinct modes are accepted in positional order",
    );

    let invalid = [
        CastChoices::new(PlayOptionId(99)),
        CastChoices::new(option_id),
        CastChoices::new(option_id).with_modes(vec![metadata_mode]),
        CastChoices::new(option_id).with_modes(vec![implemented_mode, implemented_mode]),
        CastChoices::new(option_id)
            .with_modes(vec![second_implemented_mode, implemented_mode])
            .with_targets(vec![TargetSelection::single(
                slot_id,
                Target::Player(PlayerId::Two),
            )]),
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
fn cost_configuration_visitor_preserves_option_order() {
    let definition = CardDefinition::new(
        CardDefinitionId(10_201),
        "Ordered Costs",
        CardSet::Alpha,
        false,
        CardBehavior::Unsupported,
    );
    let mut option = PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Cast Ordered Costs",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(1, 0),
        CardEffectStatus::Implemented,
    );
    let alternatives = [AlternativeCostId(3), AlternativeCostId(7)];
    let additional = [AdditionalCostId(11), AdditionalCostId(13)];
    option.alternative_costs = alternatives
        .into_iter()
        .map(|id| AlternativeCostDef {
            id,
            label: format!("Alternative {}", id.0),
            mana_cost: ManaCost::new(1, 0),
        })
        .collect();
    option.additional_costs = additional
        .into_iter()
        .map(|id| AdditionalCostDef {
            id,
            label: format!("Additional {}", id.0),
            mana_cost: Some(ManaCost::new(1, 0)),
        })
        .collect();

    let game = ready_game();
    let mut actual = Vec::new();
    assert!(
        game.visit_cost_configurations(
            &definition,
            GameObjectId(10_201),
            &option,
            CastSourceZone::Hand,
            |configuration| {
                actual.push(configuration);
                ControlFlow::Continue(())
            },
        )
        .is_continue()
    );

    let additional_sets = [
        vec![],
        vec![additional[0]],
        vec![additional[1]],
        vec![additional[0], additional[1]],
    ];
    let expected = [None, Some(alternatives[0]), Some(alternatives[1])]
        .into_iter()
        .flat_map(|alternative| {
            additional_sets
                .iter()
                .cloned()
                .map(move |additional| CostConfiguration::new(alternative, additional))
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    for invalid in [
        vec![additional[1], additional[0]],
        vec![additional[0], additional[0]],
        vec![AdditionalCostId(99)],
    ] {
        assert!(!actual.contains(&CostConfiguration::new(None, invalid)));
    }
}

#[test]
fn generated_mode_selections_are_canonical_combinations() {
    let modes = [ModeId(0), ModeId(1)];
    assert_eq!(
        mode_id_selections(&modes, 2, 2, false),
        vec![vec![ModeId(0), ModeId(1)]],
    );
    assert_eq!(
        mode_id_selections(&modes, 2, 2, true),
        vec![
            vec![ModeId(0), ModeId(0)],
            vec![ModeId(0), ModeId(1)],
            vec![ModeId(1), ModeId(1)],
        ],
    );
}

#[test]
fn selected_modal_effects_freeze_multiplicity_in_printed_order() {
    const FIRST: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const SECOND: EffectDef = EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const MODAL: AbilityDef = AbilityDef::modal_spell(
        "Choose modes.",
        &[
            AbilityDef::spell("First mode", FIRST),
            AbilityDef::spell("Second mode", SECOND),
        ],
        1,
        3,
        true,
    );
    let DeclarativeAbilityDef::Spell(spell) = MODAL.definition else {
        panic!("the fixture is a modal spell")
    };

    assert_eq!(
        Game::selected_spell_mode_effects(spell, &[ModeId(1), ModeId(0), ModeId(1)]),
        Some(vec![FIRST, SECOND, SECOND]),
    );
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

    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
    );

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
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue),
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
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ANKH_OF_MISHRA))
    );

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
fn ankh_damages_the_entering_lands_controller_not_its_owner() {
    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One));
    let borrowed_mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    game.players[1].hand.push(borrowed_mountain.clone());

    let play_land = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::PlayLand { card, .. } if *card == borrowed_mountain.id)
        })
        .expect("the active player may play the land they currently hold");
    game.apply(PlayerId::Two, play_land).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20, "the physical owner is unharmed");
    assert_eq!(game.players[1].life, 18, "the land's controller takes 2");
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
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Red),
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
        ability: activated_ability_for(&game, CardInstanceId(10_000), 0),
        targets: activated_targets(Target::Permanent(CardInstanceId(10_001))),
        sacrifice: None,
        x: 0,
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
        object.ability_origin() == Some(primary_ability(cards::ANKH_OF_MISHRA))
            && object.ability_text().is_some()
    }));
}

#[test]
fn simultaneous_triggers_are_put_on_the_stack_in_apnap_order() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::Two),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack[0].source,
        Some(CardInstanceId(10_000)),
        "the active player's trigger is put on the stack first"
    );
    assert_eq!(
        game.stack[1].source,
        Some(CardInstanceId(10_001)),
        "the nonactive player's trigger is on top and resolves first"
    );
}

#[test]
fn targeted_trigger_chooses_public_targets_while_being_put_on_stack() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(7),
        "target creature an opponent controls",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
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
    game.capture_trigger(&TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_000),
            ability: primary_ability(cards::ANKH_OF_MISHRA),
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
        resolver: StackAbilityResolver::Declarative(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(7)),
            amount: ValueDef::Constant(2),
        }),
        context: TriggerContext {
            object: None,
            object_controller: None,
            event_player: None,
            amount: None,
        },
        condition: None,
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
fn nonbattlefield_card_targets_are_zone_incarnations() {
    static INSTANT_OR_SORCERY: [ObjectPredicateDef; 2] = [
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ];
    let predicate = AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&INSTANT_OR_SORCERY),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    };
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let stone_rain = card(10_001, cards::STONE_RAIN, PlayerId::One);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    let opposing_bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[0]
        .graveyard
        .extend([bolt.clone(), stone_rain.clone(), mountain]);
    game.players[1].graveyard.push(opposing_bolt);

    let targets = game.ability_targets_matching(
        predicate,
        PlayerId::One,
        GameObjectId(99_999),
        TriggerContext::empty(),
    );
    assert_eq!(
        targets,
        vec![Target::Card(bolt.id), Target::Card(stone_rain.id)]
    );

    let old_bolt = game.players[0].graveyard.remove(0);
    let (new_bolt, zone_change) = game.zone_change_card(old_bolt);
    game.players[0].hand.push(new_bolt);
    assert_eq!(zone_change.previous, bolt.id);
    assert_eq!(
        game.ability_targets_matching(
            predicate,
            PlayerId::One,
            GameObjectId(99_999),
            TriggerContext::empty(),
        ),
        vec![Target::Card(stone_rain.id)],
        "a target does not follow the physical card to its new zone object",
    );
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
    game.capture_trigger(&TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_010),
            ability: primary_ability(cards::SAVANNAH_LIONS),
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
        resolver: StackAbilityResolver::Declarative(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        }),
        context: TriggerContext {
            object: Some(CardInstanceId(10_010)),
            object_controller: Some(PlayerId::One),
            event_player: Some(PlayerId::One),
            amount: None,
        },
        condition: None,
    });
    game.destroy_permanent(CardInstanceId(10_010));
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15, "last known power was five");
}

#[test]
fn workshop_mana_is_three_individual_values_without_an_unimplemented_restriction() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MISHRA_S_WORKSHOP, PlayerId::One));
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability,
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
                    ability,
                })
            && mana.restrictions.is_empty()
    }));
}

#[test]
fn explicitly_tagged_triggered_mana_ability_resolves_without_the_stack() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_mana(
        "Whenever this becomes tapped, add {C}.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::Source),
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    )];
    let definition_id = CardDefinitionId(10_050);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test triggered mana source",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
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
            ability: AbilityOrigin::Printed {
                definition: cards::SAVANNAH_LIONS,
                part: CardPartId::PRIMARY,
                ability: crate::AbilityId(1),
            },
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

    let effect = game
        .catalog
        .get(cards::ARMAGEDDON)
        .expect("Armageddon is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect;
    game.resolve_effect_def(effect, &armageddon, TriggerContext::empty());

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

    game.resolve_balance(PlayerId::One);
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
                    && game
                        .permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .count()
    });
    assert_eq!(land_counts, [1, 1]);
    assert_eq!(game.players[0].hand.len(), game.players[1].hand.len());
}

#[test]
fn balance_recounts_creatures_after_loxodon_smiter_replaces_its_discard() {
    let mut game = ready_game();
    let balance = card(10_010, cards::BALANCE, PlayerId::One);
    game.players[0].hand.push(balance.clone());
    game.players[1]
        .hand
        .push(card(10_011, cards::LOXODON_SMITER, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    game.apply(
        PlayerId::One,
        cast_action(balance.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let discard = game
        .observe(PlayerId::Two)
        .decision
        .expect("Balance makes player two discard down to zero");
    assert_eq!(discard.visibility, DecisionVisibility::Private);
    let smiter = discard
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::LOXODON_SMITER)
        })
        .expect("Loxodon Smiter is the discard choice")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: discard.id,
            options: vec![smiter],
        },
    )
    .unwrap();

    let sacrifice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the creature step is counted after the discard step");
    assert_eq!(sacrifice.visibility, DecisionVisibility::Public);
    assert!(sacrifice.prompt.contains("creature"));
    assert_eq!(sacrifice.options.len(), 1);
    assert!(
        sacrifice.options[0]
            .card
            .is_some_and(|(_, definition)| definition == cards::LOXODON_SMITER)
    );
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: sacrifice.id,
            options: vec![sacrifice.options[0].id],
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::LOXODON_SMITER)
    );
    assert_eq!(
        game.players[1]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::LOXODON_SMITER)
            .count(),
        1,
    );
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardsDiscarded {
            player: PlayerId::Two,
            cards,
        } if cards.iter().any(|(_, definition)| *definition == cards::LOXODON_SMITER)
    )));
}

#[test]
fn balance_defers_one_apnap_trigger_batch_until_its_decisions_finish() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SU_CHI, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::One),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_005, cards::MOUNTAIN, PlayerId::One),
    ]);

    game.resolve_balance(PlayerId::One);
    let discard = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(discard.kind, DecisionKind::Choice);
    assert!(discard.prompt.contains("discard"));
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: discard.id,
            options: discard.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();

    let sacrifice = game.observe(PlayerId::One).decision.unwrap();
    let su_chi = sacrifice
        .options
        .iter()
        .filter(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::SU_CHI)
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(su_chi.len(), 2);
    assert!(sacrifice.prompt.contains("creature"));
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: sacrifice.id,
            options: su_chi,
        },
    )
    .unwrap();

    let order = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(order.kind, DecisionKind::TriggerOrder);
    assert_eq!(order.options.len(), 2);
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: order.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2);
    assert!(
        game.stack
            .iter()
            .all(|object| object.kind == StackObjectKind::TriggeredAbility)
    );
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
            ability: activated_ability_for(&game, permanent_id, 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            sacrifice: None,
            x: 0,
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
fn counterspell_removes_an_older_spell_without_disturbing_an_intervening_spell() {
    let mut game = ready_game();
    let older_bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let intervening_bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::One);
    game.players[1]
        .hand
        .extend([older_bolt.clone(), intervening_bolt.clone()]);
    game.players[1].mana_pool.red = 2;
    game.players[0].hand.push(counterspell.clone());
    game.players[0].mana_pool.blue = 2;
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(
            older_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let older_stack_id = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            intervening_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let intervening_stack_id = game.stack[1].id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(older_stack_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();

    pass_priority_pair(&mut game);

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].id, intervening_stack_id);
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the targeted older spell was countered",
    );

    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 17);
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
fn order_of_leitbur_can_gain_first_strike() {
    let mut game = ready_game();
    let order = creature(10_000, cards::ORDER_OF_LEITBUR, PlayerId::One);
    let order_id = order.card.id;
    game.battlefield.push(order);
    game.players[0].mana_pool.white = 1;
    let activation = Action::ActivateAbility {
        source: order_id,
        ability: activated_ability_for(&game, order_id, 0),
        targets: Vec::new(),
        sacrifice: None,
        x: 0,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    let order = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == order_id)
        .unwrap();
    assert!(
        game.permanent_has_executable_keyword(order, KeywordAbility::FirstStrike),
        "the resolved declarative activation grants executable first strike",
    );
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
fn protection_does_not_prevent_a_protected_creature_from_blocking() {
    let mut game = ready_game();
    let mut lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    lion.attacking = true;
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    game.battlefield = vec![lion, knight];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn protection_prevents_damage_from_a_source_of_the_protected_color() {
    let mut game = ready_game();
    let lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    let lion_id = lion.card.id;
    let knight_id = knight.card.id;
    game.battlefield = vec![lion, knight];

    game.damage_target_from(Some(lion_id), Some(Target::Permanent(knight_id)), 2);

    let knight = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == knight_id)
        .expect("protection keeps Black Knight on the battlefield");
    assert_eq!(knight.damage, 0);
}

#[test]
fn vampire_nighthawk_deathtouch_and_lifelink_are_executable_keyword_abilities() {
    let mut game = ready_game();
    game.players[0].life = 10;
    let nighthawk = creature(10_000, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    let nighthawk_id = nighthawk.card.id;
    let angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield = vec![nighthawk, angel];

    game.damage_target_from(Some(nighthawk_id), Some(Target::Permanent(angel_id)), 1);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel_id),
        "one point from a source with deathtouch is lethal",
    );
    assert_eq!(game.players[0].life, 11);

    game.damage_target_from(Some(nighthawk_id), Some(Target::Player(PlayerId::Two)), 2);
    assert_eq!(game.players[0].life, 13);
    assert_eq!(game.players[1].life, 18);
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
fn hellrider_burns_once_per_attacker_including_itself() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let hellrider = creature(10_000, cards::HELLRIDER, PlayerId::One);
    let hellrider_id = hellrider.card.id;
    game.battlefield.push(hellrider);
    let lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    // A creature the opponent controls is not "a creature you control", and
    // it is not attacking anyway.
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    let life_before = game.players[1].life;

    for attacker in [hellrider_id, lions_id] {
        game.apply(PlayerId::One, Action::DeclareAttacker { attacker })
            .unwrap();
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    // Drain the whole batch rather than stopping at the expected life total,
    // so a third trigger would show up as too much damage.
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_decisions.is_empty() {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "one trigger per attacking creature"
    );
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
    game.finish_rules_procedure();
    assert_eq!(game.players[0].life, 10);
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 12);
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: tome_id,
            ability: activated_ability_for(&game, tome_id, 0),
            targets: Vec::new(),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);
}

#[test]
fn library_of_alexandria_draw_activation_keeps_its_printed_ability_id() {
    let mut game = ready_game();
    for id in 10_000..10_007 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    let library = creature(10_010, cards::LIBRARY_OF_ALEXANDRIA, PlayerId::One);
    let library_id = library.card.id;
    game.battlefield.push(library);

    let expected_origin = AbilityOrigin::Printed {
        definition: cards::LIBRARY_OF_ALEXANDRIA,
        part: CardPartId::PRIMARY,
        ability: AbilityId(1),
    };
    let activation = Action::ActivateAbility {
        source: library_id,
        ability: expected_origin,
        targets: Vec::new(),
        sacrifice: None,
        x: 0,
    };

    assert_eq!(activated_ability_for(&game, library_id, 0), expected_origin);
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.stack[0].ability_origin(), Some(expected_origin));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), 8);
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

fn game_with_test_fused_split(
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
    let definition_id = CardDefinitionId(10_066);
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
        assert!(game.trigger_event_matches(
            TriggerEventDef::SpellCast(predicate),
            &event,
            GameObjectId(99_999),
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
    let definition_id = CardDefinitionId(10_067);
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
    let mut factory = creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One);
    factory.factory_animated = true;
    let blood_moon = creature(10_001, cards::BLOOD_MOON, PlayerId::Two);
    game.battlefield = vec![factory, blood_moon];

    let snapshot = game.battlefield_exit_snapshot(&game.battlefield[0]);
    assert_eq!(snapshot.last_known.power, Some(2));
    assert_eq!(snapshot.last_known.toughness, Some(2));
    assert_eq!(snapshot.object.subtypes.as_ref(), &["Mountain"]);
    for card_type in [CardType::Land, CardType::Creature, CardType::Artifact] {
        assert!(snapshot.object.types.contains(card_type));
    }

    let event = CommittedTriggerEvent::ZoneChanged {
        object: snapshot.object,
        from: ZoneKind::Battlefield,
        to: ZoneKind::Graveyard,
    };
    for card_type in [CardType::Land, CardType::Creature, CardType::Artifact] {
        assert!(game.trigger_event_matches(
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::HasType(card_type),
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            &event,
            GameObjectId(99_999),
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
        sacrifice: None,
        x: 0,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![first_ring_id, second_ring_id],
        "the behavior-free Tome activation previews its printed four-mana cost",
    );
    assert!(game.battlefield.iter().all(|permanent| !permanent.tapped));

    let definition_id = CardDefinitionId(10_065);
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
        sacrifice: None,
        x: 0,
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
        sacrifice: Some(artifact_id),
        x: 0,
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
            sacrifice: None,
            x: 0,
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
                sacrifice: None,
                x: 0,
            })
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: strip_id,
            ability: activated_ability_for(&game, strip_id, 0),
            targets: activated_targets(Target::Permanent(opposing_id)),
            sacrifice: None,
            x: 0,
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
        sacrifice: None,
        x: 0,
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
            ability: activated_ability_for(&game, second_strip_id, 0),
            targets: activated_targets(Target::Permanent(first_strip_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    let response = Action::ActivateAbility {
        source: first_strip_id,
        ability: activated_ability_for(&game, first_strip_id, 0),
        targets: activated_targets(Target::Permanent(other_land_id)),
        sacrifice: None,
        x: 0,
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
        ability: activated_ability_for(&game, orb_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        sacrifice: None,
        x: 0,
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
        ability: activated_ability_for(&game, orb_id, 0),
        targets: activated_targets(Target::Permanent(target_id)),
        sacrifice: None,
        x: 0,
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
    javeliners.counters[CounterKind::Javelin.index()] = 1;
    javeliners.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let source = javeliners.card.id;
    game.battlefield = vec![javeliners];
    let action = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        sacrifice: None,
        x: 0,
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
            ability: activated_ability_for(&game, orb_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
            x: 0,
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
fn first_strike_kills_a_normal_blocker_before_it_can_hit_back() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut knight = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    knight.attacking = true;
    let knight_id = knight.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(knight_id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![knight, blocker];
    let opponent_life = game.players[1].life;

    game.advance_step();

    assert_eq!(game.step, Step::CombatDamage);
    assert!(game.regular_combat_damage_pending());
    assert!(
        game.observe(PlayerId::One).regular_combat_damage_pending,
        "the public observation distinguishes the priority window between damage waves",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the 2/2 first striker kills the 1/2 blocker in the strike wave",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == knight_id)
            .unwrap()
            .damage,
        0,
        "the normal blocker does not hit back in the strike wave",
    );

    pass_priority_pair(&mut game);

    assert_eq!(
        game.step,
        Step::CombatDamage,
        "a second combat-damage step begins after both players get priority",
    );
    assert!(!game.regular_combat_damage_pending());
    assert!(
        !game.observe(PlayerId::One).regular_combat_damage_pending,
        "ordinary priority after regular damage is not an inter-wave window",
    );
    assert_eq!(
        game.players[1].life, opponent_life,
        "killing the blocker does not make the first striker unblocked later",
    );
    assert_eq!(
        game.events()
            .iter()
            .filter(|event| matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::CombatDamage,
                    ..
                }
            ))
            .count(),
        2,
        "both strike waves are observable as CombatDamage steps",
    );
}

#[test]
fn delayed_combat_damage_effect_queued_between_strike_waves_fires_once() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    game.advance_step();
    assert!(
        game.regular_combat_damage_pending(),
        "the first-strike wave leaves an inter-wave priority window",
    );

    let life_before = game.players[0].life;
    game.delayed_triggers.push(DelayedTrigger {
        object: Box::new(spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        step: TurnStepDef::CombatDamage,
        player: PlayerRelation::Any,
        effect: &LOSE_ONE,
    });

    pass_priority_pair(&mut game);

    assert_eq!(game.step, Step::CombatDamage);
    assert!(
        !game.regular_combat_damage_pending(),
        "the regular combat-damage step has begun",
    );
    assert_eq!(game.players[0].life, life_before - 1);
    assert!(game.delayed_triggers.is_empty());
}

#[test]
fn first_strike_blocker_kills_a_normal_attacker_before_it_deals_damage() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    knight.blocking = Some(attacker_id);
    let knight_id = knight.card.id;
    game.battlefield = vec![attacker, knight];

    game.advance_step();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attacker_id),
        "the normal attacker dies during the first-strike damage step",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == knight_id)
            .unwrap()
            .damage,
        0,
        "the normal attacker never deals its combat damage",
    );
}

#[test]
fn double_strike_hits_an_unblocked_player_in_both_damage_steps() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);

    pass_priority_pair(&mut game);
    assert_eq!(game.step, Step::CombatDamage);
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "double strike deals damage once in each combat-damage step",
    );
}

#[test]
fn double_striker_stays_blocked_after_killing_its_only_blocker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;

    game.advance_step();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the blocker dies in the first damage step",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life, life_before,
        "a blocked nontrampling attacker cannot redirect its second hit to the player",
    );
}

#[test]
fn double_striker_can_trample_after_killing_its_only_blocker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "the strike wave assigns lethal to the blocker and tramples over",
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life,
        life_before - 10,
        "trample can assign the whole second hit after every blocker has left",
    );
}

#[test]
fn double_strike_recomputes_multi_blocker_assignment_for_the_second_step() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut first = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    first.blocking = Some(attacker_id);
    let mut second = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    second.blocking = Some(attacker_id);
    let mut blocker_ids = [first.card.id, second.card.id];
    blocker_ids.sort_unstable();
    game.battlefield = vec![attacker, first, second];

    game.advance_step();
    let first_assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: blocker_ids
            .iter()
            .copied()
            .zip([2, 0])
            .map(|(recipient, amount)| CombatDamageAssignment {
                recipient: Target::Permanent(recipient),
                amount,
            })
            .collect(),
    };
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&first_assignment)
    );
    game.apply(PlayerId::One, first_assignment).unwrap();
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .unwrap()
            .combat_damage_assignment
            .is_empty(),
    );

    pass_priority_pair(&mut game);

    assert_eq!(game.pending_combat_attackers, vec![attacker_id]);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .unwrap()
            .combat_damage_assignment
            .is_empty(),
        "the first wave's assignment cannot leak into the regular wave",
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { attacker, .. } if *attacker == attacker_id)),
        "the still-double-striking attacker assigns again against the surviving blockers",
    );
}

#[test]
fn first_strike_step_does_not_prompt_an_ineligible_multi_blocked_attacker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut first_striker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    first_striker.attacking = true;
    let mut normal_attacker = creature(10_001, cards::SU_CHI, PlayerId::One);
    normal_attacker.attacking = true;
    let normal_id = normal_attacker.card.id;
    let mut first_blocker = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    first_blocker.blocking = Some(normal_id);
    let mut second_blocker = creature(10_003, cards::SERRA_ANGEL, PlayerId::Two);
    second_blocker.blocking = Some(normal_id);
    game.battlefield = vec![
        first_striker,
        normal_attacker,
        first_blocker,
        second_blocker,
    ];

    game.advance_step();

    assert!(
        game.pending_combat_attackers.is_empty(),
        "the normal attacker is not asked to assign during the strike wave",
    );
    pass_priority_pair(&mut game);
    assert_eq!(
        game.pending_combat_attackers,
        vec![normal_id],
        "the normal attacker assigns when the regular damage step begins",
    );
}

#[test]
fn losing_double_strike_between_damage_steps_prevents_the_second_hit() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap()
        .temporary_keywords
        .retain(|keyword| *keyword != KeywordAbility::DoubleStrike);

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "a combatant in the strike-wave snapshot needs double strike now to hit again",
    );
}

#[test]
fn a_normal_attacker_that_gains_a_strike_keyword_still_hits_in_the_regular_wave() {
    for gained_keyword in [KeywordAbility::FirstStrike, KeywordAbility::DoubleStrike] {
        let mut game = ready_game();
        game.step = Step::DeclareBlockers;
        let mut first_striker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
        first_striker.attacking = true;
        let mut normal_attacker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
        normal_attacker.attacking = true;
        let normal_id = normal_attacker.card.id;
        game.battlefield = vec![first_striker, normal_attacker];

        game.advance_step();
        let life_after_first_wave = game.players[1].life;
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == normal_id)
            .unwrap()
            .temporary_keywords
            .push(gained_keyword);

        pass_priority_pair(&mut game);

        assert_eq!(
            game.players[1].life,
            life_after_first_wave - 2,
            "a normal combatant that gains {gained_keyword:?} after the strike wave remains eligible for regular damage",
        );
    }
}

#[test]
fn a_first_striker_that_gains_double_strike_hits_in_the_regular_wave() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap()
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "gaining double strike makes a first-wave combatant eligible again",
    );
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
        game.mana_ability_activations(&game.battlefield[2])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green]
    );
}

#[test]
fn lands_derive_intrinsic_mana_in_effective_subtype_order() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::PLAINS, PlayerId::One),
        creature(10_001, cards::ISLAND, PlayerId::One),
        creature(10_002, cards::SWAMP, PlayerId::One),
        creature(10_003, cards::MOUNTAIN, PlayerId::One),
        creature(10_004, cards::FOREST, PlayerId::One),
        creature(10_005, cards::TAIGA, PlayerId::One),
    ]);

    for (index, (land_type, color)) in [
        (BasicLandType::Plains, ManaColor::White),
        (BasicLandType::Island, ManaColor::Blue),
        (BasicLandType::Swamp, ManaColor::Black),
        (BasicLandType::Mountain, ManaColor::Red),
        (BasicLandType::Forest, ManaColor::Green),
    ]
    .into_iter()
    .enumerate()
    {
        let activations = game.mana_ability_activations(&game.battlefield[index]);
        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].color, color);
        assert_eq!(
            activations[0].ability,
            AbilityOrigin::IntrinsicBasicLand(land_type)
        );
    }

    let taiga = game.mana_ability_activations(&game.battlefield[5]);
    assert_eq!(
        taiga
            .iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
                ManaColor::Green,
            ),
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                ManaColor::Red,
            ),
        ]
    );
}

#[test]
fn a_basic_land_subtype_only_grants_mana_to_a_land() {
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest creature",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_creature(ManaCost::default(), &["Forest"], 1, 1);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    assert_eq!(game.effective_land_types(&game.battlefield[0]), [false; 5]);
    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty()
    );
}

#[test]
fn printed_and_intrinsic_mana_abilities_coexist() {
    static ABILITIES: [AbilityDef; 1] = [abilities::tap_for(ManaColor::Green)];
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest with printed mana",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_land(&["Forest"]).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![
            (
                AbilityOrigin::Printed {
                    definition: definition_id,
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                },
                ManaColor::Green,
            ),
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
                ManaColor::Green,
            ),
        ]
    );
}

#[test]
fn blood_moon_replaces_nonbasic_land_abilities_with_intrinsic_red_mana() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::MISHRA_S_WORKSHOP, PlayerId::One),
        creature(10_003, cards::TAIGA, PlayerId::One),
    ]);

    for permanent in &game.battlefield[1..] {
        assert_eq!(
            game.effective_land_types(permanent),
            [false, false, false, true, false]
        );
        let activations = game.mana_ability_activations(permanent);
        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].color, ManaColor::Red);
        assert_eq!(
            activations[0].ability,
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain)
        );
        assert!(activations[0].effect.restrictions.is_empty());
        assert!(
            game.effective_behavior(permanent).is_none(),
            "Blood Moon grants intrinsic rules, not a special-behavior hook"
        );
    }
}

#[test]
fn blood_moon_preserves_nonland_subtypes_on_a_land_creature() {
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest Dryad",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature_without_mana_cost(&["Forest", "Gate", "Dryad"], 1, 1)
            .with_type(CardType::Land);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let blood_moon = game.catalog.get(cards::BLOOD_MOON).unwrap().clone();
    game.catalog = CardCatalog::new([blood_moon, definition]).unwrap();
    game.turns_started[PlayerId::One.index()] = 1;
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, definition_id, PlayerId::One),
    ]);

    let permanent = &game.battlefield[1];
    let event = game.trigger_event_object(permanent);
    assert!(event.types.contains(CardType::Land));
    assert!(event.types.contains(CardType::Creature));
    assert_eq!(event.subtypes.as_ref(), &["Mountain", "Dryad"]);
    assert_eq!(
        game.mana_ability_activations(permanent)
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
            ManaColor::Red,
        )]
    );
}

#[test]
fn dryad_arbor_is_a_green_land_creature_with_summoning_sick_intrinsic_mana() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 0;
    let arbor_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One));

    let arbor = &game.battlefield[0];
    let types = game.permanent_types(arbor).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Forest", "Dryad"]
    );
    assert_eq!(
        game.effective_rules(arbor).unwrap().colors(),
        [false, false, false, false, true]
    );
    assert_eq!(
        (game.power(arbor), game.toughness(arbor)),
        (Some(1), Some(1))
    );
    assert!(
        game.mana_ability_activations(arbor).is_empty(),
        "Dryad Arbor's intrinsic tap ability observes summoning sickness",
    );

    game.turns_started[PlayerId::One.index()] = 1;
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
            ManaColor::Green,
        )],
    );
}

#[test]
fn magical_hack_changes_a_land_type_and_its_intrinsic_mana_but_preserves_dryad() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let arbor_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    let cast = cast_action(hack.id, vec![Target::Permanent(arbor_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    let arbor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == arbor_id)
        .unwrap();
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Island", "Dryad"]
    );
    assert_eq!(
        game.mana_ability_activations(arbor)
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Island),
            ManaColor::Blue,
        )],
    );
}

#[test]
fn magical_hack_can_target_a_nonland_permanent_without_basic_land_type_words() {
    let mut game = ready_game();
    let lotus_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(lotus_id.0, cards::BLACK_LOTUS, PlayerId::Two));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    let cast = cast_action(hack.id, vec![Target::Permanent(lotus_id)], Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "a nonland permanent is a legal target even when it has no words to replace",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    let lotus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lotus_id)
        .expect("the unchanged target remains on the battlefield");
    assert_eq!(
        lotus.text_changes,
        vec![BasicLandTypeChange {
            from: BasicLandType::Forest,
            to: BasicLandType::Island,
        }],
    );
    assert_eq!(
        game.permanent_types(lotus),
        Some(CardTypeSet::single(CardType::Artifact)),
    );
    assert!(game.effective_subtypes(lotus).is_empty());
}

#[test]
fn magical_hack_fizzles_without_a_choice_when_its_permanent_target_leaves() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_action(hack.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    game.destroy_permanent(land_id);
    pass_priority_pair(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MAGICAL_HACK),
    );
}

#[test]
fn magical_hack_on_stage_applies_to_land_types_that_stage_later_copies() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    let hack = card(10_002, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;
    game.apply(
        PlayerId::One,
        cast_action(hack.id, vec![Target::Permanent(stage_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: activated_ability_for(&game, stage_id, 0),
            targets: activated_targets(Target::Permanent(arbor_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let stage = &game.battlefield[0];
    assert_eq!(
        game.effective_subtypes(stage).as_ref(),
        &["Island", "Dryad"]
    );
    assert_eq!(
        game.mana_ability_activations(stage)
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue],
    );
}

#[test]
fn magical_hack_does_not_rewrite_land_types_added_by_presence() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    let mut land = creature(land_id.0, cards::MOUNTAIN, PlayerId::One);
    land.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Mountain,
        to: BasicLandType::Island,
    });
    let mut presence = creature(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    presence.attached_to = Some(land_id);
    game.battlefield.extend([land, presence]);

    assert_eq!(game.effective_land_types(&game.battlefield[0]), [true; 5]);
    let colors = game
        .mana_ability_activations(&game.battlefield[0])
        .into_iter()
        .map(|activation| activation.color)
        .collect::<Vec<_>>();
    assert_eq!(colors.len(), 5);
    for expected in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        assert!(colors.contains(&expected));
    }
}

#[test]
fn magical_hack_deduplicates_basic_types_and_intrinsic_mana() {
    let mut game = ready_game();
    let mut taiga = creature(10_000, cards::TAIGA, PlayerId::One);
    taiga.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Forest,
        to: BasicLandType::Mountain,
    });
    game.battlefield.push(taiga);

    assert_eq!(
        game.effective_subtypes(&game.battlefield[0]).as_ref(),
        &["Mountain"],
    );
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Red],
    );
}

#[test]
fn nyleas_presence_attaches_draws_and_adds_all_five_intrinsic_abilities() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::THESPIANS_STAGE, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    let library_before = game.players[0].library.len();

    let cast = cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::NYLEAS_PRESENCE)
        .expect("Nylea's Presence entered")
        .card
        .id;
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == aura_id)
            .unwrap()
            .attached_to,
        Some(land_id),
    );
    assert_eq!(game.effective_land_types(&game.battlefield[0]), [true; 5]);
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![
            ManaColor::Colorless,
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ],
    );

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].library.len(), library_before - 1);

    game.destroy_permanent(aura_id);
    assert_eq!(game.effective_land_types(&game.battlefield[0]), [false; 5]);
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Colorless],
    );
}

#[test]
fn blood_moon_and_presence_apply_land_type_operations_in_timestamp_order() {
    let target = CardInstanceId(10_001);

    let mut moon_then_presence = ready_game();
    let mut newer_presence = creature(10_002, cards::NYLEAS_PRESENCE, PlayerId::One);
    newer_presence.attached_to = Some(target);
    moon_then_presence.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(target.0, cards::THESPIANS_STAGE, PlayerId::One),
        newer_presence,
    ]);
    assert_eq!(
        moon_then_presence.effective_land_types(&moon_then_presence.battlefield[1]),
        [true; 5],
        "a newer additive effect applies after Blood Moon's set effect",
    );

    let mut presence_then_moon = ready_game();
    let mut older_presence = creature(10_000, cards::NYLEAS_PRESENCE, PlayerId::One);
    older_presence.attached_to = Some(target);
    presence_then_moon.battlefield.extend([
        older_presence,
        creature(target.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(10_002, cards::BLOOD_MOON, PlayerId::One),
    ]);
    assert_eq!(
        presence_then_moon.effective_land_types(&presence_then_moon.battlefield[1]),
        [false, false, false, true, false],
        "a newer Blood Moon set effect overwrites Presence's earlier additions",
    );
}

#[test]
fn an_aura_with_an_illegal_land_target_neither_enters_nor_draws() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    let library_before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    game.destroy_permanent(land_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::NYLEAS_PRESENCE),
    );
    assert_eq!(game.players[0].library.len(), library_before);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::NYLEAS_PRESENCE),
    );
}

#[test]
fn presence_goes_to_the_graveyard_when_its_attached_land_leaves() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    game.apply(
        PlayerId::One,
        cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::NYLEAS_PRESENCE)
        .unwrap()
        .card
        .id;
    game.destroy_permanent(land_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura_id),
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::NYLEAS_PRESENCE),
    );
}

#[test]
fn stage_copies_dryad_arbors_copiable_values_but_not_hack_or_presence() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    let aura_id = CardInstanceId(10_002);
    let stage = creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One);
    let mut arbor = creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One);
    arbor.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Forest,
        to: BasicLandType::Island,
    });
    let mut presence = creature(aura_id.0, cards::NYLEAS_PRESENCE, PlayerId::One);
    presence.attached_to = Some(arbor_id);
    game.battlefield.extend([stage, arbor, presence]);
    assert_eq!(game.effective_land_types(&game.battlefield[1]), [true; 5]);

    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(arbor_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let stage = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap();
    let types = game.permanent_types(stage).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        game.effective_subtypes(stage).as_ref(),
        &["Forest", "Dryad"]
    );
    assert_eq!(
        (game.power(stage), game.toughness(stage)),
        (Some(1), Some(1))
    );
    assert_eq!(
        game.effective_rules(stage).unwrap().colors(),
        [false, false, false, false, true]
    );
    assert!(stage.tapped, "copying does not untap or reenter Stage");
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap()
        .tapped = false;
    assert_eq!(
        game.mana_ability_activations(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == stage_id)
                .unwrap(),
        )
        .into_iter()
        .map(|activation| (activation.ability, activation.color))
        .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
            ManaColor::Green,
        )],
    );

    game.destroy_permanent(aura_id);
    let arbor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == arbor_id)
        .unwrap();
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Island", "Dryad"],
        "removing Presence reveals the earlier text change",
    );
}

#[test]
fn a_new_stage_can_copy_dryad_arbor_but_the_result_is_summoning_sick() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 0;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;

    let copy = Action::ActivateAbility {
        source: stage_id,
        ability: copy_ability,
        targets: activated_targets(Target::Permanent(arbor_id)),
        sacrifice: None,
        x: 0,
    };
    assert!(
        game.legal_actions(PlayerId::One).contains(&copy),
        "Stage is not a creature while it pays the tap cost",
    );
    game.apply(PlayerId::One, copy).unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty(),
        "the copied creature cannot use a tap ability in its controller's first turn",
    );
    game.turns_started[PlayerId::One.index()] = 1;
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green],
    );
}

#[test]
fn stage_copying_stage_does_not_duplicate_indistinguishable_legal_actions() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let copying_stage = CardInstanceId(10_000);
    let copied_stage = CardInstanceId(10_001);
    let mountain = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(copying_stage.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(copied_stage.0, cards::THESPIANS_STAGE, PlayerId::Two),
        creature(mountain.0, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let copy_ability = activated_ability_for(&game, copying_stage, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: copying_stage,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(copied_stage)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;
    game.players[0].mana_pool.colorless = 2;

    assert_eq!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .filter(|effective| {
                effective.origin == copy_ability
                    && matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::Activated(_)
                    )
            })
            .count(),
        2,
        "both copiable Stage abilities remain part of the permanent",
    );

    let copy_mountain = Action::ActivateAbility {
        source: copying_stage,
        ability: copy_ability,
        targets: activated_targets(Target::Permanent(mountain)),
        sacrifice: None,
        x: 0,
    };
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|action| **action == copy_mountain)
            .count(),
        1,
        "the two rules-identical Stage abilities produce one external action",
    );
}

#[test]
fn stage_keeps_a_resolved_factory_animation_after_copying_another_land() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let factory_id = CardInstanceId(10_001);
    let mountain_id = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(factory_id.0, cards::MISHRA_S_FACTORY, PlayerId::One),
        creature(mountain_id.0, cards::MOUNTAIN, PlayerId::Two),
    ]);

    let original_copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: original_copy_ability,
            targets: activated_targets(Target::Permanent(factory_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let animate = Action::ActivateAbility {
        source: stage_id,
        ability: activated_ability_for(&game, stage_id, 0),
        targets: Vec::new(),
        sacrifice: None,
        x: 0,
    };
    game.players[0].mana_pool.colorless = 1;
    assert!(
        game.legal_actions(PlayerId::One).contains(&animate),
        "the copied Factory animation coexists with Stage's retained ability",
    );
    game.apply(PlayerId::One, animate).unwrap();
    assert!(game.battlefield[0].factory_animated);

    let retained_copy_ability = activated_ability_for(&game, stage_id, 2);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: retained_copy_ability,
            targets: activated_targets(Target::Permanent(mountain_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let stage = &game.battlefield[0];
    let types = game.permanent_types(stage).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Artifact));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        (game.power(stage), game.toughness(stage)),
        (Some(2), Some(2))
    );
    assert_eq!(game.effective_behavior(stage), None);

    let pump = Action::ActivateAbility {
        source: factory_id,
        ability: activated_ability_for(&game, factory_id, 1),
        targets: activated_targets(Target::Permanent(stage_id)),
        sacrifice: None,
        x: 0,
    };
    assert!(
        game.legal_actions(PlayerId::One).contains(&pump),
        "the still-animated object remains an Assembly-Worker pump target",
    );
}

#[test]
fn stage_does_not_copy_a_land_that_leaves_before_the_ability_resolves() {
    let mut game = ready_game();
    let stage_id = CardInstanceId(10_000);
    let target_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(target_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: activated_ability_for(&game, stage_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    game.destroy_permanent(target_id);
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let stage = &game.battlefield[0];
    assert!(stage.copy_effect.is_none());
    assert_eq!(
        game.mana_ability_activations(stage)
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Colorless],
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
    let copied_types = game.permanent_types(copied).unwrap();
    assert!(copied_types.contains(CardType::Artifact));
    assert!(copied_types.contains(CardType::Creature));
    assert!(
        copied_types.contains(CardType::Enchantment),
        "Copy Artifact retains its copy-process type exception",
    );
    assert_eq!(game.power(copied), Some(4));
    assert!(game.has_flying(copied));
}

#[test]
fn copy_artifact_resolves_a_copied_icy_manipulator_ability_from_its_frozen_origin() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::Two),
        creature(10_001, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let copy = card(10_002, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(
            copy.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let copied_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved")
        .card
        .id;
    let target_id = CardInstanceId(10_001);
    let ability = activated_ability_for(&game, copied_id, 0);
    assert_eq!(ability, primary_ability(cards::ICY_MANIPULATOR));

    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: copied_id,
            ability,
            targets: activated_targets(Target::Permanent(target_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].card.definition, cards::ICY_MANIPULATOR);
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ICY_MANIPULATOR))
    );
    assert_eq!(
        game.observe(PlayerId::One).stack[0].definition,
        cards::ICY_MANIPULATOR,
        "stack presentation follows the frozen copied ability definition",
    );

    game.destroy_permanent(copied_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .is_some_and(|permanent| permanent.tapped),
        "the copied Icy ability resolves after its physical source leaves play",
    );
}

#[test]
fn granted_activation_freezes_payload_before_sacrificing_grant_source() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "any target",
        AbilityTargetPredicate::AnyTarget,
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated(
        "Sacrifice an artifact: This creature deals 2 damage to any target.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            controller: PlayerRelation::You,
        }],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(0)),
            amount: ValueDef::Constant(2),
        },
    )
    .with_targets(&TARGETS);
    static GRANTOR_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures you control have the test ability.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )];
    let grantor_definition_id = CardDefinitionId(10_062);
    let mut grantor_definition = CardDefinition::new(
        grantor_definition_id,
        "Activated snapshot test grantor",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    grantor_definition.rules =
        CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&GRANTOR_ABILITIES);
    synchronize_single_part_definition(&mut grantor_definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(grantor_definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let grantor = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(grantor.0, grantor_definition_id, PlayerId::One),
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);
    let origin = AbilityOrigin::Granted {
        source: grantor,
        source_definition: grantor_definition_id,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    };
    let activation = Action::ActivateAbility {
        source: receiver,
        ability: origin,
        targets: activated_targets(Target::Player(PlayerId::Two)),
        sacrifice: Some(grantor),
        x: 0,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != grantor),
        "the continuous-effect source was sacrificed as the activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen stack payload");
    assert_eq!(payload.origin, origin);
    assert_eq!(payload.target_defs, &TARGETS);
    assert_eq!(
        payload.targets,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
    );
    assert!(matches!(
        payload.resolver,
        StackAbilityResolver::Declarative(EffectDef::DealDamage { .. })
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 18,
        "resolution must use the definition frozen before the grant disappeared",
    );
}

#[test]
fn separate_grant_sites_receive_distinct_structural_origins() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has flying.\nThis permanent has flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId(10_063);
    let mut definition = CardDefinition::new(
        definition_id,
        "Grant identity test card",
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
    let source = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));

    let granted = game
        .effective_abilities(&game.battlefield[0])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. } | AbilityOrigin::IntrinsicBasicLand(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![
            AbilityOrigin::Granted {
                source,
                source_definition: definition_id,
                source_part: CardPartId::PRIMARY,
                source_ability: AbilityId::PRIMARY,
                grant: GrantId::PRIMARY,
            },
            AbilityOrigin::Granted {
                source,
                source_definition: definition_id,
                source_part: CardPartId::PRIMARY,
                source_ability: AbilityId::PRIMARY,
                grant: GrantId(1),
            },
        ]
    );
}

#[test]
fn a_nonmatching_grant_site_still_advances_the_structural_origin() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has flying. Creatures you control have flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId(10_080);
    let mut definition = CardDefinition::new(
        definition_id,
        "Nonmatching grant identity test card",
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
    let source = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);

    let granted = game
        .effective_abilities(&game.battlefield[1])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. } | AbilityOrigin::IntrinsicBasicLand(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![AbilityOrigin::Granted {
            source,
            source_definition: definition_id,
            source_part: CardPartId::PRIMARY,
            source_ability: AbilityId::PRIMARY,
            grant: GrantId(1),
        }]
    );
}

#[test]
fn nonmatching_composite_grant_sites_still_advance_structural_origins() {
    static GRANTED_ABILITY: AbilityDef = abilities::flying();
    static MISSED_COMPONENTS: [AppliedEffectDef; 1] =
        [AppliedEffectDef::GrantAbility(&GRANTED_ABILITY)];
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::Apply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&MISSED_COMPONENTS),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "The attached permanent has flying.\nThis permanent has flying.",
        EffectDef::Sequence(&EFFECTS),
    )];
    let definition_id = CardDefinitionId(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Conditional composite grant identity test card",
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
    let source = CardInstanceId(10_001);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));

    let granted = game
        .effective_abilities(&game.battlefield[0])
        .into_iter()
        .filter_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. } | AbilityOrigin::IntrinsicBasicLand(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        granted,
        vec![AbilityOrigin::Granted {
            source,
            source_definition: definition_id,
            source_part: CardPartId::PRIMARY,
            source_ability: AbilityId::PRIMARY,
            grant: GrantId(1),
        }]
    );
}

static COPY_GRANT_A: AbilityDef = AbilityDef::activated(
    "Gain 1 life.",
    &[],
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);
static COPY_GRANT_B: AbilityDef = AbilityDef::activated(
    "Lose 1 life.",
    &[],
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);
static COPY_GRANT_SOURCE_A_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Creatures you control have the first test ability.",
    EffectDef::Apply {
        recipient: EffectRecipientDef::MatchingObjects {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        },
        effect: AppliedEffectDef::GrantAbility(&COPY_GRANT_A),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    },
)];
static COPY_GRANT_SOURCE_B_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Creatures you control have the second test ability.",
    EffectDef::Apply {
        recipient: EffectRecipientDef::MatchingObjects {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        },
        effect: AppliedEffectDef::GrantAbility(&COPY_GRANT_B),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    },
)];

fn copy_grant_source_definition(
    id: CardDefinitionId,
    name: &'static str,
    abilities: &'static [AbilityDef],
) -> CardDefinition {
    let mut definition = CardDefinition::new(
        id,
        name,
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(abilities);
    synchronize_single_part_definition(&mut definition);
    definition
}

fn copied_grant_source_game() -> (
    Game,
    CardInstanceId,
    CardInstanceId,
    CardDefinitionId,
    CardDefinitionId,
) {
    let definition_a = CardDefinitionId(10_064);
    let definition_b = CardDefinitionId(10_065);
    let source_a = copy_grant_source_definition(
        definition_a,
        "First grant source",
        &COPY_GRANT_SOURCE_A_ABILITIES,
    );
    let source_b = copy_grant_source_definition(
        definition_b,
        "Second grant source",
        &COPY_GRANT_SOURCE_B_ABILITIES,
    );
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([source_a, source_b]);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let grantor = CardInstanceId(10_000);
    let receiver = CardInstanceId(10_001);
    let mut copied_source = creature(grantor.0, cards::COPY_ARTIFACT, PlayerId::One);
    copied_source.copy_effect = Some(copied_characteristics(definition_a));
    game.battlefield.extend([
        copied_source,
        creature(receiver.0, cards::ATOG, PlayerId::One),
    ]);
    (game, grantor, receiver, definition_a, definition_b)
}

fn sole_granted_origin(game: &Game, receiver: CardInstanceId) -> AbilityOrigin {
    let receiver = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == receiver)
        .expect("the granted-ability receiver is on the battlefield");
    game.effective_abilities(receiver)
        .into_iter()
        .find_map(|effective| match effective.origin {
            AbilityOrigin::Granted { .. } => Some(effective.origin),
            AbilityOrigin::Printed { .. } | AbilityOrigin::IntrinsicBasicLand(_) => None,
        })
        .expect("the copied source grants an ability")
}

const fn copied_grant_origin(
    grantor: CardInstanceId,
    definition: CardDefinitionId,
) -> AbilityOrigin {
    AbilityOrigin::Granted {
        source: grantor,
        source_definition: definition,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    }
}

#[test]
fn copied_grant_source_definition_is_part_of_the_granted_ability_origin() {
    let (mut game, grantor, receiver, definition_a, definition_b) = copied_grant_source_game();
    let first_origin = sole_granted_origin(&game, receiver);
    assert_eq!(first_origin, copied_grant_origin(grantor, definition_a));
    let stale_action = Action::ActivateAbility {
        source: receiver,
        ability: first_origin,
        targets: Vec::new(),
        sacrifice: None,
        x: 0,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&stale_action));

    game.battlefield[0].copy_effect = Some(copied_characteristics(definition_b));
    let second_origin = sole_granted_origin(&game, receiver);
    assert_eq!(second_origin, copied_grant_origin(grantor, definition_b));
    assert_ne!(first_origin, second_origin);
    let current_actions = game.legal_actions(PlayerId::One);
    assert!(
        !current_actions.contains(&stale_action),
        "a stale action must not alias a same-position grant from different copied rules",
    );
    assert!(current_actions.contains(&Action::ActivateAbility {
        source: receiver,
        ability: second_origin,
        targets: Vec::new(),
        sacrifice: None,
        x: 0,
    }));
}

static MULTI_SLOT_ACTIVATION_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(
        TargetSlotId(3),
        "opponent",
        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
    ),
    AbilityTargetDef::exactly_one(
        TargetSlotId(7),
        "creature an opponent controls",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    ),
];
static MULTI_SLOT_ACTIVATION_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetSlotId(3)),
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetSlotId(7)),
        amount: ValueDef::Constant(1),
    },
];
static MULTI_SLOT_ACTIVATION_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "Sacrifice this artifact: It deals 1 damage to target opponent and 1 damage to target creature that player controls.",
    &[AbilityCostDef::SacrificeSource],
    EffectDef::Sequence(&MULTI_SLOT_ACTIVATION_EFFECTS),
)
.with_targets(&MULTI_SLOT_ACTIVATION_TARGETS)];

#[test]
fn declarative_activation_preserves_multiple_slots_before_sacrificing_its_source() {
    let definition_id = CardDefinitionId(10_063);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-slot activation test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0))
        .with_abilities(&MULTI_SLOT_ACTIVATION_ABILITIES);
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
    let source = CardInstanceId(10_000);
    let creature_target = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(creature_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let targets = vec![
        TargetSelection::single(TargetSlotId(3), Target::Player(PlayerId::Two)),
        TargetSelection::single(TargetSlotId(7), Target::Permanent(creature_target)),
    ];
    let activation = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: targets.clone(),
        sacrifice: None,
        x: 0,
    };

    let invalid_slots = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![
            TargetSelection::single(TargetSlotId(7), Target::Player(PlayerId::Two)),
            TargetSelection::single(TargetSlotId(3), Target::Permanent(creature_target)),
        ],
        sacrifice: None,
        x: 0,
    };
    assert!(game.apply(PlayerId::One, invalid_slots).is_err());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == source),
        "slot validation must happen before sacrificing the source",
    );
    assert!(game.stack.is_empty());

    assert!(
        game.legal_actions(PlayerId::One).contains(&activation),
        "declarative action generation must retain abilities with multiple target slots",
    );
    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "the source was sacrificed as an activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen payload");
    assert_eq!(payload.target_defs, &MULTI_SLOT_ACTIVATION_TARGETS);
    assert_eq!(payload.targets, targets);

    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature_target)
            .is_some_and(|permanent| permanent.damage == 1),
    );
}

#[test]
fn one_ability_target_slot_resolves_for_every_selected_legal_target() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
        id: TargetSlotId(5),
        label: "up to two creatures an opponent controls",
        predicate: AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
        minimum: 1,
        maximum: 2,
    }];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Deal 1 damage to up to two target creatures an opponent controls.",
        &[],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(5)),
            amount: ValueDef::Constant(1),
        },
    )
    .with_targets(&TARGETS)];

    let definition_id = CardDefinitionId(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-target slot test card",
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
    let source = CardInstanceId(10_000);
    let first_target = CardInstanceId(10_001);
    let second_target = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(first_target.0, cards::SERRA_ANGEL, PlayerId::Two),
        creature(second_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let action = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![TargetSelection::new(
            TargetSlotId(5),
            vec![
                Target::Permanent(first_target),
                Target::Permanent(second_target),
            ],
        )],
        sacrifice: None,
        x: 0,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    for target in [first_target, second_target] {
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == target)
                .is_some_and(|permanent| permanent.damage == 1),
            "every legal target selected in the slot receives the effect",
        );
    }
}

#[test]
fn granted_ability_keeps_its_frozen_resolver_when_the_source_changes() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "permanent",
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated(
        "{T}: Tap target permanent.",
        &[AbilityCostDef::TapSource],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetSlotId(0)),
        },
    )
    .with_targets(&TARGETS)
    .with_implementation(AbilityImplementationDef::CustomFull {
        behavior: Some(CardBehavior::IcyManipulator),
        explanation: "The test intentionally grants a custom resolver.",
    });
    static SOURCE_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has the test ability.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&GRANTED_ABILITY),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )];
    let definition_id = CardDefinitionId(10_061);
    let mut definition = CardDefinition::new(
        definition_id,
        "Granted resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&SOURCE_ABILITIES);
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
    game.battlefield.extend([
        creature(10_000, definition_id, PlayerId::One),
        creature(10_001, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let source = CardInstanceId(10_000);
    let target = CardInstanceId(10_001);
    let source_card = game.battlefield[0].card.clone();
    let origin = AbilityOrigin::Granted {
        source,
        source_definition: definition_id,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    };
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Permanent(target)),
        Vec::new(),
    );
    assert_eq!(game.stack[0].ability_origin(), Some(origin));
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Custom(CardBehavior::IcyManipulator))
    ));

    // This models a continuous/copy effect changing the effective rules of a
    // source after activation. The origin remains provenance, while the stack
    // object's executable payload must remain the Icy Manipulator procedure.
    game.battlefield[0].copy_effect = Some(copied_characteristics(cards::JAYEMDAE_TOME));
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .is_some_and(|permanent| permanent.tapped),
        "resolution must not rediscover a different handler from the changed source",
    );
}

#[test]
fn declarative_clause_uses_its_own_resolver_on_a_card_with_custom_behavior() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        TargetSlotId(0),
        "any target",
        AbilityTargetPredicate::AnyTarget,
    )];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "Deal 1 damage to any target.",
            &[],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                amount: ValueDef::Constant(1),
            },
        )
        .with_targets(&TARGETS),
        AbilityDef::custom_full(
            "A separate custom clause.",
            CardBehavior::IcyManipulator,
            "The test keeps one explicitly custom clause beside the declarative clause.",
        ),
    ];
    let definition_id = CardDefinitionId(10_060);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mixed resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_060, definition_id, PlayerId::One));
    let source = CardInstanceId(10_060);
    let source_card = game.battlefield[0].card.clone();
    let origin = primary_ability(definition_id);
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Player(PlayerId::Two)),
        Vec::new(),
    );
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Declarative(
            EffectDef::DealDamage { .. }
        ))
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 19,
        "the selected definition must not dispatch through Icy's unrelated hook",
    );
}

#[test]
fn resolving_ability_masks_an_illegal_target_in_each_frozen_slot() {
    static TARGETS: [AbilityTargetDef; 2] = [
        AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "first creature you control",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        ),
        AbilityTargetDef::exactly_one(
            TargetSlotId(1),
            "second creature you control",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        ),
    ];
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(0)),
            amount: ValueDef::Constant(1),
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetSlotId(1)),
            amount: ValueDef::Constant(1),
        },
    ];

    let mut game = ready_game();
    let source = CardInstanceId(10_000);
    let first = CardInstanceId(10_001);
    let second = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(first.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(second.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    game.stack.push(StackObject {
        id: StackObjectId(20_000),
        kind: StackObjectKind::TriggeredAbility,
        card: card(20_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        source: Some(source),
        ability: Some(StackAbilityPayload {
            origin: primary_ability(cards::ANKH_OF_MISHRA),
            definition: None,
            presentation_definition: cards::ANKH_OF_MISHRA,
            text: Some("Test two-slot trigger"),
            target_defs: TARGETS.to_vec(),
            targets: vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(first)),
                TargetSelection::single(TargetSlotId(1), Target::Permanent(second)),
            ],
            context: TriggerContext {
                object: None,
                object_controller: None,
                event_player: None,
                amount: None,
            },
            resolver: StackAbilityResolver::Declarative(EffectDef::Sequence(&EFFECTS)),
            condition: None,
            mode_effects: Vec::new(),
            x: 0,
        }),
        controller: PlayerId::One,
        signature: None,
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        text_changes: Vec::new(),
        cast_via_flashback: false,
        is_copy: false,
    });

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == first)
        .unwrap()
        .controller = PlayerId::Two;
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == first)
            .unwrap()
            .damage,
        0,
        "an illegal target in one slot is ignored",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second)
            .unwrap()
            .damage,
        1,
        "the legal target in the other slot still receives its effect",
    );
}

#[test]
fn copy_artifact_copies_declarative_mana_abilities_without_a_behavior_hook() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SOL_RING, PlayerId::Two));
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
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    let copied_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved")
        .card
        .id;
    let ability = mana_ability_for(&game, copied_id, ManaColor::Colorless);
    assert_eq!(ability, primary_ability(cards::SOL_RING));
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: copied_id,
            ability,
            color: ManaColor::Colorless,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 2);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == copied_id)
            .is_some_and(|permanent| permanent.tapped)
    );
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
fn wrath_and_supreme_verdict_use_equivalent_declarative_creature_sweepers() {
    let game = ready_game();
    for (definition, can_regenerate, cannot_be_countered) in [
        (cards::WRATH_OF_GOD, false, false),
        (cards::SUPREME_VERDICT, true, true),
    ] {
        let definition = game.catalog.get(definition).unwrap();
        assert_eq!(definition.rules.special_behavior(), None);
        assert!(
            definition
                .rules
                .ability_clauses()
                .iter()
                .all(|ability| ability.implementation == AbilityImplementationDef::Definition)
        );
        assert!(definition.rules.ability_clauses().iter().any(|ability| {
            let EffectDef::Destroy {
                object:
                    EffectRecipientDef::MatchingObjects {
                        object,
                        zones,
                        controller,
                    },
                can_regenerate: actual,
            } = ability.effect
            else {
                return false;
            };
            object == ObjectPredicateDef::HasType(CardType::Creature)
                && zones == [ZoneKind::Battlefield]
                && controller == PlayerRelation::Any
                && actual == can_regenerate
        }));
        assert_eq!(
            definition.rules.ability_clauses().iter().any(|ability| {
                matches!(
                    ability.effect,
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::CannotBeCountered,
                        ..
                    }
                )
            }),
            cannot_be_countered,
        );
    }
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

    game.battlefield[0].regeneration_shields = 1;

    let wrath = spell(10_001, cards::WRATH_OF_GOD, PlayerId::Two, 0);
    let effect = game
        .catalog
        .get(cards::WRATH_OF_GOD)
        .expect("Wrath of God is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect;
    game.resolve_effect_def(effect, &wrath, TriggerContext::empty());
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
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if targets.is_empty() && *source == source_id)
            })
            .expect("the ability is offered with an untapped Mountain and an empty pool");

        let before = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
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
            .map(|permanent| {
                (
                    game.power(permanent),
                    game.toughness(permanent),
                    game.has_flying(permanent),
                )
            })
            .unwrap();
        match definition {
            cards::DRAGON_WHELP => {
                assert_eq!(
                    after.0,
                    before.0.map(|value| value + 1),
                    "Dragon Whelp grew"
                );
            }
            cards::GOBLIN_BALLOON_BRIGADE => {
                assert!(!before.2);
                assert!(after.2, "Goblin Balloon Brigade gained flying");
            }
            cards::GRANITE_GARGOYLE => {
                assert_eq!(
                    after.1,
                    before.1.map(|value| value + 1),
                    "Granite Gargoyle gained toughness",
                );
            }
            _ => unreachable!(),
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
    let sign = card(10_000, cards::SIGN_IN_BLOOD, PlayerId::One);
    game.players[0].hand.push(sign.clone());
    game.players[0].mana_pool.black = 2;
    game.apply(
        PlayerId::One,
        cast_action(sign.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

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
    game.battlefield
        .push(creature(10_004, cards::JUGGERNAUT, PlayerId::Two)); // artifact

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
    assert!(
        blockers.contains(&CardInstanceId(10_004)),
        "an artifact creature may block regardless of colour",
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
    assert_eq!(
        returned.counters[CounterKind::PlusOnePlusOne.index()],
        1,
        "with a +1/+1 counter"
    );
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
fn declarative_ritual_psionic_blast_and_sign_in_blood_resolve() {
    let mut game = ready_game();
    let ritual = card(10_000, cards::DARK_RITUAL, PlayerId::One);
    game.players[0].hand.push(ritual.clone());
    game.players[0].mana_pool.black = 1;
    game.apply(
        PlayerId::One,
        cast_action(ritual.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.black, 3);

    let mut game = ready_game();
    let blast = card(10_000, cards::PSIONIC_BLAST, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(blast.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 18);
    assert_eq!(game.players[1].life, 16);

    let mut game = ready_game();
    let sign = card(10_000, cards::SIGN_IN_BLOOD, PlayerId::One);
    game.players[0].hand.push(sign.clone());
    game.players[0].mana_pool.black = 2;
    game.apply(
        PlayerId::One,
        cast_action(sign.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert_eq!(game.players[1].life, 18);
}

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
        .push(KeywordAbility::ProtectionFrom(ManaColor::Red));
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
        overload.effect,
        EffectDef::Counter {
            object: EffectRecipientDef::MatchingObjects {
                controller: PlayerRelation::NotYou,
                ..
            }
        }
    ));
}

#[test]
fn a_non_executable_cannot_be_countered_clause_does_not_change_gameplay() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::CannotBeCountered,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
    .with_source_zones(&[ZoneKind::Stack])
    .with_implementation(AbilityImplementationDef::NotImplemented {
        explanation: "Test-only incomplete clause.",
    })];
    let definition_id = CardDefinitionId(20_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Incomplete uncounterable spell",
        CardSet::ReturnToRavnica,
        false,
        CardBehavior::Unsupported,
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
    static COMPONENTS: [AppliedEffectDef; 1] = [AppliedEffectDef::CannotBeCountered];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Composite(&COMPONENTS),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
    .with_source_zones(&[ZoneKind::Stack])];
    let definition_id = CardDefinitionId(20_001);
    let mut definition = CardDefinition::new(
        definition_id,
        "Composite uncounterable spell",
        CardSet::ReturnToRavnica,
        false,
        CardBehavior::Unsupported,
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
    static COMPONENTS: [AppliedEffectDef; 1] = [AppliedEffectDef::CannotBeCountered];
    let mut object = spell(20_002, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    object.applied_effects.push(AppliedStackEffect {
        source: None,
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

    let definition_id = CardDefinitionId(20_003);
    let mut definition = CardDefinition::new(
        definition_id,
        "Modal overload test",
        CardSet::ReturnToRavnica,
        false,
        CardBehavior::Unsupported,
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
        game.validated_cast_signature(PlayerId::One, card_id, &forged)
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

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(game.battlefield[0].card.id));
    assert!(game.observe(PlayerId::One).decision.is_none());
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
fn any_target_damage_can_remove_a_planeswalker() {
    let definition_id = CardDefinitionId(10_075);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Planeswalker",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_planeswalker(ManaCost::default(), &["Test"], 3)
        .with_supertype(CardSupertype::Legendary);
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
    let mut planeswalker = creature(10_000, definition_id, PlayerId::Two);
    planeswalker.loyalty = Some(3);
    let planeswalker_id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 1;

    let action = cast_action(
        bolt.id,
        vec![Target::Permanent(planeswalker_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != planeswalker_id)
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == definition_id)
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

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(game.battlefield[0].card.id));
    assert!(game.observe(PlayerId::One).decision.is_none());
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

/// Casts a creature, resolves it, and explicitly targets player two while its
/// reveal-and-exile trigger is being put on the stack.
fn cast_and_place_reveal_trigger(game: &mut Game, instance: u32, definition: CardDefinitionId) {
    let creature = card(instance, definition, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    game.players[0].mana_pool.white = 3;
    game.players[0].mana_pool.black = 3;
    game.players[0].mana_pool.blue = 3;
    game.players[0].mana_pool.colorless = 3;
    game.apply(
        PlayerId::One,
        cast_action(creature.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(game);

    let placement = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger's controller chooses its target");
    assert_eq!(placement.kind, DecisionKind::TriggerPlacement);
    assert_eq!(placement.visibility, DecisionVisibility::Public);
    assert_eq!(placement.minimum, 1);
    assert_eq!(placement.maximum, 1);
    assert_eq!(placement.options.len(), 1, "there is one opponent");
    assert!(
        game.stack.is_empty(),
        "the target is chosen before placement"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: placement.id,
            options: vec![placement.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the trigger freezes its rules payload");
    assert_eq!(
        payload.targets,
        vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
        "resolution uses the opponent selected during trigger placement",
    );
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "the hand choice waits until the trigger resolves",
    );
}

#[test]
fn sin_collector_exiles_an_instant_or_sorcery_from_the_revealed_hand() {
    let mut game = ready_game();
    let lions = card(12_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let bolt = card(12_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.extend([lions.clone(), bolt.clone()]);
    cast_and_place_reveal_trigger(&mut game, 12_100, cards::SIN_COLLECTOR);

    pass_priority_pair(&mut game);

    // Only the Bolt qualifies; the creature is not offered.
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(_, definition)| definition))
        .collect();
    assert_eq!(offered, vec![cards::LIGHTNING_BOLT]);
    assert_eq!(
        game.observe(PlayerId::One).last_seen_hand,
        Some((
            PlayerId::Two,
            vec![(lions.id, lions.definition), (bolt.id, bolt.definition)],
        )),
        "revealing the hand exposes ineligible cards too",
    );

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(game.players[1].exile[0].definition, cards::LIGHTNING_BOLT);
    assert_eq!(game.players[1].hand.len(), 1, "the creature stays");
    assert!(
        game.players[1].graveyard.is_empty(),
        "exiled, not discarded"
    );
}

#[test]
fn sin_collector_can_be_responded_to_with_an_eligible_instant() {
    let mut game = ready_game();
    let lions = card(12_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let bolt = card(12_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.extend([lions.clone(), bolt.clone()]);
    game.players[1].mana_pool.red = 1;
    cast_and_place_reveal_trigger(&mut game, 12_100, cards::SIN_COLLECTOR);

    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let response = cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::Two).contains(&response));
    game.apply(PlayerId::Two, response).unwrap();
    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 17);
    assert_eq!(game.stack.len(), 1, "Sin Collector's trigger remains");
    pass_priority_pair(&mut game);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "the Bolt is no longer in hand when the trigger resolves",
    );
    assert_eq!(
        game.observe(PlayerId::One).last_seen_hand,
        Some((PlayerId::Two, vec![(lions.id, lions.definition)])),
        "the hand is revealed at resolution, after the response",
    );
    assert!(game.players[1].exile.is_empty());
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
    );
}

#[test]
fn lifebane_zombie_only_takes_green_or_white_creatures() {
    let mut game = ready_game();
    for (instance, definition) in [
        (12_000, cards::SAVANNAH_LIONS), // white creature
        (12_001, cards::ARBOR_ELF),      // green creature
        (12_002, cards::JUZAM_DJINN),    // black creature
        (12_003, cards::LIGHTNING_BOLT), // not a creature
    ] {
        game.players[1]
            .hand
            .push(card(instance, definition, PlayerId::Two));
    }
    cast_and_place_reveal_trigger(&mut game, 12_100, cards::LIFEBANE_ZOMBIE);
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(_, definition)| definition))
        .collect();
    assert_eq!(offered, vec![cards::SAVANNAH_LIONS, cards::ARBOR_ELF]);

    let elf = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, card)| card == cards::ARBOR_ELF)
        })
        .expect("the green creature is eligible");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![elf.id],
        },
    )
    .unwrap();
    assert_eq!(game.players[1].exile[0].definition, cards::ARBOR_ELF);
}

#[test]
fn a_reveal_and_exile_creature_asks_nothing_of_an_empty_hand() {
    let mut game = ready_game();
    game.players[1].hand.clear();
    cast_and_place_reveal_trigger(&mut game, 12_100, cards::SIN_COLLECTOR);
    pass_priority_pair(&mut game);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "nothing to take, so nothing to ask"
    );
    assert_eq!(
        game.observe(PlayerId::One).last_seen_hand,
        Some((PlayerId::Two, Vec::new())),
        "the empty hand was still revealed",
    );
    assert_eq!(game.battlefield.len(), 1, "the creature still arrives");
}

#[test]
fn declarative_destroy_spells_enforce_their_target_types_and_resolve() {
    for (spell_definition, target_definition, colored_mana) in [
        (cards::SHATTER, cards::BLACK_VISE, ManaColor::Red),
        (cards::SHATTER, cards::JUGGERNAUT, ManaColor::Red),
        (cards::DISENCHANT, cards::ENERGY_FLUX, ManaColor::White),
        (cards::DISENCHANT, cards::JUGGERNAUT, ManaColor::White),
        (cards::SINKHOLE, cards::MOUNTAIN, ManaColor::Black),
        (
            cards::URGENT_EXORCISM,
            cards::STRANGLEROOT_GEIST,
            ManaColor::White,
        ),
        (cards::STONE_RAIN, cards::MOUNTAIN, ManaColor::Red),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, target_definition, PlayerId::Two);
        let target_id = target.card.id;
        let spell = card(10_001, spell_definition, PlayerId::One);
        game.battlefield.push(target);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.colorless = 3;
        match colored_mana {
            ManaColor::White => game.players[0].mana_pool.white = 2,
            ManaColor::Blue => game.players[0].mana_pool.blue = 2,
            ManaColor::Black => game.players[0].mana_pool.black = 2,
            ManaColor::Red => game.players[0].mana_pool.red = 2,
            ManaColor::Green => game.players[0].mana_pool.green = 2,
            ManaColor::Colorless => game.players[0].mana_pool.colorless = 5,
        }
        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert!(
            game.legal_actions(PlayerId::One).contains(&action),
            "{spell_definition:?} accepts its declared target type",
        );
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != target_id),
            "{spell_definition:?} destroys its target on resolution",
        );
    }
}

#[test]
fn sage_and_relic_barrier_use_the_shared_activated_ability_stack() {
    let mut game = ready_game();
    let sage = creature(10_000, cards::SAGE_OF_LAT_NAM, PlayerId::One);
    let sage_id = sage.card.id;
    let ring = creature(10_001, cards::SOL_RING, PlayerId::One);
    let ring_id = ring.card.id;
    game.battlefield = vec![sage, ring];
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: sage_id,
            ability: activated_ability_for(&game, sage_id, 0),
            targets: Vec::new(),
            sacrifice: Some(ring_id),
            x: 0,
        },
    )
    .unwrap();
    assert!(game.battlefield[0].tapped);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != ring_id)
    );
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);

    let mut game = ready_game();
    let barrier = creature(10_000, cards::RELIC_BARRIER, PlayerId::One);
    let barrier_id = barrier.card.id;
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield = vec![barrier, ring];
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: barrier_id,
            ability: activated_ability_for(&game, barrier_id, 0),
            targets: activated_targets(Target::Permanent(ring_id)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    assert!(game.battlefield[0].tapped);
    assert!(!game.battlefield[1].tapped);
    pass_priority_pair(&mut game);
    assert!(game.battlefield[1].tapped);
}

#[test]
fn migrated_upkeep_and_death_triggers_resolve_from_the_stack() {
    for (definition, active_player, damaged_player) in [
        (cards::COPPER_TABLET, PlayerId::Two, PlayerId::Two),
        (cards::JUZAM_DJINN, PlayerId::One, PlayerId::One),
        (cards::SERENDIB_EFREET, PlayerId::One, PlayerId::One),
    ] {
        let mut game = ready_game();
        game.active_player = active_player;
        game.priority = active_player;
        game.step = Step::Upkeep;
        game.battlefield
            .push(creature(10_000, definition, PlayerId::One));
        game.handle_upkeep_triggers();
        game.finish_rules_procedure();
        assert_eq!(game.stack.len(), 1);
        pass_priority_pair(&mut game);
        assert_eq!(game.players[damaged_player.index()].life, 19);
        assert_eq!(game.players[damaged_player.opponent().index()].life, 20);
    }

    let mut game = ready_game();
    let vampire = creature(10_000, cards::SENGIR_VAMPIRE, PlayerId::One);
    let vampire_id = vampire.card.id;
    let lion = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lion_id = lion.card.id;
    game.battlefield = vec![vampire, lion];
    game.damage_target_from(Some(vampire_id), Some(Target::Permanent(lion_id)), 1);
    game.check_state_based_actions();
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    let vampire = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vampire_id)
        .unwrap();
    assert_eq!(vampire.counters[CounterKind::PlusOnePlusOne.index()], 1);
    assert_eq!(game.power(vampire), Some(5));
}

#[test]
fn state_based_actions_repeat_after_static_toughness_bonuses_disappear() {
    let mut game = ready_game();
    let mut first_king = creature(10_000, cards::GOBLIN_KING, PlayerId::One);
    first_king.damage = 3;
    let mut second_king = creature(10_001, cards::GOBLIN_KING, PlayerId::One);
    second_king.damage = 2;
    let mut balloon = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    balloon.damage = 1;
    game.battlefield = vec![first_king, second_king, balloon];

    game.check_state_based_actions();

    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 3);
}

#[test]
fn simultaneous_deaths_use_the_pre_exit_trigger_listener_snapshot() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Whenever a creature dies, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId(10_080);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test death listener",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&ABILITIES);
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
    let mut first = creature(10_000, definition_id, PlayerId::One);
    first.damage = 1;
    let mut second = creature(10_001, definition_id, PlayerId::One);
    second.damage = 1;
    game.battlefield = vec![first, second];

    game.check_state_based_actions();

    assert!(game.battlefield.is_empty());
    assert_eq!(game.pending_triggers.len(), 4);
}

#[test]
fn simultaneous_exits_keep_pre_exit_characteristics_for_trigger_matching() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Whenever a Mountain leaves the battlefield, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Subtype("Mountain"),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId(10_081);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Mountain exit listener",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&ABILITIES);
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
    let moon = creature(10_000, cards::BLOOD_MOON, PlayerId::One);
    let moon_id = moon.card.id;
    let taiga = creature(10_001, cards::TAIGA, PlayerId::One);
    let taiga_id = taiga.card.id;
    game.battlefield = vec![creature(10_002, definition_id, PlayerId::One), moon, taiga];

    game.move_permanents_to_graveyard(&[moon_id, taiga_id]);

    assert_eq!(game.pending_triggers.len(), 1);
    assert_eq!(game.pending_triggers[0].context.object, Some(taiga_id));
}

#[test]
fn urgent_exorcism_takes_spirits_and_enchantments_but_nothing_else() {
    // The predicate is a subtype or a card type, so a plain creature is out
    // of reach while a Spirit creature is not.
    for (target_definition, legal) in [
        (cards::STRANGLEROOT_GEIST, true),
        (cards::ENERGY_FLUX, true),
        (cards::SAVANNAH_LIONS, false),
        (cards::BLACK_VISE, false),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, target_definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::URGENT_EXORCISM, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.white = 1;
        game.players[0].mana_pool.colorless = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{target_definition:?} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
    }
}

#[test]
fn ray_of_revelation_destroys_an_enchantment() {
    let mut game = ready_game();
    let target = creature(10_000, cards::ENERGY_FLUX, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let spell = card(10_001, cards::RAY_OF_REVELATION, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
}

#[test]
fn mizzium_mortars_cannot_be_aimed_at_your_own_creature() {
    for (controller, legal) in [(PlayerId::Two, true), (PlayerId::One, false)] {
        let mut game = ready_game();
        let target = creature(10_000, cards::SERRA_ANGEL, controller);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::MIZZIUM_MORTARS, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.red = 1;
        game.players[0].mana_pool.colorless = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "a creature controlled by {controller} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        // Serra Angel is 4/4, so four damage is exactly lethal.
        assert!(game.battlefield.is_empty());
    }
}

#[test]
fn thragtusk_gains_five_life_when_it_enters() {
    let mut game = ready_game();
    let tusk = card(10_001, cards::THRAGTUSK, PlayerId::One);
    game.players[0].hand.push(tusk.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 4;

    game.apply(
        PlayerId::One,
        cast_action(tusk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    // The trigger is a stack object now, so it needs its own resolution.
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 25);
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn think_twice_draws_a_card() {
    let mut game = ready_game();
    let spell = card(10_001, cards::THINK_TWICE, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn blasphemous_act_burns_down_both_sides() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::JUZAM_DJINN, PlayerId::Two));
    // A land is not a creature and must survive.
    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::Two));
    let spell = card(10_003, cards::BLASPHEMOUS_ACT, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 8;

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1, "only the land is left");
    assert_eq!(game.battlefield[0].card.definition, cards::MOUNTAIN);
}

#[test]
fn obzedat_drains_the_opponent_when_it_enters() {
    let mut game = ready_game();
    let obzedat = card(10_001, cards::OBZEDAT_GHOST_COUNCIL, PlayerId::One);
    game.players[0].hand.push(obzedat.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 1;

    // The creature spell itself takes no targets; the entry trigger picks its
    // own when it goes on the stack.
    game.apply(
        PlayerId::One,
        cast_action(obzedat.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    if let Some(decision) = game.observe(PlayerId::One).decision {
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
    }
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert_eq!(game.players[0].life, 22);
}

#[test]
fn tragic_slip_shrinks_a_creature_and_kills_a_small_one() {
    // Savannah Lions is 2/1, so -1/-1 is lethal; Serra Angel is 4/4 and lives.
    for (definition, survives) in [(cards::SAVANNAH_LIONS, false), (cards::SERRA_ANGEL, true)] {
        let mut game = ready_game();
        let target = creature(10_000, definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::TRAGIC_SLIP, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 1;

        game.apply(
            PlayerId::One,
            cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        if survives {
            let permanent = game.battlefield.first().expect("the angel survives");
            assert_eq!(game.power(permanent), Some(3));
            assert_eq!(game.toughness(permanent), Some(3));
        } else {
            assert!(game.battlefield.is_empty(), "{definition:?} should die");
        }
    }
}

#[test]
fn quicken_draws_alongside_its_flash_grant() {
    let mut game = ready_game();
    let spell = card(10_001, cards::QUICKEN, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.blue = 1;
    let before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn mutilate_scales_with_the_swamps_you_control() {
    for (swamps, angel_survives) in [(1, true), (4, false)] {
        let mut game = ready_game();
        for index in 0..swamps {
            game.battlefield
                .push(creature(11_000 + index, cards::SWAMP, PlayerId::One));
        }
        // Serra Angel is 4/4, so it dies only once four Swamps are out.
        game.battlefield
            .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
        let spell = card(10_001, cards::MUTILATE, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 2;
        game.players[0].mana_pool.colorless = 2;

        game.apply(
            PlayerId::One,
            cast_action(spell.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let angel = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL);
        if angel_survives {
            let angel = angel.expect("one Swamp is only -1/-1");
            assert_eq!(game.power(angel), Some(3));
            assert_eq!(game.toughness(angel), Some(3));
        } else {
            assert!(angel.is_none(), "four Swamps is -4/-4 and lethal");
        }
    }
}

#[test]
fn abrupt_decay_only_reaches_cheap_nonland_permanents() {
    // Savannah Lions is {W}, Serra Angel is {3}{W}{W}, and a land is a land.
    for (definition, legal) in [
        (cards::SAVANNAH_LIONS, true),
        (cards::BLACK_VISE, true),
        (cards::SERRA_ANGEL, false),
        (cards::MOUNTAIN, false),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::ABRUPT_DECAY, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 1;
        game.players[0].mana_pool.green = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{definition:?} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(game.battlefield.is_empty(), "{definition:?} is destroyed");
    }
}

#[test]
fn abrupt_decay_says_on_the_card_that_it_cannot_be_countered() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two));
    let decay = card(10_001, cards::ABRUPT_DECAY, PlayerId::One);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(decay.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.green = 1;
    game.players[1].hand.push(counterspell.clone());
    game.players[1].mana_pool.blue = 2;

    game.apply(
        PlayerId::One,
        cast_action(
            decay.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let decay_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(decay_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the Decay resolved despite the Counterspell"
    );
}

#[test]
fn unburial_rites_reanimates_from_your_own_graveyard() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::SERRA_ANGEL, PlayerId::One));
    // The opponent's graveyard is out of reach.
    game.players[1]
        .graveyard
        .push(card(10_002, cards::JUZAM_DJINN, PlayerId::Two));
    let rites = card(10_001, cards::UNBURIAL_RITES, PlayerId::One);
    game.players[0].hand.push(rites.clone());
    // {4}{B} to cast; the flashback cost is the white half.
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 4;

    let theirs = cast_action(
        rites.id,
        vec![Target::Card(CardInstanceId(10_002))],
        Vec::new(),
        0,
    );
    assert!(
        !game.legal_actions(PlayerId::One).contains(&theirs),
        "their graveyard is not yours"
    );

    game.apply(
        PlayerId::One,
        cast_action(
            rites.id,
            vec![Target::Card(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SERRA_ANGEL);
    assert_eq!(game.battlefield[0].controller, PlayerId::One);
    assert!(
        !game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the angel left the graveyard, though the Rites itself arrives there"
    );
}

#[test]
fn oblivion_ring_exiles_another_nonland_permanent() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    // A land is not a legal target, and neither is the Ring itself.
    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::Two));
    let ring = card(10_001, cards::OBLIVION_RING, PlayerId::One);
    game.players[0].hand.push(ring.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(ring.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry trigger asks for its target");
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(_, definition)| definition))
        .collect();
    assert_eq!(
        offered,
        vec![cards::SERRA_ANGEL],
        "neither the land nor the Ring itself is offered"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::SERRA_ANGEL)
    );
}

#[test]
fn war_priest_of_thune_may_decline_to_destroy() {
    for destroy in [true, false] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::ENERGY_FLUX, PlayerId::Two));
        let priest = card(10_001, cards::WAR_PRIEST_OF_THUNE, PlayerId::One);
        game.players[0].hand.push(priest.clone());
        game.players[0].mana_pool.white = 1;
        game.players[0].mana_pool.colorless = 1;

        game.apply(
            PlayerId::One,
            cast_action(priest.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the trigger asks about its optional target");
        assert_eq!(decision.minimum, 0, "you may, so declining is an answer");
        let options = if destroy {
            vec![decision.options[0].id]
        } else {
            Vec::new()
        };
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let flux_alive = game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ENERGY_FLUX);
        assert_eq!(flux_alive, !destroy);
    }
}

#[test]
fn war_priest_of_thune_arrives_even_with_no_enchantment_to_destroy() {
    let mut game = ready_game();
    let priest = card(10_001, cards::WAR_PRIEST_OF_THUNE, PlayerId::One);
    game.players[0].hand.push(priest.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(priest.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "nothing to destroy, so nothing to ask"
    );
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn rest_in_peace_exiles_both_graveyards_as_it_enters() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(10_002, cards::JUZAM_DJINN, PlayerId::Two));
    let rip = card(10_001, cards::REST_IN_PEACE, PlayerId::One);
    game.players[0].hand.push(rip.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(rip.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert!(game.players[0].graveyard.is_empty());
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[0].exile[0].definition, cards::SAVANNAH_LIONS);
    assert_eq!(game.players[1].exile[0].definition, cards::JUZAM_DJINN);
}

#[test]
fn counterflux_counters_theirs_and_survives_theirs() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let flux = card(10_001, cards::COUNTERFLUX, PlayerId::One);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[1].mana_pool.blue = 2;
    game.players[1].hand.push(counterspell.clone());
    game.players[0].hand.push(flux.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.red = 1;
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
        cast_action(flux.id, vec![Target::Spell(bolt_on_stack)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let flux_on_stack = game.stack[1].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(flux_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    for _ in 0..3 {
        pass_priority_pair(&mut game);
    }

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20, "the Bolt never resolved");
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::COUNTERSPELL,
        "their Counterspell resolved and did nothing"
    );
}

#[test]
fn flinthoof_boar_can_buy_haste_the_turn_it_arrives() {
    let mut game = ready_game();
    let mut boar = creature(10_000, cards::FLINTHOOF_BOAR, PlayerId::One);
    // Summoning sick: it entered on the turn now in progress.
    boar.entered_controller_turn = game.turns_started[0];
    game.battlefield.push(boar);
    game.players[0].mana_pool.red = 1;
    let boar_id = CardInstanceId(10_000);

    assert!(
        !game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == boar_id)
                .unwrap()
        ),
        "summoning sick before the ability"
    );

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == boar_id),
        )
        .expect("the haste ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let boar = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == boar_id)
        .expect("the boar is still there");
    assert!(game.can_attack(boar), "it bought haste until end of turn");
}

#[test]
fn arbor_elf_untaps_a_forest_but_not_a_mountain() {
    let mut game = ready_game();
    let mut elf = creature(10_000, cards::ARBOR_ELF, PlayerId::One);
    elf.entered_controller_turn = game.turns_started[0] - 1;
    game.battlefield.push(elf);
    let mut forest = creature(10_001, cards::FOREST, PlayerId::One);
    forest.tapped = true;
    game.battlefield.push(forest);
    let mut mountain = creature(10_002, cards::MOUNTAIN, PlayerId::One);
    mountain.tapped = true;
    game.battlefield.push(mountain);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == CardInstanceId(10_000))
        })
        .expect("the elf can untap something");
    let Action::ActivateAbility { targets, .. } = &activate else {
        unreachable!("the action just matched")
    };
    assert_eq!(
        targets
            .iter()
            .flat_map(TargetSelection::targets)
            .copied()
            .collect::<Vec<_>>(),
        vec![Target::Permanent(CardInstanceId(10_001))],
        "only the Forest is a legal target"
    );

    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let tapped = |game: &Game, id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .expect("still on the battlefield")
            .tapped
    };
    assert!(!tapped(&game, 10_001), "the Forest untapped");
    assert!(tapped(&game, 10_002), "the Mountain did not");
}

#[test]
fn unflinching_courage_pumps_what_it_enchants() {
    let mut game = ready_game();
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    // A second creature must not be affected.
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::One));
    let aura = card(10_001, cards::UNFLINCHING_COURAGE, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel_id)
        .expect("the angel is enchanted, not gone");
    assert_eq!(game.power(angel), Some(6), "4/4 plus 2/2");
    assert_eq!(game.toughness(angel), Some(6));

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the lions are still there");
    assert_eq!(
        game.power(lions),
        Some(2),
        "the other creature is untouched"
    );
}

#[test]
fn an_aura_falls_off_when_its_host_dies() {
    let mut game = ready_game();
    let lions = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    let aura = card(10_001, cards::UNFLINCHING_COURAGE, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(lions_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield.len(), 2, "creature and aura");

    game.destroy_permanent_without_regeneration(lions_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield.is_empty(),
        "the aura followed its host off the battlefield"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::UNFLINCHING_COURAGE),
        "and went to its owner's graveyard"
    );
}

#[test]
fn underworld_connections_lends_its_land_a_draw_ability() {
    let mut game = ready_game();
    let mut swamp = creature(10_000, cards::SWAMP, PlayerId::One);
    swamp.entered_controller_turn = game.turns_started[0] - 1;
    let swamp_id = swamp.card.id;
    game.battlefield.push(swamp);
    let aura = card(10_001, cards::UNDERWORLD_CONNECTIONS, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(swamp_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let library_before = game.players[0].library.len();
    // The Aura became a new object as it left the stack, so its permanent id
    // is not the card id it was cast from.
    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::UNDERWORLD_CONNECTIONS)
        .expect("the aura is on the battlefield")
        .card
        .id;
    // The Swamp still has its own mana ability, so pick the granted one by
    // its origin rather than by guessing at the order.
    let draw = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, ability, .. }
                    if *source == swamp_id
                        && matches!(ability, AbilityOrigin::Granted { source: granter, .. }
                            if *granter == aura_id)
            )
        })
        .expect("the aura granted the land an activated ability");
    game.apply(PlayerId::One, draw).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), library_before - 1);
    assert_eq!(game.players[0].life, 19, "one life paid");
}

#[test]
fn thragtusk_leaves_a_beast_behind() {
    let mut game = ready_game();
    let tusk = card(10_001, cards::THRAGTUSK, PlayerId::One);
    game.players[0].hand.push(tusk.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 4;

    game.apply(
        PlayerId::One,
        cast_action(tusk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 25, "the entry trigger gained 5");

    let tusk_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::THRAGTUSK)
        .expect("the tusk is on the battlefield")
        .card
        .id;
    game.destroy_permanent_without_regeneration(tusk_id);
    game.check_state_based_actions();
    // Placing a captured trigger on the stack, and resolving it, happen as the
    // game processes actions, so the test has to keep playing rather than only
    // poking the engine.
    for _ in 0..12 {
        if game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::BEAST_TOKEN_3_3_GREEN)
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let beast = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BEAST_TOKEN_3_3_GREEN)
        .expect("a Beast token replaced it");
    assert_eq!(game.power(beast), Some(3));
    assert_eq!(game.toughness(beast), Some(3));
    assert_eq!(beast.controller, PlayerId::One);
}

#[test]
fn a_token_ceases_to_exist_rather_than_reaching_a_graveyard() {
    let mut game = ready_game();
    game.create_token(PlayerId::One, cards::BEAST_TOKEN_3_3_GREEN);
    let token_id = game.battlefield[0].card.id;
    assert!(game.players[0].graveyard.is_empty());

    game.destroy_permanent_without_regeneration(token_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield.is_empty(),
        "the token left the battlefield"
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and ceased to exist rather than landing in a graveyard"
    );
}

#[test]
fn a_token_is_never_deck_legal() {
    let catalog = poc::catalog().expect("catalog builds");
    for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
        assert!(
            !catalog.is_allowed_in(cards::BEAST_TOKEN_3_3_GREEN, format),
            "a token belongs to no format's card pool"
        );
    }
}
#[test]
fn put_onto_battlefield_reaches_a_board_state_directly() {
    let mut game = ready_game();
    let id = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is in the catalog");

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield");
    assert_eq!(angel.controller, PlayerId::Two);
    assert_eq!(game.power(angel), Some(4));
    assert!(!angel.tapped);

    assert_eq!(
        game.put_onto_battlefield(PlayerId::One, CardDefinitionId(60_000)),
        Err(ZoneError::UnknownCard(CardDefinitionId(60_000))),
        "an unknown definition is refused rather than guessed at"
    );
}

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
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object,
        ability: &CARD_COST_FLASHBACK,
    });
}

#[test]
fn snapcaster_grants_an_ordinary_card_cost_flashback_ability() {
    let catalog = poc::catalog().unwrap();
    let snapcaster = catalog.get(cards::SNAPCASTER_MAGE).unwrap();
    let trigger = snapcaster.rules.ability(AbilityId(1)).unwrap();
    let EffectDef::Apply {
        effect: AppliedEffectDef::GrantAbility(granted),
        duration: EffectDurationDef::UntilEndOfTurn,
        ..
    } = trigger.effect
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
fn snapcaster_granted_recall_can_discard_every_card_in_hand_for_x() {
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
    let definition_id = CardDefinitionId(20_100);
    let flashback = AlternativeCostId(1);
    let overload = AlternativeCostId(2);
    let targets = Box::leak(
        vec![AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "opponent",
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )]
        .into_boxed_slice(),
    );
    let abilities = Box::leak(
        vec![
            AbilityDef::spell(
                "Test spell deals 1 damage to target opponent.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::Constant(1),
                },
            )
            .with_targets(targets),
            AbilityDef::alternative_cast(
                ManaCost::default(),
                AlternativeCastKindDef::Flashback,
                None,
                EffectDef::None,
            )
            .with_implementation(AbilityImplementationDef::NotImplemented {
                explanation: "Test-only incomplete flashback.",
            }),
            AbilityDef::alternative_cast(
                ManaCost::default(),
                AlternativeCastKindDef::Overload,
                Some("Test spell deals 1 damage to each opponent."),
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            )
            .with_implementation(AbilityImplementationDef::NotImplemented {
                explanation: "Test-only incomplete overload.",
            }),
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

    assert_eq!(bloodrush.activation_text, None);
    assert_eq!(definition.source_zones, [ZoneKind::Hand]);
    assert_eq!(
        definition.costs.as_slice(),
        [
            AbilityCostDef::Mana(mana_cost!("{R}{G}")),
            AbilityCostDef::DiscardSource,
        ],
    );
    let EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetSlotId(0)),
        effect: AppliedEffectDef::Composite(components),
        duration: EffectDurationDef::UntilEndOfTurn,
    } = bloodrush.effect
    else {
        panic!("Rampager should apply one composite effect until end of turn")
    };
    assert!(matches!(
        components,
        [
            AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(4),
                toughness: ValueDef::Constant(4),
            },
            AppliedEffectDef::GrantAbility(ability),
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
    blocker.blocking = Some(attacker_id);
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
        GameEvent::AbilityFizzled { object, source, definition }
            if *object == ability_object
                && *source == rampager_id
                && *definition == cards::GHOR_CLAN_RAMPAGER
    )));
    assert!(!game.events.iter().any(|event| matches!(
        event,
        GameEvent::AbilityResolved { object, .. } if *object == ability_object
    )));
}
fn acceptance_cast_action_for_card(game: &Game, player: PlayerId, spell: GameObjectId) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the acceptance spell has a legal cast action")
}

fn acceptance_cast_action_targeting(
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

fn acceptance_attempt_counterspell(game: &mut Game, counterspell: GameObjectId) {
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
        .find(|candidate| candidate.definition == cards::CAVERN_OF_SOULS)
        .expect("Cavern entered after its creature type was chosen");
    assert_eq!(
        permanent.chosen_creature_type.as_deref(),
        Some(creature_type)
    );
    permanent.id
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
                .all(|ability| ability.implementation == AbilityImplementationDef::Definition),
            "{} should not use a card-local resolver",
            definition.name,
        );
        let modal = definition
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) => spell.modal(),
                DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
            .expect("an Elemental Blast has declarative modes");
        assert_eq!((modal.minimum, modal.maximum), (1, 1));
        assert!(!modal.may_repeat);
        assert_eq!(modal.modes.len(), 2);
        assert!(modal.modes.iter().all(|mode| {
            mode.implementation == AbilityImplementationDef::Definition
                && matches!(mode.definition, DeclarativeAbilityDef::Spell(spell) if spell.modal().is_none())
        }));
        assert!(
            modal
                .modes
                .iter()
                .any(|mode| matches!(mode.effect, EffectDef::Counter { .. }))
        );
        assert!(
            modal
                .modes
                .iter()
                .any(|mode| matches!(mode.effect, EffectDef::Destroy { .. }))
        );
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
    assert_eq!(game.mana_drain_pending[PlayerId::Two.index()], 2);
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
            from: ZoneKind::Hand,
            to: ZoneKind::Graveyard,
            cause: ZoneMoveCauseDef::EffectControlledBy(PlayerRelation::Opponent),
        }
    );
    assert_eq!(
        replacement.0.effect,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Source,
            zone: ZoneKind::Battlefield,
        }
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
        (ZoneMoveCause::Cost, false),
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
    );

    assert!(game.players[0].graveyard.is_empty());
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::One && permanent.card.definition == cards::LOXODON_SMITER
    }));
}

#[test]
fn opponent_spells_and_abilities_put_a_discarded_smiter_onto_the_battlefield() {
    let mut game = ready_game();
    let hymn = card(19_070, cards::HYMN_TO_TOURACH, PlayerId::One);
    let smiter = card(19_071, cards::LOXODON_SMITER, PlayerId::Two);
    game.players[0].hand.push(hymn.clone());
    game.players[1].hand.push(smiter);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    let action = acceptance_cast_action_for_card(&game, PlayerId::One, hymn.id);
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
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::Two && permanent.card.definition == cards::LOXODON_SMITER
    }));
    assert!(game.players[1].graveyard.is_empty());
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
        .find(|permanent| permanent.definition == cards::CAVERN_OF_SOULS)
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
                effect.effect == AppliedEffectDef::CannotBeCountered
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
        effect.effect == AppliedEffectDef::CannotBeCountered
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
                effect.effect == AppliedEffectDef::CannotBeCountered
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
                effect.effect == AppliedEffectDef::CannotBeCountered
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

/// Casts a modal spell by picking one mode. `slot` is the chosen mode's own
/// target slot, which is not always zero.
fn cast_mode(card: GameObjectId, mode: ModeId, slot: TargetSlotId, targets: Vec<Target>) -> Action {
    let mut choices = CastChoices::default().with_modes(vec![mode]);
    if !targets.is_empty() {
        choices = choices.with_targets(vec![TargetSelection::new(slot, targets)]);
    }
    Action::CastSpell {
        card,
        choices,
        sacrifices: Vec::new(),
    }
}

#[test]
fn azorius_charm_gives_your_creatures_lifelink_but_not_theirs() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(0), TargetSlotId(0), Vec::new()),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let lifelink = |game: &Game, definition| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("still on the battlefield");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink)
    };
    assert!(lifelink(&game, cards::SAVANNAH_LIONS), "yours gains it");
    assert!(!lifelink(&game, cards::SERRA_ANGEL), "theirs does not");
}

#[test]
fn azorius_charm_can_instead_draw_a_card() {
    let mut game = ready_game();
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;
    let before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(1), TargetSlotId(0), Vec::new()),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn izzet_charm_deals_two_damage_in_its_implemented_mode() {
    let mut game = ready_game();
    // Savannah Lions is 2/1, so two damage kills it.
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two));
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(1),
            TargetSlotId(1),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
}

#[test]
fn izzet_charm_loots_by_drawing_two_then_discarding_two_of_choice() {
    let mut game = ready_game();
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    // One card already in hand, so the two drawn cards leave a choice of
    // three rather than a forced discard of everything.
    let keeper = card(10_002, cards::BLACK_LOTUS, PlayerId::One);
    game.players[0].hand.push(keeper.clone());
    let library_before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(2), TargetSlotId(0), Vec::new()),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[0].library.len(),
        library_before - 2,
        "both cards were drawn"
    );
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the discard is the caster's choice");
    assert_eq!((decision.minimum, decision.maximum), (2, 2));
    let discards = decision
        .options
        .iter()
        .filter(|option| option.card != Some((keeper.id, cards::BLACK_LOTUS)))
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(discards.len(), 2, "the drawn cards are discardable");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: discards,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].id, keeper.id, "the Lotus was kept");
    assert_eq!(
        game.players[0].graveyard.len(),
        3,
        "two cards and the charm"
    );
}

#[test]
fn a_discard_with_no_choice_left_needs_no_decision() {
    let mut game = ready_game();
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(2), TargetSlotId(0), Vec::new()),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // Exactly the two drawn cards are in hand, so there is nothing to choose.
    assert!(game.players[0].hand.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn selesnya_charm_pumps_and_grants_trample() {
    let catalog = poc::catalog().unwrap();
    let charm_definition = catalog.get(cards::SELESNYA_CHARM).unwrap();
    let DeclarativeAbilityDef::Spell(spell) =
        charm_definition.rules.ability_clauses()[0].definition
    else {
        panic!("Selesnya Charm should have a spell ability")
    };
    let mode = spell.mode(ModeId(0)).unwrap();
    let EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetSlotId(0)),
        effect: AppliedEffectDef::Composite(components),
        duration: EffectDurationDef::UntilEndOfTurn,
    } = mode.effect
    else {
        panic!("Selesnya Charm should apply one composite effect until end of turn")
    };
    assert!(matches!(
        components,
        [
            AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(2),
                toughness: ValueDef::Constant(2),
            },
            AppliedEffectDef::GrantAbility(ability),
        ] if ability.definition == DeclarativeAbilityDef::Keyword(KeywordAbility::Trample)
    ));

    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(0),
            TargetSlotId(0),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("still there");
    assert_eq!(game.power(lions), Some(4), "2/1 plus 2/2");
    assert_eq!(game.toughness(lions), Some(3));
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));
}

#[test]
fn selesnya_charm_can_instead_make_a_knight() {
    let mut game = ready_game();
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(2), TargetSlotId(0), Vec::new()),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let knight = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::KNIGHT_TOKEN_2_2_WHITE)
        .expect("a Knight token arrived");
    assert_eq!(game.power(knight), Some(2));
    assert_eq!(game.toughness(knight), Some(2));
    assert_eq!(knight.controller, PlayerId::One);
    assert!(game.permanent_has_executable_keyword(knight, KeywordAbility::Vigilance));
}

#[test]
fn selesnya_charm_exiles_only_a_big_creature() {
    // Juzam Djinn is 5/5 and qualifies; Serra Angel is 4/4 and does not.
    for (definition, legal) in [(cards::JUZAM_DJINN, true), (cards::SERRA_ANGEL, false)] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, definition, PlayerId::Two));
        let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
        game.players[0].hand.push(charm.clone());
        game.players[0].mana_pool.green = 1;
        game.players[0].mana_pool.white = 1;

        let action = cast_mode(
            charm.id,
            ModeId(1),
            TargetSlotId(1),
            vec![Target::Permanent(CardInstanceId(10_000))],
        );
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{definition:?} should be {}",
            if legal { "exilable" } else { "too small" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[1].exile[0].definition, definition);
    }
}

#[test]
fn selesnya_charm_reads_current_power_not_printed_power() {
    // A 4/4 pumped to 6/6 by the charm's own first mode qualifies for the
    // second, which is why the predicate reads live power.
    let mut game = ready_game();
    let mut angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    angel.power_bonus = 2;
    game.battlefield.push(angel);
    let charm = card(10_001, cards::SELESNYA_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;

    let action = cast_mode(
        charm.id,
        ModeId(1),
        TargetSlotId(1),
        vec![Target::Permanent(CardInstanceId(10_000))],
    );
    assert!(
        game.legal_actions(PlayerId::One).contains(&action),
        "a 4/4 pumped to 6/6 is now big enough"
    );
}

#[test]
fn boros_charm_burns_a_player_for_four() {
    let mut game = ready_game();
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(0),
            TargetSlotId(0),
            vec![Target::Player(PlayerId::Two)],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 16);
    assert_eq!(
        game.players[0].life, 20,
        "it is a targeted burn, not a sweep"
    );
}

#[test]
fn boros_charm_grants_double_strike_until_cleanup() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(2),
            TargetSlotId(1),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(10_000))
        .unwrap();
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::DoubleStrike));

    game.finish_cleanup();
    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == CardInstanceId(10_000))
        .unwrap();
    assert!(!game.permanent_has_executable_keyword(lions, KeywordAbility::DoubleStrike));
}

#[test]
fn primeval_bounty_makes_a_beast_only_for_its_controller() {
    for (caster, expect_token) in [(PlayerId::One, true), (PlayerId::Two, false)] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::PRIMEVAL_BOUNTY)
            .expect("cataloged");
        let lions = card(10_001, cards::SAVANNAH_LIONS, caster);
        game.players[caster.index()].hand.push(lions.clone());
        game.players[caster.index()].mana_pool.white = 1;
        // A creature spell is sorcery-speed, so the caster needs the turn.
        game.active_player = caster;
        game.priority = caster;

        game.apply(caster, cast_action(lions.id, Vec::new(), Vec::new(), 0))
            .unwrap();
        for _ in 0..8 {
            if game
                .battlefield
                .iter()
                .any(|p| p.card.definition == cards::BEAST_TOKEN_3_3_GREEN)
            {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let made_token = game
            .battlefield
            .iter()
            .any(|p| p.card.definition == cards::BEAST_TOKEN_3_3_GREEN);
        assert_eq!(
            made_token,
            expect_token,
            "a creature cast by {caster} should {} a Beast",
            if expect_token { "make" } else { "not make" }
        );
    }
}

#[test]
fn primeval_bounty_gains_life_only_for_its_own_lands() {
    for (lander, expect_life) in [(PlayerId::One, 23), (PlayerId::Two, 20)] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::PRIMEVAL_BOUNTY)
            .expect("cataloged");
        game.players[lander.index()]
            .hand
            .push(card(10_002, cards::FOREST, lander));
        game.players[lander.index()].land_played_this_turn = false;
        game.play_land(lander, CardInstanceId(10_002), PlayOptionId::DEFAULT);
        for _ in 0..8 {
            if game.players[0].life != 20 {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
        assert_eq!(
            game.players[0].life,
            expect_life,
            "a land played by {lander} should {} life",
            if expect_life > 20 { "gain" } else { "not gain" }
        );
    }
}

#[test]
fn vault_of_the_archangel_arms_only_your_creatures() {
    let mut game = ready_game();
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::VAULT_OF_THE_ARCHANGEL)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == vault)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 2;

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == vault))
        .expect("the vault ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let armed = |game: &Game, definition| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("still there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::Deathtouch)
            && game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink)
    };
    assert!(armed(&game, cards::SAVANNAH_LIONS), "yours gets both");
    assert!(!armed(&game, cards::SERRA_ANGEL), "theirs gets neither");
}

#[test]
fn gavony_township_grows_only_your_creatures() {
    let mut game = ready_game();
    let township = game
        .put_onto_battlefield(PlayerId::One, cards::GAVONY_TOWNSHIP)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == township)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == township)
        })
        .expect("the township ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let power_of = |game: &Game, controller| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| {
                permanent.card.definition == cards::SAVANNAH_LIONS
                    && permanent.controller == controller
            })
            .expect("still there");
        game.power(permanent)
    };
    assert_eq!(
        power_of(&game, PlayerId::One),
        Some(3),
        "2/1 plus a counter"
    );
    assert_eq!(
        power_of(&game, PlayerId::Two),
        Some(2),
        "theirs is untouched"
    );
}

#[test]
fn shadowborn_demon_cannot_point_its_trigger_at_a_demon() {
    // Juzam Djinn is a Djinn and a legal target; Desecration Demon is not.
    // The other Demon deliberately has no entry trigger of its own, so this
    // test has exactly one trigger to answer.
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::JUZAM_DJINN)
        .expect("cataloged");
    let other_demon = game
        .put_onto_battlefield(PlayerId::Two, cards::DESECRATION_DEMON)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SHADOWBORN_DEMON)
        .expect("cataloged");

    // A captured trigger reaches the stack as the game processes actions.
    for _ in 0..6 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry trigger asks for a target");
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(id, _)| id))
        .collect();
    assert!(
        !offered.contains(&CardInstanceId(other_demon.0)),
        "another Demon is not a legal target"
    );
    assert_eq!(offered.len(), 1, "only the Djinn qualifies");

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();
    for _ in 0..6 {
        if !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::JUZAM_DJINN)
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::JUZAM_DJINN),
        "the Djinn was destroyed"
    );
}

#[test]
fn order_of_leitbur_can_pump_itself() {
    let mut game = ready_game();
    let order = game
        .put_onto_battlefield(PlayerId::One, cards::ORDER_OF_LEITBUR)
        .expect("cataloged");
    game.players[0].mana_pool.white = 2;
    let pump = activated_ability_for(&game, order, 1);

    // The Order also grants itself first strike for {W}; the pump is the
    // later printed clause.
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability,
                    ..
                } if *source == order && *ability == pump
            )
        })
        .expect("the pump is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let order = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == order)
        .expect("still there");
    assert_eq!(game.power(order), Some(3), "2/1 plus one power");
    assert_eq!(game.toughness(order), Some(1), "toughness is unchanged");
}

#[test]
fn encroaching_wastes_spares_basic_lands() {
    // A Mountain is Basic and safe; a dual land is not.
    for (definition, legal) in [(cards::MOUNTAIN, false), (cards::TUNDRA, true)] {
        let mut game = ready_game();
        let wastes = game
            .put_onto_battlefield(PlayerId::One, cards::ENCROACHING_WASTES)
            .expect("cataloged");
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == wastes)
            .unwrap()
            .entered_controller_turn = game.turns_started[0] - 1;
        let target = game
            .put_onto_battlefield(PlayerId::Two, definition)
            .expect("cataloged");
        game.players[0].mana_pool.colorless = 4;

        let offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } if source == wastes => Some(
                    targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .copied()
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(
            offered.contains(&Target::Permanent(target)),
            legal,
            "{definition:?} should be {}",
            if legal {
                "destroyable"
            } else {
                "protected by Basic"
            }
        );
    }
}

#[test]
fn goblin_digging_team_only_hits_walls() {
    let mut game = ready_game();
    let team = game
        .put_onto_battlefield(PlayerId::One, cards::GOBLIN_DIGGING_TEAM)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == team)
        .unwrap()
        .entered_controller_turn = game.turns_started[0] - 1;
    // Savannah Lions is a Cat, not a Wall, so there is nothing to point at.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == team)
        }),
        "with no Wall in play the ability has no legal target"
    );

    let wall = game
        .put_onto_battlefield(PlayerId::Two, cards::WALL_OF_STONE)
        .expect("cataloged");
    let targets = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == team => Some(
                targets
                    .iter()
                    .flat_map(TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(targets, vec![vec![Target::Permanent(wall)]]);
}

#[test]
fn ruric_thar_burns_whoever_cast_the_noncreature_spell() {
    // It hits its own controller too, which is the point of the card.
    for caster in [PlayerId::One, PlayerId::Two] {
        let mut game = ready_game();
        game.put_onto_battlefield(PlayerId::One, cards::RURIC_THAR_THE_UNBOWED)
            .expect("cataloged");
        let bolt = card(10_001, cards::LIGHTNING_BOLT, caster);
        game.players[caster.index()].hand.push(bolt.clone());
        game.players[caster.index()].mana_pool.red = 1;
        game.active_player = caster;
        game.priority = caster;
        let before = [game.players[0].life, game.players[1].life];

        game.apply(
            caster,
            cast_action(
                bolt.id,
                vec![Target::Player(caster.opponent())],
                Vec::new(),
                0,
            ),
        )
        .unwrap();
        for _ in 0..8 {
            if game.players[caster.index()].life < before[caster.index()] {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        assert_eq!(
            game.players[caster.index()].life,
            before[caster.index()] - 6,
            "the caster {caster} takes six"
        );
    }
}

#[test]
fn assemble_the_legion_musters_one_more_soldier_every_upkeep() {
    let mut game = ready_game();
    game.turn = 2;
    game.put_onto_battlefield(PlayerId::One, cards::ASSEMBLE_THE_LEGION)
        .expect("cataloged");

    let mut mustered = Vec::new();
    for _ in 0..3 {
        loop {
            game.start_next_turn();
            if game.active_player == PlayerId::One {
                break;
            }
        }
        for _ in 0..8 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .take(decision.minimum.max(1))
                    .map(|option| option.id)
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
        mustered.push(
            game.battlefield
                .iter()
                .filter(|permanent| permanent.card.definition == cards::SOLDIER_TOKEN_1_1_RED_WHITE)
                .count(),
        );
    }

    // One counter is added before the tokens are made, so the first upkeep
    // already musters a Soldier and each later one musters one more.
    assert_eq!(mustered, vec![1, 3, 6]);
}

#[test]
fn a_first_striker_kills_a_smaller_blocker_before_it_can_answer() {
    let mut game = ready_game();
    // Black Knight is a 2/2 first striker; Savannah Lions is a 2/1, so
    // without an earlier damage step both would die together.
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];

    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert!(game.regular_combat_damage_pending());
    pass_priority_pair(&mut game);

    let survivor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("the first striker survives");
    assert_eq!(survivor.damage, 0, "the blocker never got to swing back");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker_id),
        "the blocker died in the first-strike step",
    );
}

#[test]
fn boros_charm_double_strike_hits_an_unblocked_player_twice() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield = vec![attacker];
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;
    let life_before = game.players[1].life;

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(2),
            TargetSlotId(1),
            vec![Target::Permanent(attacker_id)],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "double strike deals once before the inter-wave priority window",
    );
    assert!(game.regular_combat_damage_pending());

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "a 2/1 double striker deals two damage twice"
    );
}

#[test]
fn archangel_of_thune_grows_the_team_on_its_own_lifelink_damage() {
    let mut game = ready_game();
    let mut angel = creature(10_000, cards::ARCHANGEL_OF_THUNE, PlayerId::One);
    angel.attacking = true;
    game.battlefield = vec![angel];
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));

    game.deal_combat_damage();
    for _ in 0..8 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let counters = |id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| permanent.counters[CounterKind::PlusOnePlusOne.index()])
    };
    assert_eq!(counters(10_000), Some(1), "the Angel counts itself");
    assert_eq!(counters(10_001), Some(1));
    assert_eq!(counters(10_002), Some(0), "not the opponent's creature");
    // Lifelink gained 3, and the trigger is one counter per gain rather than
    // one per point of life.
    assert_eq!(game.players[0].life, 23);
}

#[test]
fn rhox_faithmender_doubles_your_life_gain_but_not_your_opponent_s() {
    for (gainer, expected) in [(PlayerId::One, 8), (PlayerId::Two, 4)] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::RHOX_FAITHMENDER, PlayerId::One));
        let before = game.players[gainer.index()].life;

        game.gain_life(gainer, 4);

        assert_eq!(
            game.players[gainer.index()].life - before,
            expected,
            "life gained by {gainer}",
        );
    }
}

#[test]
fn two_faithmenders_multiply_together_rather_than_adding() {
    let mut game = ready_game();
    for id in [10_000, 10_001] {
        game.battlefield
            .push(creature(id, cards::RHOX_FAITHMENDER, PlayerId::One));
    }
    let before = game.players[0].life;

    game.gain_life(PlayerId::One, 3);

    assert_eq!(game.players[0].life - before, 12);
}

#[test]
fn think_twice_can_be_flashed_back_once_and_then_is_gone() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::THINK_TWICE, PlayerId::One));
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    let hand_before = game.players[0].hand.len();

    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(10_000)))
        .expect("a card in the graveyard offers its flashback option");
    game.apply(PlayerId::One, flashback).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), hand_before + 1, "it drew");
    assert!(
        game.players[0].graveyard.is_empty(),
        "a flashback spell does not return to the graveyard"
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { .. })),
        "an exiled card cannot be flashed back again",
    );
}

#[test]
fn a_card_in_hand_is_not_offered_its_flashback_cost() {
    let mut game = ready_game();
    let think_twice = card(10_000, cards::THINK_TWICE, PlayerId::One);
    game.players[0].hand.push(think_twice);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;

    let options = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { choices, .. } => Some(choices.play_option()),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(options, vec![PlayOptionId::DEFAULT]);
}

#[test]
fn flinthoof_boar_grows_for_a_mountain_you_control_and_only_once() {
    let mut game = ready_game();
    let boar = creature(10_000, cards::FLINTHOOF_BOAR, PlayerId::One);
    let boar_id = boar.card.id;
    game.battlefield.push(boar);
    // The opponent's Mountain is not one you control.
    game.battlefield
        .push(creature(10_001, cards::MOUNTAIN, PlayerId::Two));

    let stats = |game: &Game| {
        let boar = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == boar_id)
            .expect("still there");
        (game.power(boar), game.toughness(boar))
    };
    assert_eq!(stats(&game), (Some(2), Some(2)), "printed 2/2");

    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));

    game.battlefield[2].text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Mountain,
        to: BasicLandType::Island,
    });
    assert_eq!(
        stats(&game),
        (Some(2), Some(2)),
        "the condition reads the land's effective subtype",
    );

    game.battlefield
        .push(creature(10_003, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));

    // "As long as you control a Mountain" is a condition, so a second one
    // adds nothing.
    game.battlefield
        .push(creature(10_004, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));
}

#[test]
fn a_wall_may_block_but_never_attacks_and_never_stops_a_juggernaut() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let juggernaut = creature(10_000, cards::JUGGERNAUT, PlayerId::One);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);
    let wall = creature(10_001, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    let lions = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);

    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker } if *attacker == wall_id)
        ),
        "defender keeps the Wall home",
    );

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: juggernaut_id,
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    let blocks = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        blocks,
        vec![lions_id],
        "the Wall cannot block a Juggernaut, but the Lions can"
    );
}

#[test]
fn boros_reckoner_returns_the_damage_it_took_to_a_target_of_its_choice() {
    let mut game = ready_game();
    let reckoner = creature(10_000, cards::BOROS_RECKONER, PlayerId::One);
    game.battlefield.push(reckoner);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;
    let life_before = game.players[1].life;

    game.apply(
        PlayerId::Two,
        cast_action(
            bolt.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    for _ in 0..12 {
        if game.players[1].life <= life_before - 3 {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Aim the trigger at the player who threw the Bolt rather than
            // taking whichever option happens to come first.
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .map_or_else(
                    || {
                        decision
                            .options
                            .iter()
                            .take(decision.minimum.max(1))
                            .map(|option| option.id)
                            .collect::<Vec<_>>()
                    },
                    |option| vec![option.id],
                );
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    // Three damage in, three damage back out at the player who threw it.
    assert_eq!(game.players[1].life, life_before - 3);
}

#[test]
fn burning_earth_burns_only_the_nonbasic_taps() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::BURNING_EARTH)
        .expect("cataloged");
    let foundry = game
        .put_onto_battlefield(PlayerId::Two, cards::SACRED_FOUNDRY)
        .expect("cataloged");
    let mountain = game
        .put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");
    let life_before = game.players[1].life;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    let tap_for_red = |game: &Game, source: GameObjectId| {
        game.legal_actions(PlayerId::Two)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateManaAbility { source: id, color, .. }
                    if *id == source && *color == ManaColor::Red)
            })
            .expect("the land taps for red")
    };

    let action = tap_for_red(&game, mountain);
    game.apply(PlayerId::Two, action).unwrap();
    assert_eq!(
        game.players[1].life, life_before,
        "a basic Mountain is not a nonbasic land"
    );

    let action = tap_for_red(&game, foundry);
    game.apply(PlayerId::Two, action).unwrap();
    for _ in 0..8 {
        if game.players[1].life < life_before {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert_eq!(game.players[1].life, life_before - 1);
}

#[test]
fn celestial_flare_only_takes_a_creature_that_is_in_combat() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    // Sitting at home, so the Flare cannot reach it.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
    game.players[0].hand.push(flare.clone());
    game.players[0].mana_pool.white = 2;

    game.apply(
        PlayerId::One,
        cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // One candidate means no decision: the Angel simply goes.
    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![CardInstanceId(10_001)],
        "the attacker was sacrificed and the untapped Lions stayed"
    );
}

#[test]
fn celestial_flare_lets_the_targeted_player_pick_which_attacker_dies() {
    let mut game = ready_game();
    for id in [10_000, 10_001] {
        let mut attacker = creature(id, cards::SAVANNAH_LIONS, PlayerId::Two);
        attacker.attacking = true;
        game.battlefield.push(attacker);
    }
    let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
    game.players[0].hand.push(flare.clone());
    game.players[0].mana_pool.white = 2;

    game.apply(
        PlayerId::One,
        cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the sacrifice is the targeted player's choice");
    assert_eq!(decision.player, PlayerId::Two);
    let keep = decision
        .options
        .iter()
        .find(|option| option.card == Some((CardInstanceId(10_001), cards::SAVANNAH_LIONS)))
        .expect("both attackers are offered");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![keep.id],
        },
    )
    .unwrap();

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![CardInstanceId(10_000)],
        "the one they chose is the one that died"
    );
}

#[test]
fn thundermaw_hellkite_only_shocks_the_fliers_across_the_table() {
    let mut game = ready_game();
    // A flier they control, a ground creature they control, and a flier of
    // your own: only the first is named.
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::THUNDERMAW_HELLKITE)
        .expect("cataloged");
    for _ in 0..8 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let state = |id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| (permanent.damage, permanent.tapped))
    };
    assert_eq!(state(10_000), Some((1, true)), "their flier");
    assert_eq!(state(10_001), Some((0, false)), "their ground creature");
    assert_eq!(state(10_002), Some((0, false)), "your own flier");
}

#[test]
fn azorius_charm_puts_an_attacker_back_on_top_of_its_library() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;
    let library_before = game.players[1].library.len();

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(2),
            TargetSlotId(2),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty(), "the attacker left combat");
    assert_eq!(game.players[1].library.len(), library_before + 1);
    assert_eq!(
        game.players[1].library.last().map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "on top, not shuffled in",
    );
}

#[test]
fn an_order_can_buy_first_strike_and_win_a_trade_it_would_have_lost() {
    let mut game = ready_game();
    let mut order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::One);
    order.attacking = true;
    let order_id = order.card.id;
    game.battlefield.push(order);
    // Another 2/1: without first strike the two would kill each other.
    let mut blocker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = Some(order_id);
    game.battlefield.push(blocker);
    game.players[0].mana_pool.black = 1;
    let first_strike = activated_ability_for(&game, order_id, 0);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability,
                    ..
                } if *source == order_id && *ability == first_strike
            )
        })
        .expect("the first-strike ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert_eq!(game.step, Step::CombatDamage);
    assert!(
        game.regular_combat_damage_pending(),
        "the bought first strike creates an inter-wave priority window",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![order_id],
        "the Order struck first and took nothing back"
    );
}

#[test]
fn syncopate_exiles_the_spell_when_its_controller_will_not_pay() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    let syncopate = card(10_001, cards::SYNCOPATE, PlayerId::One);
    game.players[0].hand.push(syncopate.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    let spell = game.stack.last().expect("the Bolt is on the stack").id;
    // Enough to pay, so the choice is real rather than a formality.
    game.players[1].mana_pool.colorless = 2;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_action(syncopate.id, vec![Target::Spell(spell)], Vec::new(), 2),
    )
    .unwrap();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the Bolt's controller is asked, not Syncopate's");
    assert_eq!(decision.player, PlayerId::Two);
    let decline = decision
        .options
        .iter()
        .find(|option| option.label == "Let it be countered")
        .expect("declining is always available");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline.id],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 20, "the Bolt never resolved");
    assert!(game.players[1].graveyard.is_empty(), "exiled, not buried");
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT]
    );
}

#[test]
fn izzet_charm_lets_a_paying_controller_keep_the_spell() {
    let mut game = ready_game();
    let ritual = card(10_000, cards::DARK_RITUAL, PlayerId::Two);
    game.players[1].hand.push(ritual.clone());
    game.players[1].mana_pool.black = 1;
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    game.apply(
        PlayerId::Two,
        cast_action(ritual.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let spell = game.stack.last().expect("the Ritual is on the stack").id;
    game.players[1].mana_pool.colorless = 2;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(0),
            TargetSlotId(0),
            vec![Target::Spell(spell)],
        ),
    )
    .unwrap();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game.observe(PlayerId::Two).decision.expect("a real choice");
    let pay = decision
        .options
        .iter()
        .find(|option| option.label == "Pay the cost")
        .expect("they can afford it");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![pay.id],
        },
    )
    .unwrap();

    assert!(
        game.stack.iter().any(|object| object.id == spell),
        "paying keeps the spell on the stack"
    );
    assert_eq!(game.players[1].mana_pool.colorless, 0, "the two was spent");
}

#[test]
fn tragic_slip_is_a_minus_one_until_something_dies() {
    for morbid in [false, true] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
        if morbid {
            game.battlefield
                .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
            game.destroy_permanent(CardInstanceId(10_001));
        }
        let slip = card(10_002, cards::TRAGIC_SLIP, PlayerId::One);
        game.players[0].hand.push(slip.clone());
        game.players[0].mana_pool.black = 1;

        game.apply(
            PlayerId::One,
            cast_action(
                slip.id,
                vec![Target::Permanent(CardInstanceId(10_000))],
                Vec::new(),
                0,
            ),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let angel = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(10_000));
        if morbid {
            assert!(angel.is_none(), "a 4/4 does not survive -13/-13");
        } else {
            let angel = angel.expect("a 4/4 shrugs off -1/-1");
            assert_eq!(
                (game.power(angel), game.toughness(angel)),
                (Some(3), Some(3))
            );
        }
    }
}

#[test]
fn ratchet_bomb_sweeps_the_mana_value_it_ticked_up_to() {
    let mut game = ready_game();
    let bomb = game
        .put_onto_battlefield(PlayerId::One, cards::RATCHET_BOMB)
        .expect("cataloged");
    // Savannah Lions costs one, Serra Angel five, and a land is spared
    // whatever the count.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");

    let activate = |game: &Game, index: usize| {
        let mut actions = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Printed { ability, .. },
                    ..
                } if source == bomb => Some((
                    ability,
                    Action::ActivateAbility {
                        source,
                        ability: AbilityOrigin::Printed {
                            definition: cards::RATCHET_BOMB,
                            part: CardPartId::PRIMARY,
                            ability,
                        },
                        targets: Vec::new(),
                        sacrifice: None,
                        x: 0,
                    },
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        actions.sort_by_key(|(ability, _)| *ability);
        actions.get(index).map(|(_, action)| action.clone())
    };

    // Tick to one charge counter.
    let tick = activate(&game, 0).expect("the charge ability is activatable");
    game.apply(PlayerId::One, tick).unwrap();
    pass_priority_pair(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bomb)
        .expect("still there")
        .tapped = false;

    let detonate = activate(&game, 1).expect("the sweep ability is activatable");
    game.apply(PlayerId::One, detonate).unwrap();
    pass_priority_pair(&mut game);

    let left = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.definition)
        .collect::<Vec<_>>();
    assert_eq!(
        left,
        vec![cards::SERRA_ANGEL, cards::MOUNTAIN],
        "only the one-drop matched the single charge counter"
    );
}

#[test]
fn sigarda_stops_an_opponents_edict() {
    for sigarda_out in [false, true] {
        let mut game = ready_game();
        let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
        attacker.attacking = true;
        game.battlefield.push(attacker);
        if sigarda_out {
            game.battlefield.push(creature(
                10_003,
                cards::SIGARDA_HOST_OF_HERONS,
                PlayerId::Two,
            ));
        }
        let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
        game.players[0].hand.push(flare.clone());
        game.players[0].mana_pool.white = 2;

        game.apply(
            PlayerId::One,
            cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let attacker_survived = game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(10_000));
        assert_eq!(attacker_survived, sigarda_out, "Sigarda out: {sigarda_out}");
    }
}

#[test]
fn kessig_wolf_run_offers_only_the_x_it_can_actually_pay() {
    let mut game = ready_game();
    // Only the floating mana pays, so the affordable range is exact. The Run
    // itself taps for the ability, so its own colorless is not available.
    game.battlefield.clear();
    let run = game
        .put_onto_battlefield(PlayerId::One, cards::KESSIG_WOLF_RUN)
        .expect("cataloged");
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;

    let mut offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { source, x, .. } if source == run => Some(x),
            _ => None,
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();
    assert_eq!(offered, vec![0, 1, 2, 3], "five mana, less the two colored");

    let pump = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, x, .. } if *source == run && *x == 3))
        .expect("X of three is affordable");
    game.apply(PlayerId::One, pump).unwrap();
    pass_priority_pair(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions)
        .expect("still there");
    assert_eq!(game.power(lions), Some(5), "2/1 plus three");
    assert_eq!(game.toughness(lions), Some(1), "toughness is untouched");
    assert!(game.permanent_has_executable_keyword(lions, KeywordAbility::Trample));
}

#[test]
fn gaze_of_granite_sweeps_up_to_the_x_it_was_cast_for() {
    let mut game = ready_game();
    game.battlefield.clear();
    // One, five, and a land: X of two takes the first and spares the rest.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");
    let sweeper = card(10_000, cards::GAZE_OF_GRANITE, PlayerId::One);
    game.players[0].hand.push(sweeper.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(sweeper.id, Vec::new(), Vec::new(), 2),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL, cards::MOUNTAIN],
        "the two-or-less nonland permanent is the only one destroyed"
    );
}

#[test]
fn blasphemous_act_gets_cheaper_as_the_board_fills_up() {
    let mut game = ready_game();
    game.battlefield.clear();
    let act = card(10_000, cards::BLASPHEMOUS_ACT, PlayerId::One);
    game.players[0].hand.push(act.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 2;

    let castable = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == act.id))
    };
    assert!(!castable(&game), "nine mana is out of reach on three");

    // Six creatures take it to {2}{R}, which the pool covers. Both sides
    // count: the reduction is not about who controls them.
    for (index, owner) in [PlayerId::One, PlayerId::Two]
        .into_iter()
        .cycle()
        .take(6)
        .enumerate()
    {
        game.battlefield.push(creature(
            10_010 + u32::try_from(index).unwrap(),
            cards::SAVANNAH_LIONS,
            owner,
        ));
    }
    assert!(castable(&game), "six creatures pay for six of the eight");

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == act.id))
        .expect("castable");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "thirteen damage to each creature"
    );
}

#[test]
fn scavenging_ooze_only_grows_on_a_creature_card() {
    for (definition, expect_growth) in [(cards::SAVANNAH_LIONS, true), (cards::MOUNTAIN, false)] {
        let mut game = ready_game();
        let ooze = game
            .put_onto_battlefield(PlayerId::One, cards::SCAVENGING_OOZE)
            .expect("cataloged");
        let food = card(10_000, definition, PlayerId::Two);
        game.players[1].graveyard.push(food.clone());
        game.players[0].mana_pool.green = 1;
        let life_before = game.players[0].life;

        let eat = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == ooze))
            .expect("the graveyard card is a legal target");
        game.apply(PlayerId::One, eat).unwrap();
        pass_priority_pair(&mut game);

        assert!(game.players[1].graveyard.is_empty(), "it was exiled");
        assert_eq!(game.players[1].exile.len(), 1);
        let ooze = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == ooze)
            .expect("still there");
        let counters = ooze.counters[CounterKind::PlusOnePlusOne.index()];
        assert_eq!(counters, u16::from(expect_growth), "{definition:?}");
        assert_eq!(
            game.players[0].life - life_before,
            i16::from(expect_growth),
            "{definition:?}"
        );
    }
}

#[test]
fn demonic_rising_only_pays_off_with_exactly_one_creature() {
    for (creatures, expect_demon) in [(0, false), (1, true), (2, false)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.put_onto_battlefield(PlayerId::One, cards::DEMONIC_RISING)
            .expect("cataloged");
        for _ in 0..creatures {
            game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
                .expect("cataloged");
        }
        // The opponent's creatures are not yours, whatever the count.
        game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
            .expect("cataloged");

        game.step = Step::PostcombatMain;
        game.advance_step();
        for _ in 0..8 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }

        let demons = game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::DEMON_TOKEN_5_5_BLACK)
            .count();
        assert_eq!(demons, usize::from(expect_demon), "{creatures} creatures");
    }
}

#[test]
fn izzet_staticaster_hits_every_copy_of_the_creature_it_names() {
    let mut game = ready_game();
    game.battlefield.clear();
    let caster = game
        .put_onto_battlefield(PlayerId::One, cards::IZZET_STATICASTER)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == caster)
        .expect("just entered")
        .entered_controller_turn = game.turns_started[0] - 1;
    // Two Lions on one side, one on the other, and an unrelated creature.
    let mut lions = Vec::new();
    for owner in [PlayerId::Two, PlayerId::Two, PlayerId::One] {
        lions.push(
            game.put_onto_battlefield(owner, cards::SAVANNAH_LIONS)
                .expect("cataloged"),
        );
    }
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");

    let zap = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == caster
                && targets.iter().flat_map(TargetSelection::targets).any(|target| {
                    *target == Target::Permanent(lions[0])
                }))
        })
        .expect("a Lion is a legal target");
    game.apply(PlayerId::One, zap).unwrap();
    pass_priority_pair(&mut game);

    // A 2/1 dies to one damage, whoever controls it.
    assert!(
        lions
            .iter()
            .all(|lion| !game.battlefield.iter().any(|p| p.card.id == *lion)),
        "every Savannah Lions was named"
    );
    assert!(
        game.battlefield.iter().any(|p| p.card.id == angel),
        "the Angel shares no name"
    );
}

#[test]
fn izzet_staticaster_reads_the_name_copied_by_thespians_stage() {
    let mut game = ready_game();
    game.battlefield.clear();
    let stage = game
        .put_onto_battlefield(PlayerId::One, cards::THESPIANS_STAGE)
        .expect("cataloged");
    let arbor = game
        .put_onto_battlefield(PlayerId::One, cards::DRYAD_ARBOR)
        .expect("cataloged");
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage,
            ability: activated_ability_for(&game, stage, 0),
            targets: activated_targets(Target::Permanent(arbor)),
            sacrifice: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let caster = game
        .put_onto_battlefield(PlayerId::Two, cards::IZZET_STATICASTER)
        .expect("cataloged");
    game.priority = PlayerId::Two;
    game.consecutive_passes = 0;
    let zap = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
            if *source == caster
                && targets.iter().flat_map(TargetSelection::targets).any(|target| {
                    *target == Target::Permanent(stage)
                }))
        })
        .expect("the Stage presenting Dryad Arbor is a legal target");
    game.apply(PlayerId::Two, zap).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        [stage, arbor].iter().all(|id| !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == *id)),
        "the copied and physical Dryad Arbors share a copiable name",
    );
}

#[test]
fn oblivion_ring_gives_back_exactly_what_it_took() {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    // A second creature the Ring never touched, to prove the link is
    // specific rather than a sweep of the exile zone.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[1]
        .exile
        .push(card(10_050, cards::MOUNTAIN, PlayerId::Two));

    let ring = game
        .put_onto_battlefield(PlayerId::One, cards::OBLIVION_RING)
        .expect("cataloged");
    let drain = |game: &mut Game| {
        for _ in 0..10 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .filter(|option| option.card == Some((angel, cards::SERRA_ANGEL)))
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);

    assert!(
        !game.battlefield.iter().any(|p| p.card.id == angel),
        "the Angel was exiled"
    );

    game.destroy_permanent(ring);
    drain(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
        "the Angel came back and the unrelated exiled Mountain stayed put"
    );
    assert_eq!(game.players[1].exile.len(), 1, "the Mountain is untouched");
}

#[test]
fn detention_sphere_takes_every_copy_and_gives_them_all_back() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = [
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .expect("cataloged"),
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .expect("cataloged"),
    ];
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    let sphere = game
        .put_onto_battlefield(PlayerId::One, cards::DETENTION_SPHERE)
        .expect("cataloged");

    let drain = |game: &mut Game| {
        for _ in 0..12 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                // Take the optional exile, and name a Lion when asked.
                let options = decision
                    .options
                    .iter()
                    .filter(|option| {
                        option.label == "Do it"
                            || option.card == Some((lions[0], cards::SAVANNAH_LIONS))
                    })
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };

    // The Sphere's trigger needs a target chosen when it is put on the stack.
    let target = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ChooseDecision { .. }));
    if let Some(action) = target {
        let _ = game.apply(PlayerId::One, action);
    }
    drain(&mut game);

    let on_field = |game: &Game, id: GameObjectId| game.battlefield.iter().any(|p| p.card.id == id);
    assert!(
        !on_field(&game, lions[0]) && !on_field(&game, lions[1]),
        "both Lions left"
    );
    assert!(on_field(&game, angel), "the Angel shares no name");

    game.destroy_permanent(sphere);
    drain(&mut game);

    let names = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.definition)
        .collect::<Vec<_>>();
    assert_eq!(
        names
            .iter()
            .filter(|d| **d == cards::SAVANNAH_LIONS)
            .count(),
        2,
        "both Lions came back"
    );
}

#[test]
fn angel_of_serenity_takes_from_both_zones_and_returns_to_hand() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.players[1]
        .graveyard
        .push(card(10_050, cards::SERRA_ANGEL, PlayerId::Two));
    let hand_before = game.players[1].hand.len();

    let angel = game
        .put_onto_battlefield(PlayerId::One, cards::ANGEL_OF_SERENITY)
        .expect("cataloged");
    // Take every offered target, and accept the optional exile.
    let drain = |game: &mut Game| {
        for _ in 0..12 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let cards = decision
                    .options
                    .iter()
                    .filter(|option| option.card.is_some())
                    .map(|option| option.id)
                    .take(decision.maximum)
                    .collect::<Vec<_>>();
                let options = if cards.is_empty() {
                    decision
                        .options
                        .iter()
                        .filter(|option| option.label == "Do it")
                        .map(|option| option.id)
                        .chain(decision.options.iter().map(|option| option.id))
                        .take(decision.minimum.max(1))
                        .collect::<Vec<_>>()
                } else {
                    cards
                };
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == lions),
        "the creature on the battlefield was taken"
    );
    assert!(
        game.players[1].graveyard.is_empty(),
        "so was the creature card in the graveyard"
    );
    assert_eq!(game.players[1].exile.len(), 2);

    game.destroy_permanent(angel);
    drain(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        hand_before + 2,
        "both came back to hand rather than to the battlefield"
    );
    assert!(game.players[1].exile.is_empty());
}

#[test]
fn quicken_lets_one_sorcery_be_cast_at_instant_speed() {
    let mut game = ready_game();
    let quicken = card(10_000, cards::QUICKEN, PlayerId::One);
    game.players[0].hand.push(quicken.clone());
    let sorceries = [
        card(10_001, cards::MIND_TWIST, PlayerId::One),
        card(10_002, cards::MIND_TWIST, PlayerId::One),
    ];
    for sorcery in &sorceries {
        game.players[0].hand.push(sorcery.clone());
    }
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.black = 4;
    // The opponent's turn, where a sorcery is never castable.
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;

    let castable = |game: &Game, id: CardInstanceId| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    };
    assert!(!castable(&game, sorceries[0].id), "not on their turn");

    game.apply(
        PlayerId::One,
        cast_action(quicken.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // Quicken resolving handed priority back to the active player.
    game.priority = PlayerId::One;
    assert!(castable(&game, sorceries[0].id), "the grant covers it");
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[0].id))
        .expect("castable");
    game.apply(PlayerId::One, cast).unwrap();
    game.priority = PlayerId::One;

    assert!(
        !castable(&game, sorceries[1].id),
        "the grant covered the next sorcery, not every one"
    );
}

#[test]
fn obzedat_blinks_itself_and_comes_back_hasty_next_upkeep() {
    let mut game = ready_game();
    game.battlefield.clear();
    let obzedat = game
        .put_onto_battlefield(PlayerId::One, cards::OBZEDAT_GHOST_COUNCIL)
        .expect("cataloged");
    // Its entry trigger fires too; take every decision as it comes.
    let drain = |game: &mut Game| {
        for _ in 0..14 {
            if game.stack.is_empty()
                && game.pending_triggers.is_empty()
                && game.pending_decisions.is_empty()
            {
                break;
            }
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                let options = decision
                    .options
                    .iter()
                    .filter(|option| option.label == "Do it")
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                continue;
            }
            let player = game.priority;
            if game.apply(player, Action::PassPriority).is_err() {
                break;
            }
        }
    };
    drain(&mut game);
    assert!(
        game.battlefield.iter().any(|p| p.card.id == obzedat),
        "it starts on the battlefield"
    );

    game.step = Step::PostcombatMain;
    game.advance_step();
    drain(&mut game);
    assert!(
        !game.battlefield.iter().any(|p| p.card.id == obzedat),
        "the end step exiled it"
    );
    assert_eq!(game.players[0].exile.len(), 1);

    // Their turn, then back to ours: it returns at our upkeep, not theirs.
    game.start_next_turn();
    drain(&mut game);
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "not on the opponent's upkeep"
    );

    game.start_next_turn();
    drain(&mut game);
    let back = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::OBZEDAT_GHOST_COUNCIL)
        .expect("it came back");
    assert!(
        game.permanent_has_executable_keyword(back, KeywordAbility::Haste),
        "and it can attack straight away"
    );
    assert!(game.players[0].exile.is_empty());
}

#[test]
fn aetherling_dodges_a_blocker_and_comes_back_at_the_end_step() {
    let mut game = ready_game();
    game.battlefield.clear();
    let aetherling = game
        .put_onto_battlefield(PlayerId::One, cards::AETHERLING)
        .expect("cataloged");
    let wall = game
        .put_onto_battlefield(PlayerId::Two, cards::WALL_OF_STONE)
        .expect("cataloged");
    game.players[0].mana_pool.blue = 2;

    let activate = |game: &mut Game, index: usize| {
        let mut printed = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    ability: AbilityOrigin::Printed { ability, .. },
                    source,
                    ..
                } if source == aetherling => Some(ability),
                _ => None,
            })
            .collect::<Vec<_>>();
        printed.sort_unstable();
        printed.dedup();
        let ability = printed[index];
        game.apply(
            PlayerId::One,
            Action::ActivateAbility {
                source: aetherling,
                ability: AbilityOrigin::Printed {
                    definition: cards::AETHERLING,
                    part: CardPartId::PRIMARY,
                    ability,
                },
                targets: Vec::new(),
                sacrifice: None,
                x: 0,
            },
        )
        .unwrap();
        pass_priority_pair(game);
    };

    // The unblockable ability is the second printed clause.
    activate(&mut game, 1);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == aetherling)
        .expect("still there")
        .attacking = true;
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::DeclareBlocker { blocker, .. } if *blocker == wall)
        ),
        "nothing can block it this turn"
    );

    // The first clause blinks it until the end step.
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    activate(&mut game, 0);
    assert!(
        !game.battlefield.iter().any(|p| p.card.id == aetherling),
        "it left for exile"
    );

    game.advance_step();
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::AETHERLING),
        "and returned at the end step"
    );
}

#[test]
fn restoration_angel_blinks_a_creature_within_one_resolution() {
    let mut game = ready_game();
    game.battlefield.clear();
    let lions = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    // A creature the Angel may not target, so the choice is not vacuous.
    let serra = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.tap_permanent(lions);

    game.put_onto_battlefield(PlayerId::One, cards::RESTORATION_ANGEL)
        .expect("cataloged");
    for _ in 0..12 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let cards = decision
                .options
                .iter()
                .filter(|option| option.card.is_some())
                .map(|option| option.id)
                .take(decision.maximum)
                .collect::<Vec<_>>();
            let options = if cards.is_empty() {
                decision
                    .options
                    .iter()
                    .filter(|option| option.label == "Do it")
                    .map(|option| option.id)
                    .chain(decision.options.iter().map(|option| option.id))
                    .take(decision.minimum.max(1))
                    .collect::<Vec<_>>()
            } else {
                cards
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.players[0].exile.is_empty(),
        "the blink returned the card rather than leaving it exiled"
    );
    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the Lions came back to the battlefield");
    assert_ne!(
        returned.card.id, lions,
        "a blinked permanent returns as a new object"
    );
    assert!(!returned.tapped, "the new object is untapped");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == serra),
        "the untargetable Angel stayed put"
    );
}

/// Answers every waiting decision by taking what is offered and otherwise
/// passing, until the stack and the trigger queue are empty.
fn drain_pending(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1))
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Rule 603.4 checks an intervening-if twice. Shadowborn Demon is the pair of
/// checks in one card: a full graveyard means it never triggers, and a
/// graveyard filled after it triggers means the ability resolves for nothing.
#[test]
fn an_intervening_if_is_checked_when_it_triggers_and_again_when_it_resolves() {
    let graveyard = |game: &mut Game, creatures: usize| {
        game.players[0].graveyard = (0..creatures)
            .map(|index| {
                card(
                    11_000 + u32::try_from(index).expect("small index"),
                    cards::SAVANNAH_LIONS,
                    PlayerId::One,
                )
            })
            .collect();
    };
    let upkeep_with = |creatures: usize| {
        let mut game = ready_game();
        game.battlefield.clear();
        game.battlefield
            .push(creature(10_000, cards::SHADOWBORN_DEMON, PlayerId::One));
        game.battlefield
            .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
        graveyard(&mut game, creatures);
        game.turn = 2;
        game.step = Step::Upkeep;
        game.handle_upkeep_triggers();
        game
    };

    // Five is fewer than six, so the Demon is hungry.
    let mut hungry = upkeep_with(5);
    assert!(
        !hungry.pending_triggers.is_empty() || !hungry.stack.is_empty(),
        "the condition held, so the ability triggered"
    );

    // Six is not fewer than six, so it never triggers at all.
    let fed = upkeep_with(6);
    assert!(
        fed.pending_triggers.is_empty() && fed.stack.is_empty(),
        "the condition failed, so nothing triggered"
    );

    // Filling the graveyard after the trigger makes it resolve for nothing.
    let mut interrupted = upkeep_with(5);
    graveyard(&mut interrupted, 6);
    drain_pending(&mut interrupted);
    assert_eq!(
        interrupted.battlefield.len(),
        2,
        "the second check failed, so nothing was sacrificed"
    );

    // Left alone, the Demon eats. Which creature it takes is its controller's
    // choice, and the Demon itself is a legal one.
    drain_pending(&mut hungry);
    assert_eq!(
        hungry.battlefield.len(),
        1,
        "both checks held, so a creature was sacrificed"
    );
}

#[test]
fn delayed_trigger_partition_preserves_order_and_waiting_capacity() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const LOSE_TWO: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    };
    const LOSE_THREE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    const LOSE_FOUR: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(4),
    };
    let delayed = |id: u32, step: TurnStepDef, effect: &'static EffectDef| DelayedTrigger {
        object: Box::new(spell(id, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        step,
        player: PlayerRelation::Any,
        effect,
    };

    let mut game = ready_game();
    game.delayed_triggers = Vec::with_capacity(8);
    game.delayed_triggers.extend([
        delayed(10_000, TurnStepDef::End, &LOSE_ONE),
        delayed(10_001, TurnStepDef::Draw, &LOSE_THREE),
        delayed(10_002, TurnStepDef::End, &LOSE_TWO),
        delayed(10_003, TurnStepDef::Draw, &LOSE_FOUR),
    ]);
    let waiting_capacity = game.delayed_triggers.capacity();
    let event_start = game.events.len();

    game.fire_delayed_triggers(TurnStepDef::End);

    let lost = game.events[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount,
            } => Some(*amount),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(lost, vec![1, 2], "due effects keep their queued order");
    assert_eq!(
        game.delayed_triggers
            .iter()
            .map(|delayed| delayed.object.id.0)
            .collect::<Vec<_>>(),
        vec![10_001, 10_003],
        "waiting effects keep their queued order"
    );
    assert_eq!(
        game.delayed_triggers.capacity(),
        waiting_capacity,
        "partitioning reuses the waiting queue allocation"
    );
}

#[test]
fn delayed_effect_enqueued_during_firing_waits_for_the_next_matching_step() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    const ENQUEUE_LOSS: EffectDef = EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: &LOSE_ONE,
    };
    let mut game = ready_game();
    game.delayed_triggers = Vec::with_capacity(4);
    game.delayed_triggers.push(DelayedTrigger {
        object: Box::new(spell(10_000, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: &ENQUEUE_LOSS,
    });
    let waiting_capacity = game.delayed_triggers.capacity();
    let life_before = game.players[0].life;

    game.fire_delayed_triggers(TurnStepDef::End);

    assert_eq!(game.players[0].life, life_before);
    assert_eq!(game.delayed_triggers.len(), 1);
    assert_eq!(*game.delayed_triggers[0].effect, LOSE_ONE);
    assert_eq!(game.delayed_triggers.capacity(), waiting_capacity);

    game.fire_delayed_triggers(TurnStepDef::End);

    assert_eq!(game.players[0].life, life_before - 1);
    assert!(game.delayed_triggers.is_empty());
    assert_eq!(game.delayed_triggers.capacity(), waiting_capacity);
}

#[test]
fn stacked_quickens_are_all_spent_by_the_same_next_sorcery() {
    let mut game = ready_game();
    let quickens = [
        card(10_000, cards::QUICKEN, PlayerId::One),
        card(10_001, cards::QUICKEN, PlayerId::One),
    ];
    let sorceries = [
        card(10_002, cards::MIND_TWIST, PlayerId::One),
        card(10_003, cards::MIND_TWIST, PlayerId::One),
    ];
    game.players[0].hand.extend(quickens.iter().cloned());
    game.players[0].hand.extend(sorceries.iter().cloned());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.black = 4;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;

    for quicken in &quickens {
        game.apply(
            PlayerId::One,
            cast_action(quicken.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);
        game.priority = PlayerId::One;
    }
    assert_eq!(game.sorcery_flash_grants[0], 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[0].id))
        .expect("both Quicken grants cover the same next sorcery");
    game.apply(PlayerId::One, cast).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 0);
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == sorceries[1].id)
        ),
        "the second sorcery needs a new timing permission"
    );
}

#[test]
fn quicken_consumes_its_grant_for_the_selected_sorcery_part() {
    let definition_id = CardDefinitionId(10_068);
    let instant = CardRules::new_instant(ManaCost::default());
    let sorcery = CardRules::new_sorcery(ManaCost::default());
    let (mut game, _, _) = game_with_test_fused_split(definition_id, &instant, &sorcery);
    let split = card(10_000, definition_id, PlayerId::One);
    let next_sorcery = card(10_001, cards::MIND_TWIST, PlayerId::One);
    game.players[0]
        .hand
        .extend([split.clone(), next_sorcery.clone()]);
    game.players[0].mana_pool.black = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.sorcery_flash_grants[0] = 1;

    let cast_second_part = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == split.id && choices.play_option() == PlayOptionId(1)
            )
        })
        .expect("Quicken makes the selected sorcery part castable now");
    game.apply(PlayerId::One, cast_second_part).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 0);
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == next_sorcery.id)
        ),
        "consumption follows the selected part rather than the primary instant characteristics"
    );
}

#[test]
fn quicken_preserves_its_grant_for_the_selected_instant_part() {
    let definition_id = CardDefinitionId(10_069);
    let sorcery = CardRules::new_sorcery(ManaCost::default());
    let instant = CardRules::new_instant(ManaCost::default());
    let (mut game, _, _) = game_with_test_fused_split(definition_id, &sorcery, &instant);
    let split = card(10_000, definition_id, PlayerId::One);
    let next_sorcery = card(10_001, cards::MIND_TWIST, PlayerId::One);
    game.players[0]
        .hand
        .extend([split.clone(), next_sorcery.clone()]);
    game.players[0].mana_pool.black = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.sorcery_flash_grants[0] = 1;

    let cast_second_part = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == split.id && choices.play_option() == PlayOptionId(1)
            )
        })
        .expect("the selected instant part is castable without using Quicken");
    game.apply(PlayerId::One, cast_second_part).unwrap();
    game.priority = PlayerId::One;

    assert_eq!(game.sorcery_flash_grants[0], 1);
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::CastSpell { card, .. } if *card == next_sorcery.id)
        ),
        "the grant remains available for the next sorcery"
    );
}
