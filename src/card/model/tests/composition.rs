use crate::card::{
    AbilityDef, AbilityTargetDef, CardComposition, CardRules, CardStructure, CardType,
    DoubleFacedKind, ObjectPredicateDef, PlayActionKind, PlayRestriction, SpellForm, abilities,
};
use crate::{CardPartId, PlayOptionId, TargetSlotId};

#[test]
fn single_composition_derives_one_primary_cast_option() {
    let rules = CardRules::new_instant(mana_cost!("{R}"));
    let composition = CardComposition::single("Test spell", rules);

    assert_eq!(composition.parts.len(), 1);
    assert_eq!(composition.parts[0].id, CardPartId::PRIMARY);
    assert_eq!(composition.parts[0].rules, rules);
    assert!(matches!(
        composition.structure,
        CardStructure::Single {
            main: CardPartId::PRIMARY,
        }
    ));
    assert_eq!(composition.play_options.len(), 1);
    assert_eq!(composition.play_options[0].id, PlayOptionId::DEFAULT);
    assert_eq!(
        composition.play_options[0].action,
        PlayActionKind::CastSpell
    );
    assert_eq!(
        composition.play_options[0].form,
        SpellForm::Part(CardPartId::PRIMARY),
    );
}

#[test]
fn double_faced_kind_controls_whether_the_back_face_is_playable() {
    static FACES: [(&str, CardRules); 2] = [
        ("Front", CardRules::new_instant(mana_cost!("{W}"))),
        ("Back", CardRules::new_land(&[])),
    ];

    let transforming = CardComposition::double_faced(&FACES, DoubleFacedKind::Transforming);
    assert_eq!(transforming.play_options.len(), 1);

    let modal = CardComposition::double_faced(&FACES, DoubleFacedKind::Modal);
    assert_eq!(modal.play_options.len(), 2);
    assert_eq!(modal.play_options[0].action, PlayActionKind::CastSpell);
    assert_eq!(modal.play_options[1].action, PlayActionKind::PlayLand);
    assert_eq!(modal.play_options[1].form, SpellForm::Part(CardPartId(1)),);
}

#[test]
fn fused_split_composition_combines_targets_in_printed_order() {
    static HALVES: [(&str, CardRules); 2] = [
        (
            "First",
            CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::destroy_target(
                "Destroy target creature.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                false,
            )),
        ),
        (
            "Second",
            CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
                false,
            )),
        ),
    ];
    let composition = CardComposition::split(&HALVES, Some(mana_cost!("{2}{U}{R}")));

    assert!(matches!(
        composition.structure,
        CardStructure::Split {
            ref parts,
            fused: Some(PlayOptionId(2)),
        } if parts == &[CardPartId::PRIMARY, CardPartId(1)]
    ));
    let fused = &composition.play_options[2];
    assert_eq!(fused.restriction, PlayRestriction::FromHandOnly);
    assert_eq!(
        fused.form,
        SpellForm::Combined(vec![CardPartId::PRIMARY, CardPartId(1)]),
    );
    assert_eq!(fused.targets.len(), 2);
    assert_eq!(fused.targets[0].id, TargetSlotId(0));
    assert_eq!(fused.targets[1].id, TargetSlotId(1));
}

#[test]
fn optional_additional_costs_are_not_alternative_casts() {
    let rules = CardRules::new_instant(mana_cost!("{G}"))
        .with_ability(abilities::buyback(mana_cost!("{3}")));
    let composition = CardComposition::single("Buyback test", rules);
    let option = &composition.play_options[0];

    assert!(option.alternative_costs.is_empty());
    assert_eq!(option.additional_costs.len(), 1);
    assert_eq!(option.additional_costs[0].label, "Buyback");
    assert_eq!(
        option.additional_costs[0].mana_cost,
        Some(mana_cost!("{3}"))
    );
}
