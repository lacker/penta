use super::{
    AppliedEffectDef, BasicLandType, CardBehavior, CardRules, CardSupertype, CardType, ControlFlow,
    CounterKind, DeclarativeAbilityDef, EffectDef, Game, GameObjectId, KeywordAbility,
    ObjectPredicateDef, ObjectQueryDef, Permanent, PlayerId, RetiredObject, TriggerContext,
    ValueDef,
};

impl Game {
    pub(super) fn base_stats(&self, permanent: &Permanent) -> Option<crate::CreatureStats> {
        // Printed power and toughness are ignored while a permanent is not a
        // creature. This matters for type-changing attachment procedures:
        // a bestowed creature, a Licid in Aura form, or a reconfigured
        // Equipment must not keep taking creature state-based actions merely
        // because its card still prints a power/toughness box.
        if !self
            .permanent_types(permanent)?
            .contains(CardType::Creature)
        {
            return None;
        }
        // Once Factory's animation ability resolves, removing its printed
        // abilities does not end the continuous animation effect. In
        // particular, Blood Moon changes its land subtype and abilities but
        // leaves the active artifact-creature types and 2/2 base stats intact.
        if let Some(animation) = permanent.animation {
            Some(crate::CreatureStats {
                power: animation.definition.power,
                toughness: animation.definition.toughness,
            })
        } else {
            self.effective_rules(permanent)
                .and_then(CardRules::creature_stats)
        }
    }

    pub(super) fn controls_land_type(&self, player: PlayerId, land_type: BasicLandType) -> bool {
        self.battlefield.iter().any(|permanent| {
            permanent.controller == player
                && self.effective_land_types(permanent)[land_type.index()]
        })
    }

    pub(super) fn plus_one_counter_bonus(permanent: &Permanent) -> i16 {
        i16::try_from(permanent.counters(CounterKind::PlusOnePlusOne)).unwrap_or(i16::MAX)
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
            self.player_relation_matches(
                permanent.controller,
                query.controller,
                controller,
                TriggerContext::empty(),
            ) && self.trigger_object_matches(
                query.object,
                &self.trigger_event_object(permanent),
                source,
                false,
            )
        })
    }

    pub(super) fn static_power_toughness_bonus(&self, permanent: &Permanent) -> (i16, i16) {
        let mut total = (0_i16, 0_i16);
        let result = self.visit_static_applied_effects(permanent, |applied| {
            if let AppliedEffectDef::ModifyPowerToughness { power, toughness } = applied.effect {
                // A static bonus is measured from its own source's controller,
                // not from whoever it is being applied to.
                let controller = self
                    .controller_of_object(applied.source)
                    .unwrap_or(permanent.controller);
                let bonus = |value: ValueDef| -> i16 {
                    let amount = match value {
                        ValueDef::Constant(amount) => amount,
                        ValueDef::AnyMatchingObject(query) => i32::from(
                            self.any_battlefield_object_matches(query, applied.source, controller),
                        ),
                        // A bonus that counts is how a token whose printed
                        // power and toughness are defined by the board is
                        // expressed: a zero-power body plus the count.
                        ValueDef::CountMatchingObjects(query) => i32::try_from(
                            self.objects_matching_query(
                                *query,
                                controller,
                                applied.source,
                                TriggerContext::empty(),
                            )
                            .len(),
                        )
                        .unwrap_or(i32::MAX),
                        // Everything else stays a seam; the boundary test
                        // rejects a card that reaches for one.
                        _ => 0,
                    };
                    i16::try_from(amount.clamp(i32::from(i16::MIN), i32::from(i16::MAX)))
                        .expect("the static bonus was clamped to i16")
                };
                total = (
                    total.0.saturating_add(bonus(power)),
                    total.1.saturating_add(bonus(toughness)),
                );
            }
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
        total
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
        let behavior = self.effective_behavior(permanent);
        let conditional_bonus = match behavior {
            Some(CardBehavior::KirdApe)
                if self.controls_land_type(permanent.controller, BasicLandType::Forest) =>
            {
                (1, 2)
            }
            Some(CardBehavior::SedgeTroll)
                if self.controls_land_type(permanent.controller, BasicLandType::Swamp) =>
            {
                (1, 1)
            }
            _ => (0, 0),
        };
        let ascended = if behavior == Some(CardBehavior::BloodBaronOfVizkopa)
            && self.players[permanent.controller.index()].life >= 30
            && self.players[permanent.controller.opponent().index()].life <= 10
        {
            6
        } else {
            0
        };
        let counter_bonus = Self::plus_one_counter_bonus(permanent);
        crate::CreatureStats {
            power: base.power
                + ascended
                + permanent.power_bonus
                + static_bonus.0
                + conditional_bonus.0
                + counter_bonus,
            toughness: base.toughness
                + ascended
                + permanent.toughness_bonus
                + static_bonus.1
                + conditional_bonus.1
                + counter_bonus,
        }
    }

    pub(super) fn has_flying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Flying)
            || self.blood_baron_has_ascended(permanent)
    }

    /// Blood Baron of Vizkopa's condition: 30 or more life for its controller
    /// and 10 or less for the opponent. While it holds the Baron is +6/+6 and
    /// flies.
    pub(super) fn blood_baron_has_ascended(&self, permanent: &Permanent) -> bool {
        self.effective_behavior(permanent) == Some(CardBehavior::BloodBaronOfVizkopa)
            && self.players[permanent.controller.index()].life >= 30
            && self.players[permanent.controller.opponent().index()].life <= 10
    }

    pub(super) fn has_trample(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Trample)
    }

    pub(super) fn has_undying(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Undying)
    }

    pub(super) fn has_indestructible(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Indestructible)
    }

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
        self.battlefield
            .iter()
            .filter_map(|permanent| self.effective_rules(permanent))
            .flat_map(CardRules::ability_clauses)
            .filter(|ability| ability.is_executable())
            .filter_map(|ability| ability.declarative_effect())
            .any(|effect| effect == EffectDef::LandwalkCanBeBlocked(land_type))
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

    pub(super) fn can_use_tap_ability(&self, permanent: &Permanent) -> bool {
        self.base_stats(permanent).is_none_or(|_| {
            self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
                || self.turns_started[permanent.controller.index()]
                    > permanent.entered_controller_turn
        })
    }
}
