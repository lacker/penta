use super::*;
use crate::card::{
    ChoiceVisibilityDef, ChooseDef, EffectPaymentDef, ObjectChoiceBindingDef, PayOrDef,
    PlayerSetDef, abilities, tokens,
};
use crate::mana_cost;
use crate::poc::{self, cards};
use crate::{
    AbilityCoverageDef, AbilityTargetDef, AbilityTargetPredicate, AdditionalCostDef,
    AdditionalCostId, AlternativeCastManaCostDef, AlternativeCostDef, AlternativeCostId,
    BattlefieldEntryModificationDef, CardComposition, CardDefinition, CardEffectStatus,
    CardInstanceId, CardPart, CardPartId, CardPrinting, CardRules, CardStructure, CastChoices,
    DoubleFacedKind, EffectExecutionDef, ManaSpendEffectDef, ModeDef, ModeSetDef,
    ObjectBindingIndex, ObjectSetDef, PlayOptionDef, PlayOptionId, PlayerRelation,
    ReplacementEffectDef, ReplacementEventDef, SpellForm, StackObjectId, TargetIndex,
    TargetPredicate, TargetSelection, TargetSlotDef, TargetSlotId, ZonePlacement,
};

mod token_fixtures;
pub(super) use token_fixtures::*;

static TEST_FLYING_ABILITY: [AbilityDef; 1] = [abilities::flying()];
static TEST_FLYING_TRAMPLE_ABILITIES: [AbilityDef; 2] = [abilities::flying(), abilities::trample()];
pub(super) static TEST_MISHRAS_FACTORY_CHARACTERISTICS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(
        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
    ),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Assembly-Worker"])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];
static CARD_COST_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();
const TEST_OPPONENT_LAND_ENTRY_TEXT: &str = "Lands your opponents control enter tapped.";
static TEST_OPPONENT_LANDS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    TEST_OPPONENT_LAND_ENTRY_TEXT,
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasType(CardType::Land),
        controller: PlayerRelation::Opponent,
        cast: None,
    },
    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
)];
static TEST_EXTERNAL_ENTER_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];
static TEST_EXTERNAL_PAYMENT: [ReplacementEffectDef; 1] = [ReplacementEffectDef::PayOr {
    payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 2),
    if_paid: &[],
    if_declined: &TEST_EXTERNAL_ENTER_TAPPED,
}];
static TEST_EXTERNAL_CONTEXT_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    "Lands your opponents control enter tapped unless you control a Plains and pay 2 life.",
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasType(CardType::Land),
        controller: PlayerRelation::Opponent,
        cast: None,
    },
    ReplacementEffectDef::Conditional {
        condition: ConditionDef::Exists(ObjectQueryDef::matching(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        )),
        if_true: &TEST_EXTERNAL_PAYMENT,
        if_false: &TEST_EXTERNAL_ENTER_TAPPED,
    },
)];
static TEST_GRANTED_ENTRY_REPLACEMENT: AbilityDef =
    abilities::enters_tapped("This permanent enters tapped.");
static TEST_SELF_GRANTED_ENTRY_ABILITY: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This permanent has \"This permanent enters tapped.\"",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&TEST_GRANTED_ENTRY_REPLACEMENT),
    },
)];
static TEST_SELF_PLAINS_ABILITY: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This land is a Plains in addition to its other types.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Plains]),
    },
)];
static TEST_PLAINS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    "Plains your opponents control enter tapped.",
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        controller: PlayerRelation::Opponent,
        cast: None,
    },
    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
)];
static TEST_OPPONENT_ENCHANTMENTS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
        "Enchantments your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::HasType(CardType::Enchantment),
            controller: PlayerRelation::Opponent,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )];

pub(super) fn ready_game() -> Game {
    ready_game_with_seed(0)
}

/// The same board with a chosen seed, for the effects that consult the
/// replay-stable randomiser.
pub(super) fn ready_game_with_seed(seed: u64) -> Game {
    let deck = poc::mono_red_atog();
    let mut game = Game::new(poc::catalog().unwrap(), [deck.clone(), deck], seed).unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.stack.clear();
    game.pending_decisions.clear();
    game.pending_combat_assignments.clear();
    game.combat_damage_stage = CombatDamageStage::NotStarted;
    game.combat_blocked_attackers.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.exile.clear();
        player.outside_game.clear();
        player.life = i16::from(rules::STARTING_LIFE);
        player.mana_pool = ManaPool::default();
        player.mana.clear();
    }
    game
}

/// A bot-wire checkpoint and the true hidden-zone hypothesis behind it.
/// Focused reconstruction tests mutate the former and use the latter only to
/// prove that authored executable state cannot be spliced through the wire.
pub(super) fn checkpoint_fixture(
    game: &Game,
    viewer: PlayerId,
) -> (serde_json::Value, serde_json::Value) {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let definitions = |cards: &[CardInstance]| {
        cards
            .iter()
            .map(|card| card.definition.0)
            .collect::<Vec<_>>()
    };
    let opponent = viewer.opponent();
    let opponent_label = match opponent {
        PlayerId::One => "p1",
        PlayerId::Two => "p2",
    };
    let opponent_hand = &game.players[opponent.index()].hand;
    let drawn_indices = game.drawn_this_turn[opponent.index()]
        .iter()
        .filter_map(|id| opponent_hand.iter().position(|card| card.id == *id))
        .collect::<Vec<_>>();
    let hidden = serde_json::json!({
        "hands": {(opponent_label): definitions(opponent_hand)},
        "libraries": {
            "p1": definitions(&game.players[PlayerId::One.index()].library),
            "p2": definitions(&game.players[PlayerId::Two.index()].library),
        },
        "outsideGame": {
            "p1": definitions(&game.players[PlayerId::One.index()].outside_game),
            "p2": definitions(&game.players[PlayerId::Two.index()].outside_game),
        },
        "drawnThisTurn": {(opponent_label): drawn_indices},
    });
    (wire, hidden)
}

pub(super) fn card(id: u32, definition: CardDefinitionId, owner: PlayerId) -> CardInstance {
    CardInstance {
        id: CardInstanceId(id),
        definition,
        owner,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(id)]),
        characteristics: CharacteristicSource::Card(definition),
        counters: [0; CounterKind::COUNT],
    }
}

pub(super) fn creature(id: u32, definition: CardDefinitionId, controller: PlayerId) -> Permanent {
    Permanent::entering(
        card(id, definition, controller),
        CardPartId::PRIMARY,
        controller,
        0,
    )
}

pub(super) fn token_permanent(
    id: u32,
    token: TokenCharacteristics,
    controller: PlayerId,
) -> Permanent {
    let object = ObjectInstance {
        id: GameObjectId(id),
        definition: ObjectKind::Token,
        owner: controller,
        backing: ObjectBacking::None,
        characteristics: CharacteristicSource::Token(token),
        counters: [0; CounterKind::COUNT],
    };
    Permanent::entering_token(object, token, controller, 0)
}

pub(super) fn is_token_with(permanent: &Permanent, token: TokenCharacteristics) -> bool {
    let Some(actual) = Game::effective_rules_source(permanent).token_characteristics() else {
        return false;
    };
    permanent.card.definition.is_token()
        && actual.name() == token.name()
        && actual.rules() == token.rules()
        && actual.structure == token.structure
}

/// Attach constant resolved characteristic leaves to one permanent as a
/// single timestamped effect. Test setup uses this instead of recreating the
/// fragmented animation, ability, and power/toughness state this model
/// replaced.
#[allow(clippy::too_many_lines)]
pub(super) fn attach_constant_resolved_characteristics(
    game: &mut Game,
    permanent: GameObjectId,
    effects: &[AppliedEffectDef],
    expiration: ContinuousEffectExpiration,
) -> ContinuousEffectTimestamp {
    fn flatten(effect: AppliedEffectDef, leaves: &mut Vec<AppliedEffectDef>) {
        match effect {
            AppliedEffectDef::Composite(components) => {
                for component in components {
                    flatten(*component, leaves);
                }
            }
            leaf => leaves.push(leaf),
        }
    }

    let timestamp = game.allocate_continuous_effect_timestamp();
    let target = game
        .battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("the resolved characteristic target is on the battlefield");
    let source = AbilitySourceRef {
        object: permanent,
        ability: AbilityOrigin::Printed {
            definition: target
                .card
                .definition
                .card_definition()
                .expect("the fixture source is a printed card"),
            part: target.presented,
            ability: AbilityId::PRIMARY,
        },
    };
    let mut leaves = Vec::new();
    for effect in effects {
        flatten(*effect, &mut leaves);
    }
    let mut used_grants = [false; 256];
    for grant in target
        .resolved_continuous_effects
        .iter()
        .filter_map(|effect| match effect.kind {
            ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                grant,
                ..
            }) => Some(grant),
            _ => None,
        })
    {
        used_grants[grant.index()] = true;
    }
    let target = game
        .battlefield
        .iter_mut()
        .find(|candidate| candidate.card.id == permanent)
        .expect("the resolved characteristic target is on the battlefield");
    for (component_order, definition) in leaves.into_iter().enumerate() {
        let AppliedEffectDef::Characteristic(operation) = definition else {
            panic!("resolved characteristic fixtures accept only characteristic leaves");
        };
        let kind = match operation {
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Add(ability)) => {
                let grant = used_grants
                    .iter()
                    .position(|used| !used)
                    .and_then(GrantId::from_index)
                    .expect("one fixture permanent has at most 256 resolved grants");
                used_grants[grant.index()] = true;
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                    ability: *ability,
                    grant,
                })
            }
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Remove(predicate)) => {
                ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Remove(predicate))
            }
            CharacteristicOperationDef::Abilities(
                AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles,
            ) => panic!("a linked-exile grant is a static shape, not a resolved one"),
            CharacteristicOperationDef::BasicLandTypes(operation) => {
                ResolvedContinuousEffectKind::BasicLandTypes(operation)
            }
            CharacteristicOperationDef::CardTypes(operation) => {
                ResolvedContinuousEffectKind::CardTypes(operation)
            }
            CharacteristicOperationDef::Colors(operation) => {
                ResolvedContinuousEffectKind::Colors(operation)
            }
            CharacteristicOperationDef::CreatureTypes(operation) => {
                ResolvedContinuousEffectKind::CreatureTypes(operation)
            }
            CharacteristicOperationDef::Subtypes(operation) => {
                ResolvedContinuousEffectKind::Subtypes(operation)
            }
            CharacteristicOperationDef::PowerToughness(operation) => {
                let constant = |value| {
                    let ValueDef::Constant(value) = value else {
                        panic!("resolved characteristic fixtures require constant P/T values");
                    };
                    i16::try_from(value).expect("fixture P/T fits in i16")
                };
                ResolvedContinuousEffectKind::PowerToughness(match operation {
                    PowerToughnessOperationDef::SetBase { power, toughness } => {
                        ResolvedPowerToughnessOperation::SetBase {
                            power: constant(power),
                            toughness: constant(toughness),
                        }
                    }
                    PowerToughnessOperationDef::SetBasePower(power) => {
                        ResolvedPowerToughnessOperation::SetBasePower {
                            power: constant(power),
                        }
                    }
                    PowerToughnessOperationDef::SetBaseToughness(toughness) => {
                        ResolvedPowerToughnessOperation::SetBaseToughness {
                            toughness: constant(toughness),
                        }
                    }
                    PowerToughnessOperationDef::Modify { power, toughness } => {
                        ResolvedPowerToughnessOperation::Modify {
                            power: constant(power),
                            toughness: constant(toughness),
                        }
                    }
                    PowerToughnessOperationDef::Switch => ResolvedPowerToughnessOperation::Switch,
                })
            }
        };
        target
            .resolved_continuous_effects
            .push(ResolvedContinuousEffect {
                definition,
                source,
                timestamp,
                component_order: u16::try_from(component_order)
                    .expect("one fixture effect has at most 65,536 components"),
                expiration,
                kind,
            });
    }
    timestamp
}

fn copied_characteristics(definition: CardDefinitionId) -> CopiableCharacteristics {
    CopiableCharacteristics {
        base: ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
        added_types: CardTypeSet::empty(),
        added_abilities: Vec::new(),
        retain_printed_subtypes: false,
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

pub(super) fn cast_action(
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

pub(super) fn mana_ability_for(
    game: &Game,
    source: GameObjectId,
    color: ManaColor,
) -> AbilityOrigin {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .into_iter()
        .flat_map(|permanent| game.mana_ability_activations(permanent))
        .find(|activation| activation.color == color)
        .expect("source has an effective mana ability for the requested color")
        .ability
}

pub(in crate::game) fn activated_ability_for(
    game: &Game,
    source: GameObjectId,
    index: usize,
) -> AbilityOrigin {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .expect("source is on the battlefield");
    game.activated_ability_origin(permanent, index)
}

/// The plain activation of one ability: no targets, no cost objects, no X,
/// and no modes. Most printed abilities are activated exactly this way.
fn plain_activation(source: GameObjectId, ability: AbilityOrigin) -> Action {
    Action::ActivateAbility {
        source,
        ability,
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
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
        card: card(id, definition, controller).into(),
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
        colors: None,
        cast_via_flashback: false,
        cast_at_instant_speed: false,
        cast_from_zone: None,
        cast_face_down: false,
        colors_of_mana_spent: ColorSet::empty(),
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

/// Takes the split an unassigned attacker would have made anyway: lethal to
/// each blocker in order, the rest over the top. A lone blocker used to get
/// this for free; trample now makes it a real choice, so a test that only
/// cares about the damage totals asks for the obvious one.
fn take_default_combat_assignment(game: &mut Game) {
    while let Some(action) = game
        .legal_actions(game.priority)
        .into_iter()
        .find(|action| matches!(action, Action::AssignCombatDamage { .. }))
    {
        let Action::AssignCombatDamage { attacker, .. } = &action else {
            unreachable!("filtered to assignments");
        };
        let blockers: Vec<_> = game
            .battlefield
            .iter()
            .filter(|permanent| permanent.blocking == vec![*attacker])
            .map(|permanent| permanent.card.id)
            .collect();
        let split = game.default_damage_split(*attacker, &blockers);
        let wanted = Action::AssignCombatDamage {
            attacker: *attacker,
            assignments: split
                .into_iter()
                .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
                .collect(),
        };
        let player = game.priority;
        game.apply(player, wanted)
            .expect("the default split is legal");
    }
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

/// Passes priority, one player at a time, until the stack empties or a
/// decision interrupts. Resolving a trigger that asks a question stops the
/// round mid-way, which `pass_priority_pair` cannot express.
pub(super) fn pass_until_decision(game: &mut Game) {
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() || game.stack.is_empty() {
            return;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
}

/// Runs the game forward -- passing priority, answering any decision that is
/// not the one being waited for -- until `prompt` is asked. Triggers that go
/// on the stack can put an ordering choice in front of the interesting one.
fn advance_to_prompt(game: &mut Game, player: PlayerId, prompt: &str) -> DecisionObservation {
    for _ in 0..24 {
        if let Some(decision) = game.observe(player).decision {
            if decision.prompt == prompt {
                return decision;
            }
            choose_all_offered(game, player);
            continue;
        }
        if let Some(other) = game
            .decision_player()
            .filter(|other| *other != player && game.observe(*other).decision.is_some())
        {
            choose_all_offered(game, other);
            continue;
        }
        let holder = game.priority;
        game.apply(holder, Action::PassPriority).unwrap();
    }
    panic!("{prompt} was never asked");
}

fn choose_all_offered(game: &mut Game, player: PlayerId) {
    let decision = game
        .observe(player)
        .decision
        .expect("a decision is waiting");
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

mod ability_resolution;
mod activation_costs_and_turns;
mod activation_prohibitions;
mod activation_timing;
mod additional_costs;
mod alternative_costs;
mod arabian_lands;
mod armageddon_clock;
mod aspect_of_wolf;
mod assigns_no_combat_damage;
mod attachment_targets;
mod attack_deterrents;
mod attack_restrictions;
mod attacked_last_turn;
mod attacking_walls;
mod aura_control;
mod aura_death;
mod aura_tap_triggers;
mod aura_upkeep;
mod avr_stale_audits;
mod banding;
mod banding_assignment;
mod banding_blocked;
mod banding_formation;
mod banding_with_other;
mod blaze_of_glory;
mod blessing_and_flame;
mod blink_under_your_control;
mod block_restriction_cards;
mod blocker_status;
mod blocking_after_death;
mod blocking_prevention;
mod blocking_relation;
mod blocking_relationship;
mod board_conditioned_statics;
mod board_sized_counter;
mod candelabra_of_tawnos;
mod cannot_attack;
mod cannot_block;
mod casting_and_targets;
mod casting_modes;
mod casting_windows;
mod chaos_orb;
mod chosen_colors;
mod cocoon;
mod coin_flips;
mod combat;
mod combat_and_life;
mod combat_constraints;
mod combat_keywords;
mod combat_relation_cards;
mod combat_rescues;
mod conditional_anthems;
mod continuous_and_zones;
mod control_duration;
mod copy_effects;
mod copy_transform;
mod counted_bodies;
mod counted_statics;
mod counter_conditions;
mod countering_and_mana;
mod creature_bond;
mod cyclone;
mod damage_dealers;
mod damage_dealt_by;
mod damage_history;
mod damage_limits;
mod damage_redirection;
mod damaged_by_this_creature;
mod decisions_and_effects;
mod deep_spawn;
mod delayed_triggers;
mod derelor;
mod detain;
mod dgm_stale_audits;
mod discard_cost;
mod disharmony;
mod dka_stale_audits;
mod elder_spawn;
mod energy_tap;
mod entry_replacements;
mod equipment;
mod equipment_cards;
mod equipment_expansion_batch_one;
mod equipment_expansion_batch_three;
mod equipment_expansion_batch_two;
mod erg_raiders;
mod evolve;
mod evolve_scavenge_cards;
mod exalted;
mod exile_source_costs;
mod extra_turns;
mod face_down;
mod fateful_hour;
mod fetch_tapped;
mod filtered_player_prevention;
mod gatecrash_keyrunes;
mod gatecrash_stale_audits;
mod ghouls;
mod giant_shark;
mod gloom;
mod granted_ability_cards;
mod greatest_power;
mod guardian_beast;
mod gyre_sage;
mod held_tapped;
mod howling_mine;
mod identity_and_mana;
mod instill_energy;
mod into_the_wilds;
mod isd_dgm_deck_cards;
mod isd_token_cards;
mod jade_statue;
mod keymaster_rogue;
mod killing_glare;
mod laces;
mod land_and_ability_layers;
mod land_characteristics;
mod landwalk;
mod life_and_death_amounts;
mod living_artifact;
mod looking_at_another_library;
mod m13_more_stale;
mod m13_stale_audits;
mod m14_stale_audits;
mod mana_ability_costs;
mod mana_and_costs;
mod mana_triggers;
mod mana_vault;
mod meekstone;
mod menace;
mod mentor_of_the_meek;
mod mill_until_land;
mod miracle;
mod mishras_war_machine;
mod modal_effects;
mod morbid;
mod morbid_entry;
mod multi_block;
mod must_block;
mod old_school_interactions;
mod old_school_permanents;
mod old_school_spells;
mod old_school_upkeep;
mod one_sided_block_triggers;
mod open_activation;
mod osai_vultures;
mod paralyze;
mod part_water;
mod play_from_hand;
mod poison;
mod populate;
mod populate_cards;
mod power_blocking;
mod power_blocking_restrictions;
mod premodern_bw;
mod premodern_cards;
mod premodern_cycling;
mod premodern_free_spells;
mod premodern_gat;
mod premodern_goblins;
mod premodern_hermit;
mod premodern_hosers;
mod premodern_kicker;
mod premodern_lands;
mod premodern_landstill;
mod premodern_library_selection;
mod premodern_permanents;
mod premodern_pyrokinesis;
mod premodern_replenish;
mod premodern_sligh;
mod premodern_split_and_lock;
mod premodern_stasis;
mod premodern_zone_and_denial;
mod prevention;
mod prevention_modes;
mod primordial_ooze;
mod prohibition_cards;
mod quota_and_aura_upkeep;
mod rabid_wombat;
mod rampage;
mod random_discard;
mod regeneration;
mod reliquary_tower;
mod removal_and_keywords;
mod replacements_and_presentation;
mod resolution_destinations;
mod revealed_hands;
mod ring_of_maruf;
mod rings;
mod rise_from_the_grave;
mod sacrifice_costs;
mod sacrificed_toughness;
mod scavenge;
mod search_and_reveal;
mod second_spell_each_turn;
mod sentinel;
mod shroud_grants;
mod silence_and_sturmgeist;
mod silenced_sources;
mod sized_searches;
mod soulbond;
mod spell_colors;
mod spell_cost_reduction;
mod spore;
mod spore_cloud;
mod stale_followups;
mod stat_counters;
mod state_triggers_and_life;
mod static_animation;
mod static_keyword_predicates;
mod storage_lands;
mod street_spasm;
mod subtype_protection;
mod switched_stats;
mod target_toughness;
mod targeted_answers;
mod targeting_characteristics;
mod thread_safety;
mod token_status_cards;
mod toughness_payouts;
mod transmogrant;
mod traumatize;
mod trigger_event_matchers;
mod triggers_and_stack;
mod triumphs;
mod turn_and_loyalty;
mod tutors_and_fetch_lands;
mod unblocked_attackers;
mod unbounded_targets;
mod unleash;
mod untap_caps;
mod untap_skip_spells;
mod untap_skips;
mod until_end_of_combat;
mod urza_lands;
mod venarian_gold;

mod vintage_cube_abhorrent_oculus;
mod vintage_cube_adeline;
mod vintage_cube_adventure;
mod vintage_cube_agathas_soul_cauldron;
mod vintage_cube_ajani;
mod vintage_cube_amped_raptor;
mod vintage_cube_arrival;
mod vintage_cube_artifacts;
mod vintage_cube_barrowgoyf;
mod vintage_cube_bloodbraid_challenger;
mod vintage_cube_bolass_citadel;
mod vintage_cube_bountiful_landscape;
mod vintage_cube_brainstorm;
mod vintage_cube_breach;
mod vintage_cube_brightglass_gearhulk;
mod vintage_cube_cankerbloom;
mod vintage_cube_caryatid;
mod vintage_cube_cathar_commando;
mod vintage_cube_chandra;
mod vintage_cube_collective_brutality;
mod vintage_cube_colonnade;
mod vintage_cube_creatures;
mod vintage_cube_crucible;
mod vintage_cube_cryptic_command;
mod vintage_cube_cut_down;
mod vintage_cube_dack;
mod vintage_cube_damn;
mod vintage_cube_delayed_blast_fireball;
mod vintage_cube_dreadhorde_arcanist;
mod vintage_cube_duelist;
mod vintage_cube_echo_of_eons;
mod vintage_cube_eldrazi;
mod vintage_cube_elite_spellbinder;
mod vintage_cube_emperor_of_bones;
mod vintage_cube_enduring_innocence;
mod vintage_cube_eternal_witness;
mod vintage_cube_exhume;
mod vintage_cube_expedition_map;
mod vintage_cube_fatal_push;
mod vintage_cube_figure_of_destiny;
mod vintage_cube_force_of_negation;
mod vintage_cube_gadgeteer;
mod vintage_cube_gau;
mod vintage_cube_get_lost;
mod vintage_cube_ghost_vacuum;
mod vintage_cube_goblin_rabblemaster;
mod vintage_cube_graveyard;
mod vintage_cube_gut;
mod vintage_cube_hand_attack;
mod vintage_cube_horizon_land;
mod vintage_cube_infect;
mod vintage_cube_ivora;
mod vintage_cube_jace_the_mind_sculptor;
mod vintage_cube_jacked_rabbit;
mod vintage_cube_jitte;
mod vintage_cube_laelia;
mod vintage_cube_lands;
mod vintage_cube_lavaspur_boots;
mod vintage_cube_ledger_shredder;
mod vintage_cube_legion_extruder;
mod vintage_cube_library;
mod vintage_cube_lion_sash;
mod vintage_cube_lorien;
mod vintage_cube_magda;
mod vintage_cube_mana;
mod vintage_cube_manamorphose;
mod vintage_cube_manifold_key;
mod vintage_cube_mastery;
mod vintage_cube_mine_collapse;
mod vintage_cube_monarch;
mod vintage_cube_more_spells;
mod vintage_cube_mystic_confluence;
mod vintage_cube_necromancy;
mod vintage_cube_nights_whisper;
mod vintage_cube_ninjutsu;
mod vintage_cube_nissa;
mod vintage_cube_occult_epiphany;
mod vintage_cube_ocelot_pride;
mod vintage_cube_omnath;
mod vintage_cube_oracle;
mod vintage_cube_orcish_bowmasters;
mod vintage_cube_overlord;
mod vintage_cube_paradoxical_outcome;
mod vintage_cube_path;
mod vintage_cube_phantasmal_image;
mod vintage_cube_phlage;
mod vintage_cube_ponder;
mod vintage_cube_portable_hole;
mod vintage_cube_prismatic_ending;
mod vintage_cube_psychic_frog;
mod vintage_cube_relic;
mod vintage_cube_saheeli;
mod vintage_cube_shieldbreaker;
mod vintage_cube_spells;
mod vintage_cube_static_prison;
mod vintage_cube_stoneforge;
mod vintage_cube_stormchasers_talent;
mod vintage_cube_subtlety;
mod vintage_cube_sunfall;
mod vintage_cube_surveil_land;
mod vintage_cube_sword_of_the_meek;
mod vintage_cube_tamiyo;
mod vintage_cube_tendrils;
mod vintage_cube_thoughtseize;
mod vintage_cube_time_spiral;
mod vintage_cube_titania;
mod vintage_cube_underworld_breach;
mod vintage_cube_ursine;
mod vintage_cube_verge;
mod vintage_cube_voice_of_victory;
mod vintage_cube_vote;
mod vintage_cube_walk_in_closet;
mod vintage_cube_walking_ballista;
mod vintage_cube_worldspine_wurm;
mod wards;
mod while_source_tapped;
mod word_of_binding;
mod xenic_poltergeist;
mod zone_effects;
mod zone_move_replacements;

use copy_effects::{
    copied_grant_origin, copied_grant_source_game, resolve_copy_artifact, sole_granted_origin,
};
use countering_and_mana::{acceptance_attempt_counterspell, acceptance_cast_action_targeting};
use delayed_triggers::{drain_pending, installing_object};
use modal_effects::cast_mode;
use old_school_spells::game_with_test_fused_split;
use removal_and_keywords::dust_to_dust_targets;
use search_and_reveal::stack_library;
