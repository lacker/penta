use super::*;
use crate::{FlexibleManaPayment, ManaPaymentChoice};

/// Both optional members are absent unless the ability offers the choice
/// they answer, so an old consumer reading an ordinary mana ability sees the
/// wire shape it always saw.
#[test]
fn mana_actions_publish_their_choices_only_when_there_is_one() {
    let plain = action_json(&Action::ActivateManaAbility {
        source: GameObjectId(9),
        ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
    });
    assert!(plain.get("countersRemoved").is_none());
    assert!(
        plain.get("costObject").is_none(),
        "a Mountain sacrifices nothing",
    );

    let sacrificing = action_json(&Action::ActivateManaAbility {
        source: GameObjectId(9),
        ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
        color: ManaColor::Red,
        counters_removed: Some(2),
        cost_object: Some(GameObjectId(14)),
        combination: None,
    });
    assert_eq!(sacrificing["countersRemoved"], 2);
    assert_eq!(
        sacrificing["costObject"], 14,
        "which permanent the cost consumes is part of the action",
    );
}

#[test]
fn activated_actions_serialize_printed_and_granted_origins() {
    let mana = action_json(&Action::ActivateManaAbility {
        source: GameObjectId(9),
        ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
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
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    });
    assert_eq!(activated["ability"]["kind"], "printed");
    assert_eq!(
        activated["ability"]["definition"],
        crate::card::cards::MISHRA_S_FACTORY.get()
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
            source_definition: crate::CardDefinitionId::new(8),
            source_part: crate::CardPartId(1),
            source_ability: crate::AbilityId(2),
            grant: crate::GrantId(3),
        },
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
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
fn token_actions_serialize_their_exact_ability_origins() {
    let token = action_json(&Action::ActivateAbility {
        source: GameObjectId(13),
        ability: AbilityOrigin::Token {
            part: crate::CardPartId(1),
            ability: crate::AbilityId(4),
        },
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    });
    assert_eq!(token["ability"]["kind"], "token");
    assert_eq!(token["ability"]["partId"], 1);
    assert_eq!(token["ability"]["abilityId"], 4);
    assert!(token["ability"].get("definition").is_none());

    let token_granted = action_json(&Action::ActivateAbility {
        source: GameObjectId(14),
        ability: AbilityOrigin::TokenGranted {
            source: GameObjectId(13),
            source_part: crate::CardPartId(1),
            source_ability: crate::AbilityId(4),
            grant: crate::GrantId(5),
        },
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    });
    assert_eq!(token_granted["ability"]["kind"], "tokenGranted");
    assert_eq!(token_granted["ability"]["source"], 13);
    assert_eq!(token_granted["ability"]["sourcePartId"], 1);
    assert_eq!(token_granted["ability"]["sourceAbilityId"], 4);
    assert_eq!(token_granted["ability"]["grantId"], 5);
    assert!(token_granted["ability"].get("sourceDefinition").is_none());
}

#[test]
fn emblem_actions_serialize_their_exact_ability_origins() {
    let emblem = action_json(&Action::ActivateAbility {
        source: GameObjectId(16),
        ability: AbilityOrigin::Emblem {
            ability: crate::AbilityId(6),
        },
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    });
    assert_eq!(
        emblem["ability"],
        json!({
            "kind": "emblem",
            "abilityId": 6,
        })
    );

    let emblem_granted = action_json(&Action::ActivateAbility {
        source: GameObjectId(17),
        ability: AbilityOrigin::EmblemGranted {
            source: GameObjectId(16),
            source_ability: crate::AbilityId(6),
            grant: crate::GrantId(7),
        },
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    });
    assert_eq!(
        emblem_granted["ability"],
        json!({
            "kind": "emblemGranted",
            "source": 16,
            "sourceAbilityId": 6,
            "grantId": 7,
        })
    );
    assert!(emblem_granted["ability"].get("sourceDefinition").is_none());
    assert!(emblem_granted["ability"].get("sourcePartId").is_none());
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
    assert!(spell["choices"].get("manaPayment").is_none());
}

#[test]
fn cast_actions_publish_only_explicit_flexible_symbol_payments() {
    let life = action_json(&Action::CastSpell {
        card: GameObjectId(20),
        choices: CastChoices::default().with_mana_payment(ManaPaymentChoice::new(vec![
            FlexibleManaPayment::new(crate::FlexibleManaSymbol::RedPhyrexian, 1),
        ])),
        sacrifices: Vec::new(),
    });
    assert_eq!(life["choices"]["manaPayment"][0]["symbol"], "R/P");
    assert_eq!(life["choices"]["manaPayment"][0]["count"], 1);
    assert_eq!(life["choices"]["manaPayment"][0]["payWith"], "life");

    let generic = action_json(&Action::CastSpell {
        card: GameObjectId(21),
        choices: CastChoices::default().with_mana_payment(ManaPaymentChoice::new(vec![
            FlexibleManaPayment::new(crate::FlexibleManaSymbol::TwoBlack, 2),
        ])),
        sacrifices: Vec::new(),
    });
    assert_eq!(generic["choices"]["manaPayment"][0]["symbol"], "2/B");
    assert_eq!(generic["choices"]["manaPayment"][0]["count"], 2);
    assert_eq!(generic["choices"]["manaPayment"][0]["payWith"], "generic");
}
