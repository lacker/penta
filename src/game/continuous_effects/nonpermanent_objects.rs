impl Game {
    /// Visits static effects from battlefield sources for a spell or a card
    /// outside the battlefield. These objects use the same declarative query
    /// vocabulary as permanents, but their characteristics are assembled as
    /// snapshots rather than through the permanent layer walk.
    pub(super) fn visit_battlefield_static_applied_effects_for_object(
        &self,
        affected: StaticAffectedObject<'_>,
        kind: StaticEffectKind,
        mut visitor: impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
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

    pub(super) fn static_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected: StaticAffectedObject<'_>,
    ) -> bool {
        let (affected_id, controller, owner, zone) = match affected {
            StaticAffectedObject::Permanent { affected, .. } => (
                affected.card.id,
                Some(affected.controller),
                affected.card.owner,
                ZoneKind::Battlefield,
            ),
            StaticAffectedObject::Object {
                characteristics,
                controller,
                owner,
                zone,
                ..
            } => (characteristics.id, controller, owner, zone),
        };
        match recipient.0 {
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::Source)) => {
                source.card.id == affected_id
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::AttachedToSource)) => {
                source.attached_to == Some(affected_id)
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
                query.zones.contains(&zone)
                    && self.query_player_constraints_match(
                        controller,
                        owner,
                        query,
                        (source.controller, source.card.id),
                        TriggerContext::empty(),
                        None,
                    )
                    && match affected {
                        StaticAffectedObject::Permanent {
                            affected,
                            prospective,
                        } => self.static_object_predicate_matches(
                            query.object,
                            source,
                            affected,
                            prospective,
                        ),
                        StaticAffectedObject::Object {
                            characteristics,
                            is_spell,
                            ..
                        } => self.trigger_object_matches(
                            query.object,
                            characteristics,
                            source.card.id,
                            is_spell,
                        ),
                    }
            }
            // None of these name a permanent a static effect could apply to;
            // a static effect has no chosen target either, and the mixed
            // recipient belongs to a resolving damage clause.
            EffectRecipientSetDef::LegalTargets(_)
            | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
            | EffectRecipientSetDef::Objects(
                ObjectSetDef::One(
                    ObjectRefDef::Binding(_)
                    | ObjectRefDef::CreatingSource
                    | ObjectRefDef::ZoneChangeSuccessor(_)
                    | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                    | ObjectRefDef::ResolvingObject
                    | ObjectRefDef::AdditionalCostObject(_)
                    | ObjectRefDef::AbilityGrantSource
                    | ObjectRefDef::Target(_)
                    | ObjectRefDef::SourceOfTargetedStackObject(_)
                    | ObjectRefDef::TriggeringObject
                    | ObjectRefDef::DamagedObject,
                )
                | ObjectSetDef::Binding(_)
                | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
                | ObjectSetDef::MatchingBinding { .. }
                | ObjectSetDef::Matching { .. }
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::PermanentsTargetedBy(_)
                | ObjectSetDef::PlayerAttachments(_)
                | ObjectSetDef::LegalAttachmentHosts(_)
                | ObjectSetDef::LinkedExiles
                | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                | ObjectSetDef::PermanentsControlledBy(_)
                | ObjectSetDef::TokensCreatedBy(_)
                | ObjectSetDef::BottomOfGraveyard(_)
                | ObjectSetDef::SharingNameWith(_)
                | ObjectSetDef::SharingNameWithIn { .. }
                | ObjectSetDef::NamesAppearingAtLeast { .. }
                | ObjectSetDef::ExceptObject { .. }
                | ObjectSetDef::SharingNameWithBinding { .. }
                | ObjectSetDef::TopOfGraveyardMatching { .. },
            )
            // A static clause names what it affects outright; nothing static
            // is scoped to what a creature happens to be attacking.
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::Players(_) => false,
        }
    }
}
