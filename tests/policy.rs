use penta::card::{self, cards};
use penta::game::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, PermanentObservation, StackObservation,
};
use penta::poc;
use penta::{
    AbilityId, AbilityOrigin, Action, AlternativeCostId, AttackDefender, BasicLandType,
    CardInstanceId, CardPartId, CastChoices, CastSignature, CostConfiguration, Game, GameResult,
    HandcraftedPolicy, ManaPool, PlayOptionId, PlayerId, PlayerObservation, Policy, RandomPolicy,
    SpellForm, StackObjectKind, Step, Target, TargetSelection, TargetSlotId, play_game,
};

const ACTION_LIMIT: usize = 50_000;
const PRIMARY_PRINTED_ABILITY: AbilityOrigin = AbilityOrigin::Printed {
    definition: penta::CardDefinitionId(0),
    part: CardPartId::PRIMARY,
    ability: AbilityId::PRIMARY,
};

fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

const fn printed_ability(definition: penta::CardDefinitionId, ability: u8) -> AbilityOrigin {
    AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: AbilityId(ability),
    }
}

fn policy_observation(
    battlefield: Vec<PermanentObservation>,
    legal_actions: Vec<Action>,
) -> PlayerObservation {
    PlayerObservation {
        viewer: PlayerId::One,
        turn: 3,
        active_turn: 2,
        active_player: PlayerId::One,
        priority: PlayerId::One,
        step: Step::PrecombatMain,
        regular_combat_damage_pending: false,
        life_totals: [20, 20],
        mana_pools: [ManaPool::default(), ManaPool::default()],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [50, 50],
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        battlefield,
        emblems: Vec::new(),
        stack: Vec::new(),
        decision: None,
        result: None,
        legal_actions,
        checkpoint: serde_json::json!({}),
    }
}

fn permanent(
    id: u32,
    definition: penta::CardDefinitionId,
    controller: PlayerId,
    power: Option<i16>,
    toughness: Option<i16>,
) -> PermanentObservation {
    PermanentObservation {
        id: CardInstanceId(id),
        definition,
        presented: CardPartId::PRIMARY,
        controller,
        attached_to: None,
        types: penta::CardTypeSet::empty(),
        chosen_creature_type: None,
        chosen_card_name: None,
        tapped: false,
        power,
        toughness,
        damage: 0,
        loyalty: None,
        loyalty_ability_used_this_turn: false,
        attack_defender: None,
        attacking: false,
        blocked_this_combat: false,
        blocking: None,
        flying: false,
        can_attack: false,
        entered_this_turn: false,
    }
}

fn stack_object(
    id: u32,
    definition: penta::CardDefinitionId,
    controller: PlayerId,
    kind: StackObjectKind,
    targets: Vec<Target>,
) -> StackObservation {
    StackObservation {
        id: CardInstanceId(id),
        kind,
        source: None,
        ability: None,
        ability_text: None,
        definition,
        controller,
        counterable: true,
        signature: (kind == StackObjectKind::Spell).then(|| {
            CastSignature::from_validated_choices(
                SpellForm::Part(CardPartId::PRIMARY),
                CastChoices::default(),
            )
        }),
        targets,
        chosen_permanents: Vec::new(),
        x: 0,
    }
}

const BLOODRUSH: AbilityOrigin = AbilityOrigin::Printed {
    definition: cards::GHOR_CLAN_RAMPAGER,
    part: CardPartId::PRIMARY,
    ability: AbilityId(1),
};

fn bloodrush_action(source: CardInstanceId, target: CardInstanceId) -> Action {
    Action::ActivateAbility {
        source,
        ability: BLOODRUSH,
        targets: activated_targets(Target::Permanent(target)),
        cost_object: None,
        x: 0,
    }
}

#[path = "policy/abilities.rs"]
mod abilities;
#[path = "policy/combat.rs"]
mod combat;
#[path = "policy/decisions.rs"]
mod decisions;
#[path = "policy/general.rs"]
mod general;
#[path = "policy/random.rs"]
mod random;
#[path = "policy/simulation.rs"]
mod simulation;
#[path = "policy/targeting.rs"]
mod targeting;
