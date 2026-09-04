use super::{
    AggregateOperationDef, AppliedEffectDef, ConditionDef, CounterKind, EffectRecipientDef,
    ManaColor, ManaCost, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, ObjectValueDef,
    PlayerRefDef, PlayerRelation, TokenCharacteristics, ValueDef, ZoneKind,
};
use crate::ids::{Binding, TargetIndex};

include!("costs/quantities.rs");

/// A cost expression shared by casting, activation, and resolving payment
/// procedures. The surrounding procedure determines who pays it, what object
/// is its source, and whether it supports the expression's required choices.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostDef {
    Mana(ManaCost),
    /// Pay the same mana cost a computed number of times. Fixed single
    /// payments should use [`Self::Mana`]; this form preserves quantities
    /// such as chosen X and the number of selected modes.
    ManaTimes {
        cost: ManaCost,
        quantity: CostQuantityDef,
    },
    /// Pay generic mana whose amount is evaluated by the surrounding
    /// resolving procedure.
    GenericMana(ValueDef),
    /// Pay a computed amount of one fixed color.
    ColoredMana {
        color: ManaColor,
        amount: ValueDef,
    },
    /// Pay a referenced object's mana cost with a generic reduction.
    ObjectManaCostReducedBy {
        object: &'static EffectRecipientDef,
        generic: u16,
    },
    /// Pay snow mana. Snow is a quality of the producing source rather than
    /// another mana type, so ordinary [`ManaCost`] cannot represent it.
    SnowMana(u16),
    /// Pay the printed mana cost of the referenced object. A binding may be
    /// supplied by another cost in this same activation, which is Back from
    /// the Brink's "exile ... and pay its mana cost" shape.
    ManaCostOf(ObjectRefDef),
    /// Pay generic mana equal to a multiple of the mana value of a target
    /// chosen for this activation. Targets are chosen before costs are paid,
    /// so the amount is fixed by the announced activation.
    ManaValueOfTarget {
        target: TargetIndex,
        multiplier: u8,
    },
    TapSource,
    UntapSource,
    SacrificeSource,
    /// Sacrifice the exact permanent named by an ability-context reference.
    ///
    /// Unlike [`Self::SacrificePermanent`], this is not a choice among
    /// matching permanents. It supports granted abilities whose cost names
    /// the object that granted them, while preserving split-control rules:
    /// the activating player must control the referenced permanent.
    SacrificeObject(ObjectRefDef),
    /// Remove counters from the permanent carrying this ability as the
    /// ability is activated. The source must carry at least `amount`; paying
    /// the cost removes them before the ability is put on the stack.
    RemoveCountersFromSource {
        kind: CounterKind,
        amount: u16,
    },
    /// Remove as many counters as the payer likes, with the number chosen
    /// as the ability is activated. The storage lands' "remove any number of
    /// storage counters" is the whole reason it exists: how many come off is
    /// how much mana comes out, so the choice cannot be made after the fact.
    ///
    /// Enumeration replaces it with a [`Self::RemoveCountersFromSource`] of
    /// the chosen size, so nothing downstream ever pays this form directly.
    RemoveAnyNumberOfCountersFromSource(CounterKind),
    /// Discard the card that carries this ability from its owner's hand.
    DiscardSource,
    /// "Discard your hand." Every card at once and no choice about which, so
    /// unlike [`Self::DiscardCards`] it needs no window to ask in -- and a
    /// player with nothing in hand pays it by discarding nothing.
    DiscardHand,
    PayLife(u16),
    /// Pay life a computed number of times.
    PayLifeTimes(CostQuantityDef),
    /// Spend a fixed amount of energy.
    Energy(u16),
    /// Put exactly this many cards from the top of the payer's library into
    /// their graveyard. Unlike milling as an effect, a cost cannot be paid
    /// partially: the library must contain the full amount before the
    /// ability can be activated.
    MillCards(u16),
    /// Exile exactly this many cards from the top of the payer's library.
    /// As a cost it is payable only when the full number is present.
    ExileTopCards(u16),
    DiscardCards(u16),
    /// Discard that many cards chosen at random from the payer's hand. Unlike
    /// [`Self::DiscardCards`] nobody chooses, so paying it needs no decision:
    /// the cards leave as the cost is paid.
    DiscardCardsAtRandom(u8),
    /// Draw cards as a cost. Some resolving costs, notably cumulative
    /// upkeep, require an action that is normally an effect.
    DrawCards(u16),
    /// Put counters on the object that carries the cost.
    PutCountersOnSource {
        kind: CounterKind,
        amount: u16,
    },
    SacrificePermanent {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    /// Sacrifice that many matching permanents, chosen one at a time as the
    /// ability is activated.
    ///
    /// Unlike [`Self::SacrificePermanent`] the choices are not enumerated
    /// into the action: Bolas's Citadel asks for ten of them, and a board of
    /// twenty would name nearly two hundred thousand ways to pay one cost.
    /// A decision bounds the same selection the way the decision model
    /// already bounds every other large one.
    SacrificePermanents {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        count: u8,
    },
    /// Sacrifice a computed number of matching permanents.
    Sacrifice {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    /// Discard a computed number of matching cards.
    Discard {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    /// Exile a computed number of matching objects from one zone.
    Exile {
        object: ObjectPredicateDef,
        from: ZoneKind,
        quantity: CostQuantityDef,
    },
    /// Return a computed number of matching permanents to hand.
    ReturnToHand {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    /// Ninjutsu's cost: return an unblocked attacker you control to its
    /// owner's hand. Which one is chosen as the ability is activated, and
    /// what makes a creature eligible is combat state rather than any
    /// printed characteristic -- so unlike the costs above this one names no
    /// predicate at all.
    ReturnUnblockedAttackerToHand,
    /// Tap exactly that many matching untapped permanents, chosen one at a
    /// time as the ability is activated. A single payer is carried directly
    /// by the activation so mana planning can reserve it; larger quotas use a
    /// bounded decision instead of enumerating every combination. The source
    /// may pay when it remains untapped after every other cost. Unlike crew,
    /// the quota counts permanents rather than total power.
    TapPermanents {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        count: u8,
    },
    /// Tap a computed number of matching untapped permanents.
    Tap {
        object: ObjectPredicateDef,
        quantity: CostQuantityDef,
    },
    ExileSource,
    /// Exert the permanent carrying this ability (CR 701.39): it will not
    /// untap during its controller's next untap step. Unlike tapping it,
    /// exerting costs nothing now and everything next turn, and unlike a
    /// sacrifice the permanent is still there in the meantime.
    ExertSource,
    /// Return the permanent carrying this ability to its owner's hand. Like
    /// a sacrifice the source leaves the battlefield to pay, but unlike one
    /// it comes back to be cast again, which is the whole shape of
    /// Attunement: the card is the cost and the card is reusable.
    ReturnSourceToHand,
    /// Discard a matching card from the payer's own hand, chosen as the
    /// ability is activated. Unlike [`Self::DiscardCards`] the card travels
    /// with the activation rather than being counted, which is what "discard
    /// a card" and "discard a land card" both need.
    DiscardCardMatching(ObjectPredicateDef),
    /// Reveal one matching card from the payer's hand as the ability is
    /// activated, without moving it. The chosen object travels with the
    /// activation so its name can be read during resolution.
    RevealCardFromHand(ObjectPredicateDef),
    /// Exile a matching card from the payer's own hand. Unlike discarding,
    /// the card never enters a graveyard; Cadaverous Bloom is the canonical
    /// mana-ability use.
    ExileCardFromHand(ObjectPredicateDef),
    /// Choose matching objects and move them as the cost is paid.
    MoveToZone(MoveToZoneCostDef),
    /// Crew's and saddle's cost: tap any number of other untapped creatures
    /// you control whose power adds up to at least this much (CR 702.122a,
    /// CR 702.166a). Which creatures pay is chosen one at a time as the
    /// ability is activated, for the same reason a multiple sacrifice is: a
    /// board of ten creatures names a thousand ways to pay one cost.
    TapCreaturesWithTotalPower {
        minimum: u8,
    },
    /// Sacrifice creatures until their combined power reaches this minimum.
    SacrificeCreaturesWithTotalPower(u16),
    /// Add or remove that many loyalty counters. A planeswalker's abilities
    /// are the only costs paid this way, and paying one is what makes them
    /// once per turn at sorcery speed.
    Loyalty(i8),
    /// Add mana to the payer's pool as a cost action.
    AddMana(&'static AddManaEffectDef),
    /// Have a player related to the payer gain life.
    GainLife {
        player: PlayerRelation,
        amount: u16,
    },
    /// Have a player related to the payer create tokens.
    CreateTokens {
        player: PlayerRelation,
        token: &'static TokenCharacteristics,
        amount: u16,
    },
    /// Gain control of matching permanents not already controlled by the
    /// payer.
    GainControlPermanents {
        object: ObjectPredicateDef,
        amount: u16,
    },
    /// Have the payer flip this many coins.
    FlipCoins(u16),
    /// Choose a positive generic-mana amount during payment.
    ChosenGenericMana,
    /// Choose an energy amount during payment.
    ChosenEnergy,
    /// Remove a positive chosen number of counters from a referenced object.
    RemoveAnyNumberOfCounters {
        object: &'static EffectRecipientDef,
        kind: CounterKind,
    },
    /// Move one matching permanent the payer controls to a named zone.
    MovePermanentMatching {
        object: ObjectPredicateDef,
        zone: ZoneKind,
    },
    /// Discard one matching card as part of a resolving payment.
    DiscardMatching(ObjectPredicateDef),
    /// Sacrifice one matching permanent as part of a resolving payment.
    SacrificePermanentMatching(ObjectPredicateDef),
    /// Forage (CR 701.59): exile three cards from the graveyard or sacrifice
    /// a Food.
    Forage,
    /// Pay every child cost as one cost expression.
    All(&'static [CostDef]),
    /// Choose exactly one child cost to pay.
    Choice(&'static [CostDef]),
    Special(&'static str),
}

impl CostDef {
    #[must_use]
    pub const fn mana(cost: ManaCost) -> Self {
        Self::Mana(cost)
    }

    #[must_use]
    pub const fn life(amount: u16) -> Self {
        Self::PayLife(amount)
    }

    #[must_use]
    pub const fn snow_mana(amount: u16) -> Self {
        Self::SnowMana(amount)
    }

    #[must_use]
    pub const fn draw_cards(amount: u16) -> Self {
        Self::DrawCards(amount)
    }

    #[must_use]
    pub const fn discard_cards(amount: u16) -> Self {
        Self::DiscardCards(amount)
    }

    #[must_use]
    pub const fn put_counters_on_source(kind: CounterKind, amount: u16) -> Self {
        Self::PutCountersOnSource { kind, amount }
    }

    #[must_use]
    pub const fn sacrifice_permanents(
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        count: u8,
    ) -> Self {
        Self::SacrificePermanents {
            object,
            controller,
            count,
        }
    }

    #[must_use]
    pub const fn exile_top_cards(amount: u16) -> Self {
        Self::ExileTopCards(amount)
    }

    #[must_use]
    pub const fn add_mana(effect: &'static AddManaEffectDef) -> Self {
        Self::AddMana(effect)
    }

    #[must_use]
    pub const fn gain_life(player: PlayerRelation, amount: u16) -> Self {
        Self::GainLife { player, amount }
    }

    #[must_use]
    pub const fn create_tokens(
        player: PlayerRelation,
        token: &'static TokenCharacteristics,
        amount: u16,
    ) -> Self {
        Self::CreateTokens {
            player,
            token,
            amount,
        }
    }

    #[must_use]
    pub const fn gain_control_permanents(object: ObjectPredicateDef, amount: u16) -> Self {
        Self::GainControlPermanents { object, amount }
    }

    #[must_use]
    pub const fn flip_coins(amount: u16) -> Self {
        Self::FlipCoins(amount)
    }

    #[must_use]
    pub const fn pay_mana(cost: ManaCost) -> Self {
        Self::Mana(cost)
    }

    #[must_use]
    pub const fn pay_mana_times(cost: ManaCost, quantity: CostQuantityDef) -> Self {
        match quantity {
            CostQuantityDef::Fixed(1) => Self::Mana(cost),
            _ => Self::ManaTimes { cost, quantity },
        }
    }

    #[must_use]
    pub const fn pay_life(quantity: CostQuantityDef) -> Self {
        match quantity {
            CostQuantityDef::Fixed(amount) => Self::PayLife(amount as u16),
            _ => Self::PayLifeTimes(quantity),
        }
    }

    #[must_use]
    pub const fn sacrifice(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Sacrifice { object, quantity }
    }

    #[must_use]
    pub const fn discard(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Discard { object, quantity }
    }

    #[must_use]
    pub const fn exile(
        object: ObjectPredicateDef,
        from: ZoneKind,
        quantity: CostQuantityDef,
    ) -> Self {
        Self::Exile {
            object,
            from,
            quantity,
        }
    }

    #[must_use]
    pub const fn return_to_hand(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::ReturnToHand { object, quantity }
    }

    #[must_use]
    pub const fn tap(object: ObjectPredicateDef, quantity: CostQuantityDef) -> Self {
        Self::Tap { object, quantity }
    }

    #[must_use]
    pub const fn forage() -> Self {
        Self::Forage
    }

    #[must_use]
    pub const fn all(costs: &'static [Self]) -> Self {
        Self::All(costs)
    }

    #[must_use]
    pub const fn choice(costs: &'static [Self]) -> Self {
        Self::Choice(costs)
    }
}

/// Const-friendly storage for activated-ability costs.
///
/// Most card definitions borrow a promoted slice. Common constructors whose
/// costs include a parameter, such as Bloodrush's mana cost, can instead own a
/// small inline list without introducing a mechanic-specific cost primitive.
#[derive(Clone, Copy, Debug)]
pub struct AbilityCostList(AbilityCostStorage);

#[derive(Clone, Copy, Debug)]
enum AbilityCostStorage {
    Borrowed(&'static [CostDef]),
    One([CostDef; 1]),
    Two([CostDef; 2]),
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
    pub(crate) const fn borrowed(costs: &'static [CostDef]) -> Self {
        Self(AbilityCostStorage::Borrowed(costs))
    }

    #[must_use]
    pub(crate) const fn one(cost: CostDef) -> Self {
        Self(AbilityCostStorage::One([cost]))
    }

    #[must_use]
    pub(crate) const fn two(first: CostDef, second: CostDef) -> Self {
        Self(AbilityCostStorage::Two([first, second]))
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[CostDef] {
        match &self.0 {
            AbilityCostStorage::Borrowed(costs) => costs,
            AbilityCostStorage::One(costs) => costs,
            AbilityCostStorage::Two(costs) => costs,
        }
    }

    #[must_use]
    pub fn contains(&self, cost: &CostDef) -> bool {
        self.as_slice().contains(cost)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, CostDef> {
        self.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a AbilityCostList {
    type Item = &'a CostDef;
    type IntoIter = std::slice::Iter<'a, CostDef>;

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

/// Every land subtype in CR 205.3i. Effects that remove "all land types"
/// use this vocabulary while leaving creature, artifact, enchantment, and
/// other subtype families untouched.
pub const LAND_SUBTYPES: &[&str] = &[
    "Cave",
    "Desert",
    "Forest",
    "Gate",
    "Island",
    "Lair",
    "Locus",
    "Mine",
    "Mountain",
    "Plains",
    "Planet",
    "Power-Plant",
    "Sphere",
    "Swamp",
    "Tower",
    "Town",
    "Urza's",
    "Urza’s",
];

/// Every nonbasic land subtype in [`LAND_SUBTYPES`]. Characteristic-defining
/// abilities such as Planar Nexus use this vocabulary in every zone.
pub const NONBASIC_LAND_SUBTYPES: &[&str] = &[
    "Cave",
    "Desert",
    "Gate",
    "Lair",
    "Locus",
    "Mine",
    "Planet",
    "Power-Plant",
    "Sphere",
    "Tower",
    "Town",
    "Urza's",
    "Urza’s",
];

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

    /// This type as a reusable one-element static slice, for declarative
    /// predicates and effects whose shape accepts a set of land types.
    #[must_use]
    pub const fn singleton(self) -> &'static [Self] {
        match self {
            Self::Plains => &[Self::Plains],
            Self::Island => &[Self::Island],
            Self::Swamp => &[Self::Swamp],
            Self::Mountain => &[Self::Mountain],
            Self::Forest => &[Self::Forest],
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

/// One mana type read by an effect. A chosen color belongs to the source
/// permanent, so the same value can feed a mana effect or a continuous
/// characteristic operation without either one owning the choice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaTypeDef {
    Fixed(ManaColor),
    ChosenColor,
}

/// Where a mana-selection domain obtains its types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaTypeSourceDef {
    Fixed(&'static [ManaColor]),
    /// The distinct mana types actually produced by the referenced object's
    /// mana ability in the event being processed. This is deliberately not a
    /// "could produce" query: Mana Flare follows the activation's output.
    ProducedBy(ObjectRefDef),
    /// The union of mana types the referenced permanents' mana abilities
    /// could produce under CR 106.7. Costs and restrictions on spending that
    /// mana do not narrow the answer.
    CouldBeProducedBy(ObjectSetDef),
}

/// A filter applied after a mana-type source has been evaluated. "Any color"
/// excludes colorless, while "any type" includes it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaTypeFilterDef {
    AnyType,
    Colors,
}

/// A domain from which a mana effect can select types. Its source, filter,
/// and selection procedure are independent so cards can compose exactly the
/// rule they print.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaTypeSetDef {
    pub source: ManaTypeSourceDef,
    pub filter: ManaTypeFilterDef,
}

impl ManaTypeSetDef {
    #[must_use]
    pub const fn fixed(types: &'static [ManaColor]) -> Self {
        Self {
            source: ManaTypeSourceDef::Fixed(types),
            filter: ManaTypeFilterDef::AnyType,
        }
    }

    #[must_use]
    pub const fn produced_by(object: ObjectRefDef) -> Self {
        Self {
            source: ManaTypeSourceDef::ProducedBy(object),
            filter: ManaTypeFilterDef::AnyType,
        }
    }

    #[must_use]
    pub const fn could_be_produced_by(objects: ObjectSetDef) -> Self {
        Self {
            source: ManaTypeSourceDef::CouldBeProducedBy(objects),
            filter: ManaTypeFilterDef::AnyType,
        }
    }

    #[must_use]
    pub const fn colors_only(mut self) -> Self {
        self.filter = ManaTypeFilterDef::Colors;
        self
    }
}

/// Which kind of mana an effect adds. A choice is made as the mana ability
/// resolves; it is not modeled as several interchangeable colors already in
/// the pool.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSelectionDef {
    One(ManaTypeDef),
    /// One colour picked from a list, with the whole amount in that colour.
    /// A dual land offers "add {W} or {U}", not a mixture.
    Choice(ManaTypeSetDef),
    /// Every unit chosen independently from a list, which is what "in any
    /// combination of" means. Each way of splitting the amount is a separate
    /// activation, the way a counter size or a sacrificed permanent already
    /// is: a mana ability has no window in which to ask afterwards.
    Combination(ManaTypeSetDef),
    /// Choose one complete mana bundle. Unlike [`Self::Choice`], alternatives
    /// may contain different amounts and more than one mana type.
    ChoiceOfBundles(&'static [super::ManaSplit]),
    /// One colour picked from among the colours of the cards this permanent
    /// exiled. Imprint is the only clause that says this, and it cannot be a
    /// list: which colours the ability makes is decided by what was imprinted
    /// on it, so the list is read off the board as the ability is offered.
    /// A permanent that imprinted nothing, or imprinted a colourless card,
    /// produces nothing at all.
    ColorsOfLinkedExiles,
}

/// A restriction carried by produced mana until that mana is spent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaRestrictionDef {
    CastSpell(ObjectPredicateDef),
    /// "This mana can't be spent to cast nonartifact spells." A prohibition
    /// rather than a permission: unlike [`Self::CastSpell`] every other use
    /// stays open, so a Powerstone's mana still activates abilities and pays
    /// for anything that is not a cast at all.
    CannotCastSpell(ObjectPredicateDef),
    CastCreatureSpellOfChosenType,
    ActivateAbility(ObjectPredicateDef),
    /// This mana can be spent only on a cumulative-upkeep payment.
    CumulativeUpkeep,
    Special(&'static str),
}

/// An effect applied to the spell or ability paid for with a mana unit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSpendEffectDef {
    ApplyToPaidSpell(AppliedEffectDef),
    /// "If that mana is spent on a creature spell, it gains haste until end
    /// of turn." The same rider with a question in front of it: what the
    /// mana pays for decides whether the effect applies at all. An effect
    /// that grants an ability keeps applying to the permanent a paid
    /// permanent spell becomes, which is the only reason a land prints this.
    ApplyToPaidSpellMatching {
        object: ObjectPredicateDef,
        effect: AppliedEffectDef,
    },
    ApplyToPaidAbility(AppliedEffectDef),
    Special(&'static str),
}

/// One set of indistinguishable mana units created by an effect. The runtime
/// pool may store `amount` as a count keyed by the remaining fields.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AddManaEffectDef {
    pub mana: ManaSelectionDef,
    pub amount: u16,
    /// One further mana of a second colour, produced by the same activation.
    /// "Add {W}{U}" is one ability making two unlike mana, which `mana` and
    /// `amount` between them cannot say: they describe a run of identical
    /// units.
    pub also: Option<ManaColor>,
    pub restrictions: &'static [ManaRestrictionDef],
    pub spend_effects: &'static [ManaSpendEffectDef],
    /// Damage the source deals to its controller as this mana ability
    /// resolves. This is damage rather than a life-payment cost, so ordinary
    /// damage prevention and source attribution still apply.
    pub damage_to_controller: u16,
    /// Who the mana goes to. Almost always the ability's own controller, but
    /// a trigger watching everyone's lands says "its controller", meaning the
    /// player whose land was tapped rather than the watcher.
    pub recipient: PlayerRefDef,
    /// An amount read off the board when the ability is offered, for "add
    /// one for each counter on this creature". Resolved before the activation
    /// is built, so every later reader still sees a plain number.
    pub variable_amount: Option<ValueDef>,
    /// A larger amount that replaces [`Self::amount`] while its condition
    /// holds, for "add {C}. If you control ..., add {C}{C} instead". The
    /// colour does not change, so this is an amount rather than a second
    /// mana selection.
    pub amount_override: Option<ManaAmountOverrideDef>,
    /// "If there are no mining counters on this land, sacrifice it." The
    /// check belongs to this ability's own resolution, which is why it is a
    /// rider here rather than a state trigger: a mana ability never uses the
    /// stack, and a land that spends its last counter is gone before anyone
    /// could respond. It also means nothing happens when some other effect
    /// takes the counters away, which is what the printed card says.
    pub sacrifice_source_when_out_of: Option<CounterKind>,
}

/// "... add this much instead."
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaAmountOverrideDef {
    pub condition: ConditionDef,
    pub amount: u16,
}

impl AddManaEffectDef {
    #[must_use]
    pub const fn one(mana: ManaColor) -> Self {
        Self::one_of_type(ManaTypeDef::Fixed(mana))
    }

    #[must_use]
    pub const fn one_of_type(mana: ManaTypeDef) -> Self {
        Self {
            mana: ManaSelectionDef::One(mana),
            amount: 1,
            also: None,
            restrictions: &[],
            spend_effects: &[],
            damage_to_controller: 0,
            recipient: PlayerRefDef::EffectController,
            variable_amount: None,
            amount_override: None,
            sacrifice_source_when_out_of: None,
        }
    }

    /// One mana of each of two colours, from one activation. The filter
    /// lands print exactly this and nothing else does.
    #[must_use]
    pub const fn one_of_each(first: ManaColor, second: ManaColor) -> Self {
        let mut effect = Self::one(first);
        effect.also = Some(second);
        effect
    }

    /// "Add X mana in any combination of these colours."
    #[must_use]
    pub const fn combination(mana: &'static [ManaColor], amount: u16) -> Self {
        Self::combination_from(ManaTypeSetDef::fixed(mana), amount)
    }

    #[must_use]
    pub const fn combination_from(mana: ManaTypeSetDef, amount: u16) -> Self {
        Self {
            mana: ManaSelectionDef::Combination(mana),
            amount,
            also: None,
            restrictions: &[],
            spend_effects: &[],
            damage_to_controller: 0,
            recipient: PlayerRefDef::EffectController,
            variable_amount: None,
            amount_override: None,
            sacrifice_source_when_out_of: None,
        }
    }

    /// "Add one mana of any of the exiled card's colors."
    #[must_use]
    pub const fn colors_of_linked_exiles() -> Self {
        Self {
            mana: ManaSelectionDef::ColorsOfLinkedExiles,
            amount: 1,
            also: None,
            restrictions: &[],
            spend_effects: &[],
            damage_to_controller: 0,
            recipient: PlayerRefDef::EffectController,
            variable_amount: None,
            amount_override: None,
            sacrifice_source_when_out_of: None,
        }
    }

    #[must_use]
    pub const fn choice(mana: &'static [ManaColor]) -> Self {
        Self::choice_from(ManaTypeSetDef::fixed(mana))
    }

    #[must_use]
    pub const fn choice_of_bundles(bundles: &'static [super::ManaSplit]) -> Self {
        Self {
            mana: ManaSelectionDef::ChoiceOfBundles(bundles),
            amount: 0,
            also: None,
            restrictions: &[],
            spend_effects: &[],
            damage_to_controller: 0,
            recipient: PlayerRefDef::EffectController,
            variable_amount: None,
            amount_override: None,
            sacrifice_source_when_out_of: None,
        }
    }

    #[must_use]
    pub const fn choice_from(mana: ManaTypeSetDef) -> Self {
        Self {
            mana: ManaSelectionDef::Choice(mana),
            amount: 1,
            also: None,
            restrictions: &[],
            spend_effects: &[],
            damage_to_controller: 0,
            recipient: PlayerRefDef::EffectController,
            variable_amount: None,
            amount_override: None,
            sacrifice_source_when_out_of: None,
        }
    }

    /// "... If you control ..., add this much instead."
    #[must_use]
    pub const fn with_variable_amount(mut self, amount: ValueDef) -> Self {
        self.variable_amount = Some(amount);
        self
    }

    #[must_use]
    pub const fn with_amount_override(mut self, condition: ConditionDef, amount: u16) -> Self {
        self.amount_override = Some(ManaAmountOverrideDef { condition, amount });
        self
    }

    #[must_use]
    pub const fn any_color() -> Self {
        Self::choice(&ManaColor::COLORS)
    }

    /// Spends the source when the named counter runs out. See
    /// [`Self::sacrifice_source_when_out_of`] for why this rides the ability
    /// rather than triggering off the empty permanent.
    #[must_use]
    pub const fn sacrificing_source_when_out_of(mut self, kind: CounterKind) -> Self {
        self.sacrifice_source_when_out_of = Some(kind);
        self
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

    #[must_use]
    pub const fn with_damage_to_controller(mut self, amount: u16) -> Self {
        self.damage_to_controller = amount;
        self
    }

    /// Sends the mana to the controller of the object that triggered the
    /// ability, which is what "its controller adds" asks for when the trigger
    /// watches lands nobody in particular controls.
    #[must_use]
    pub const fn to_triggering_objects_controller(mut self) -> Self {
        self.recipient = PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject);
        self
    }
}
