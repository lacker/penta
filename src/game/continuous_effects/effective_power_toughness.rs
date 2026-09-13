use super::{
    ControlFlow, DeclarativeAbilityDef, Game, StaticAffectedObject, StaticAppliedEffect,
    StaticEffectKind, StaticEffectSource, StaticEffectTraversal,
};

impl Game {
    /// Add surviving layer-6 static grants to the intrinsic layer-7 walk.
    /// Intrinsic effects retain their existing traversal, including effects
    /// that already started in earlier layers (CR 613.6).
    pub(super) fn visit_granted_static_power_toughness(
        &self,
        input: StaticEffectSource<'_>,
        affected: StaticAffectedObject<'_>,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let source = input.permanent;
        let prospective = match affected {
            StaticAffectedObject::Permanent { prospective, .. } => {
                prospective.filter(|permanent| permanent.card.id == source.card.id)
            }
            StaticAffectedObject::Object { .. } => None,
        };
        self.visit_effective_granted_abilities(source, prospective, |effective, timestamp| {
            let DeclarativeAbilityDef::Static(definition) = effective.ability.definition else {
                return ControlFlow::Continue(());
            };
            if !definition.source_zones.contains(&input.zone) {
                return ControlFlow::Continue(());
            }
            let Some(effect) = effective.ability.declarative_effect() else {
                return ControlFlow::Continue(());
            };
            let mut traversal = StaticEffectTraversal {
                source,
                source_timestamp: timestamp.max(input.timestamp),
                source_presentation: Self::effective_rules_source(source),
                source_origin: effective.origin,
                text_words: self.text_word_map_for_permanent(source),
                affected,
                next_grant: 0,
                next_component_order: 0,
            };
            self.visit_static_effect(
                effect,
                &mut traversal,
                StaticEffectKind::PowerToughness,
                visitor,
            )
        })
    }
}
