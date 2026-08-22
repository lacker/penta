use super::{
    AppliedEffectDef, ConditionDef, CounterKind, ManaColor, ManaCost, ObjectPredicateDef,
    ObjectRefDef, PlayerRefDef, PlayerRelation, ValueDef,
};

/// One atomic cost. The surrounding rules procedure determines who pays it
/// and what object, if any, is the source.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostDef {
    Mana(ManaCost),
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
    PayLife(u16),
    DiscardCards(u8),
    /// Discard that many cards chosen at random from the payer's hand. Unlike
    /// [`Self::DiscardCards`] nobody chooses, so paying it needs no decision:
    /// the cards leave as the cost is paid.
    DiscardCardsAtRandom(u8),
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
    /// Ninjutsu's cost: return an unblocked attacker you control to its
    /// owner's hand. Which one is chosen as the ability is activated, and
    /// what makes a creature eligible is combat state rather than any
    /// printed characteristic -- so unlike the costs above this one names no
    /// predicate at all.
    ReturnUnblockedAttackerToHand,
    /// Tap a chosen untapped permanent other than the source, for "tap an
    /// untapped Gate you control". Unlike [`Self::TapSource`] the permanent
    /// paying is selected when the ability is activated.
    TapPermanent {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
    },
    ExileSource,
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
    /// Exile a matching card from the payer's own hand. Unlike discarding,
    /// the card never enters a graveyard; Cadaverous Bloom is the canonical
    /// mana-ability use.
    ExileCardFromHand(ObjectPredicateDef),
    /// Exile a matching card from the controller's own graveyard. The card is
    /// chosen when the cost is paid, so it travels with the action rather
    /// than being a target.
    /// Exile `count` matching cards from the activating player's graveyard.
    /// Most printed forms take one; Grim Lavamancer takes two, and the player
    /// chooses which, so every combination is its own offered activation.
    ExileCardsFromGraveyard {
        object: ObjectPredicateDef,
        count: u8,
    },
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
    One([AbilityCostDef; 1]),
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
    pub(crate) const fn one(cost: AbilityCostDef) -> Self {
        Self(AbilityCostStorage::One([cost]))
    }

    #[must_use]
    pub(crate) const fn two(first: AbilityCostDef, second: AbilityCostDef) -> Self {
        Self(AbilityCostStorage::Two([first, second]))
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[AbilityCostDef] {
        match &self.0 {
            AbilityCostStorage::Borrowed(costs) => costs,
            AbilityCostStorage::One(costs) => costs,
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
    /// One colour picked from a list, with the whole amount in that colour.
    /// A dual land offers "add {W} or {U}", not a mixture.
    Choice(&'static [ManaColor]),
    /// Every unit chosen independently from a list, which is what "in any
    /// combination of" means. Each way of splitting the amount is a separate
    /// activation, the way a counter size or a sacrificed permanent already
    /// is: a mana ability has no window in which to ask afterwards.
    Combination(&'static [ManaColor]),
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

    #[must_use]
    pub const fn choice(mana: &'static [ManaColor]) -> Self {
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
