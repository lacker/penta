use super::*;

#[test]
fn activated_actions_serialize_their_exact_ability_origin() {
    let mana = action_json(&Action::ActivateManaAbility {
        source: GameObjectId(9),
        ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
        color: ManaColor::Red,
    });
    assert_eq!(mana["ability"]["kind"], "intrinsicBasicLand");
    assert_eq!(mana["ability"]["landType"], "mountain");

    let activated = action_json(&Action::ActivateAbility {
        source: GameObjectId(10),
        ability: AbilityOrigin::Printed {
            definition: crate::card::cards::MISHRA_S_FACTORY,
            part: crate::CardPartId::PRIMARY,
            ability: crate::AbilityId(2),
        },
        targets: vec![
            crate::TargetSelection::single(crate::TargetSlotId(3), Target::Player(PlayerId::Two)),
            crate::TargetSelection::single(
                crate::TargetSlotId(7),
                Target::Permanent(GameObjectId(11)),
            ),
        ],
        cost_object: None,
        x: 0,
    });
    assert_eq!(activated["ability"]["kind"], "printed");
    assert_eq!(
        activated["ability"]["definition"],
        crate::card::cards::MISHRA_S_FACTORY.0
    );
    assert_eq!(activated["ability"]["partId"], 0);
    assert_eq!(activated["ability"]["abilityId"], 2);
    assert_eq!(activated["target"]["type"], "player");
    assert_eq!(activated["targets"].as_array().unwrap().len(), 2);
    assert_eq!(activated["targetSelections"][0]["slotId"], 3);
    assert_eq!(activated["targetSelections"][1]["slotId"], 7);
    assert_eq!(
        activated["targetSelections"][1]["targets"][0]["objectId"],
        11
    );

    let granted = action_json(&Action::ActivateAbility {
        source: GameObjectId(12),
        ability: AbilityOrigin::Granted {
            source: GameObjectId(9),
            source_definition: crate::CardDefinitionId(8),
            source_part: crate::CardPartId(1),
            source_ability: crate::AbilityId(2),
            grant: crate::GrantId(3),
        },
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    });
    assert_eq!(granted["ability"]["kind"], "granted");
    assert_eq!(granted["ability"]["source"], 9);
    assert_eq!(granted["ability"]["sourceDefinition"], 8);
    assert_eq!(granted["ability"]["sourcePartId"], 1);
    assert_eq!(granted["ability"]["sourceAbilityId"], 2);
    assert_eq!(granted["ability"]["grantId"], 3);
    assert!(granted["ability"].get("abilityId").is_none());
}

#[test]
fn special_actions_serialize_their_source_and_exact_ability_origin() {
    let ordinary = action_json(&Action::TakeSpecialAction {
        source: GameObjectId(22),
        ability: AbilityOrigin::Printed {
            definition: crate::CardDefinitionId(17),
            part: crate::CardPartId(2),
            ability: crate::AbilityId(4),
        },
        effect_id: None,
    });
    let effect_scoped = action_json(&Action::TakeSpecialAction {
        source: GameObjectId(23),
        ability: AbilityOrigin::Printed {
            definition: crate::CardDefinitionId(17),
            part: crate::CardPartId(2),
            ability: crate::AbilityId(4),
        },
        effect_id: Some(91),
    });

    assert_eq!(ordinary["type"], "TakeSpecialAction");
    assert_eq!(ordinary["source"], 22);
    assert_eq!(ordinary["ability"]["kind"], "printed");
    assert_eq!(ordinary["ability"]["definition"], 17);
    assert_eq!(ordinary["ability"]["partId"], 2);
    assert_eq!(ordinary["ability"]["abilityId"], 4);
    assert!(ordinary["effectId"].is_null());

    assert_eq!(effect_scoped["source"], 23);
    assert_eq!(effect_scoped["effectId"], 91);
}

#[test]
fn action_json_locks_play_option_modes_costs_x_and_target_slots() {
    let land = action_json(&Action::PlayLand {
        card: GameObjectId(10),
        option: crate::PlayOptionId(9),
    });
    assert_eq!(land["card"], 10);
    assert_eq!(land["playOptionId"], 9);

    let spell = action_json(&Action::CastSpell {
        card: GameObjectId(11),
        choices: structured_choices(),
        sacrifices: vec![GameObjectId(12)],
    });
    assert_eq!(spell["card"], 11);
    assert_eq!(spell["playOptionId"], 2);
    assert_eq!(spell["choices"]["modeIds"], json!([3, 1]));
    assert_eq!(spell["choices"]["alternativeCostId"], 4);
    assert_eq!(spell["choices"]["additionalCostIds"], json!([5]));
    assert_eq!(spell["choices"]["x"], 6);
    assert_eq!(spell["choices"]["targetSelections"][0]["slotId"], 7);
    assert_eq!(
        spell["choices"]["targetSelections"][0]["targets"][0]["objectId"],
        20
    );
    assert_eq!(
        spell["choices"]["targetSelections"][1]["targets"][0]["objectId"],
        21
    );
    assert_eq!(spell["sacrifices"], json!([12]));
}
