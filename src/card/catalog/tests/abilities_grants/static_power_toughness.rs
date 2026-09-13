#[test]
fn catalog_accepts_granted_static_power_toughness_programs() {
    static PUMP: AbilityDef = AbilityDef::static_ability(
        "This creature gets +2/+2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
        },
    );
    CardCatalog::new([definition_granting(&PUMP)])
        .expect("the layer-7 reader visits surviving grants");
}

#[test]
fn catalog_rejects_mixed_lane_and_nonbattlefield_static_grants() {
    static MIXED: AbilityDef = AbilityDef::static_ability(
        "This object is a creature with base power and toughness 2/2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            ]),
        },
    );
    static GRAVEYARD: AbilityDef = AbilityDef::static_ability(
        "This creature card gets +2/+2 in the graveyard.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
        },
    )
    .with_source_zones(&[ZoneKind::Graveyard]);
    for ability in [&MIXED, &GRAVEYARD] {
        assert!(matches!(
            error(definition_granting(ability)),
            CatalogError::InvalidGrantedAbility { problem, .. }
                if *problem == GrantedAbilityValidationError::ExecutableStaticAbility
        ));
    }
}
