//! Cards built on the shared destruction-replacement event.

use super::*;

fn permanent_is_present(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

fn emergence_game(aura_definition: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let land = creature(20_000, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    let mut aura = creature(20_001, aura_definition, PlayerId::One);
    let aura_id = aura.card.id;
    aura.attached_to = Some(land_id);
    game.battlefield.extend([land, aura]);
    (game, land_id, aura_id)
}

#[test]
fn emergence_auras_animate_and_save_their_lands() {
    for (definition, expected_power, expected_toughness, lethal_damage) in [
        (cards::CRACKLING_EMERGENCE, 3, 3, None),
        (cards::HARMONIOUS_EMERGENCE, 4, 5, Some(5)),
    ] {
        let (mut game, land, aura) = emergence_game(definition);
        let animated = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == land)
            .expect("the enchanted land is present");
        assert_eq!(game.power(animated), Some(expected_power));
        assert_eq!(game.toughness(animated), Some(expected_toughness));

        if let Some(damage) = lethal_damage {
            game.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == land)
                .expect("the animated land is present")
                .damage = damage;
            game.check_state_based_actions();
        } else {
            game.destroy_permanent(land);
        }

        assert!(permanent_is_present(&game, land));
        assert!(!permanent_is_present(&game, aura));
        let saved = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == land)
            .expect("the replacement saved its land");
        assert!(game.has_indestructible(saved));
        assert_eq!(saved.damage, lethal_damage.unwrap_or(0));
    }
}

#[test]
fn regeneration_shield_can_be_chosen_over_emergence() {
    let (mut game, land, aura) = emergence_game(cards::CRACKLING_EMERGENCE);
    game.add_regeneration_shield(land);

    game.destroy_permanent(land);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the affected player chooses between two replacement effects");
    let shield = decision
        .options
        .iter()
        .find(|option| option.label == "Use a regeneration shield")
        .expect("the regeneration shield is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![shield],
        },
    )
    .expect("the regeneration replacement is chosen");

    let saved = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == land)
        .expect("the shield saved the land");
    assert!(saved.tapped);
    assert_eq!(saved.regeneration_shields, 0);
    assert!(permanent_is_present(&game, aura));
}

#[test]
fn mossbridge_troll_uses_other_creatures_for_its_total_power_cost() {
    let mut game = ready_game();
    let source = creature(21_000, cards::MOSSBRIDGE_TROLL, PlayerId::One);
    let source_id = source.card.id;
    let first = creature(21_001, cards::MOSSBRIDGE_TROLL, PlayerId::One);
    let first_id = first.card.id;
    let second = creature(21_002, cards::MOSSBRIDGE_TROLL, PlayerId::One);
    let second_id = second.card.id;
    game.battlefield.extend([source, first, second]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
        })
        .expect("the two other Trolls can pay the total-power cost");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");

    for payer in [first_id, second_id] {
        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("choose a payer");
        assert!(
            decision
                .options
                .iter()
                .all(|option| option.card.is_none_or(|(id, _)| id != source_id)),
            "Mossbridge Troll cannot tap itself for its own cost",
        );
        let option = decision
            .options
            .iter()
            .find(|option| option.card.is_some_and(|(id, _)| id == payer))
            .expect("the requested other creature is offered")
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .expect("the payer is chosen");
    }
    pass_priority_pair(&mut game);

    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source_id)
        .expect("Mossbridge Troll remains");
    assert_eq!(game.power(troll), Some(25));
    assert_eq!(game.toughness(troll), Some(25));
}

fn activate_pyramids_mode(
    game: &mut Game,
    pyramids: GameObjectId,
    mode_index: usize,
    target: GameObjectId,
) {
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    let mode = ModeId::from_index(mode_index).expect("one of Pyramids' two modes");
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                modes,
                targets,
                ..
            } => {
                *source == pyramids
                    && modes == &[mode]
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(target)))
            }
            _ => false,
        })
        .expect("the chosen Pyramids mode is legal");
    game.apply(PlayerId::One, action)
        .expect("Pyramids activates");
    pass_priority_pair(game);
}

#[test]
fn pyramids_destroys_an_aura_attached_to_a_land() {
    let (mut game, land, aura) = emergence_game(cards::CRACKLING_EMERGENCE);
    let pyramids = creature(22_000, cards::PYRAMIDS, PlayerId::One);
    let pyramids_id = pyramids.card.id;
    game.battlefield.push(pyramids);

    activate_pyramids_mode(&mut game, pyramids_id, 0, aura);

    assert!(!permanent_is_present(&game, aura));
    assert!(permanent_is_present(&game, land));
}

#[test]
fn pyramids_replacement_applies_only_to_the_next_destruction() {
    let mut game = ready_game();
    let land = creature(23_000, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    let pyramids = creature(23_001, cards::PYRAMIDS, PlayerId::One);
    let pyramids_id = pyramids.card.id;
    game.battlefield.extend([land, pyramids]);

    activate_pyramids_mode(&mut game, pyramids_id, 1, land_id);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == land_id)
        .expect("the land is present")
        .damage = 3;
    game.destroy_permanent(land_id);

    let saved = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == land_id)
        .expect("the next destruction is replaced");
    assert_eq!(saved.damage, 0);

    game.destroy_permanent(land_id);
    assert!(!permanent_is_present(&game, land_id));
}

#[test]
fn static_regeneration_does_not_replace_a_sacrifice() {
    let mut game = ready_game();
    let troll = creature(24_000, cards::MOSSBRIDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    game.sacrifice_permanent(troll_id);

    assert!(!permanent_is_present(&game, troll_id));
}

#[test]
fn static_regeneration_does_not_save_zero_toughness() {
    let mut game = ready_game();
    let troll = creature(25_000, cards::MOSSBRIDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    attach_constant_resolved_characteristics(
        &mut game,
        troll_id,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(0),
            ValueDef::Constant(-5),
        )],
        ContinuousEffectExpiration::Never,
    );

    game.check_state_based_actions();

    assert!(!permanent_is_present(&game, troll_id));
}
