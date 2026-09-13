use super::*;
use crate::card::{BasicLandType, cards};

fn pending_land_type_substitution() -> Game {
    let mut game = crate::game::tests::ready_game();
    for (owner, definition) in [
        (PlayerId::One, cards::ISLAND),
        (PlayerId::Two, cards::ISLAND),
        (PlayerId::Two, cards::FOREST),
    ] {
        game.put_onto_battlefield(owner, definition).unwrap();
    }
    let charm = crate::game::tests::card(95_000, cards::VISION_CHARM, PlayerId::One);
    let charm_id = charm.id;
    game.players[0].hand.push(charm);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == charm_id && choices.modes() == [crate::ModeId(1)])
        })
        .expect("Vision Charm offers its land-type mode");
    game.apply(PlayerId::One, cast).unwrap();
    crate::game::tests::pass_until_decision(&mut game);
    game
}

#[test]
fn land_type_substitution_checkpoint_resumes_the_authored_choice() {
    let mut host = pending_land_type_substitution();
    let (wire, mut rebuilt) = rebuild_current_checkpoint(&host, PlayerId::One, 95_001);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    assert_eq!(
        wire["checkpoint"]["decisionState"]["continuation"]["kind"],
        "basicLandTypeSubstitution"
    );
    assert_eq!(
        wire["checkpoint"]["decisionState"]["continuation"]["continuation"]["effect"]["abilityPath"],
        json!([1]),
        "the effect belongs to Vision Charm's second mode"
    );
    let opponent = host.checkpoint_json(PlayerId::Two);
    assert_eq!(opponent["hasDeferredState"], true);
    assert!(opponent["decisionState"].is_null());

    let choice = host.observe(PlayerId::One).decision.unwrap();
    let option = choice
        .options
        .iter()
        .find(|option| option.label == "Island → Swamp")
        .unwrap()
        .id;
    for game in [&mut host, &mut rebuilt] {
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: choice.id,
                options: vec![option],
            },
        )
        .unwrap();
        crate::game::tests::drain_pending(game);
        for permanent in &game.battlefield {
            let expected = if permanent.card.definition == cards::ISLAND {
                BasicLandType::Swamp
            } else {
                BasicLandType::Forest
            };
            assert_eq!(
                game.effective_land_types(permanent),
                std::array::from_fn(|index| index == expected.index())
            );
        }
    }
    assert_eq!(
        crate::protocol::protocol_actions(&host.observe(PlayerId::One)),
        crate::protocol::protocol_actions(&rebuilt.observe(PlayerId::One))
    );
    assert_eq!(
        host.checkpoint_json(PlayerId::One),
        rebuilt.checkpoint_json(PlayerId::One),
        "both games expose the same checkpoint availability after resolution"
    );
}

#[test]
fn land_type_substitution_checkpoint_rejects_changed_options() {
    let host = pending_land_type_substitution();
    let (mut wire, _) = rebuild_current_checkpoint(&host, PlayerId::One, 95_003);
    wire["decision"]["options"][0]["label"] = json!("Island → Island");
    let error = Game::from_observation_checkpoint(
        host.catalog.clone(),
        host.format,
        &wire,
        &true_hidden_hypothesis(&host, PlayerId::One),
        95_004,
    )
    .map(|_| ())
    .expect_err("a checkpoint cannot change the authored land-type pairs");
    assert!(error.contains("land-type substitution"), "{error}");
}
