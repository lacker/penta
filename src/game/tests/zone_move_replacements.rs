use super::*;
use crate::ObjectSetBindingIndex;
use crate::card::DestroyFollowUpDef;

fn setup_nexus_and_rest_in_peace() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let rest = game
        .put_onto_battlefield(PlayerId::Two, cards::REST_IN_PEACE)
        .expect("Rest in Peace is cataloged");
    let nexus = game
        .put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    // The helper is about static replacement interactions, not Rest in
    // Peace's enter trigger. Direct setup bypasses the normal trigger-
    // placement boundary, so discard that unrelated pending trigger.
    game.pending_triggers.clear();
    (game, nexus, rest)
}

fn choose_replacement_from(game: &mut Game, player: PlayerId, source: GameObjectId) {
    let decision = game
        .observe(player)
        .decision
        .expect("two replacement effects require an affected-player choice");
    assert_eq!(
        decision.prompt,
        "Choose a replacement effect for Ugin's Nexus"
    );
    assert_eq!(decision.player, player);
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!((decision.minimum, decision.maximum), (1, 1));
    assert!(!decision.cancellable);
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == source))
        .expect("the named replacement source is offered")
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the replacement-order choice is legal");
}

#[test]
fn ugins_nexus_replaces_its_own_graveyard_move_and_performs_the_followup() {
    let mut game = ready_game();
    game.battlefield.clear();
    let nexus = game
        .put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");

    game.destroy_permanent(nexus);

    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::UGINS_NEXUS);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::PermanentLeftBattlefield {
            card,
            destination: BattlefieldExit::Exile,
            ..
        } if *card == nexus
    )));
}

#[test]
fn sacrificing_ugins_nexus_uses_the_same_prospective_replacement_path() {
    let mut game = ready_game();
    game.battlefield.clear();
    let nexus = game
        .put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");

    game.sacrifice_permanent(nexus);

    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::UGINS_NEXUS);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
}

#[test]
fn choosing_nexus_over_rest_in_peace_exiles_it_and_creates_an_extra_turn() {
    let (mut game, nexus, _rest) = setup_nexus_and_rest_in_peace();

    game.destroy_permanent(nexus);
    choose_replacement_from(&mut game, PlayerId::One, nexus);

    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::UGINS_NEXUS);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
}

#[test]
fn choosing_rest_in_peace_over_nexus_exiles_it_without_the_followup() {
    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();

    game.destroy_permanent(nexus);
    choose_replacement_from(&mut game, PlayerId::One, rest);

    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::UGINS_NEXUS);
    assert!(game.extra_turns.is_empty());
}

#[test]
fn a_simultaneously_exiting_rest_in_peace_still_competes_with_nexus() {
    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();

    game.move_permanents_to_graveyard(&[rest, nexus]);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the frozen Rest in Peace still replaces the Nexus move");
    assert_eq!(decision.options.len(), 2);
    assert!(decision.options.iter().any(|option| option.card
        == Some((
            rest,
            ObjectCharacteristics::card(cards::REST_IN_PEACE, CardPartId::PRIMARY)
        ))));
    assert!(decision.options.iter().any(|option| option.card
        == Some((
            nexus,
            ObjectCharacteristics::card(cards::UGINS_NEXUS, CardPartId::PRIMARY)
        ))));

    choose_replacement_from(&mut game, PlayerId::One, nexus);

    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].exile[0].definition, cards::UGINS_NEXUS);
    assert_eq!(game.players[1].exile[0].definition, cards::REST_IN_PEACE);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
}

#[test]
fn legend_rule_resumes_after_a_zone_move_replacement_choice() {
    let (mut game, first_nexus, rest) = setup_nexus_and_rest_in_peace();
    let second_nexus = game
        .put_onto_battlefield(PlayerId::One, cards::UGINS_NEXUS)
        .expect("a second Ugin's Nexus is cataloged");

    game.check_state_based_actions();

    let doomed = if first_nexus == second_nexus {
        unreachable!("distinct permanents have distinct identities")
    } else {
        first_nexus
    };
    choose_replacement_from(&mut game, PlayerId::One, doomed);

    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::UGINS_NEXUS)
            .count(),
        1
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == rest)
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.extra_turns, vec![PlayerId::One]);
}

#[test]
fn divine_offering_waits_for_ugins_nexus_replacement_before_gaining_life() {
    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();
    let offering = card(20_000, cards::DIVINE_OFFERING, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(offering.clone());
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(offering.id, vec![Target::Permanent(nexus)], Vec::new(), 0),
    )
    .expect("Divine Offering can target Ugin's Nexus");
    let spell = game
        .stack
        .last()
        .expect("Divine Offering is on the stack")
        .id;
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        20,
        "the rest of Divine Offering remains suspended behind the replacement choice"
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::DIVINE_OFFERING),
        "the resolving spell does not leave the stack procedure before its effect finishes"
    );
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .all(|card| card.definition != cards::DIVINE_OFFERING)
    );
    assert!(
        !game.events.iter().any(|event| matches!(
            event,
            GameEvent::SpellResolved { card, .. } if *card == spell
        )),
        "SpellResolved is emitted only after the replacement choice"
    );
    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert_eq!(game.players[PlayerId::One.index()].life, 25);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::SpellResolved { card, .. } if *card == spell
    )));
    assert!(game.extra_turns.is_empty());
}

#[test]
fn destroy_outcome_followup_waits_for_replacements_and_counts_only_graveyard_moves() {
    static FOLLOWUP_EFFECTS: [EffectDef; 2] = [
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
        },
    ];
    const DESTROY_AND_COUNT: EffectDef = EffectDef::Destroy {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Artifact),
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        can_regenerate: true,
        then: Some(DestroyFollowUpDef {
            binding: ObjectSetBindingIndex::PRIMARY,
            effect: &EffectDef::Sequence(&FOLLOWUP_EFFECTS),
        }),
    };

    let (mut game, _nexus, rest) = setup_nexus_and_rest_in_peace();
    let object = spell(20_001, cards::PARASELENE, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(DESTROY_AND_COUNT),
        &object,
        TriggerContext::empty(),
    );

    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        21,
        "the follow-up ran, but the permanent exiled instead was not counted",
    );
}

#[test]
fn activated_ability_resolves_only_after_ugins_nexus_replacement_choice() {
    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();
    let vraska = game
        .put_onto_battlefield(PlayerId::One, cards::VRASKA_THE_UNSEEN)
        .expect("Vraska is cataloged");

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: vraska,
            ability: activated_ability_for(&game, vraska, 1),
            targets: activated_targets(Target::Permanent(nexus)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .expect("Vraska can target Ugin's Nexus");
    let ability = game.stack.last().expect("the ability is on the stack").id;
    pass_priority_pair(&mut game);

    assert!(!game.events.iter().any(|event| matches!(
        event,
        GameEvent::AbilityResolved { object, .. } if *object == ability
    )));
    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::AbilityResolved { object, .. } if *object == ability
    )));
}

#[test]
fn declarative_chain_followup_waits_for_ugins_nexus_replacement_choice() {
    const DESTROY_ARTIFACTS: EffectDef = EffectDef::Destroy {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Artifact),
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        can_regenerate: true,
        then: None,
    };
    const OFFER_COPY: EffectDef = EffectDef::May {
        player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
        effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
            object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
            controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
            count: ValueDef::Constant(1),
            retarget: true,
            colors: None,
        }),
    };
    const CHAIN_EFFECTS: [EffectDef; 2] = [DESTROY_ARTIFACTS, OFFER_COPY];

    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();
    game.players[PlayerId::Two.index()].mana_pool.red = 2;
    let mut object = spell_with_targets(
        20_001,
        cards::CHAIN_LIGHTNING,
        PlayerId::One,
        vec![Target::Player(PlayerId::Two)],
        0,
    );
    object.ability = Some(StackAbilityPayload {
        origin: primary_ability(cards::CHAIN_LIGHTNING),
        definition: None,
        presentation: ObjectCharacteristics::card(cards::CHAIN_LIGHTNING, CardPartId::PRIMARY),
        text: Some("Test declarative chain follow-up"),
        target_defs: Vec::new(),
        targets: vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Player(PlayerId::Two),
        )],
        context: TriggerContext::empty().into(),
        resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::Sequence(
            &CHAIN_EFFECTS,
        ))),
        condition: None,
        mode_effects: Vec::new(),
        resolution_destination: None,
        x: 0,
        sacrificed_mana_value: 0,
    });
    game.stack.push(object);

    game.resolve_stack_top();
    assert_eq!(
        game.observe(PlayerId::One)
            .decision
            .expect("the exit replacement choice is asked first")
            .prompt,
        "Choose a replacement effect for Ugin's Nexus"
    );

    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert_eq!(
        game.observe(PlayerId::Two)
            .decision
            .expect("the declarative follow-up resumes after the exit choice")
            .prompt,
        "Test declarative chain follow-up"
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != nexus)
    );
}

#[test]
fn sacrifice_cost_finishes_only_after_ugins_nexus_replacement_choice() {
    let (mut game, nexus, rest) = setup_nexus_and_rest_in_peace();
    let claws = game
        .put_onto_battlefield(PlayerId::One, cards::CLAWS_OF_GIX)
        .expect("Claws of Gix is cataloged");
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: claws,
            ability: activated_ability_for(&game, claws, 0),
            targets: Vec::new(),
            cost_objects: vec![nexus],
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .expect("Ugin's Nexus can be sacrificed to Claws of Gix");

    assert!(
        game.stack.is_empty(),
        "the activated ability is not placed on the stack before its cost finishes"
    );
    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert_eq!(
        game.stack.len(),
        1,
        "the activation finishes after the choice"
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 21);
    assert!(game.extra_turns.is_empty());
}

#[test]
fn simultaneous_exit_replacement_choices_follow_apnap_order() {
    let (mut game, first_nexus, rest) = setup_nexus_and_rest_in_peace();
    let second_nexus = game
        .put_onto_battlefield(PlayerId::Two, cards::UGINS_NEXUS)
        .expect("a second Ugin's Nexus is cataloged");

    // Deliberately propose the nonactive player's object first. CR 616.1
    // still requires the active player to make their choice first.
    game.move_permanents_to_graveyard(&[second_nexus, first_nexus]);
    assert_eq!(
        game.observe(PlayerId::One)
            .decision
            .expect("the active player chooses first")
            .player,
        PlayerId::One
    );
    choose_replacement_from(&mut game, PlayerId::One, rest);
    assert_eq!(
        game.observe(PlayerId::Two)
            .decision
            .expect("the nonactive player chooses second")
            .player,
        PlayerId::Two
    );
    choose_replacement_from(&mut game, PlayerId::Two, rest);
    assert!(game.pending_decisions.is_empty());
}
