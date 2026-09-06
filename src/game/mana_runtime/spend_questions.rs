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
                ..
            } => self
                .printed_trigger_event_object(
                    *object,
                    *definition,
                    *controller,
                    &CharacteristicContext::Stack { form: form.clone() },
                )
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
            ManaPaymentPurpose::CumulativeUpkeep { source, .. } => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *source)
                .map(|permanent| (self.trigger_event_object(permanent), false)),
            ManaPaymentPurpose::Other => None,
        }
    }

    pub(super) fn chosen_creature_type_for_mana_source(
        &self,
        source: GameObjectId,
    ) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_creature_type.as_deref())
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.chosen_creature_type.as_deref()
                }
                Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
            })
    }

    pub(super) fn mana_can_pay_for(&self, mana: Mana, purpose: &ManaPaymentPurpose) -> bool {
        mana.restrictions
            .iter()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, true)
                    }),
                // Nothing to check when the payment is not a cast: what the
                // clause forbids is one kind of spell, not one kind of use.
                ManaRestrictionDef::CannotCastSpell(predicate) => !self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, true)
                    }),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => {
                    let Some(source) = mana.source else {
                        return false;
                    };
                    let Some(chosen) = self.chosen_creature_type_for_mana_source(source.object)
                    else {
                        return false;
                    };
                    self.payment_object(purpose)
                        .is_some_and(|(object, is_spell)| {
                            is_spell
                                && object.types.contains(CardType::Creature)
                                && object.subtypes.contains(&chosen)
                        })
                }
                ManaRestrictionDef::ActivateAbility(predicate) => self
                    .payment_object(purpose)
                    .is_some_and(|(object, is_spell)| {
                        !is_spell
                            && self.trigger_object_matches(*predicate, &object, object.id, false)
                    }),
                ManaRestrictionDef::CumulativeUpkeep => {
                    matches!(purpose, ManaPaymentPurpose::CumulativeUpkeep { .. })
                }
                ManaRestrictionDef::Special(_) => false,
            })
            && match purpose {
                ManaPaymentPurpose::CumulativeUpkeep { snow: true, .. } => mana
                    .source
                    .and_then(|source| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == source.object)
                            .or_else(|| match self.retired_objects.get(&source.object) {
                                Some(RetiredObject::Permanent { permanent, .. }) => {
                                    Some(permanent)
                                }
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
