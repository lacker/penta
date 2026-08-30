//! The alternative ways a card can be cast.
//!
//! Flashback, overload, kicker, evoke, miracle, and casting a card face down
//! all reach the stack by a route other than paying what the card prints, so
//! each is one clause of this shape rather than a special case in the
//! casting path.

use crate::ids::{AbilityId, AlternativeCostId};

use super::super::{AlternativeCostDef, CostQuantityDef, ManaCost, ObjectPredicateDef, ZoneKind};
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
    /// Life an opponent gains as this alternative is taken. Invigorate's is
    /// the only shape of it: what the caster spends is not their own life
    /// but the other player's gain, which costs them nothing they had.
    pub opponent_life_gain: u16,
    /// Whether the card also prints a permission to use this alternative
    /// from its owner's graveyard. Detective's Phoenix's "You may cast this
    /// card from your graveyard using its bestow ability" is one clause
    /// about one alternative, so it is recorded on that alternative: the
    /// cast it permits is the same cast in every other respect, down to the
    /// target it takes and the clause it resolves.
    pub from_graveyard: bool,
    /// The smallest X this alternative may be cast for. "Kicker {X}. X can't
    /// be 0" is the whole reason it exists: casts are enumerated from zero,
    /// and a kick of nothing would be a kick that cost nothing.
    pub minimum_x: u16,
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
    /// Retrace (CR 702.81a): cast from a graveyard for its own cost plus a
    /// discarded land. Like escape the card is not exiled afterwards, which
    /// is what makes a land-heavy hand into a repeatable spell.
    Retrace,
    /// Cast for its warp cost. Like a kicker it is an ordinary cast from
    /// hand for a different price; what it changes is that the permanent is
    /// exiled at the beginning of the next end step and may be cast again
    /// from there on a later turn, which the card's own clauses say.
    Warp,
    /// Cast for its dash cost (CR 702.109a). Like a kicker it is an
    /// ordinary cast from hand for a different price; what it changes is
    /// that the creature arrives hasty and goes back to its owner's hand at
    /// the beginning of the next end step, which the card's own clauses say.
    Dash,
    /// Cast for its impending cost (CR 702.175a). Like a kicker it is an
    /// ordinary cast from hand for a different price; what it changes is how
    /// the permanent arrives, which the card's own clauses say.
    Impending,
    /// Rebound's free cast (CR 702.87a), taken from exile at the beginning
    /// of the caster's next upkeep. Unlike the free cast beside it the card
    /// is not exiled afterwards: it was cast from exile rather than from
    /// hand, so rebound has nothing more to say about it and it goes to the
    /// graveyard like any other spell.
    Rebound,
    /// Emerge (CR 702.119a): cast by sacrificing a permanent the ability
    /// names and paying the emerge cost reduced by that permanent's mana
    /// value. The reduction is generic only, so the coloured pips are still
    /// owed in their own colours -- which is what keeps a big artifact from
    /// paying for the whole spell. Written as an alternative because that is
    /// what it is; what it adds is that the sacrifice named in
    /// `additional_cost` also settles what the cast costs.
    Emerge,
    /// Bestow (CR 702.103a): cast as an Aura spell for its bestow cost.
    /// Unlike every other kind here it changes what the spell is rather than
    /// only what it costs -- an Aura spell with "enchant creature", which is
    /// why the clause carries its own target and its own attaching effect.
    /// While the permanent it becomes stays attached it is an Aura and not a
    /// creature; when the enchanted creature leaves it becomes a creature
    /// instead of dying, which is what the permanent's recorded kind is read
    /// for.
    Bestow,
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
    /// Offspring (CR 702.126a): an additional cost paid as the creature is
    /// cast, which nothing about the spell reads except its own arrival --
    /// "if you do, when this creature enters" is a trigger asking whether it
    /// was paid. Written as an alternative for the same reason kicker is:
    /// what varies is the cost, and the permanent records which it paid.
    Offspring,
    /// Plot (CR 702.170a). Unlike every other kind here, this is not a way to
    /// cast the card at all: the cost is paid to the plot special action,
    /// which exiles the card, and the cast that follows on a later turn is
    /// free. The clause is written as an alternative so that the cost has
    /// somewhere printed to live, and nothing offers it as a cast.
    Plot,
    /// Splice onto Arcane (CR 702.47a). Like plot this is not a way to cast
    /// the card at all: the card stays in hand and what is cast is somebody
    /// else's Arcane spell, which gains this card's instructions. The clause
    /// is written as an alternative so the splice cost has somewhere printed
    /// to live, and nothing offers it as a cast.
    Splice,
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
            Self::AlternativeCost => "Alternative cost",
            Self::Escape => "Escape",
            Self::Impending => "Impending",
            Self::Dash => "Dash",
            Self::Warp => "Warp",
            Self::Retrace => "Retrace",
            Self::Foretell => "Foretell",
            Self::Offspring => "Offspring",
            Self::Plot => "Plot",
            Self::Bestow => "Bestow",
            Self::Splice => "Splice",
            Self::Emerge => "Emerge",
            Self::Rebound => "Rebound",
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
            Self::Offspring,
            Self::Plot,
            Self::Bestow,
            Self::Rebound,
            Self::Impending,
            Self::Dash,
            Self::Warp,
            Self::Retrace,
            Self::Emerge,
            Self::Splice,
            Self::WithoutPayingManaCost,
            Self::Flashback,
            Self::Overload,
            Self::Miracle,
            Self::Kicked,
            Self::AlternativeCost,
            crate::card::face_down::morph_cast(),
            crate::card::face_down::disguise_cast(),
        ]
        .into_iter()
        .find(|kind| kind.label() == label)
    }
}

impl AlternativeCastAbilityDef {
    fn count_word(count: u8) -> String {
        match count {
            0 => "zero".into(),
            1 => "one".into(),
            2 => "two".into(),
            3 => "three".into(),
            4 => "four".into(),
            5 => "five".into(),
            6 => "six".into(),
            7 => "seven".into(),
            8 => "eight".into(),
            9 => "nine".into(),
            10 => "ten".into(),
            _ => count.to_string(),
        }
    }

    /// The common printed Escape shape. Exceptional Escape costs retain
    /// their authored text, while the ordinary mana-plus-cards form can be
    /// rendered entirely from its semantic cost.
    fn common_escape_rules_text(self) -> Option<String> {
        let AlternativeCastManaCostDef::Fixed(mana_cost) = self.mana_cost else {
            return None;
        };
        let Some(SpellAdditionalCostDef::Exile {
            object: ObjectPredicateDef::Any,
            from: ZoneKind::Graveyard,
            quantity: CostQuantityDef::Fixed(cards),
        }) = self.additional_cost
        else {
            return None;
        };
        Some(format!(
            "Escape—{mana_cost}, Exile {} other card{} from your graveyard. (You may cast this card from your graveyard for its escape cost.)",
            Self::count_word(cards),
            if cards == 1 { "" } else { "s" },
        ))
    }

    /// Plot's reminder, which repeats the cost twice and so is long enough
    /// to sit apart from the walk over every other kind.
    fn plot_rules_text(mana_cost: AlternativeCastManaCostDef) -> String {
        match mana_cost {
            AlternativeCastManaCostDef::Fixed(mana_cost) => format!(
                "Plot {mana_cost} (You may pay {mana_cost} and exile this card from your hand. Cast it as a sorcery on a later turn without paying its mana cost. Plot only as a sorcery.)",
            ),
            // No card prints a plot cost equal to its own mana cost.
            AlternativeCastManaCostDef::ThisCardManaCost => "Plot".into(),
        }
    }

    /// The two kinds whose reminder is the clause's own printed text: what
    /// each does is too card-specific to rebuild from a cost.
    fn printed_rules_text(self, default: &'static str) -> String {
        self.stack_text
            .map_or_else(|| default.to_owned(), std::borrow::ToOwned::to_owned)
    }

    /// The kinds whose reminder is a fixed sentence rather than something
    /// rebuilt from a cost, each of which a card may override with its own
    /// printed text.
    fn fixed_rules_text(self) -> Option<String> {
        if self.kind == AlternativeCastKindDef::Escape
            && self.stack_text.is_none()
            && let Some(text) = self.common_escape_rules_text()
        {
            return Some(text);
        }
        let default = match self.kind {
            // Every printed bestow cost is a cost the card writes out, and
            // Detective's Phoenix writes a nonmana one, so the reminder is
            // taken from the clause rather than rebuilt from a mana cost.
            AlternativeCastKindDef::Rebound => {
                "Rebound (If you cast this spell from your hand, exile it as it resolves. At the beginning of your next upkeep, you may cast this card from exile without paying its mana cost.)"
            }
            AlternativeCastKindDef::Bestow => {
                "Bestow (If you cast this card for its bestow cost, it's an Aura spell with enchant creature. It becomes a creature again if it's not attached to a creature.)"
            }
            AlternativeCastKindDef::Impending => {
                "Impending (If you cast this spell for its impending cost, it enters with time counters and isn't a creature until the last is removed.)"
            }
            // The printed reminder names what is sacrificed, which the cost
            // itself does not say.
            AlternativeCastKindDef::Emerge => {
                "Emerge (You may cast this spell by sacrificing a permanent and paying the emerge cost reduced by that permanent's mana value.)"
            }
            AlternativeCastKindDef::Retrace => {
                "Retrace (You may cast this card from your graveyard by discarding a land card in addition to paying its other costs.)"
            }
            AlternativeCastKindDef::Escape => {
                "Escape (You may cast this card from your graveyard for its escape cost.)"
            }
            _ => return None,
        };
        Some(self.printed_rules_text(default))
    }

    #[must_use]
    pub fn rules_text(self) -> String {
        if let Some(text) = self.fixed_rules_text() {
            return text;
        }
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
            (AlternativeCastKindDef::Warp, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Warp {mana_cost} (You may cast this card from your hand for its warp cost. Exile it at the beginning of the next end step, then you may cast it from exile on a later turn.)",
                )
            }
            (AlternativeCastKindDef::Warp, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Warp—the warp cost is equal to this card's mana cost.".into()
            }
            (AlternativeCastKindDef::Dash, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Dash—the dash cost is equal to this card's mana cost.".into()
            }
            (AlternativeCastKindDef::Dash, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Dash {mana_cost} (You may cast this spell for its dash cost. If you do, it gains haste, and it's returned from the battlefield to its owner's hand at the beginning of the next end step.)",
                )
            }
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
            (AlternativeCastKindDef::Plot, mana_cost) => Self::plot_rules_text(mana_cost),
            // The reminder names what a splice does to somebody else's
            // spell, which the cost alone cannot say.
            (AlternativeCastKindDef::Splice, AlternativeCastManaCostDef::Fixed(mana_cost)) => {
                format!(
                    "Splice onto Arcane {mana_cost} (As you cast an Arcane spell, you may reveal this card from your hand and pay its splice cost. If you do, add this card's effects to that spell.)",
                )
            }
            (AlternativeCastKindDef::Splice, AlternativeCastManaCostDef::ThisCardManaCost) => {
                "Splice onto Arcane—the splice cost is equal to this card's mana cost.".into()
            }
            // Whatever printed or granted these said it in its own words, so
            // that text is the reminder and the kind's name is the fallback.
            (
                AlternativeCastKindDef::WithoutPayingManaCost | AlternativeCastKindDef::Offspring,
                _,
            ) => self
                .stack_text
                .map_or_else(|| self.kind.label().to_owned(), std::borrow::ToOwned::to_owned),
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
            // Every kind whose reminder is a fixed sentence answered above.
            (
                AlternativeCastKindDef::Rebound
                | AlternativeCastKindDef::Bestow
                | AlternativeCastKindDef::Impending
                | AlternativeCastKindDef::Emerge
                | AlternativeCastKindDef::Retrace
                | AlternativeCastKindDef::Escape,
                _,
            ) => unreachable!("answered by fixed_rules_text"),
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
