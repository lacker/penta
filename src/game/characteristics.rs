use super::{
    BattlefieldExitSnapshot, CardBehavior, CardDefinitionId, CardPartId, CardRules, CardSupertype,
    CardType, CardTypeSet, CopiableCharacteristics, CounterKind, DeclarativeAbilityDef, Game,
    Permanent, PermanentLastKnownInformation, TriggerEventObject,
};

impl Game {
    pub(super) fn count_behavior(&self, behavior: CardBehavior) -> u16 {
        u16::try_from(
            self.battlefield
                .iter()
                .filter(|permanent| self.effective_behavior(permanent) == Some(behavior))
                .count(),
        )
        .unwrap_or(u16::MAX)
    }

    pub(super) fn is_artifact_permanent(&self, permanent: &Permanent) -> bool {
        self.permanent_types(permanent)
            .is_some_and(|types| types.contains(CardType::Artifact))
    }

    pub(super) fn initialize_battlefield_entry(&self, permanent: &mut Permanent) {
        let starting_loyalty = self.effective_rules(permanent).and_then(|rules| {
            rules
                .has_type(CardType::Planeswalker)
                .then(|| rules.starting_loyalty())
                .flatten()
        });
        if let Some(loyalty) = starting_loyalty {
            permanent.set_counters(CounterKind::Loyalty, loyalty);
        }
    }

    pub(super) fn effective_permanent_name<'a>(&'a self, permanent: &Permanent) -> Option<&'a str> {
        let (definition, part) = Self::effective_rules_source(permanent);
        self.catalog
            .get(definition)?
            .part(part)
            .map(|part| part.name.as_str())
    }

    /// Resolves the printed rules currently supplying baseline permanent
    /// characteristics. A copy's copiable rules take precedence over the
    /// physical card's presented part.
    pub(super) fn effective_rules<'a>(&'a self, permanent: &Permanent) -> Option<&'a CardRules> {
        let (definition, part) = Self::effective_rules_source(permanent);
        self.catalog
            .get(definition)?
            .part(part)
            .map(|part| &part.rules)
    }

    pub(super) fn effective_rules_source(permanent: &Permanent) -> (CardDefinitionId, CardPartId) {
        permanent
            .copy_effect
            .as_ref()
            .map_or((permanent.card.definition, permanent.presented), |copy| {
                copy.base
            })
    }

    pub(super) fn copiable_characteristics(permanent: &Permanent) -> CopiableCharacteristics {
        permanent
            .copy_effect
            .clone()
            .unwrap_or_else(|| CopiableCharacteristics {
                base: (permanent.card.definition, permanent.presented),
                added_types: CardTypeSet::empty(),
                added_abilities: Vec::new(),
            })
    }

    pub(super) fn trigger_event_object(&self, permanent: &Permanent) -> TriggerEventObject {
        let rules = self
            .effective_rules(permanent)
            .expect("a battlefield object has effective rules");
        TriggerEventObject {
            id: permanent.card.id,
            token: self.is_token(permanent.card.definition),
            types: self
                .permanent_types(permanent)
                .expect("a battlefield object has effective types"),
            controller: permanent.controller,
            attacking_or_blocking: permanent.attacking || permanent.blocking.is_some(),
            colors: Self::effective_colors(permanent, rules),
            subtypes: self.effective_subtypes(permanent),
            mana_value: self.permanent_mana_value(permanent),
            power: self.power_ignoring_static_effects(permanent),
            toughness: self.toughness_ignoring_static_effects(permanent),
            keywords: self.keyword_mask_ignoring_static_effects(permanent, None),
            supertypes: {
                let mut supertypes = [false; CardSupertype::COUNT];
                for supertype in CardSupertype::ALL {
                    supertypes[supertype.index()] = rules.has_supertype(supertype);
                }
                supertypes
            },
            attacking: permanent.attacking,
            attacked_this_turn: permanent.attacked_this_turn,
        }
    }

    pub(super) fn trigger_event_object_with_prospective(
        &self,
        permanent: &Permanent,
        prospective: &Permanent,
    ) -> TriggerEventObject {
        let rules = self
            .effective_rules(permanent)
            .expect("a battlefield object has effective rules");
        TriggerEventObject {
            id: permanent.card.id,
            token: self.is_token(permanent.card.definition),
            types: self
                .permanent_types(permanent)
                .expect("a battlefield object has effective types"),
            controller: permanent.controller,
            attacking_or_blocking: permanent.attacking || permanent.blocking.is_some(),
            colors: Self::effective_colors(permanent, rules),
            subtypes: self.effective_subtypes_with_prospective(permanent, prospective),
            mana_value: self.permanent_mana_value(permanent),
            power: self.power_ignoring_static_effects(permanent),
            toughness: self.toughness_ignoring_static_effects(permanent),
            keywords: self.keyword_mask_ignoring_static_effects(permanent, Some(prospective)),
            supertypes: {
                let mut supertypes = [false; CardSupertype::COUNT];
                for supertype in CardSupertype::ALL {
                    supertypes[supertype.index()] = rules.has_supertype(supertype);
                }
                supertypes
            },
            attacking: permanent.attacking,
            attacked_this_turn: permanent.attacked_this_turn,
        }
    }

    pub(super) fn battlefield_exit_snapshot(
        &self,
        permanent: &Permanent,
    ) -> BattlefieldExitSnapshot {
        let abilities = self.effective_abilities(permanent);
        let mut keywords = Vec::new();
        for effective in &abilities {
            if effective.ability.is_executable()
                && let DeclarativeAbilityDef::Keyword(ability) = effective.ability.definition
                && !keywords.contains(&ability)
            {
                keywords.push(ability);
            }
        }
        BattlefieldExitSnapshot {
            object: self.trigger_event_object(permanent),
            abilities,
            last_known: PermanentLastKnownInformation {
                power: self.power(permanent),
                toughness: self.toughness(permanent),
                mana_value: self.permanent_mana_value(permanent),
                keywords,
            },
        }
    }
}
