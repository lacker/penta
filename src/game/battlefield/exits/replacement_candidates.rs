// Replacement candidates for a proposed battlefield departure.
// Included into exits.rs to share its frozen batch representation.

impl Game {
    /// Whether the permanent about to leave is one this replacement's object
    /// predicate covers. Match the same stable owner and token properties as
    /// the nonbattlefield zone-move path.
    fn exiting_object_matches(
        &self,
        object: GameObjectId,
        controller: PlayerId,
        predicate: ObjectPredicateDef,
    ) -> bool {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        else {
            return false;
        };
        self.zone_move_object_matches(
            predicate,
            permanent.card.owner,
            permanent.card.definition.is_token(),
            controller,
        )
    }

    fn battlefield_exit_replacement_event_applies(
        &self,
        proposed: &PendingBattlefieldExitMove,
        replacement: &FrozenZoneMoveReplacement,
    ) -> bool {
        match replacement.replacement.event {
            ReplacementEventDef::WouldMove {
                from: None | Some(ZoneKind::Battlefield),
                to,
                cause: ZoneMoveCauseDef::Any,
            } => replacement.source.object == proposed.object && to == proposed.destination,
            ReplacementEventDef::AnyObjectWouldMove { object, to } => {
                to == proposed.destination
                    && self.exiting_object_matches(proposed.object, replacement.controller, object)
            }
            ReplacementEventDef::WouldBeDestroyed { object } => {
                let BattlefieldExitCause::Destroy {
                    regeneration_prohibited,
                    ..
                } = proposed.cause
                else {
                    return false;
                };
                let Some(permanent) = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == proposed.object)
                else {
                    return false;
                };
                let matches = self.trigger_object_matches_for_controller(
                    object,
                    &self.targeting_event_object(permanent),
                    replacement.source.object,
                    false,
                    Some(replacement.controller),
                );
                let regeneration_allowed = !Self::replacement_regenerates(replacement.effect)
                    || (!regeneration_prohibited
                        && !self.has_applied_rule(permanent, AppliedRuleDef::CannotRegenerate));
                matches && regeneration_allowed
            }
            _ => false,
        }
    }

    fn applicable_battlefield_exit_replacements(
        &self,
        batch: &PendingBattlefieldExitBatch,
        move_index: usize,
    ) -> Vec<ApplicableZoneMoveReplacement> {
        let proposed = &batch.moves[move_index];
        if proposed.replaced_with_nothing {
            return Vec::new();
        }
        let mut candidates = batch
            .replacements
            .iter()
            .filter(|replacement| !proposed.applied.contains(&replacement.source))
            .filter(|replacement| {
                if let Some(condition) = replacement.replacement.condition {
                    match condition {
                        ReplacementConditionDef::SourceTapped => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == replacement.source.object)
                            .is_some_and(|permanent| permanent.tapped),
                        ReplacementConditionDef::CreatureDiedThisTurn => {
                            self.creature_died_this_turn
                        }
                        // How a permanent's spell was paid for is asked as
                        // it enters, and hand or library size as a draw would
                        // happen; none is a question about leaving.
                        ReplacementConditionDef::SourceCastWith(_)
                        | ReplacementConditionDef::SourcePaidAdditionalCost(_)
                        | ReplacementConditionDef::SourceNotCastFrom(_)
                        | ReplacementConditionDef::OpponentWasDealtDamageThisTurn
                        | ReplacementConditionDef::ControllerHandAtMost(_)
                        | ReplacementConditionDef::ControllerLibraryAtLeast(_)
                        | ReplacementConditionDef::ControllerLibraryEmpty => false,
                    }
                } else {
                    true
                }
            })
            .filter(|replacement| {
                self.battlefield_exit_replacement_event_applies(proposed, replacement)
            })
            .map(|replacement| ApplicableZoneMoveReplacement {
                move_index,
                presentation: replacement.presentation,
                text: replacement.text,
                action: BattlefieldExitReplacementAction::Ability {
                    context: ReplacementEffectContext {
                        source: replacement.source,
                        controller: replacement.controller,
                    },
                    effect: replacement.effect,
                    once: replacement.replacement.once,
                },
            })
            .collect::<Vec<_>>();
        if let BattlefieldExitCause::Destroy {
            regeneration_prohibited: false,
            ..
        } = proposed.cause
            && let Some(permanent) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == proposed.object)
            && permanent.regeneration_shields > 0
            && !self.has_applied_rule(permanent, AppliedRuleDef::CannotRegenerate)
        {
            candidates.push(ApplicableZoneMoveReplacement {
                move_index,
                presentation: Self::effective_rules_source(permanent),
                text: "Use a regeneration shield",
                action: BattlefieldExitReplacementAction::RegenerationShield,
            });
        }
        if let BattlefieldExitCause::Destroy {
            by_effect: true, ..
        } = proposed.cause
            && let Some(permanent) = self
                .battlefield
                .iter()
                .find(|p| p.card.id == proposed.object)
            && permanent.counters(CounterKind::Shield) > 0
        {
            candidates.push(ApplicableZoneMoveReplacement {
                move_index,
                presentation: Self::effective_rules_source(permanent),
                text: "Remove a shield counter instead of destroying this permanent",
                action: BattlefieldExitReplacementAction::ShieldCounter,
            });
        }
        if !proposed.commander_considered
            && matches!(proposed.destination, ZoneKind::Hand | ZoneKind::Library)
            && self.is_commander(proposed.object)
            && let Some(permanent) = self
                .battlefield
                .iter()
                .find(|p| p.card.id == proposed.object)
        {
            candidates.push(ApplicableZoneMoveReplacement {
                move_index,
                presentation: Self::effective_rules_source(permanent),
                text: "Choose whether to return the commander to the command zone",
                action: BattlefieldExitReplacementAction::Commander,
            });
        }
        candidates
    }
}
