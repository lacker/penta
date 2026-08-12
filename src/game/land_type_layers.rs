use super::{
    AppliedEffectDef, BasicLandType, CREATURE_TYPES, CardType, ContinuousEffectTimestamp,
    ControlFlow, Cow, DeclarativeAbilityDef, EffectDef, EffectDurationDef, EffectRecipientDef,
    Game, LandTypeOperation, ObjectPredicateDef, Permanent, TriggerContext, ZoneKind,
};

/// Land subtype vocabulary from CR 205.3i. Type-setting effects must remove
/// every land subtype while preserving subtypes belonging to the object's
/// other card types.
const LAND_SUBTYPES: &[&str] = &[
    "Cave",
    "Desert",
    "Forest",
    "Gate",
    "Island",
    "Lair",
    "Locus",
    "Mine",
    "Mountain",
    "Plains",
    "Planet",
    "Power-Plant",
    "Sphere",
    "Swamp",
    "Tower",
    "Town",
    "Urza's",
    "Urza’s",
];

impl Game {
    fn land_type_operations(
        &self,
        permanent: &Permanent,
    ) -> Vec<(ContinuousEffectTimestamp, LandTypeOperation)> {
        self.land_type_operations_from_sources(permanent, None)
    }

    fn land_type_operations_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> Vec<(ContinuousEffectTimestamp, LandTypeOperation)> {
        if prospective.card.id != permanent.card.id {
            return self.land_type_operations(permanent);
        }
        self.land_type_operations_from_sources(permanent, Some(prospective))
    }

    pub(super) fn prospective_continuous_effect_timestamp(&self) -> ContinuousEffectTimestamp {
        ContinuousEffectTimestamp(
            self.battlefield
                .iter()
                .chain(self.emblems.iter())
                .map(|permanent| permanent.timestamp.0.saturating_add(1))
                .max()
                .unwrap_or(self.next_continuous_effect_timestamp)
                .max(self.next_continuous_effect_timestamp),
        )
    }

    /// Collects the layer-4 land-type slice without asking for an effective
    /// subtype or effective ability. That separation prevents the recursive
    /// characteristic query that a future full layer evaluator will replace.
    ///
    /// A Set operation removes rules-text/copy abilities under CR 305.7. When
    /// that hits the source of another land-type effect, the Set effect is
    /// applied first as a dependency regardless of timestamp. This guarded
    /// one-pass dependency rule is sufficient for Blood Moon versus Urborg,
    /// Yavimaya, and copies of them; granted static abilities remain rejected
    /// by catalog validation until the general fixed-point evaluator exists.
    fn land_type_operations_from_sources(
        &self,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<(ContinuousEffectTimestamp, LandTypeOperation)> {
        let sources = self.land_type_effect_sources(prospective);

        let mut operations = Vec::new();
        for (source, timestamp) in &sources {
            if self.raw_land_type_set_applies(source, &sources) {
                continue;
            }
            self.collect_land_type_operations_from_source(
                source,
                affected,
                *timestamp,
                &mut operations,
            );
        }
        operations.sort_by_key(|(timestamp, _)| *timestamp);
        operations
    }

    pub(super) fn land_type_effect_sources<'a>(
        &'a self,
        prospective: Option<&'a Permanent>,
    ) -> Vec<(&'a Permanent, ContinuousEffectTimestamp)> {
        let mut sources = self
            .battlefield
            .iter()
            .filter(|source| self.supplies_land_type_effect(source))
            .map(|source| (source, source.timestamp))
            .collect::<Vec<_>>();
        if let Some(prospective) = prospective
            && self.supplies_land_type_effect(prospective)
            && !sources
                .iter()
                .any(|(source, _)| source.card.id == prospective.card.id)
        {
            sources.push((prospective, self.prospective_continuous_effect_timestamp()));
        }
        sources
    }

    fn supplies_land_type_effect(&self, source: &Permanent) -> bool {
        self.effective_rules(source).is_some_and(|rules| {
            rules
                .ability_clauses()
                .iter()
                .copied()
                .chain(
                    source
                        .copy_effect
                        .iter()
                        .flat_map(|copy| copy.added_abilities.iter())
                        .map(|ability| ability.definition),
                )
                .any(|ability| {
                    ability.is_executable()
                        && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                        && ability
                            .declarative_effect()
                            .is_some_and(Self::effect_contains_land_type_operation)
                })
        })
    }

    fn effect_contains_land_type_operation(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::effect_contains_land_type_operation),
            EffectDef::Apply {
                effect,
                duration:
                    EffectDurationDef::WhileSourceRemainsInZone
                    | EffectDurationDef::UntilSourceLeavesZone,
                ..
            } => Self::applied_effect_contains_land_type_operation(effect),
            _ => false,
        }
    }

    fn applied_effect_contains_land_type_operation(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_contains_land_type_operation),
            AppliedEffectDef::AddLandTypes(_) | AppliedEffectDef::SetLandTypes(_) => true,
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::DoesNotUntapDuringUntapStep
            | AppliedEffectDef::CannotBeEnchanted
            | AppliedEffectDef::CannotBecomeEnchanted
            | AppliedEffectDef::CannotChangeController
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::PreventDamageFrom(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::GrantAbility(_)
            | AppliedEffectDef::RemoveAbilities(_)
            | AppliedEffectDef::Animate(_)
            | AppliedEffectDef::Special(_) => false,
        }
    }

    /// Whether any candidate setter targets `affected`, before suppressing a
    /// candidate whose own rules text another setter removes.
    fn raw_land_type_set_applies(
        &self,
        affected: &Permanent,
        sources: &[(&Permanent, ContinuousEffectTimestamp)],
    ) -> bool {
        sources.iter().any(|(source, timestamp)| {
            let mut operations = Vec::new();
            self.collect_land_type_operations_from_source(
                source,
                affected,
                *timestamp,
                &mut operations,
            );
            operations
                .iter()
                .any(|(_, operation)| matches!(operation, LandTypeOperation::SetTo(_)))
        })
    }

    pub(super) fn rules_text_abilities_removed(&self, affected: &Permanent) -> bool {
        let sources = self.land_type_effect_sources(None);
        self.rules_text_abilities_removed_from_sources(affected, &sources)
    }

    pub(super) fn rules_text_abilities_removed_with_prospective(
        &self,
        affected: &Permanent,
        prospective: &Permanent,
    ) -> bool {
        let sources = self.land_type_effect_sources(Some(prospective));
        self.rules_text_abilities_removed_from_sources(affected, &sources)
    }

    pub(super) fn rules_text_abilities_removed_from_sources(
        &self,
        affected: &Permanent,
        sources: &[(&Permanent, ContinuousEffectTimestamp)],
    ) -> bool {
        sources.iter().any(|(source, timestamp)| {
            if self.raw_land_type_set_applies(source, sources) {
                return false;
            }
            let mut operations = Vec::new();
            self.collect_land_type_operations_from_source(
                source,
                affected,
                *timestamp,
                &mut operations,
            );
            operations
                .iter()
                .any(|(_, operation)| matches!(operation, LandTypeOperation::SetTo(_)))
        })
    }

    fn collect_land_type_operations_from_source(
        &self,
        source: &Permanent,
        affected: &Permanent,
        source_timestamp: ContinuousEffectTimestamp,
        operations: &mut Vec<(ContinuousEffectTimestamp, LandTypeOperation)>,
    ) {
        let Some(rules) = self.effective_rules(source) else {
            return;
        };
        for ability in rules
            .ability_clauses()
            .iter()
            .copied()
            .chain(
                source
                    .copy_effect
                    .iter()
                    .flat_map(|copy| copy.added_abilities.iter())
                    .map(|ability| ability.definition),
            )
            .filter(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                    && ability.declarative_effect().is_some()
            })
        {
            self.collect_land_type_operations_from_effect(
                ability
                    .declarative_effect()
                    .expect("filtered to declarative effects"),
                source,
                affected,
                source_timestamp,
                operations,
            );
        }
    }

    fn collect_land_type_operations_from_effect(
        &self,
        effect: EffectDef,
        source: &Permanent,
        affected: &Permanent,
        source_timestamp: ContinuousEffectTimestamp,
        operations: &mut Vec<(ContinuousEffectTimestamp, LandTypeOperation)>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.collect_land_type_operations_from_effect(
                        *effect,
                        source,
                        affected,
                        source_timestamp,
                        operations,
                    );
                }
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration:
                    EffectDurationDef::WhileSourceRemainsInZone
                    | EffectDurationDef::UntilSourceLeavesZone,
            } if self.land_type_recipient_matches(recipient, source, affected) => {
                Self::collect_applied_land_type_operations(effect, source_timestamp, operations);
            }
            _ => {}
        }
    }

    fn collect_applied_land_type_operations(
        effect: AppliedEffectDef,
        source: ContinuousEffectTimestamp,
        operations: &mut Vec<(ContinuousEffectTimestamp, LandTypeOperation)>,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::collect_applied_land_type_operations(*effect, source, operations);
                }
            }
            AppliedEffectDef::AddLandTypes(types) => {
                operations.push((source, LandTypeOperation::Add(types)));
            }
            AppliedEffectDef::SetLandTypes(types) => {
                operations.push((source, LandTypeOperation::SetTo(types)));
            }
            AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::DoesNotUntapDuringUntapStep
            | AppliedEffectDef::CannotBeEnchanted
            | AppliedEffectDef::CannotBecomeEnchanted
            | AppliedEffectDef::CannotChangeController
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::PreventDamageFrom(_)
            | AppliedEffectDef::Animate(_)
            | AppliedEffectDef::ModifyPowerToughness { .. }
            | AppliedEffectDef::GrantAbility(_)
            | AppliedEffectDef::RemoveAbilities(_)
            | AppliedEffectDef::Special(_) => {}
        }
    }

    fn land_type_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> bool {
        match recipient {
            EffectRecipientDef::Source => source.card.id == affected.card.id,
            EffectRecipientDef::AttachedPermanent => source.attached_to == Some(affected.card.id),
            EffectRecipientDef::MatchingObjects {
                object,
                zones,
                controller,
            } => {
                zones.contains(&ZoneKind::Battlefield)
                    && self.player_relation_matches(
                        affected.controller,
                        controller,
                        source.controller,
                        TriggerContext::empty(),
                    )
                    && self.land_type_object_predicate_matches(object, source, affected)
            }
            EffectRecipientDef::ChosenPermanent(_)
            | EffectRecipientDef::ControllerOfTarget(_)
            | EffectRecipientDef::ObjectsControlledByTarget { .. }
            | EffectRecipientDef::ObjectsOwnedByTarget { .. }
            | EffectRecipientDef::CardsOwnedByTarget { .. }
            | EffectRecipientDef::Controller
            | EffectRecipientDef::Opponent
            | EffectRecipientDef::EachPlayer
            | EffectRecipientDef::Target(_)
            | EffectRecipientDef::ObjectsSharingNameWithTarget(_)
            | EffectRecipientDef::TriggeringObject
            | EffectRecipientDef::ControllerOfTriggeringObject
            | EffectRecipientDef::EventPlayer => false,
        }
    }

    fn land_type_object_predicate_matches(
        &self,
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Any => true,
            ObjectPredicateDef::Source => source.card.id == affected.card.id,
            ObjectPredicateDef::Token => self.is_token(affected.card.definition),
            ObjectPredicateDef::HasType(card_type) => self
                .permanent_types(affected)
                .is_some_and(|types| types.contains(card_type)),
            ObjectPredicateDef::Supertype(supertype) => self
                .effective_rules(affected)
                .is_some_and(|rules| rules.has_supertype(supertype)),
            ObjectPredicateDef::All(predicates) => predicates.iter().all(|predicate| {
                self.land_type_object_predicate_matches(*predicate, source, affected)
            }),
            ObjectPredicateDef::AnyOf(predicates) => predicates.iter().any(|predicate| {
                self.land_type_object_predicate_matches(*predicate, source, affected)
            }),
            ObjectPredicateDef::Not(predicate) => {
                !self.land_type_object_predicate_matches(*predicate, source, affected)
            }
            ObjectPredicateDef::HasAnyBasicLandType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::ColorCount(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
            | ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::DebutSet(_)
            | ObjectPredicateDef::SharesNameWithSource
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::HasNonManaActivatedAbility
            | ObjectPredicateDef::Special(_) => false,
        }
    }

    /// Ordered subtypes after the continuous effects currently modeled by the
    /// engine. Layer-3 text changes apply to the copied/printed line first;
    /// timestamp-ordered layer-4 Set/Add operations then model Blood Moon and
    /// Aura-granted basic land types. Nonland subtypes such as Dryad survive.
    pub(super) fn effective_subtypes(&self, permanent: &Permanent) -> Cow<'static, [&'static str]> {
        let operations = self.land_type_operations(permanent);
        self.effective_subtypes_with_operations(permanent, operations)
    }

    pub(super) fn effective_subtypes_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> Cow<'static, [&'static str]> {
        let operations = self.land_type_operations_with_prospective(permanent, prospective);
        self.effective_subtypes_with_operations(permanent, operations)
    }

    fn effective_subtypes_with_operations(
        &self,
        permanent: &Permanent,
        operations: Vec<(ContinuousEffectTimestamp, LandTypeOperation)>,
    ) -> Cow<'static, [&'static str]> {
        fn is_land_subtype(subtype: &str) -> bool {
            LAND_SUBTYPES.contains(&subtype)
        }

        let Some(rules) = self.effective_rules(permanent) else {
            return Cow::Borrowed(&[]);
        };
        let animation = permanent
            .animation
            .filter(|animation| animation.all_creature_types || !animation.subtypes.is_empty());
        if permanent.text_changes.is_empty() && operations.is_empty() && animation.is_none() {
            return Cow::Borrowed(rules.subtypes());
        }

        let mut subtypes = rules.subtypes().to_vec();
        for change in &permanent.text_changes {
            for subtype in &mut subtypes {
                if BasicLandType::from_subtype(subtype) == Some(change.from) {
                    *subtype = change.to.subtype();
                }
            }
        }

        let mut seen = [false; BasicLandType::ALL.len()];
        subtypes.retain(|subtype| {
            let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                return true;
            };
            let keep = !seen[land_type.index()];
            seen[land_type.index()] = true;
            keep
        });

        for (_, operation) in operations {
            match operation {
                LandTypeOperation::SetTo(types) => {
                    let mut insertion = subtypes
                        .iter()
                        .position(|subtype| is_land_subtype(subtype))
                        .unwrap_or(0);
                    subtypes.retain(|subtype| !is_land_subtype(subtype));
                    insertion = insertion.min(subtypes.len());
                    for land_type in types {
                        if subtypes
                            .iter()
                            .any(|subtype| BasicLandType::from_subtype(subtype) == Some(*land_type))
                        {
                            continue;
                        }
                        subtypes.insert(insertion, land_type.subtype());
                        insertion += 1;
                    }
                }
                LandTypeOperation::Add(types) => {
                    let mut insertion = subtypes
                        .iter()
                        .position(|subtype| !is_land_subtype(subtype))
                        .unwrap_or(subtypes.len());
                    for land_type in types {
                        if subtypes
                            .iter()
                            .any(|subtype| BasicLandType::from_subtype(subtype) == Some(*land_type))
                        {
                            continue;
                        }
                        subtypes.insert(insertion, land_type.subtype());
                        insertion += 1;
                    }
                }
            }
        }
        if let Some(animation) = animation {
            if animation.replaces_subtypes {
                subtypes.clear();
            }
            if animation.all_creature_types {
                subtypes.extend(CREATURE_TYPES.iter().copied());
            }
            for subtype in animation.subtypes {
                if !subtypes.contains(subtype) {
                    subtypes.push(subtype);
                }
            }
        }
        Cow::Owned(subtypes)
    }

    /// Basic land subtypes in effective type-line order, with duplicate types
    /// collapsed before the rules grant one intrinsic ability for each type.
    fn visit_effective_basic_land_types(
        &self,
        permanent: &Permanent,
        mut visitor: impl FnMut(BasicLandType) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        if !self
            .permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Land))
        {
            return ControlFlow::Continue(());
        }

        let mut present = [false; BasicLandType::ALL.len()];
        for subtype in self.effective_subtypes(permanent).iter() {
            let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                continue;
            };
            if present[land_type.index()] {
                continue;
            }
            present[land_type.index()] = true;
            if visitor(land_type).is_break() {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }

    /// Basic land subtypes after the continuous effects currently modeled by
    /// the engine.
    pub(super) fn effective_land_types(&self, permanent: &Permanent) -> [bool; 5] {
        let mut types = [false; 5];
        let result = self.visit_effective_basic_land_types(permanent, |land_type| {
            types[land_type.index()] = true;
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
        types
    }
}
