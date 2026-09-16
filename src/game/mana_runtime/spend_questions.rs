// What a payment is for, and whether one mana may be spent on it.
//
// Split out of `mana_runtime.rs` for the source-size budget along a seam the
// file already had: next door is about making mana and spending it, and this
// is about the restrictions and riders that ask what it is being spent on.
// Included textually, so the imports here are that module's.

impl Game {
    pub(super) fn payment_object(
        &self,
        purpose: &ManaPaymentPurpose,
    ) -> Option<(TriggerEventObject, bool)> {
        match purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                controller,
                form,
                alternative,
                x,
                ..
            } => self
                .spell_view_characteristics(super::SpellView {
                    object: *object,
                    definition: *definition,
                    controller: *controller,
                    owner: self
                        .card_in_nonbattlefield_zone(*object)
                        .map_or(*controller, |(_, card)| card.owner),
                    form,
                    source_zone: None,
                    x: *x,
                    face_down: alternative.and_then(crate::card::AlternativeCastKindDef::face_down),
                    bestow: *alternative == Some(crate::card::AlternativeCastKindDef::Bestow),
                })
                .map(|object| (object, true)),
            ManaPaymentPurpose::Ability { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false))
                .or_else(|| match self.retired_objects.get(source) {
                    Some(RetiredObject::Permanent { permanent, .. }) => {
                        Some((self.trigger_event_object(permanent), false))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                })
                .or_else(|| {
                    let (zone, card) = self.card_in_nonbattlefield_zone(*source)?;
                    let context = match zone {
                        ZoneKind::Library => CharacteristicContext::Library,
                        ZoneKind::Hand => CharacteristicContext::Hand,
                        ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                        ZoneKind::Exile => CharacteristicContext::Exile,
                        ZoneKind::Command => CharacteristicContext::Command,
                        ZoneKind::Battlefield | ZoneKind::Stack => return None,
                    };
                    self.printed_trigger_event_object(
                        card.id,
                        card.definition,
                        card.owner,
                        &context,
                    )
                    .map(|object| (object, false))
                }),
            ManaPaymentPurpose::Payment { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false)),
            ManaPaymentPurpose::Other => None,
        }
    }

    #[cfg(test)]
    pub(super) fn mana_can_pay_for(&self, mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        self.mana_can_pay_for_cost(mana, purpose, ManaCost::default())
    }

    fn spell_mana_restrictions(&self, purpose: &ManaPaymentPurpose) -> Vec<ManaRestrictionDef> {
        use crate::card::{AppliedEffectDef, AppliedRuleDef, EffectRecipientDef};
        fn collect_applied(effect: AppliedEffectDef, result: &mut Vec<ManaRestrictionDef>) {
            match effect {
                AppliedEffectDef::Composite(effects) => {
                    for effect in effects {
                        collect_applied(*effect, result);
                    }
                }
                AppliedEffectDef::Rule(AppliedRuleDef::ManaPaymentRestriction(restriction)) => {
                    result.push(restriction);
                }
                _ => {}
            }
        }
        fn collect(effect: EffectDef, result: &mut Vec<ManaRestrictionDef>) {
            match effect {
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect,
                } => {
                    collect_applied(effect, result);
                }
                EffectDef::Sequence(effects) => {
                    for effect in effects {
                        collect(*effect, result);
                    }
                }
                _ => {}
            }
        }
        let ManaPaymentPurpose::Spell {
            definition,
            form,
            alternative,
            ..
        } = purpose
        else {
            return Vec::new();
        };
        if alternative
            .and_then(crate::card::AlternativeCastKindDef::face_down)
            .is_some()
        {
            return Vec::new();
        }
        let Some(definition) = self.catalog.get(*definition) else {
            return Vec::new();
        };
        let parts: &[crate::CardPartId] = match form {
            crate::card::SpellForm::Part(part) => core::slice::from_ref(part),
            crate::card::SpellForm::Combined(parts) => parts,
        };
        let mut restrictions = Vec::new();
        for part in parts.iter().filter_map(|part| definition.part(*part)) {
            for attached in part.rules.indexed_abilities() {
                let ability = attached.definition;
                if matches!(ability.definition, DeclarativeAbilityDef::Static(definition)
                    if definition.source_zones.contains(&ZoneKind::Stack))
                    && let Some(effect) = ability.declarative_effect()
                {
                    collect(effect, &mut restrictions);
                }
            }
        }
        restrictions
    }

    pub(in crate::game) fn payment_allows_mana(&self, purpose: &ManaPaymentPurpose) -> bool {
        // Spell-side restrictions currently concern the payment's spell, not
        // the produced unit. Use the same predicate for floating and generated mana.
        self.spell_mana_restrictions(purpose)
            .iter()
            .all(|restriction| {
                self.mana_restriction_allows(
                    Mana::unrestricted(ManaColor::Colorless),
                    purpose,
                    ManaCost::default(),
                    *restriction,
                )
            })
    }

    pub(super) fn mana_requires_nongeneric(
        &self,
        mana: Mana,
        purpose: &ManaPaymentPurpose,
        cost: ManaCost,
    ) -> bool {
        fn permits_generic(
            game: &Game,
            mana: Mana,
            purpose: &ManaPaymentPurpose,
            cost: ManaCost,
            restriction: ManaRestrictionDef,
        ) -> bool {
            match restriction {
                ManaRestrictionDef::CannotPayGeneric => false,
                ManaRestrictionDef::AnyOf(alternatives) => alternatives
                    .iter()
                    .any(|restriction| permits_generic(game, mana, purpose, cost, *restriction)),
                other => game.mana_restriction_allows(mana, purpose, cost, other),
            }
        }
        !mana
            .restrictions
            .iter()
            .all(|restriction| permits_generic(self, mana, purpose, cost, *restriction))
    }

    pub(super) fn mana_can_pay_for_cost(
        &self,
        mana: Mana,
        purpose: &ManaPaymentPurpose,
        cost: ManaCost,
    ) -> bool {
        self.payment_allows_mana(purpose)
            && mana
                .restrictions
                .iter()
                .all(|restriction| self.mana_restriction_allows(mana, purpose, cost, *restriction))
            && match purpose {
                ManaPaymentPurpose::Payment { snow: true, .. } => mana
                    .source
                    .and_then(|source| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == source.object)
                            .or_else(|| match self.retired_objects.get(&source.object) {
                                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent),
                                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => {
                                    None
                                }
                            })
                    })
                    .and_then(|permanent| self.permanent_supertypes(permanent))
                    .is_some_and(|types| types.contains(crate::card::CardSupertype::Snow)),
                _ => true,
            }
    }

    fn mana_restriction_allows(
        &self,
        mana: Mana,
        purpose: &ManaPaymentPurpose,
        cost: ManaCost,
        restriction: ManaRestrictionDef,
    ) -> bool {
        match &restriction {
            ManaRestrictionDef::CannotPayGeneric => true,
            ManaRestrictionDef::AnyOf(alternatives) => alternatives
                .iter()
                .any(|alternative| self.mana_restriction_allows(mana, purpose, cost, *alternative)),
            ManaRestrictionDef::CastSpell(predicate) => {
                self.payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(
                                *predicate,
                                &object,
                                mana.source.map_or(object.id, |source| source.object),
                                true,
                            )
                    })
            }
            // Nothing to check when the payment is not a cast: what the
            // clause forbids is one kind of spell, not one kind of use.
            ManaRestrictionDef::CannotCastSpell(predicate) => !self
                .payment_object(purpose)
                .is_some_and(|(object, is_spell)| {
                    is_spell
                        && self.trigger_object_matches(
                            *predicate,
                            &object,
                            mana.source.map_or(object.id, |source| source.object),
                            true,
                        )
                }),
            ManaRestrictionDef::ActivateAbility(predicate) => self
                .payment_object(purpose)
                .is_some_and(|(object, is_spell)| {
                    matches!(purpose, ManaPaymentPurpose::Ability { .. })
                        && !is_spell
                        && self.trigger_object_matches(
                            *predicate,
                            &object,
                            mana.source.map_or(object.id, |source| source.object),
                            false,
                        )
                }),
            ManaRestrictionDef::Payment(expected) => {
                matches!(purpose, ManaPaymentPurpose::Payment { label: Some(actual), .. } if actual == expected)
            }
            ManaRestrictionDef::CastFrom(zone) => matches!(purpose,
                ManaPaymentPurpose::Spell { source_zone: Some(actual), .. } if actual == zone),
            ManaRestrictionDef::CastYourCommander => match purpose {
                ManaPaymentPurpose::Spell {
                    commander_owner,
                    controller,
                    ..
                } => *commander_owner == Some(*controller),
                _ => false,
            },
            ManaRestrictionDef::PayCostContaining(color) => {
                super::mana_planning::mana_cost_amount(cost, *color) > 0
            }
            ManaRestrictionDef::Special(_) => false,
        }
    }

    pub(super) fn mana_has_spend_effect_for(mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.spend_effects.iter().any(|effect| {
            matches!(
                (purpose, effect),
                (
                    ManaPaymentPurpose::Spell { .. },
                    ManaSpendEffectDef::ApplyToPaidSpell(_)
                        | ManaSpendEffectDef::ApplyToPaidSpellMatching { .. }
                ) | (
                    ManaPaymentPurpose::Ability { .. },
                    ManaSpendEffectDef::ApplyToPaidAbility(_)
                )
            )
        })
    }
}
