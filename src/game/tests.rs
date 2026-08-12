use super::*;
use crate::card::abilities;
use crate::mana_cost;
use crate::poc::{self, cards};
use crate::{
    AbilityCoverageDef, AbilityTargetDef, AbilityTargetPredicate, AdditionalCostDef,
    AdditionalCostId, AlternativeCastManaCostDef, AlternativeCostDef, AlternativeCostId,
    BattlefieldEntryModificationDef, CardComposition, CardDefinition, CardEffectStatus,
    CardInstanceId, CardPart, CardPartId, CardPrinting, CardRules, CardStructure, CastChoices,
    DoubleFacedKind, EffectExecutionDef, ManaSpendEffectDef, ModeDef, ModeSetDef, PlayOptionDef,
    PlayOptionId, PlayerRelation, ReplacementEffectDef, ReplacementEventDef, SpellForm,
    StackObjectId, TargetIndex, TargetPredicate, TargetSelection, TargetSlotDef, TargetSlotId,
    ZonePlacement,
};

static TEST_FLYING_ABILITY: [AbilityDef; 1] = [abilities::flying()];
static TEST_FLYING_TRAMPLE_ABILITIES: [AbilityDef; 2] = [abilities::flying(), abilities::trample()];
static CARD_COST_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();
const TEST_OPPONENT_LAND_ENTRY_TEXT: &str = "Lands your opponents control enter tapped.";
static TEST_OPPONENT_LANDS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    TEST_OPPONENT_LAND_ENTRY_TEXT,
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasType(CardType::Land),
        controller: PlayerRelation::Opponent,
    },
    EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )),
)];
static TEST_EXTERNAL_PAYMENT_COST: [CostDef; 1] = [CostDef::PayLife(2)];
static TEST_EXTERNAL_ENTER_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];
static TEST_EXTERNAL_PAYMENT: [ReplacementEffectDef; 1] = [ReplacementEffectDef::OptionalPayment {
    payment: PaymentDef::new(PlayerRelation::You, &TEST_EXTERNAL_PAYMENT_COST),
    if_paid: &[],
    if_declined: &TEST_EXTERNAL_ENTER_TAPPED,
}];
static TEST_EXTERNAL_CONTEXT_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    "Lands your opponents control enter tapped unless you control a Plains and pay 2 life.",
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasType(CardType::Land),
        controller: PlayerRelation::Opponent,
    },
    EffectDef::Replacement(ReplacementEffectDef::Conditional {
        condition: ConditionDef::Exists(ObjectQueryDef {
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
            zones: &[ZoneKind::Battlefield],
            controller: PlayerRelation::You,
        }),
        if_true: &TEST_EXTERNAL_PAYMENT,
        if_false: &TEST_EXTERNAL_ENTER_TAPPED,
    }),
)];
static TEST_GRANTED_ENTRY_REPLACEMENT: AbilityDef =
    abilities::enters_tapped("This permanent enters tapped.");
static TEST_SELF_GRANTED_ENTRY_ABILITY: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This permanent has \"This permanent enters tapped.\"",
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::GrantAbility(&TEST_GRANTED_ENTRY_REPLACEMENT),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    },
)];
static TEST_SELF_PLAINS_ABILITY: [AbilityDef; 1] = [AbilityDef::static_ability(
    "This land is a Plains in addition to its other types.",
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::AddLandTypes(&[BasicLandType::Plains]),
        duration: EffectDurationDef::WhileSourceRemainsInZone,
    },
)];
static TEST_PLAINS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] = [AbilityDef::replacement_for(
    "Plains your opponents control enter tapped.",
    ReplacementEventDef::ObjectEntersBattlefield {
        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        controller: PlayerRelation::Opponent,
    },
    EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )),
)];
static TEST_OPPONENT_ENCHANTMENTS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
        "Enchantments your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::HasType(CardType::Enchantment),
            controller: PlayerRelation::Opponent,
        },
        EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::Tapped,
        )),
    )];

pub(super) fn ready_game() -> Game {
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

pub(super) fn creature(id: u32, definition: CardDefinitionId, controller: PlayerId) -> Permanent {
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
        colors: None,
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
            .filter(|permanent| permanent.blocking == Some(*attacker))
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
fn pass_until_decision(game: &mut Game) {
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
mod alternative_costs;
mod casting_and_targets;
mod casting_modes;
mod chaos_orb;
mod combat;
mod combat_and_life;
mod combat_keywords;
mod continuous_and_zones;
mod copy_effects;
mod countering_and_mana;
mod decisions_and_effects;
mod delayed_triggers;
mod entry_replacements;
mod guardian_beast;
mod identity_and_mana;
mod land_and_ability_layers;
mod land_characteristics;
mod mana_and_costs;
mod mana_vault;
mod modal_effects;
mod old_school_interactions;
mod old_school_permanents;
mod old_school_spells;
mod old_school_upkeep;
mod premodern_cards;
mod premodern_lands;
mod premodern_library_selection;
mod premodern_permanents;
mod premodern_split_and_lock;
mod premodern_zone_and_denial;
mod removal_and_keywords;
mod replacements_and_presentation;
mod search_and_reveal;
mod state_triggers_and_life;
mod targeted_answers;
mod triggers_and_stack;
mod turn_and_loyalty;
mod zone_effects;

use copy_effects::{
    copied_grant_origin, copied_grant_source_game, resolve_copy_artifact, sole_granted_origin,
};
use countering_and_mana::{acceptance_attempt_counterspell, acceptance_cast_action_targeting};
use delayed_triggers::drain_pending;
use modal_effects::cast_mode;
use old_school_spells::game_with_test_fused_split;
use removal_and_keywords::dust_to_dust_targets;
use search_and_reveal::stack_library;
