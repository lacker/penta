// What sits on the stack, and what it carries.
//
// Split out of `mod.rs` for the source-size budget, and split here because
// these types answer one question together: a spell or an ability waiting to
// resolve, the frozen payload an ability keeps, and the resolver that will
// run it. Included textually, so the imports here are that module's.

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
struct StackObject {
    id: GameObjectId,
    kind: StackObjectKind,
    card: ObjectInstance,
    /// The permanent object whose ability this is. Spell objects have no
    /// source; their `card` is the stack incarnation itself.
    source: Option<GameObjectId>,
    /// The complete executable ability captured when this object was put on
    /// the stack. The origin remains useful provenance, but resolution never
    /// uses it to rediscover rules from a source that may since have changed.
    ability: Option<StackAbilityPayload>,
    controller: PlayerId,
    /// Present exactly for spell objects. This freezes form, modes, costs, X,
    /// and target-slot bindings for resolution and copy effects.
    signature: Option<CastSignature>,
    chosen_permanents: Vec<GameObjectId>,
    /// Effects carried by mana used to pay for this object. They are attached
    /// before the spell is finalized on the stack and retain their source.
    applied_effects: Vec<AppliedStackEffect>,
    /// Indefinite text changes applied while this object is on the stack.
    /// They transfer to a resolving permanent but are not copied by spell-copy
    /// effects.
    text_changes: Vec<BasicLandTypeChange>,
    /// Colours imposed on this object by a copy effect or a resolving
    /// characteristic effect, such as "except that the copy is red" or a
    /// Lace. The override lasts for this stack incarnation.
    colors: Option<ColorSet>,
    /// Casting choices, payment facts, and provenance carried through
    /// resolution. Ability objects and objects put directly on the stack have
    /// none; spell copies retain only the parts CR 707.10 copies.
    cast: Option<CastContext>,
    /// The copiable characteristics supplied by the rule that allowed this
    /// spell to be cast face down. The permanent it becomes keeps the same
    /// values, while only its controller may inspect the physical card.
    face_down: Option<FaceDownCharacteristics>,
    is_copy: bool,
}

/// The immutable rules payload of an activated or triggered ability on the
/// stack. `origin` describes where the ability came from; the remaining fields
/// are the authoritative frozen characteristics used for presentation,
/// target legality, and resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
struct StackAbilityPayload {
    origin: AbilityOrigin,
    /// The complete activated or spell ability as it existed when this object
    /// was put on the stack. Copy effects that retain the resolving ability
    /// need its costs and targets as copiable values, not only its resolver;
    /// triggered payloads do not currently need this optional snapshot.
    definition: Option<Box<AbilityDef>>,
    presentation: ObjectCharacteristics,
    text: Option<&'static str>,
    target_defs: Vec<AbilityTargetDef>,
    targets: Vec<TargetSelection>,
    context: EffectResolutionContext,
    resolver: StackAbilityResolver,
    /// The intervening-if condition, re-read as this ability resolves.
    condition: Option<&'static TriggerConditionDef>,
    /// Selected declarative mode effects frozen in canonical printed order.
    /// Repeated modes remain repeated procedures.
    mode_effects: Vec<ScopedEffect>,
    /// Where a successfully resolving spell card goes. This is frozen as a
    /// property of the stack object because optional additional costs can
    /// change it independently of the ability whose instructions resolve.
    /// Activated and triggered abilities carry `None`.
    resolution_destination: Option<SpellResolutionDestinationDef>,
    /// The X chosen when the ability was activated, so its effects read the
    /// same number the cost was paid for.
    x: u16,
    /// What this activation's sacrificed costs added up to in mana value,
    /// read the same way and for the same reason: the permanents are gone
    /// before the ability resolves.
    sacrificed_mana_value: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StackAbilityResolver {
    Declarative(ScopedEffect),
    Prepared {
        reference: ScopedEffect,
        effect: PreparedEffect,
    },
    /// The declarative program still runs when rule 608.2b would ordinarily
    /// stop an ability whose targets have all become illegal.
    DeclarativeIgnoringTargetFizzle(ScopedEffect),
    /// A linked triggered ability whose resolution offers its source card
    /// for one exact alternative cost.
    CastOffer(AlternativeCastKindDef),
}

impl StackAbilityResolver {
    fn linked_cast_offer(ability: &AbilityDef) -> Option<Self> {
        match ability.definition {
            DeclarativeAbilityDef::AlternativeCast(alternative)
                if alternative.kind == AlternativeCastKindDef::Miracle =>
            {
                Some(Self::CastOffer(alternative.kind))
            }
            _ => None,
        }
    }
}

/// One authored effect together with the start of its clause-local target
/// range in an instantiated stack object's flattened target list.
///
/// An authored [`TargetIndex`] is deliberately local to its ability clause.
/// Modal branches can each name target zero; freezing a spell assigns every
/// selected branch a distinct base and resolution translates through it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ScopedEffect {
    effect: EffectDef,
    target_base: usize,
}

impl ScopedEffect {
    const fn primary(effect: EffectDef) -> Self {
        Self {
            effect,
            target_base: 0,
        }
    }

    const fn with_effect(self, effect: EffectDef) -> Self {
        Self {
            effect,
            target_base: self.target_base,
        }
    }

    fn target_slot(self, target: TargetIndex) -> TargetSlotId {
        TargetSlotId::from_index(self.target_base + target.index())
            .expect("validated target composition fits the runtime slot space")
    }
}

impl StackObject {
    fn target_selections(&self) -> &[TargetSelection] {
        self.signature.as_ref().map_or_else(
            || {
                self.ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
                    .unwrap_or_default()
            },
            CastSignature::targets,
        )
    }

    fn replace_target_selections(
        &mut self,
        targets: &[TargetSelection],
    ) -> Result<(), crate::casting::TargetReplacementError> {
        let signature = self
            .signature
            .as_ref()
            .map(|signature| signature.with_replaced_targets(targets.to_vec()))
            .transpose()?;
        let ability_targets = self
            .ability
            .as_ref()
            .map(|ability| {
                CastSignature::target_shape_replacement(&ability.targets, targets.to_vec())
            })
            .transpose()?;
        self.signature = signature;
        if let (Some(ability), Some(targets)) = (&mut self.ability, ability_targets) {
            ability.targets = targets;
        }
        Ok(())
    }

    fn iter_targets(&self) -> impl Iterator<Item = &Target> {
        self.signature
            .iter()
            .flat_map(CastSignature::iter_targets)
            .chain(
                self.ability
                    .iter()
                    .filter(|_| self.signature.is_none())
                    .flat_map(|ability| ability.targets.iter())
                    .flat_map(TargetSelection::targets),
            )
    }

    fn ability_origin(&self) -> Option<AbilityOrigin> {
        self.ability.as_ref().map(|ability| ability.origin)
    }

    fn ability_text(&self) -> Option<&'static str> {
        self.ability.as_ref().and_then(|ability| ability.text)
    }

    fn presentation(&self) -> ObjectCharacteristics {
        self.ability.as_ref().map_or_else(
            || {
                ObjectCharacteristics::card(
                    self.card
                        .definition
                        .card_definition()
                        .expect("a spell object is backed by a card definition"),
                    self.signature
                        .as_ref()
                        .and_then(|signature| match signature.form() {
                            crate::card::SpellForm::Part(part) => Some(*part),
                            crate::card::SpellForm::Combined(parts) => parts.first().copied(),
                        })
                        .unwrap_or(CardPartId::PRIMARY),
                )
            },
            |ability| ability.presentation,
        )
    }

    fn targets(&self) -> Vec<Target> {
        self.iter_targets().copied().collect()
    }

    /// Targets announced for this spell or ability. Installed abilities may
    /// retain an earlier ability's selections as lexical references for
    /// resolution, but those are not targets of the triggered ability and
    /// must not be presented publicly as though they were chosen again.
    fn declared_targets(&self) -> Vec<Target> {
        if let Some(signature) = &self.signature {
            return signature.iter_targets().copied().collect();
        }
        self.ability
            .iter()
            .flat_map(|ability| {
                ability
                    .targets
                    .iter()
                    .take(ability.target_defs.len())
                    .flat_map(TargetSelection::targets)
            })
            .copied()
            .collect()
    }

    fn first_target(&self) -> Option<Target> {
        self.iter_targets().next().copied()
    }

    fn target_count(&self) -> usize {
        self.iter_targets().count()
    }

    fn x(&self) -> u16 {
        self.signature.as_ref().map_or_else(
            || self.ability.as_ref().map_or(0, |ability| ability.x),
            CastSignature::x,
        )
    }

    /// How many colours paid for this object, which is zero for everything
    /// that was never cast.
    fn colors_spent_count(&self) -> u8 {
        self.cast
            .as_ref()
            .map_or(0, CastContext::colors_spent_count)
    }
}
