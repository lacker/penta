//! Reconstruction coverage for creator-owned token semantics.

use super::*;
use crate::TokenCharacteristics;
use crate::card::{CardArt, CardTypeSet, DeclarativeAbilityDef};
use crate::game::PendingTrigger;

fn clue() -> TokenCharacteristics {
    crate::card::tokens::clue().with_art(CardArt::new(
        "ef607895-d6d2-44ab-a6b4-84af55fce593",
        "Daneen Wilkerson",
    ))
}

fn incubator() -> TokenCharacteristics {
    crate::card::tokens::incubator().with_art(CardArt::new(
        "2c5ed737-657b-43bf-b222-941da7579a4a",
        "Johann Bodin",
    ))
}

fn created_token(game: &Game, token: TokenCharacteristics) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.token_characteristics == Some(token))
        .expect("the authored token arrived")
}

fn directly_authored_token(game: &Game, definition: CardDefinitionId) -> TokenCharacteristics {
    let ability = game
        .catalog
        .get(definition)
        .and_then(|definition| definition.rules.ability(AbilityId::PRIMARY))
        .expect("the printed creator has its token ability");
    let crate::card::AbilityProgramDef::Effects(crate::card::EffectDef::CreateToken {
        token, ..
    }) = ability.effect.definition
    else {
        panic!("the printed creator directly creates its token");
    };
    token
}

fn drain_pending(game: &mut Game) {
    for _ in 0..24 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the reconstructed decision answer is legal");
        } else {
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("the reconstructed game accepts priority");
        }
    }
    panic!("the reconstructed token state did not settle");
}

fn double_faced_copy_token_game() -> (Game, GameObjectId, CardPartId, CardPartId) {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let mut source = crate::game::tests::creature(
        81_010,
        crate::card::cards::HUNTMASTER_OF_THE_FELLS,
        PlayerId::One,
    );
    let front = source.presented;
    let back = game
        .catalog
        .get(crate::card::cards::HUNTMASTER_OF_THE_FELLS)
        .and_then(|definition| definition.other_face(front))
        .expect("Huntmaster has a back face");
    source.presented = back;
    let source_id = source.card.id;
    game.battlefield.push(source);
    let (copy, faces, presented) = {
        let source = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
            .expect("the source is present");
        (
            Game::copiable_characteristics(source),
            game.double_faced_copiable_characteristics(source),
            source.presented,
        )
    };
    game.create_token_copy(PlayerId::One, copy, faces, presented);
    drain_pending(&mut game);
    let token_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the copy-token arrived")
        .card
        .id;
    (game, token_id, front, back)
}

fn checkpoint_permanent(wire: &Value, id: GameObjectId) -> &Value {
    wire["checkpoint"]["battlefield"]
        .as_array()
        .expect("checkpoint battlefield is an array")
        .iter()
        .find(|permanent| permanent["objectId"] == json!(id.0))
        .expect("the checkpoint carries the permanent")
}

fn checkpoint_permanent_mut(wire: &mut Value, id: GameObjectId) -> &mut Value {
    wire["checkpoint"]["battlefield"]
        .as_array_mut()
        .expect("checkpoint battlefield is an array")
        .iter_mut()
        .find(|permanent| permanent["objectId"] == json!(id.0))
        .expect("the checkpoint carries the permanent")
}

#[test]
fn checkpoint_round_trips_authored_and_card_copy_tokens() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    game.create_token(PlayerId::One, clue());
    game.create_token_copy(
        PlayerId::One,
        CopiableCharacteristics {
            base: ObjectCharacteristics::card(crate::card::cards::SERRA_ANGEL, CardPartId::PRIMARY),
            added_types: CardTypeSet::empty(),
            added_abilities: Vec::new(),
            retain_printed_subtypes: false,
            base_power_toughness: None,
            colors: None,
            added_creature_types: Vec::new(),
            no_mana_cost: false,
        },
        None,
        CardPartId::PRIMARY,
    );
    drain_pending(&mut game);

    let authored_id = created_token(&game, clue()).card.id;
    let copy_id = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Token
                && permanent.token_characteristics.is_none()
                && permanent.copy_effect.as_ref().is_some_and(|copy| {
                    copy.base
                        == ObjectCharacteristics::card(
                            crate::card::cards::SERRA_ANGEL,
                            CardPartId::PRIMARY,
                        )
                })
        })
        .expect("the token copying a printed card arrived")
        .card
        .id;

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_001);
    assert_eq!(
        wire["checkpoint"]["version"],
        json!(crate::protocol::CHECKPOINT_VERSION),
    );

    let authored = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == authored_id)
        .expect("the authored token reconstructs");
    assert_eq!(authored.card.definition, ObjectKind::Token);
    assert_eq!(authored.token_characteristics, Some(clue()));
    assert_eq!(authored.presented, CardPartId::PRIMARY);

    let copied = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == copy_id)
        .expect("the card-copy token reconstructs");
    assert_eq!(copied.card.definition, ObjectKind::Token);
    assert_eq!(copied.token_characteristics, None);
    assert_eq!(
        copied.copy_effect.as_ref().map(|copy| copy.base),
        Some(ObjectCharacteristics::card(
            crate::card::cards::SERRA_ANGEL,
            CardPartId::PRIMARY,
        )),
    );
}

/// An X/X token's size is a copiable value the board cannot recompute: the
/// card it was read from is in exile and the creature that exiled it is
/// gone. The checkpoint carries the numbers beside the authored token.
#[test]
fn checkpoint_round_trips_the_size_an_x_x_token_came_out_at() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.put_onto_battlefield(PlayerId::Two, crate::card::cards::ICY_MANIPULATOR)
        .expect("cataloged");
    let apparition = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SKYCLAVE_APPARITION)
        .expect("cataloged");
    drain_pending(&mut game);
    game.destroy_permanent(apparition);
    drain_pending(&mut game);

    let illusion = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the Illusion arrived");
    let illusion_id = illusion.card.id;
    assert_eq!(game.power(illusion), Some(4), "four for a four-mana card");

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_021);
    assert_eq!(
        checkpoint_permanent(&wire, illusion_id)["tokenStats"],
        json!([4, 4]),
        "the size travels with the token",
    );
    let rebuilt_illusion = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == illusion_id)
        .expect("the Illusion reconstructs");
    assert_eq!(rebuilt.power(rebuilt_illusion), Some(4));
    assert_eq!(rebuilt.toughness(rebuilt_illusion), Some(4));
}

#[test]
fn checkpoint_round_trips_back_up_double_faced_copy_token() {
    let (game, token_id, front, back) = double_faced_copy_token_game();

    let (_, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_011);
    let token = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == token_id)
        .expect("the copy-token reconstructs");
    let faces = token
        .double_faced_token_copy
        .as_ref()
        .expect("both copied faces reconstruct");
    assert_eq!(faces.kind, DoubleFacedKind::Transforming);
    assert_eq!(faces.front_part, front);
    assert_eq!(faces.back_part, back);
    assert_eq!(
        faces.front.base,
        ObjectCharacteristics::card(crate::card::cards::HUNTMASTER_OF_THE_FELLS, front),
    );
    assert_eq!(
        faces.back.base,
        ObjectCharacteristics::card(crate::card::cards::HUNTMASTER_OF_THE_FELLS, back),
    );
    assert_eq!(token.presented, back);
    assert_eq!(rebuilt.permanent_mana_value(token), 4);

    rebuilt.transform_permanent(token_id);

    let token = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == token_id)
        .expect("the transformed copy-token remains");
    assert_eq!(token.presented, front);
    assert_eq!(
        Game::effective_rules_source(token),
        ObjectCharacteristics::card(crate::card::cards::HUNTMASTER_OF_THE_FELLS, front),
    );
    assert_eq!(rebuilt.permanent_mana_value(token), 4);
}

#[test]
fn checkpoint_rejects_malformed_double_faced_copy_token_state() {
    let (game, token_id, front, _) = double_faced_copy_token_game();
    let viewer = PlayerId::One;
    let (wire, _) = rebuild_current_checkpoint(&game, viewer, 81_012);
    let hidden = true_hidden_hypothesis(&game, viewer);
    let rebuild = |wire: &Value| {
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, wire, &hidden, 81_013)
    };

    let mut same_face_ids = wire.clone();
    checkpoint_permanent_mut(&mut same_face_ids, token_id)["doubleFacedTokenCopy"]["backPartId"] =
        json!(front.0);
    assert!(
        rebuild(&same_face_ids)
            .expect_err("identical physical face IDs are invalid")
            .contains("uses the same ID for both faces"),
    );

    let mut invalid_presented = wire.clone();
    checkpoint_permanent_mut(&mut invalid_presented, token_id)["presentedPartId"] = json!(u8::MAX);
    assert!(
        rebuild(&invalid_presented)
            .expect_err("a third physical face is invalid")
            .contains("presents neither of its physical faces"),
    );

    let mut printed_owner = wire.clone();
    checkpoint_permanent_mut(&mut printed_owner, token_id)["objectKind"] = json!({
        "kind": "card",
        "definition": crate::card::cards::SERRA_ANGEL.get(),
    });
    assert!(
        rebuild(&printed_owner)
            .expect_err("printed cards cannot carry token-copy faces")
            .contains("card permanent carries double-faced token-copy values"),
    );

    let mut authored_and_copied = wire;
    let locator = token_characteristics_locator(&game.catalog, clue())
        .expect("the authored Clue has a locator");
    checkpoint_permanent_mut(&mut authored_and_copied, token_id)["tokenCharacteristics"] =
        serde_json::to_value(locator).expect("the token locator serializes");
    assert!(
        rebuild(&authored_and_copied)
            .expect_err("authored and copied token faces are ambiguous")
            .contains("both authored and copied double-faced values"),
    );
}

#[test]
fn checkpoint_round_trips_a_sacrificed_clue_ability_on_the_stack() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    game.create_token(PlayerId::One, clue());
    drain_pending(&mut game);
    let clue_id = created_token(&game, clue()).card.id;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == clue_id),
        )
        .expect("the Clue ability can be activated");
    game.apply(PlayerId::One, activation)
        .expect("the Clue activation is legal");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != clue_id),
        "sacrificing the Clue is a cost",
    );
    assert_eq!(game.stack.len(), 1);

    let hand_before = game.players[PlayerId::One.index()].hand.len();
    let (_, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_002);
    let stacked = rebuilt
        .stack
        .iter()
        .next()
        .expect("the ability reconstructs");
    assert_eq!(stacked.card.definition, ObjectKind::Ability);
    assert_eq!(stacked.source, Some(clue_id));
    let payload = stacked
        .ability
        .as_ref()
        .expect("the frozen payload reconstructs");
    assert_eq!(
        payload.origin,
        AbilityOrigin::Token {
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    );
    assert_eq!(
        payload.presentation,
        ObjectCharacteristics::token(clue(), CardPartId::PRIMARY),
    );

    drain_pending(&mut rebuilt);
    assert!(rebuilt.stack.is_empty());
    assert_eq!(
        rebuilt.players[PlayerId::One.index()].hand.len(),
        hand_before + 1,
        "the reconstructed static ability definition still resolves",
    );
}

#[test]
fn checkpoint_round_trips_a_pending_token_trigger() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let token = directly_authored_token(&game, crate::card::cards::SERPENT_GENERATOR);
    game.create_token(PlayerId::One, token);
    drain_pending(&mut game);
    let source_object = created_token(&game, token).card.id;
    let ability = token
        .rules()
        .ability(AbilityId::PRIMARY)
        .copied()
        .expect("the Snake has its poison trigger");
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        panic!("the Snake ability is triggered");
    };
    let origin = AbilityOrigin::Token {
        part: CardPartId::PRIMARY,
        ability: AbilityId::PRIMARY,
    };
    game.pending_triggers.push(PendingTrigger {
        id: 0,
        source: AbilitySourceRef {
            object: source_object,
            ability: origin,
        },
        presentation: ObjectCharacteristics::token(token, CardPartId::PRIMARY),
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: ability.text,
        target_defs: triggered.targets.to_vec(),
        targets: Vec::new(),
        effect: ability
            .declarative_effect()
            .expect("the poison trigger is declarative"),
        resolver: Game::ability_resolver(origin, &ability),
        context: EffectResolutionContext::empty(),
        condition: triggered.condition,
        modes: None,
        x: 0,
    });
    game.next_trigger_id = 1;

    let (_, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_003);
    let pending = rebuilt
        .pending_triggers
        .first()
        .expect("the token trigger reconstructs");
    assert_eq!(pending.source.object, source_object);
    assert_eq!(pending.source.ability, origin);
    assert_eq!(
        pending.presentation,
        ObjectCharacteristics::token(token, CardPartId::PRIMARY),
    );
    assert_eq!(pending.text, ability.text);
}

#[test]
fn token_granted_origins_recover_their_creator_owned_ability() {
    let catalog = crate::poc::catalog().expect("the card catalog builds");
    let granted = clue()
        .rules()
        .ability(AbilityId::PRIMARY)
        .copied()
        .expect("the Clue ability exists");
    let origin = AbilityOrigin::TokenGranted {
        source: GameObjectId(81_006),
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    };
    let locator = ability_locator_for_origin(&catalog, origin, |candidate| *candidate == granted)
        .expect("the token-granted ability has a creator-owned locator");
    assert!(matches!(locator, model::AbilityLocator::Token { .. }));
    assert_eq!(catalog_ability(&catalog, &locator), Some(granted));
    assert_eq!(
        ability_origin_from_snapshot(ability_origin_snapshot(origin)),
        origin,
    );
}

#[test]
fn checkpoint_round_trips_a_transformed_incubator_part() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    game.create_token(PlayerId::One, incubator());
    drain_pending(&mut game);
    let incubator_id = created_token(&game, incubator()).card.id;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == incubator_id)
        .expect("the Incubator is on the battlefield")
        .presented = CardPartId(1);

    let (_, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_004);
    let transformed = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == incubator_id)
        .expect("the transformed Incubator reconstructs");
    assert_eq!(transformed.card.definition, ObjectKind::Token);
    assert_eq!(transformed.token_characteristics, Some(incubator()));
    assert_eq!(transformed.presented, CardPartId(1));
    assert_eq!(
        Game::effective_rules_source(transformed),
        ObjectCharacteristics::token(incubator(), CardPartId(1)),
    );
}

#[test]
fn checkpoint_round_trips_a_tetravite_owned_by_its_declarative_creator() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let token = crate::card::sets::TETRAVITE;
    game.create_token_from(PlayerId::One, token, Some(GameObjectId(81_007)));
    drain_pending(&mut game);
    let token_id = created_token(&game, token).card.id;

    let locator = token_characteristics_locator(&game.catalog, token)
        .expect("the declarative Tetravus creator owns a semantic locator");
    let creator = catalog_ability(&game.catalog, locator.creator())
        .expect("the declarative creator ability reconstructs");
    assert!(creator.effect.custom_behavior().is_none());
    assert_eq!(
        catalog_token_characteristics(&game.catalog, &locator),
        Some(token),
    );

    let (_, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_008);
    let permanent = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == token_id)
        .expect("the declaratively created Tetravite reconstructs");
    assert_eq!(permanent.card.definition, ObjectKind::Token);
    assert_eq!(permanent.token_characteristics, Some(token));
    assert_eq!(permanent.created_by, Some(GameObjectId(81_007)));
}

#[test]
fn current_checkpoint_version_is_advertised_and_rejects_predecessor() {
    let game = crate::game::tests::ready_game();
    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let mut wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let checkpoint_version = crate::protocol::CHECKPOINT_VERSION;
    let predecessor = checkpoint_version
        .checked_sub(1)
        .expect("the checkpoint format has a predecessor");
    let capability = format!("reconstruction.checkpoint.v{checkpoint_version}");
    assert_eq!(wire["checkpoint"]["version"], json!(checkpoint_version));
    assert!(
        wire["protocolCapabilities"]
            .as_array()
            .is_some_and(|capabilities| capabilities
                .iter()
                .any(|advertised| advertised == &capability)),
    );

    wire["checkpoint"]["version"] = json!(predecessor);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        81_005,
    )
    .expect_err("the previous checkpoint format is not interpreted as current");
    assert!(error.contains(&format!(
        "checkpoint version {predecessor} does not match {checkpoint_version}",
    )));
}
