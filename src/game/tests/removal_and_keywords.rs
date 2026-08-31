use super::*;
#[test]
fn nevinyrrals_disk_uses_the_shared_stack_and_destroys_every_named_type() {
    let mut game = ready_game();
    let disk = creature(10_000, cards::NEVINYRRALS_DISK, PlayerId::One);
    let disk_id = disk.card.id;
    let mox = creature(10_001, cards::MOX_RUBY, PlayerId::One);
    let lions = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let moat = creature(10_003, cards::MOAT, PlayerId::Two);
    let mountain = creature(10_004, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    let mut troll = creature(10_005, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    troll.regeneration_shields = 1;
    game.battlefield
        .extend([disk, mox, lions, moat, mountain, troll]);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let activation = Action::ActivateAbility {
        source: disk_id,
        ability: activated_ability_for(&game, disk_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool,
        ManaPool::default()
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == disk_id)
            .is_some_and(|permanent| permanent.tapped),
        "the Disk taps but remains on the battlefield while its ability is on the stack"
    );
    assert!(matches!(
        game.stack
            .last()
            .and_then(|object| object.ability.as_ref())
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Declarative(_))
    ));

    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![mountain_id, troll_id],
        "the land and regenerated creature are the only survivors"
    );
    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("the Troll regenerated");
    assert!(troll.tapped);
    assert_eq!(troll.regeneration_shields, 0);
}

#[test]
fn object_queries_can_constrain_controller_and_owner_independently() {
    let mut game = ready_game();
    let mut stolen = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::Two);
    stolen.controller = PlayerId::One;
    let stolen_id = stolen.card.id;
    let yours = creature(10_011, cards::SAVANNAH_LIONS, PlayerId::One);
    let yours_id = yours.card.id;
    let theirs = creature(10_012, cards::SAVANNAH_LIONS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.extend([stolen, yours, theirs]);

    let object = spell(10_013, cards::WRATH_OF_GOD, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Destroy {
            object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                related_player: None,
                controller: Some(PlayerSetDef::One(PlayerRefDef::EffectController)),
                owner: Some(PlayerSetDef::Related(PlayerRelation::Opponent)),
                relative_position: None,
                excluding_target: None,
            })),
            can_regenerate: true,
            then: None,
        }),
        &object,
        TriggerContext::empty(),
    );

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != stolen_id),
        "only the permanent you control but an opponent owns matches both constraints",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == yours_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs_id)
    );
}

#[test]
fn zone_relative_queries_span_battlefield_and_graveyard() {
    let mut game = ready_game();
    let permanent = creature(10_020, cards::SAVANNAH_LIONS, PlayerId::One);
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    let graveyard_card = card(10_021, cards::SAVANNAH_LIONS, PlayerId::One);
    let graveyard_id = graveyard_card.id;
    game.players[PlayerId::One.index()]
        .graveyard
        .push(graveyard_card);
    game.players[PlayerId::Two.index()].graveyard.push(card(
        10_022,
        cards::SAVANNAH_LIONS,
        PlayerId::Two,
    ));

    let matches = game.objects_matching_query(
        ObjectQueryDef::matching(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield, ZoneKind::Graveyard],
            PlayerRelation::You,
        ),
        PlayerId::One,
        permanent_id,
        TriggerContext::empty(),
    );

    assert_eq!(
        matches,
        [Target::Permanent(permanent_id), Target::Card(graveyard_id)],
    );
}

#[test]
fn derived_object_players_use_last_known_controller_and_owner() {
    let mut game = ready_game();
    let mut chosen = creature(10_030, cards::SAVANNAH_LIONS, PlayerId::Two);
    chosen.controller = PlayerId::One;
    let chosen_id = chosen.card.id;
    game.battlefield.push(chosen);
    let mut context = EffectResolutionContext::empty();
    context.bind_single_object(
        ObjectBindingIndex::PRIMARY,
        Some(Target::Permanent(chosen_id)),
    );
    game.move_target_to_zone(
        Target::Permanent(chosen_id),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    let resolving = spell_with_targets(
        10_031,
        cards::SWORDS_TO_PLOWSHARES,
        PlayerId::One,
        vec![Target::Permanent(chosen_id)],
        0,
    );

    let recipients = |game: &Game, reference| {
        game.effect_recipients(
            EffectRecipientDef::player(reference),
            &resolving,
            &context,
            ScopedEffect::primary(EffectDef::None),
        )
    };
    assert_eq!(
        recipients(
            &game,
            PlayerRefDef::ControllerOf(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
        ),
        [Target::Player(PlayerId::One)],
    );
    assert_eq!(
        recipients(
            &game,
            PlayerRefDef::OwnerOf(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
        ),
        [Target::Player(PlayerId::Two)],
    );
    assert_eq!(
        recipients(
            &game,
            PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        ),
        [Target::Player(PlayerId::One)],
        "a later instruction uses the target's last-known controller",
    );
    assert_eq!(
        recipients(
            &game,
            PlayerRefDef::OwnerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        ),
        [Target::Player(PlayerId::Two)],
        "a later instruction uses the target's last-known owner",
    );
}

#[test]
fn direct_object_target_references_recheck_legality() {
    static BLACK: ObjectPredicateDef = ObjectPredicateDef::Color(ManaColor::Black);
    static NONBLACK_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&BLACK),
    ]);
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: NONBLACK_CREATURE,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];

    let mut game = ready_game();
    let target = creature(10_040, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let mut doom_blade = spell(10_041, cards::DOOM_BLADE, PlayerId::One, 0);
    doom_blade.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        cast_choices(vec![Target::Permanent(target_id)], 0),
    ));
    doom_blade.ability = Some(StackAbilityPayload {
        origin: primary_ability(cards::DOOM_BLADE),
        definition: None,
        presentation: ObjectCharacteristics::card(cards::DOOM_BLADE, CardPartId::PRIMARY),
        text: Some("Test direct object target reference"),
        target_defs: TARGETS.to_vec(),
        targets: vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Permanent(target_id),
        )],
        context: TriggerContext::empty().into(),
        resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::None)),
        condition: None,
        mode_effects: Vec::new(),
        resolution_destination: None,
        x: 0,
        sacrificed_mana_value: 0,
    });
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target_id)
        .expect("the target remains on the battlefield")
        .card
        .definition = ObjectKind::Card(cards::BLACK_KNIGHT);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target_id)
        .expect("the target remains on the battlefield")
        .card
        .characteristics = CharacteristicSource::Card(cards::BLACK_KNIGHT);

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Tap {
            object: EffectRecipientDef::object(ObjectRefDef::Target(TargetIndex::PRIMARY)),
        }),
        &doom_blade,
        TriggerContext::empty(),
    );

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .expect("the now-black target remains")
            .tapped,
        "a target that became illegal is not affected through an object reference",
    );
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
        .declarative_effect()
        .expect("Wrath uses a resolving effect program");
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &wrath,
        TriggerContext::empty(),
    );
    assert!(game.battlefield.is_empty());
}

#[test]
fn shatter_does_not_destroy_darksteel_ingot() {
    let mut game = ready_game();
    let ingot = creature(10_000, cards::DARKSTEEL_INGOT, PlayerId::Two);
    let ingot_id = ingot.card.id;
    let shatter = card(10_001, cards::SHATTER, PlayerId::One);
    game.battlefield.push(ingot);
    game.players[0].hand.push(shatter.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.red = 1;

    let cast = cast_action(shatter.id, vec![Target::Permanent(ingot_id)], Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "indestructible does not make the artifact an illegal target",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let ingot = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ingot_id)
        .expect("Darksteel Ingot survives Shatter");
    assert!(game.permanent_has_executable_keyword(ingot, KeywordAbility::Indestructible));
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SHATTER),
        "Shatter resolved and went to its owner's graveyard",
    );
}

#[test]
fn indestructible_stops_destruction_and_lethal_damage_but_not_other_death() {
    let mut game = ready_game();
    let mut lions = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    lions
        .temporary_keywords
        .push(KeywordAbility::Indestructible);
    game.battlefield.push(lions);

    game.destroy_permanent_without_regeneration(lions_id);
    assert_eq!(
        game.battlefield.len(),
        1,
        "no-regeneration destroy still fails"
    );

    game.battlefield[0].damage = 1;
    game.check_state_based_actions();
    assert_eq!(
        game.battlefield.len(),
        1,
        "lethal damage does not destroy it"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        lions_id,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(0),
            ValueDef::Constant(-1),
        )],
        ContinuousEffectExpiration::Never,
    );
    game.check_state_based_actions();
    assert!(
        game.battlefield.is_empty(),
        "zero toughness puts it into the graveyard without destroying it",
    );

    let mut angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    angel
        .temporary_keywords
        .push(KeywordAbility::Indestructible);
    angel.damage = 1;
    angel.deathtouch_damage = true;
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    game.check_state_based_actions();
    assert_eq!(
        game.battlefield.len(),
        1,
        "deathtouch damage does not destroy it"
    );

    game.sacrifice_permanent(angel_id);
    assert!(
        game.battlefield.is_empty(),
        "indestructible can be sacrificed"
    );
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
        attacker: CardInstanceId(10_001),
        defender: AttackDefender::Player(PlayerId::Two),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: CardInstanceId(10_002),
        defender: AttackDefender::Player(PlayerId::Two),
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
fn negate_and_essence_scatter_split_the_stack_by_card_kind() {
    let mut game = ready_game();
    // A creature spell and a noncreature spell, both waiting to resolve.
    game.stack
        .push(spell(10_001, cards::SAVANNAH_LIONS, PlayerId::Two, 0));
    game.stack
        .push(spell(10_002, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    let scatter = card(10_003, cards::ESSENCE_SCATTER, PlayerId::One);
    let negate = card(10_004, cards::NEGATE, PlayerId::One);
    game.players[0]
        .hand
        .extend([scatter.clone(), negate.clone()]);
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 2;

    let spells_hit = |game: &Game, card| -> Vec<StackObjectId> {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell {
                    card: offered,
                    choices,
                    ..
                } if offered == card => choices.iter_targets().find_map(|target| match target {
                    Target::Spell(id) => Some(*id),
                    _ => None,
                }),
                _ => None,
            })
            .collect()
    };

    let scatter_targets = spells_hit(&game, scatter.id);
    assert_eq!(
        scatter_targets,
        vec![StackObjectId(10_001)],
        "Essence Scatter sees only the creature spell"
    );
    let negate_targets = spells_hit(&game, negate.id);
    assert_eq!(
        negate_targets,
        vec![StackObjectId(10_002)],
        "Negate sees only the noncreature spell"
    );

    game.apply(
        PlayerId::One,
        cast_action(
            negate.id,
            vec![Target::Spell(StackObjectId(10_002))],
            Vec::new(),
            0,
        ),
    )
    .expect("Negate can target the noncreature spell");
    pass_priority_pair(&mut game);
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
fn duress_takes_a_noncreature_nonland_card_of_the_casters_choosing() {
    let mut game = ready_game();
    let duress = card(10_000, cards::DURESS, PlayerId::One);
    game.players[0].hand.push(duress.clone());
    game.players[0].mana_pool.black = 1;
    game.players[1].hand.extend([
        card(10_001, cards::SAVANNAH_LIONS, PlayerId::Two), // creature: off limits
        card(10_002, cards::MOUNTAIN, PlayerId::Two),       // land: off limits
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two), // fair game
        card(10_004, cards::BLACK_LOTUS, PlayerId::Two),    // fair game
    ]);

    game.apply(
        PlayerId::One,
        cast_action(
            duress.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("Duress can target the opponent");
    pass_priority_pair(&mut game);

    let seen = game
        .observe(PlayerId::One)
        .last_seen_hand
        .expect("Duress reveals the complete hand to its controller");
    assert_eq!(seen.0, PlayerId::Two);
    assert_eq!(seen.1.len(), 4, "the observation includes excluded cards");

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses");
    assert_eq!(
        decision.options.len(),
        2,
        "both noncreature, nonland cards are legal choices"
    );
    // The hand is revealed, so the choice is public rather than hidden.
    assert_eq!(decision.visibility, DecisionVisibility::Public);

    let choice = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::LIGHTNING_BOLT)
            })
        })
        .expect("the Bolt is a legal choice")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![choice],
        },
    )
    .expect("choosing the revealed card is legal");

    assert_eq!(game.players[1].hand.len(), 3, "one card was discarded");
    assert!(
        !game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and it was the one the caster named"
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardsDiscarded {
            player: PlayerId::Two,
            cards,
        } if cards.iter().any(|(_, definition)| *definition == cards::LIGHTNING_BOLT)
    )));
}

#[test]
fn duress_observes_the_hand_without_asking_when_nothing_can_be_discarded() {
    let mut game = ready_game();
    let duress = card(10_010, cards::DURESS, PlayerId::One);
    game.players[0].hand.push(duress.clone());
    game.players[0].mana_pool.black = 1;
    game.players[1].hand.extend([
        card(10_011, cards::SAVANNAH_LIONS, PlayerId::Two),
        card(10_012, cards::MOUNTAIN, PlayerId::Two),
    ]);

    game.apply(
        PlayerId::One,
        cast_action(
            duress.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.observe(PlayerId::One)
            .last_seen_hand
            .expect("the full hand was still observed")
            .1
            .len(),
        2
    );
    assert!(game.players[1].graveyard.is_empty());
}

#[test]
fn a_thoughtseize_shaped_sequence_loses_life_after_the_generic_hand_choice() {
    static DISCARD_CHOSEN: EffectDef = EffectDef::DiscardCards {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    };
    static CHOOSE_NONLAND: EffectDef = EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &DISCARD_CHOSEN,
    });
    static THOUGHTSEIZE_SHAPED: [EffectDef; 3] = [
        EffectDef::LookAtHand {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
        CHOOSE_NONLAND,
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ];

    let mut game = ready_game();
    game.players[1].hand.extend([
        card(10_001, cards::MOUNTAIN, PlayerId::Two),
        card(10_002, cards::LIGHTNING_BOLT, PlayerId::Two),
        card(10_003, cards::BLACK_LOTUS, PlayerId::Two),
    ]);
    let source = spell_with_targets(
        10_000,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&THOUGHTSEIZE_SHAPED)),
        &source,
        TriggerContext::empty(),
    );

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster chooses a legal card from the revealed hand");
    assert_eq!(game.players[0].life, 20, "the sequence tail is suspended");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(
        game.players[0].life, 18,
        "the tail resumes after discarding"
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::LIGHTNING_BOLT,
    );

    let mut no_match = ready_game();
    no_match.players[1]
        .hand
        .push(card(10_011, cards::MOUNTAIN, PlayerId::Two));
    let source = spell_with_targets(
        10_010,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    no_match.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&THOUGHTSEIZE_SHAPED)),
        &source,
        TriggerContext::empty(),
    );

    assert!(no_match.pending_decisions.is_empty());
    assert_eq!(
        no_match.players[0].life, 18,
        "the independent life-loss instruction still resolves when no card qualifies",
    );
    assert!(no_match.players[1].graveyard.is_empty());
}

include!("removal_and_keywords/card_selection_and_keywords.rs");
