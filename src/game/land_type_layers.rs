use super::continuous_effects::StaticEffectKind;
use super::continuous_effects::StaticSetCharacteristicLayerGuard;
use super::{
    AppliedEffectDef, BasicLandType, CREATURE_TYPES, CardType, CharacteristicOperationDef,
    ContinuousEffectTimestamp, ControlFlow, Cow, CreatureTypeSetDef, DeclarativeAbilityDef,
    EffectDef, EffectRecipientDef, EffectRecipientSetDef, Game, LandTypeOperation, ObjectKind,
    ObjectPredicateDef, ObjectRefDef, ObjectSetDef, Permanent, ResolvedContinuousEffectKind,
    SetOperationDef, TriggerContext, ZoneKind,
};
use crate::card::LAND_SUBTYPES;

#[derive(Clone)]
enum SubtypeLayerOperation {
    BasicLand(LandTypeOperation),
    Creature(SetOperationDef<CreatureTypeSetDef>),
    Named(SetOperationDef<&'static [&'static str]>),
    /// The same as adding named subtypes, over a list a copy carries rather
    /// than one a card printed. Owned because the copy's exceptions are
    /// interned per game rather than authored as a static slice.
    AddedNamed(Vec<&'static str>),
}

impl Game {
    fn land_type_operations(
        &self,
        permanent: &Permanent,
    ) -> Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)> {
        self.land_type_operations_from_sources(permanent, None)
    }

    fn land_type_operations_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)> {
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
    ) -> Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)> {
        let sources = self.land_type_effect_sources(prospective);

        let resolved = prospective
            .filter(|prospective| prospective.card.id == affected.card.id)
            .unwrap_or(affected);
        let mut operations = resolved
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::BasicLandTypes(operation) => Some((
                    effect.timestamp,
                    effect.component_order,
                    Self::resolved_land_type_operation(operation),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        for (source, timestamp) in &sources {
            if self.resolved_land_type_set_applies(source)
                || self.raw_land_type_set_applies(source, &sources)
            {
                continue;
            }
            self.collect_land_type_operations_from_source(
                source,
                affected,
                *timestamp,
                &mut operations,
            );
        }
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        operations
    }

    const fn resolved_land_type_operation(
        operation: SetOperationDef<&'static [BasicLandType]>,
    ) -> LandTypeOperation {
        match operation {
            SetOperationDef::Add(types) => LandTypeOperation::Add(types),
            SetOperationDef::Remove(types) => LandTypeOperation::Remove(types),
            SetOperationDef::Set(types) => LandTypeOperation::SetTo(types),
        }
    }

    fn resolved_land_type_set_applies(&self, permanent: &Permanent) -> bool {
        permanent.resolved_continuous_effects.iter().any(|effect| {
            self.resolved_continuous_effect_is_active(effect)
                && matches!(
                    effect.kind,
                    ResolvedContinuousEffectKind::BasicLandTypes(SetOperationDef::Set(_))
                )
        })
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
        self.with_effective_rules(source, |rules| {
            rules
                .ability_clauses()
                .iter()
                .copied()
                .chain(
                    source
                        .active_copy_values()
                        .into_iter()
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
        .unwrap_or(false)
    }

    fn effect_contains_land_type_operation(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::effect_contains_land_type_operation),
            EffectDef::IfCondition { then, .. } => Self::effect_contains_land_type_operation(*then),
            EffectDef::StaticApply { effect, .. } => {
                Self::applied_effect_contains_land_type_operation(effect)
            }
            _ => false,
        }
    }

    fn applied_effect_contains_land_type_operation(effect: AppliedEffectDef) -> bool {
        match effect {
            AppliedEffectDef::Composite(effects) => effects
                .iter()
                .copied()
                .any(Self::applied_effect_contains_land_type_operation),
            AppliedEffectDef::Characteristic(
                CharacteristicOperationDef::BasicLandTypes(_)
                | CharacteristicOperationDef::ChosenBasicLandType,
            ) => true,
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
        }
    }

    /// Whether any candidate setter targets `affected`, before suppressing a
    /// candidate whose own rules text another setter removes.
    ///
    /// A land that sets its own type is not one of those candidates. What
    /// CR 305.7 silences is a land whose types somebody else replaced; an
    /// ability cannot be the reason it is itself ignored, or Multiversal
    /// Passage would never be anything at all.
    fn raw_land_type_set_applies(
        &self,
        affected: &Permanent,
        sources: &[(&Permanent, ContinuousEffectTimestamp)],
    ) -> bool {
        sources
            .iter()
            .filter(|(source, _)| source.card.id != affected.card.id)
            .any(|(source, timestamp)| {
                let mut operations = Vec::new();
                self.collect_land_type_operations_from_source(
                    source,
                    affected,
                    *timestamp,
                    &mut operations,
                );
                operations.iter().any(|(_, _, operation)| {
                    matches!(
                        operation,
                        LandTypeOperation::SetTo(_) | LandTypeOperation::SetToChosen(_)
                    )
                })
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
        if self.resolved_land_type_set_applies(affected) {
            return true;
        }
        sources.iter().any(|(source, timestamp)| {
            if self.resolved_land_type_set_applies(source)
                || self.raw_land_type_set_applies(source, sources)
            {
                return false;
            }
            let mut operations = Vec::new();
            self.collect_land_type_operations_from_source(
                source,
                affected,
                *timestamp,
                &mut operations,
            );
            operations.iter().any(|(_, _, operation)| {
                matches!(
                    operation,
                    LandTypeOperation::SetTo(_) | LandTypeOperation::SetToChosen(_)
                )
            })
        })
    }

    fn collect_land_type_operations_from_source(
        &self,
        source: &Permanent,
        affected: &Permanent,
        source_timestamp: ContinuousEffectTimestamp,
        operations: &mut Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)>,
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
                    .active_copy_values()
                    .into_iter()
                    .flat_map(|copy| copy.added_abilities.iter())
                    .map(|ability| ability.definition),
            )
            .filter(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                    && ability.declarative_effect().is_some()
            })
        {
            let mut component_order = 0;
            self.collect_land_type_operations_from_effect(
                ability
                    .declarative_effect()
                    .expect("filtered to declarative effects"),
                source,
                affected,
                source_timestamp,
                &mut component_order,
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
        component_order: &mut u16,
        operations: &mut Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)>,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.collect_land_type_operations_from_effect(
                        *effect,
                        source,
                        affected,
                        source_timestamp,
                        component_order,
                        operations,
                    );
                }
            }
            EffectDef::IfCondition { condition, then }
                if self.trigger_condition_holds(
                    condition,
                    source.card.id,
                    source.controller,
                    TriggerContext::empty(),
                    None,
                    None,
                ) =>
            {
                self.collect_land_type_operations_from_effect(
                    *then,
                    source,
                    affected,
                    source_timestamp,
                    component_order,
                    operations,
                );
            }
            EffectDef::StaticApply { recipient, effect }
                if self.land_type_recipient_matches(recipient, source, affected) =>
            {
                Self::collect_applied_land_type_operations(
                    effect,
                    source_timestamp,
                    source.chosen_basic_land_type,
                    component_order,
                    operations,
                );
            }
            _ => {}
        }
    }

    fn collect_applied_land_type_operations(
        effect: AppliedEffectDef,
        source: ContinuousEffectTimestamp,
        chosen: Option<BasicLandType>,
        component_order: &mut u16,
        operations: &mut Vec<(ContinuousEffectTimestamp, u16, LandTypeOperation)>,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    Self::collect_applied_land_type_operations(
                        *effect,
                        source,
                        chosen,
                        component_order,
                        operations,
                    );
                }
            }
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(
                operation,
            )) => {
                let order = *component_order;
                *component_order = component_order
                    .checked_add(1)
                    .expect("one static ability contains at most 65,536 components");
                operations.push((source, order, Self::resolved_land_type_operation(operation)));
            }
            // A permanent that was never told which type to be says nothing
            // at all, which is what a Multiversal Passage put onto the
            // battlefield without choosing comes to.
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::ChosenBasicLandType) => {
                if let Some(chosen) = chosen {
                    let order = *component_order;
                    *component_order = component_order
                        .checked_add(1)
                        .expect("one static ability contains at most 65,536 components");
                    operations.push((source, order, LandTypeOperation::SetToChosen(chosen)));
                }
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => {}
        }
    }

    fn land_type_recipient_matches(
        &self,
        recipient: EffectRecipientDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> bool {
        match recipient.0 {
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::Source)) => {
                source.card.id == affected.card.id
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::AttachedToSource)) => {
                source.attached_to == Some(affected.card.id)
            }
            EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
                query.zones.contains(&ZoneKind::Battlefield)
                    && self.query_player_constraints_match(
                        Some(affected.controller),
                        affected.card.owner,
                        query,
                        (source.controller, source.card.id),
                        TriggerContext::empty(),
                        None,
                    )
                    && self.land_type_object_predicate_matches(query.object, source, affected)
            }
            EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
            | EffectRecipientSetDef::LegalTargets(_)
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
                | ObjectSetDef::LegalTargets(_)
                | ObjectSetDef::PermanentsTargetedBy(_)
                | ObjectSetDef::LegalAttachmentHosts(_)
                | ObjectSetDef::LinkedExiles(_)
                | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                | ObjectSetDef::PermanentsControlledBy(_)
                | ObjectSetDef::BottomOfGraveyard(_)
                | ObjectSetDef::SharingNameWith(_)
                | ObjectSetDef::SharingNameWithBinding { .. }
                | ObjectSetDef::TopOfGraveyardMatching { .. },
            )
            // A static clause names what it affects outright; nothing static
            // is scoped to what a creature happens to be attacking.
            | EffectRecipientSetDef::DefenderOf(_)
            | EffectRecipientSetDef::Players(_) => false,
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
            ObjectPredicateDef::Token => affected.card.definition.is_token(),
            ObjectPredicateDef::HasType(CardType::Land) => self
                .permanent_types_below_static_effects(affected)
                .is_some_and(|types| types.contains(CardType::Land)),
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
            ObjectPredicateDef::HasSourcesChosenScalar(
                crate::card::BattlefieldEntryChoiceDestinationDef::CardName,
            ) => source.chosen_card_name.as_deref().is_some_and(|chosen| {
                self.object_card_name(affected.card.id)
                    .is_some_and(|actual| actual == chosen)
            }),
            ObjectPredicateDef::HasAnyBasicLandType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::ColorCount(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::Named(_)
            | ObjectPredicateDef::HasChosenName
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
            | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
            | ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::PowerGreaterThan(_)
            | ObjectPredicateDef::PowerLessThan(_)
            | ObjectPredicateDef::ToughnessGreaterThanItsPower
            | ObjectPredicateDef::ToughnessGreaterThan(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::OwnedBy(_)
            | ObjectPredicateDef::DebutSet(_)
            | ObjectPredicateDef::HasName(_)
            | ObjectPredicateDef::HasSourcesChosenScalar(_)
            | ObjectPredicateDef::TargetsObjectMatching(_)
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::HasAbility(_)
            | ObjectPredicateDef::HasCounter(_)
            | ObjectPredicateDef::HasAnyCounter
            | ObjectPredicateDef::CounterCount { .. }
            | ObjectPredicateDef::Tapped
            | ObjectPredicateDef::WasDealtDamageThisTurn
            | ObjectPredicateDef::DealtDamageThisTurn
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::Saddled
            | ObjectPredicateDef::Blocking
            | ObjectPredicateDef::BlockedBySource
            | ObjectPredicateDef::BlockingSource
            | ObjectPredicateDef::BandedWithSource
            | ObjectPredicateDef::Unpaired
            | ObjectPredicateDef::PairedWithSource
            | ObjectPredicateDef::Enchanted
            | ObjectPredicateDef::AttachedTo(_)
            | ObjectPredicateDef::AttachedToSource
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::CameUnderControlThisTurn
            | ObjectPredicateDef::EnteredThisTurn
            | ObjectPredicateDef::AttackedDuringControllersLastTurn
            | ObjectPredicateDef::HasNonManaActivatedAbility
            | ObjectPredicateDef::Special(_) => false,
        }
    }

    /// Ordered subtypes after the continuous effects currently modeled by the
    /// engine. Layer-3 text changes apply to the copied/printed line first;
    /// timestamp-ordered layer-4 Set/Add operations then model Blood Moon and
    /// Aura-granted basic land types. Nonland subtypes such as Dryad survive.
    pub(super) fn effective_subtypes(&self, permanent: &Permanent) -> Cow<'static, [&'static str]> {
        let operations = self.subtype_layer_operations(permanent, None);
        self.effective_subtypes_with_operations(permanent, operations)
    }

    pub(super) fn effective_subtypes_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> Cow<'static, [&'static str]> {
        let operations = self.subtype_layer_operations(permanent, Some(prospective));
        self.effective_subtypes_with_operations(permanent, operations)
    }

    fn subtype_layer_operations(
        &self,
        permanent: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Vec<(ContinuousEffectTimestamp, u16, SubtypeLayerOperation)> {
        let characteristic = prospective
            .filter(|prospective| prospective.card.id == permanent.card.id)
            .unwrap_or(permanent);
        let land_operations = prospective.map_or_else(
            || self.land_type_operations(permanent),
            |prospective| self.land_type_operations_with_prospective(permanent, prospective),
        );
        let mut operations = land_operations
            .into_iter()
            .map(|(timestamp, order, operation)| {
                (
                    timestamp,
                    order,
                    SubtypeLayerOperation::BasicLand(operation),
                )
            })
            .collect::<Vec<_>>();
        operations.extend(
            characteristic
                .resolved_continuous_effects
                .iter()
                .filter(|effect| self.resolved_continuous_effect_is_active(effect))
                .filter_map(|effect| match effect.kind {
                    ResolvedContinuousEffectKind::CreatureTypes(operation) => Some((
                        effect.timestamp,
                        effect.component_order,
                        SubtypeLayerOperation::Creature(operation),
                    )),
                    ResolvedContinuousEffectKind::Subtypes(operation) => Some((
                        effect.timestamp,
                        effect.component_order,
                        SubtypeLayerOperation::Named(operation),
                    )),
                    _ => None,
                }),
        );
        // "Except it's a Zombie ...": a copy exception, added on top of the
        // types it copied rather than replacing them.
        if let Some(added) = characteristic
            .active_copy_values()
            .map(|copy| copy.added_creature_types.clone())
            .filter(|added| !added.is_empty())
        {
            operations.push((
                characteristic.timestamp,
                0,
                SubtypeLayerOperation::AddedNamed(added),
            ));
        }
        // The other half of bestow's type change: while attached it is an
        // Aura, which is the subtype the enchantment half needs.
        if Self::is_bestowed_aura(characteristic) {
            operations.push((
                characteristic.timestamp,
                u16::MAX,
                SubtypeLayerOperation::Named(SetOperationDef::Add(&["Aura"])),
            ));
        }
        if let Some(_pass) = StaticSetCharacteristicLayerGuard::enter() {
            let mut collect = |applied: super::StaticAppliedEffect| {
                match applied.effect {
                    AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::CreatureTypes(operation),
                    ) => operations.push((
                        applied.timestamp,
                        applied.component_order,
                        SubtypeLayerOperation::Creature(operation),
                    )),
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        operation,
                    )) => operations.push((
                        applied.timestamp,
                        applied.component_order,
                        SubtypeLayerOperation::Named(operation),
                    )),
                    _ => {}
                }
                ControlFlow::Continue(())
            };
            let result = if let Some(prospective) = prospective {
                self.visit_static_applied_effects_with_prospective(
                    permanent,
                    prospective,
                    StaticEffectKind::Subtypes,
                    &mut collect,
                )
            } else {
                self.visit_static_applied_effects(
                    permanent,
                    StaticEffectKind::Subtypes,
                    &mut collect,
                )
            };
            debug_assert!(result.is_continue());
        }
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        operations
    }

    /// The copying card's own printed subtypes, when the copy effect kept
    /// them: "except it's an Illusion in addition to its other types" names
    /// the subtype line the card already prints.
    fn retained_printed_subtypes(&self, permanent: &Permanent) -> &'static [&'static str] {
        let Some(_) = permanent
            .active_copy_values()
            .filter(|copy| copy.retain_printed_subtypes)
        else {
            return &[];
        };
        match permanent.card.definition {
            ObjectKind::Card(definition) => self
                .catalog
                .get(definition)
                .and_then(|card| card.part(permanent.presented))
                .map_or(&[], |part| part.rules.subtypes()),
            ObjectKind::Token => permanent
                .token_characteristics
                .and_then(|token| token.part(permanent.presented))
                .map_or(&[], |part| part.rules.subtypes()),
            ObjectKind::Emblem | ObjectKind::Ability => &[],
        }
    }

    /// Applies the subtype layer's operations in timestamp order. Split
    /// from the reader above because the two are different jobs: one works
    /// out the starting line, and this one edits it.
    /// One layer-4 land-subtype operation. Split out of the walk above for
    /// the source-size budget; the three shapes are the ones CR 305.7 gives
    /// a land's types.
    fn apply_basic_land_subtype_operation(
        subtypes: &mut Vec<&'static str>,
        operation: LandTypeOperation,
    ) {
        fn is_land_subtype(subtype: &str) -> bool {
            LAND_SUBTYPES.contains(&subtype)
        }

        match operation {
            // One type or several, the set is the same operation: every land
            // subtype it had goes, and these take their place.
            LandTypeOperation::SetTo(_) | LandTypeOperation::SetToChosen(_) => {
                let chosen = [match operation {
                    LandTypeOperation::SetToChosen(chosen) => chosen,
                    _ => BasicLandType::Plains,
                }];
                let types: &[BasicLandType] = match operation {
                    LandTypeOperation::SetTo(types) => types,
                    _ => &chosen,
                };
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
            LandTypeOperation::Remove(types) => {
                subtypes.retain(|subtype| {
                    BasicLandType::from_subtype(subtype)
                        .is_none_or(|land_type| !types.contains(&land_type))
                });
            }
        }
    }

    fn apply_subtype_operations(
        subtypes: &mut Vec<&'static str>,
        operations: Vec<(ContinuousEffectTimestamp, u16, SubtypeLayerOperation)>,
    ) {
        for (_, _, operation) in operations {
            match operation {
                SubtypeLayerOperation::BasicLand(operation) => {
                    Self::apply_basic_land_subtype_operation(subtypes, operation);
                }
                SubtypeLayerOperation::Creature(operation) => {
                    let (types, removes_existing, removes_named) = match operation {
                        SetOperationDef::Add(types) => (types, false, false),
                        SetOperationDef::Remove(types) => (types, false, true),
                        SetOperationDef::Set(types) => (types, true, false),
                    };
                    if removes_existing {
                        subtypes.retain(|subtype| !CREATURE_TYPES.contains(subtype));
                    } else if removes_named {
                        subtypes.retain(|subtype| {
                            !(types.named.contains(subtype)
                                || types.all && CREATURE_TYPES.contains(subtype))
                        });
                        continue;
                    }
                    if types.all {
                        for creature_type in CREATURE_TYPES {
                            if !subtypes.contains(creature_type) {
                                subtypes.push(creature_type);
                            }
                        }
                    }
                    for subtype in types.named {
                        if !subtypes.contains(subtype) {
                            subtypes.push(subtype);
                        }
                    }
                }
                SubtypeLayerOperation::AddedNamed(types) => {
                    for subtype in types {
                        if !subtypes.contains(&subtype) {
                            subtypes.push(subtype);
                        }
                    }
                }
                SubtypeLayerOperation::Named(operation) => match operation {
                    SetOperationDef::Add(types) => {
                        for subtype in types {
                            if !subtypes.contains(subtype) {
                                subtypes.push(subtype);
                            }
                        }
                    }
                    SetOperationDef::Remove(types) => {
                        subtypes.retain(|subtype| !types.contains(subtype));
                    }
                    SetOperationDef::Set(types) => {
                        subtypes.clear();
                        for subtype in types {
                            if !subtypes.contains(subtype) {
                                subtypes.push(subtype);
                            }
                        }
                    }
                },
            }
        }
    }

    fn effective_subtypes_with_operations(
        &self,
        permanent: &Permanent,
        operations: Vec<(ContinuousEffectTimestamp, u16, SubtypeLayerOperation)>,
    ) -> Cow<'static, [&'static str]> {
        let Some(rules) = self.effective_rules(permanent) else {
            return Cow::Borrowed(&[]);
        };
        let retained = self.retained_printed_subtypes(permanent);
        if permanent.text_changes.is_empty()
            && operations.is_empty()
            && retained.is_empty()
            && !self.has_subtypes_without_their_card_type(permanent, rules.subtypes())
        {
            return Cow::Borrowed(rules.subtypes());
        }

        let mut subtypes = rules.subtypes().to_vec();
        for subtype in retained {
            if !subtypes.contains(subtype) {
                subtypes.push(subtype);
            }
        }
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

        Self::apply_subtype_operations(&mut subtypes, operations);
        self.drop_subtypes_without_their_card_type(permanent, &mut subtypes);
        Cow::Owned(subtypes)
    }

    /// A subtype belongs to a card type, and goes when that type does
    /// (CR 205.1b): an Enduring Innocence that comes back as an enchantment
    /// is no longer a Sheep Glimmer, because it is no longer a creature.
    ///
    /// Audit: partial -- creature types only. The other kinds of subtype are
    /// carried on cards whose type line the engine never takes that type
    /// away from, so nothing in the catalog can tell the difference yet.
    fn drop_subtypes_without_their_card_type(
        &self,
        permanent: &Permanent,
        subtypes: &mut Vec<&'static str>,
    ) {
        if !self.is_a_creature_permanent(permanent) {
            subtypes.retain(|subtype| crate::card::creature_type_name(subtype).is_none());
        }
    }

    /// Whether any of these subtypes has lost the card type it belongs to,
    /// which is what decides whether the printed list can be handed back as
    /// it stands.
    fn has_subtypes_without_their_card_type(
        &self,
        permanent: &Permanent,
        subtypes: &[&'static str],
    ) -> bool {
        !self.is_a_creature_permanent(permanent)
            && subtypes
                .iter()
                .any(|subtype| crate::card::creature_type_name(subtype).is_some())
    }

    fn is_a_creature_permanent(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Creature))
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
