use super::*;
use serde_json::json;

const LICID_TRANSFORM: AbilityOrigin = AbilityOrigin::Printed {
    definition: cards::DOMINATING_LICID,
    part: CardPartId::PRIMARY,
    ability: AbilityId(0),
};
static DESTROY_THEN_READ_HOST_CONTROLLER: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex(0)),
        can_regenerate: false,
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex(1)),
        amount: ValueDef::Constant(1),
    },
];

fn add_host_and_dominating_licid(
    game: &mut Game,
    source_controller: PlayerId,
) -> (GameObjectId, GameObjectId) {
    let source = GameObjectId(10_000);
    let host = GameObjectId(10_001);
    game.battlefield.extend([
        creature(source.0, cards::DOMINATING_LICID, source_controller),
        creature(host.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    (source, host)
}

fn install_licid_effect(
    game: &mut Game,
    source: GameObjectId,
    host: GameObjectId,
    ender: PlayerId,
    id: u64,
) {
    let transform = game
        .catalog
        .get(cards::DOMINATING_LICID)
        .unwrap()
        .part(CardPartId::PRIMARY)
        .unwrap()
        .rules
        .ability(AbilityId(0))
        .copied()
        .unwrap();
    let EffectDef::BecomeAuraAndAttach { end, .. } = transform.declarative_effect().unwrap() else {
        panic!("Dominating Licid has the shared transform effect");
    };
    let licid = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    licid.attachment_form = Some(AttachmentForm::Licid);
    licid.licid_effects.push(LicidEffect {
        id: ContinuousEffectTimestamp(id),
        ender,
        transform_action: LICID_TRANSFORM,
        end: *end,
    });
    assert!(game.try_attach(source, host));
}

fn controller(game: &Game, object: GameObjectId) -> PlayerId {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .unwrap()
        .controller
}

fn gain_control_while_source_remains(
    game: &mut Game,
    source: GameObjectId,
    host: GameObjectId,
    controller: PlayerId,
) {
    let mut object = spell_with_targets(
        20_000 + source.0,
        cards::LIGHTNING_BOLT,
        controller,
        vec![Target::Permanent(host)],
        0,
    );
    object.source = Some(source);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::GainControlWhileSourceRemains {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            while_tapped: false,
        }),
        &object,
        TriggerContext::empty(),
    );
}

#[test]
fn later_licid_control_outlives_an_earlier_until_cleanup_control_effect() {
    let mut game = ready_game();
    let (source, host) = add_host_and_dominating_licid(&mut game, PlayerId::One);

    game.gain_control_until_end_of_turn(&[host], PlayerId::Two);
    assert_eq!(controller(&game, host), PlayerId::Two);
    install_licid_effect(&mut game, source, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::One);

    game.finish_cleanup();
    assert_eq!(controller(&game, host), PlayerId::One);
    assert!(game.end_aura_effect(source));
    assert_eq!(controller(&game, host), PlayerId::One);
}

#[test]
fn later_until_cleanup_control_reveals_the_older_licid_then_the_base() {
    let mut game = ready_game();
    let (source, host) = add_host_and_dominating_licid(&mut game, PlayerId::Two);

    install_licid_effect(&mut game, source, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::Two);
    game.gain_control_until_end_of_turn(&[host], PlayerId::One);
    assert_eq!(controller(&game, host), PlayerId::One);

    game.finish_cleanup();
    assert_eq!(controller(&game, host), PlayerId::Two);
    assert!(game.end_aura_effect(source));
    assert_eq!(controller(&game, host), PlayerId::One);
}

#[test]
fn later_licid_control_reveals_an_older_source_sustained_control_effect() {
    let mut game = ready_game();
    let (licid, host) = add_host_and_dominating_licid(&mut game, PlayerId::One);
    let holder = GameObjectId(10_002);
    game.battlefield
        .push(creature(holder.0, cards::SEDGE_TROLL, PlayerId::Two));

    gain_control_while_source_remains(&mut game, holder, host, PlayerId::Two);
    assert_eq!(controller(&game, host), PlayerId::Two);
    install_licid_effect(&mut game, licid, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::One);

    assert!(game.end_aura_effect(licid));
    assert_eq!(controller(&game, host), PlayerId::Two);
    game.battlefield
        .retain(|permanent| permanent.card.id != holder);
    game.check_state_based_actions();
    assert_eq!(controller(&game, host), PlayerId::One);
}

#[test]
fn later_source_sustained_control_reveals_the_older_licid_then_the_base() {
    let mut game = ready_game();
    let (licid, host) = add_host_and_dominating_licid(&mut game, PlayerId::Two);
    let holder = GameObjectId(10_002);
    game.battlefield
        .push(creature(holder.0, cards::SEDGE_TROLL, PlayerId::One));

    install_licid_effect(&mut game, licid, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::Two);
    gain_control_while_source_remains(&mut game, holder, host, PlayerId::One);
    assert_eq!(controller(&game, host), PlayerId::One);

    game.battlefield
        .retain(|permanent| permanent.card.id != holder);
    game.check_state_based_actions();
    assert_eq!(controller(&game, host), PlayerId::Two);
    assert!(game.end_aura_effect(licid));
    assert_eq!(controller(&game, host), PlayerId::One);
}

#[test]
fn redundant_source_sustained_control_keeps_its_timestamped_place() {
    let mut game = ready_game();
    let host = GameObjectId(10_000);
    let older_holder = GameObjectId(10_001);
    let newer_holder = GameObjectId(10_002);
    game.battlefield.extend([
        creature(host.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(older_holder.0, cards::SEDGE_TROLL, PlayerId::Two),
        creature(newer_holder.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);

    gain_control_while_source_remains(&mut game, older_holder, host, PlayerId::Two);
    game.gain_control_until_end_of_turn(&[host], PlayerId::One);
    gain_control_while_source_remains(&mut game, newer_holder, host, PlayerId::One);
    assert_eq!(controller(&game, host), PlayerId::One);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == host)
            .unwrap()
            .control_while_source_remains
            .len(),
        2,
    );

    game.finish_cleanup();
    assert_eq!(controller(&game, host), PlayerId::One);
    game.battlefield
        .retain(|permanent| permanent.card.id != newer_holder);
    game.check_state_based_actions();
    assert_eq!(controller(&game, host), PlayerId::Two);
    game.battlefield
        .retain(|permanent| permanent.card.id != older_holder);
    game.check_state_based_actions();
    assert_eq!(controller(&game, host), PlayerId::One);
}

#[test]
fn temporarily_stealing_a_dominating_licid_updates_and_restores_its_host() {
    let mut game = ready_game();
    let (source, host) = add_host_and_dominating_licid(&mut game, PlayerId::Two);
    install_licid_effect(&mut game, source, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::Two);

    game.gain_control_until_end_of_turn(&[source], PlayerId::One);
    assert_eq!(controller(&game, source), PlayerId::One);
    assert_eq!(controller(&game, host), PlayerId::One);

    game.finish_cleanup();
    assert_eq!(controller(&game, source), PlayerId::Two);
    assert_eq!(controller(&game, host), PlayerId::Two);
}

#[test]
fn removing_a_dominating_licid_rederives_control_before_the_next_sequence_clause() {
    let mut game = ready_game();
    let (source, host) = add_host_and_dominating_licid(&mut game, PlayerId::Two);
    install_licid_effect(&mut game, source, host, PlayerId::One, 50_000);
    assert_eq!(controller(&game, host), PlayerId::Two);

    let life = game.players[PlayerId::One.index()].life;
    let mut object = spell(20_000, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::default().with_targets(vec![
            TargetSelection::new(TargetSlotId(0), vec![Target::Permanent(source)]),
            TargetSelection::new(TargetSlotId(1), vec![Target::Permanent(host)]),
        ]),
    ));
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&DESTROY_THEN_READ_HOST_CONTROLLER)),
        &object,
        TriggerContext::empty(),
    );

    assert_eq!(controller(&game, host), PlayerId::One);
    assert_eq!(game.players[PlayerId::One.index()].life, life + 1);
    assert_eq!(game.players[PlayerId::Two.index()].life, 20);
}

#[test]
fn checkpoint_v3_round_trips_overlapping_control_layers_and_licid_provenance() {
    let mut game = ready_game();
    for player in &mut game.players {
        player.hand.clear();
        player.library.clear();
        player.outside_game.clear();
    }
    let (source, host) = add_host_and_dominating_licid(&mut game, PlayerId::Two);
    install_licid_effect(&mut game, source, host, PlayerId::One, 50_000);
    game.gain_control_until_end_of_turn(&[host], PlayerId::One);
    assert_eq!(controller(&game, host), PlayerId::One);

    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &actions,
    );
    let hidden = json!({
        "hands": { "p1": [], "p2": [] },
        "libraries": { "p1": [], "p2": [] },
        "outsideGame": { "p1": [], "p2": [] }
    });
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 7)
            .unwrap();

    let rebuilt_host = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == host)
        .unwrap();
    assert_eq!(rebuilt_host.controller, PlayerId::One);
    assert_eq!(rebuilt_host.control_layer_base, Some(PlayerId::One));
    assert_eq!(rebuilt_host.control_until_end_of_turn.len(), 1);
    let rebuilt_source = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    assert_eq!(rebuilt_source.attachment_form, Some(AttachmentForm::Licid));
    assert_eq!(rebuilt_source.licid_effects.len(), 1);
    assert_eq!(
        rebuilt_source.licid_effects[0].end.declarative_effect(),
        Some(EffectDef::EndAuraEffect)
    );

    rebuilt.finish_cleanup();
    assert_eq!(controller(&rebuilt, host), PlayerId::Two);
    assert!(rebuilt.end_licid_effect(source, 50_000));
    assert_eq!(controller(&rebuilt, host), PlayerId::One);
}

#[test]
fn checkpoint_v3_round_trips_source_sustained_control_layers() {
    let mut game = ready_game();
    for player in &mut game.players {
        player.hand.clear();
        player.library.clear();
        player.outside_game.clear();
    }
    let holder = GameObjectId(10_000);
    let host = GameObjectId(10_001);
    game.battlefield.extend([
        creature(holder.0, cards::SEDGE_TROLL, PlayerId::Two),
        creature(host.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    gain_control_while_source_remains(&mut game, holder, host, PlayerId::Two);

    let observation = game.observe(PlayerId::One);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &actions,
    );
    let hidden = json!({
        "hands": { "p1": [], "p2": [] },
        "libraries": { "p1": [], "p2": [] },
        "outsideGame": { "p1": [], "p2": [] }
    });
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 7)
            .unwrap();

    let rebuilt_host = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == host)
        .unwrap();
    assert_eq!(rebuilt_host.controller, PlayerId::Two);
    assert_eq!(rebuilt_host.control_while_source_remains.len(), 1);
    assert_eq!(rebuilt_host.control_while_source_remains[0].source, holder);

    rebuilt
        .battlefield
        .retain(|permanent| permanent.card.id != holder);
    rebuilt.check_state_based_actions();
    assert_eq!(controller(&rebuilt, host), PlayerId::One);
}

#[test]
fn copying_an_attached_source_into_and_out_of_dominating_licid_reconciles_immediately() {
    let source = GameObjectId(10_000);
    let host = GameObjectId(10_001);
    let dominating = GameObjectId(10_002);
    let quickening = GameObjectId(10_003);
    let mut game = ready_game();
    game.battlefield.extend([
        creature(source.0, cards::QUICKENING_LICID, PlayerId::Two),
        creature(host.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(dominating.0, cards::DOMINATING_LICID, PlayerId::One),
        creature(quickening.0, cards::QUICKENING_LICID, PlayerId::One),
    ]);
    let transform = game
        .catalog
        .get(cards::QUICKENING_LICID)
        .unwrap()
        .part(CardPartId::PRIMARY)
        .unwrap()
        .rules
        .ability(AbilityId(0))
        .copied()
        .unwrap();
    let EffectDef::BecomeAuraAndAttach { end, .. } = transform.declarative_effect().unwrap() else {
        panic!("Quickening Licid has the shared transform effect");
    };
    let licid = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
        .unwrap();
    licid.attachment_form = Some(AttachmentForm::Licid);
    licid.licid_effects.push(LicidEffect {
        id: ContinuousEffectTimestamp(50_000),
        ender: PlayerId::Two,
        transform_action: AbilityOrigin::Printed {
            definition: cards::QUICKENING_LICID,
            part: CardPartId::PRIMARY,
            ability: AbilityId(0),
        },
        end: *end,
    });
    assert!(game.try_attach(source, host));
    assert_eq!(controller(&game, host), PlayerId::One);

    let copy_into = spell_with_targets(
        20_000,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(dominating)],
        0,
    );
    let mut copy_into = copy_into;
    copy_into.source = Some(source);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::BecomeCopyOf {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            retain_source_ability: false,
        }),
        &copy_into,
        TriggerContext::empty(),
    );
    assert_eq!(controller(&game, host), PlayerId::Two);

    let mut copy_out = spell_with_targets(
        20_001,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(quickening)],
        0,
    );
    copy_out.source = Some(source);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::BecomeCopyOf {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            retain_source_ability: false,
        }),
        &copy_out,
        TriggerContext::empty(),
    );
    assert_eq!(controller(&game, host), PlayerId::One);
}
