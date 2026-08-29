//! Shared fixtures for the engine's behaviour tests.
//!
//! Split out of the parent module for the source-size budget: what stays
//! next door is the list of test modules, which grows with every card. The
//! imports are the parent module's, reached the way every other test module
//! reaches them.

use super::*;

pub(in crate::game) static TEST_FLYING_ABILITY: [AbilityDef; 1] = [abilities::flying()];
pub(in crate::game) static TEST_FLYING_TRAMPLE_ABILITIES: [AbilityDef; 2] =
    [abilities::flying(), abilities::trample()];
pub(super) static TEST_MISHRAS_FACTORY_CHARACTERISTICS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(
        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
    ),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Assembly-Worker"])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];
pub(in crate::game) static CARD_COST_FLASHBACK: AbilityDef =
    abilities::flashback_for_card_mana_cost();
pub(in crate::game) const TEST_OPPONENT_LAND_ENTRY_TEXT: &str =
    "Lands your opponents control enter tapped.";
pub(in crate::game) static TEST_OPPONENT_LANDS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
        TEST_OPPONENT_LAND_ENTRY_TEXT,
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::HasType(CardType::Land),
            controller: PlayerRelation::Opponent,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )];
pub(in crate::game) static TEST_EXTERNAL_ENTER_TAPPED: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::ModifyBattlefieldEntry(
        BattlefieldEntryModificationDef::Tapped,
    )];
pub(in crate::game) static TEST_EXTERNAL_PAYMENT: [ReplacementEffectDef; 1] =
    [ReplacementEffectDef::PayOr {
        payment: EffectPaymentDef::life(PlayerSetDef::Related(PlayerRelation::You), 2),
        if_paid: &[],
        if_declined: &TEST_EXTERNAL_ENTER_TAPPED,
    }];
pub(in crate::game) static TEST_EXTERNAL_CONTEXT_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
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
pub(in crate::game) static TEST_GRANTED_ENTRY_REPLACEMENT: AbilityDef =
    abilities::enters_tapped("This permanent enters tapped.");
pub(in crate::game) static TEST_SELF_GRANTED_ENTRY_ABILITY: [AbilityDef; 1] =
    [AbilityDef::static_ability(
        "This permanent has \"This permanent enters tapped.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&TEST_GRANTED_ENTRY_REPLACEMENT),
        },
    )];
pub(in crate::game) static TEST_SELF_PLAINS_ABILITY: [AbilityDef; 1] =
    [AbilityDef::static_ability(
        "This land is a Plains in addition to its other types.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Plains]),
        },
    )];
pub(in crate::game) static TEST_PLAINS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
        "Plains your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
            controller: PlayerRelation::Opponent,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )];
pub(in crate::game) static TEST_OPPONENT_ENCHANTMENTS_ENTER_TAPPED_ABILITY: [AbilityDef; 1] =
    [AbilityDef::replacement_for(
        "Enchantments your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::HasType(CardType::Enchantment),
            controller: PlayerRelation::Opponent,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )];

pub(in crate::game) fn ready_game() -> Game {
    ready_game_with_seed(0)
}

/// The same board with a chosen seed, for the effects that consult the
/// replay-stable randomiser.
pub(in crate::game) fn ready_game_with_seed(seed: u64) -> Game {
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
pub(in crate::game) fn checkpoint_fixture(
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
            .map(|card| card.definition.get())
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

pub(in crate::game) fn card(
    id: u32,
    definition: CardDefinitionId,
    owner: PlayerId,
) -> CardInstance {
    CardInstance {
        id: CardInstanceId(id),
        definition,
        owner,
        backing: ObjectBacking::Cards(vec![PhysicalCardId(id)]),
        characteristics: CharacteristicSource::Card(definition),
        counters: crate::game::counters::Counters::new(),
    }
}

pub(in crate::game) fn creature(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
) -> Permanent {
    Permanent::entering(
        card(id, definition, controller),
        CardPartId::PRIMARY,
        controller,
        0,
        0,
    )
}

pub(in crate::game) fn token_permanent(
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
        counters: crate::game::counters::Counters::new(),
    };
    Permanent::entering_token(object, token, controller, 0, 0)
}

pub(in crate::game) fn is_token_with(permanent: &Permanent, token: TokenCharacteristics) -> bool {
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
pub(in crate::game) fn attach_constant_resolved_characteristics(
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
                AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(_),
            ) => panic!("a linked-exile grant is a static shape, not a resolved one"),
            CharacteristicOperationDef::ChosenBasicLandType => {
                panic!("a chosen land type is read live rather than resolved")
            }
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
                    PowerToughnessOperationDef::Define { .. } => {
                        panic!("a characteristic-defining ability has no resolved form")
                    }
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

pub(in crate::game) fn copied_characteristics(
    definition: CardDefinitionId,
) -> CopiableCharacteristics {
    CopiableCharacteristics {
        base: ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
        added_types: CardTypeSet::empty(),
        added_abilities: Vec::new(),
        retain_printed_subtypes: false,
        base_power_toughness: None,
        colors: None,
        added_creature_types: Vec::new(),
        no_mana_cost: false,
    }
}

pub(in crate::game) fn cast_choices(targets: Vec<Target>, x: u16) -> CastChoices {
    let choices = CastChoices::default().with_x(x);
    if targets.is_empty() {
        choices
    } else {
        choices.with_targets(vec![TargetSelection::new(TargetSlotId(0), targets)])
    }
}

pub(in crate::game) fn cast_action(
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

pub(in crate::game) fn resolve_channel(game: &mut Game) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    let channel = card(99_188, cards::CHANNEL, PlayerId::One);
    let channel_id = channel.id;
    game.players[PlayerId::One.index()].hand.push(channel);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == channel_id))
        .expect("Channel can be cast with two green mana");
    game.apply(PlayerId::One, cast)
        .expect("Channel cast applies");
    pass_priority_pair(game);
    assert!(
        !game
            .ongoing_mana_ability_activations(PlayerId::One)
            .is_empty(),
        "resolving Channel creates its declarative ongoing mana ability",
    );
}

pub(in crate::game) fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

pub(in crate::game) const fn primary_ability(definition: CardDefinitionId) -> AbilityOrigin {
    AbilityOrigin::Printed {
        definition,
        part: CardPartId::PRIMARY,
        ability: crate::AbilityId::PRIMARY,
    }
}

pub(in crate::game) fn mana_ability_for(
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
pub(in crate::game) fn plain_activation(source: GameObjectId, ability: AbilityOrigin) -> Action {
    Action::ActivateAbility {
        source,
        ability,
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    }
}

pub(in crate::game) fn synchronize_single_part_definition(definition: &mut CardDefinition) {
    let composition = CardComposition::single(definition.name.clone(), definition.rules);
    definition.parts = composition.parts;
    definition.structure = composition.structure;
    definition.play_options = composition.play_options;
}

/// Reads a creature card's characteristic-defined body from each ordinary
/// card or spell zone the engine currently stores.
pub(in crate::game) fn card_stats_in_zone(
    mut game: Game,
    id: u32,
    definition: CardDefinitionId,
    owner: PlayerId,
    zone: ZoneKind,
) -> (i16, i16) {
    let object = GameObjectId(id);
    match zone {
        ZoneKind::Library => game.players[owner.index()]
            .library
            .push(card(id, definition, owner)),
        ZoneKind::Hand => game.players[owner.index()]
            .hand
            .push(card(id, definition, owner)),
        ZoneKind::Battlefield => {
            game.battlefield.push(creature(id, definition, owner));
        }
        ZoneKind::Graveyard => game.players[owner.index()]
            .graveyard
            .push(card(id, definition, owner)),
        ZoneKind::Stack => game.stack.push(spell(id, definition, owner, 0)),
        ZoneKind::Exile => game.players[owner.index()]
            .exile
            .push(card(id, definition, owner)),
        ZoneKind::Command => unreachable!("ordinary cards are not stored in command"),
    }
    if zone == ZoneKind::Stack {
        let characteristics = game
            .stack_trigger_event_object(game.stack.last().expect("the spell is on the stack"))
            .expect("the object is a creature spell");
        (
            characteristics.power.expect("the creature has power"),
            characteristics
                .toughness
                .expect("the creature has toughness"),
        )
    } else {
        (
            game.current_or_last_known_power(object)
                .expect("the creature has power"),
            game.current_or_last_known_toughness(object)
                .expect("the creature has toughness"),
        )
    }
}

pub(in crate::game) fn spell(
    id: u32,
    definition: CardDefinitionId,
    controller: PlayerId,
    x: u16,
) -> StackObject {
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
        cast_via_suspend: false,
        cast_at_instant_speed: false,
        cast_from_zone: None,
        face_down: None,
        colors_of_mana_spent: ColorSet::empty(),
        phyrexian_symbols_paid_with_life: 0,
        is_copy: false,
    }
}

pub(in crate::game) fn spell_with_targets(
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
pub(in crate::game) fn take_default_combat_assignment(game: &mut Game) {
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

pub(in crate::game) fn pass_priority_pair(game: &mut Game) {
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

/// Passes priority, one player at a time, until the stack empties or a
/// decision interrupts. Resolving a trigger that asks a question stops the
/// round mid-way, which `pass_priority_pair` cannot express.
pub(in crate::game) fn pass_until_decision(game: &mut Game) {
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
pub(in crate::game) fn advance_to_prompt(
    game: &mut Game,
    player: PlayerId,
    prompt: &str,
) -> DecisionObservation {
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

pub(in crate::game) fn choose_all_offered(game: &mut Game, player: PlayerId) {
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

pub(in crate::game) fn choose_decision_by_label(game: &mut Game, player: PlayerId, label: &str) {
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
