use super::*;
use crate::action_view::{action_target_selections, animated_action_kind};

#[test]
fn woe_hob_prepared_copy_uses_the_inset_frame_name() {
    let game = WebGame::new(
        "The Deck",
        "Goblins",
        "Handcrafted",
        true,
        2,
        Some("old-school-93-94".into()),
    )
    .unwrap();
    let mut observation = game.session.engine().observe(game.human);
    let copy = CardInstanceId(90_111);
    observation.exiles[game.human.index()].push((copy, penta::card::cards::EMERITUS_OF_IDEATION));
    observation
        .exiled_part_copies
        .push((copy, penta::CardPartId(1)));
    let cast = Action::CastSpell {
        card: copy,
        choices: penta::CastChoices::new(penta::PlayOptionId(1)),
        sacrifices: Vec::new(),
    };
    assert_eq!(game.instance_name(&observation, copy), "Ancestral Recall");
    assert_eq!(
        game.action_label(&observation, &cast),
        "Cast Ancestral Recall"
    );
}

#[test]
fn woe_hob_alternative_equip_payment_retains_source_targets_and_animation() {
    let mut game = WebGame::new(
        "The Deck",
        "Goblins",
        "Handcrafted",
        true,
        2,
        Some("old-school-93-94".into()),
    )
    .unwrap();
    let kili = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::KILI_THE_RESOURCEFUL)
        .unwrap();
    let equip = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::BONESPLITTER)
        .unwrap();
    let observation = game.session.engine().observe(game.human);
    let targets = vec![penta::TargetSelection::single(
        penta::TargetSlotId(0),
        Target::Permanent(kili),
    )];
    let action = Action::ActivateAbilityWithAlternativeCost {
        cost: penta::AlternativeAbilityCost {
            source: kili,
            ability: AbilityOrigin::Printed {
                definition: penta::card::cards::KILI_THE_RESOURCEFUL,
                part: penta::CardPartId::PRIMARY,
                ability: penta::AbilityId(1),
            },
        },
        source: equip,
        ability: AbilityOrigin::Printed {
            definition: penta::card::cards::BONESPLITTER,
            part: penta::CardPartId::PRIMARY,
            ability: penta::AbilityId(1),
        },
        targets: targets.clone(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert_eq!(animated_action_kind(&action), "ability");
    assert_eq!(
        action_target_selections(&action, game.human)[0]["targetCardIds"],
        json!([kili.0])
    );
    assert!(
        game.action_label(&observation, &action)
            .contains("alternative cost from Kíli the Resourceful")
    );
    assert!(
        game.action_ability_label(&observation, &action)
            .unwrap()
            .contains("alternative cost from Kíli the Resourceful")
    );
}
