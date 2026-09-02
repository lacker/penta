use super::{
    CardType, CardTypeSet, CostConfiguration, EffectRecipientDef, EffectResolutionContext, Game,
    GameObjectId, ObjectPredicateDef, PlayOptionDef, PlayerId, PlayerRelation, RetiredObject,
    ScopedEffect, StackObject, Target, TriggerContext, ValueDef,
};
use crate::card::SpellCastQueryDef;

/// How many symbols of one colour a printed mana cost carries. A hybrid
/// symbol counts once for each colour it offers, which is what makes a
/// permanent cast for one colour still count toward the other's devotion.
fn devotion_symbols(cost: crate::card::ManaCost, color: crate::card::ManaColor) -> u16 {
    let plain = match color {
        crate::card::ManaColor::White => cost.white,
        crate::card::ManaColor::Blue => cost.blue,
        crate::card::ManaColor::Black => cost.black,
        crate::card::ManaColor::Red => cost.red,
        crate::card::ManaColor::Green => cost.green,
        // Colourless is a mana type rather than a colour, so nothing is
        // devoted to it.
        crate::card::ManaColor::Colorless => return 0,
    };
    crate::card::FlexibleManaSymbol::ALL
        .into_iter()
        .filter(|symbol| symbol.contains_color(color))
        .map(|symbol| cost.flexible_count(symbol))
        .fold(plain, u16::saturating_add)
}

impl Game {
    pub(super) fn additional_cost_payment_counts_for(
        option: &PlayOptionDef,
        costs: &CostConfiguration,
    ) -> Vec<u16> {
        option
            .additional_costs
            .iter()
            .map(|cost| {
                u16::try_from(
                    costs
                        .additional()
                        .iter()
                        .filter(|paid| **paid == cost.id)
                        .count(),
                )
                .unwrap_or(u16::MAX)
            })
            .collect()
    }

    /// Payment counts for every optional additional cost on a spell, in the
    /// play option's declarative order.
    pub(super) fn additional_cost_payment_counts(&self, spell: GameObjectId) -> Vec<u16> {
        let object = self
            .stack
            .iter()
            .find(|candidate| candidate.id == spell)
            .or_else(|| match self.retired_objects.get(&spell) {
                Some(RetiredObject::Stack(object)) => Some(object),
                Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
            });
        let Some(signature) = object.and_then(|object| object.signature.as_ref()) else {
            return Vec::new();
        };
        let Some(option) = object
            .and_then(|object| object.card.definition.card_definition())
            .and_then(|definition| self.catalog.get(definition))
            .and_then(|definition| definition.play_option(signature.play_option()))
        else {
            return Vec::new();
        };
        Self::additional_cost_payment_counts_for(option, signature.costs())
    }

    pub(super) fn additional_cost_payment_count(
        &self,
        spell: GameObjectId,
        cost: crate::AdditionalCostIndex,
    ) -> u16 {
        self.additional_cost_payment_counts(spell)
            .get(cost.index())
            .copied()
            .unwrap_or(0)
    }

    pub(super) fn source_additional_cost_payments(
        &self,
        source: GameObjectId,
        cost: crate::AdditionalCostIndex,
    ) -> u16 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.cast.as_ref())
            .and_then(|cast| cast.additional_costs.get(cost.index()).copied())
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Permanent { permanent, .. }) => permanent
                    .cast
                    .as_ref()
                    .and_then(|cast| cast.additional_costs.get(cost.index()).copied()),
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
            .unwrap_or_else(|| self.additional_cost_payment_count(source, cost))
    }

    /// How many times a spell paid a repeatable optional additional cost.
    /// The cast's record lists one entry per payment, so this is how many of
    /// those entries name a cost the card lets you pay more than once.
    pub(super) fn repeatable_additional_cost_payments_for(
        option: &PlayOptionDef,
        costs: &CostConfiguration,
    ) -> u16 {
        u16::try_from(
            costs
                .additional()
                .iter()
                .filter(|paid| {
                    option
                        .additional_costs
                        .iter()
                        .any(|cost| cost.id == **paid && cost.repeatable)
                })
                .count(),
        )
        .unwrap_or(u16::MAX)
    }

    pub(super) fn spells_cast_matching_this_turn(
        &self,
        query: SpellCastQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> i32 {
        let count =
            self.spell_cast_history_this_turn
                .iter()
                .filter_map(|id| {
                    self.stack
                        .iter()
                        .find(|object| object.id == *id)
                        .or_else(|| match self.retired_objects.get(id) {
                            Some(RetiredObject::Stack(object)) => Some(object.as_ref()),
                            Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. })
                            | None => None,
                        })
                })
                .filter(|spell| {
                    self.player_relation_matches(
                        spell.controller,
                        query.player,
                        evaluation_controller,
                        context,
                    )
                })
                .filter_map(|spell| self.stack_trigger_event_object(spell))
                .filter(|spell| {
                    self.trigger_object_matches_for_controller(
                        query.spell,
                        spell,
                        source,
                        true,
                        Some(evaluation_controller),
                    )
                })
                .count();
        i32::try_from(count).unwrap_or(i32::MAX)
    }

    /// The values that read a player rather than an object: what they are
    /// devoted to, and how much library they have left. Shared by the
    /// resolving path and by the conditions that compare two of them.
    pub(super) fn player_readable_value(
        &self,
        value: crate::card::ValueDef,
        controller: crate::ids::PlayerId,
    ) -> i32 {
        match value {
            // Read live off the board, and off the permanents' printed mana
            // costs -- a copy effect that changes what a permanent is
            // changes what it contributes with it.
            crate::card::ValueDef::DevotionTo(color) => self
                .battlefield
                .iter()
                .filter(|permanent| permanent.controller == controller)
                .filter_map(|permanent| self.effective_rules(permanent))
                .filter_map(|rules| rules.mana_cost())
                .map(|cost| i32::from(devotion_symbols(cost, color)))
                .sum(),
            crate::card::ValueDef::BasicLandTypesControlled(relation) => {
                let mut found = [false; crate::card::BasicLandType::ALL.len()];
                for permanent in self.battlefield.iter().filter(|permanent| {
                    self.player_relation_matches(
                        permanent.controller,
                        relation,
                        controller,
                        crate::game::TriggerContext::empty(),
                    )
                }) {
                    for subtype in self.effective_subtypes(permanent).iter() {
                        if let Some(basic) = crate::card::BasicLandType::from_subtype(subtype) {
                            found[basic.index()] = true;
                        }
                    }
                }
                i32::try_from(found.into_iter().filter(|seen| *seen).count()).unwrap_or(0)
            }
            // A whole-game tally rather than a zone count, but read the
            // same way: whichever players the relation names, added up.
            crate::card::ValueDef::SpellsCastThisGame(relation) => {
                [crate::ids::PlayerId::One, crate::ids::PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        self.player_relation_matches(
                            *player,
                            relation,
                            controller,
                            crate::game::TriggerContext::empty(),
                        )
                    })
                    .map(|player| i32::from(self.total_spells_cast[player.index()]))
                    .sum()
            }
            crate::card::ValueDef::LibrarySize(relation) => {
                [crate::ids::PlayerId::One, crate::ids::PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        self.player_relation_matches(
                            *player,
                            relation,
                            controller,
                            crate::game::TriggerContext::empty(),
                        )
                    })
                    .map(|player| {
                        i32::try_from(self.players[player.index()].library.len())
                            .unwrap_or(i32::MAX)
                    })
                    .sum()
            }
            _ => 0,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn effect_value(
        &self,
        value: ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> i32 {
        match value {
            ValueDef::Constant(value) => value,
            ValueDef::CreaturesDiedThisTurn => i32::from(self.creatures_died_this_turn),
            // A count of players, not of life: the clause asks how many
            // opponents lost any at all.
            ValueDef::OpponentsWhoLostLifeThisTurn => {
                i32::from(self.lost_life_this_turn[object.controller.opponent().index()])
            }
            ValueDef::CardTypesAmongGraveyards(player) => {
                self.card_types_among_graveyards(player, object.controller)
            }
            ValueDef::IfCardTypesAmongGraveyards(condition) => {
                let held = self.card_types_among_graveyards(condition.player, object.controller);
                let branch = if held >= i32::from(condition.minimum) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(branch, object, context, scoped)
            }
            ValueDef::ChosenX => i32::from(object.x()),
            ValueDef::SourcePower => object
                .source
                .and_then(|source| self.current_or_last_known_power(source))
                .map_or(0, i32::from),
            ValueDef::SourceToughness => object
                .source
                .and_then(|source| self.current_or_last_known_toughness(source))
                .map_or(0, i32::from),
            ValueDef::TriggerEventAmount => context.trigger.amount.unwrap_or(0),
            // A pile the player keeps rather than one on a permanent, so
            // nothing on the battlefield has to be found to read it.
            ValueDef::PlayerCounters { player, kind } => {
                [crate::ids::PlayerId::One, crate::ids::PlayerId::Two]
                    .into_iter()
                    .filter(|seat| {
                        self.player_relation_matches(
                            *seat,
                            player,
                            object.controller,
                            crate::game::TriggerContext::empty(),
                        )
                    })
                    .map(|seat| i32::from(self.players[seat.index()].counters.count(kind)))
                    .sum()
            }
            // Frozen when the cost was paid: the permanents it names are
            // already in a graveyard by the time anything reads this.
            ValueDef::SacrificedManaValue => object
                .ability
                .as_ref()
                .map_or(0, |ability| i32::from(ability.sacrificed_mana_value)),
            // Resolved per target by the divided-damage path; anything else
            // reading it has no target in hand and so no share.
            // Neither has an answer while an effect resolves: nothing is
            // being divided, and only the static power-and-toughness layer
            // has an affected object whose cost it could read.
            // None of these has an answer while an effect resolves: nothing
            // is being divided, and only the static power-and-toughness
            // layer has an affected object or a source pile to read.
            ValueDef::DistinctTargets
            | ValueDef::DividedAmongTargets
            | ValueDef::ResolvedRecipientCount
            | ValueDef::AffectedManaValue
            | ValueDef::AffectedColorCount => 0,
            ValueDef::SourceCastX => self
                .battlefield
                .iter()
                .find(|permanent| Some(permanent.card.id) == object.source)
                .map_or(0, |permanent| {
                    i32::from(permanent.cast.as_ref().map_or(0, |cast| cast.x))
                }),
            ValueDef::TriggeringObjectPower => context
                .trigger
                .object
                .and_then(|object| self.current_or_last_known_power(object))
                .map_or(0, i32::from),
            ValueDef::TriggeringObjectToughness => context
                .trigger
                .object
                .and_then(|object| self.current_or_last_known_toughness(object))
                .map_or(0, i32::from),
            ValueDef::TargetPower(target) => {
                Self::chosen_targets(object, scoped.target_slot(target))
                    .find_map(|target| match target {
                        Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                            self.current_or_last_known_power(id)
                        }
                        Target::Player(_) => None,
                    })
                    .map_or(0, i32::from)
            }
            ValueDef::TargetToughness(target) => {
                Self::chosen_targets(object, scoped.target_slot(target))
                    .find_map(|target| match target {
                        Target::Permanent(id) => self.current_or_last_known_toughness(id),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .map_or(0, i32::from)
            }
            // A rule of the format rather than anything on the board, so
            // the relation only says whose game it is and every player's
            // answer is the same one.
            ValueDef::StartingLifeTotal(_) => i32::from(self.format.rules().starting_life),
            ValueDef::LifeTotal(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .find(|candidate| {
                    self.player_relation_matches(
                        *candidate,
                        relation,
                        object.controller,
                        context.trigger,
                    )
                })
                .map_or(0, |player| i32::from(self.players[player.index()].life)),
            ValueDef::TargetLibrarySize(target) => {
                Self::chosen_targets(object, scoped.target_slot(target))
                    .find_map(|target| match target {
                        Target::Player(player) => {
                            i32::try_from(self.players[player.index()].library.len()).ok()
                        }
                        Target::Permanent(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .unwrap_or(0)
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
            // Characteristic reads resolve a general object reference, then
            // consult the live object or its last-known information.
            ValueDef::ObjectPower(reference) => self
                .effect_object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_power(referenced))
                .map_or(0, i32::from),
            ValueDef::ObjectManaValue(reference) => self
                .effect_object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_mana_value(referenced))
                .map_or(0, i32::from),
            // Zero without a payment behind it: a branch that reads this
            // outside a chosen-amount payment does nothing rather than
            // guessing at a number.
            ValueDef::PaidAmount => i32::from(context.paid_amount.unwrap_or(0)),
            // Read off the object that was cast. An ability, a token, or a
            // copy has nothing spent on it and counts zero, which is what
            // converge on a copied spell means.
            ValueDef::ColorsOfManaSpent => i32::from(object.colors_spent_count()),
            ValueDef::DevotionTo(_)
            | ValueDef::LibrarySize(_)
            | ValueDef::SpellsCastThisGame(_)
            | ValueDef::BasicLandTypesControlled(_) => {
                self.player_readable_value(value, object.controller)
            }
            ValueDef::CardsDrawnThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        object.controller,
                        context.trigger,
                    )
                })
                .map(|player| i32::from(self.cards_drawn_this_turn[player.index()]))
                .sum(),
            // The same shape again, over the land drops a player has taken
            // this turn.
            ValueDef::LandsPlayedThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        object.controller,
                        context.trigger,
                    )
                })
                .map(|player| i32::from(self.players[player.index()].lands_played_this_turn))
                .sum(),
            // The same shape as the draw tally beside it, over the running
            // total of life gained rather than a net change.
            ValueDef::LifeGainedThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        object.controller,
                        context.trigger,
                    )
                })
                .map(|player| i32::from(self.life_gained_this_turn[player.index()]))
                .sum(),
            // What the step before this one matched -- the land cards a
            // discard took. Zero without such a step behind it.
            ValueDef::MatchedCount => i32::from(context.matched_count.unwrap_or(0)),
            ValueDef::MatchedCardTypes => i32::from(context.matched_card_types.unwrap_or(0)),
            ValueDef::MatchedManaValue => i32::from(context.matched_mana_value.unwrap_or(0)),
            // Read off the binding rather than off the board: what was
            // exiled is no longer anywhere to count.
            ValueDef::BoundObjectCount(binding) => {
                i32::try_from(context.object_group(binding).len()).unwrap_or(i32::MAX)
            }
            ValueDef::CountObjects(objects) => i32::try_from(
                self.effect_recipients(
                    EffectRecipientDef::objects(*objects),
                    object,
                    context,
                    scoped,
                )
                .len(),
            )
            .unwrap_or(i32::MAX),
            ValueDef::CardTypesAmongObjects(objects) => self
                .card_types_among_targets(&self.effect_objects(*objects, object, context, scoped)),
            // Everybody's spells, minus the one carrying the ability: it was
            // counted as it was cast, and storm copies what came before it.
            // Read from this turn's cast order rather than from the running
            // total: the number is about the spells cast before this one, and
            // a spell cast in response to the trigger came after it.
            ValueDef::SpellsCastBeforeThisTurn => {
                i32::from(self.spells_cast_before(object.source.unwrap_or(object.id)))
            }
            ValueDef::AdditionalCostPayments(cost) => {
                let source = object.source.unwrap_or(object.id);
                let paid = self.additional_cost_payment_count(source, cost);
                if paid > 0 {
                    return i32::from(paid);
                }
                i32::from(
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .and_then(|permanent| {
                            permanent
                                .cast
                                .as_ref()
                                .and_then(|cast| cast.additional_costs.get(cost.index()).copied())
                        })
                        .unwrap_or(0),
                )
            }
            ValueDef::IfAdditionalCostPaid(conditional) => {
                let source = object.source.unwrap_or(object.id);
                let selected = if self.source_additional_cost_payments(source, conditional.cost) > 0
                {
                    conditional.if_paid
                } else {
                    conditional.otherwise
                };
                self.effect_value(selected, object, context, scoped)
            }
            ValueDef::CountersOnSource(kind) => object.source.map_or(0, |source| {
                i32::from(self.current_or_last_known_counters(source, kind))
            }),
            ValueDef::CountersOnObject(counted) => self
                .object_reference_target(counted.object, object, context, scoped)
                .and_then(|target| match target {
                    Target::Permanent(id) => self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == id),
                    Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
                })
                .map_or(0, |permanent| i32::from(permanent.counters(counted.kind))),
            ValueDef::DamageTakenThisTurn { player, source } => {
                let player = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .find(|candidate| {
                        self.player_relation_matches(
                            *candidate,
                            player,
                            object.controller,
                            context.trigger,
                        )
                    })
                    .unwrap_or(object.controller);
                i32::from(self.damage_taken_this_turn(player, source))
            }
            ValueDef::CardsInHandAbove { player, threshold } => {
                let player = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .find(|candidate| {
                        self.player_relation_matches(
                            *candidate,
                            player,
                            object.controller,
                            context.trigger,
                        )
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
            ValueDef::CountMatchingObjects(query) => i32::try_from(
                self.objects_matching_effect_query(*query, object, context, scoped)
                    .len(),
            )
            .unwrap_or(i32::MAX),
            ValueDef::DistinctNamesAmong(query) => self.distinct_names_among(
                &self.objects_matching_effect_query(*query, object, context, scoped),
            ),
            ValueDef::CountMatchingPlayerAttachments(query) => {
                let source = object.source.unwrap_or(object.id);
                i32::try_from(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            permanent.attached_player.is_some_and(|player| {
                                self.player_relation_matches_for_source(
                                    player,
                                    query.player,
                                    object.controller,
                                    source,
                                    TriggerContext::empty(),
                                )
                            }) && self.trigger_object_matches(
                                query.object,
                                &self.trigger_event_object(permanent),
                                source,
                                false,
                            )
                        })
                        .count(),
                )
                .unwrap_or(i32::MAX)
            }
            ValueDef::CountSpellsCastThisTurn(query) => self.spells_cast_matching_this_turn(
                *query,
                object.controller,
                object.source.unwrap_or(object.id),
                context.trigger,
            ),
            ValueDef::AggregateObjectValues(aggregate) => {
                let values = self
                    .effect_objects(aggregate.objects, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| {
                        let id = match target {
                            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => id,
                            Target::Player(_) => return None,
                        };
                        match aggregate.select {
                            crate::card::ObjectValueDef::ManaValue => {
                                self.current_or_last_known_mana_value(id).map(i32::from)
                            }
                            crate::card::ObjectValueDef::Power => {
                                self.current_or_last_known_power(id).map(i32::from)
                            }
                            crate::card::ObjectValueDef::Toughness => {
                                self.current_or_last_known_toughness(id).map(i32::from)
                            }
                            crate::card::ObjectValueDef::Counters(kind) => {
                                Some(i32::from(self.current_or_last_known_counters(id, kind)))
                            }
                        }
                    });
                match aggregate.operation {
                    crate::card::AggregateOperationDef::Minimum => values.min().unwrap_or(0),
                    crate::card::AggregateOperationDef::Maximum => values.max().unwrap_or(0),
                    crate::card::AggregateOperationDef::Sum => {
                        values.fold(0_i32, i32::saturating_add)
                    }
                }
            }
            ValueDef::AggregatePlayerObjectCounts(aggregate) => {
                let counts = self
                    .effect_players(aggregate.players, object, context, scoped)
                    .into_iter()
                    .map(|player| {
                        i32::try_from(
                            self.objects_matching_effect_query_for_controller(
                                aggregate.query,
                                player,
                                object,
                                context,
                                scoped,
                            )
                            .len(),
                        )
                        .unwrap_or(i32::MAX)
                    });
                match aggregate.operation {
                    crate::card::AggregateOperationDef::Minimum => counts.min().unwrap_or(0),
                    crate::card::AggregateOperationDef::Maximum => counts.max().unwrap_or(0),
                    crate::card::AggregateOperationDef::Sum => {
                        counts.fold(0_i32, i32::saturating_add)
                    }
                }
            }
            ValueDef::AnyMatchingObject(query) => i32::from(self.any_battlefield_object_matches(
                query,
                object.source.unwrap_or(object.id),
                object.controller,
            )),
            ValueDef::IfTargetMatches(_)
            | ValueDef::IfSourceMatches(_)
            | ValueDef::IfMatchingObjectCount(_)
            | ValueDef::IfCreatureDiedThisTurn(_)
            | ValueDef::IfControllerLifeAtMost(_)
            | ValueDef::IfCondition(_) => {
                self.conditional_effect_value(value, object, context, scoped)
            }
            ValueDef::Negate(inner) => self
                .effect_value(*inner, object, context, scoped)
                .saturating_neg(),
            ValueDef::Scaled(scaled) => self
                .effect_value(scaled.value, object, context, scoped)
                .saturating_mul(scaled.factor),
            ValueDef::Halved(halved) => {
                halved.apply(self.effect_value(halved.value, object, context, scoped))
            }
            ValueDef::Quotient(quotient) => quotient.apply(
                self.effect_value(quotient.numerator, object, context, scoped),
                self.effect_value(quotient.denominator, object, context, scoped),
            ),
            ValueDef::Sum(sum) => self
                .effect_value(sum.left, object, context, scoped)
                .saturating_add(self.effect_value(sum.right, object, context, scoped)),
        }
    }

    /// Whether the permanent a target slot points at matches, reading it as
    /// it last existed when it is no longer on the battlefield.
    ///
    /// "If that creature was a Human" is asked after the destruction that
    /// removed it, and a permanent that leaves gets a fresh object identity
    /// in its new zone -- so the corpse in the retired table is the only
    /// thing the old target still names.
    fn permanent_condition_matches(
        &self,
        predicate: ObjectPredicateDef,
        id: GameObjectId,
        source: GameObjectId,
    ) -> bool {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .or_else(|| match self.retired_objects.get(&id) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.as_ref()),
                _ => None,
            })
        {
            return self.trigger_object_matches(
                predicate,
                &self.trigger_event_object(permanent),
                source,
                false,
            );
        }
        self.card_in_nonbattlefield_zone(id)
            .is_some_and(|(zone, card)| self.card_object_matches(predicate, card, zone, source))
    }

    /// Resolve values that select between two branches separately from the
    /// direct value forms above.
    fn conditional_effect_value(
        &self,
        value: ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> i32 {
        match value {
            ValueDef::IfSourceMatches(condition) => {
                let source = object.source.unwrap_or(object.id);
                let chosen = if self.source_matches_value_predicate(source, condition.object) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            ValueDef::IfTargetMatches(condition) => {
                let source = object.source.unwrap_or(object.id);
                // The chosen target rather than the still-legal one: "if that
                // creature was a Human" is asked after the destruction that
                // made it illegal, which is the only time it is interesting.
                let matched = Self::chosen_targets(object, scoped.target_slot(condition.slot)).any(
                    |target| match target {
                        // A card that has already moved is a new object in
                        // its new zone, so follow the move: "if it's a green
                        // card" asked after returning it to hand is the same
                        // question about the same physical card.
                        Target::Card(id) => {
                            let id = if self.card_in_nonbattlefield_zone(id).is_some() {
                                id
                            } else {
                                self.successors.get(&id).copied().unwrap_or(id)
                            };
                            self.card_in_nonbattlefield_zone(id)
                                .is_some_and(|(zone, card)| {
                                    self.card_object_matches(condition.object, card, zone, source)
                                })
                        }
                        Target::Permanent(id) => {
                            self.permanent_condition_matches(condition.object, id, source)
                        }
                        Target::Player(_) | Target::Spell(_) => false,
                    },
                );
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
                let chosen = if crate::game::effect_support::compare(
                    &count,
                    condition.comparison,
                    &i32::from(condition.amount),
                ) {
                    condition.then
                } else {
                    condition.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            ValueDef::IfControllerLifeAtMost(branches) => {
                let chosen = if i32::from(self.players[object.controller.index()].life)
                    <= i32::from(branches.threshold)
                {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.effect_value(chosen, object, context, scoped)
            }
            ValueDef::IfCondition(branches) => {
                let chosen = if self.trigger_condition_holds(
                    branches.condition,
                    object.source.unwrap_or(object.id),
                    object.controller,
                    context.trigger,
                    object.ability.as_ref().map(|ability| ability.origin),
                    Some((object, scoped, context)),
                ) {
                    branches.then
                } else {
                    branches.otherwise
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

impl Game {
    /// How many distinct card types appear among the cards in every
    /// graveyard.
    ///
    /// Types rather than cards, and the union across both players: a single
    /// artifact creature card is worth two, and twenty cards split between
    /// artifacts and creatures are worth the same two.
    /// How many distinct names a set of objects has between them. A nameless
    /// object is not a name: a token with no card behind it shares "no name"
    /// with every other one, and "lands with different names" does not mean
    /// to count them.
    pub(super) fn distinct_names_among(&self, objects: &[Target]) -> i32 {
        let mut names = objects
            .iter()
            .filter_map(|target| match target {
                Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                    self.object_card_name(*id)
                }
                Target::Player(_) => None,
            })
            .collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();
        i32::try_from(names.len()).unwrap_or(i32::MAX)
    }

    /// How many distinct printed card types occur among a resolved set.
    pub(super) fn card_types_among_targets(&self, objects: &[Target]) -> i32 {
        let mut seen = CardTypeSet::empty();
        for object in objects {
            let id = match object {
                Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => *id,
                Target::Player(_) => continue,
            };
            if let Some(definition) = self
                .object_definition(id)
                .and_then(|definition| self.catalog.get(definition))
            {
                seen = seen.union(definition.rules.types());
            }
        }
        i32::try_from(
            CardType::ALL
                .into_iter()
                .filter(|card_type| seen.contains(*card_type))
                .count(),
        )
        .unwrap_or(0)
    }

    pub(super) fn card_types_among_graveyards(
        &self,
        relation: PlayerRelation,
        controller: PlayerId,
    ) -> i32 {
        let mut seen = CardTypeSet::empty();
        for (index, player) in self.players.iter().enumerate() {
            let candidate = if index == 0 {
                PlayerId::One
            } else {
                PlayerId::Two
            };
            if !self.player_relation_matches(
                candidate,
                relation,
                controller,
                super::TriggerContext::empty(),
            ) {
                continue;
            }
            for card in &player.graveyard {
                if let Some(definition) = self.catalog.get(card.definition) {
                    seen = seen.union(definition.rules.types());
                }
            }
        }
        i32::try_from(
            CardType::ALL
                .into_iter()
                .filter(|card_type| seen.contains(*card_type))
                .count(),
        )
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::devotion_symbols;
    use crate::ManaColor;

    #[test]
    fn every_flexible_colored_component_counts_toward_devotion() {
        let cost = crate::mana_cost!("{2/B}{R/P}{G/U/P}{C/W}");
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ] {
            assert_eq!(devotion_symbols(cost, color), 1);
        }
        assert_eq!(devotion_symbols(cost, ManaColor::Colorless), 0);
    }
}
