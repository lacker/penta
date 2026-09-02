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
            binding: Binding!("conditional_cards"),
        }),
        &source,
        EffectResolutionContext::empty(),
    );

    assert!(
        context.bindings().contains_key("conditional_cards"),
        "the binder creates its slot before evaluating the condition"
    );
    assert!(
        context
            .object_group(Binding!("conditional_cards"))
            .is_empty()
    );
}

#[test]
fn mill_until_stops_when_the_accumulated_group_satisfies_its_predicate() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].library.extend([
        card(10_099, cards::PONDER, PlayerId::One),
        card(10_100, cards::MOUNTAIN, PlayerId::One),
        card(10_101, cards::PONDER, PlayerId::One),
        card(10_102, cards::PLAINS, PlayerId::One),
        card(10_103, cards::GRIZZLY_BEARS, PlayerId::One),
    ]);
    let source = spell(10_104, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    let binding = Binding!("milled_cards");

    let context = game.resolve_bound_output_effect(
        ScopedEffect::primary(EffectDef::BindOutput {
            effect: &EffectDef::MillUntil(&crate::card::MillUntilDef {
                player: EffectRecipientDef::Controller,
                until: crate::card::ObjectSetPredicateDef {
                    filter: Some(crate::card::ObjectSetFilterDef::Predicate(
                        &ObjectPredicateDef::HasType(CardType::Land),
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                matched_zone: ZoneKind::Graveyard,
            }),
            binding,
        }),
        &source,
        EffectResolutionContext::empty(),
    );

    assert_eq!(game.players[PlayerId::One.index()].library.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].graveyard.len(), 4);
    assert_eq!(context.object_group(binding).len(), 4);
}
