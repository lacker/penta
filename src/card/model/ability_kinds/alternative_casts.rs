//! The alternative ways a card can be cast.
//!
//! Flashback, overload, kicker, evoke, miracle, and casting a card face down
//! all reach the stack by a route other than paying what the card prints, so
//! each is one clause of this shape rather than a special case in the
//! casting path.

use crate::ids::{AbilityId, AlternativeCostId};

use super::super::{AlternativeCostDef, ManaCost};
use super::{AbilityTargetDef, SpellAdditionalCostDef, TriggerConditionDef};

/// The rules procedure and mana cost supplied by a printed
/// alternative-casting keyword.
///
/// A play option exposes a derived [`AlternativeCostDef`] whose identity is
/// the positional [`AbilityId`] of this clause. An overload clause uses its
/// [`AbilityDef::effect`] as the targetless text-replacement result; flashback
/// uses `EffectDef::None` and changes where the card may be cast and where it
/// goes after the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlternativeCastAbilityDef {
    pub mana_cost: AlternativeCastManaCostDef,
    pub kind: AlternativeCastKindDef,
    /// Rules text for the spell as modified by this alternative, when the
    /// procedure changes its visible instructions (as overload does).
    pub stack_text: Option<&'static str>,
    /// A nonmana cost paid in place of the mana one. The objects it names are
    /// spent the way the zone says: a permanent is sacrificed, a card in a
    /// graveyard is exiled, a card in hand is discarded.
    pub additional_cost: Option<SpellAdditionalCostDef>,
    /// A board condition the alternative requires. Mogg Salvage's free cast
    /// is only available while the two lands it names are out, so a false
    /// condition means the alternative is not offered at all.
    pub condition: Option<&'static TriggerConditionDef>,
    /// What the modified spell targets. Overload replaces "target" with
    /// "each" and so declares none, but a kicked spell targets exactly what
    /// the unkicked one does -- and the clause carries its own instructions,
    /// so it has to declare the slots those instructions read.
    pub targets: &'static [AbilityTargetDef],
    /// Life paid as part of this alternative, on top of whatever mana it
    /// names. "You may pay 4 life rather than pay this spell's mana cost" is
    /// a mana cost of nothing and a life cost of four.
    pub life: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternativeCastKindDef {
    Flashback,
    Overload,
    /// Cast from hand only in the window opened by drawing the card, as the
    /// first card drawn that turn.
    Miracle,
    /// A plain "you may <do something> rather than pay this spell's mana
    /// cost". Like flashback it only changes what the spell costs, never what
    /// it does, so the spell's own clause still supplies the instructions --
    /// what it carries instead is a nonmana cost in `additional_cost`.
    AlternativeCost,
    /// Cast with its buyback paid (CR 702.27). Like a kicker it only costs
    /// more, so the spell resolves exactly as printed; what it changes is
    /// where the card goes afterwards -- to its owner's hand rather than the
    /// graveyard, which is the whole of the mechanic.
    Buyback,
    /// Cast from hand with its kicker paid. A kicker is printed as an
    /// optional additional cost, but the kicked spell is exactly a spell cast
    /// for the printed cost plus the kicker with a different set of
    /// instructions -- which is what an alternative cast already is, so the
    /// mana cost here is the whole kicked total rather than the surcharge.
    Kicked,
    /// Cast from a graveyard for its escape cost (CR 702.152a). Like
    /// flashback it is a permission to cast the card where it lies, and the
    /// cards it exiles are an additional cost -- but unlike flashback the
    /// card is not exiled afterwards, so a creature that escaped and later
    /// dies may escape again.
    Escape,
    /// Cast for its impending cost (CR 702.175a). Like a kicker it is an
    /// ordinary cast from hand for a different price; what it changes is how
    /// the permanent arrives, which the card's own clauses say.
    Impending,
    /// Cast where it lies without paying its mana cost, and exiled rather
    /// than buried afterwards. Dreadhorde Arcanist's clause is not flashback
    /// -- it grants no keyword and lasts only for the resolution that
    /// offered it -- but what it does to the cast and to the card afterwards
    /// is the same pair of things.
    WithoutPayingManaCost,
    /// Cast from exile for its foretell cost (CR 702.143a). The card got
    /// there by the foretell special action, which exiles it face down for
    /// {2} during its owner's turn; this is the other half, and it may not
    /// be taken until a later turn.
    Foretell,
    /// Cast using the supplied face-down copiable values. The spell's own
    /// clauses are not what it does while face down, so this kind changes the
    /// object rather than only its cost. Morph and Disguise choose different
    /// values without allocating either presentation a card definition.
    FaceDown {
        label: &'static str,
        characteristics: super::super::FaceDownCharacteristics,
    },
}

/// How an alternative-casting ability determines the cost it supplies.
///
/// Printed abilities normally carry a fixed cost. A granted ability such as
/// Snapcaster Mage's flashback instead reads the mana cost of the card that
/// gained it, after a concrete play option has selected the spell form.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternativeCastManaCostDef {
    Fixed(ManaCost),
    ThisCardManaCost,
}

impl AlternativeCastManaCostDef {
    #[must_use]
    pub const fn resolve(self, card_mana_cost: Option<ManaCost>) -> Option<ManaCost> {
        match self {
            Self::Fixed(mana_cost) => Some(mana_cost),
            Self::ThisCardManaCost => card_mana_cost,
        }
    }
}

impl AlternativeCastKindDef {
    #[must_use]
    pub const fn face_down(self) -> Option<super::super::FaceDownCharacteristics> {
        match self {
            Self::FaceDown {
                characteristics, ..
            } => Some(characteristics),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Flashback => "Flashback",
            Self::Overload => "Overload",
            Self::Miracle => "Miracle",
            Self::Kicked => "Kicker",
            Self::Buyback => "Buyback",
            Self::AlternativeCost => "Alternative cost",
            Self::Escape => "Escape",
            Self::Impending => "Impending",
            Self::Foretell => "Foretell",
            Self::WithoutPayingManaCost => "Without paying its mana cost",
            Self::FaceDown { label, .. } => label,
        }
    }

    /// The inverse of [`Self::label`], for a snapshot that names the kind
    /// rather than storing an enum whose order could move.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        [
            Self::Escape,
            Self::Foretell,
            Self::Impending,
            Self::WithoutPayingManaCost,
            Self::Flashback,
            Self::Overload,
            Self::Miracle,
            Self::Kicked,
            Self::Buyback,
            Self::AlternativeCost,
            crate::card::face_down::morph_cast(),
            crate::card::face_down::disguise_cast(),
        ]
        .into_iter()
        .find(|kind| kind.label() == label)
    }
}

impl AlternativeCastAbilityDef {
    #[must_use]
    pub fn rules_text(self) -> String {
        match (self.kind, self.mana_cost) {
            (AlternativeCastKindDef::Flashback, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Flashback {mana_cost} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
                )
            }
            (
                AlternativeCastKindDef::Flashback,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Flashback—the flashback cost is equal to this card's mana cost. (You may cast this card from your graveyard for its flashback cost. Then exile it.)".into(),
            (AlternativeCastKindDef::Impending, _) => self.stack_text.map_or_else(
                || {
                    "Impending (If you cast this spell for its impending cost, it enters with time counters and isn't a creature until the last is removed.)".into()
                },
                std::borrow::ToOwned::to_owned,
            ),
            (AlternativeCastKindDef::Escape, _) => self.stack_text.map_or_else(
                || "Escape (You may cast this card from your graveyard for its escape cost.)".into(),
                std::borrow::ToOwned::to_owned,
            ),
            (AlternativeCastKindDef::Overload, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Overload {mana_cost} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
                )
            }
            (
                AlternativeCastKindDef::Overload,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Overload—the overload cost is equal to this card's mana cost. (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")".into(),
            (AlternativeCastKindDef::Miracle, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Miracle {mana_cost} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)",
                )
            }
            (
                AlternativeCastKindDef::Miracle,
                AlternativeCastManaCostDef::ThisCardManaCost,
            ) => "Miracle—the miracle cost is equal to this card's mana cost. (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)".into(),
            // The printed reminder names the surcharge, but the cost carried
            // here is the kicked total, so the card supplies its own text.
            (AlternativeCastKindDef::Kicked, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!("Kicked, for {mana_cost} in total")
            }
            (AlternativeCastKindDef::Kicked, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Kicked".into()
            }
            // As with kicker, the printed reminder names the surcharge while
            // the cost carried here is the bought-back total.
            (AlternativeCastKindDef::Buyback, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!("Bought back, for {mana_cost} in total")
            }
            (AlternativeCastKindDef::Buyback, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Bought back".into()
            }
            // The card prints what is paid instead, so it supplies the text.
            (AlternativeCastKindDef::AlternativeCost, _) => "Alternative cost".into(),
            (AlternativeCastKindDef::Foretell, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Foretell {mana_cost} (During your turn, you may pay {{2}} and exile this card from your hand face down. Cast it on a later turn for its foretell cost.)",
                )
            }
            // No card prints a foretell cost equal to its own mana cost.
            (AlternativeCastKindDef::Foretell, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Foretell".into()
            }
            // Never printed on the card being cast: whatever granted the
            // permission said this, so its own text is the reminder.
            (AlternativeCastKindDef::WithoutPayingManaCost, _) => self
                .stack_text
                .map_or_else(|| "Without paying its mana cost".into(), std::borrow::ToOwned::to_owned),
            // Morph is printed on the card that has it; casting face down is
            // the rule that applies to every such card, and the cost of doing
            // it is always {3}.
            (
                AlternativeCastKindDef::FaceDown {
                    characteristics, ..
                },
                _,
            ) => format!(
                "You may cast this card face down as a {} for {{3}}.",
                if characteristics == crate::card::face_down::disguise() {
                    "2/2 creature spell with ward {2}"
                } else {
                    "2/2 creature spell"
                }
            ),
        }
    }

    #[must_use]
    pub fn alternative_cost(
        self,
        ability: AbilityId,
        card_mana_cost: Option<ManaCost>,
    ) -> Option<AlternativeCostDef> {
        Some(AlternativeCostDef {
            id: AlternativeCostId(ability.0),
            label: self.kind.label().into(),
            mana_cost: self.mana_cost.resolve(card_mana_cost)?,
        })
    }
}
