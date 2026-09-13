use super::*;
use crate::card::{
    EffectRecipientDef, ObjectPredicateDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    TokenCharacteristics, ValueDef,
};

static GRANTED: AbilityDef = AbilityDef::static_ability("A nested ability.", EffectDef::None);
static APPLIED: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&GRANTED),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
];
static PERFORM: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&APPLIED),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};
static PROGRAM: [ReplacementEffectDef; 1] = [ReplacementEffectDef::Perform(&PERFORM)];
static OUTER: AbilityDef = AbilityDef::replacement(
    "Perform nested definitions while replacing an event.",
    ReplacementEffectDef::Sequence(&PROGRAM),
);

static NESTED_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Test"], &[], 1, 1).with_name("Nested Walker Test");
static CREATE_TOKEN: EffectDef = EffectDef::CreateToken(crate::card::CreateTokenDef::new(
    crate::card::TokenDef::Literal(NESTED_TOKEN),
));
static MILL_THEN: EffectDef = EffectDef::Sequence(&[
    EffectDef::Mill {
        player: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    CREATE_TOKEN,
]);
static EXILE_OTHERWISE: EffectDef = EffectDef::ExileTopAndMayCast {
    player: EffectRecipientDef::Controller,
    otherwise: Some(&CREATE_TOKEN),
};
static SACRIFICE_THEN_AND_OTHERWISE: EffectDef = EffectDef::SacrificeOfChoice {
    count: ValueDef::Constant(1),
    player: EffectRecipientDef::Controller,
    object: ObjectPredicateDef::Any,
    then: Some(&EffectDef::None),
    amount: SacrificedAmountDef::Power,
    otherwise: Some(&CREATE_TOKEN),
    optional: true,
};
static RETURN_THEN: EffectDef = EffectDef::PutOntoBattlefieldThen {
    object: EffectRecipientDef::object(crate::card::ObjectRefDef::Binding(Binding!("object"))),
    binding: Binding!("objects"),
    counters: None,
    then: &CREATE_TOKEN,
};

#[test]
fn checkpoint_semantic_walkers_descend_replacement_programs() {
    assert_eq!(child_abilities(&OUTER), vec![&GRANTED]);
    assert!(applied_effects(&OUTER).contains(&APPLIED[1]));
}

#[test]
fn recursive_effect_children_round_trip_all_continuation_branches() {
    let cases = [
        (&MILL_THEN, vec![1]),
        (&EXILE_OTHERWISE, vec![0]),
        (&SACRIFICE_THEN_AND_OTHERWISE, vec![1]),
        (&RETURN_THEN, vec![0]),
    ];
    for (root, expected_path) in cases {
        let mut path = Vec::new();
        assert!(locate_effect(
            *root,
            crate::game::EffectLocalRules::default(),
            None,
            ScopedEffect::primary(CREATE_TOKEN),
            &mut path,
        ));
        assert_eq!(path, expected_path);

        let mut rebuilt = *root;
        for index in path {
            rebuilt = child_effects(rebuilt)[index];
        }
        assert_eq!(rebuilt, CREATE_TOKEN);
    }
}

#[test]
fn scoped_effect_ability_paths_preserve_mode_scope_and_existing_root_snapshots() {
    use crate::CardDefinitionId;
    use crate::card::{CardDefinition, CardRules, sets};

    static DESTROY: EffectDef = EffectDef::Destroy {
        object: EffectRecipientDef::Source,
        then: None,
    };
    static MODES: [AbilityDef; 1] = [AbilityDef::spell(
        "Destroy without regeneration.",
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &DESTROY,
        },
    )];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::modal_spell("Choose one.", &MODES)];

    let definition = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Scoped Modal Checkpoint",
        sets::alpha::SET,
        CardRules::new_instant(crate::ManaCost::new(1, 0)).with_abilities(&ABILITIES),
    );
    let catalog = CardCatalog::new([definition]).expect("the modal catalog validates");
    let outer = ability_locator(&catalog, |candidate| *candidate == ABILITIES[0]).unwrap();
    let mode = ability_locator(&catalog, |candidate| *candidate == MODES[0]).unwrap();
    let scoped = ScopedEffect::at(DESTROY, 3).with_rule(AppliedRuleDef::CannotRegenerate);

    let mut snapshot = scoped_effect_snapshot(&ABILITIES[0], scoped).unwrap();
    assert_eq!(snapshot.ability_path, vec![0]);
    assert_eq!(snapshot.path, vec![0]);
    assert_eq!(snapshot.target_base, 3);
    assert_eq!(
        catalog_scoped_effect(&catalog, &outer, &snapshot),
        Some(scoped)
    );
    snapshot.ability_path[0] = 99;
    assert!(catalog_scoped_effect(&catalog, &outer, &snapshot).is_none());

    // Locating directly from the mode preserves the preexisting effect path
    // encoding. An older snapshot without abilityPath still decodes identically.
    let root = scoped_effect_snapshot(&MODES[0], scoped).unwrap();
    let wire = serde_json::to_value(&root).unwrap();
    assert_eq!(wire, serde_json::json!({ "path": [0], "targetBase": 3 }));
    let restored: ScopedEffectSnapshot = serde_json::from_value(wire).unwrap();
    assert!(restored.ability_path.is_empty());
    assert_eq!(
        catalog_scoped_effect(&catalog, &mode, &restored),
        Some(scoped)
    );
}
