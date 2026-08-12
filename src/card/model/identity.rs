use crate::ids::{CardDefinitionId, CardPartId, MeldRecipeId, PlayOptionId};

use super::{CardRules, ManaCost};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSet {
    Alpha,
    Beta,
    Unlimited,
    CollectorsEdition,
    InternationalCollectorsEdition,
    ArabianNights,
    Antiquities,
    Revised,
    Legends,
    TheDark,
    FallenEmpires,
    Promo1994,
    IceAge,
    Mirage,
    Visions,
    Tempest,
    Stronghold,
    PortalSecondAge,
    UrzasSaga,
    MercadianMasques,
    Nemesis,
    Invasion,
    Planeshift,
    Apocalypse,
    Odyssey,
    Judgment,
    Onslaught,
    Darksteel,
    PlanarChaos,
    FutureSight,
    Innistrad,
    DarkAscension,
    AvacynRestored,
    Magic2013,
    ReturnToRavnica,
    Gatecrash,
    DragonsMaze,
    Magic2014,
    Theros,
    KhansOfTarkir,
    ModernHorizons2,
    /// Tokens are game objects rather than printed cards. They live in the
    /// catalog so a client can look one up by definition, and belong to no
    /// set a format allows, so they are never deck-legal.
    Token,
}

/// Stable identity of one exact printing of a card.
///
/// A card may have several printings in one set, such as basic lands with
/// different art. Variant zero is the primary printing when no alternate is
/// specified.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardPrintingId {
    pub definition: CardDefinitionId,
    pub set: CardSet,
    pub variant: u16,
}

impl CardPrintingId {
    #[must_use]
    pub const fn new(definition: CardDefinitionId, set: CardSet) -> Self {
        Self {
            definition,
            set,
            variant: 0,
        }
    }

    #[must_use]
    pub const fn with_variant(definition: CardDefinitionId, set: CardSet, variant: u16) -> Self {
        Self {
            definition,
            set,
            variant,
        }
    }
}

/// One cataloged set-and-variant printing of a canonical card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardPrinting {
    pub id: CardPrintingId,
}

impl CardPrinting {
    #[must_use]
    pub const fn new(definition: CardDefinitionId, set: CardSet) -> Self {
        Self {
            id: CardPrintingId::new(definition, set),
        }
    }

    #[must_use]
    pub const fn with_variant(definition: CardDefinitionId, set: CardSet, variant: u16) -> Self {
        Self {
            id: CardPrintingId::with_variant(definition, set, variant),
        }
    }
}

/// One independently addressable bundle of printed characteristics.
///
/// A part is broader than a physical face: the two halves of a split card are
/// separate parts printed on one face, while a transforming card has one part
/// on each physical face.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPart {
    pub id: CardPartId,
    pub name: String,
    pub rules: CardRules,
}

/// Whether a card part has a printed mana cost.
///
/// `Cost(ManaCost::default())` represents a printed `{0}` cost. `None` means
/// that no mana cost exists at all; it is not a cost that can ordinarily be
/// paid. This is stored directly in [`CardRules`] so a land or back face never
/// needs a dummy zero cost.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrintedManaCost {
    None,
    Cost(ManaCost),
}

impl PrintedManaCost {
    #[must_use]
    pub const fn as_option(self) -> Option<ManaCost> {
        match self {
            Self::None => None,
            Self::Cost(cost) => Some(cost),
        }
    }

    /// Both a nonexistent mana cost and a printed `{0}` cost have mana value
    /// zero, even though only the latter is a payable printed cost.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Cost(cost) => cost.mana_value(),
        }
    }
}

impl CardPart {
    #[must_use]
    pub fn new(id: CardPartId, name: impl Into<String>, rules: CardRules) -> Self {
        Self {
            id,
            name: name.into(),
            rules,
        }
    }

    #[must_use]
    pub const fn printed_mana_cost(&self) -> PrintedManaCost {
        self.rules.printed_mana_cost
    }

    #[must_use]
    pub const fn mana_cost(&self) -> Option<ManaCost> {
        self.rules.printed_mana_cost.as_option()
    }
}

/// The rules family used by a two-faced card.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DoubleFacedKind {
    Transforming,
    Modal,
}

/// A secondary spell frame printed alongside a card's ordinary characteristics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternateSpellKind {
    Adventure,
    Omen,
}

/// The physical/logical topology of a canonical card definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CardStructure {
    Single {
        main: CardPartId,
    },
    Split {
        parts: Vec<CardPartId>,
        /// The play option that combines the parts, if the card has one.
        fused: Option<PlayOptionId>,
    },
    Flip {
        normal: CardPartId,
        flipped: CardPartId,
    },
    DoubleFaced {
        front: CardPartId,
        back: CardPartId,
        kind: DoubleFacedKind,
    },
    AlternateSpell {
        main: CardPartId,
        alternate: CardPartId,
        kind: AlternateSpellKind,
    },
    /// A physical card that can participate in a separately cataloged meld
    /// recipe. The recipe, rather than either component definition, supplies
    /// the combined object's result characteristics.
    MeldPart {
        front: CardPartId,
        recipe: MeldRecipeId,
    },
}

/// One named-object condition and one physical-card requirement in a future
/// meld recipe.
///
/// These are deliberately separate. An object's effective name can satisfy
/// `required_name` even when it is a token or copy, while a successful meld
/// must ultimately be backed by the physical `required_card`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldComponentDef {
    pub required_name: String,
    pub required_card: CardDefinitionId,
}

/// Characteristics of the combined object produced by a meld recipe.
///
/// This is not a printing and does not pretend to be either component card.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldResultDef {
    pub name: String,
    pub rules: CardRules,
}

/// Catalog data needed to implement meld later without conflating its name
/// predicate with its physical-card validation.
///
/// No supported format executes meld today; this type is intentionally not
/// wired into game actions or resolution yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldRecipeDef {
    pub id: MeldRecipeId,
    pub components: [MeldComponentDef; 2],
    pub result: MeldResultDef,
}

/// The characteristic parts used by an object while it is a spell.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SpellForm {
    Part(CardPartId),
    /// Combined parts retain printed order, which is also resolution order for
    /// a fused split spell.
    Combined(Vec<CardPartId>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayActionKind {
    CastSpell,
    PlayLand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayRestriction {
    Normal,
    FromHandOnly,
    /// "Cast this spell only before the combat damage step." Legal until the
    /// damage is about to be dealt, which is what makes Berserk a decision the
    /// defender can play around rather than a guaranteed blowout.
    BeforeCombatDamage,
}

/// A catalog-level description of what can occupy one target slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetPredicate {
    AnyTarget,
    Player,
    Permanent,
    CreaturePermanent,
    Spell,
    NoncreatureSpell,
}

/// A zone in which an ability can exist or an object can be selected.
///
/// This is catalog vocabulary. Runtime zones may store objects differently,
/// but card definitions should not need to know those storage details.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneKind {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

/// A player described relative to an ability's controller or triggering
/// event, rather than by a game-specific player identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerRelation {
    Any,
    You,
    /// Any player other than the ability's controller. This matches "you
    /// don't control" without assuming that every other player is an
    /// opponent.
    NotYou,
    Opponent,
    ActivePlayer,
    NonactivePlayer,
    /// The player identified directly by the event, such as the player whose
    /// upkeep began or who cast a spell.
    EventPlayer,
    /// The player the ability's own source chose as it entered. Only a
    /// permanent that made such a choice matches anyone at all.
    ChosenPlayer,
}
