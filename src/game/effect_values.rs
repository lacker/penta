use super::{
    EffectRecipientDef, Game, PlayerId, ScopedEffect, StackObject, Target, TriggerContext, ValueDef,
};

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn effect_value(
        &self,
        value: ValueDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) -> i32 {
        match value {
            ValueDef::Constant(value) => value,
            ValueDef::ChosenX => i32::from(object.x()),
            ValueDef::SourcePower => object
                .source
                .and_then(|source| self.current_or_last_known_power(source))
                .map_or(0, i32::from),
            ValueDef::SourceToughness => object
                .source
                .and_then(|source| self.current_or_last_known_toughness(source))
                .map_or(0, i32::from),
            ValueDef::TriggerEventAmount => context.amount.unwrap_or(0),
            // Resolved per target by the divided-damage path; anything else
            // reading it has no target in hand and so no share.
            ValueDef::DividedAmongTargets => 0,
            ValueDef::TargetPower(target) => {
                Self::chosen_targets(object, scoped.target_slot(target))
                    .find_map(|target| match target {
                        Target::Permanent(id) => self.current_or_last_known_power(id),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .map_or(0, i32::from)
            }
            ValueDef::TargetManaValue(target) => {
                Self::chosen_targets(object, scoped.target_slot(target))
                    .find_map(|target| match target {
                        Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                            self.current_or_last_known_mana_value(id)
                        }
                        Target::Player(_) => None,
                    })
                    .map_or(0, i32::from)
            }
            ValueDef::CountersOnSource(kind) => object.source.map_or(0, |source| {
                i32::from(self.current_or_last_known_counters(source, kind))
            }),
            ValueDef::CardsInHandAbove { player, threshold } => {
                let player = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .find(|candidate| {
                        self.player_relation_matches(*candidate, player, object.controller, context)
                    })
                    .unwrap_or(object.controller);
                i32::try_from(
                    self.players[player.index()]
                        .hand
                        .len()
                        .saturating_sub(usize::from(threshold)),
                )
                .unwrap_or(i32::MAX)
            }
            ValueDef::CountMatchingObjects(query) => {
                let recipient = EffectRecipientDef::MatchingObjects {
                    object: query.object,
                    zones: query.zones,
                    controller: query.controller,
                };
                i32::try_from(
                    self.effect_recipients(recipient, object, context, scoped)
                        .len(),
                )
                .unwrap_or(i32::MAX)
            }
            ValueDef::AnyMatchingObject(query) => i32::from(self.any_battlefield_object_matches(
                query,
                object.source.unwrap_or(object.id),
                object.controller,
            )),
            ValueDef::IfTargetMatches(_)
            | ValueDef::IfMatchingObjectCount(_)
            | ValueDef::IfCreatureDiedThisTurn(_) => {
                self.conditional_effect_value(value, object, context, scoped)
            }
            ValueDef::Negate(inner) => self
                .effect_value(*inner, object, context, scoped)
                .saturating_neg(),
        }
    }

    /// Resolve values that select between two branches separately from the
    /// direct value forms above.
    fn conditional_effect_value(
        &self,
        value: ValueDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) -> i32 {
        match value {
            ValueDef::IfTargetMatches(condition) => {
                let source = object.source.unwrap_or(object.id);
                let matched = self
                    .effect_recipients(
                        EffectRecipientDef::Target(condition.slot),
                        object,
                        context,
                        scoped,
                    )
                    .into_iter()
                    .any(|target| match target {
                        Target::Card(id) => {
                            self.card_in_nonbattlefield_zone(id)
                                .is_some_and(|(zone, card)| {
                                    self.card_object_matches(condition.object, card, zone, source)
                                })
                        }
                        Target::Permanent(id) => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                            .is_some_and(|permanent| {
                                self.trigger_object_matches(
                                    condition.object,
                                    &self.trigger_event_object(permanent),
                                    source,
                                    false,
                                )
                            }),
                        Target::Player(_) | Target::Spell(_) => false,
                    });
                let chosen = if matched {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            ValueDef::IfMatchingObjectCount(condition) => {
                let count = self.effect_value(
                    ValueDef::CountMatchingObjects(&condition.query),
                    object,
                    context,
                    scoped,
                );
                let chosen = if count == i32::from(condition.equals) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            ValueDef::IfCreatureDiedThisTurn(branches) => {
                let chosen = if self.creature_died_this_turn {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            // The caller only routes conditional values here.
            _ => 0,
        }
    }
}
