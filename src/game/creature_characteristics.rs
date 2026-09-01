use std::cell::Cell;

use super::continuous_effects::StaticEffectKind;
use super::{
    AppliedEffectDef, BasicLandType, CardSupertype, CardType, CharacteristicOperationDef,
    ContinuousEffectTimestamp, ControlFlow, CounterKind, DeclarativeAbilityDef, EffectDef, Game,
    GameObjectId, KeywordAbility, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, Permanent,
    PlayerId, PlayerRelation, PowerToughnessOperationDef, ResolvedContinuousEffectKind,
    ResolvedPowerToughnessOperation, RetiredObject, Target, TriggerContext, ValueDef,
};

type BaseStatSetter = (ContinuousEffectTimestamp, u16, Option<i16>, Option<i16>);

/// What a characteristic-defining ability says about each half, with `None`
/// meaning "the printed number stands" (CR 604.3).
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct DefinedStats {
    power: Option<i16>,
    toughness: Option<i16>,
}

impl DefinedStats {
    /// Layer 7a over the printed corner: whichever halves are defined
    /// replace what the card prints, and the rest is left alone.
    pub(super) fn over(self, printed: crate::CreatureStats) -> crate::CreatureStats {
        crate::CreatureStats {
            power: self.power.unwrap_or(printed.power),
            toughness: self.toughness.unwrap_or(printed.toughness),
        }
    }
}

thread_local! {
    /// Guards the live layer-7 walk when a static recipient predicate asks for
    /// power or toughness while that same walk is being assembled.
    static STATIC_POWER_TOUGHNESS_LAYER_PASS: Cell<bool> = const { Cell::new(false) };
}

struct StaticPowerToughnessLayerGuard;

impl StaticPowerToughnessLayerGuard {
    fn enter() -> Option<Self> {
        STATIC_POWER_TOUGHNESS_LAYER_PASS
            .with(|pass| if pass.replace(true) { None } else { Some(Self) })
    }
}

impl Drop for StaticPowerToughnessLayerGuard {
    fn drop(&mut self) {
        STATIC_POWER_TOUGHNESS_LAYER_PASS.with(|pass| pass.set(false));
    }
}

impl Game {
    pub(super) fn base_stats(&self, permanent: &Permanent) -> Option<crate::CreatureStats> {
        if !self
            .permanent_types(permanent)?
            .contains(CardType::Creature)
        {
            return None;
        }
        let mut setters = permanent
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::PowerToughness(
                    ResolvedPowerToughnessOperation::SetBase { power, toughness },
                ) => Some((
                    effect.timestamp,
                    effect.component_order,
                    Some(power),
                    Some(toughness),
                )),
                ResolvedContinuousEffectKind::PowerToughness(
                    ResolvedPowerToughnessOperation::SetBasePower { power },
                ) => Some((effect.timestamp, effect.component_order, Some(power), None)),
                ResolvedContinuousEffectKind::PowerToughness(
                    ResolvedPowerToughnessOperation::SetBaseToughness { toughness },
                ) => Some((
                    effect.timestamp,
                    effect.component_order,
                    None,
                    Some(toughness),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        let defined = self.defining_stats(permanent, &mut setters);
        if setters.is_empty() {
            // "Except it's a 1/1" travels with the copy, so it answers before
            // the copied card's own printed stats do.
            if let Some((power, toughness)) = permanent
                .active_copy_values()
                .and_then(|copy| copy.base_power_toughness)
            {
                return Some(defined.over(crate::CreatureStats { power, toughness }));
            }
            return self
                .effective_rules(permanent)
                .and_then(|rules| rules.creature_stats())
                .map(|printed| defined.over(printed));
        }
        // Applied in order rather than by taking the latest outright: a setter
        // that names only power leaves the toughness under it standing, which
        // the printed stats supply when nothing else has.
        setters.sort_by_key(|(timestamp, order, _, _)| (*timestamp, *order));
        // Layer 7a first: what a characteristic-defining ability says is the
        // base every 7b setter below then replaces or leaves standing.
        let mut stats = defined.over(
            self.effective_rules(permanent)
                .and_then(|rules| rules.creature_stats())
                .unwrap_or(crate::CreatureStats {
                    power: 0,
                    toughness: 0,
                }),
        );
        for (_, _, power, toughness) in setters {
            stats = crate::CreatureStats {
                power: power.unwrap_or(stats.power),
                toughness: toughness.unwrap_or(stats.toughness),
            };
        }
        Some(stats)
    }

    /// Walks the statics that apply to this permanent, appending every
    /// layer-7b base setter to `setters` and returning what any layer-7a
    /// characteristic-defining ability says (CR 604.3).
    ///
    /// The two come out of one walk because they come from one place: a
    /// defining ability is a static ability like the others, and reaching it
    /// through this walk is what makes it stop applying when the permanent
    /// loses its abilities.
    fn defining_stats(
        &self,
        permanent: &Permanent,
        setters: &mut Vec<BaseStatSetter>,
    ) -> DefinedStats {
        let mut defined = DefinedStats::default();
        let Some(_pass) = StaticPowerToughnessLayerGuard::enter() else {
            return defined;
        };
        let result = self.visit_static_applied_effects(
            permanent,
            StaticEffectKind::PowerToughness,
            |applied| {
                let AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    operation,
                )) = applied.effect
                else {
                    unreachable!("the power/toughness filter admits only stat operations");
                };
                let (power, toughness) = match operation {
                    PowerToughnessOperationDef::SetBase { power, toughness } => (
                        Some(self.static_power_toughness_value(permanent, applied.source, power)),
                        Some(self.static_power_toughness_value(
                            permanent,
                            applied.source,
                            toughness,
                        )),
                    ),
                    PowerToughnessOperationDef::SetBasePower(power) => (
                        Some(self.static_power_toughness_value(permanent, applied.source, power)),
                        None,
                    ),
                    PowerToughnessOperationDef::Define { power, toughness } => {
                        // Applied to the base rather than pushed as a setter:
                        // 7a is under every 7b setter no matter which
                        // timestamp each one carries.
                        if let Some(power) = power {
                            defined.power = Some(self.static_power_toughness_value(
                                permanent,
                                applied.source,
                                power,
                            ));
                        }
                        if let Some(toughness) = toughness {
                            defined.toughness = Some(self.static_power_toughness_value(
                                permanent,
                                applied.source,
                                toughness,
                            ));
                        }
                        return ControlFlow::Continue(());
                    }
                    _ => return ControlFlow::Continue(()),
                };
                setters.push((applied.timestamp, applied.component_order, power, toughness));
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        defined
    }

    /// What a characteristic-defining ability printed on this card says
    /// about a copy of it that is not on the battlefield (CR 604.3). Every
    /// amount is measured from `controller`, who outside the battlefield is
    /// the card's owner.
    ///
    /// Behind the same guard the battlefield walk uses: an amount that asks
    /// after a power would otherwise be able to ask after this one, and a
    /// definition that cannot be computed leaves the printed number
    /// standing rather than looping.
    pub(super) fn card_defined_stats(
        &self,
        definition: &crate::card::CardDefinition,
        id: GameObjectId,
        controller: PlayerId,
    ) -> DefinedStats {
        let mut defined = DefinedStats::default();
        let Some(_pass) = StaticPowerToughnessLayerGuard::enter() else {
            return defined;
        };
        let measure = |value: ValueDef| {
            let amount = self.static_stat_value(value, id, controller);
            i16::try_from(amount.clamp(i32::from(i16::MIN), i32::from(i16::MAX))).ok()
        };
        for ability in definition.rules.ability_clauses() {
            if !ability.is_executable() {
                continue;
            }
            let Some(crate::card::EffectDef::StaticApply {
                effect:
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                        PowerToughnessOperationDef::Define { power, toughness },
                    )),
                ..
            }) = ability.declarative_effect()
            else {
                continue;
            };
            if let Some(power) = power {
                defined.power = measure(power);
            }
            if let Some(toughness) = toughness {
                defined.toughness = measure(toughness);
            }
        }
        defined
    }

    pub(super) fn controls_land_type(&self, player: PlayerId, land_type: BasicLandType) -> bool {
        self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self.effective_land_types(permanent)[land_type.index()]
        })
    }

    /// What every counter on this permanent adds to its power and toughness.
    /// Most kinds are markers and add nothing; the ones that do carry their
    /// own amounts rather than being special-cased here.
    pub(super) fn counter_stat_bonus(permanent: &Permanent) -> (i16, i16) {
        permanent
            .counters
            .iter()
            .fold((0, 0), |total, (kind, held)| {
                let (power, toughness) = kind.power_toughness_bonus();
                if power == 0 && toughness == 0 {
                    return total;
                }
                let count = i16::try_from(held).unwrap_or(i16::MAX);
                (
                    total.0.saturating_add(power.saturating_mul(count)),
                    total.1.saturating_add(toughness.saturating_mul(count)),
                )
            })
    }

    /// Whether any permanent on the battlefield matches, which is what an "as
    /// long as you control a ..." clause asks. `controller` is whoever the
    /// query's player relation is measured against.
    pub(super) fn any_battlefield_object_matches(
        &self,
        query: &ObjectQueryDef,
        source: GameObjectId,
        controller: PlayerId,
    ) -> bool {
        self.battlefield.iter().any(|permanent| {
            self.query_player_constraints_match(
                Some(permanent.controller),
                permanent.card.owner,
                *query,
                (controller, source),
                TriggerContext::empty(),
                None,
            ) && self.trigger_object_matches(
                query.object,
                &self.trigger_event_object(permanent),
                source,
                false,
            )
        })
    }

    fn static_object_set_value(&self, value: ValueDef, source: GameObjectId) -> Option<i32> {
        let aggregate =
            match value {
                ValueDef::CountObjects(objects) => {
                    return Some(
                        i32::try_from(self.source_object_set_targets(*objects, source).len())
                            .unwrap_or(i32::MAX),
                    );
                }
                ValueDef::CardTypesAmongObjects(objects) => {
                    return Some(self.card_types_among_targets(
                        &self.source_object_set_targets(*objects, source),
                    ));
                }
                ValueDef::AggregateObjectValues(aggregate) => aggregate,
                _ => return None,
            };
        let values = self
            .source_object_set_targets(aggregate.objects, source)
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
                }
            });
        Some(match aggregate.operation {
            crate::card::AggregateOperationDef::Minimum => values.min().unwrap_or(0),
            crate::card::AggregateOperationDef::Maximum => values.max().unwrap_or(0),
            crate::card::AggregateOperationDef::Sum => values.fold(0_i32, i32::saturating_add),
        })
    }

    /// One value inside a static power/toughness bonus. A scale multiplies
    /// another such value, which is how "+2/+2 for each Aura attached to it"
    /// is expressed; everything outside this vocabulary stays a seam, and the
    /// boundary test rejects a card that reaches for one.
    pub(super) fn static_stat_value(
        &self,
        value: ValueDef,
        source: GameObjectId,
        controller: PlayerId,
    ) -> i32 {
        if let Some(value) = self.static_object_set_value(value, source) {
            return value;
        }
        match value {
            ValueDef::Constant(amount) => amount,
            ValueDef::AnyMatchingObject(query) => {
                i32::from(self.any_battlefield_object_matches(query, source, controller))
            }
            // A bonus that counts is how a token whose printed power and
            // toughness are defined by the board is expressed: a zero-power
            // body plus the count.
            ValueDef::CountMatchingObjects(query) => i32::try_from(
                self.objects_matching_query(*query, controller, source, TriggerContext::empty())
                    .len(),
            )
            .unwrap_or(i32::MAX),
            ValueDef::Scaled(scaled) => self
                .static_stat_value(scaled.value, source, controller)
                .saturating_mul(scaled.factor),
            ValueDef::Halved(halved) => {
                halved.apply(self.static_stat_value(halved.value, source, controller))
            }
            ValueDef::Quotient(quotient) => quotient.apply(
                self.static_stat_value(quotient.numerator, source, controller),
                self.static_stat_value(quotient.denominator, source, controller),
            ),
            ValueDef::Sum(sum) => self
                .static_stat_value(sum.left, source, controller)
                .saturating_add(self.static_stat_value(sum.right, source, controller)),
            ValueDef::IfSourceMatches(branches) => {
                let value = if self.source_matches_value_predicate(source, branches.object) {
                    branches.then
                } else {
                    branches.otherwise
                };
                self.static_stat_value(value, source, controller)
            }
            // "Your hand" is the static effect's own controller's, measured
            // live, so a creature defined by it changes size as cards come
            // and go. A threshold of zero is the plain count.
            ValueDef::CardsInHandAbove { player, threshold } => {
                // "Its controller's hand" on an Aura means the enchanted
                // permanent's controller, which only the source can answer --
                // the general relation test has no source to follow.
                let counted = if player == PlayerRelation::ControllerOfAttachedPermanent {
                    self.attached_host_controller_of(source)
                        .unwrap_or(controller)
                } else {
                    [PlayerId::One, PlayerId::Two]
                        .into_iter()
                        .find(|candidate| {
                            self.player_relation_matches(
                                *candidate,
                                player,
                                controller,
                                TriggerContext::empty(),
                            )
                        })
                        .unwrap_or(controller)
                };
                i32::try_from(
                    self.players[counted.index()]
                        .hand
                        .len()
                        .saturating_sub(usize::from(threshold)),
                )
                .unwrap_or(i32::MAX)
            }
            // Read from every graveyard rather than from the board, so a
            // Lhurgoyf resizes as cards arrive there.
            ValueDef::CardTypesAmongGraveyards(player) => {
                self.card_types_among_graveyards(player, controller)
            }
            // Counters on the effect's own source, which is plain state
            // rather than anything derived: an Equipment that grows gives
            // what it carries, and the bonus follows every counter.
            ValueDef::CountersOnSource(kind) => {
                i32::from(self.current_or_last_known_counters(source, kind))
            }
            // The static ability names the object whose counters matter.
            // CreatingSource follows the affected token's captured
            // provenance to that exact battlefield incarnation. There is no
            // last-known fallback: an orphaned token reads zero.
            ValueDef::CountersOnObject(counted) => {
                let object = match counted.object {
                    ObjectRefDef::Source => Some(source),
                    ObjectRefDef::CreatingSource => self.creating_source_of(source),
                    _ => None,
                };
                i32::from(
                    object
                        .and_then(|object| {
                            self.battlefield
                                .iter()
                                .find(|permanent| permanent.card.id == object)
                        })
                        .map_or(0, |permanent| permanent.counters(counted.kind)),
                )
            }
            // Domain, read live off the board: the Kavu resizes as lands
            // with new basic types arrive and leave.
            ValueDef::BasicLandTypesControlled(_) => self.player_readable_value(value, controller),
            // A tally the turn keeps, read the same way: the creature
            // resizes as its controller draws.
            ValueDef::CardsDrawnThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        controller,
                        TriggerContext::empty(),
                    )
                })
                .map(|player| i32::from(self.cards_drawn_this_turn[player.index()]))
                .sum(),
            _ => 0,
        }
    }

    /// Whether a value-producing clause's own source currently matches a
    /// characteristic predicate. Battlefield and stack objects use their
    /// effective characteristics; cards elsewhere use their zone-aware card
    /// characteristics. A source that has already left every live zone
    /// matches nothing.
    pub(super) fn source_matches_value_predicate(
        &self,
        source: GameObjectId,
        predicate: ObjectPredicateDef,
    ) -> bool {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return self.trigger_object_matches(
                predicate,
                &self.trigger_event_object(permanent),
                source,
                false,
            );
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == source) {
            return self
                .stack_trigger_event_object(object)
                .is_some_and(|characteristics| {
                    self.trigger_object_matches(predicate, &characteristics, source, true)
                });
        }
        self.card_in_nonbattlefield_zone(source)
            .is_some_and(|(zone, card)| self.card_object_matches(predicate, card, zone, source))
    }

    pub(super) fn static_power_toughness_bonus(&self, permanent: &Permanent) -> (i16, i16) {
        let Some(_pass) = StaticPowerToughnessLayerGuard::enter() else {
            return (0, 0);
        };
        let mut total = (0_i16, 0_i16);
        let result = self.visit_static_applied_effects(
            permanent,
            StaticEffectKind::PowerToughness,
            |applied| {
                if let AppliedEffectDef::Characteristic(
                    CharacteristicOperationDef::PowerToughness(
                        PowerToughnessOperationDef::Modify { power, toughness },
                    ),
                ) = applied.effect
                {
                    total = (
                        total.0.saturating_add(self.static_power_toughness_value(
                            permanent,
                            applied.source,
                            power,
                        )),
                        total.1.saturating_add(self.static_power_toughness_value(
                            permanent,
                            applied.source,
                            toughness,
                        )),
                    );
                }
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        total
    }

    fn static_power_toughness_value(
        &self,
        permanent: &Permanent,
        source: GameObjectId,
        value: ValueDef,
    ) -> i16 {
        // The one amount measured from the affected object rather than from
        // the effect's source: Opalescence gives each enchantment a body its
        // own cost decides.
        if value == ValueDef::AffectedManaValue {
            return i16::try_from(self.permanent_mana_value(permanent)).unwrap_or(i16::MAX);
        }
        if value == ValueDef::AffectedColorCount {
            let color_count = self
                .permanent_colors(permanent)
                .into_iter()
                .filter(|present| *present)
                .count();
            return i16::try_from(color_count).unwrap_or(i16::MAX);
        }
        // Every other static amount is measured from its own source's
        // controller, not from whoever it is being applied to.
        let controller = self
            .controller_of_object(source)
            .unwrap_or(permanent.controller);
        let amount = self.static_stat_value(value, source, controller);
        i16::try_from(amount.clamp(i32::from(i16::MIN), i32::from(i16::MAX)))
            .expect("the static amount was clamped to i16")
    }

    fn resolved_power_toughness_bonus(&self, permanent: &Permanent) -> (i16, i16) {
        permanent
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .fold((0_i16, 0_i16), |total, effect| match effect.kind {
                ResolvedContinuousEffectKind::PowerToughness(
                    ResolvedPowerToughnessOperation::Modify { power, toughness },
                ) => (
                    total.0.saturating_add(power),
                    total.1.saturating_add(toughness),
                ),
                _ => total,
            })
    }

    /// Power without continuous static bonuses.
    ///
    /// Characteristics handed to static resolution cannot use full `power`:
    /// static effects are resolved by matching each source against the
    /// affected permanent's characteristics, so asking for power there would
    /// re-enter this computation forever. Target legality reaches around that
    /// with `Game::targeting_event_object`; a `PowerAtLeast` predicate asked
    /// during trigger or static matching still sees counters and
    /// until-end-of-turn pumps but not a Crusade-style static.
    ///
    /// Keywords were truncated for the same reason and no longer are:
    /// [`Game::collect_ability_layer_operations`] stratifies its walk so only
    /// queries raised inside it fall back, which would work here too and would
    /// retire the second characteristics view.
    pub(super) fn power_ignoring_static_effects(&self, permanent: &Permanent) -> Option<i16> {
        let base = self.base_stats(permanent)?;
        Some(self.creature_stats_parts(permanent, base, (0, 0)).power)
    }

    pub(super) fn power(&self, permanent: &Permanent) -> Option<i16> {
        self.creature_stats(permanent).map(|stats| stats.power)
    }

    pub(super) fn creature_stats(&self, permanent: &Permanent) -> Option<crate::CreatureStats> {
        let base = self.base_stats(permanent)?;
        let static_bonus = self.static_power_toughness_bonus(permanent);
        Some(self.creature_stats_parts(permanent, base, static_bonus))
    }

    pub(super) fn toughness_ignoring_static_effects(&self, permanent: &Permanent) -> Option<i16> {
        let base = self.base_stats(permanent)?;
        Some(self.creature_stats_parts(permanent, base, (0, 0)).toughness)
    }

    pub(super) fn toughness(&self, permanent: &Permanent) -> Option<i16> {
        self.creature_stats(permanent).map(|stats| stats.toughness)
    }

    fn creature_stats_parts(
        &self,
        permanent: &Permanent,
        base: crate::CreatureStats,
        static_bonus: (i16, i16),
    ) -> crate::CreatureStats {
        let counter_bonus = Self::counter_stat_bonus(permanent);
        let resolved_bonus = self.resolved_power_toughness_bonus(permanent);
        let stats = crate::CreatureStats {
            power: base.power + resolved_bonus.0 + static_bonus.0 + counter_bonus.0,
            toughness: base.toughness + resolved_bonus.1 + static_bonus.1 + counter_bonus.1,
        };
        // CR 613.4e: the switch is applied after everything above, and two
        // switches in effect at once cancel -- so what matters is the parity
        // of how many are applied, not their order among themselves.
        if self.power_toughness_switches(permanent).is_multiple_of(2) {
            stats
        } else {
            crate::CreatureStats {
                power: stats.toughness,
                toughness: stats.power,
            }
        }
    }

    /// How many switch effects currently apply to this permanent, counted
    /// across both the resolved effects it carries and the statics that name
    /// it. Only the parity is ever used.
    fn power_toughness_switches(&self, permanent: &Permanent) -> usize {
        let resolved = permanent
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter(|effect| {
                matches!(
                    effect.kind,
                    ResolvedContinuousEffectKind::PowerToughness(
                        ResolvedPowerToughnessOperation::Switch
                    )
                )
            })
            .count();
        let Some(_pass) = StaticPowerToughnessLayerGuard::enter() else {
            return resolved;
        };
        let mut statics = 0;
        let result = self.visit_static_applied_effects(
            permanent,
            StaticEffectKind::PowerToughness,
            |applied| {
                if matches!(
                    applied.effect,
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                        PowerToughnessOperationDef::Switch
                    ))
                ) {
                    statics += 1;
                }
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        resolved + statics
    }

    pub(super) fn has_flying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Flying)
    }

    pub(super) fn has_trample(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Trample)
    }

    pub(super) fn has_undying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Undying)
    }

    /// Which counter a dying creature comes back with, if any. Undying and
    /// persist are the same clause read from opposite ends, and each is
    /// barred by the counter it puts on: a creature that has already come
    /// back stays where it lands the second time. A creature with both is
    /// answered by undying, since one return is all either of them gets.
    pub(super) fn returns_from_death_with(&self, permanent: &Permanent) -> Option<CounterKind> {
        if self.has_undying(permanent) && permanent.counters(CounterKind::PlusOnePlusOne) == 0 {
            return Some(CounterKind::PlusOnePlusOne);
        }
        if self.permanent_has_executable_keyword(permanent, KeywordAbility::Persist)
            && permanent.counters(CounterKind::MinusOneMinusOne) == 0
        {
            return Some(CounterKind::MinusOneMinusOne);
        }
        None
    }

    pub(super) fn has_indestructible(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Indestructible)
    }

    #[cfg(test)]
    pub(super) fn has_hexproof(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof)
    }

    /// CR 702.14: landwalk beats blocking when the defending player controls
    /// a land of the named type. A creature can carry several, and any one of
    /// them is enough, so this asks the question once for all five.
    pub(super) fn landwalk_beats(&self, permanent: &Permanent, defender: PlayerId) -> bool {
        let basic = BasicLandType::ALL.iter().any(|land_type| {
            self.permanent_has_executable_keyword(permanent, KeywordAbility::Landwalk(*land_type))
                && self.controls_land_type(defender, *land_type)
                && !self.landwalk_can_be_blocked(*land_type)
        });
        basic
            || (self.permanent_has_executable_keyword(permanent, KeywordAbility::LegendaryLandwalk)
                && self.controls_land_matching(
                    defender,
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ))
    }

    /// Whether the player controls a land matching this predicate. The two
    /// non-basic walks name a subtype and a supertype rather than a basic
    /// land type, so they cannot go through `controls_land_type`.
    fn controls_land_matching(&self, player: PlayerId, land: ObjectPredicateDef) -> bool {
        self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self
                    .effective_rules(permanent)
                    .is_some_and(|rules| rules.types().contains(CardType::Land))
                && self.trigger_object_matches(
                    land,
                    &self.trigger_event_object(permanent),
                    permanent.card.id,
                    false,
                )
        })
    }

    /// Whether something on the battlefield says creatures with this landwalk
    /// can be blocked anyway. The keyword itself is untouched, so anything
    /// else that reads it still sees it; only blocking ignores it.
    fn landwalk_can_be_blocked(&self, land_type: BasicLandType) -> bool {
        self.battlefield.iter().any(|permanent| {
            self.find_effective_ability(permanent, |effective| {
                effective.ability.is_executable()
                    && matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::Static(_)
                    )
                    && effective.ability.declarative_effect()
                        == Some(EffectDef::LandwalkCanBeBlocked(land_type))
            })
            .is_some()
        })
    }

    pub(super) fn permanent_has_executable_keyword(
        &self,
        permanent: &Permanent,
        expected: KeywordAbility,
    ) -> bool {
        self.find_effective_ability(permanent, |effective| {
            effective.ability.is_executable()
                && matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Keyword(actual) if actual == expected
                )
        })
        .is_some()
    }

    pub(super) fn source_controller_with_keyword(
        &self,
        source: GameObjectId,
        expected: KeywordAbility,
    ) -> Option<PlayerId> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return self
                .permanent_has_executable_keyword(permanent, expected)
                .then_some(permanent.controller);
        }
        match self.retired_objects.get(&source) {
            Some(RetiredObject::Permanent {
                permanent,
                keywords,
                ..
            }) if keywords.contains(&expected) => Some(permanent.controller),
            Some(
                RetiredObject::Permanent { .. } | RetiredObject::Card(_) | RetiredObject::Stack(_),
            )
            | None => None,
        }
    }

    #[cfg(test)]
    pub(super) fn has_forestwalk(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(
            permanent,
            KeywordAbility::Landwalk(BasicLandType::Forest),
        )
    }

    /// Whether this permanent may pay a `{T}` or `{Q}` activation cost.
    /// Both symbols apply the creature continuous-control rule; haste lifts it.
    pub(super) fn can_use_tap_or_untap_ability(&self, permanent: &Permanent) -> bool {
        self.base_stats(permanent).is_none_or(|_| {
            self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
                || self.turns_started[permanent.controller.index()]
                    > permanent.entered_controller_turn
        })
    }
}

impl Game {
    /// The colours of every card exiled with `source`, deduplicated. Read
    /// from the catalog for the same reason the stats below are: nothing
    /// continuous applies outside the battlefield, so printed is what an
    /// exiled card is.
    pub(super) fn linked_exile_colors(&self, source: GameObjectId) -> Vec<crate::card::ManaColor> {
        let mut colors = self
            .linked_exile_ids(source)
            .into_iter()
            .filter_map(|exiled| {
                self.card_in_nonbattlefield_zone(exiled)
                    .map(|(_, card)| card.definition)
            })
            .filter_map(|definition| self.catalog.get(definition))
            .flat_map(|card| {
                crate::card::ManaColor::COLORS
                    .into_iter()
                    .filter(|color| card.rules.has_color(*color))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        colors.sort_unstable();
        colors.dedup();
        colors
    }
}
