impl Game {
    /// Matches the common characteristic predicates used by live static
    /// effects without eagerly assembling every characteristic layer.
    ///
    /// The general trigger matcher still owns the complete predicate
    /// vocabulary. Falling back to its snapshot preserves that behavior for
    /// predicates that need values such as power, keywords, or battlefield
    /// relationships; simple type, color, and subtype queries only compute
    /// the layers they actually inspect.
    fn static_object_predicate_matches(
        &self,
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> bool {
        self.lazy_match(predicate, source, affected, prospective)
            .unwrap_or_else(|| {
                self.trigger_object_matches(
                    predicate,
                    &prospective.map_or_else(
                        || self.trigger_event_object(affected),
                        |prospective| {
                            self.trigger_event_object_with_prospective(affected, prospective)
                        },
                    ),
                    source.card.id,
                    false,
                )
            })
    }

    /// `None` means that this predicate needs the complete trigger snapshot.
    /// Composite predicates retain useful short-circuit answers: one known
    /// false arm decides `All`, and one known true arm decides `AnyOf`, even
    /// when a different arm would require the fallback.
    /// Whether a land carries any of these basic types, reading the subtype
    /// layer rather than the printed line.
    fn static_basic_land_type_matches(
        &self,
        land_types: &[crate::card::BasicLandType],
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Option<bool> {
        if !self.permanent_types(affected)?.contains(CardType::Land) {
            return Some(false);
        }
        let subtypes = prospective.map_or_else(
            || self.effective_subtypes(affected),
            |prospective| self.effective_subtypes_with_prospective(affected, prospective),
        );
        Some(
            land_types
                .iter()
                .any(|land_type| subtypes.contains(&land_type.subtype())),
        )
    }

    /// The predicates that need nothing beyond the two permanents in hand.
    /// Split from the walker below only to keep either readable; together
    /// they are one decision about what a shortcut can answer.
    fn static_leaf_predicate_matches_lazily(
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> Option<bool> {
        match predicate {
            ObjectPredicateDef::Any => Some(true),
            ObjectPredicateDef::Source => Some(source.card.id == affected.card.id),
            ObjectPredicateDef::Token => Some(affected.card.definition.is_token()),
            ObjectPredicateDef::Tapped => Some(affected.tapped),
            ObjectPredicateDef::WasDealtDamageThisTurn => Some(affected.was_dealt_damage_this_turn),
            ObjectPredicateDef::DealtDamageThisTurn => Some(affected.dealt_damage_this_turn),
            ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Ability
            | ObjectPredicateDef::ActivatedAbility
            | ObjectPredicateDef::TriggeredAbility
            | ObjectPredicateDef::DeclaredTargetCount { .. }
            | ObjectPredicateDef::HasDeclaredTarget(_)
            | ObjectPredicateDef::HasDeclaredPlayerTarget(_) => Some(false),
            ObjectPredicateDef::Unpaired => Some(affected.paired_with.is_none()),
            // Symmetric, so reading it off the source is the same answer.
            ObjectPredicateDef::PairedWithSource => {
                Some(source.paired_with == Some(affected.card.id))
            }
            _ => None,
        }
    }

    fn static_name_predicate_matches_lazily(
        &self,
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
    ) -> Option<bool> {
        match predicate {
            ObjectPredicateDef::NameEquals(name) => self
                .source_card_name(name, source.card.id)
                .zip(self.object_card_name(affected.card.id))
                .map(|(expected, actual)| actual == expected),
            ObjectPredicateDef::NameIn(names) => self
                .object_card_name(affected.card.id)
                .map(|actual| {
                    self.source_card_name_set(*names, source.card.id)
                        .contains(actual.as_ref())
                }),
            _ => None,
        }
    }

    fn lazy_match(
        &self,
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Option<bool> {
        if let Some(answer) = Self::static_leaf_predicate_matches_lazily(predicate, source, affected)
        {
            return Some(answer);
        }
        let nested = |predicate| self.lazy_match(predicate, source, affected, prospective);
        match predicate {
            ObjectPredicateDef::HasAnyBasicLandType(land_types) => {
                self.static_basic_land_type_matches(land_types, affected, prospective)
            }
            ObjectPredicateDef::HasType(card_type) => self
                .permanent_types(affected)
                .map(|types| types.contains(card_type)),
            ObjectPredicateDef::Color(color) => {
                let rules = self.effective_rules(affected)?;
                let colors = self.effective_colors(affected, &rules);
                Some(color.color_index().is_some_and(|index| colors[index]))
            }
            ObjectPredicateDef::ColorCount(count) => {
                let rules = self.effective_rules(affected)?;
                let colors = self.effective_colors(affected, &rules);
                Some(colors.iter().filter(|present| **present).count() == usize::from(count))
            }
            ObjectPredicateDef::Subtype(subtype) => {
                let subtypes = prospective.map_or_else(
                    || self.effective_subtypes(affected),
                    |prospective| self.effective_subtypes_with_prospective(affected, prospective),
                );
                Some(subtypes.contains(&subtype))
            }
            ObjectPredicateDef::Supertype(supertype) => {
                self.static_supertype_matches(supertype, affected, prospective)
            }
            predicate @ (ObjectPredicateDef::All(_) | ObjectPredicateDef::AnyOf(_)) => {
                self.static_composite_predicate_match_lazily(
                    predicate,
                    source,
                    affected,
                    prospective,
                )
            }
            ObjectPredicateDef::Not(predicate) => nested(*predicate).map(|matches| !matches),
            predicate @ (ObjectPredicateDef::NameEquals(_) | ObjectPredicateDef::NameIn(_)) => {
                self.static_name_predicate_matches_lazily(predicate, source, affected)
            }
            ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::GenericManaCostAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
            | ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::PowerGreaterThan(_)
            | ObjectPredicateDef::ToughnessGreaterThan(_)
            | ObjectPredicateDef::PowerLessThan(_)
            | ObjectPredicateDef::ToughnessGreaterThanItsPower
            | ObjectPredicateDef::HasCounter(_)
            | ObjectPredicateDef::HasAnyCounter
            | ObjectPredicateDef::CounterCount { .. }
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::OwnedBy(_)
            | ObjectPredicateDef::DebutSet(_)
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::HasAbility(_)
            | ObjectPredicateDef::HasNonManaActivatedAbility
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::Saddled
            | ObjectPredicateDef::AttachedToSource
            | ObjectPredicateDef::Blocking
            | ObjectPredicateDef::BlockedBySource
            | ObjectPredicateDef::BlockingSource
            | ObjectPredicateDef::BandedWithSource
            | ObjectPredicateDef::Enchanted
            | ObjectPredicateDef::AttachedTo(_)
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::CameUnderControlThisTurn
            | ObjectPredicateDef::EnteredThisTurn
            | ObjectPredicateDef::AttackedDuringControllersLastTurn
            // These need the complete snapshot, or were answered by the leaf
            // helper, and are listed so new predicates cannot fall through.
            | ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::Token
            | ObjectPredicateDef::Tapped
            | ObjectPredicateDef::WasDealtDamageThisTurn
            | ObjectPredicateDef::DealtDamageThisTurn
            | ObjectPredicateDef::Unpaired
            | ObjectPredicateDef::PairedWithSource
            | ObjectPredicateDef::HasSourcesChosenScalar(_)
            | ObjectPredicateDef::TargetsObjectMatching(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Ability
            | ObjectPredicateDef::ActivatedAbility
            | ObjectPredicateDef::TriggeredAbility
            | ObjectPredicateDef::DeclaredTargetCount { .. }
            | ObjectPredicateDef::HasDeclaredTarget(_)
            | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
            | ObjectPredicateDef::Special(_) => None,
        }
    }

    fn static_supertype_matches(
        &self,
        supertype: CardSupertype,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Option<bool> {
        prospective
            .map_or_else(
                || self.permanent_supertypes(affected),
                |prospective| {
                    self.permanent_supertypes_with_prospective(affected, prospective)
                },
            )
            .map(|supertypes| supertypes.contains(supertype))
    }

    fn static_composite_predicate_match_lazily(
        &self,
        predicate: ObjectPredicateDef,
        source: &Permanent,
        affected: &Permanent,
        prospective: Option<&Permanent>,
    ) -> Option<bool> {
        match predicate {
            ObjectPredicateDef::All(predicates) => self
                .static_composite_predicate_matches_lazily(
                    predicates,
                    source,
                    affected,
                    prospective,
                    false,
                ),
            ObjectPredicateDef::AnyOf(predicates) => self
                .static_composite_predicate_matches_lazily(
                    predicates,
                    source,
                    affected,
                    prospective,
                    true,
                ),
            _ => unreachable!("caller only forwards composite predicates"),
        }
    }

    fn static_composite_predicate_matches_lazily(
        &self,
        predicates: &[ObjectPredicateDef],
        source: &Permanent,
        affected: &Permanent,
        prospective: Option<&Permanent>,
        decisive_match: bool,
    ) -> Option<bool> {
        let mut needs_snapshot = false;
        for predicate in predicates {
            match self.lazy_match(
                *predicate,
                source,
                affected,
                prospective,
            ) {
                Some(matches) if matches == decisive_match => return Some(decisive_match),
                Some(_) => {}
                None => needs_snapshot = true,
            }
        }
        (!needs_snapshot).then_some(!decisive_match)
    }
}
