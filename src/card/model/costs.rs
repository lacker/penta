use super::{
    AppliedEffectDef, CounterKind, ManaColor, ManaCost, ObjectPredicateDef, PlayerRelation,
};

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
    pub const fn any_color() -> Self {
        Self::choice(&ManaColor::COLORS)
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
