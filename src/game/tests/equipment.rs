//! Equip.
//!
//! Equipment and Auras both attach, and until now only Auras did. The
//! differences are the interesting part: an Aura attaches as its own spell
//! resolves and dies when it comes loose, while Equipment attaches through an
//! ability at sorcery speed and simply stays put.

use super::*;

const MANA_AND_TAP_ELF_EQUIPMENT_ID: CardDefinitionId = CardDefinitionId::new(10_090);
const COUNTER_MANA_ELF_ID: CardDefinitionId = CardDefinitionId::new(10_091);
const COUNT_ONE_SOURCE_EQUIPMENT_ID: CardDefinitionId = CardDefinitionId::new(10_092);

static MANA_AND_TAP_ELF_EQUIP_COSTS: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{G}")),
    AbilityCostDef::TapPermanents {
        object: ObjectPredicateDef::Subtype("Elf"),
        controller: PlayerRelation::You,
        count: 1,
    },
];

static MANA_AND_TAP_ELF_EQUIP_ABILITIES: [AbilityDef; 1] = [abilities::equip(
    &MANA_AND_TAP_ELF_EQUIP_COSTS,
    "{G}, Tap an untapped Elf you control: Attach this Equipment to target creature you control. Equip only as a sorcery.",
)];

static COUNTER_MANA_ELF_COSTS: [AbilityCostDef; 1] = [AbilityCostDef::RemoveCountersFromSource {
    kind: CounterKind::named("charge"),
    amount: 1,
}];

static COUNTER_MANA_ELF_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "Remove a charge counter from this creature: Add {G}.",
    &COUNTER_MANA_ELF_COSTS,
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
)];

static COUNT_ONE_SOURCE_EQUIP_COSTS: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{G}")),
    AbilityCostDef::TapPermanents {
        object: ObjectPredicateDef::HasType(CardType::Artifact),
        controller: PlayerRelation::You,
        count: 1,
    },
];

static COUNT_ONE_SOURCE_EQUIP_ABILITIES: [AbilityDef; 1] = [abilities::equip(
    &COUNT_ONE_SOURCE_EQUIP_COSTS,
    "{G}, Tap an untapped artifact you control: Attach this Equipment to target creature you control. Equip only as a sorcery.",
)];

fn mana_and_tap_elf_equipment_definition() -> CardDefinition {
    let mut definition = CardDefinition::new(
        MANA_AND_TAP_ELF_EQUIPMENT_ID,
        "Mana and tap-cost Equipment test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default())
        .with_subtypes(&["Equipment"])
        .with_abilities(&MANA_AND_TAP_ELF_EQUIP_ABILITIES);
    synchronize_single_part_definition(&mut definition);
    definition
}

fn counter_mana_elf_definition() -> CardDefinition {
    let mut definition = CardDefinition::new(
        COUNTER_MANA_ELF_ID,
        "Counter mana Elf test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_creature(ManaCost::default(), &["Elf"], 1, 1)
        .with_abilities(&COUNTER_MANA_ELF_ABILITIES);
    synchronize_single_part_definition(&mut definition);
    definition
}

fn count_one_source_equipment_definition() -> CardDefinition {
    let mut definition = CardDefinition::new(
        COUNT_ONE_SOURCE_EQUIPMENT_ID,
        "Count-one source Equipment test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default())
        .with_subtypes(&["Equipment"])
        .with_abilities(&COUNT_ONE_SOURCE_EQUIP_ABILITIES);
    synchronize_single_part_definition(&mut definition);
    definition
}

fn equipped_board() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let staff = creature(10_000, cards::COBBLED_WINGS, PlayerId::One);
    let staff_id = staff.card.id;
    game.battlefield.push(staff);
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    (game, staff_id, troll_id)
}

fn equip(game: &mut Game, source: GameObjectId, host: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } => {
                *actual == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host))
            }
            _ => false,
        })
        .expect("equip is offered for that creature");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

fn attached_to(game: &Game, id: GameObjectId) -> Option<GameObjectId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield")
        .attached_to
}

fn is_tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
        .tapped
}

fn mixed_cost_equip_action(
    game: &Game,
    equipment: GameObjectId,
    host: GameObjectId,
    cost_object: GameObjectId,
) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    cost_objects,
                    ..
                } if *source == equipment
                    && cost_objects.as_slice() == [cost_object]
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host))
            )
        })
}

#[test]
fn equipping_attaches_and_grants_its_bonus() {
    let (mut game, staff_id, troll_id) = equipped_board();
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("there");
    assert!(
        !game.permanent_has_executable_keyword(troll, KeywordAbility::Flying),
        "no flying before it is equipped"
    );

    equip(&mut game, staff_id, troll_id);

    assert_eq!(attached_to(&game, staff_id), Some(troll_id));
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("there");
    assert!(
        game.permanent_has_executable_keyword(troll, KeywordAbility::Flying),
        "the equipped creature has flying"
    );
}

#[test]
fn a_tap_cost_candidate_cannot_also_pay_the_equip_mana_cost() {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(mana_and_tap_elf_equipment_definition());
    game.catalog = CardCatalog::new(definitions).unwrap();

    let equipment = creature(10_000, MANA_AND_TAP_ELF_EQUIPMENT_ID, PlayerId::One);
    let equipment_id = equipment.card.id;
    let host = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let mana_elf = creature(10_002, cards::LLANOWAR_ELVES, PlayerId::One);
    let mana_elf_id = mana_elf.card.id;
    game.battlefield.extend([equipment, host, mana_elf]);

    assert!(
        mixed_cost_equip_action(&game, equipment_id, host_id, mana_elf_id).is_none(),
        "the Elf cannot both tap for mana and pay the separate tap cost",
    );

    let other_elf = creature(10_003, cards::ELVISH_ARCHERS, PlayerId::One);
    let other_elf_id = other_elf.card.id;
    game.battlefield.push(other_elf);
    let action = mixed_cost_equip_action(&game, equipment_id, host_id, other_elf_id)
        .expect("the other Elf can tap while Llanowar Elves pays the mana");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mana_elf_id],
    );

    game.apply(PlayerId::One, action)
        .expect("the mixed equip activation is legal");
    assert!(is_tapped(&game, mana_elf_id));
    assert!(is_tapped(&game, other_elf_id));
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 0);
    assert_eq!(
        attached_to(&game, equipment_id),
        None,
        "equip uses the stack"
    );

    drain_pending(&mut game);
    assert_eq!(attached_to(&game, equipment_id), Some(host_id));
}

#[test]
fn a_non_tapping_mana_ability_can_share_the_equip_tap_cost_payer() {
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([
        mana_and_tap_elf_equipment_definition(),
        counter_mana_elf_definition(),
    ]);
    game.catalog = CardCatalog::new(definitions).unwrap();

    let equipment = creature(10_000, MANA_AND_TAP_ELF_EQUIPMENT_ID, PlayerId::One);
    let equipment_id = equipment.card.id;
    let host = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let mut mana_elf = creature(10_002, COUNTER_MANA_ELF_ID, PlayerId::One);
    mana_elf.counters.set(CounterKind::named("charge"), 1);
    let mana_elf_id = mana_elf.card.id;
    game.battlefield.extend([equipment, host, mana_elf]);

    let action = mixed_cost_equip_action(&game, equipment_id, host_id, mana_elf_id)
        .expect("a counter-only mana ability leaves the Elf available to tap");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &action),
        vec![mana_elf_id],
    );

    game.apply(PlayerId::One, action)
        .expect("the same Elf can raise mana and pay the tap cost");
    let mana_elf = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mana_elf_id)
        .expect("the counter-only mana ability preserves its source");
    assert!(mana_elf.tapped);
    assert_eq!(mana_elf.counters(CounterKind::named("charge")), 0);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 0);

    drain_pending(&mut game);
    assert_eq!(attached_to(&game, equipment_id), Some(host_id));
}

#[test]
fn a_count_one_tap_cost_can_use_its_untapped_source() {
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(count_one_source_equipment_definition());
    game.catalog = CardCatalog::new(definitions).unwrap();

    let equipment = creature(10_000, COUNT_ONE_SOURCE_EQUIPMENT_ID, PlayerId::One);
    let equipment_id = equipment.card.id;
    let host = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.extend([equipment, host]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);

    let action = mixed_cost_equip_action(&game, equipment_id, host_id, equipment_id)
        .expect("the eligible untapped source can pay the count-one tap cost");
    game.apply(PlayerId::One, action).expect("it activates");

    assert!(is_tapped(&game, equipment_id));
    drain_pending(&mut game);
    assert_eq!(attached_to(&game, equipment_id), Some(host_id));
}

/// Equip is sorcery-speed, which is what stops it being an instant-speed
/// combat trick.
#[test]
fn equip_is_not_offered_outside_a_main_phase() {
    let (mut game, staff_id, _) = equipped_board();
    game.step = Step::DeclareBlockers;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == staff_id)
        }),
        "equip waits for a main phase"
    );
}

/// The difference that matters: an Aura in this position would be in the
/// graveyard, and Equipment is not.
#[test]
fn losing_its_creature_leaves_the_equipment_on_the_battlefield() {
    let (mut game, staff_id, troll_id) = equipped_board();
    equip(&mut game, staff_id, troll_id);

    game.battlefield
        .retain(|permanent| permanent.card.id != troll_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == staff_id),
        "it stays put rather than dying with its creature"
    );
    assert_eq!(
        attached_to(&game, staff_id),
        None,
        "and it comes loose rather than staying attached to nothing"
    );
}

/// Equipping again moves it rather than attaching twice.
#[test]
fn equipping_a_second_creature_moves_it() {
    let (mut game, staff_id, troll_id) = equipped_board();
    let second = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let second_id = second.card.id;
    game.battlefield.push(second);
    game.players[PlayerId::One.index()].mana_pool.colorless = 6;

    equip(&mut game, staff_id, troll_id);
    equip(&mut game, staff_id, second_id);

    assert_eq!(attached_to(&game, staff_id), Some(second_id));
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("there");
    assert!(
        !game.permanent_has_executable_keyword(troll, KeywordAbility::Flying),
        "the creature it left keeps nothing"
    );
}

/// "As long as equipped creature is a Human" follows the Equipment, so the
/// same Pitchfork gives +1/+1 on one creature and nothing on another.
#[test]
fn a_conditional_bonus_follows_the_attachment() {
    let mut game = ready_game();
    let pitchfork = creature(10_000, cards::SHARPENED_PITCHFORK, PlayerId::One);
    let pitchfork_id = pitchfork.card.id;
    game.battlefield.push(pitchfork);
    // Savannah Lions is a Cat, and Icatian Moneychanger is a Human.
    let cat = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let cat_id = cat.card.id;
    game.battlefield.push(cat);
    let human = creature(10_002, cards::ICATIAN_MONEYCHANGER, PlayerId::One);
    let human_id = human.card.id;
    game.battlefield.push(human);
    game.players[PlayerId::One.index()].mana_pool.colorless = 6;

    equip(&mut game, pitchfork_id, cat_id);
    let cat = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == cat_id)
        .expect("there");
    assert_eq!(
        (game.power(cat), game.toughness(cat)),
        (Some(2), Some(1)),
        "a Cat is not a Human, so it gets only first strike"
    );

    equip(&mut game, pitchfork_id, human_id);
    let human = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == human_id)
        .expect("there");
    assert_eq!(
        (game.power(human), game.toughness(human)),
        (Some(1), Some(3)),
        "a 0/2 Human with the conditional +1/+1"
    );
    let cat = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == cat_id)
        .expect("there");
    assert_eq!(
        (game.power(cat), game.toughness(cat)),
        (Some(2), Some(1)),
        "and the Cat is back to printed"
    );
}
