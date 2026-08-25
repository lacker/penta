// The small enums an effect names but that are not effects themselves.
//
// Each is one word of vocabulary several clauses reach for: what a follow-up
// reads off a sacrifice, which abilities a removal names, which turns a
// clause means, and who may watch a choice. Included textually into
// `effects.rs`, so the imports here are the parent module's.

/// Which characteristic of a sacrificed permanent a follow-up reads.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SacrificedAmountDef {
    Power,
    Toughness,
}

/// A reusable selector for structural abilities.
///
/// Object predicates use this to ask what a card has, while continuous effects
/// use the same vocabulary to remove matching abilities. `Any` supports
/// ordinary "loses all abilities" effects. The keyword form is also the seam
/// needed by text-changing cards that replace one landwalk ability with
/// another without treating the whole rules box as opaque text.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityPredicateDef {
    Any,
    Keyword(KeywordAbility),
    /// A flashback alternative-cast ability, whatever cost it names.
    Flashback,
    /// Every "bands with other" ability, whatever quality it names. Two cards
    /// strip them all at once, and neither says which qualities it means.
    AnyBandsWithOther,
}

impl AbilityPredicateDef {
    /// Whether one structural ability satisfies this selector.
    #[must_use]
    pub(crate) fn matches(self, ability: &AbilityDef) -> bool {
        match self {
            Self::Any => true,
            Self::Keyword(expected) => matches!(
                ability.definition,
                DeclarativeAbilityDef::Keyword(actual) if actual == expected
            ),
            Self::Flashback => matches!(
                ability.definition,
                DeclarativeAbilityDef::AlternativeCast(alternative)
                    if alternative.kind == AlternativeCastKindDef::Flashback
            ),
            Self::AnyBandsWithOther => matches!(
                ability.definition,
                DeclarativeAbilityDef::Keyword(KeywordAbility::BandsWithOther(_))
            ),
        }
    }
}
/// An event that a replacement ability can modify before it is committed.
///
/// Replacement events deliberately have their own vocabulary rather than
/// reusing [`TriggerEventDef`]: triggers observe events that have already
/// happened, while replacement abilities inspect and modify prospective
/// events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TurnKindDef {
    /// Match a regular or extra turn.
    Any,
    /// Match only the next turn in the ordinary turn order.
    Regular,
    /// Match only a turn created by a spell or ability.
    Extra,
}

impl TurnKindDef {
    #[must_use]
    pub const fn matches(self, turn: Self) -> bool {
        matches!(
            (self, turn),
            (Self::Any, _) | (Self::Regular, Self::Regular) | (Self::Extra, Self::Extra)
        )
    }
}

/// Who may observe a pending choice and its available options.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChoiceVisibilityDef {
    Public,
    Private,
}

/// One static modification to what something costs.
///
/// Five spellings of a single idea, kept together because every consumer
/// takes them together: the mana planner prices a spell against all of them
/// at once, and every clause that is not about cost passes over the whole
/// family in one arm.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostModificationDef {
    /// A permanent making the activated abilities of matching permanents
    /// cost more. Like the spell increase beside it the amount is a whole
    /// mana cost, and like it the effect is read off the battlefield rather
    /// than baked into the affected permanent.
    AbilityIncrease {
        permanent: ObjectPredicateDef,
        amount: ManaCost,
    },
    /// A permanent making activated abilities of matching sources cost more,
    /// including sources outside the battlefield. Anointed Peacekeeper says
    /// "sources" rather than "permanents", so cycling and graveyard
    /// activations with the chosen name are taxed too.
    SourceAbilityIncrease {
        source: ObjectPredicateDef,
        amount: ManaCost,
    },
    /// A permanent making matching permanents' activated abilities cost less
    /// to activate. The mirror of [`Self::AbilityIncrease`], and like a
    /// spell discount it touches generic mana only.
    ///
    /// The printed floor is part of the effect rather than a rule of its
    /// own: "this effect can't reduce the mana in that cost to less than one
    /// mana" is what keeps a free ability from becoming free twice over.
    AbilityReduction {
        permanent: ObjectPredicateDef,
        amount: ValueDef,
        /// The least mana the cost may be left with. An ability whose cost
        /// already holds no more than this is untouched.
        minimum: u16,
    },
    /// A permanent making matching spells cost more. The mirror of
    /// [`Self::SpellReduction`], but the amount is a whole mana cost rather
    /// than a number: an increase can name a colour, which a discount never
    /// does (CR 601.2f lets a reduction touch generic mana only).
    SpellIncrease {
        spell: ObjectPredicateDef,
        caster: PlayerRelation,
        amount: ManaCost,
    },
    /// A permanent offering a different mana cost for matching spells. This
    /// replaces only the spell's mana cost: additional costs and the ordinary
    /// increase/reduction pass still apply afterwards (CR 118.9d, 601.2f).
    /// `zones` describes where the card being cast must currently be; the
    /// alternative does not itself grant permission to cast from those zones.
    SpellAlternative {
        spell: ObjectPredicateDef,
        caster: PlayerRelation,
        zones: &'static [ZoneKind],
        cost: ManaCost,
    },
    /// Matching spells cost that much less generic mana to cast, read off a
    /// permanent rather than off the card being cast.
    SpellReduction {
        spell: ObjectPredicateDef,
        caster: PlayerRelation,
        amount: ValueDef,
    },
}

/// What a clause does to an attachment: which object moves onto which host,
/// or comes off one.
///
/// The five spellings live together because every consumer takes them
/// together. The attachment rules walk all of them to decide what is legally
/// attached to what, and each exhaustive match that is not about attachment
/// listed all five only to pass them over.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AttachmentDef {
    /// An Aura spell attaching itself to what it enchants. The permanent the
    /// spell becomes is what attaches, so this is only meaningful on the spell
    /// clause of an Aura.
    Attach { object: EffectRecipientDef },
    /// The mirror of [`Self::Attach`]: the named permanent moves onto this
    /// ability's own source, which is what "attach it to this creature" says.
    AttachToSource { object: EffectRecipientDef },
    /// Soulbond's pairing. The chosen creature and the ability's source
    /// record each other; the pair is symmetric and survives until one of
    /// them stops being a creature its controller controls.
    PairWithSource { object: EffectRecipientDef },
    /// Reconfigure's paired attach/unattach procedure. A selected creature
    /// becomes the new host; selecting none ends this attachment incarnation.
    Reconfigure { object: EffectRecipientDef },
    /// Detach the named Equipment or Fortification without moving it. This is
    /// a rules action rather than a zone change: Elbrus does it immediately
    /// before transforming, while the host and both objects remain otherwise
    /// unchanged.
    Unattach { object: EffectRecipientDef },
}
