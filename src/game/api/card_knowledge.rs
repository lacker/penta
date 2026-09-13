use super::{Game, PlayerId};

impl Game {
    pub(in crate::game) fn card_is_known_to(
        &self,
        card: &crate::game::CardInstance,
        zone: crate::card::ZoneKind,
        viewer: PlayerId,
    ) -> bool {
        use crate::card::ZoneKind;
        if zone == ZoneKind::Library && self.library_cast_in_progress(card.owner) {
            return false;
        }
        let ordinary = match zone {
            ZoneKind::Hand => card.owner == viewer,
            ZoneKind::Library => false,
            ZoneKind::Exile => {
                !self.exiled_card_is_face_down(card.id)
                    || (card.owner == viewer && !self.exiled_card_is_hidden_from_owner(card.id))
            }
            _ => true,
        };
        if ordinary {
            return true;
        }
        self.continuously_known_cards(viewer).contains(&card.id)
    }

    fn continuously_known_cards(
        &self,
        viewer: PlayerId,
    ) -> std::collections::BTreeSet<crate::game::GameObjectId> {
        use crate::card::AppliedRuleDef;
        let mut cards = std::collections::BTreeSet::new();
        self.visit_player_static_rules_with_source(viewer, |source, rule| {
            if let AppliedRuleDef::KnownCards(query) = rule {
                let controller = self
                    .battlefield
                    .iter()
                    .chain(self.emblems.iter())
                    .find(|permanent| permanent.card.id == source)
                    .map_or(viewer, |permanent| permanent.controller);
                for target in self.objects_matching_query(
                    query,
                    controller,
                    source,
                    crate::game::TriggerContext::empty(),
                ) {
                    if let crate::Target::Card(card) = target {
                        cards.insert(card);
                    }
                }
            }
        });
        cards
    }

    pub(super) fn observed_known_cards(
        &self,
        viewer: PlayerId,
    ) -> Vec<crate::game::observation::KnownCardObservation> {
        use crate::card::ZoneKind;
        let known = self.continuously_known_cards(viewer);
        let mut cards = Vec::new();
        for owner in [PlayerId::One, PlayerId::Two] {
            for (zone, objects) in [
                (ZoneKind::Library, &self.players[owner.index()].library),
                (ZoneKind::Hand, &self.players[owner.index()].hand),
            ] {
                if (zone == ZoneKind::Hand && owner == viewer)
                    || (zone == ZoneKind::Library && self.library_cast_in_progress(owner))
                {
                    continue;
                }
                for (position_from_top, card) in objects.iter().rev().enumerate() {
                    if known.contains(&card.id) {
                        cards.push(crate::game::observation::KnownCardObservation {
                            card: card.id,
                            definition: card.definition,
                            owner,
                            zone,
                            position_from_top,
                        });
                    }
                }
            }
        }
        cards
    }

    /// CR 401.5: removing a library spell does not reveal its successor until
    /// casting has finished, including any suspended cost replacement choice.
    pub(super) fn library_cast_in_progress(&self, owner: PlayerId) -> bool {
        fn pending(completion: &crate::game::BattlefieldExitCompletion, owner: PlayerId) -> bool {
            use crate::game::BattlefieldExitCompletion as Completion;
            match completion {
                Completion::CompleteSpellCast { object, .. }
                | Completion::ContinueSpellManaPayment { object, .. } => {
                    object.controller == owner
                        && object.cast.as_ref().is_some_and(|cast| {
                            cast.source_zone == Some(crate::game::CastSourceZone::Library)
                        })
                }
                Completion::Completions(completions) => completions
                    .iter()
                    .any(|completion| pending(completion, owner)),
                Completion::ContinueBattlefieldExitReplacements { batch } => batch
                    .completion
                    .as_deref()
                    .is_some_and(|completion| pending(completion, owner)),
                _ => false,
            }
        }
        self.pending_decisions.iter().any(|decision| {
            matches!(&decision.continuation, crate::game::DecisionContinuation::BattlefieldExitReplacement { batch, .. }
                if batch.completion.as_deref().is_some_and(|completion| pending(completion, owner)))
        })
    }
}
