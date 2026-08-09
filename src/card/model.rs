use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, MeldRecipeId,
    ModeId, PlayOptionId, TargetSlotId,
};

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
    /// Cast from the graveyard for an alternative cost, exiling the card as it
    /// leaves the stack. This is flashback, CR 702.34.
    Flashback,
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
    Opponent,
    ActivePlayer,
    NonactivePlayer,
    /// The player identified directly by the event, such as the player whose
    /// upkeep began or who cast a spell.
    EventPlayer,
}

/// One of the five colors of Magic, or colorless mana.
///
/// The same vocabulary is used by card characteristics, mana-producing
/// effects, and the runtime mana pool. `Colorless` is a mana type rather than
/// a color, so it has no index in a card's five-color characteristic set.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl ManaColor {
    pub const COLORS: [Self; 5] = [Self::White, Self::Blue, Self::Black, Self::Red, Self::Green];

    pub const ALL: [Self; 6] = [
        Self::White,
        Self::Blue,
        Self::Black,
        Self::Red,
        Self::Green,
        Self::Colorless,
    ];

    #[must_use]
    pub const fn color_index(self) -> Option<usize> {
        match self {
            Self::White => Some(0),
            Self::Blue => Some(1),
            Self::Black => Some(2),
            Self::Red => Some(3),
            Self::Green => Some(4),
            Self::Colorless => None,
        }
    }
}

/// The colors an object has as a characteristic.
///
/// Colorless is represented by the empty set, not by a sixth flag. The
/// protocol-facing [`CardRules::colors`] method continues to project this as
/// `[white, blue, black, red, green]` for compatibility.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ColorSet(u8);

impl ColorSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    /// # Panics
    ///
    /// Panics if `colors` contains [`ManaColor::Colorless`], which is a mana
    /// type rather than a color characteristic.
    pub const fn from_colors(colors: &[ManaColor]) -> Self {
        let mut result = Self::empty();
        let mut index = 0;
        while index < colors.len() {
            result = result.with(colors[index]);
            index += 1;
        }
        result
    }

    #[must_use]
    /// # Panics
    ///
    /// Panics if `color` is [`ManaColor::Colorless`]. A colorless object is
    /// represented by an empty set.
    pub const fn with(mut self, color: ManaColor) -> Self {
        let Some(index) = color.color_index() else {
            panic!("colorless is not a color characteristic");
        };
        self.0 |= 1 << index;
        self
    }

    #[must_use]
    pub const fn contains(self, color: ManaColor) -> bool {
        let Some(index) = color.color_index() else {
            return false;
        };
        self.0 & (1 << index) != 0
    }

    #[must_use]
    pub const fn is_colorless(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn to_flags(self) -> [bool; 5] {
        [
            self.contains(ManaColor::White),
            self.contains(ManaColor::Blue),
            self.contains(ManaColor::Black),
            self.contains(ManaColor::Red),
            self.contains(ManaColor::Green),
        ]
    }
}

/// One atomic card type. A card's type line is represented by a
/// [`CardTypeSet`], so combinations such as artifact creatures, enchantment
/// creatures, artifact lands, and land creatures do not require bespoke enum
/// variants.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardType {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    Land,
    Planeswalker,
    Sorcery,
}

impl CardType {
    pub const COUNT: usize = 7;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Artifact,
        Self::Creature,
        Self::Enchantment,
        Self::Instant,
        Self::Land,
        Self::Planeswalker,
        Self::Sorcery,
    ];

    /// Conventional type-line order for the combinations the catalog can
    /// currently express. This is deliberately independent of bit indexes.
    pub const DISPLAY_ORDER: [Self; Self::COUNT] = [
        Self::Artifact,
        Self::Enchantment,
        Self::Land,
        Self::Creature,
        Self::Planeswalker,
        Self::Instant,
        Self::Sorcery,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Artifact => 0,
            Self::Creature => 1,
            Self::Enchantment => 2,
            Self::Instant => 3,
            Self::Land => 4,
            Self::Planeswalker => 5,
            Self::Sorcery => 6,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Artifact => "Artifact",
            Self::Creature => "Creature",
            Self::Enchantment => "Enchantment",
            Self::Instant => "Instant",
            Self::Land => "Land",
            Self::Planeswalker => "Planeswalker",
            Self::Sorcery => "Sorcery",
        }
    }
}

/// A const-friendly set of card types stored on one card part.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct CardTypeSet(u32);

impl CardTypeSet {
    pub const EMPTY: Self = Self(0);

    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    #[must_use]
    pub const fn single(card_type: CardType) -> Self {
        Self(1 << card_type.index())
    }

    #[must_use]
    pub const fn with(mut self, card_type: CardType) -> Self {
        self.0 |= 1 << card_type.index();
        self
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[must_use]
    pub const fn contains(self, card_type: CardType) -> bool {
        self.0 & (1 << card_type.index()) != 0
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_creature(self) -> bool {
        self.contains(CardType::Creature)
    }

    #[must_use]
    pub const fn is_artifact(self) -> bool {
        self.contains(CardType::Artifact)
    }

    #[must_use]
    pub const fn is_permanent(self) -> bool {
        self.contains(CardType::Artifact)
            || self.contains(CardType::Creature)
            || self.contains(CardType::Enchantment)
            || self.contains(CardType::Land)
            || self.contains(CardType::Planeswalker)
    }

    /// Compatibility spelling used by the existing protocol `kind` field.
    ///
    /// Current single-type cards retain names such as `Instant`; an artifact
    /// creature retains `ArtifactCreature`. New combinations are represented
    /// by concatenating their card types in rules-defined type-line order.
    #[must_use]
    pub fn kind_name(self) -> String {
        CardType::DISPLAY_ORDER
            .into_iter()
            .filter(|card_type| self.contains(*card_type))
            .map(CardType::name)
            .collect()
    }

    #[must_use]
    pub fn type_name(self) -> String {
        CardType::DISPLAY_ORDER
            .into_iter()
            .filter(|card_type| self.contains(*card_type))
            .map(CardType::name)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl fmt::Debug for CardTypeSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_set()
            .entries(
                CardType::DISPLAY_ORDER
                    .into_iter()
                    .filter(|card_type| self.contains(*card_type)),
            )
            .finish()
    }
}

/// A composable predicate over a card or game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectPredicateDef {
    Any,
    Source,
    HasType(CardType),
    Spell,
    NoncreatureSpell,
    Color(ManaColor),
    Subtype(&'static str),
    /// Mana value at most this much, for "with mana value N or less".
    ManaValueAtMost(u8),
    /// Mana value exactly this much, where the number is read off the
    /// ability's own source rather than printed on the card.
    ManaValueEqualTo(ValueDef),
    /// Mana value at most a computed amount, for "with mana value X or less".
    ManaValueAtMostValue(ValueDef),
    /// Power at least this much, for "power N or greater". Reads current
    /// power on the battlefield, so a pumped creature qualifies.
    PowerAtLeast(i16),
    /// Controlled by a player in this relation to the ability's controller,
    /// for "a creature you control" and "whenever you cast".
    ControlledBy(PlayerRelation),
    /// Carries this supertype. Negate it for "nonbasic".
    Supertype(CardSupertype),
    /// Has the same printed name as the ability's source. Negate it for
    /// "not named <this card>".
    SharesNameWithSource,
    /// Currently attacking or blocking. Only a battlefield object can be, so
    /// this never matches a card or a spell.
    AttackingOrBlocking,
    /// Has this keyword. Protection is not askable this way, because it is a
    /// keyword per color rather than one keyword.
    HasKeyword(KeywordAbility),
    All(&'static [ObjectPredicateDef]),
    AnyOf(&'static [ObjectPredicateDef]),
    Not(&'static ObjectPredicateDef),
    /// A narrow, named predicate that cannot yet be expressed by the common
    /// vocabulary. The engine owns the meaning of each supported name.
    Special(&'static str),
}

/// The legal subject of one ability target slot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityTargetPredicate {
    AnyTarget,
    Player(PlayerRelation),
    Object {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        /// Relation of the object's controller, when the zone supplies one.
        controller: Option<PlayerRelation>,
        /// Relation of the physical card's owner. This is the relevant
        /// relation for private zones such as a graveyard.
        owner: Option<PlayerRelation>,
    },
}

/// A const-friendly target declaration kept beside a printed ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityTargetDef {
    pub id: TargetSlotId,
    pub label: &'static str,
    pub predicate: AbilityTargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
}

impl AbilityTargetDef {
    #[must_use]
    pub const fn exactly_one(
        id: TargetSlotId,
        label: &'static str,
        predicate: AbilityTargetPredicate,
    ) -> Self {
        Self {
            id,
            label,
            predicate,
            minimum: 1,
            maximum: 1,
        }
    }

    /// Any number of targets up to a limit, for "up to three target ...".
    /// Choosing none is a legal choice.
    #[must_use]
    pub const fn up_to(
        id: TargetSlotId,
        label: &'static str,
        predicate: AbilityTargetPredicate,
        maximum: u8,
    ) -> Self {
        Self {
            id,
            label,
            predicate,
            minimum: 0,
            maximum,
        }
    }

    /// One spell target, optionally narrowed by color, type, or another
    /// object predicate. Stack object enumeration already excludes abilities,
    /// so callers only need to state the characteristic restriction.
    #[must_use]
    pub const fn exactly_one_spell(
        id: TargetSlotId,
        label: &'static str,
        object: ObjectPredicateDef,
    ) -> Self {
        Self::exactly_one(
            id,
            label,
            AbilityTargetPredicate::Object {
                object,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )
    }

    /// One permanent target, optionally narrowed by color, type, or another
    /// object predicate.
    #[must_use]
    pub const fn exactly_one_permanent(
        id: TargetSlotId,
        label: &'static str,
        object: ObjectPredicateDef,
    ) -> Self {
        Self::exactly_one(
            id,
            label,
            AbilityTargetPredicate::Object {
                object,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )
    }
}

/// A cost paid to activate an ability. The ability category, rather than the
/// presence of an `AddMana` effect, determines whether it is a mana ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityCostDef {
    Mana(ManaCost),
    TapSource,
    UntapSource,
    SacrificeSource,
    PayLife(u16),
    DiscardCards(u8),
    SacrificePermanent {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    ExileSource,
    Special(&'static str),
}

/// A basic land subtype used by type-changing effects and mana provenance.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicLandType {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

impl BasicLandType {
    pub const ALL: [Self; 5] = [
        Self::Plains,
        Self::Island,
        Self::Swamp,
        Self::Mountain,
        Self::Forest,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Plains => 0,
            Self::Island => 1,
            Self::Swamp => 2,
            Self::Mountain => 3,
            Self::Forest => 4,
        }
    }

    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Plains),
            1 => Some(Self::Island),
            2 => Some(Self::Swamp),
            3 => Some(Self::Mountain),
            4 => Some(Self::Forest),
            _ => None,
        }
    }

    #[must_use]
    pub const fn mana_color(self) -> ManaColor {
        match self {
            Self::Plains => ManaColor::White,
            Self::Island => ManaColor::Blue,
            Self::Swamp => ManaColor::Black,
            Self::Mountain => ManaColor::Red,
            Self::Forest => ManaColor::Green,
        }
    }

    #[must_use]
    pub const fn subtype(self) -> &'static str {
        match self {
            Self::Plains => "Plains",
            Self::Island => "Island",
            Self::Swamp => "Swamp",
            Self::Mountain => "Mountain",
            Self::Forest => "Forest",
        }
    }

    #[must_use]
    pub fn from_subtype(subtype: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|land_type| land_type.subtype() == subtype)
    }
}

/// Which kind of mana an effect adds. A choice is made as the mana ability
/// resolves; it is not modeled as several interchangeable colors already in
/// the pool.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSelectionDef {
    One(ManaColor),
    Choice(&'static [ManaColor]),
}

/// A restriction carried by produced mana until that mana is spent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaRestrictionDef {
    CastSpell(ObjectPredicateDef),
    CastCreatureSpellOfChosenType,
    ActivateAbility(ObjectPredicateDef),
    Special(&'static str),
}

/// An effect applied to the spell or ability paid for with a mana unit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSpendEffectDef {
    ApplyToPaidSpell(AppliedEffectDef),
    ApplyToPaidAbility(AppliedEffectDef),
    Special(&'static str),
}

/// One set of indistinguishable mana units created by an effect. The runtime
/// pool may store `amount` as a count keyed by the remaining fields.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AddManaEffectDef {
    pub mana: ManaSelectionDef,
    pub amount: u16,
    pub restrictions: &'static [ManaRestrictionDef],
    pub spend_effects: &'static [ManaSpendEffectDef],
}

impl AddManaEffectDef {
    #[must_use]
    pub const fn one(mana: ManaColor) -> Self {
        Self {
            mana: ManaSelectionDef::One(mana),
            amount: 1,
            restrictions: &[],
            spend_effects: &[],
        }
    }

    #[must_use]
    pub const fn choice(mana: &'static [ManaColor]) -> Self {
        Self {
            mana: ManaSelectionDef::Choice(mana),
            amount: 1,
            restrictions: &[],
            spend_effects: &[],
        }
    }

    #[must_use]
    pub const fn with_amount(mut self, amount: u16) -> Self {
        self.amount = amount;
        self
    }

    #[must_use]
    pub const fn with_restrictions(mut self, restrictions: &'static [ManaRestrictionDef]) -> Self {
        self.restrictions = restrictions;
        self
    }

    #[must_use]
    pub const fn with_spend_effects(
        mut self,
        spend_effects: &'static [ManaSpendEffectDef],
    ) -> Self {
        self.spend_effects = spend_effects;
        self
    }
}

/// A value evaluated from the resolving spell or ability and its captured
/// event. `SourcePower` and `SourceToughness` deliberately leave current-versus
/// last-known-information selection to the runtime source reference.
/// A set of objects described the way [`EffectRecipientDef::MatchingObjects`]
/// describes one, so a count and a sweep name their subject identically.
/// The two branches of a conditional value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionalValueDef {
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

/// A conditional value that asks how many objects match.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CountConditionDef {
    pub query: ObjectQueryDef,
    pub equals: u8,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

/// A conditional value that asks what the chosen target is.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TargetConditionDef {
    pub slot: TargetSlotId,
    pub object: ObjectPredicateDef,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectQueryDef {
    pub object: ObjectPredicateDef,
    pub zones: &'static [ZoneKind],
    pub controller: PlayerRelation,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueDef {
    Constant(i32),
    ChosenX,
    SourcePower,
    SourceToughness,
    TriggerEventAmount,
    CardsInHandAbove {
        player: PlayerRelation,
        threshold: u8,
    },
    /// How many objects match, for the "for each" clauses. Held by reference
    /// so that `ValueDef` stays small enough to embed freely.
    CountMatchingObjects(&'static ObjectQueryDef),
    /// One when at least one object matches, zero otherwise. "As long as you
    /// control a Mountain" is a condition rather than a count, so counting
    /// matches would pay a second Mountain twice.
    AnyMatchingObject(&'static ObjectQueryDef),
    /// The negation of another value, so a "for each" penalty can reuse the
    /// same count a bonus would.
    Negate(&'static ValueDef),
    /// How many counters of one kind sit on the ability's own source.
    CountersOnSource(CounterKind),
    /// The morbid condition. Held by reference so that `ValueDef` stays one
    /// word wide; a second inline value would grow everything embedding it.
    IfCreatureDiedThisTurn(&'static ConditionalValueDef),
    /// One value when the chosen target matches, another when it does not.
    /// Held by reference for the same reason.
    IfTargetMatches(&'static TargetConditionDef),
    /// One value when exactly that many objects match, another otherwise.
    /// This is how an intervening-if condition becomes an amount.
    IfMatchingObjectCount(&'static CountConditionDef),
}

/// An object or player affected by an effect. Targets are chosen when a spell
/// or stack ability is formed; triggering subjects come from captured events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectRecipientDef {
    Source,
    /// What this permanent is attached to, for an Aura's own static clauses.
    AttachedPermanent,
    /// Every battlefield permanent sharing a name with the chosen target,
    /// including the target itself. "And each other one with the same name"
    /// names the same set.
    ObjectsSharingNameWithTarget(TargetSlotId),
    Controller,
    Opponent,
    Target(TargetSlotId),
    TriggeringObject,
    /// The triggering object's controller when this effect resolves, using
    /// last-known information if that object is no longer live.
    ControllerOfTriggeringObject,
    /// The player named directly by the event, such as the player whose
    /// upkeep began or who cast the triggering spell.
    EventPlayer,
    MatchingObjects {
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller: PlayerRelation,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDurationDef {
    Permanent,
    UntilEndOfTurn,
    WhileSourceRemainsInZone,
    UntilSourceLeavesZone,
}

/// A continuous or rules-modifying effect applied to a game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppliedEffectDef {
    CannotBeCountered,
    /// A creature matching this predicate cannot block the affected creature.
    CannotBeBlockedBy(ObjectPredicateDef),
    /// Adds land subtypes without removing the object's existing subtypes.
    AddLandTypes(&'static [BasicLandType]),
    ModifyPowerToughness {
        power: ValueDef,
        toughness: ValueDef,
    },
    GrantAbility(&'static AbilityDef),
    Special(&'static str),
}

/// Declarative effect primitives interpreted by the rules engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectDef {
    None,
    Sequence(&'static [EffectDef]),
    AddMana(AddManaEffectDef),
    DealDamage {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    GainLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    DrawCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Each recipient chooses that many cards from their own hand and
    /// discards them. A player holding fewer cards discards their whole hand.
    DiscardCards {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    LoseLife {
        recipient: EffectRecipientDef,
        amount: ValueDef,
    },
    Tap {
        object: EffectRecipientDef,
    },
    Untap {
        object: EffectRecipientDef,
    },
    /// Puts token copies of `token` onto the battlefield under the resolving
    /// object's controller.
    CreateToken {
        token: CardDefinitionId,
        count: ValueDef,
    },
    /// An Aura spell attaching itself to what it enchants. The permanent the
    /// spell becomes is what attaches, so this is only meaningful on the spell
    /// clause of an Aura.
    Attach {
        object: EffectRecipientDef,
    },
    Destroy {
        object: EffectRecipientDef,
        can_regenerate: bool,
    },
    Sacrifice {
        object: EffectRecipientDef,
    },
    /// Each recipient player chooses one permanent they control that matches,
    /// and sacrifices it. Unlike [`Self::Sacrifice`] the choice is the
    /// player's, so nothing happens when they control nothing matching.
    SacrificeOfChoice {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
    },
    Counter {
        object: EffectRecipientDef,
    },
    /// Counters unless the spell's own controller pays this much generic
    /// mana. `zone` is where a spell countered this way goes, which is the
    /// graveyard unless the card says otherwise.
    CounterUnlessPaid {
        object: EffectRecipientDef,
        amount: ValueDef,
        zone: ZoneKind,
    },
    AddCounters {
        object: EffectRecipientDef,
        kind: CounterKind,
        amount: ValueDef,
    },
    /// On resolution, choose two different basic land-type words and apply
    /// the resulting indefinite, noncopiable text change to the object.
    ChangeTextBasicLandType {
        object: EffectRecipientDef,
    },
    /// Replaces the source permanent's copiable values with the target's.
    /// Some copy effects, such as Thespian's Stage, retain the resolving
    /// ability as an exception to the copied values.
    BecomeCopyOf {
        object: EffectRecipientDef,
        retain_source_ability: bool,
    },
    OptionalManaPayment {
        cost: ManaCost,
        effect: &'static EffectDef,
    },
    EntersTapped,
    /// Lets the next sorcery its controller casts this turn be cast as
    /// though it had flash.
    GrantFlashToNextSorcery,
    /// An effect its controller may decline. Held by reference so that
    /// `EffectDef` does not grow a recursive inline copy of itself.
    May(&'static EffectDef),
    /// Exiles, remembering which object sent it there so a later clause can
    /// bring it back. This is the Oblivion Ring shape.
    ExileLinkedToSource {
        object: EffectRecipientDef,
    },
    /// Returns everything this ability's source exiled, to the named zone.
    /// A returned permanent keeps `grant` until end of turn, which is how
    /// Obzedat comes back ready to attack.
    ReturnLinkedExiles {
        zone: ZoneKind,
        grant: Option<KeywordAbility>,
    },
    /// Makes an object unblockable for the rest of the turn.
    MakeUnblockableThisTurn {
        object: EffectRecipientDef,
    },
    /// Queues an effect for the next time that step begins.
    AtNextStep {
        step: TurnStepDef,
        player: PlayerRelation,
        effect: &'static EffectDef,
    },
    /// A static prohibition: no spell or ability an opponent controls can
    /// make this ability's controller sacrifice a permanent.
    CannotBeForcedToSacrifice,
    /// This card costs that much less generic mana to cast. A static ability
    /// that works from the hand, where casting reads it.
    ReduceGenericCostBy(ValueDef),
    /// Multiplies the amount of the event a replacement ability is replacing.
    /// This means nothing outside a replacement whose event carries an amount.
    MultiplyEventAmount(u8),
    MoveToZone {
        object: EffectRecipientDef,
        zone: ZoneKind,
    },
    /// Choose and store a creature type for an object as it enters. This is a
    /// replacement procedure rather than a resolving stack effect.
    ChooseCreatureType {
        object: EffectRecipientDef,
    },
    Apply {
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
    },
    /// A descriptive marker for an effect portion the shared vocabulary does
    /// not yet represent. The surrounding costs, targets, and timing can still
    /// remain declarative; clause coverage records whether and how it executes.
    Special(&'static str),
}

impl EffectDef {
    #[must_use]
    pub const fn counter_target(target: TargetSlotId) -> Self {
        Self::Counter {
            object: EffectRecipientDef::Target(target),
        }
    }

    #[must_use]
    pub const fn destroy_target(target: TargetSlotId, can_regenerate: bool) -> Self {
        Self::Destroy {
            object: EffectRecipientDef::Target(target),
            can_regenerate,
        }
    }
}

/// Turn structure used by beginning/end-of-step trigger declarations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TurnStepDef {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

/// The committed event observed by a triggered ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerEventDef {
    ZoneChanged {
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    },
    BecomesTapped(ObjectPredicateDef),
    /// A permanent was tapped to pay for one of its own mana abilities. This
    /// is narrower than [`Self::BecomesTapped`]: attacking or a tap effect
    /// does not produce mana and does not fire this.
    TappedForMana(ObjectPredicateDef),
    /// A creature was declared as an attacker. Every matching attacker in one
    /// declaration triggers separately, as CR 508.2 has them all attack at
    /// once rather than one at a time.
    Attacks(ObjectPredicateDef),
    SpellCast(ObjectPredicateDef),
    AbilityActivated(ObjectPredicateDef),
    StepBegins {
        step: TurnStepDef,
        player: PlayerRelation,
    },
    DamageDealt {
        source: ObjectPredicateDef,
        recipient: EffectRecipientDef,
    },
    ManaAdded(PlayerRelation),
    /// A player gained life. The amount is available as
    /// [`ValueDef::TriggerEventAmount`].
    LifeGained(PlayerRelation),
    /// A creature dealt damage by this ability's source this turn died.
    DamagedCreatureDied,
    Special(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpellAbilityDef {
    Nonmodal {
        targets: &'static [AbilityTargetDef],
    },
    Modal(ModalSpellDef),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModalSpellDef {
    /// Each mode is an ordinary spell ability. Its positional index supplies
    /// the stable [`ModeId`] used by casting and presentation.
    pub modes: &'static [AbilityDef],
    pub minimum: u8,
    pub maximum: u8,
    /// Some spells explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
}

impl ModalSpellDef {
    #[must_use]
    pub const fn new(
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self {
            modes,
            minimum,
            maximum,
            may_repeat,
        }
    }

    #[must_use]
    pub const fn choose_one(modes: &'static [AbilityDef]) -> Self {
        Self::new(modes, 1, 1, false)
    }
}

impl SpellAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self::Nonmodal { targets: &[] }
    }

    /// Adds targets to an ordinary, nonmodal spell definition.
    ///
    /// # Panics
    ///
    /// Panics for a modal wrapper because each mode declares its own targets.
    #[must_use]
    pub const fn with_targets(self, targets: &'static [AbilityTargetDef]) -> Self {
        match self {
            Self::Nonmodal { .. } => Self::Nonmodal { targets },
            Self::Modal(_) => panic!("targets belong on modal spell branches"),
        }
    }

    #[must_use]
    pub const fn modal_spell(
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self::Modal(ModalSpellDef::new(modes, minimum, maximum, may_repeat))
    }

    /// Returns targets declared directly by a nonmodal spell. Modal wrappers
    /// have no direct targets; selected branches supply them instead.
    #[must_use]
    pub const fn targets(self) -> &'static [AbilityTargetDef] {
        match self {
            Self::Nonmodal { targets } => targets,
            Self::Modal(_) => &[],
        }
    }

    #[must_use]
    pub const fn modal(self) -> Option<ModalSpellDef> {
        match self {
            Self::Nonmodal { .. } => None,
            Self::Modal(modal) => Some(modal),
        }
    }

    #[must_use]
    pub fn mode(self, id: ModeId) -> Option<&'static AbilityDef> {
        self.modal()?.modes.get(id.index())
    }
}

impl Default for SpellAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
    pub targets: &'static [AbilityTargetDef],
}

impl ActivatedAbilityDef {
    #[must_use]
    pub const fn new(costs: &'static [AbilityCostDef]) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            costs,
            targets: &[],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggeredAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    pub targets: &'static [AbilityTargetDef],
}

impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(event: TriggerEventDef) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        self.targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

/// A replacement ability changes how an event happens and never uses the
/// stack. It is modeled separately from a triggered ability even when both
/// watch the same event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplacementAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: ReplacementEventDef,
}

/// The event changed by a replacement ability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEventDef {
    /// This ability's source would move between the named zones for the
    /// specified reason. Matching happens before the object leaves `from`.
    WouldMove {
        from: ZoneKind,
        to: ZoneKind,
        cause: ZoneMoveCauseDef,
    },
    /// A player would gain life, matched relative to the replacement
    /// ability's controller.
    WouldGainLife(PlayerRelation),
    /// Entry replacement effects whose exact event is already identified by
    /// their effect primitive (for example, enters tapped or choosing a
    /// creature type as an object enters).
    EntersBattlefield,
    Special(&'static str),
}

/// What is causing a proposed zone move. A controlled effect is matched
/// relative to the replacement ability's controller; rules and costs do not
/// have an effect controller and therefore only match [`Self::Any`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneMoveCauseDef {
    Any,
    EffectControlledBy(PlayerRelation),
}

impl ReplacementAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event: ReplacementEventDef::EntersBattlefield,
        }
    }

    #[must_use]
    pub const fn with_event(mut self, event: ReplacementEventDef) -> Self {
        self.event = event;
        self
    }
}

impl Default for ReplacementAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A rules-defined action a player may take without using the stack, such as
/// turning a face-down permanent face up. This is deliberately distinct from
/// both activated abilities and mana abilities; its timing category is never
/// inferred from its cost or effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpecialActionDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: &'static [AbilityCostDef],
}

impl SpecialActionDef {
    #[must_use]
    pub const fn new(source_zones: &'static [ZoneKind], costs: &'static [AbilityCostDef]) -> Self {
        Self {
            source_zones,
            costs,
        }
    }
}

impl StaticAbilityDef {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
        }
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
        self
    }
}

impl Default for StaticAbilityDef {
    fn default() -> Self {
        Self::new()
    }
}

/// A keyword ability carried as an ordinary, ordered rules clause.
///
/// The clause's [`AbilityImplementationDef`] says whether the engine currently
/// executes the keyword. This keeps unimplemented keywords such as first
/// strike visible and accurately reflected in aggregate coverage without
/// hiding them in card-level booleans.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeywordAbility {
    Flying,
    Trample,
    Haste,
    FirstStrike,
    DoubleStrike,
    Banding,
    Vigilance,
    Defender,
    Deathtouch,
    Lifelink,
    Reach,
    Flash,
    Hexproof,
    Intimidate,
    Undying,
    Mountainwalk,
    ProtectionFrom(ManaColor),
}

impl KeywordAbility {
    /// A dense index for the keywords that carry no parameter, so a set of
    /// them fits in a bitmask. Protection is excluded: it is really one
    /// keyword per color.
    #[must_use]
    pub const fn simple_index(self) -> Option<u32> {
        Some(match self {
            Self::Flying => 0,
            Self::Trample => 1,
            Self::Haste => 2,
            Self::FirstStrike => 3,
            Self::DoubleStrike => 4,
            Self::Banding => 5,
            Self::Vigilance => 6,
            Self::Defender => 7,
            Self::Deathtouch => 8,
            Self::Lifelink => 9,
            Self::Reach => 10,
            Self::Flash => 11,
            Self::Hexproof => 12,
            Self::Intimidate => 13,
            Self::Undying => 14,
            Self::Mountainwalk => 15,
            Self::ProtectionFrom(_) => return None,
        })
    }
}

/// The rules category and structural procedure of an ability. Text and
/// implementation coverage live on [`AbilityDef`] so every printed clause has
/// one canonical text string regardless of how it executes. Identity is
/// supplied only when a definition is attached.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarativeAbilityDef {
    Spell(SpellAbilityDef),
    ActivatedMana(ActivatedAbilityDef),
    TriggeredMana(TriggeredAbilityDef),
    Activated(ActivatedAbilityDef),
    Triggered(TriggeredAbilityDef),
    Static(StaticAbilityDef),
    Replacement(ReplacementAbilityDef),
    SpecialAction(SpecialActionDef),
    Keyword(KeywordAbility),
    /// Transitional structural marker for a clause still dispatched through
    /// the owning card's legacy custom behavior.
    Legacy,
}

/// How completely one printed rules clause is implemented.
///
/// Fully declarative clauses need no explanation: their structure and effect
/// are the implementation. Every other variant explains the custom
/// implementation or remaining gap beside the clause that owns it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AbilityImplementationDef {
    Definition,
    CustomFull {
        behavior: Option<CardBehavior>,
        explanation: &'static str,
    },
    CustomPartial {
        behavior: Option<CardBehavior>,
        explanation: &'static str,
    },
    NotImplemented {
        explanation: &'static str,
    },
}

impl AbilityImplementationDef {
    #[must_use]
    pub const fn explanation(self) -> Option<&'static str> {
        match self {
            Self::Definition => None,
            Self::CustomFull { explanation, .. }
            | Self::CustomPartial { explanation, .. }
            | Self::NotImplemented { explanation } => Some(explanation),
        }
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        match self {
            Self::CustomFull { behavior, .. } | Self::CustomPartial { behavior, .. } => behavior,
            Self::Definition | Self::NotImplemented { .. } => None,
        }
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        !matches!(self, Self::NotImplemented { .. })
    }
}

/// One printed rules clause and its implementation.
///
/// The category is explicit even when the implementation remains custom; the
/// engine never infers stack behavior from costs, targets, or effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityDef {
    pub text: &'static str,
    /// Optional action-menu wording for a targeted activated ability. This is
    /// presentation attached to the exact ability, not a second rules text.
    pub activation_text: Option<ActivatedAbilityText>,
    pub definition: DeclarativeAbilityDef,
    pub effect: EffectDef,
    pub implementation: AbilityImplementationDef,
}

impl AbilityDef {
    #[must_use]
    pub const fn spell(text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::new()),
            effect,
        )
    }

    /// A one-target counterspell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn counter_target(text: &'static str, target: &'static AbilityTargetDef) -> Self {
        Self::spell(text, EffectDef::counter_target(target.id))
            .with_targets(core::slice::from_ref(target))
    }

    /// A one-target destroy spell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn destroy_target(
        text: &'static str,
        target: &'static AbilityTargetDef,
        can_regenerate: bool,
    ) -> Self {
        Self::spell(text, EffectDef::destroy_target(target.id, can_regenerate))
            .with_targets(core::slice::from_ref(target))
    }

    #[must_use]
    pub const fn unimplemented_spell(text: &'static str, explanation: &'static str) -> Self {
        Self::spell(text, EffectDef::None)
            .with_implementation(AbilityImplementationDef::NotImplemented { explanation })
    }

    #[must_use]
    pub const fn modal_spell(
        text: &'static str,
        modes: &'static [AbilityDef],
        minimum: u8,
        maximum: u8,
        may_repeat: bool,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::modal_spell(
                modes, minimum, maximum, may_repeat,
            )),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn choose_one_spell(text: &'static str, modes: &'static [AbilityDef]) -> Self {
        Self::modal_spell(text, modes, 1, 1, false)
    }

    #[must_use]
    pub const fn activated_mana(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::ActivatedMana(ActivatedAbilityDef::new(costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered_mana(
        text: &'static str,
        event: TriggerEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::TriggeredMana(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn activated(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Activated(ActivatedAbilityDef::new(costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered(text: &'static str, event: TriggerEventDef, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(TriggeredAbilityDef::new(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn static_ability(text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Static(StaticAbilityDef::new()),
            effect,
        )
    }

    #[must_use]
    pub const fn keyword(text: &'static str, ability: KeywordAbility) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Keyword(ability),
            EffectDef::None,
        )
    }

    #[must_use]
    pub const fn replacement(text: &'static str, effect: EffectDef) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Replacement(ReplacementAbilityDef::new()),
            effect,
        )
    }

    #[must_use]
    pub const fn replacement_for(
        text: &'static str,
        event: ReplacementEventDef,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Replacement(ReplacementAbilityDef::new().with_event(event)),
            effect,
        )
    }

    #[must_use]
    pub const fn special_action(
        text: &'static str,
        source_zones: &'static [ZoneKind],
        costs: &'static [AbilityCostDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::SpecialAction(SpecialActionDef::new(source_zones, costs)),
            effect,
        )
    }

    #[must_use]
    pub const fn defined(
        text: &'static str,
        definition: DeclarativeAbilityDef,
        effect: EffectDef,
    ) -> Self {
        Self {
            text,
            activation_text: None,
            definition,
            effect,
            implementation: AbilityImplementationDef::Definition,
        }
    }

    #[must_use]
    pub const fn custom_full(
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::CustomFull {
                behavior: Some(behavior),
                explanation,
            },
        }
    }

    #[must_use]
    pub const fn custom_partial(
        text: &'static str,
        behavior: CardBehavior,
        explanation: &'static str,
    ) -> Self {
        Self {
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::CustomPartial {
                behavior: Some(behavior),
                explanation,
            },
        }
    }

    #[must_use]
    pub const fn not_implemented(text: &'static str, explanation: &'static str) -> Self {
        Self {
            text,
            activation_text: None,
            definition: DeclarativeAbilityDef::Legacy,
            effect: EffectDef::None,
            implementation: AbilityImplementationDef::NotImplemented { explanation },
        }
    }

    #[must_use]
    pub const fn with_implementation(mut self, implementation: AbilityImplementationDef) -> Self {
        self.implementation = implementation;
        self
    }

    /// Overrides the canonical text supplied by a common ability constructor.
    /// This is reserved for Oracle clauses that include reminder text; the
    /// underlying keyword or mana semantics remain shared.
    #[must_use]
    pub const fn with_text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
    }

    #[must_use]
    pub const fn with_activation_text(
        mut self,
        targeted: &'static str,
        summary: &'static str,
    ) -> Self {
        self.activation_text = Some(ActivatedAbilityText { targeted, summary });
        self
    }

    #[must_use]
    pub const fn with_targets(mut self, targets: &'static [AbilityTargetDef]) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::Spell(definition) => {
                *definition = definition.with_targets(targets);
            }
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => definition.targets = targets,
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => definition.targets = targets,
            DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::Legacy => {}
        }
        self
    }

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Static(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Replacement(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::SpecialAction(definition) => {
                definition.source_zones = source_zones;
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::Legacy => {}
        }
        self
    }

    #[must_use]
    pub const fn uses_stack(self) -> bool {
        matches!(
            self.definition,
            DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
        )
    }

    fn own_implementation_status(self) -> ImplementationStatus {
        match self.implementation {
            AbilityImplementationDef::Definition | AbilityImplementationDef::CustomFull { .. } => {
                ImplementationStatus::Complete
            }
            AbilityImplementationDef::CustomPartial { .. } => ImplementationStatus::Partial,
            AbilityImplementationDef::NotImplemented { .. } => ImplementationStatus::MetadataOnly,
        }
    }

    fn implementation_status(self) -> ImplementationStatus {
        let own = self.own_implementation_status();
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            return own;
        };
        let Some(modal) = spell.modal() else {
            return own;
        };
        if !self.implementation.is_executable() {
            return own;
        }
        let mut statuses = modal
            .modes
            .iter()
            .copied()
            .map(AbilityDef::own_implementation_status);
        let modes = statuses.next().map_or(own, |first| {
            statuses.fold(first, ImplementationStatus::combine)
        });
        if self.implementation == AbilityImplementationDef::Definition
            && self.effect == EffectDef::None
        {
            modes
        } else {
            own.combine(modes)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetSlotDef {
    pub id: TargetSlotId,
    pub label: String,
    pub predicate: TargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
}

impl TargetSlotDef {
    #[must_use]
    pub fn exactly_one(
        id: TargetSlotId,
        label: impl Into<String>,
        predicate: TargetPredicate,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            predicate,
            minimum: 1,
            maximum: 1,
        }
    }
}

fn object_predicate_implies(predicate: ObjectPredicateDef, expected: ObjectPredicateDef) -> bool {
    if predicate == expected {
        return true;
    }
    match predicate {
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .any(|predicate| object_predicate_implies(predicate, expected)),
        ObjectPredicateDef::AnyOf(predicates) => {
            !predicates.is_empty()
                && predicates
                    .iter()
                    .copied()
                    .all(|predicate| object_predicate_implies(predicate, expected))
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

impl AbilityTargetDef {
    pub(super) fn presentation(self) -> Option<TargetSlotDef> {
        let predicate = match self.predicate {
            AbilityTargetPredicate::AnyTarget => TargetPredicate::AnyTarget,
            AbilityTargetPredicate::Player(_) => TargetPredicate::Player,
            AbilityTargetPredicate::Object { object, zones, .. } if zones == [ZoneKind::Stack] => {
                if object_predicate_implies(object, ObjectPredicateDef::NoncreatureSpell) {
                    TargetPredicate::NoncreatureSpell
                } else {
                    TargetPredicate::Spell
                }
            }
            AbilityTargetPredicate::Object { object, zones, .. }
                if zones == [ZoneKind::Battlefield] =>
            {
                if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature))
                {
                    TargetPredicate::CreaturePermanent
                } else {
                    TargetPredicate::Permanent
                }
            }
            AbilityTargetPredicate::Object { .. } => return None,
        };
        Some(TargetSlotDef {
            id: self.id,
            label: self.label.into(),
            predicate,
            minimum: self.minimum,
            maximum: self.maximum,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeDef {
    pub id: ModeId,
    pub label: String,
    pub targets: Vec<TargetSlotDef>,
    pub effect_status: CardEffectStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeSetDef {
    pub minimum: u8,
    pub maximum: u8,
    /// Some cards explicitly allow the same mode to be chosen more than once.
    pub may_repeat: bool,
    pub modes: Vec<ModeDef>,
}

impl ModeSetDef {
    #[must_use]
    pub fn choose_one(modes: Vec<ModeDef>) -> Self {
        Self {
            minimum: 1,
            maximum: 1,
            may_repeat: false,
            modes,
        }
    }
}

impl AbilityDef {
    fn mode_presentation(self, id: ModeId, outer_is_executable: bool) -> Option<ModeDef> {
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            return None;
        };
        if spell.modal().is_some() {
            return None;
        }
        Some(ModeDef {
            id,
            label: self.text.into(),
            targets: spell
                .targets()
                .iter()
                .copied()
                .map(AbilityTargetDef::presentation)
                .collect::<Option<Vec<_>>>()?,
            effect_status: if outer_is_executable && self.implementation.is_executable() {
                CardEffectStatus::Implemented
            } else {
                CardEffectStatus::MetadataOnly
            },
        })
    }
}

/// A named alternative to the cost supplied by a play option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlternativeCostDef {
    pub id: AlternativeCostId,
    pub label: String,
    pub mana_cost: ManaCost,
}

/// A named additional cost. Some additional costs are nonmana costs, so the
/// mana component is optional and the authoritative rules remain in `label`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdditionalCostDef {
    pub id: AdditionalCostId,
    pub label: String,
    pub mana_cost: Option<ManaCost>,
}

/// One legal way to play a card. This is distinct from rules-text modes and
/// from alternative/additional cost choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayOptionDef {
    pub id: PlayOptionId,
    pub label: String,
    pub action: PlayActionKind,
    pub form: SpellForm,
    pub mana_cost: Option<ManaCost>,
    pub restriction: PlayRestriction,
    pub modes: Option<ModeSetDef>,
    pub targets: Vec<TargetSlotDef>,
    pub alternative_costs: Vec<AlternativeCostDef>,
    pub additional_costs: Vec<AdditionalCostDef>,
    pub effect_status: CardEffectStatus,
}

impl PlayOptionDef {
    #[must_use]
    pub fn cast(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        mana_cost: ManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self::cast_with_printed_mana_cost(
            id,
            label,
            form,
            PrintedManaCost::Cost(mana_cost),
            effect_status,
        )
    }

    /// Defines a cast action without collapsing a nonexistent printed cost
    /// into `{0}`. A spell with `PrintedManaCost::None` ordinarily needs a
    /// separate casting permission or alternative cost before it is legal.
    #[must_use]
    pub fn cast_with_printed_mana_cost(
        id: PlayOptionId,
        label: impl Into<String>,
        form: SpellForm,
        printed_mana_cost: PrintedManaCost,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::CastSpell,
            form,
            mana_cost: printed_mana_cost.as_option(),
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn play_land(
        id: PlayOptionId,
        label: impl Into<String>,
        part: CardPartId,
        effect_status: CardEffectStatus,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            action: PlayActionKind::PlayLand,
            form: SpellForm::Part(part),
            mana_cost: None,
            restriction: PlayRestriction::Normal,
            modes: None,
            targets: Vec::new(),
            alternative_costs: Vec::new(),
            additional_costs: Vec::new(),
            effect_status,
        }
    }

    #[must_use]
    pub fn with_targets(mut self, targets: Vec<TargetSlotDef>) -> Self {
        self.targets = targets;
        self
    }

    #[must_use]
    pub fn with_modes(mut self, modes: ModeSetDef) -> Self {
        self.modes = Some(modes);
        self
    }

    #[must_use]
    pub const fn restricted_to_hand(mut self) -> Self {
        self.restriction = PlayRestriction::FromHandOnly;
        self
    }
}

/// The structured portion of a card definition supplied by a set record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardComposition {
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
}

impl CardComposition {
    #[must_use]
    pub fn single(name: impl Into<String>, rules: CardRules) -> Self {
        let printed_mana_cost = rules.printed_mana_cost;
        let name = name.into();
        let is_land = rules.has_type(CardType::Land);
        let effect_status = match rules.implementation_status() {
            ImplementationStatus::MetadataOnly => CardEffectStatus::MetadataOnly,
            ImplementationStatus::Complete | ImplementationStatus::Partial => {
                CardEffectStatus::Implemented
            }
        };
        let name_for_flashback = name.clone();
        let part = CardPart::new(CardPartId::PRIMARY, name.clone(), rules);
        let mut option = if is_land {
            PlayOptionDef::play_land(
                PlayOptionId::DEFAULT,
                name,
                CardPartId::PRIMARY,
                effect_status,
            )
        } else {
            PlayOptionDef::cast_with_printed_mana_cost(
                PlayOptionId::DEFAULT,
                name,
                SpellForm::Part(CardPartId::PRIMARY),
                printed_mana_cost,
                effect_status,
            )
        };
        if let Some(modes) = rules.presentation_spell_modes() {
            option = option.with_modes(modes);
        }
        let mut play_options = vec![option];
        if let Some(cost) = rules.flashback_cost() {
            let mut flashback = PlayOptionDef::cast(
                PlayOptionId(1),
                format!("{name_for_flashback} with flashback"),
                SpellForm::Part(CardPartId::PRIMARY),
                cost,
                effect_status,
            );
            flashback.restriction = PlayRestriction::Flashback;
            if let Some(modes) = rules.presentation_spell_modes() {
                flashback = flashback.with_modes(modes);
            }
            play_options.push(flashback);
        }
        Self {
            parts: vec![part],
            structure: CardStructure::Single {
                main: CardPartId::PRIMARY,
            },
            play_options,
        }
    }
}

/// Canonical artwork metadata used when no exact printing is selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardArt {
    pub scryfall_id: &'static str,
    pub artist: &'static str,
}

impl CardArt {
    #[must_use]
    pub const fn new(scryfall_id: &'static str, artist: &'static str) -> Self {
        Self {
            scryfall_id,
            artist,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardDefinition {
    pub id: CardDefinitionId,
    pub name: String,
    pub art: Option<CardArt>,
    /// The canonical record's debut set within this catalog.
    ///
    /// Rules that care where a card debuted, such as City in a Bottle, use
    /// this field. Format legality instead considers every known `printing`.
    pub debut_set: CardSet,
    pub printings: Vec<CardPrinting>,
    /// Compatibility view of the primary/front part. Contextual rules should
    /// use `parts` once the game engine is part-aware.
    pub rules: CardRules,
    pub parts: Vec<CardPart>,
    pub structure: CardStructure,
    pub play_options: Vec<PlayOptionDef>,
}

impl CardDefinition {
    /// Creates a definition using the built-in metadata for `behavior`.
    #[must_use]
    pub fn new(
        id: CardDefinitionId,
        name: impl Into<String>,
        debut_set: CardSet,
        is_basic_land: bool,
        behavior: CardBehavior,
    ) -> Self {
        let name = name.into();
        let rules = if is_basic_land {
            (*behavior.rules()).with_supertype(CardSupertype::Basic)
        } else {
            *behavior.rules()
        };
        let composition = CardComposition::single(name.clone(), rules);
        Self {
            id,
            name,
            art: None,
            debut_set,
            printings: vec![CardPrinting::new(id, debut_set)],
            rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    #[must_use]
    pub const fn is_basic_land(&self) -> bool {
        self.rules.has_type(CardType::Land) && self.rules.has_supertype(CardSupertype::Basic)
    }

    #[must_use]
    pub fn part(&self, id: CardPartId) -> Option<&CardPart> {
        self.parts.iter().find(|part| part.id == id)
    }

    #[must_use]
    pub fn play_option(&self, id: PlayOptionId) -> Option<&PlayOptionDef> {
        self.play_options.iter().find(|option| option.id == id)
    }

    /// Derives card-level coverage from every ordered clause on every part.
    /// A mix of complete and unimplemented parts is partial; a card is
    /// metadata-only only when every represented clause is unimplemented.
    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        let mut statuses = self
            .parts
            .iter()
            .map(|part| part.rules.implementation_status());
        statuses
            .next()
            .map_or(ImplementationStatus::Complete, |first| {
                statuses.fold(first, ImplementationStatus::combine)
            })
    }

    #[must_use]
    pub fn primary_part_id(&self) -> CardPartId {
        match &self.structure {
            CardStructure::Single { main } | CardStructure::AlternateSpell { main, .. } => *main,
            CardStructure::Split { parts, .. } => {
                parts.first().copied().unwrap_or(CardPartId::PRIMARY)
            }
            CardStructure::Flip { normal, .. } => *normal,
            CardStructure::DoubleFaced { front, .. } | CardStructure::MeldPart { front, .. } => {
                *front
            }
        }
    }

    #[must_use]
    pub fn primary_part(&self) -> Option<&CardPart> {
        self.part(self.primary_part_id())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardBehavior {
    ArgothianPixies,
    Atog,
    AugurOfBolas,
    Balance,
    Berserk,
    BlackVise,
    BloodBaronOfVizkopa,
    /// Legacy dispatch key retained for source compatibility; the card now
    /// uses a declarative choose-one spell definition.
    BlueElementalBlast,
    BloodMoon,
    ChainLightning,
    Channel,
    ChaosOrb,
    CityInABottle,
    CopyArtifact,
    Crusade,
    DemonicTutor,
    Detonate,
    DivineOffering,
    Dispel,
    Dissipate,
    DoomBlade,
    DrainLife,
    DragonWhelp,
    DustToDust,
    Duress,
    Earthquake,
    ErhnamDjinn,
    EssenceScatter,
    Fireball,
    Fork,
    GiantGrowth,
    GlassesOfUrza,
    GoblinGrenade,
    GrislySalvage,
    HurkylsRecall,
    HymnToTourach,
    HypnoticSpecter,
    IcyManipulator,
    IcatianJavelineers,
    IronclawOrcs,
    Juggernaut,
    KirdApe,
    LifebaneZombie,
    LibraryOfAlexandria,
    ManaDrain,
    ManaVault,
    MazeOfIth,
    MindTwist,
    Moat,
    Mulch,
    Negate,
    NevinyrralsDisk,
    Pendelhaven,
    PillarOfFlame,
    Putrefy,
    Recall,
    Regrowth,
    SedgeTroll,
    SinCollector,
    SylvanLibrary,
    Terror,
    TimeVault,
    Timetwister,
    FellwarStone,
    LightningBolt,
    MishrasFactory,
    OrcishMechanics,
    /// Legacy dispatch key retained for source compatibility; the card now
    /// uses a declarative choose-one spell definition.
    RedElementalBlast,
    Smoke,
    SphinxsRevelation,
    StoneGiant,
    /// Legacy dispatch key retained for source compatibility; the card now
    /// uses a declarative creature-sweeper definition.
    SupremeVerdict,
    SwordsToPlowshares,
    TimeWalk,
    Triskelion,
    Tetravus,
    TheAbyss,
    UltimatePrice,
    WarleadersHelix,
    WheelOfFortune,
    WhirlingDervish,
    WinterOrb,
    // Compatibility rules keys retained while CardDefinition::new still
    // accepts CardBehavior instead of CardRules directly.
    Mountain,
    Plains,
    Unsupported,
}

/// A kind of counter a permanent can carry. Only `PlusOnePlusOne` has rules
/// meaning of its own; the rest are named markers that the cards putting them
/// there give meaning to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CounterKind {
    PlusOnePlusOne,
    Javelin,
    Muster,
    Charge,
}

impl CounterKind {
    pub const COUNT: usize = 4;

    pub const ALL: [Self; Self::COUNT] = [
        Self::PlusOnePlusOne,
        Self::Javelin,
        Self::Muster,
        Self::Charge,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::PlusOnePlusOne => 0,
            Self::Javelin => 1,
            Self::Muster => 2,
            Self::Charge => 3,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PlusOnePlusOne => "+1/+1",
            Self::Javelin => "javelin",
            Self::Muster => "muster",
            Self::Charge => "charge",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
    World,
}

impl CardSupertype {
    pub const COUNT: usize = 4;

    pub const ALL: [Self; Self::COUNT] = [Self::Basic, Self::Legendary, Self::Snow, Self::World];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Legendary => 1,
            Self::Snow => 2,
            Self::World => 3,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Legendary => "Legendary",
            Self::Snow => "Snow",
            Self::World => "World",
        }
    }
}

/// How completely the engine implements a card or independently modeled part.
///
/// Ordinary construction defaults to [`Self::Complete`]. Explanations live on
/// the non-declarative clause implementations that caused a non-complete
/// aggregate status, rather than being duplicated at card level.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImplementationStatus {
    #[default]
    Complete,
    Partial,
    MetadataOnly,
}

impl ImplementationStatus {
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Partial, _)
            | (_, Self::Partial)
            | (Self::Complete, Self::MetadataOnly)
            | (Self::MetadataOnly, Self::Complete) => Self::Partial,
            (Self::MetadataOnly, Self::MetadataOnly) => Self::MetadataOnly,
            (Self::Complete, Self::Complete) => Self::Complete,
        }
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Whether the current game engine may execute an independently modeled play
/// option or mode. Ordinary single-card options derive this gate from their
/// clause-local [`ImplementationStatus`] instead of storing another status on
/// [`CardRules`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardEffectStatus {
    Implemented,
    MetadataOnly,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaCost {
    pub generic: u16,
    pub white: u16,
    pub blue: u16,
    pub black: u16,
    pub red: u16,
    pub green: u16,
    /// Number of `{R/W}` hybrid symbols in this cost.
    pub white_red_hybrid: u16,
    pub variable_x: bool,
    pub x_multiplier: u16,
}

/// Why a symbolic mana-cost string could not be represented by [`ManaCost`].
///
/// Penta accepts the canonical braced notation used by Oracle, such as
/// `{2}{G}{G}` or `{X}{R}`. Symbols outside the engine's current mana model
/// are rejected instead of being approximated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaCostParseError {
    pub offset: usize,
    pub kind: ManaCostParseErrorKind,
}

impl ManaCostParseError {
    const fn new(offset: usize, kind: ManaCostParseErrorKind) -> Self {
        Self { offset, kind }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaCostParseErrorKind {
    Empty,
    ExpectedOpeningBrace,
    UnterminatedSymbol,
    EmptySymbol,
    InvalidSymbol,
    DuplicateGenericSymbol,
    Overflow,
}

impl fmt::Display for ManaCostParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let problem = match self.kind {
            ManaCostParseErrorKind::Empty => "a mana cost cannot be empty",
            ManaCostParseErrorKind::ExpectedOpeningBrace => {
                "each mana symbol must start with an opening brace"
            }
            ManaCostParseErrorKind::UnterminatedSymbol => {
                "a mana symbol is missing its closing brace"
            }
            ManaCostParseErrorKind::EmptySymbol => "a mana symbol cannot be empty",
            ManaCostParseErrorKind::InvalidSymbol => {
                "the mana symbol is invalid or unsupported by the current engine"
            }
            ManaCostParseErrorKind::DuplicateGenericSymbol => {
                "a mana cost may contain only one numeric generic symbol"
            }
            ManaCostParseErrorKind::Overflow => "the mana cost exceeds the supported numeric range",
        };
        write!(formatter, "{problem} at byte {}", self.offset)
    }
}

impl Error for ManaCostParseError {}

impl ManaCost {
    /// Parses canonical braced mana symbols without allocating.
    ///
    /// This is `const` so [`crate::mana_cost!`] can validate literals during
    /// compilation. Runtime callers will usually prefer `str::parse`, which
    /// uses the same parser through [`FromStr`]. An empty string is invalid:
    /// a card with no mana cost is represented by [`PrintedManaCost::None`],
    /// while `{0}` is a real, payable printed cost.
    ///
    /// # Errors
    ///
    /// Returns a [`ManaCostParseError`] at the first malformed or currently
    /// unsupported symbol, duplicate numeric symbol, or numeric overflow.
    #[allow(clippy::too_many_lines)]
    pub const fn parse_symbols(symbols: &str) -> Result<Self, ManaCostParseError> {
        let bytes = symbols.as_bytes();
        if bytes.is_empty() {
            return Err(ManaCostParseError::new(0, ManaCostParseErrorKind::Empty));
        }

        let mut cost = Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            white_red_hybrid: 0,
            variable_x: false,
            x_multiplier: 0,
        };
        let mut offset = 0;
        let mut saw_generic = false;

        while offset < bytes.len() {
            if bytes[offset] != b'{' {
                return Err(ManaCostParseError::new(
                    offset,
                    ManaCostParseErrorKind::ExpectedOpeningBrace,
                ));
            }
            let symbol_start = offset + 1;
            let mut symbol_end = symbol_start;
            while symbol_end < bytes.len() && bytes[symbol_end] != b'}' {
                symbol_end += 1;
            }
            if symbol_end == bytes.len() {
                return Err(ManaCostParseError::new(
                    offset,
                    ManaCostParseErrorKind::UnterminatedSymbol,
                ));
            }
            if symbol_end == symbol_start {
                return Err(ManaCostParseError::new(
                    symbol_start,
                    ManaCostParseErrorKind::EmptySymbol,
                ));
            }

            let symbol_len = symbol_end - symbol_start;
            if symbol_len == 1 {
                let parsed = match bytes[symbol_start] {
                    b'W' => Self::checked_increment(cost.white),
                    b'U' => Self::checked_increment(cost.blue),
                    b'B' => Self::checked_increment(cost.black),
                    b'R' => Self::checked_increment(cost.red),
                    b'G' => Self::checked_increment(cost.green),
                    b'X' => Self::checked_increment(cost.x_multiplier),
                    b'0'..=b'9' => {
                        if saw_generic {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::DuplicateGenericSymbol,
                            ));
                        }
                        saw_generic = true;
                        Ok((bytes[symbol_start] - b'0') as u16)
                    }
                    _ => Err(ManaCostParseErrorKind::InvalidSymbol),
                };
                let value = match parsed {
                    Ok(value) => value,
                    Err(kind) => return Err(ManaCostParseError::new(symbol_start, kind)),
                };
                match bytes[symbol_start] {
                    b'W' => cost.white = value,
                    b'U' => cost.blue = value,
                    b'B' => cost.black = value,
                    b'R' => cost.red = value,
                    b'G' => cost.green = value,
                    b'X' => {
                        cost.variable_x = true;
                        cost.x_multiplier = value;
                    }
                    b'0'..=b'9' => cost.generic = value,
                    _ => {}
                }
            } else if symbol_len == 3
                && bytes[symbol_start] == b'R'
                && bytes[symbol_start + 1] == b'/'
                && bytes[symbol_start + 2] == b'W'
            {
                cost.white_red_hybrid = match Self::checked_increment(cost.white_red_hybrid) {
                    Ok(value) => value,
                    Err(kind) => return Err(ManaCostParseError::new(symbol_start, kind)),
                };
            } else {
                let first = bytes[symbol_start];
                if !first.is_ascii_digit() {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::InvalidSymbol,
                    ));
                }
                if saw_generic {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::DuplicateGenericSymbol,
                    ));
                }
                if first == b'0' {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::InvalidSymbol,
                    ));
                }
                let mut value = 0_u16;
                let mut digit = symbol_start;
                while digit < symbol_end {
                    let byte = bytes[digit];
                    if !byte.is_ascii_digit() {
                        return Err(ManaCostParseError::new(
                            digit,
                            ManaCostParseErrorKind::InvalidSymbol,
                        ));
                    }
                    value = match value.checked_mul(10) {
                        Some(value) => value,
                        None => {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::Overflow,
                            ));
                        }
                    };
                    value = match value.checked_add((byte - b'0') as u16) {
                        Some(value) => value,
                        None => {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::Overflow,
                            ));
                        }
                    };
                    digit += 1;
                }
                cost.generic = value;
                saw_generic = true;
            }

            offset = symbol_end + 1;
        }

        Ok(cost)
    }

    const fn checked_increment(value: u16) -> Result<u16, ManaCostParseErrorKind> {
        match value.checked_add(1) {
            Some(value) => Ok(value),
            None => Err(ManaCostParseErrorKind::Overflow),
        }
    }

    /// Mana value with each `{X}` treated as zero.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        self.generic
            .saturating_add(self.white)
            .saturating_add(self.blue)
            .saturating_add(self.black)
            .saturating_add(self.red)
            .saturating_add(self.green)
            .saturating_add(self.white_red_hybrid)
    }

    #[must_use]
    pub const fn new(generic: u16, red: u16) -> Self {
        Self {
            generic,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            white_red_hybrid: 0,
            variable_x: false,
            x_multiplier: 0,
        }
    }

    #[must_use]
    pub const fn colored(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            white_red_hybrid: 0,
            variable_x: false,
            x_multiplier: 0,
        }
    }

    #[must_use]
    pub const fn with_x(red: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            white_red_hybrid: 0,
            variable_x: true,
            x_multiplier: 1,
        }
    }

    #[must_use]
    pub const fn colored_x(white: u16, blue: u16, black: u16, red: u16, green: u16) -> Self {
        Self {
            generic: 0,
            white,
            blue,
            black,
            red,
            green,
            white_red_hybrid: 0,
            variable_x: true,
            x_multiplier: 1,
        }
    }

    #[must_use]
    pub const fn variable(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
        x_multiplier: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            white_red_hybrid: 0,
            variable_x: true,
            x_multiplier,
        }
    }

    #[must_use]
    pub const fn white_red_hybrid(count: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            white_red_hybrid: count,
            variable_x: false,
            x_multiplier: 0,
        }
    }
}

impl FromStr for ManaCost {
    type Err = ManaCostParseError;

    fn from_str(symbols: &str) -> Result<Self, Self::Err> {
        Self::parse_symbols(symbols)
    }
}

/// Builds a [`ManaCost`] from canonical braced symbols and validates the
/// literal at compile time.
///
/// ```
/// # use penta::{ManaCost, mana_cost};
/// const COST: ManaCost = mana_cost!("{2}{G}{G}");
/// assert_eq!(COST.generic, 2);
/// assert_eq!(COST.green, 2);
/// ```
///
/// ```compile_fail
/// # use penta::{ManaCost, mana_cost};
/// const COST: ManaCost = mana_cost!("2GG");
/// ```
#[macro_export]
macro_rules! mana_cost {
    ($symbols:literal) => {{
        const COST: $crate::ManaCost = match $crate::ManaCost::parse_symbols($symbols) {
            Ok(cost) => cost,
            Err(_) => panic!("invalid mana cost literal"),
        };
        COST
    }};
}

/// How a land enters the battlefield before replacement effects are applied.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LandEntry {
    Untapped,
    Tapped,
    TappedUnlessControlsLandType([bool; 5]),
    PayLifeOrTapped(u8),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatureStats {
    pub power: i16,
    pub toughness: i16,
}

/// How a client should describe activating a permanent's targeted ability.
///
/// `targeted` is a template with `{}` where the target's name goes, so a menu
/// can name the effect instead of the card; `summary` is the same effect with
/// no particular target picked yet.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityText {
    pub targeted: &'static str,
    pub summary: &'static str,
}

/// Const-friendly storage for the ordered rules clauses of one card part.
///
/// A card with one clause stores it inline; cards with several clauses use a
/// promoted static slice, preserving source order without heap allocation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum CardAbilityList {
    None,
    One(AbilityDef),
    Many(&'static [AbilityDef]),
}

/// One reusable ability definition attached to a card part at a stable
/// position. The attachment supplies identity; [`AbilityDef`] supplies only
/// rules text and semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AttachedAbilityDef {
    pub id: AbilityId,
    pub definition: AbilityDef,
}

impl CardAbilityList {
    #[must_use]
    pub fn as_slice(&self) -> &[AbilityDef] {
        match self {
            Self::None => &[],
            Self::One(ability) => std::slice::from_ref(ability),
            Self::Many(abilities) => abilities,
        }
    }
}

/// Declarative rules metadata kept beside a card's catalog identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct CardRules {
    card_types: CardTypeSet,
    supertypes: [bool; CardSupertype::COUNT],
    subtypes: &'static [&'static str],
    printed_mana_cost: PrintedManaCost,
    land_entry: LandEntry,
    starting_loyalty: Option<u16>,
    creature_stats: Option<CreatureStats>,
    /// Ordered printed rules clauses. Abilities supplied by the rules, such as
    /// those intrinsic to basic land types, are derived by the game engine.
    abilities: CardAbilityList,
    colors: ColorSet,
    /// The cost this card can be cast for from its owner's graveyard. The
    /// printed clause still carries the reminder text; this is what gives
    /// casting a second play option to offer.
    flashback: Option<ManaCost>,
}

impl CardRules {
    const fn base(card_types: CardTypeSet, printed_mana_cost: PrintedManaCost) -> Self {
        let mana_cost = match printed_mana_cost {
            PrintedManaCost::None => ManaCost::new(0, 0),
            PrintedManaCost::Cost(cost) => cost,
        };
        let mut colors = ColorSet::empty();
        if mana_cost.white > 0 || mana_cost.white_red_hybrid > 0 {
            colors = colors.with(ManaColor::White);
        }
        if mana_cost.blue > 0 {
            colors = colors.with(ManaColor::Blue);
        }
        if mana_cost.black > 0 {
            colors = colors.with(ManaColor::Black);
        }
        if mana_cost.red > 0 || mana_cost.white_red_hybrid > 0 {
            colors = colors.with(ManaColor::Red);
        }
        if mana_cost.green > 0 {
            colors = colors.with(ManaColor::Green);
        }
        Self {
            card_types,
            supertypes: [false; CardSupertype::COUNT],
            subtypes: &[],
            printed_mana_cost,
            land_entry: LandEntry::Untapped,
            starting_loyalty: None,
            creature_stats: None,
            abilities: CardAbilityList::None,
            colors,
            flashback: None,
        }
    }

    /// Declares the flashback cost the printed clause names. The clause itself
    /// stays in the ability list so the reminder text is still cataloged.
    #[must_use]
    pub const fn with_flashback(mut self, cost: ManaCost) -> Self {
        self.flashback = Some(cost);
        self
    }

    #[must_use]
    pub const fn flashback_cost(&self) -> Option<ManaCost> {
        self.flashback
    }

    #[must_use]
    pub const fn new_creature(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Creature),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_creature_without_mana_cost(
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Creature),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_artifact_creature(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_artifact_creature_without_mana_cost(
        subtypes: &'static [&'static str],
        power: i16,
        toughness: i16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules.creature_stats = Some(CreatureStats { power, toughness });
        rules
    }

    #[must_use]
    pub const fn new_land(subtypes: &'static [&'static str]) -> Self {
        let mut rules = Self::base(CardTypeSet::single(CardType::Land), PrintedManaCost::None);
        rules.subtypes = subtypes;
        rules
    }

    #[must_use]
    pub const fn new_artifact(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_enchantment(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Enchantment),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_instant(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Instant),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_instant_without_mana_cost() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Instant),
            PrintedManaCost::None,
        )
    }

    #[must_use]
    pub const fn new_sorcery(mana_cost: ManaCost) -> Self {
        Self::base(
            CardTypeSet::single(CardType::Sorcery),
            PrintedManaCost::Cost(mana_cost),
        )
    }

    #[must_use]
    pub const fn new_sorcery_without_mana_cost() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Sorcery),
            PrintedManaCost::None,
        )
    }

    #[must_use]
    pub const fn new_planeswalker(
        mana_cost: ManaCost,
        subtypes: &'static [&'static str],
        starting_loyalty: u16,
    ) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Planeswalker),
            PrintedManaCost::Cost(mana_cost),
        );
        rules.subtypes = subtypes;
        rules.starting_loyalty = Some(starting_loyalty);
        rules
    }

    /// Creates a planeswalker back face, which has neither a printed mana cost
    /// nor a printed starting-loyalty value.
    #[must_use]
    pub const fn new_planeswalker_without_mana_cost(subtypes: &'static [&'static str]) -> Self {
        let mut rules = Self::base(
            CardTypeSet::single(CardType::Planeswalker),
            PrintedManaCost::None,
        );
        rules.subtypes = subtypes;
        rules
    }

    #[must_use]
    pub const fn types(&self) -> CardTypeSet {
        self.card_types
    }

    #[must_use]
    pub const fn has_type(&self, card_type: CardType) -> bool {
        self.card_types.contains(card_type)
    }

    /// Compatibility spelling for clients that still expose one `kind`
    /// string instead of a collection of card types.
    #[must_use]
    pub fn kind_name(&self) -> String {
        self.card_types.kind_name()
    }

    #[must_use]
    pub const fn subtypes(&self) -> &'static [&'static str] {
        self.subtypes
    }

    #[must_use]
    pub const fn printed_mana_cost(&self) -> PrintedManaCost {
        self.printed_mana_cost
    }

    #[must_use]
    pub const fn mana_cost(&self) -> Option<ManaCost> {
        self.printed_mana_cost.as_option()
    }

    #[must_use]
    pub const fn land_entry_procedure(&self) -> LandEntry {
        self.land_entry
    }

    #[must_use]
    pub const fn starting_loyalty(&self) -> Option<u16> {
        self.starting_loyalty
    }

    #[must_use]
    pub const fn creature_stats(&self) -> Option<CreatureStats> {
        self.creature_stats
    }

    #[must_use]
    pub const fn colors(&self) -> [bool; 5] {
        self.colors.to_flags()
    }

    #[must_use]
    pub const fn color_set(&self) -> ColorSet {
        self.colors
    }

    #[must_use]
    pub const fn has_color(&self, color: ManaColor) -> bool {
        self.colors.contains(color)
    }

    /// Returns a concise explanation when internal or compatibility code has
    /// bypassed the type-specific constructors and produced contradictory
    /// characteristics.
    #[must_use]
    pub(super) const fn coherence_error(&self) -> Option<&'static str> {
        if self.card_types.is_empty() {
            return Some("a card part must have at least one card type");
        }
        let instant = self.has_type(CardType::Instant);
        let sorcery = self.has_type(CardType::Sorcery);
        if instant && sorcery {
            return Some("one card part cannot be both an instant and a sorcery");
        }
        if (instant || sorcery) && self.card_types.is_permanent() {
            return Some("an instant or sorcery cannot also be a permanent card type");
        }
        if self.has_type(CardType::Land) && !matches!(self.printed_mana_cost, PrintedManaCost::None)
        {
            return Some("a land cannot have a printed mana cost");
        }
        if self.has_type(CardType::Creature) && self.creature_stats.is_none() {
            return Some("a creature must have power and toughness");
        }
        if !self.has_type(CardType::Creature) && self.creature_stats.is_some() {
            return Some("a noncreature cannot have creature power and toughness");
        }
        if !self.has_type(CardType::Planeswalker) && self.starting_loyalty.is_some() {
            return Some("a nonplaneswalker cannot have starting loyalty");
        }
        if self.has_type(CardType::Planeswalker)
            && matches!(self.printed_mana_cost, PrintedManaCost::Cost(_))
            && self.starting_loyalty.is_none()
        {
            return Some("a castable planeswalker face must have starting loyalty");
        }
        if !self.has_type(CardType::Land) && !matches!(self.land_entry, LandEntry::Untapped) {
            return Some("nonland rules cannot declare a land-entry procedure");
        }
        None
    }

    #[cfg(test)]
    pub(super) const fn with_printed_mana_cost_for_test(
        mut self,
        printed_mana_cost: PrintedManaCost,
    ) -> Self {
        self.printed_mana_cost = printed_mana_cost;
        self
    }

    #[must_use]
    pub const fn with_ability(mut self, ability: AbilityDef) -> Self {
        self.abilities = CardAbilityList::One(ability);
        self
    }

    #[must_use]
    pub const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = if abilities.is_empty() {
            CardAbilityList::None
        } else {
            CardAbilityList::Many(abilities)
        };
        self
    }

    #[must_use]
    pub fn special_behavior(&self) -> Option<CardBehavior> {
        self.ability_clauses()
            .iter()
            .find_map(|ability| ability.implementation.custom_behavior())
    }

    #[must_use]
    pub fn ability_clauses(&self) -> &[AbilityDef] {
        self.abilities.as_slice()
    }

    fn presentation_spell_modes(&self) -> Option<ModeSetDef> {
        let mut spell_abilities = self.ability_clauses().iter().filter_map(|ability| {
            let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                return None;
            };
            Some((ability, spell))
        });
        let (ability, spell) = spell_abilities.next()?;
        let modal = spell.modal()?;
        if spell_abilities.next().is_some() {
            return None;
        }
        let modes = modal
            .modes
            .iter()
            .copied()
            .enumerate()
            .map(|(index, mode)| {
                mode.mode_presentation(
                    ModeId::from_index(index)?,
                    ability.implementation.is_executable(),
                )
            })
            .collect::<Option<Vec<_>>>()?;
        Some(ModeSetDef {
            minimum: modal.minimum,
            maximum: modal.maximum,
            may_repeat: modal.may_repeat,
            modes,
        })
    }

    /// Iterates the ordered ability definitions with the positional identity
    /// they receive when attached to this card part.
    ///
    /// # Panics
    ///
    /// Panics when a rule set contains more than 256 clauses. Catalog
    /// validation rejects such a definition before it can enter a game.
    #[must_use]
    pub fn indexed_abilities(&self) -> impl ExactSizeIterator<Item = AttachedAbilityDef> + '_ {
        self.ability_clauses()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, definition)| AttachedAbilityDef {
                id: AbilityId::from_index(index)
                    .expect("validated card parts contain at most 256 abilities"),
                definition,
            })
    }

    /// Looks up one attached ability by its positional identity.
    #[must_use]
    pub fn ability(&self, id: AbilityId) -> Option<&AbilityDef> {
        self.ability_clauses().get(id.index())
    }

    /// Renders the ordered card text from the same clauses used by execution
    /// and implementation auditing.
    #[must_use]
    pub fn rules_text(&self) -> std::borrow::Cow<'static, str> {
        match self.abilities {
            CardAbilityList::None => std::borrow::Cow::Borrowed(""),
            CardAbilityList::One(ability) => std::borrow::Cow::Borrowed(ability.text),
            CardAbilityList::Many(abilities) => std::borrow::Cow::Owned(
                abilities
                    .iter()
                    .map(|ability| ability.text)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }

    #[must_use]
    pub fn implementation_status(&self) -> ImplementationStatus {
        // Playing a land and casting/using a modeled creature body are shared,
        // executable rules even when every card-specific clause is deferred.
        let mut has_full = self.has_type(CardType::Land) || self.creature_stats.is_some();
        let mut has_partial = false;
        let mut has_unimplemented = false;
        for ability in self.ability_clauses() {
            match ability.implementation_status() {
                ImplementationStatus::Complete => has_full = true,
                ImplementationStatus::Partial => has_partial = true,
                ImplementationStatus::MetadataOnly => has_unimplemented = true,
            }
        }
        if has_partial || (has_full && has_unimplemented) {
            ImplementationStatus::Partial
        } else if has_unimplemented {
            ImplementationStatus::MetadataOnly
        } else {
            ImplementationStatus::Complete
        }
    }

    #[must_use]
    pub const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.supertypes[supertype.index()] = true;
        self
    }

    #[must_use]
    pub const fn with_subtypes(mut self, subtypes: &'static [&'static str]) -> Self {
        self.subtypes = subtypes;
        self
    }

    #[must_use]
    pub const fn has_supertype(&self, supertype: CardSupertype) -> bool {
        self.supertypes[supertype.index()]
    }

    #[must_use]
    pub fn has_subtype(&self, subtype: &str) -> bool {
        self.subtypes.contains(&subtype)
    }

    #[must_use]
    pub fn type_line(&self) -> String {
        let mut words = [
            CardSupertype::Basic,
            CardSupertype::Legendary,
            CardSupertype::Snow,
            CardSupertype::World,
        ]
        .into_iter()
        .filter(|supertype| self.has_supertype(*supertype))
        .map(CardSupertype::name)
        .collect::<Vec<_>>();
        let type_name = self.card_types.type_name();
        words.push(&type_name);
        let mut line = words.join(" ");
        if !self.subtypes.is_empty() {
            line.push_str(" — ");
            line.push_str(&self.subtypes.join(" "));
        }
        line
    }

    /// Overrides colors supplied by a color indicator or another printed
    /// characteristic that cannot be derived from the mana cost.
    #[must_use]
    pub const fn printed_colors(mut self, colors: &'static [ManaColor]) -> Self {
        self.colors = ColorSet::from_colors(colors);
        self
    }

    #[must_use]
    pub const fn with_type(mut self, card_type: CardType) -> Self {
        self.card_types = self.card_types.with(card_type);
        self
    }

    /// Supplies the printed power and toughness after a definition has been
    /// assembled with the creature card type.
    ///
    /// # Panics
    ///
    /// Panics when called on rules without the creature card type.
    #[must_use]
    pub const fn with_creature_stats(mut self, stats: CreatureStats) -> Self {
        assert!(
            self.has_type(CardType::Creature),
            "with_creature_stats() is only valid for creature rules"
        );
        self.creature_stats = Some(stats);
        self
    }

    /// Declares the procedure used when these land rules enter the battlefield.
    ///
    /// # Panics
    ///
    /// Panics when called on rules without the land card type.
    #[must_use]
    pub const fn land_entry(mut self, land_entry: LandEntry) -> Self {
        assert!(
            self.has_type(CardType::Land),
            "land_entry() is only valid for land rules"
        );
        self.land_entry = land_entry;
        self
    }

    /// Whether the printed clauses declare this keyword, regardless of its
    /// current implementation coverage.
    #[must_use]
    pub fn has_keyword(&self, expected: KeywordAbility) -> bool {
        self.ability_clauses().iter().any(
            |ability| matches!(ability.definition, DeclarativeAbilityDef::Keyword(actual) if actual == expected),
        )
    }

    /// Whether the card declares this keyword and the engine executes it.
    #[must_use]
    pub fn has_executable_keyword(&self, expected: KeywordAbility) -> bool {
        self.ability_clauses().iter().any(|ability| {
            ability.implementation.is_executable()
                && matches!(ability.definition, DeclarativeAbilityDef::Keyword(actual) if actual == expected)
        })
    }

    pub(super) const fn unsupported() -> Self {
        Self::base(
            CardTypeSet::single(CardType::Artifact),
            PrintedManaCost::None,
        )
        .with_ability(AbilityDef::not_implemented(
            "Rules text is not implemented.",
            "The card's printed rules have not been cataloged or implemented.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, CardBehavior,
        CardComposition, CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId,
        CardRules, CardSet, CardType, CardTypeSet, CreatureStats, DeclarativeAbilityDef, EffectDef,
        EffectRecipientDef, ImplementationStatus, LandEntry, ManaColor, ManaCost,
        ManaCostParseErrorKind, ManaRestrictionDef, ObjectPredicateDef, PrintedManaCost,
        TargetPredicate, TriggerEventDef,
    };
    use crate::{AbilityId, CardDefinitionId, CardPartId, ModeId, TargetSlotId};

    static DEFERRED_CLAUSE: [AbilityDef; 1] = [AbilityDef::not_implemented(
        "A deferred card-specific ability.",
        "The card-specific ability is not executed.",
    )];

    #[test]
    fn modal_spell_semantics_derive_their_presentation_modes() {
        const RULES: CardRules = CardRules::new_instant(crate::mana_cost!("{0}")).with_ability(
            AbilityDef::choose_one_spell(
                "Choose one.",
                &[
                    AbilityDef::counter_target(
                        "Counter target blue spell",
                        &AbilityTargetDef::exactly_one_spell(
                            TargetSlotId(3),
                            "blue spell",
                            ObjectPredicateDef::Color(ManaColor::Blue),
                        ),
                    ),
                    AbilityDef::destroy_target(
                        "Destroy target blue permanent",
                        &AbilityTargetDef::exactly_one_permanent(
                            TargetSlotId(4),
                            "blue permanent",
                            ObjectPredicateDef::Color(ManaColor::Blue),
                        ),
                        true,
                    ),
                ],
            ),
        );
        let rules = RULES;
        let composition = CardComposition::single("Test Modal Spell", rules);
        let modes = composition.play_options[0]
            .modes
            .as_ref()
            .expect("semantic modes synthesize the presentation choices");

        assert_eq!(modes.minimum, 1);
        assert_eq!(modes.maximum, 1);
        assert!(!modes.may_repeat);
        assert_eq!(modes.modes[0].id, ModeId(0));
        assert_eq!(modes.modes[0].label, "Counter target blue spell");
        assert_eq!(modes.modes[0].targets[0].predicate, TargetPredicate::Spell);
        assert_eq!(modes.modes[1].label, "Destroy target blue permanent");
        assert_eq!(
            modes.modes[1].targets[0].predicate,
            TargetPredicate::Permanent
        );
        assert_eq!(
            match rules.ability_clauses()[0].definition {
                DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(0)),
                _ => None,
            }
            .expect("first positional mode")
            .effect,
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetSlotId(3)),
            }
        );
        assert_eq!(
            match rules.ability_clauses()[0].definition {
                DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(1)),
                _ => None,
            }
            .expect("second positional mode")
            .effect,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetSlotId(4)),
                can_regenerate: true,
            }
        );
        assert_eq!(rules.rules_text(), "Choose one.");
    }

    #[test]
    fn printing_ids_distinguish_variants_within_one_set() {
        let definition = CardDefinitionId(7);
        let primary = CardPrintingId::new(definition, CardSet::Alpha);
        let alternate = CardPrintingId::with_variant(definition, CardSet::Alpha, 1);

        assert_eq!(primary.variant, 0);
        assert_ne!(primary, alternate);
        assert_eq!(
            CardPrinting::with_variant(definition, CardSet::Alpha, 1).id,
            alternate
        );
    }

    #[test]
    fn definitions_start_with_their_primary_printing() {
        let id = CardDefinitionId(7);
        let definition = CardDefinition::new(
            id,
            "Test Card",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );

        assert_eq!(
            definition.printings,
            vec![CardPrinting::new(id, CardSet::Alpha)]
        );
    }

    #[test]
    fn planeswalkers_are_permanents() {
        let types = CardTypeSet::single(CardType::Planeswalker);
        assert!(types.is_permanent());
        assert!(!types.is_creature());
    }

    #[test]
    fn artifact_creatures_have_both_card_types() {
        let rules = CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 3, 3);
        assert!(rules.has_type(CardType::Artifact));
        assert!(rules.has_type(CardType::Creature));
        assert!(!rules.has_type(CardType::Enchantment));
        assert_eq!(rules.kind_name(), "ArtifactCreature");
        assert_eq!(rules.type_line(), "Artifact Creature — Golem");
    }

    #[test]
    fn composable_types_cover_magic_card_type_combinations() {
        let enchantment_creature = CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 2, 2)
            .with_type(CardType::Enchantment);
        assert_eq!(
            enchantment_creature.type_line(),
            "Enchantment Creature — Dryad"
        );

        let artifact_land = CardRules::new_land(&[]).with_type(CardType::Artifact);
        assert_eq!(artifact_land.type_line(), "Artifact Land");

        let land_creature = CardRules::new_land(&[])
            .with_type(CardType::Creature)
            .with_subtypes(&["Forest", "Dryad"])
            .with_creature_stats(CreatureStats {
                power: 1,
                toughness: 1,
            });
        assert_eq!(land_creature.type_line(), "Land Creature — Forest Dryad");
        assert_eq!(
            CardComposition::single("Land creature", land_creature).play_options[0].action,
            super::PlayActionKind::PlayLand
        );
    }

    #[test]
    fn white_red_hybrid_costs_have_both_printed_colors() {
        let rules = CardRules::new_creature(mana_cost!("{R/W}{R/W}{R/W}"), &[], 1, 1);
        assert_eq!(rules.colors(), [true, false, false, true, false]);
    }

    #[test]
    fn symbolic_mana_costs_parse_at_compile_time_and_runtime() {
        const COMPILED: ManaCost = mana_cost!("{2}{G}{G}");
        assert_eq!(COMPILED, "{2}{G}{G}".parse().unwrap());
        assert_eq!(COMPILED.generic, 2);
        assert_eq!(COMPILED.green, 2);
        assert_eq!(mana_cost!("{X}{X}{U}").x_multiplier, 2);
        assert_eq!(mana_cost!("{0}"), ManaCost::default());
    }

    #[test]
    fn symbolic_mana_costs_reject_invalid_or_unsupported_notation() {
        for (symbols, expected) in [
            ("", ManaCostParseErrorKind::Empty),
            ("2GG", ManaCostParseErrorKind::ExpectedOpeningBrace),
            ("{2", ManaCostParseErrorKind::UnterminatedSymbol),
            ("{}", ManaCostParseErrorKind::EmptySymbol),
            ("{C}", ManaCostParseErrorKind::InvalidSymbol),
            ("{2}{3}", ManaCostParseErrorKind::DuplicateGenericSymbol),
            ("{65536}", ManaCostParseErrorKind::Overflow),
        ] {
            assert_eq!(ManaCost::parse_symbols(symbols).unwrap_err().kind, expected);
        }
    }

    #[test]
    fn clause_implementation_drives_the_ordinary_play_option_gate() {
        let implemented = CardRules::new_instant(ManaCost::default());
        assert_eq!(
            ImplementationStatus::default(),
            ImplementationStatus::Complete
        );
        assert_eq!(
            CardComposition::single("Implemented", implemented).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );

        let uncategorized =
            CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::not_implemented(
                "Text with no assigned implementation.",
                "The card-specific ability is not executed.",
            ));
        assert_eq!(
            uncategorized.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        let custom =
            CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::custom_full(
                "A card-local effect.",
                CardBehavior::LightningBolt,
                "Implemented by the named card-local special behavior.",
            ));
        assert_eq!(
            custom.implementation_status(),
            ImplementationStatus::Complete
        );
        assert_eq!(custom.special_behavior(), Some(CardBehavior::LightningBolt));

        let metadata_only =
            CardRules::new_instant(ManaCost::default()).with_ability(AbilityDef::not_implemented(
                "A deferred spell effect.",
                "The card-specific ability is not executed.",
            ));
        assert_eq!(
            metadata_only.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            CardComposition::single("Deferred", metadata_only).play_options[0].effect_status,
            CardEffectStatus::MetadataOnly
        );
        let metadata_definition = CardDefinition::new(
            CardDefinitionId(8),
            "Unsupported",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        assert_eq!(
            metadata_definition.implementation_status(),
            ImplementationStatus::MetadataOnly
        );

        let partial = CardRules::new_enchantment(ManaCost::default()).with_ability(
            AbilityDef::custom_partial(
                "A custom clause with one deferred rider.",
                CardBehavior::LightningBolt,
                "One rider is deferred.",
            ),
        );
        assert_eq!(
            partial.ability_clauses()[0].implementation.explanation(),
            Some("One rider is deferred.")
        );
        assert_eq!(
            partial.implementation_status(),
            ImplementationStatus::Partial
        );
        assert_eq!(
            CardComposition::single("Partial", partial).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );
    }

    #[test]
    fn vanilla_creature_body_is_complete() {
        let rules = CardRules::new_creature(ManaCost::default(), &[], 2, 2);

        assert_eq!(
            rules.implementation_status(),
            ImplementationStatus::Complete
        );
    }

    #[test]
    fn creature_body_with_an_unimplemented_clause_is_partial() {
        let rules = CardRules::new_creature(ManaCost::default(), &[], 2, 2)
            .with_abilities(&DEFERRED_CLAUSE);

        assert_eq!(rules.implementation_status(), ImplementationStatus::Partial);
        assert_eq!(
            CardComposition::single("Partial creature", rules).play_options[0].effect_status,
            CardEffectStatus::Implemented
        );
    }

    #[test]
    fn noncreature_with_only_an_unimplemented_clause_is_metadata_only() {
        let rules =
            CardRules::new_enchantment(ManaCost::default()).with_abilities(&DEFERRED_CLAUSE);

        assert_eq!(
            rules.implementation_status(),
            ImplementationStatus::MetadataOnly
        );
        assert_eq!(
            CardComposition::single("Deferred enchantment", rules).play_options[0].effect_status,
            CardEffectStatus::MetadataOnly
        );
    }

    #[test]
    fn no_mana_cost_is_distinct_from_a_printed_zero_cost() {
        let rules = CardRules::new_sorcery(ManaCost::default());
        let zero = CardPart::new(CardPartId::PRIMARY, "Zero", rules);
        let no_cost_rules = CardRules::new_sorcery_without_mana_cost();
        let none = CardPart::new(CardPartId::PRIMARY, "None", no_cost_rules);

        assert_eq!(
            zero.printed_mana_cost(),
            PrintedManaCost::Cost(ManaCost::default())
        );
        assert_eq!(none.printed_mana_cost(), PrintedManaCost::None);
        assert_eq!(zero.printed_mana_cost().mana_value(), 0);
        assert_eq!(none.printed_mana_cost().mana_value(), 0);

        let composition = CardComposition::single("No-cost spell", no_cost_rules);
        assert_eq!(composition.parts[0].mana_cost(), None);
        assert_eq!(composition.play_options[0].mana_cost, None);
    }

    #[test]
    fn typed_rules_expose_coherent_kind_specific_characteristics() {
        let creature = CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear"], 2, 2);
        assert_eq!(creature.types(), CardTypeSet::single(CardType::Creature));
        assert_eq!(creature.subtypes(), &["Bear"]);
        assert_eq!(
            creature.creature_stats(),
            Some(CreatureStats {
                power: 2,
                toughness: 2,
            })
        );
        assert_eq!(creature.starting_loyalty(), None);
        assert_eq!(creature.land_entry_procedure(), LandEntry::Untapped);
        assert_eq!(creature.coherence_error(), None);

        let land = CardRules::new_land(&["Forest"]);
        assert_eq!(land.types(), CardTypeSet::single(CardType::Land));
        assert_eq!(land.printed_mana_cost(), PrintedManaCost::None);
        assert_eq!(land.creature_stats(), None);
        assert_eq!(land.coherence_error(), None);
    }

    #[test]
    fn coherence_validation_covers_kind_specific_invariants() {
        let mut creature_without_stats =
            CardRules::new_creature(ManaCost::default(), &["Bear"], 2, 2);
        creature_without_stats.creature_stats = None;

        let mut instant_with_stats = CardRules::new_instant(ManaCost::default());
        instant_with_stats.creature_stats = Some(CreatureStats {
            power: 1,
            toughness: 1,
        });

        let mut instant_with_loyalty = CardRules::new_instant(ManaCost::default());
        instant_with_loyalty.starting_loyalty = Some(3);

        let mut planeswalker_without_loyalty =
            CardRules::new_planeswalker(ManaCost::default(), &["Test"], 3);
        planeswalker_without_loyalty.starting_loyalty = None;

        let mut instant_with_land_entry = CardRules::new_instant(ManaCost::default());
        instant_with_land_entry.land_entry = LandEntry::Tapped;

        let permanent_instant =
            CardRules::new_instant(ManaCost::default()).with_type(CardType::Artifact);

        for (rules, expected) in [
            (
                creature_without_stats,
                "a creature must have power and toughness",
            ),
            (
                instant_with_stats,
                "a noncreature cannot have creature power and toughness",
            ),
            (
                instant_with_loyalty,
                "a nonplaneswalker cannot have starting loyalty",
            ),
            (
                planeswalker_without_loyalty,
                "a castable planeswalker face must have starting loyalty",
            ),
            (
                instant_with_land_entry,
                "nonland rules cannot declare a land-entry procedure",
            ),
            (
                permanent_instant,
                "an instant or sorcery cannot also be a permanent card type",
            ),
        ] {
            assert_eq!(rules.coherence_error(), Some(expected));
        }
    }

    #[test]
    #[should_panic(expected = "land_entry() is only valid for land rules")]
    fn nonlands_cannot_declare_a_land_entry_procedure() {
        let _ = CardRules::new_instant(ManaCost::default()).land_entry(LandEntry::Tapped);
    }

    #[test]
    #[should_panic(expected = "with_creature_stats() is only valid for creature rules")]
    fn noncreatures_cannot_declare_creature_stats() {
        let _ = CardRules::new_land(&[]).with_creature_stats(CreatureStats {
            power: 1,
            toughness: 1,
        });
    }

    #[test]
    fn ability_category_is_explicit_and_not_inferred_from_effect() {
        const COSTS: &[AbilityCostDef] = &[AbilityCostDef::TapSource];
        const ADD_MANA: EffectDef = EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green));
        const MANA_ABILITY: AbilityDef = AbilityDef::activated_mana("Add green.", COSTS, ADD_MANA);
        const ORDINARY_TRIGGER: AbilityDef = AbilityDef::triggered(
            "Add green when this dies.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(super::ZoneKind::Battlefield),
                to: Some(super::ZoneKind::Graveyard),
            },
            ADD_MANA,
        );
        const TURN_FACE_UP: AbilityDef = AbilityDef::special_action(
            "Turn this face up.",
            &[super::ZoneKind::Battlefield],
            &[AbilityCostDef::Mana(ManaCost::new(3, 0))],
            EffectDef::Special("turn face up"),
        );
        static ABILITIES: [AbilityDef; 3] = [MANA_ABILITY, ORDINARY_TRIGGER, TURN_FACE_UP];

        assert!(!MANA_ABILITY.uses_stack());
        assert!(ORDINARY_TRIGGER.uses_stack());
        assert!(!TURN_FACE_UP.uses_stack());

        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&ABILITIES);
        let attached = rules.indexed_abilities().collect::<Vec<_>>();
        assert_eq!(attached[0].id, AbilityId::PRIMARY);
        assert_eq!(attached[1].id, AbilityId(1));
        assert_eq!(attached[2].id, AbilityId(2));
    }

    #[test]
    fn mana_effects_keep_restrictions_attached_to_each_counted_unit() {
        const RESTRICTIONS: &[ManaRestrictionDef] = &[ManaRestrictionDef::CastSpell(
            ObjectPredicateDef::HasType(super::CardType::Artifact),
        )];
        let workshop_mana = AddManaEffectDef::one(ManaColor::Colorless)
            .with_amount(3)
            .with_restrictions(RESTRICTIONS);

        assert_eq!(workshop_mana.amount, 3);
        assert_eq!(workshop_mana.restrictions, RESTRICTIONS);
    }
}
