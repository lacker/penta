use super::{
    ControlFlow, Game, Permanent, StaticAppliedEffect, StaticEffectKind, StaticEffectSource,
};

impl Game {
    pub(in crate::game) fn visit_static_applied_effects(
        &self,
        affected: &Permanent,
        kind: StaticEffectKind,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // Emblems sit outside every zone but their abilities apply, so they
        // are walked alongside the battlefield and nowhere else.
        let land_type_sources = self.land_type_effect_sources(None);
        for source in self.battlefield.iter().chain(self.emblems.iter()) {
            let source_presentation = Self::effective_rules_source(source);
            let prepared = self.prepared_static_program(source_presentation);
            if prepared.is_some_and(|program| !program.supplies(kind.prepared_lane())) {
                continue;
            }
            if self
                .visit_static_source_effects(
                    StaticEffectSource::battlefield(source, source.timestamp),
                    affected,
                    None,
                    kind,
                    &land_type_sources,
                    prepared,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        for source in self.graveyard_static_sources() {
            let source_presentation = Self::effective_rules_source(&source);
            let prepared = self.prepared_static_program(source_presentation);
            if prepared.is_some_and(|program| !program.supplies(kind.prepared_lane())) {
                continue;
            }
            if self
                .visit_static_source_effects(
                    StaticEffectSource::graveyard(&source),
                    affected,
                    None,
                    kind,
                    &land_type_sources,
                    prepared,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    pub(in crate::game) fn visit_static_applied_effects_with_prospective(
        &self,
        affected: &Permanent,
        prospective: &Permanent,
        kind: StaticEffectKind,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let prospective_source = (prospective.card.id == affected.card.id).then_some(prospective);
        let land_type_sources = self.land_type_effect_sources(prospective_source);
        for source in self.battlefield.iter().chain(prospective_source) {
            let source_presentation = Self::effective_rules_source(source);
            let prepared = self.prepared_static_program(source_presentation);
            if prepared.is_some_and(|program| !program.supplies(kind.prepared_lane())) {
                continue;
            }
            let timestamp = if prospective_source
                .is_some_and(|prospective| std::ptr::eq(source, prospective))
            {
                self.prospective_continuous_effect_timestamp()
            } else {
                source.timestamp
            };
            if self
                .visit_static_source_effects(
                    StaticEffectSource::battlefield(source, timestamp),
                    affected,
                    prospective_source,
                    kind,
                    &land_type_sources,
                    prepared,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        for source in self.graveyard_static_sources() {
            let source_presentation = Self::effective_rules_source(&source);
            let prepared = self.prepared_static_program(source_presentation);
            if prepared.is_some_and(|program| !program.supplies(kind.prepared_lane())) {
                continue;
            }
            if self
                .visit_static_source_effects(
                    StaticEffectSource::graveyard(&source),
                    affected,
                    prospective_source,
                    kind,
                    &land_type_sources,
                    prepared,
                    &mut visitor,
                )
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }
}
