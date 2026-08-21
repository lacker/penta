//! Physical double-faced representation stays independent from copy values.

use super::*;

fn synthetic_double_faced_token(
    id: u32,
    kind: DoubleFacedKind,
    back: ObjectCharacteristics,
) -> Permanent {
    let front = copied_characteristics(cards::SERRA_ANGEL);
    let mut permanent = Permanent::entering(
        ObjectInstance {
            id: GameObjectId(id),
            definition: ObjectKind::Token,
            owner: PlayerId::One,
            backing: ObjectBacking::None,
            characteristics: CharacteristicSource::Copy(cards::SERRA_ANGEL),
            counters: [0; CounterKind::COUNT],
        },
        CardPartId::PRIMARY,
        PlayerId::One,
        0,
    );
    permanent.double_faced_token_copy = Some(DoubleFacedCopiableCharacteristics {
        kind,
        front_part: CardPartId::PRIMARY,
        back_part: CardPartId(1),
        front,
        back: CopiableCharacteristics {
            base: back,
            added_types: CardTypeSet::empty(),
            added_abilities: Vec::new(),
            retain_printed_subtypes: false,
        },
    });
    permanent
}

#[test]
fn single_faced_card_copying_incubator_cannot_transform() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut copier = creature(90_000, cards::COPY_ARTIFACT, PlayerId::One);
    copier.copy_effect = Some(CopiableCharacteristics {
        base: ObjectCharacteristics::token(tokens::incubator(), CardPartId::PRIMARY),
        added_types: CardTypeSet::empty(),
        added_abilities: Vec::new(),
        retain_printed_subtypes: false,
    });
    let copier_id = copier.card.id;
    game.battlefield.push(copier);

    game.transform_permanent(copier_id);

    let copier = &game.battlefield[0];
    assert_eq!(copier.presented, CardPartId::PRIMARY);
    assert_eq!(
        Game::effective_rules_source(copier),
        ObjectCharacteristics::token(tokens::incubator(), CardPartId::PRIMARY),
    );
    assert_eq!(
        game.observe(PlayerId::One).battlefield[0].physical_face,
        None,
        "effective transforming values do not make a physical SFC double-faced",
    );
}

#[test]
fn physical_double_faced_card_transforms_without_changing_its_copy_effect() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut copier = creature(90_001, cards::HUNTMASTER_OF_THE_FELLS, PlayerId::One);
    let expected_copy = copied_characteristics(cards::SERRA_ANGEL);
    copier.copy_effect = Some(expected_copy.clone());
    let copier_id = copier.card.id;
    game.battlefield.push(copier);

    game.transform_permanent(copier_id);

    let copier = &game.battlefield[0];
    assert_ne!(copier.presented, CardPartId::PRIMARY);
    assert_eq!(copier.copy_effect, Some(expected_copy));
    assert_eq!(
        Game::effective_rules_source(copier),
        ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
    );
    assert_eq!(game.permanent_mana_value(copier), 5);
    assert_eq!(
        game.observe(PlayerId::One).battlefield[0].physical_face,
        Some(PhysicalFaceObservation {
            kind: DoubleFacedKind::Transforming,
            side: PhysicalFaceSide::Back,
        }),
    );
    game.battlefield[0].face_down = true;
    assert_eq!(
        game.observe(PlayerId::One).battlefield[0].physical_face,
        None,
        "face-down observations hide physical topology",
    );
}

#[test]
fn token_copy_of_back_face_up_transforming_card_keeps_both_faces() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut source = creature(90_002, cards::HUNTMASTER_OF_THE_FELLS, PlayerId::One);
    let front = source.presented;
    let back = game
        .catalog
        .get(cards::HUNTMASTER_OF_THE_FELLS)
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
            .expect("source is present");
        (
            Game::copiable_characteristics(source),
            game.double_faced_copiable_characteristics(source),
            source.presented,
        )
    };
    game.create_token_copy(PlayerId::One, copy, faces, presented);
    drain_pending(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the DFC copy-token arrived");
    assert_eq!(token.presented, back);
    assert!(token.copy_effect.is_none());
    assert!(token.token_characteristics.is_none());
    assert_eq!(
        Game::effective_rules_source(token),
        ObjectCharacteristics::card(cards::HUNTMASTER_OF_THE_FELLS, back),
    );
    assert_eq!(game.permanent_mana_value(token), 4);
    let token_id = token.card.id;
    assert_eq!(
        game.observe(PlayerId::One)
            .battlefield
            .iter()
            .find(|permanent| permanent.id == token_id)
            .unwrap()
            .physical_face,
        Some(PhysicalFaceObservation {
            kind: DoubleFacedKind::Transforming,
            side: PhysicalFaceSide::Back,
        }),
    );

    game.transform_permanent(token_id);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == token_id)
        .expect("the copy-token remains");
    assert_eq!(token.presented, front);
    assert_eq!(
        Game::effective_rules_source(token),
        ObjectCharacteristics::card(cards::HUNTMASTER_OF_THE_FELLS, front),
    );
    assert_eq!(game.permanent_mana_value(token), 4);
    assert_eq!(
        game.observe(PlayerId::One)
            .battlefield
            .iter()
            .find(|permanent| permanent.id == token_id)
            .unwrap()
            .physical_face,
        Some(PhysicalFaceObservation {
            kind: DoubleFacedKind::Transforming,
            side: PhysicalFaceSide::Front,
        }),
    );
}

#[test]
fn modal_and_instant_back_faces_do_not_transform() {
    let mut game = ready_game();
    game.battlefield.clear();
    let modal = synthetic_double_faced_token(
        90_003,
        DoubleFacedKind::Modal,
        ObjectCharacteristics::card(cards::MOUNTAIN, CardPartId::PRIMARY),
    );
    let instant_back = synthetic_double_faced_token(
        90_004,
        DoubleFacedKind::Transforming,
        ObjectCharacteristics::card(cards::ANCESTRAL_RECALL, CardPartId::PRIMARY),
    );
    let ids = [modal.card.id, instant_back.card.id];
    game.battlefield.extend([modal, instant_back]);

    for id in ids {
        game.transform_permanent(id);
    }

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.presented == CardPartId::PRIMARY),
    );
}

#[test]
fn sacrificed_card_copying_token_remains_nontoken_on_the_stack() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut copier = creature(90_005, cards::COPY_ARTIFACT, PlayerId::One);
    copier.copy_effect = Some(CopiableCharacteristics {
        base: ObjectCharacteristics::token(tokens::clue(), CardPartId::PRIMARY),
        added_types: CardTypeSet::single(CardType::Enchantment),
        added_abilities: Vec::new(),
        retain_printed_subtypes: false,
    });
    let source = copier.card.id;
    game.battlefield.push(copier);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability: activated_ability_for(&game, source, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .expect("the copied Clue ability can be activated");

    let ability = game.stack.iter().next().expect("the ability is stacked");
    assert!(!game.stack_object_event_object(ability).unwrap().token);
    assert!(matches!(
        game.retired_objects.get(&source),
        Some(RetiredObject::Permanent { permanent, .. })
            if !permanent.card.definition.is_token()
    ));
}

#[test]
fn sacrificed_token_copying_card_remains_token_on_the_stack() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.create_token_copy(
        PlayerId::One,
        copied_characteristics(cards::MOGG_FANATIC),
        None,
        CardPartId::PRIMARY,
    );
    drain_pending(&mut game);
    let source = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Token)
        .expect("the Mogg Fanatic copy-token arrived")
        .card
        .id;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability: activated_ability_for(&game, source, 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .expect("the copied Mogg Fanatic ability can be activated");

    let ability = game.stack.iter().next().expect("the ability is stacked");
    assert!(game.stack_object_event_object(ability).unwrap().token);
    assert!(matches!(
        game.retired_objects.get(&source),
        Some(RetiredObject::Permanent { permanent, .. })
            if permanent.card.definition.is_token()
    ));
}
