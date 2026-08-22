use super::*;

/// Puts `library` on top of player one's library, top card first.
/// Stacks a library top card first. The top of a library is the end of the
/// vector, which is the end a draw takes from, so the first entry listed here
/// is the last one pushed.
pub(super) fn stack_library(game: &mut Game, library: &[(u32, CardDefinitionId)]) {
    for (instance, definition) in library.iter().rev() {
        game.players[0]
            .library
            .push(card(*instance, *definition, PlayerId::One));
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
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
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
    let definition_id = CardDefinitionId::new(10_075);
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
    planeswalker.set_counters(CounterKind::Loyalty, 3);
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
    let sinkhole = card(12_002, cards::SINKHOLE, PlayerId::Two);
    game.players[1]
        .hand
        .extend([lions.clone(), bolt.clone(), sinkhole.clone()]);
    cast_and_place_reveal_trigger(&mut game, 12_100, cards::SIN_COLLECTOR);

    pass_priority_pair(&mut game);

    // The instant and sorcery qualify; the creature is not offered.
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert_eq!(offered, vec![cards::LIGHTNING_BOLT, cards::SINKHOLE]);
    assert_eq!(
        game.observe(PlayerId::One).last_seen_hand,
        Some((
            PlayerId::Two,
            vec![
                (lions.id, lions.definition),
                (bolt.id, bolt.definition),
                (sinkhole.id, sinkhole.definition),
            ],
        )),
        "revealing the hand exposes ineligible cards too",
    );

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![
                decision
                    .options
                    .iter()
                    .find(|option| {
                        option.card.is_some_and(|(_, characteristics)| {
                            characteristics.card_definition() == Some(cards::LIGHTNING_BOLT)
                        })
                    })
                    .expect("the Bolt is a legal choice")
                    .id,
            ],
        },
    )
    .unwrap();

    assert_eq!(game.players[1].exile[0].definition, cards::LIGHTNING_BOLT);
    assert_eq!(game.players[1].hand.len(), 2, "the other cards stay");
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
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert_eq!(offered, vec![cards::SAVANNAH_LIONS, cards::ARBOR_ELF]);

    let elf = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::ARBOR_ELF)
            })
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
            cost_objects: vec![ring_id],
            x: 0,
            modes: Vec::new(),
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
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
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
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_080);
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
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Subtype("Mountain"),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_081);
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
    assert_eq!(
        game.pending_triggers[0].context.trigger.object,
        Some(taiga_id)
    );
}
