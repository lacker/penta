use super::*;

static ATTACHMENT_CREATURE_ANIMATION: crate::card::AnimationDef =
    crate::card::AnimationDef::new(1, 1);
static ATTACHMENT_LAND_CREATURE_ANIMATION: crate::card::AnimationDef =
    crate::card::AnimationDef::new(1, 1)
        .with_types(CardTypeSet::single(CardType::Creature).with(CardType::Land));
static TEST_ENCHANT_CREATURE: AbilityDef =
    abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET);

fn fund(game: &mut Game, player: PlayerId, color: ManaColor, amount: u16) {
    game.add_unrestricted_mana(player, color, amount);
}

fn hand_card(
    game: &mut Game,
    id: u32,
    definition: CardDefinitionId,
    player: PlayerId,
) -> GameObjectId {
    let card = card(id, definition, player);
    let id = card.id;
    game.players[player.index()].hand.push(card);
    id
}

fn battlefield_id(game: &Game, definition: CardDefinitionId) -> GameObjectId {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .unwrap_or_else(|| panic!("{definition:?} is not on the battlefield"))
        .card
        .id
}

fn battlefield_permanent(game: &Game, object: GameObjectId) -> Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .unwrap_or_else(|| panic!("{object:?} is not on the battlefield"))
        .clone()
}

fn matching_cast(
    game: &Game,
    player: PlayerId,
    card: GameObjectId,
    alternative: bool,
    target: Option<Target>,
) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } => {
                *candidate == card
                    && choices.costs().alternative().is_some() == alternative
                    && match target {
                        Some(target) => choices.iter_targets().copied().eq([target]),
                        None => choices.iter_targets().next().is_none(),
                    }
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("the requested cast action for {card:?} is not legal"))
}

fn matching_activation(
    game: &Game,
    player: PlayerId,
    source: GameObjectId,
    target: Option<Target>,
) -> Option<Action> {
    game.legal_actions(player)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: candidate,
                targets,
                ..
            } => {
                *candidate == source
                    && match target {
                        Some(target) => targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .copied()
                            .eq([target]),
                        None => targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .next()
                            .is_none(),
                    }
            }
            _ => false,
        })
}

fn choose_object(game: &mut Game, player: PlayerId, object: GameObjectId) {
    let decision = game
        .observe(player)
        .decision
        .expect("a public object choice is pending");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(candidate, _)| candidate == object)
        })
        .unwrap_or_else(|| panic!("the decision does not offer {object:?}"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the selected object is a legal decision response");
}

fn setup_permanent(
    game: &mut Game,
    player: PlayerId,
    definition: CardDefinitionId,
) -> GameObjectId {
    let id = game
        .put_onto_battlefield(player, definition)
        .unwrap_or_else(|_| panic!("{definition:?} is cataloged"));
    game.finish_rules_procedure();
    id
}

fn remove_all_abilities(game: &mut Game, target: GameObjectId, stack_id: u32) {
    let object = spell_with_targets(
        stack_id,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(target)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Any),
            duration: EffectDurationDef::UntilEndOfTurn,
        }),
        &object,
        TriggerContext::empty(),
    );
}

#[test]
fn attach_no_ops_reattaches_with_a_timestamp_and_distinguishes_sbas() {
    let mut game = ready_game();
    let garrison = GameObjectId(20_000);
    let first_land = GameObjectId(20_001);
    let second_land = GameObjectId(20_002);
    let creature = GameObjectId(20_003);
    game.battlefield.extend([
        super::creature(garrison.0, cards::DARKSTEEL_GARRISON, PlayerId::One),
        super::creature(first_land.0, cards::MOUNTAIN, PlayerId::One),
        super::creature(second_land.0, cards::FOREST, PlayerId::One),
        super::creature(creature.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);

    let original_timestamp = battlefield_permanent(&game, garrison).timestamp;
    assert!(game.try_attach(garrison, first_land));
    let first_timestamp = battlefield_permanent(&game, garrison).timestamp;
    assert_ne!(first_timestamp, original_timestamp);

    assert!(!game.try_attach(garrison, first_land));
    assert_eq!(
        battlefield_permanent(&game, garrison).timestamp,
        first_timestamp,
        "attaching to the current host is a strict no-op",
    );
    assert!(!game.try_attach(garrison, creature));
    let after_illegal = battlefield_permanent(&game, garrison);
    assert_eq!(after_illegal.attached_to, Some(first_land));
    assert_eq!(after_illegal.timestamp, first_timestamp);

    assert!(game.try_attach(garrison, second_land));
    let moved = battlefield_permanent(&game, garrison);
    assert_eq!(moved.attached_to, Some(second_land));
    assert_ne!(moved.timestamp, first_timestamp);

    game.return_permanent_to_hand(second_land);
    game.check_state_based_actions();
    let garrison = battlefield_permanent(&game, garrison);
    assert_eq!(garrison.attached_to, None);

    let aura = GameObjectId(20_010);
    let aura_land = GameObjectId(20_011);
    game.battlefield.extend([
        super::creature(aura.0, cards::NYLEAS_PRESENCE, PlayerId::One),
        super::creature(aura_land.0, cards::MOUNTAIN, PlayerId::One),
    ]);
    assert!(game.try_attach(aura, aura_land));
    game.return_permanent_to_hand(aura_land);
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura),
        "an illegal Aura dies while a Fortification merely becomes unattached",
    );
}

#[test]
fn legacy_attach_effect_aura_remains_legally_attached() {
    let mut game = ready_game();
    let aura = GameObjectId(20_012);
    let land = GameObjectId(20_013);
    game.battlefield.extend([
        super::creature(aura.0, cards::NYLEAS_PRESENCE, PlayerId::One),
        super::creature(land.0, cards::MOUNTAIN, PlayerId::One),
    ]);

    assert!(game.try_attach(aura, land));
    game.check_state_based_actions();
    assert_eq!(battlefield_permanent(&game, aura).attached_to, Some(land));
}

#[test]
fn every_effective_enchant_restriction_applies_and_zero_is_illegal() {
    let mut game = ready_game();
    let aura = GameObjectId(20_014);
    let land = GameObjectId(20_015);
    let animated_land = GameObjectId(20_016);
    let mut aura_permanent = super::creature(aura.0, cards::NYLEAS_PRESENCE, PlayerId::One);
    aura_permanent
        .temporary_granted_abilities
        .push(TemporaryGrantedAbility {
            ability: TEST_ENCHANT_CREATURE,
            source: aura,
            source_definition: cards::NYLEAS_PRESENCE,
            source_part: CardPartId::PRIMARY,
            source_ability: AbilityId::PRIMARY,
            grant: GrantId::PRIMARY,
            timestamp: ContinuousEffectTimestamp(10),
            order: 0,
            expiration: AbilityEffectExpiration::Never,
        });
    let mut animated = super::creature(animated_land.0, cards::MOUNTAIN, PlayerId::One);
    animated.animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_CREATURE_ANIMATION,
        timestamp: ContinuousEffectTimestamp(11),
    });
    game.battlefield.extend([
        aura_permanent,
        super::creature(land.0, cards::MOUNTAIN, PlayerId::One),
        animated,
    ]);

    assert!(
        !game.try_attach(aura, land),
        "enchant land and enchant creature must both apply",
    );
    assert!(game.try_attach(aura, animated_land));

    remove_all_abilities(&mut game, aura, 20_017);
    assert!(!game.is_legal_attachment_host(
        &battlefield_permanent(&game, aura),
        animated_land,
        false,
    ));
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura),
        "an Aura with zero enchant abilities is put into its owner's graveyard",
    );
}

#[test]
fn attached_noncreature_detaches_after_losing_its_attachment_subtype() {
    let mut game = ready_game();
    let garrison = GameObjectId(20_014);
    let land = GameObjectId(20_015);
    game.battlefield.extend([
        super::creature(garrison.0, cards::DARKSTEEL_GARRISON, PlayerId::One),
        super::creature(land.0, cards::MOUNTAIN, PlayerId::One),
    ]);
    assert!(game.try_attach(garrison, land));

    let copied = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == garrison)
        .expect("Garrison is present");
    copied.copy_effect = Some(copied_characteristics(cards::JAYEMDAE_TOME));
    let copied = battlefield_permanent(&game, garrison);
    assert!(
        !game
            .permanent_types(&copied)
            .expect("copied permanent has types")
            .contains(CardType::Creature),
    );
    assert_eq!(game.attachment_kind(&copied), None);

    game.check_state_based_actions();
    let detached = battlefield_permanent(&game, garrison);
    assert_eq!(detached.attached_to, None);
    assert_eq!(detached.card.definition, cards::DARKSTEEL_GARRISON);
}

#[test]
fn animated_equipment_and_fortification_can_target_themselves_but_do_not_attach() {
    let mut equipment_game = ready_game();
    let equipment = GameObjectId(20_016);
    let mut animated_equipment =
        super::creature(equipment.0, cards::COLOSSAL_DREADMASK, PlayerId::One);
    animated_equipment.animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_CREATURE_ANIMATION,
        timestamp: animated_equipment.timestamp,
    });
    equipment_game.battlefield.push(animated_equipment);
    fund(&mut equipment_game, PlayerId::One, ManaColor::Green, 5);
    let equip_self = matching_activation(
        &equipment_game,
        PlayerId::One,
        equipment,
        Some(Target::Permanent(equipment)),
    )
    .expect("an animated Equipment is a legal equip target");
    equipment_game.apply(PlayerId::One, equip_self).unwrap();
    pass_priority_pair(&mut equipment_game);
    assert_eq!(
        battlefield_permanent(&equipment_game, equipment).attached_to,
        None,
    );

    let mut fortification_game = ready_game();
    let fortification = GameObjectId(20_017);
    let mut animated_fortification =
        super::creature(fortification.0, cards::DARKSTEEL_GARRISON, PlayerId::One);
    animated_fortification.animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_LAND_CREATURE_ANIMATION,
        timestamp: animated_fortification.timestamp,
    });
    fortification_game.battlefield.push(animated_fortification);
    fund(
        &mut fortification_game,
        PlayerId::One,
        ManaColor::Colorless,
        3,
    );
    let fortify_self = matching_activation(
        &fortification_game,
        PlayerId::One,
        fortification,
        Some(Target::Permanent(fortification)),
    )
    .expect("an animated land Fortification is a legal fortify target");
    fortification_game
        .apply(PlayerId::One, fortify_self)
        .unwrap();
    pass_priority_pair(&mut fortification_game);
    assert_eq!(
        battlefield_permanent(&fortification_game, fortification).attached_to,
        None,
    );
}

#[test]
fn nimbus_naiad_cast_normally_is_an_unattached_enchantment_creature() {
    let mut game = ready_game();
    let naiad = hand_card(&mut game, 20_020, cards::NIMBUS_NAIAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 3);

    let cast = matching_cast(&game, PlayerId::One, naiad, false, None);
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let naiad = battlefield_permanent(&game, battlefield_id(&game, cards::NIMBUS_NAIAD));
    let types = game.permanent_types(&naiad).expect("Naiad has types");
    assert!(types.contains(CardType::Creature));
    assert!(types.contains(CardType::Enchantment));
    assert_eq!(naiad.attachment_form, None);
    assert_eq!(naiad.attached_to, None);
    assert!(game.has_flying(&naiad));
}

#[test]
fn legal_bestow_is_an_aura_and_falls_off_as_the_same_creature() {
    let mut game = ready_game();
    let host = GameObjectId(20_030);
    game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let naiad = hand_card(&mut game, 20_031, cards::NIMBUS_NAIAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 5);

    let cast = matching_cast(
        &game,
        PlayerId::One,
        naiad,
        true,
        Some(Target::Permanent(host)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    let stack_object = game.stack.last().expect("bestow is on the stack");
    let stack_types = game
        .stack_spell_types(stack_object)
        .expect("bestow has stack types");
    assert!(stack_types.contains(CardType::Enchantment));
    assert!(!stack_types.contains(CardType::Creature));
    pass_priority_pair(&mut game);

    let naiad_id = battlefield_id(&game, cards::NIMBUS_NAIAD);
    let bestowed = battlefield_permanent(&game, naiad_id);
    assert!(matches!(
        bestowed.attachment_form,
        Some(AttachmentForm::Bestowed { .. })
    ));
    assert_eq!(bestowed.attached_to, Some(host));
    assert!(
        !game
            .permanent_types(&bestowed)
            .expect("bestowed Naiad has types")
            .contains(CardType::Creature),
    );
    let enchanted = battlefield_permanent(&game, host);
    assert_eq!(game.power(&enchanted), Some(4));
    assert_eq!(game.toughness(&enchanted), Some(3));
    assert!(game.has_flying(&enchanted));

    game.return_permanent_to_hand(host);
    game.check_state_based_actions();
    let fallen = battlefield_permanent(&game, naiad_id);
    assert_eq!(fallen.attachment_form, None);
    assert_eq!(fallen.attached_to, None);
    assert!(
        game.permanent_types(&fallen)
            .expect("fallen Naiad has types")
            .contains(CardType::Creature),
    );
    assert_eq!(game.power(&fallen), Some(2));
    assert_eq!(game.toughness(&fallen), Some(2));
    assert!(game.has_flying(&fallen));
}

#[test]
fn moving_bestowed_creature_ends_bestow_before_the_new_attachment_sba() {
    let mut game = ready_game();
    let first_host = GameObjectId(20_032);
    let second_host = GameObjectId(20_033);
    game.battlefield.extend([
        super::creature(first_host.0, cards::SAVANNAH_LIONS, PlayerId::One),
        super::creature(second_host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    let naiad = hand_card(&mut game, 20_034, cards::NIMBUS_NAIAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 5);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        naiad,
        true,
        Some(Target::Permanent(first_host)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let naiad = battlefield_id(&game, cards::NIMBUS_NAIAD);
    assert!(game.try_attach(naiad, second_host));
    let moved = battlefield_permanent(&game, naiad);
    assert_eq!(moved.attachment_form, None);
    assert_eq!(moved.attached_to, Some(second_host));
    assert!(
        game.permanent_types(&moved)
            .expect("moved Naiad has types")
            .contains(CardType::Creature),
    );
    assert_eq!(game.attachment_kind(&moved), None);

    game.check_state_based_actions();
    assert_eq!(battlefield_permanent(&game, naiad).attached_to, None);
}

#[test]
fn bestow_suppresses_copied_extra_types_until_a_later_animation() {
    let mut game = ready_game();
    let host = GameObjectId(20_035);
    game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let naiad = hand_card(&mut game, 20_036, cards::NIMBUS_NAIAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 5);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        naiad,
        true,
        Some(Target::Permanent(host)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let naiad = battlefield_id(&game, cards::NIMBUS_NAIAD);
    let mut copied = copied_characteristics(cards::NIMBUS_NAIAD);
    copied.added_types = CardTypeSet::single(CardType::Artifact).with(CardType::Land);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == naiad)
        .expect("bestowed Naiad is present")
        .copy_effect = Some(copied);
    let bestowed = battlefield_permanent(&game, naiad);
    assert!(matches!(
        bestowed.attachment_form,
        Some(AttachmentForm::Bestowed { .. })
    ));
    assert_eq!(
        game.permanent_types(&bestowed),
        Some(CardTypeSet::single(CardType::Enchantment)),
    );
    assert_eq!(game.effective_subtypes(&bestowed).as_ref(), &["Aura"]);
}

#[test]
fn bestow_with_an_illegal_target_resolves_as_a_creature() {
    let mut game = ready_game();
    let host = GameObjectId(20_040);
    game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let naiad = hand_card(&mut game, 20_041, cards::NIMBUS_NAIAD, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 5);

    let cast = matching_cast(
        &game,
        PlayerId::One,
        naiad,
        true,
        Some(Target::Permanent(host)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    game.return_permanent_to_hand(host);
    pass_priority_pair(&mut game);

    let naiad = battlefield_permanent(&game, battlefield_id(&game, cards::NIMBUS_NAIAD));
    assert_eq!(naiad.attachment_form, None);
    assert_eq!(naiad.attached_to, None);
    assert!(
        game.permanent_types(&naiad)
            .expect("resolved Naiad has types")
            .contains(CardType::Creature),
    );
}

#[test]
fn thassas_emissary_draws_for_its_host_and_for_itself_after_falling_off() {
    let mut game = ready_game();
    let host = GameObjectId(20_050);
    game.battlefield.push(super::creature(
        host.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
    ));
    let emissary = hand_card(&mut game, 20_051, cards::THASSAS_EMISSARY, PlayerId::One);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 6);
    let cast = matching_cast(
        &game,
        PlayerId::One,
        emissary,
        true,
        Some(Target::Permanent(host)),
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let library_before = game.players[PlayerId::One.index()].library.len();
    game.deal_combat_damage_to_player(host, PlayerId::Two, 1);
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library_before - 1,
    );

    let emissary = battlefield_id(&game, cards::THASSAS_EMISSARY);
    game.return_permanent_to_hand(host);
    game.check_state_based_actions();
    let fallen = battlefield_permanent(&game, emissary);
    assert_eq!(fallen.attachment_form, None);
    let library_before = game.players[PlayerId::One.index()].library.len();
    game.deal_combat_damage_to_player(emissary, PlayerId::Two, 3);
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library_before - 1,
    );
}

mod equipment;
mod reanimation;
#[test]
fn mortarpods_granted_sacrifice_ability_deals_damage_from_lki() {
    let mut game = ready_game();
    let mortarpod = setup_permanent(&mut game, PlayerId::One, cards::MORTARPOD);
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    let germ = battlefield_id(&game, cards::GERM_TOKEN_0_0_BLACK);
    assert_eq!(
        battlefield_permanent(&game, mortarpod).attached_to,
        Some(germ),
    );
    assert_eq!(game.toughness(&battlefield_permanent(&game, germ)), Some(1));

    let activation = matching_activation(
        &game,
        PlayerId::One,
        germ,
        Some(Target::Player(PlayerId::Two)),
    )
    .expect("Mortarpod grants its equipped creature the sacrifice ability");
    let life_before = game.players[PlayerId::Two.index()].life;
    game.apply(PlayerId::One, activation).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != germ),
        "the Germ is sacrificed while paying the cost",
    );
    assert!(
        game.damage_source_event_object(germ).is_some(),
        "the retired Germ retains the LKI needed to be the damage source",
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].life, life_before - 1,);
}

fn stack_blocked_attack(game: &mut Game, attacker: GameObjectId, blocker: GameObjectId) {
    let attacking = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("attacker is present");
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == blocker)
        .expect("blocker is present")
        .blocking = Some(attacker);
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    game.apply(PlayerId::Two, Action::FinishDeclaringBlockers)
        .expect("the defending player may finish declaring blockers");
    assert_eq!(game.stack.len(), 1, "Simian Sling's trigger is stacked");
}

fn finish_blocked_attack(game: &mut Game, attacker: GameObjectId, blocker: GameObjectId) {
    stack_blocked_attack(game, attacker, blocker);
    pass_priority_pair(game);
}

#[test]
fn simian_sling_uses_the_blocked_object_as_source_and_hits_the_defender() {
    let mut unattached = ready_game();
    let sling = GameObjectId(20_100);
    let blocker = GameObjectId(20_101);
    unattached.battlefield.extend([
        super::creature(sling.0, cards::SIMIAN_SLING, PlayerId::One),
        super::creature(blocker.0, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    let life_before = unattached.players[PlayerId::Two.index()].life;
    finish_blocked_attack(&mut unattached, sling, blocker);
    assert_eq!(
        unattached.players[PlayerId::Two.index()].life,
        life_before - 1,
    );
    assert!(
        battlefield_permanent(&unattached, sling).dealt_damage_to_opponent_this_turn,
        "an unattached Sling is the trigger's damage source",
    );

    let mut equipped = ready_game();
    let sling = GameObjectId(20_110);
    let host = GameObjectId(20_111);
    let blocker = GameObjectId(20_112);
    equipped.battlefield.extend([
        super::creature(sling.0, cards::SIMIAN_SLING, PlayerId::One),
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
        super::creature(blocker.0, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    assert!(equipped.try_attach(sling, host));
    let life_before = equipped.players[PlayerId::Two.index()].life;
    finish_blocked_attack(&mut equipped, host, blocker);
    assert_eq!(
        equipped.players[PlayerId::Two.index()].life,
        life_before - 1,
    );
    assert!(battlefield_permanent(&equipped, host).dealt_damage_to_opponent_this_turn);
    assert!(
        !battlefield_permanent(&equipped, sling).dealt_damage_to_opponent_this_turn,
        "the equipped creature, rather than Sling, is the damage source",
    );

    let mut source_gone = ready_game();
    let sling = GameObjectId(20_113);
    let host = GameObjectId(20_114);
    let blocker = GameObjectId(20_115);
    source_gone.battlefield.extend([
        super::creature(sling.0, cards::SIMIAN_SLING, PlayerId::One),
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
        super::creature(blocker.0, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    assert!(source_gone.try_attach(sling, host));
    let life_before = source_gone.players[PlayerId::Two.index()].life;
    stack_blocked_attack(&mut source_gone, host, blocker);
    source_gone.return_permanent_to_hand(host);
    source_gone.finish_rules_procedure();
    assert!(source_gone.damage_source_event_object(host).is_some());
    pass_priority_pair(&mut source_gone);
    assert_eq!(
        source_gone.players[PlayerId::Two.index()].life,
        life_before - 1,
        "the trigger retains the blocked creature's object ID after it leaves",
    );
}

#[test]
fn quickening_licid_transforms_grants_first_strike_and_ends() {
    let mut game = ready_game();
    let licid = GameObjectId(20_120);
    let host = GameObjectId(20_121);
    let mut licid_permanent = super::creature(licid.0, cards::QUICKENING_LICID, PlayerId::One);
    licid_permanent.entered_controller_turn = 0;
    game.battlefield.extend([
        licid_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::White, 3);
    let activate = matching_activation(&game, PlayerId::One, licid, Some(Target::Permanent(host)))
        .expect("Quickening Licid's transform ability is legal");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let aura = battlefield_permanent(&game, licid);
    assert!(aura.tapped);
    assert_eq!(aura.attachment_form, Some(AttachmentForm::Licid));
    assert_eq!(aura.licid_effects.len(), 1);
    let licid_effect = aura.licid_effects[0];
    assert_eq!(licid_effect.ender, PlayerId::One);
    assert_eq!(
        licid_effect.transform_action,
        AbilityOrigin::Printed {
            definition: cards::QUICKENING_LICID,
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );
    assert_eq!(
        licid_effect.end.declarative_effect(),
        Some(EffectDef::EndAuraEffect),
    );
    assert_eq!(aura.attached_to, Some(host));
    assert_eq!(game.effective_subtypes(&aura).as_ref(), &["Aura"]);
    let types = game.permanent_types(&aura).expect("Licid has types");
    assert!(types.contains(CardType::Enchantment));
    assert!(!types.contains(CardType::Creature));
    assert!(game.permanent_has_executable_keyword(
        &battlefield_permanent(&game, host),
        KeywordAbility::FirstStrike,
    ));

    let end = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::TakeSpecialAction { source, .. } if *source == licid),
        )
        .expect("the stored ender may end the Licid effect");
    game.apply(PlayerId::One, end).unwrap();
    let restored = battlefield_permanent(&game, licid);
    assert_eq!(restored.attachment_form, None);
    assert!(restored.licid_effects.is_empty());
    assert_eq!(restored.attached_to, None);
    assert_eq!(game.effective_subtypes(&restored).as_ref(), &["Licid"]);
    assert!(
        game.permanent_types(&restored)
            .expect("restored Licid has types")
            .contains(CardType::Creature),
    );
    assert!(!game.permanent_has_executable_keyword(
        &battlefield_permanent(&game, host),
        KeywordAbility::FirstStrike,
    ));
}

#[test]
fn a_later_licid_enchant_grant_survives_an_earlier_ability_removal() {
    let mut game = ready_game();
    let licid = GameObjectId(20_124);
    let host = GameObjectId(20_125);
    let mut licid_permanent = super::creature(licid.0, cards::QUICKENING_LICID, PlayerId::One);
    licid_permanent.entered_controller_turn = 0;
    game.battlefield.extend([
        licid_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::White, 2);
    let activate = matching_activation(&game, PlayerId::One, licid, Some(Target::Permanent(host)))
        .expect("Quickening Licid's transform ability is legal");
    game.apply(PlayerId::One, activate).unwrap();

    remove_all_abilities(&mut game, licid, 20_126);
    pass_priority_pair(&mut game);
    let aura = battlefield_permanent(&game, licid);
    assert!(game.is_aura_permanent(&aura));
    assert_eq!(aura.attached_to, Some(host));
    assert!(game.is_legal_attachment_host(&aura, host, false));

    remove_all_abilities(&mut game, licid, 20_127);
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != licid),
        "a later removal also removes the timestamped enchant grant",
    );
}

#[test]
fn animating_a_licid_aura_makes_it_detach_and_die_to_sbas() {
    let mut game = ready_game();
    let licid = GameObjectId(20_122);
    let host = GameObjectId(20_123);
    let mut licid_permanent = super::creature(licid.0, cards::QUICKENING_LICID, PlayerId::One);
    licid_permanent.entered_controller_turn = 0;
    game.battlefield.extend([
        licid_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::White, 2);
    let activate = matching_activation(&game, PlayerId::One, licid, Some(Target::Permanent(host)))
        .expect("Quickening Licid's transform ability is legal");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == licid)
        .expect("Licid is present")
        .animation = Some(ResolvedAnimation {
        definition: &ATTACHMENT_CREATURE_ANIMATION,
        timestamp: ContinuousEffectTimestamp(99_000),
    });
    let animated = battlefield_permanent(&game, licid);
    let types = game
        .permanent_types(&animated)
        .expect("animated Licid has types");
    assert!(types.contains(CardType::Creature));
    assert!(types.contains(CardType::Enchantment));
    assert!(game.is_aura_permanent(&animated));

    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != licid),
        "the creature Aura first detaches, then the unattached Aura goes to the graveyard",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::QUICKENING_LICID),
    );
}

#[test]
fn dominating_licid_tracks_control_and_keeps_the_original_enders_permission() {
    let mut game = ready_game();
    let licid = GameObjectId(20_130);
    let host = GameObjectId(20_131);
    let mut licid_permanent = super::creature(licid.0, cards::DOMINATING_LICID, PlayerId::One);
    licid_permanent.entered_controller_turn = 0;
    game.battlefield.extend([
        licid_permanent,
        super::creature(host.0, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    fund(&mut game, PlayerId::One, ManaColor::Blue, 4);
    let activate = matching_activation(&game, PlayerId::One, licid, Some(Target::Permanent(host)))
        .expect("Dominating Licid's transform ability is legal");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(battlefield_permanent(&game, host).controller, PlayerId::One);
    let aura = battlefield_permanent(&game, licid);
    assert_eq!(aura.attachment_form, Some(AttachmentForm::Licid));
    assert_eq!(aura.licid_effects.len(), 1);
    let licid_effect = aura.licid_effects[0];
    assert_eq!(licid_effect.ender, PlayerId::One);
    assert_eq!(
        licid_effect.transform_action,
        AbilityOrigin::Printed {
            definition: cards::DOMINATING_LICID,
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );
    assert_eq!(
        licid_effect.end.declarative_effect(),
        Some(EffectDef::EndAuraEffect),
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == licid)
        .expect("Licid is present")
        .controller = PlayerId::Two;
    game.reconcile_attachment_control_for(Some(host));
    assert_eq!(battlefield_permanent(&game, host).controller, PlayerId::Two);

    game.priority = PlayerId::Two;
    assert!(
        game.legal_actions(PlayerId::Two).iter().all(
            |action| !matches!(action, Action::TakeSpecialAction { source, .. } if *source == licid)
        ),
        "changing the Licid's controller does not transfer the end permission",
    );
    game.priority = PlayerId::One;
    game.stack
        .push(spell(20_132, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    let stack_len = game.stack.len();
    let end = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::TakeSpecialAction { source, .. } if *source == licid),
        )
        .expect("the original ender can act with a nonempty stack");
    game.apply(PlayerId::One, end).unwrap();
    assert_eq!(
        game.stack.len(),
        stack_len,
        "the special action does not use the stack"
    );
    assert_eq!(battlefield_permanent(&game, licid).attachment_form, None);
    assert_eq!(battlefield_permanent(&game, host).controller, PlayerId::Two);
}
