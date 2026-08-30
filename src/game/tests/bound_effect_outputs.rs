use super::*;

#[test]
fn a_skipped_bound_effect_still_declares_an_empty_output() {
    let mut game = ready_game();
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    let context = game.resolve_bound_output_effect(
        ScopedEffect::primary(EffectDef::BindOutput {
            effect: &EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceOnBattlefield,
                then: &EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
            binding: crate::card::EffectOutputBindingDef::Objects("conditional_cards"),
        }),
        &source,
        EffectResolutionContext::empty(),
    );

    assert!(
        context
            .named_object_groups()
            .contains_key("conditional_cards"),
        "the binder creates its slot before evaluating the condition"
    );
    assert!(context.named_object_group("conditional_cards").is_empty());
}
