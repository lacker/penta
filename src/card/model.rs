use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fmt::Write as _;
use std::str::FromStr;

use crate::ids::{
    AbilityId, AdditionalCostId, AlternativeCostId, CardDefinitionId, CardPartId, MeldRecipeId,
    ModeId, PlayOptionId, TargetIndex, TargetSlotId,
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
    /// The single letter Magic prints for this colour.
    #[must_use]
    pub const fn from_letter(letter: u8) -> Option<Self> {
        match letter {
            b'W' => Some(Self::White),
            b'U' => Some(Self::Blue),
            b'B' => Some(Self::Black),
            b'R' => Some(Self::Red),
            b'G' => Some(Self::Green),
            _ => None,
        }
    }

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
    /// A land with at least one of the listed effective basic land subtypes.
    ///
    /// This uses the object's prospective/effective type line, so continuous
    /// effects such as Blood Moon and Nylea's Presence participate.
    HasAnyBasicLandType(&'static [BasicLandType]),
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
    /// A creature currently declared as an attacker in combat.
    Attacking,
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
    /// "Target player or planeswalker", which is every damage target except
    /// the creatures, narrowed to players in this relation.
    PlayerOrPlaneswalker(PlayerRelation),
    /// A permanent controlled by whoever controls an earlier slot's target,
    /// for "that player or that planeswalker's controller controls".
    ControlledByTargetOf {
        object: ObjectPredicateDef,
        slot: TargetIndex,
    },
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
    pub predicate: AbilityTargetPredicate,
    pub minimum: u8,
    pub maximum: u8,
    /// The total this slot divides among its targets, when the card says
    /// "divided as you choose". Every chosen target takes at least one, which
    /// is what makes the number of targets a consequence of the division.
    pub divided_total: Option<DividedTotal>,
}

/// How much a divided slot has to share out.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DividedTotal {
    Fixed(u8),
    /// The X chosen as the spell was cast, so the number of targets is not
    /// known until then either.
    ChosenX,
}

impl AbilityTargetDef {
    #[must_use]
    pub const fn exactly_one(predicate: AbilityTargetPredicate) -> Self {
        Self {
            predicate,
            minimum: 1,
            maximum: 1,
            divided_total: None,
        }
    }

    /// Any number of targets up to a limit, for "up to three target ...".
    /// Choosing none is a legal choice.
    #[must_use]
    pub const fn up_to(predicate: AbilityTargetPredicate, maximum: u8) -> Self {
        Self {
            predicate,
            minimum: 0,
            maximum,
            divided_total: None,
        }
    }

    /// One spell target, optionally narrowed by color, type, or another
    /// object predicate. Stack object enumeration already excludes abilities,
    /// so callers only need to state the characteristic restriction.
    #[must_use]
    pub const fn exactly_one_spell(object: ObjectPredicateDef) -> Self {
        Self::exactly_one(AbilityTargetPredicate::Object {
            object,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        })
    }

    /// One permanent target, optionally narrowed by color, type, or another
    /// object predicate.
    #[must_use]
    pub const fn exactly_one_permanent(object: ObjectPredicateDef) -> Self {
        Self::exactly_one(AbilityTargetPredicate::Object {
            object,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        })
    }
}

/// One atomic cost. The surrounding rules procedure determines who pays it
/// and what object, if any, is the source.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostDef {
    Mana(ManaCost),
    TapSource,
    UntapSource,
    SacrificeSource,
    /// Remove counters from the permanent carrying this ability as the
    /// ability is activated. The source must carry at least `amount`; paying
    /// the cost removes them before the ability is put on the stack.
    RemoveCountersFromSource {
        kind: CounterKind,
        amount: u16,
    },
    /// Discard the card that carries this ability from its owner's hand.
    DiscardSource,
    PayLife(u16),
    DiscardCards(u8),
    SacrificePermanent {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    ExileSource,
    /// Exile a matching card from the controller's own graveyard. The card is
    /// chosen when the cost is paid, so it travels with the action rather
    /// than being a target.
    ExileCardFromGraveyard(ObjectPredicateDef),
    /// Add or remove that many loyalty counters. A planeswalker's abilities
    /// are the only costs paid this way, and paying one is what makes them
    /// once per turn at sorcery speed.
    Loyalty(i8),
    Special(&'static str),
}

/// Compatibility name for call sites where the costs belong to an ability.
pub type AbilityCostDef = CostDef;

/// Const-friendly storage for activated-ability costs.
///
/// Most card definitions borrow a promoted slice. Common constructors whose
/// costs include a parameter, such as Bloodrush's mana cost, can instead own a
/// small inline list without introducing a mechanic-specific cost primitive.
#[derive(Clone, Copy, Debug)]
pub struct AbilityCostList(AbilityCostStorage);

#[derive(Clone, Copy, Debug)]
enum AbilityCostStorage {
    Borrowed(&'static [AbilityCostDef]),
    Two([AbilityCostDef; 2]),
}

impl PartialEq for AbilityCostList {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for AbilityCostList {}

impl std::hash::Hash for AbilityCostList {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_slice(), state);
    }
}

impl AbilityCostList {
    #[must_use]
    pub(crate) const fn borrowed(costs: &'static [AbilityCostDef]) -> Self {
        Self(AbilityCostStorage::Borrowed(costs))
    }

    #[must_use]
    pub(crate) const fn two(first: AbilityCostDef, second: AbilityCostDef) -> Self {
        Self(AbilityCostStorage::Two([first, second]))
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[AbilityCostDef] {
        match &self.0 {
            AbilityCostStorage::Borrowed(costs) => costs,
            AbilityCostStorage::Two(costs) => costs,
        }
    }

    #[must_use]
    pub fn contains(&self, cost: &AbilityCostDef) -> bool {
        self.as_slice().contains(cost)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, AbilityCostDef> {
        self.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a AbilityCostList {
    type Item = &'a AbilityCostDef;
    type IntoIter = std::slice::Iter<'a, AbilityCostDef>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
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
    pub slot: TargetIndex,
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
    /// How much of a divided total the target being affected takes. Only
    /// meaningful for an effect aimed at a slot the card divides.
    DividedAmongTargets,
    /// The power of what a target slot points at, for "damage equal to its
    /// power".
    TargetPower(TargetIndex),
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
    ObjectsSharingNameWithTarget(TargetIndex),
    Controller,
    Opponent,
    Target(TargetIndex),
    TriggeringObject,
    /// The triggering object's controller when this effect resolves, using
    /// last-known information if that object is no longer live.
    ControllerOfTriggeringObject,
    /// Everything a query matches among the permanents controlled by whoever
    /// controls a target slot, for "each creature that player controls".
    ObjectsControlledByTarget {
        object: ObjectPredicateDef,
        slot: TargetIndex,
    },
    /// The controller of what a target slot points at, for "its controller".
    /// Read when the effect resolves, using last-known information if that
    /// object has already left the battlefield.
    ControllerOfTarget(TargetIndex),
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
    /// Components applied to the same recipient for the same duration as one
    /// continuous effect.
    Composite(&'static [AppliedEffectDef]),
    CannotBeCountered,
    /// A creature matching this predicate cannot block the affected creature.
    CannotBeBlockedBy(ObjectPredicateDef),
    /// Adds land subtypes without removing the object's existing subtypes.
    AddLandTypes(&'static [BasicLandType]),
    ModifyPowerToughness {
        power: ValueDef,
        toughness: ValueDef,
    },
    /// Give the affected object an ordinary ability. The granted definition
    /// carries its own keyword, activation, or alternative-casting procedure.
    GrantAbility(&'static AbilityDef),
    /// Turn the affected permanent into a creature. This is what a manland's
    /// activated ability does, and it keeps the permanent's other types.
    Animate(&'static AnimationDef),
    Special(&'static str),
}

/// The creature a permanent becomes while an animation effect is active. A
/// manland stays a land, so these types and subtypes are added rather than
/// replacing what is printed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AnimationDef {
    pub power: i16,
    pub toughness: i16,
    /// Added on top of the printed types. `Creature` belongs here; a card
    /// that becomes an artifact creature names both.
    pub types: CardTypeSet,
    pub subtypes: &'static [&'static str],
    /// "With all creature types", which no fixed subtype list can express
    /// because changelings must keep matching types printed later.
    pub all_creature_types: bool,
    /// Whether the printed subtypes are replaced rather than added to, for
    /// "becomes a Weird" as opposed to "becomes an Assembly-Worker as well".
    pub replaces_subtypes: bool,
    /// Whether the permanent loses its printed abilities.
    pub loses_abilities: bool,
    /// The colours the permanent becomes, when the animation repaints it.
    pub colors: Option<ColorSet>,
}

impl AnimationDef {
    #[must_use]
    pub const fn new(power: i16, toughness: i16) -> Self {
        Self {
            power,
            toughness,
            types: CardTypeSet::single(CardType::Creature),
            subtypes: &[],
            all_creature_types: false,
            replaces_subtypes: false,
            loses_abilities: false,
            colors: None,
        }
    }

    /// "Loses all abilities and becomes a ..." — the printed subtypes,
    /// abilities, and colours all give way to what the effect names.
    #[must_use]
    pub const fn becoming(mut self, subtypes: &'static [&'static str], colors: ColorSet) -> Self {
        self.subtypes = subtypes;
        self.replaces_subtypes = true;
        self.loses_abilities = true;
        self.colors = Some(colors);
        self
    }

    #[must_use]
    pub const fn with_types(mut self, types: CardTypeSet) -> Self {
        self.types = types;
        self
    }

    #[must_use]
    pub const fn with_subtypes(mut self, subtypes: &'static [&'static str]) -> Self {
        self.subtypes = subtypes;
        self
    }

    #[must_use]
    pub const fn with_all_creature_types(mut self) -> Self {
        self.all_creature_types = true;
        self
    }
}

/// An event that a replacement ability can modify before it is committed.
///
/// Replacement events deliberately have their own vocabulary rather than
/// reusing [`TriggerEventDef`]: triggers observe events that have already
/// happened, while replacement abilities inspect and modify prospective
/// events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEventDef {
    /// The object carrying this ability would enter the battlefield.
    SourceEntersBattlefield,
    /// A matching object would enter the battlefield.
    ObjectEntersBattlefield {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
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
    /// Any object anywhere would be put into this zone. Unlike
    /// [`Self::WouldMove`] this does not describe the moving object's own
    /// ability: the replacement source watches from the battlefield.
    AnyObjectWouldMove { to: ZoneKind },
    /// Compatibility event for existing entry replacements whose exact
    /// subject is identified by their effect primitive.
    EntersBattlefield,
    /// A narrow, named event that is not yet part of the shared vocabulary.
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

/// Costs a player may pay while a replacement effect is modifying an event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PaymentDef {
    pub payer: PlayerRelation,
    pub costs: &'static [CostDef],
}

impl PaymentDef {
    #[must_use]
    pub const fn new(payer: PlayerRelation, costs: &'static [CostDef]) -> Self {
        Self { payer, costs }
    }
}

/// A reusable condition evaluated in an effect's source and event context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionDef {
    /// At least one object matches this zone, controller, and object query.
    Exists(ObjectQueryDef),
}

/// A typed modification to the permanent an object would become as it enters
/// the battlefield.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BattlefieldEntryModificationDef {
    Tapped,
    AddCounters { kind: CounterKind, amount: u16 },
}

/// Declarative operations performed by a replacement ability.
///
/// Branches are slices so complex replacements remain const-friendly and can
/// be resumed around a player choice without baking card names into the game
/// engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEffectDef {
    None,
    Sequence(&'static [ReplacementEffectDef]),
    ModifyBattlefieldEntry(BattlefieldEntryModificationDef),
    Conditional {
        condition: ConditionDef,
        if_true: &'static [ReplacementEffectDef],
        if_false: &'static [ReplacementEffectDef],
    },
    OptionalPayment {
        payment: PaymentDef,
        if_paid: &'static [ReplacementEffectDef],
        if_declined: &'static [ReplacementEffectDef],
    },
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
        /// Run after the sacrifice, with the sacrificed permanent's power as
        /// [`ValueDef::TriggerEventAmount`]. A sacrifice of choice waits on a
        /// decision, so anything reading what was sacrificed has to be part
        /// of the same continuation rather than the next effect in sequence.
        then: Option<&'static EffectDef>,
        /// Whether the player may decline. An optional sacrifice runs `then`
        /// only when something was actually sacrificed, which is what "if a
        /// player does" means; a compulsory one runs it either way, so an
        /// amount read off nothing is zero rather than skipped.
        optional: bool,
    },
    /// Separate everything a player controls into two piles, then let that
    /// player sacrifice the pile of their choice. The ability's controller
    /// makes the split, which is what makes the choice hard for both.
    SplitPermanentsAndSacrificeAPile {
        player: EffectRecipientDef,
    },
    /// Put that many cards from the top of a library into its owner's
    /// graveyard.
    Mill {
        player: EffectRecipientDef,
        amount: ValueDef,
    },
    /// Search a library for one matching card and put it somewhere, then
    /// shuffle. Searching a hidden zone never obliges the searcher to find,
    /// so a printed "may" adds nothing on top of this.
    SearchLibrary {
        player: EffectRecipientDef,
        object: ObjectPredicateDef,
        destination: ZoneKind,
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
    /// Stops the affected players casting noncreature spells for the rest of
    /// the turn.
    CannotCastNoncreatureSpellsThisTurn {
        player: EffectRecipientDef,
    },
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
    /// Gain control of a permanent for the rest of the turn. Control reverts
    /// in cleanup, so nothing needs to remember which effect took it.
    GainControlThisTurn {
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
    /// Adds a combat phase after the one now ending.
    AdditionalCombatPhase,
    /// Turns a double-faced permanent over to its other face.
    Transform {
        object: EffectRecipientDef,
    },
    /// Multiplies the amount of the event a replacement ability is replacing.
    /// This means nothing outside a replacement whose event carries an amount.
    MultiplyEventAmount(u8),
    /// An effect interpreted while replacing a prospective event, rather than
    /// when a spell or ability resolves from the stack.
    Replacement(ReplacementEffectDef),
    MoveToZone {
        object: EffectRecipientDef,
        zone: ZoneKind,
        /// Which end of a library the card lands on. Meaningless for every
        /// other destination.
        placement: LibraryPlacement,
        /// Who controls the permanent when the destination is the
        /// battlefield. `None` is the ordinary case, where a card arrives
        /// under its owner's control; reanimation that steals names a
        /// relation instead.
        controller: Option<PlayerRelation>,
    },
    /// Choose and store a card name for an object as it enters, the same
    /// replacement procedure as choosing a creature type.
    ChooseCardName {
        object: EffectRecipientDef,
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
    pub const fn counter_target(target: TargetIndex) -> Self {
        Self::Counter {
            object: EffectRecipientDef::Target(target),
        }
    }

    #[must_use]
    pub const fn destroy_target(target: TargetIndex, can_regenerate: bool) -> Self {
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
    /// The first time a matching creature attacks in a turn. An extra combat
    /// phase is the only way a creature attacks twice, which is exactly what
    /// the cards carrying this wording tend to grant.
    AttacksFirstTimeThisTurn(ObjectPredicateDef),
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
    /// A state trigger (CR 603.8). It has no event at all: it triggers
    /// whenever its ability's condition is true, and does not trigger again
    /// while it is already waiting or on the stack.
    StateCondition,
    /// This permanent turned over to the face carrying this ability, which is
    /// what "whenever this transforms into ..." names.
    TransformsIntoThisFace,
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
pub enum AbilityProcedureDef {
    /// Costs, action generation, trigger capture, and stack handling use the
    /// shared rules procedures for this ability category.
    Shared,
    /// Transitional compatibility path for an ability whose category is
    /// known but whose surrounding rules procedure still lives in legacy
    /// card behavior.
    Legacy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActivatedAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub costs: AbilityCostList,
    pub targets: &'static [AbilityTargetDef],
    pub procedure: AbilityProcedureDef,
}

impl ActivatedAbilityDef {
    #[must_use]
    pub const fn new(costs: &'static [AbilityCostDef]) -> Self {
        Self::with_costs(AbilityCostList::borrowed(costs))
    }

    #[must_use]
    pub(crate) const fn with_costs(costs: AbilityCostList) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            costs,
            targets: &[],
            procedure: AbilityProcedureDef::Shared,
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

    #[must_use]
    pub const fn with_procedure(mut self, procedure: AbilityProcedureDef) -> Self {
        self.procedure = procedure;
        self
    }
}

/// Whether a condition has to hold for every matching player or just one.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuantifierDef {
    Every,
    Any,
}

/// How a counted amount is compared against a printed number.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ComparisonDef {
    AtLeast,
    AtMost,
    Exactly,
}

/// An intervening-if condition, the "if ..." clause a trigger reads before it
/// does anything. Rule 603.4 checks such a condition twice: once when the
/// ability would go on the stack, and again as it resolves.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TriggerConditionDef {
    /// How many objects the query matches, against a printed number.
    ObjectCount {
        query: ObjectQueryDef,
        comparison: ComparisonDef,
        amount: u8,
    },
    /// Whose turn it is, relative to the ability's controller.
    ActivePlayer(PlayerRelation),
    /// How many spells a matching player cast during the turn before this
    /// one. "No spells were cast last turn" is every player at zero, and "a
    /// player cast two or more" is any player at two.
    SpellsCastLastTurn {
        /// Whether every matching player has to satisfy the comparison or
        /// only one. "No spells were cast last turn" is every player at zero;
        /// "a player cast two or more" is one player at two.
        quantifier: QuantifierDef,
        player: PlayerRelation,
        comparison: ComparisonDef,
        amount: u8,
    },
    /// How much loyalty the ability's own source has left.
    SourceLoyalty {
        comparison: ComparisonDef,
        amount: u8,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TriggeredAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: TriggerEventDef,
    pub targets: &'static [AbilityTargetDef],
    pub procedure: AbilityProcedureDef,
    /// Held by reference so that this definition stays small enough to pass
    /// around by value alongside a captured trigger.
    pub condition: Option<&'static TriggerConditionDef>,
}

impl TriggeredAbilityDef {
    #[must_use]
    pub const fn new(event: TriggerEventDef) -> Self {
        Self {
            source_zones: &[ZoneKind::Battlefield],
            event,
            targets: &[],
            procedure: AbilityProcedureDef::Shared,
            condition: None,
        }
    }

    #[must_use]
    pub const fn with_condition(mut self, condition: &'static TriggerConditionDef) -> Self {
        self.condition = Some(condition);
        self
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

    #[must_use]
    pub const fn with_procedure(mut self, procedure: AbilityProcedureDef) -> Self {
        self.procedure = procedure;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StaticAbilityDef {
    pub source_zones: &'static [ZoneKind],
}

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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlternativeCastKindDef {
    Flashback,
    Overload,
    /// Cast from hand only in the window opened by drawing the card, as the
    /// first card drawn that turn.
    Miracle,
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
    pub const fn label(self) -> &'static str {
        match self {
            Self::Flashback => "Flashback",
            Self::Overload => "Overload",
            Self::Miracle => "Miracle",
        }
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

/// A replacement ability changes how an event happens and never uses the
/// stack. It is modeled separately from a triggered ability even when both
/// watch the same event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplacementAbilityDef {
    pub source_zones: &'static [ZoneKind],
    pub event: ReplacementEventDef,
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

    #[must_use]
    pub const fn with_source_zones(mut self, source_zones: &'static [ZoneKind]) -> Self {
        self.source_zones = source_zones;
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
/// The clause's [`AbilityCoverageDef`] says whether the engine currently
/// executes the keyword. This keeps unimplemented keywords such as banding
/// visible and accurately reflected in aggregate coverage without hiding them
/// in card-level booleans.
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
    /// "Attacks each combat if able." Not a printed keyword, but it behaves
    /// like one: a static requirement with no parameters that several cards
    /// state in the same words.
    AttacksEachCombatIfAble,
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
            Self::AttacksEachCombatIfAble => 16,
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
    AlternativeCast(AlternativeCastAbilityDef),
    SpecialAction(SpecialActionDef),
    Keyword(KeywordAbility),
    /// Transitional structural marker for a clause still dispatched through
    /// the owning card's legacy custom behavior.
    Legacy,
}

/// How an ability's declared effect is executed.
///
/// Coverage is deliberately not represented here: a custom effect can be
/// complete or partial, and a declarative effect can likewise have a gap in
/// its costs, targeting, timing, or another non-effect portion of the clause.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectExecutionDef {
    Declarative,
    Custom(CardBehavior),
}

/// The structured effect and the resolver responsible for executing it.
///
/// Custom execution retains the structured definition as documentation and a
/// migration target, but the shared resolver must not execute that definition
/// until the execution kind becomes [`EffectExecutionDef::Declarative`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityEffectDef {
    pub definition: EffectDef,
    pub execution: EffectExecutionDef,
}

impl AbilityEffectDef {
    #[must_use]
    pub const fn declarative(definition: EffectDef) -> Self {
        Self {
            definition,
            execution: EffectExecutionDef::Declarative,
        }
    }

    #[must_use]
    pub const fn with_execution(mut self, execution: EffectExecutionDef) -> Self {
        self.execution = execution;
        self
    }

    #[must_use]
    pub const fn declarative_definition(self) -> Option<EffectDef> {
        match self.execution {
            EffectExecutionDef::Declarative => Some(self.definition),
            EffectExecutionDef::Custom(_) => None,
        }
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        match self.execution {
            EffectExecutionDef::Custom(behavior) => Some(behavior),
            EffectExecutionDef::Declarative => None,
        }
    }
}

/// Clause-level implementation coverage, independent of effect dispatch.
///
/// An explanation is optional only for an ordinary complete declarative
/// clause. Complete custom and compatibility clauses keep a note explaining
/// their implementation; partial and metadata-only clauses explain the gap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityCoverageDef {
    pub status: ImplementationStatus,
    pub explanation: Option<&'static str>,
}

impl AbilityCoverageDef {
    #[must_use]
    pub const fn complete() -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: None,
        }
    }

    #[must_use]
    pub const fn explained_complete(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Complete,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn partial(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::Partial,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn metadata_only(explanation: &'static str) -> Self {
        Self {
            status: ImplementationStatus::MetadataOnly,
            explanation: Some(explanation),
        }
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        !matches!(self.status, ImplementationStatus::MetadataOnly)
    }
}

/// One printed rules clause and its implementation.
///
/// The category is explicit even when the implementation remains custom; the
/// engine never infers stack behavior from costs, targets, or effects.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityDef {
    /// Static text for ordinary clauses and the keyword label for clauses
    /// whose full Oracle-style text is rendered from structured metadata.
    /// Use [`Self::rules_text`] when presenting a clause.
    pub text: &'static str,
    pub definition: DeclarativeAbilityDef,
    pub effect: AbilityEffectDef,
    pub coverage: AbilityCoverageDef,
}

impl AbilityDef {
    #[must_use]
    pub const fn spell(text: &'static str, effect: EffectDef) -> Self {
        Self::spell_with_targets(text, &[], effect)
    }

    #[must_use]
    pub const fn spell_with_targets(
        text: &'static str,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Spell(SpellAbilityDef::new().with_targets(targets)),
            effect,
        )
    }

    /// A one-target counterspell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn counter_target(text: &'static str, target: &'static AbilityTargetDef) -> Self {
        Self::spell_with_targets(
            text,
            core::slice::from_ref(target),
            EffectDef::counter_target(TargetIndex::PRIMARY),
        )
    }

    /// A one-target destroy spell. The effect recipient is derived from the
    /// target declaration so the two cannot drift apart.
    #[must_use]
    pub const fn destroy_target(
        text: &'static str,
        target: &'static AbilityTargetDef,
        can_regenerate: bool,
    ) -> Self {
        Self::spell_with_targets(
            text,
            core::slice::from_ref(target),
            EffectDef::destroy_target(TargetIndex::PRIMARY, can_regenerate),
        )
    }

    #[must_use]
    pub const fn unimplemented_spell(text: &'static str, explanation: &'static str) -> Self {
        Self::spell(text, EffectDef::None)
            .with_coverage(AbilityCoverageDef::metadata_only(explanation))
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
        Self::activated_with_targets(text, costs, &[], effect)
    }

    #[must_use]
    pub const fn activated_with_targets(
        text: &'static str,
        costs: &'static [AbilityCostDef],
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::activated_with_cost_list_and_targets(
            text,
            AbilityCostList::borrowed(costs),
            targets,
            effect,
        )
    }

    #[must_use]
    pub(crate) const fn activated_with_cost_list_and_targets(
        text: &'static str,
        costs: AbilityCostList,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Activated(
                ActivatedAbilityDef::with_costs(costs).with_targets(targets),
            ),
            effect,
        )
    }

    #[must_use]
    pub const fn triggered(text: &'static str, event: TriggerEventDef, effect: EffectDef) -> Self {
        Self::triggered_with_targets(text, event, &[], effect)
    }

    #[must_use]
    pub const fn triggered_with_targets(
        text: &'static str,
        event: TriggerEventDef,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(TriggeredAbilityDef::new(event).with_targets(targets)),
            effect,
        )
    }

    /// A trigger with an intervening-if condition, for "at the beginning of
    /// your upkeep, if ...".
    #[must_use]
    pub const fn triggered_if(
        text: &'static str,
        event: TriggerEventDef,
        condition: &'static TriggerConditionDef,
        effect: EffectDef,
    ) -> Self {
        Self::triggered_if_with_targets(text, event, condition, &[], effect)
    }

    #[must_use]
    pub const fn triggered_if_with_targets(
        text: &'static str,
        event: TriggerEventDef,
        condition: &'static TriggerConditionDef,
        targets: &'static [AbilityTargetDef],
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            text,
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(event)
                    .with_condition(condition)
                    .with_targets(targets),
            ),
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

    /// Defines a replacement ability that modifies how its own source enters
    /// the battlefield.
    #[must_use]
    pub const fn as_enters(text: &'static str, effect: ReplacementEffectDef) -> Self {
        Self::replacement_for(
            text,
            ReplacementEventDef::SourceEntersBattlefield,
            EffectDef::Replacement(effect),
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
    pub const fn alternative_cast(
        mana_cost: ManaCost,
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::Fixed(mana_cost),
                kind,
                stack_text,
            }),
            effect,
        )
    }

    /// Builds an alternative-casting ability whose cost is the mana cost of
    /// the card carrying the ability. This is resolved only after a concrete
    /// spell form has been selected.
    #[must_use]
    pub const fn alternative_cast_for_card_mana_cost(
        kind: AlternativeCastKindDef,
        stack_text: Option<&'static str>,
        effect: EffectDef,
    ) -> Self {
        Self::defined(
            kind.label(),
            DeclarativeAbilityDef::AlternativeCast(AlternativeCastAbilityDef {
                mana_cost: AlternativeCastManaCostDef::ThisCardManaCost,
                kind,
                stack_text,
            }),
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
            definition,
            effect: AbilityEffectDef::declarative(effect),
            coverage: AbilityCoverageDef::complete(),
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
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None)
                .with_execution(EffectExecutionDef::Custom(behavior)),
            coverage: AbilityCoverageDef::explained_complete(explanation),
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
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None)
                .with_execution(EffectExecutionDef::Custom(behavior)),
            coverage: AbilityCoverageDef::partial(explanation),
        }
    }

    #[must_use]
    pub const fn not_implemented(text: &'static str, explanation: &'static str) -> Self {
        Self {
            text,
            definition: DeclarativeAbilityDef::Legacy,
            effect: AbilityEffectDef::declarative(EffectDef::None),
            coverage: AbilityCoverageDef::metadata_only(explanation),
        }
    }

    #[must_use]
    pub const fn with_effect_execution(mut self, execution: EffectExecutionDef) -> Self {
        self.effect.execution = execution;
        self
    }

    #[must_use]
    pub const fn with_coverage(mut self, coverage: AbilityCoverageDef) -> Self {
        self.coverage = coverage;
        self
    }

    #[must_use]
    /// Selects the legacy procedure for an activated or triggered ability.
    ///
    /// # Panics
    ///
    /// Panics when called on an ability category that has no selectable procedure.
    pub const fn with_legacy_procedure(mut self) -> Self {
        match &mut self.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                definition.procedure = AbilityProcedureDef::Legacy;
            }
            DeclarativeAbilityDef::TriggeredMana(definition)
            | DeclarativeAbilityDef::Triggered(definition) => {
                definition.procedure = AbilityProcedureDef::Legacy;
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::AlternativeCast(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Keyword(_)
            | DeclarativeAbilityDef::Legacy => {
                panic!("only activated and triggered abilities have a selectable procedure")
            }
        }
        self
    }

    #[must_use]
    pub const fn is_executable(self) -> bool {
        self.coverage.is_executable()
    }

    #[must_use]
    pub const fn custom_behavior(self) -> Option<CardBehavior> {
        if self.is_executable() {
            self.effect.custom_behavior()
        } else {
            None
        }
    }

    #[must_use]
    pub const fn declarative_effect(self) -> Option<EffectDef> {
        if self.is_executable() {
            self.effect.declarative_definition()
        } else {
            None
        }
    }

    /// Renders the complete printed clause. Most abilities borrow their
    /// canonical static text; structured alternative-casting keywords insert
    /// their owned mana cost into canonical reminder text.
    #[must_use]
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.definition {
            DeclarativeAbilityDef::AlternativeCast(definition) => {
                Cow::Owned(definition.rules_text())
            }
            _ => Cow::Borrowed(self.text),
        }
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
            | DeclarativeAbilityDef::AlternativeCast(_)
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
        self.coverage.status
    }

    fn implementation_status(self) -> ImplementationStatus {
        let own = self.own_implementation_status();
        let DeclarativeAbilityDef::Spell(spell) = self.definition else {
            return own;
        };
        let Some(modal) = spell.modal() else {
            return own;
        };
        if !self.is_executable() {
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
        if self.effect.execution == EffectExecutionDef::Declarative
            && self.effect.definition == EffectDef::None
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
    /// The total this slot divides among its targets, when the card says
    /// "divided as you choose". Every chosen target takes at least one, which
    /// is what makes the number of targets a consequence of the division.
    pub divided_total: Option<DividedTotal>,
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
            divided_total: None,
        }
    }

    /// "N damage divided as you choose among one, two, or three targets."
    #[must_use]
    pub fn divided(
        id: TargetSlotId,
        label: impl Into<String>,
        predicate: TargetPredicate,
        total: u8,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            predicate,
            minimum: 1,
            maximum: total,
            divided_total: Some(DividedTotal::Fixed(total)),
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
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

fn predicate_color(predicate: ObjectPredicateDef) -> Option<ManaColor> {
    match predicate {
        ObjectPredicateDef::Color(color) => Some(color),
        ObjectPredicateDef::All(predicates) => predicates.iter().copied().find_map(predicate_color),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Subtype(subtype) => Some(subtype),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_subtype)
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_negated_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Not(inner) => match *inner {
            ObjectPredicateDef::Subtype(subtype) => Some(subtype),
            _ => None,
        },
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_negated_subtype),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_power_at_least(predicate: ObjectPredicateDef) -> Option<i16> {
    match predicate {
        ObjectPredicateDef::PowerAtLeast(power) => Some(power),
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_power_at_least),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_mana_value_at_most(predicate: ObjectPredicateDef) -> Option<u8> {
    match predicate {
        ObjectPredicateDef::ManaValueAtMost(value) => Some(value),
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_mana_value_at_most),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_controller(predicate: ObjectPredicateDef) -> Option<PlayerRelation> {
    match predicate {
        ObjectPredicateDef::ControlledBy(controller) => Some(controller),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_controller)
        }
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

fn predicate_negates(predicate: ObjectPredicateDef, expected: ObjectPredicateDef) -> bool {
    match predicate {
        // Stay deliberately conservative: `not (red land)` does not imply
        // "nonland," even though the inner conjunction implies `land`.
        ObjectPredicateDef::Not(inner) => *inner == expected,
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .any(|predicate| predicate_negates(predicate, expected)),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

const fn color_name(color: ManaColor) -> &'static str {
    match color {
        ManaColor::White => "white",
        ManaColor::Blue => "blue",
        ManaColor::Black => "black",
        ManaColor::Red => "red",
        ManaColor::Green => "green",
        ManaColor::Colorless => "colorless",
    }
}

const fn card_type_name(card_type: CardType) -> &'static str {
    match card_type {
        CardType::Artifact => "artifact",
        CardType::Creature => "creature",
        CardType::Enchantment => "enchantment",
        CardType::Instant => "instant",
        CardType::Land => "land",
        CardType::Planeswalker => "planeswalker",
        CardType::Sorcery => "sorcery",
    }
}

fn simple_disjunction_subject(predicate: ObjectPredicateDef) -> Option<String> {
    let ObjectPredicateDef::AnyOf(predicates) = predicate else {
        return None;
    };
    let subjects = predicates
        .iter()
        .copied()
        .map(|predicate| match predicate {
            ObjectPredicateDef::HasType(card_type) => Some(card_type_name(card_type)),
            ObjectPredicateDef::Subtype(subtype) => Some(subtype),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    (!subjects.is_empty()).then(|| subjects.join(" or "))
}

fn object_target_subject(object: ObjectPredicateDef, predicate: TargetPredicate) -> String {
    if let ObjectPredicateDef::Special(description) = object {
        return description.into();
    }
    if object_predicate_implies(object, ObjectPredicateDef::Attacking) {
        return "attacking creature".into();
    }
    match predicate {
        TargetPredicate::AnyTarget => "target".into(),
        TargetPredicate::Player => "player".into(),
        TargetPredicate::NoncreatureSpell => predicate_color(object).map_or_else(
            || "noncreature spell".into(),
            |color| format!("{} noncreature spell", color_name(color)),
        ),
        TargetPredicate::Spell => predicate_color(object).map_or_else(
            || "spell".into(),
            |color| format!("{} spell", color_name(color)),
        ),
        TargetPredicate::CreaturePermanent => {
            if object_predicate_implies(object, ObjectPredicateDef::AttackingOrBlocking) {
                "attacking or blocking creature".into()
            } else if object_predicate_implies(object, ObjectPredicateDef::Attacking) {
                "attacking creature".into()
            } else if let Some(subtype) = predicate_negated_subtype(object) {
                format!("non-{subtype} creature")
            } else if let Some(subtype) = predicate_subtype(object) {
                format!("{subtype} creature")
            } else if let Some(color) = predicate_color(object) {
                format!("{} creature", color_name(color))
            } else if let Some(power) = predicate_power_at_least(object) {
                format!("creature with power {power} or greater")
            } else {
                "creature".into()
            }
        }
        TargetPredicate::Permanent => {
            if let Some(subject) = simple_disjunction_subject(object) {
                subject
            } else if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Land))
                && predicate_negates(object, ObjectPredicateDef::Supertype(CardSupertype::Basic))
            {
                "nonbasic land".into()
            } else if predicate_negates(object, ObjectPredicateDef::HasType(CardType::Land)) {
                let mut subject = "nonland permanent".to_string();
                if let Some(value) = predicate_mana_value_at_most(object) {
                    let _ = write!(subject, " with mana value {value} or less");
                }
                subject
            } else if let Some(card_type) = CardType::DISPLAY_ORDER.into_iter().find(|card_type| {
                object_predicate_implies(object, ObjectPredicateDef::HasType(*card_type))
            }) {
                card_type_name(card_type).into()
            } else if let Some(subtype) = predicate_subtype(object) {
                subtype.into()
            } else if let Some(color) = predicate_color(object) {
                format!("{} permanent", color_name(color))
            } else {
                "permanent".into()
            }
        }
    }
}

fn semantic_card_subject(object: ObjectPredicateDef) -> String {
    if let Some(subject) = simple_disjunction_subject(object) {
        return format!("{subject} card");
    }
    if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
        "creature card".into()
    } else if let Some(subtype) = predicate_subtype(object) {
        format!("{subtype} card")
    } else if let ObjectPredicateDef::Special(description) = object {
        description.into()
    } else {
        "card".into()
    }
}

fn semantic_object_target_subject(
    object: ObjectPredicateDef,
    zones: &'static [ZoneKind],
    owner: Option<PlayerRelation>,
) -> String {
    if zones == [ZoneKind::Graveyard] {
        let subject = semantic_card_subject(object);
        let graveyard = match owner {
            Some(PlayerRelation::You) => "your graveyard",
            Some(PlayerRelation::Opponent) => "an opponent's graveyard",
            Some(PlayerRelation::NotYou) => "a graveyard other than yours",
            Some(PlayerRelation::ActivePlayer) => "the active player's graveyard",
            Some(PlayerRelation::NonactivePlayer) => "the nonactive player's graveyard",
            Some(PlayerRelation::EventPlayer) => "the event player's graveyard",
            Some(PlayerRelation::Any) | None => "a graveyard",
        };
        return format!("{subject} in {graveyard}");
    }
    if zones == [ZoneKind::Battlefield, ZoneKind::Graveyard]
        && object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature))
    {
        return "creature on the battlefield or creature card in a graveyard".into();
    }
    let subject = semantic_card_subject(object);
    match zones {
        [ZoneKind::Hand] => format!("{subject} in a hand"),
        [ZoneKind::Library] => format!("{subject} in a library"),
        [ZoneKind::Exile] => format!("{subject} in exile"),
        _ => subject,
    }
}

const fn player_target_label(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "target player",
        PlayerRelation::You => "yourself",
        PlayerRelation::NotYou => "target player other than you",
        PlayerRelation::Opponent => "target opponent",
        PlayerRelation::ActivePlayer => "target active player",
        PlayerRelation::NonactivePlayer => "target nonactive player",
        PlayerRelation::EventPlayer => "target event player",
    }
}

const fn player_or_planeswalker_target_label(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "target player or planeswalker",
        PlayerRelation::You => "yourself or target planeswalker",
        PlayerRelation::NotYou => "target player other than you or planeswalker",
        PlayerRelation::Opponent => "target opponent or planeswalker",
        PlayerRelation::ActivePlayer => "target active player or planeswalker",
        PlayerRelation::NonactivePlayer => "target nonactive player or planeswalker",
        PlayerRelation::EventPlayer => "target event player or planeswalker",
    }
}

const fn controller_suffix(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "",
        PlayerRelation::You => " you control",
        PlayerRelation::NotYou => " you don't control",
        PlayerRelation::Opponent => " an opponent controls",
        PlayerRelation::ActivePlayer => " the active player controls",
        PlayerRelation::NonactivePlayer => " the nonactive player controls",
        PlayerRelation::EventPlayer => " the event player controls",
    }
}

const fn owner_suffix(relation: PlayerRelation) -> &'static str {
    match relation {
        PlayerRelation::Any => "",
        PlayerRelation::You => " you own",
        PlayerRelation::NotYou => " you don't own",
        PlayerRelation::Opponent => " an opponent owns",
        PlayerRelation::ActivePlayer => " the active player owns",
        PlayerRelation::NonactivePlayer => " the nonactive player owns",
        PlayerRelation::EventPlayer => " the event player owns",
    }
}

fn append_relation_suffix(label: &mut String, suffix: &'static str) {
    if suffix.is_empty() {
        return;
    }
    // Keep the relation next to its noun: "creature you control with ...",
    // rather than making it appear to modify a later characteristic.
    let position = label.find(" with ").unwrap_or(label.len());
    label.insert_str(position, suffix);
}

fn presentation_target_predicate(predicate: AbilityTargetPredicate) -> Option<TargetPredicate> {
    match predicate {
        // A client has no slot kind narrower than every damage target, which
        // is closer than presenting only the player half of this predicate.
        AbilityTargetPredicate::AnyTarget | AbilityTargetPredicate::PlayerOrPlaneswalker(_) => {
            Some(TargetPredicate::AnyTarget)
        }
        AbilityTargetPredicate::ControlledByTargetOf { object, .. } => {
            if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
                Some(TargetPredicate::CreaturePermanent)
            } else {
                Some(TargetPredicate::Permanent)
            }
        }
        AbilityTargetPredicate::Player(_) => Some(TargetPredicate::Player),
        AbilityTargetPredicate::Object { object, zones, .. } if zones == [ZoneKind::Stack] => {
            if object_predicate_implies(object, ObjectPredicateDef::NoncreatureSpell) {
                Some(TargetPredicate::NoncreatureSpell)
            } else {
                Some(TargetPredicate::Spell)
            }
        }
        AbilityTargetPredicate::Object { object, zones, .. }
            if zones == [ZoneKind::Battlefield] =>
        {
            if object_predicate_implies(object, ObjectPredicateDef::HasType(CardType::Creature)) {
                Some(TargetPredicate::CreaturePermanent)
            } else {
                Some(TargetPredicate::Permanent)
            }
        }
        AbilityTargetPredicate::Object { .. } => None,
    }
}

impl AbilityTargetDef {
    /// Derives concise presentation text from the authoritative predicate.
    ///
    /// This is only a label: compound restrictions may be summarized, while
    /// target enumeration and legality always use [`Self::predicate`]. The
    /// renderer prefers a broader accurate noun phrase over guessing at
    /// English for an unfamiliar predicate combination.
    pub(crate) fn label(self) -> String {
        match self.predicate {
            AbilityTargetPredicate::AnyTarget => "any target".into(),
            AbilityTargetPredicate::PlayerOrPlaneswalker(relation) => {
                player_or_planeswalker_target_label(relation).into()
            }
            AbilityTargetPredicate::ControlledByTargetOf { object, .. } => {
                let predicate = presentation_target_predicate(self.predicate)
                    .expect("dependent targets always project to a permanent target");
                let subject = object_target_subject(object, predicate);
                format!("target {subject} that player or that planeswalker's controller controls")
            }
            AbilityTargetPredicate::Player(relation) => player_target_label(relation).into(),
            AbilityTargetPredicate::Object {
                object,
                zones,
                controller,
                owner,
            } => {
                let predicate = presentation_target_predicate(self.predicate);
                let subject = predicate.map_or_else(
                    || semantic_object_target_subject(object, zones, owner),
                    |predicate| object_target_subject(object, predicate),
                );
                let mut label = format!("target {subject}");
                if predicate_negates(object, ObjectPredicateDef::Source) {
                    label.insert_str("target ".len(), "another ");
                }
                if predicate_negates(object, ObjectPredicateDef::SharesNameWithSource) {
                    label.push_str(" with a different name from this source");
                }
                let relation = controller.or_else(|| predicate_controller(object));
                if let Some(relation) = relation {
                    append_relation_suffix(&mut label, controller_suffix(relation));
                } else if predicate.is_some()
                    && let Some(relation) = owner
                {
                    append_relation_suffix(&mut label, owner_suffix(relation));
                }
                label
            }
        }
    }

    pub(super) fn presentation(self, id: TargetSlotId) -> Option<TargetSlotDef> {
        let predicate = presentation_target_predicate(self.predicate)?;
        Some(TargetSlotDef {
            id,
            label: self.label(),
            predicate,
            minimum: self.minimum,
            maximum: self.maximum,
            divided_total: self.divided_total,
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
        let mut targets = Vec::with_capacity(spell.targets().len());
        for (index, target) in spell.targets().iter().copied().enumerate() {
            let id = TargetSlotId::from_index(index)?;
            let Some(target) = target.presentation(id) else {
                // The semantic target vocabulary is richer than the legacy
                // presentation predicate. An empty projection keeps runtime
                // targeting authoritative without publishing an approximation.
                targets.clear();
                break;
            };
            targets.push(target);
        }
        Some(ModeDef {
            id,
            label: self.text.into(),
            targets,
            effect_status: if outer_is_executable && self.is_executable() {
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

    /// Adds the printed alternative costs owned by alternative-casting
    /// clauses on `rules`. Existing manually authored generic alternatives
    /// remain intact.
    #[must_use]
    pub fn with_alternative_cast_costs(mut self, rules: &CardRules) -> Self {
        let card_mana_cost = self.mana_cost;
        self.alternative_costs.extend(
            rules
                .indexed_abilities()
                .filter_map(|ability| ability.alternative_cost(card_mana_cost)),
        );
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
            .with_alternative_cast_costs(&rules)
        };
        if let Some(modes) = rules.presentation_spell_modes() {
            option = option.with_modes(modes);
        }
        Self {
            parts: vec![part],
            structure: CardStructure::Single {
                main: CardPartId::PRIMARY,
            },
            play_options: vec![option],
        }
        .with_derived_spell_targets()
    }

    /// Derives nonmodal play-option target presentations from the spell
    /// clauses of the option's parts. Combined forms flatten their parts in
    /// printed order, assigning runtime slot IDs only after composition.
    ///
    /// A composition can still supply explicit presentation targets when it
    /// has no semantic spell clause. When the semantic predicate vocabulary
    /// is richer than the legacy presentation vocabulary, the projection is
    /// left empty and runtime target generation uses the semantic definition.
    #[must_use]
    pub(crate) fn with_derived_spell_targets(mut self) -> Self {
        for option in &mut self.play_options {
            if option.action != PlayActionKind::CastSpell
                || option.modes.is_some()
                || !option.targets.is_empty()
            {
                continue;
            }
            let part_ids = match &option.form {
                SpellForm::Part(part) => core::slice::from_ref(part),
                SpellForm::Combined(parts) => parts.as_slice(),
            };
            let derived = part_ids
                .iter()
                .try_fold(Vec::new(), |mut targets, part_id| {
                    let part = self.parts.iter().find(|part| part.id == *part_id)?;
                    let spell = part.rules.ability_clauses().iter().find_map(|ability| {
                        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                            return None;
                        };
                        spell.modal().is_none().then_some(spell)
                    })?;
                    for target in spell.targets() {
                        let id = TargetSlotId::from_index(targets.len())?;
                        targets.push(target.presentation(id)?);
                    }
                    Some(targets)
                });
            if let Some(derived) = derived {
                option.targets = derived;
            }
        }
        self
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

    /// The face on the other side of a double-faced card, or nothing when the
    /// card has only one side to present.
    #[must_use]
    pub fn other_face(&self, presented: CardPartId) -> Option<CardPartId> {
        let CardStructure::DoubleFaced { front, back, .. } = &self.structure else {
            return None;
        };
        if presented == *front {
            Some(*back)
        } else if presented == *back {
            Some(*front)
        } else {
            None
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
    IronclawOrcs,
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

/// Which end of a library a card is put on.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LibraryPlacement {
    #[default]
    Top,
    Bottom,
}

/// One two-colour hybrid symbol, such as `{R/W}`. Either colour pays it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HybridPair {
    WhiteBlue,
    WhiteBlack,
    WhiteRed,
    WhiteGreen,
    BlueBlack,
    BlueRed,
    BlueGreen,
    BlackRed,
    BlackGreen,
    RedGreen,
}

impl HybridPair {
    pub const COUNT: usize = 10;

    pub const ALL: [Self; Self::COUNT] = [
        Self::WhiteBlue,
        Self::WhiteBlack,
        Self::WhiteRed,
        Self::WhiteGreen,
        Self::BlueBlack,
        Self::BlueRed,
        Self::BlueGreen,
        Self::BlackRed,
        Self::BlackGreen,
        Self::RedGreen,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// The two colours, in the order Magic prints them.
    #[must_use]
    pub const fn colors(self) -> (ManaColor, ManaColor) {
        match self {
            Self::WhiteBlue => (ManaColor::White, ManaColor::Blue),
            Self::WhiteBlack => (ManaColor::White, ManaColor::Black),
            Self::WhiteRed => (ManaColor::White, ManaColor::Red),
            Self::WhiteGreen => (ManaColor::White, ManaColor::Green),
            Self::BlueBlack => (ManaColor::Blue, ManaColor::Black),
            Self::BlueRed => (ManaColor::Blue, ManaColor::Red),
            Self::BlueGreen => (ManaColor::Blue, ManaColor::Green),
            Self::BlackRed => (ManaColor::Black, ManaColor::Red),
            Self::BlackGreen => (ManaColor::Black, ManaColor::Green),
            Self::RedGreen => (ManaColor::Red, ManaColor::Green),
        }
    }

    #[must_use]
    pub const fn contains(self, color: ManaColor) -> bool {
        let (first, second) = self.colors();
        matches!(color, c if c as u8 == first as u8)
            || matches!(color, c if c as u8 == second as u8)
    }

    /// The printed symbol between the braces, such as `R/W`. Magic prints
    /// each pair in a fixed order that is not always alphabetical.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::WhiteBlue => "W/U",
            Self::WhiteBlack => "W/B",
            Self::WhiteRed => "R/W",
            Self::WhiteGreen => "G/W",
            Self::BlueBlack => "U/B",
            Self::BlueRed => "U/R",
            Self::BlueGreen => "G/U",
            Self::BlackRed => "B/R",
            Self::BlackGreen => "B/G",
            Self::RedGreen => "R/G",
        }
    }

    /// Parses the two colour letters of a hybrid symbol. The printed order
    /// varies by pair, so both orders are accepted.
    #[must_use]
    pub const fn from_letters(first: u8, second: u8) -> Option<Self> {
        let Some(first) = ManaColor::from_letter(first) else {
            return None;
        };
        let Some(second) = ManaColor::from_letter(second) else {
            return None;
        };
        let mut index = 0;
        while index < Self::COUNT {
            let pair = Self::ALL[index];
            let (a, b) = pair.colors();
            if (a as u8 == first as u8 && b as u8 == second as u8)
                || (a as u8 == second as u8 && b as u8 == first as u8)
            {
                return Some(pair);
            }
            index += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaCost {
    pub generic: u16,
    pub white: u16,
    pub blue: u16,
    pub black: u16,
    pub red: u16,
    pub green: u16,
    /// How many hybrid symbols of each colour pair this cost carries, indexed
    /// by [`HybridPair::index`].
    pub hybrid: [u16; HybridPair::COUNT],
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
            hybrid: [0; HybridPair::COUNT],
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
                && bytes[symbol_start + 1] == b'/'
                && let Some(pair) =
                    HybridPair::from_letters(bytes[symbol_start], bytes[symbol_start + 2])
            {
                let index = pair.index();
                cost.hybrid[index] = match Self::checked_increment(cost.hybrid[index]) {
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
            .saturating_add(self.hybrid_total())
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
            hybrid: [0; HybridPair::COUNT],
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
            hybrid: [0; HybridPair::COUNT],
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
            hybrid: [0; HybridPair::COUNT],
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
            hybrid: [0; HybridPair::COUNT],
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
            hybrid: [0; HybridPair::COUNT],
            variable_x: true,
            x_multiplier,
        }
    }

    /// How many hybrid symbols this cost carries in total.
    #[must_use]
    pub const fn hybrid_total(&self) -> u16 {
        let mut total: u16 = 0;
        let mut index = 0;
        while index < HybridPair::COUNT {
            total = total.saturating_add(self.hybrid[index]);
            index += 1;
        }
        total
    }

    #[must_use]
    pub const fn hybrid_pair(pair: HybridPair, count: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            hybrid: {
                let mut hybrid = [0; HybridPair::COUNT];
                hybrid[pair.index()] = count;
                hybrid
            },
            variable_x: false,
            x_multiplier: 0,
        }
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote_symbol = false;
        if self.generic > 0 {
            write!(formatter, "{{{}}}", self.generic)?;
            wrote_symbol = true;
        }
        if self.variable_x {
            for _ in 0..self.x_multiplier.max(1) {
                formatter.write_str("{X}")?;
                wrote_symbol = true;
            }
        }
        for (amount, symbol) in [
            (self.white, "W"),
            (self.blue, "U"),
            (self.black, "B"),
            (self.red, "R"),
            (self.green, "G"),
        ] {
            for _ in 0..amount {
                write!(formatter, "{{{symbol}}}")?;
                wrote_symbol = true;
            }
        }
        for pair in HybridPair::ALL {
            for _ in 0..self.hybrid[pair.index()] {
                write!(formatter, "{{{}}}", pair.symbol())?;
                wrote_symbol = true;
            }
        }
        if !wrote_symbol {
            formatter.write_str("{0}")?;
        }
        Ok(())
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CreatureStats {
    pub power: i16,
    pub toughness: i16,
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

impl AttachedAbilityDef {
    /// The stable cost-choice identity of a printed alternative-casting
    /// clause. Printed alternative costs use their owning ability's positional
    /// identity rather than a separately maintained identifier.
    #[must_use]
    pub const fn alternative_cost_id(self) -> Option<AlternativeCostId> {
        if matches!(
            self.definition.definition,
            DeclarativeAbilityDef::AlternativeCast(_)
        ) {
            Some(AlternativeCostId(self.id.0))
        } else {
            None
        }
    }

    /// Materializes the play-option view of a printed alternative cost.
    #[must_use]
    pub fn alternative_cost(self, card_mana_cost: Option<ManaCost>) -> Option<AlternativeCostDef> {
        let DeclarativeAbilityDef::AlternativeCast(definition) = self.definition.definition else {
            return None;
        };
        definition.alternative_cost(self.id, card_mana_cost)
    }
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
    starting_loyalty: Option<u16>,
    creature_stats: Option<CreatureStats>,
    /// Ordered printed rules clauses. Abilities supplied by the rules, such as
    /// those intrinsic to basic land types, are derived by the game engine.
    abilities: CardAbilityList,
    colors: ColorSet,
}

/// Whether any hybrid symbol in this cost can be paid with one colour.
const fn hybrid_includes(cost: ManaCost, color: ManaColor) -> bool {
    let mut index = 0;
    while index < HybridPair::COUNT {
        if cost.hybrid[index] > 0 && HybridPair::ALL[index].contains(color) {
            return true;
        }
        index += 1;
    }
    false
}

impl CardRules {
    const fn base(card_types: CardTypeSet, printed_mana_cost: PrintedManaCost) -> Self {
        let mana_cost = match printed_mana_cost {
            PrintedManaCost::None => ManaCost::new(0, 0),
            PrintedManaCost::Cost(cost) => cost,
        };
        let mut colors = ColorSet::empty();
        if mana_cost.white > 0 || hybrid_includes(mana_cost, ManaColor::White) {
            colors = colors.with(ManaColor::White);
        }
        if mana_cost.blue > 0 || hybrid_includes(mana_cost, ManaColor::Blue) {
            colors = colors.with(ManaColor::Blue);
        }
        if mana_cost.black > 0 || hybrid_includes(mana_cost, ManaColor::Black) {
            colors = colors.with(ManaColor::Black);
        }
        if mana_cost.red > 0 || hybrid_includes(mana_cost, ManaColor::Red) {
            colors = colors.with(ManaColor::Red);
        }
        if mana_cost.green > 0 || hybrid_includes(mana_cost, ManaColor::Green) {
            colors = colors.with(ManaColor::Green);
        }
        Self {
            card_types,
            supertypes: [false; CardSupertype::COUNT],
            subtypes: &[],
            printed_mana_cost,
            starting_loyalty: None,
            creature_stats: None,
            abilities: CardAbilityList::None,
            colors,
        }
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
            .find_map(|ability| ability.custom_behavior())
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
                mode.mode_presentation(ModeId::from_index(index)?, ability.is_executable())
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
    pub fn rules_text(&self) -> Cow<'static, str> {
        match self.abilities {
            CardAbilityList::None => Cow::Borrowed(""),
            CardAbilityList::One(ability) => ability.rules_text(),
            CardAbilityList::Many(abilities) => Cow::Owned(
                abilities
                    .iter()
                    .map(AbilityDef::rules_text)
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
            ability.is_executable()
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
        AbilityCostDef, AbilityCostList, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
        AddManaEffectDef, AlternativeCastKindDef, AlternativeCostDef, CardBehavior,
        CardComposition, CardDefinition, CardEffectStatus, CardPart, CardPrinting, CardPrintingId,
        CardRules, CardSet, CardType, CardTypeSet, CreatureStats, DeclarativeAbilityDef, EffectDef,
        EffectRecipientDef, ImplementationStatus, ManaColor, ManaCost, ManaCostParseErrorKind,
        ManaRestrictionDef, ObjectPredicateDef, PlayOptionDef, PlayerRelation, PrintedManaCost,
        SpellForm, TargetPredicate, TriggerEventDef, ZoneKind,
    };
    use crate::{
        AbilityId, AlternativeCostId, CardDefinitionId, CardPartId, ModeId, PlayOptionId,
        TargetIndex,
    };

    static DEFERRED_CLAUSE: [AbilityDef; 1] = [AbilityDef::not_implemented(
        "A deferred card-specific ability.",
        "The card-specific ability is not executed.",
    )];

    #[test]
    fn ability_cost_list_equality_and_hash_ignore_storage_representation() {
        use std::collections::{HashSet, hash_map::DefaultHasher};
        use std::hash::{Hash, Hasher};

        static COSTS: [AbilityCostDef; 2] = [
            AbilityCostDef::Mana(ManaCost::new(2, 0)),
            AbilityCostDef::DiscardSource,
        ];
        let borrowed = AbilityCostList::borrowed(&COSTS);
        let inline = AbilityCostList::two(COSTS[0], COSTS[1]);

        let hash = |costs: AbilityCostList| {
            let mut hasher = DefaultHasher::new();
            costs.hash(&mut hasher);
            hasher.finish()
        };

        assert_eq!(borrowed, inline);
        assert_eq!(hash(borrowed), hash(inline));
        assert!(HashSet::from([borrowed]).contains(&inline));
    }

    #[test]
    fn modal_spell_semantics_derive_their_presentation_modes() {
        const RULES: CardRules = CardRules::new_instant(crate::mana_cost!("{0}")).with_ability(
            AbilityDef::choose_one_spell(
                "Choose one.",
                &[
                    AbilityDef::counter_target(
                        "Counter target blue spell",
                        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(
                            ManaColor::Blue,
                        )),
                    ),
                    AbilityDef::destroy_target(
                        "Destroy target blue permanent",
                        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                            ManaColor::Blue,
                        )),
                        true,
                    ),
                    AbilityDef::spell_with_targets(
                        "Return target creature card from your graveyard.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &[ZoneKind::Graveyard],
                                controller: None,
                                owner: Some(PlayerRelation::You),
                            },
                        )],
                        EffectDef::None,
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
            modes.modes[2].label,
            "Return target creature card from your graveyard."
        );
        assert!(
            modes.modes[2].targets.is_empty(),
            "semantic-only mode targets keep an empty legacy projection"
        );
        assert_eq!(
            match rules.ability_clauses()[0].definition {
                DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(0)),
                _ => None,
            }
            .expect("first positional mode")
            .effect
            .definition,
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }
        );
        assert_eq!(
            match rules.ability_clauses()[0].definition {
                DeclarativeAbilityDef::Spell(spell) => spell.mode(ModeId(1)),
                _ => None,
            }
            .expect("second positional mode")
            .effect
            .definition,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            }
        );
        assert_eq!(rules.rules_text(), "Choose one.");
    }

    #[test]
    fn semantic_target_labels_are_derived_from_predicates() {
        let opponent =
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent));
        assert_eq!(opponent.label(), "target opponent");

        let creature_you_control = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::You),
            owner: None,
        });
        assert_eq!(creature_you_control.label(), "target creature you control");

        let constrained_creature = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Special(
                "creature with toughness less than the source's power",
            ),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::You),
            owner: None,
        });
        assert_eq!(
            constrained_creature.label(),
            "target creature you control with toughness less than the source's power"
        );

        let non_demon = AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Demon")),
        ]));
        assert_eq!(non_demon.label(), "target non-Demon creature");

        let not_red_land = AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
            &ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
        ));
        assert_eq!(
            not_red_land.label(),
            "target permanent",
            "a conservative label must not turn 'not a red land' into 'nonland'"
        );

        let graveyard = AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        });
        assert_eq!(graveyard.label(), "target creature card in your graveyard");
        assert!(
            graveyard.presentation(crate::TargetSlotId(0)).is_none(),
            "semantic-only targets still have decision labels without a legacy projection",
        );

        let blue_spell =
            AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue));
        let presentation = blue_spell
            .presentation(crate::TargetSlotId(0))
            .expect("a stack target has a presentation projection");
        assert_eq!(blue_spell.label(), "target blue spell");
        assert_eq!(presentation.label, blue_spell.label());
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
        assert_eq!(mana_cost!("{0}").to_string(), "{0}");
        assert_eq!(
            mana_cost!("{12}{X}{X}{W}{U}{B}{R}{G}{R/W}").to_string(),
            "{12}{X}{X}{W}{U}{B}{R}{G}{R/W}",
        );
    }

    #[test]
    fn alternative_cast_clauses_render_and_project_their_owned_costs() {
        static ABILITIES: [AbilityDef; 3] = [
            AbilityDef::spell("Draw a card.", EffectDef::None),
            AbilityDef::alternative_cast(
                mana_cost!("{2}{U}"),
                AlternativeCastKindDef::Flashback,
                None,
                EffectDef::None,
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{3}{R}"),
                AlternativeCastKindDef::Overload,
                Some("Draw a card for each opponent."),
                EffectDef::None,
            ),
        ];
        let rules = CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&ABILITIES);

        assert_eq!(ABILITIES[1].text, "Flashback");
        assert_eq!(
            ABILITIES[1].rules_text(),
            "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
        );
        assert_eq!(
            ABILITIES[2].rules_text(),
            "Overload {3}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
        );
        assert_eq!(
            rules.rules_text(),
            concat!(
                "Draw a card.\n",
                "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)\n",
                "Overload {3}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
            ),
        );

        let composition = CardComposition::single("Test spell", rules);
        assert_eq!(
            composition.play_options[0].alternative_costs,
            vec![
                AlternativeCostDef {
                    id: AlternativeCostId(1),
                    label: "Flashback".into(),
                    mana_cost: mana_cost!("{2}{U}"),
                },
                AlternativeCostDef {
                    id: AlternativeCostId(2),
                    label: "Overload".into(),
                    mana_cost: mana_cost!("{3}{R}"),
                },
            ],
        );

        let mut generic = PlayOptionDef::cast(
            PlayOptionId(4),
            "Generic alternative",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{1}{U}"),
            CardEffectStatus::Implemented,
        );
        generic.alternative_costs.push(AlternativeCostDef {
            id: AlternativeCostId(9),
            label: "Generic".into(),
            mana_cost: mana_cost!("{U}"),
        });
        let projected = generic.with_alternative_cast_costs(&rules);
        assert_eq!(projected.alternative_costs[0].label, "Generic");
        assert_eq!(projected.alternative_costs.len(), 3);
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
            partial.ability_clauses()[0].coverage.explanation,
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
                permanent_instant,
                "an instant or sorcery cannot also be a permanent card type",
            ),
        ] {
            assert_eq!(rules.coherence_error(), Some(expected));
        }
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
    #[should_panic(expected = "only activated and triggered abilities have a selectable procedure")]
    fn legacy_procedure_rejects_ability_categories_without_a_procedure() {
        let _ = AbilityDef::spell("Draw a card.", EffectDef::None).with_legacy_procedure();
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
