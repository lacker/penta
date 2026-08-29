use penta::card::{self, cards};
use penta::game::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, PermanentObservation, StackObservation,
};
use penta::poc;
use penta::{
    AbilityId, AbilityOrigin, Action, AlternativeCostId, AttackDefender, BasicLandType,
    CardInstanceId, CardPartId, CastChoices, CastSignature, CostConfiguration, Game, GameResult,
    HandcraftedPolicy, ManaPool, ObjectCharacteristics, PlayOptionId, PlayerId, PlayerObservation,
    Policy, RandomPolicy, SpellForm, StackObjectKind, Step, Target, TargetSelection, TargetSlotId,
    play_game,
};

const ACTION_LIMIT: usize = 50_000;
const PRIMARY_PRINTED_ABILITY: AbilityOrigin = AbilityOrigin::Printed {
    definition: penta::CardDefinitionId::new(1),
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

const fn printed(definition: penta::CardDefinitionId) -> ObjectCharacteristics {
    ObjectCharacteristics::card(definition, CardPartId::PRIMARY)
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
        poison_counters: [0, 0],
        energy_counters: [0, 0],
        counters: [Vec::new(), Vec::new()],
        monarch: None,
        life_totals: [20, 20],
        mana_pools: [ManaPool::default(), ManaPool::default()],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [50, 50],
        revealed_library_top: None,
        opponent_revealed_library_top: None,
        companions: Vec::new(),
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        face_down_exile_sizes: [0, 0],
        card_counters: Vec::new(),
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
        characteristics: ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
        token: false,
        has_individual_state: false,
        controller,
        face_down: false,
        physical_face: None,
        phased_out: false,
        types: penta::CardTypeSet::empty(),
        chosen_creature_type: None,
        chosen_basic_land_type: None,
        chosen_color: None,
        chosen_card_name: None,
        tapped: false,
        power,
        toughness,
        damage: 0,
        counters: Vec::new(),
        loyalty: None,
        loyalty_ability_used_this_turn: false,
        attack_defender: None,
        attacking: false,
        blocked_this_combat: false,
        blocking: Vec::new(),
        blocking_this_combat: false,
        attacking_band: None,
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
        characteristics: ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
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
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
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

fn standing_cast_offer(
    definition: penta::CardDefinitionId,
    zone: DecisionZone,
    cast: Action,
) -> PlayerObservation {
    let card = CardInstanceId(90_000);
    let mut observation = policy_observation(
        Vec::new(),
        vec![
            Action::Concede,
            Action::ChooseDecision {
                decision: 90,
                options: Vec::new(),
            },
            cast,
        ],
    );
    match zone {
        DecisionZone::Hand => observation.hand.push((card, definition)),
        DecisionZone::Exile => observation.exiles[PlayerId::One.index()].push((card, definition)),
        _ => panic!("the policy fixture only stages cast offers from hand or exile"),
    }
    observation.decision = Some(DecisionObservation {
        id: 90,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        source: None,
        prompt: "Cast now, or decline".to_owned(),
        visibility: DecisionVisibility::Public,
        preference: DecisionPreference::PreferOption(0),
        minimum: 1,
        maximum: 1,
        cancellable: false,
        options: vec![DecisionOption {
            id: 0,
            label: "Decline".to_owned(),
            card: Some((card, printed(definition))),
            members: Vec::new(),
            ability_text: None,
            zone,
        }],
    });
    observation
}

#[test]
fn random_policy_treats_a_standing_cast_as_an_alternative_to_declining() {
    let card = CardInstanceId(90_000);
    let cast = Action::CastSpell {
        card,
        choices: CastChoices::default(),
        sacrifices: Vec::new(),
    };
    let observation = standing_cast_offer(cards::LIGHTNING_BOLT, DecisionZone::Exile, cast.clone());
    let mut policy = RandomPolicy::new(9_001);
    let mut cast_seen = false;
    let mut decline_seen = false;

    for _ in 0..64 {
        match policy.choose_action(&observation) {
            Some(action) if action == cast => cast_seen = true,
            Some(Action::ChooseDecision { decision, options }) => {
                assert_eq!(decision, 90);
                assert_eq!(options, vec![0]);
                decline_seen = true;
            }
            other => panic!("standing offer produced an unrelated action: {other:?}"),
        }
    }

    assert!(cast_seen, "the cast is part of the random choice set");
    assert!(
        decline_seen,
        "declining remains part of the random choice set"
    );
}

#[test]
fn handcrafted_policy_accepts_a_useful_standing_cast_offer() {
    let catalog = poc::catalog().unwrap();
    let card = CardInstanceId(90_000);
    let cast = Action::CastSpell {
        card,
        choices: CastChoices::default(),
        sacrifices: Vec::new(),
    };
    let observation = standing_cast_offer(cards::LIGHTNING_BOLT, DecisionZone::Exile, cast.clone());
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(cast));
}

#[test]
fn handcrafted_policy_still_declines_an_unhelpful_standing_cast() {
    let catalog = poc::catalog().unwrap();
    let card = CardInstanceId(90_000);
    let cast = Action::CastSpell {
        card,
        choices: CastChoices::default(),
        sacrifices: Vec::new(),
    };
    let observation = standing_cast_offer(cards::FIREBALL, DecisionZone::Hand, cast);
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ChooseDecision {
            decision: 90,
            options: vec![0],
        }),
        "an X spell offered with X of zero is worse than declining",
    );
}

#[test]
fn handcrafted_policy_takes_a_preferred_optional_action_when_present() {
    let mut observation = policy_observation(
        Vec::new(),
        vec![Action::ChooseDecision {
            decision: 91,
            options: Vec::new(),
        }],
    );
    observation.decision = Some(DecisionObservation {
        id: 91,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        source: None,
        prompt: "Take an action while drawing".to_owned(),
        visibility: DecisionVisibility::Private,
        preference: DecisionPreference::PreferOption(1),
        minimum: 0,
        maximum: 1,
        cancellable: false,
        options: vec![DecisionOption {
            id: 1,
            label: "Reveal".to_owned(),
            card: Some((CardInstanceId(90_001), printed(cards::TERMINUS))),
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::Hand,
        }],
    });
    let mut policy = HandcraftedPolicy::new(poc::catalog().unwrap());

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ChooseDecision {
            decision: 91,
            options: vec![1],
        })
    );
}
