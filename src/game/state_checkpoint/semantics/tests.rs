use super::*;
use crate::card::{
    EffectRecipientDef, ObjectPredicateDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    TokenCharacteristics, ValueDef,
};
use crate::{ObjectBindingIndex, ObjectSetBindingIndex};

static GRANTED: AbilityDef = AbilityDef::not_implemented(
    "A nested ability.",
    "Only structural checkpoint traversal matters in this fixture.",
);
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
static CREATE_TOKEN: EffectDef = EffectDef::CreateToken {
    token: NESTED_TOKEN,
    copy: None,
    controller: None,
    count: ValueDef::Constant(1),
    tapped: false,
    attacking: false,
    counters: None,
    created: None,
};
static MILL_THEN: EffectDef = EffectDef::Sequence(&[
    EffectDef::Mill {
        player: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        binding: None,
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
    object: EffectRecipientDef::object(crate::card::ObjectRefDef::Binding(
        ObjectBindingIndex::PRIMARY,
    )),
    binding: ObjectSetBindingIndex::PRIMARY,
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
        assert!(locate_effect(*root, CREATE_TOKEN, &mut path));
        assert_eq!(path, expected_path);

        let mut rebuilt = *root;
        for index in path {
            rebuilt = child_effects(rebuilt)[index];
        }
        assert_eq!(rebuilt, CREATE_TOKEN);
    }
}
