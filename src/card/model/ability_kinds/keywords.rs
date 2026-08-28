// The keyword vocabulary itself: which keywords exist, the qualities the
// parameterized ones name, and the dense index that lets a set of the simple
// ones fit in a bitmask.
//
// Separated from the ability shapes next door because this answers a different
// question: those say how a clause executes, while these are the words a
// clause can be. Included textually into `ability_kinds.rs`, so the paths and
// imports here are the parent module's.

/// The quality a "bands with other" ability names.
///
/// Each printed quality is its own variant rather than a free-form predicate,
/// the way protection is one keyword per color: the checkpoint wire names them
/// individually, and only two have ever been printed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BandingQuality {
    /// "bands with other legendary creatures", which the five Legends lands
    /// grant to legendary creatures of their own color.
    LegendaryCreatures,
    /// "bands with other creatures named Wolves of the Hunt", printed on the
    /// tokens Master of the Hunt makes.
    WolvesOfTheHunt,
}

/// The number of time counters a suspend special action places.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SuspendTimeDef {
    Fixed(u16),
    /// The X paid in the suspend cost. Some printed instances prohibit zero.
    ChosenX { minimum: u16 },
}

/// The forms of suspend represented by one keyword ability (CR 702.62a).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SuspendAbilityDef {
    /// A printed hand action, including the time counters and cost it names.
    Hand {
        time: SuspendTimeDef,
        /// Kept by reference so adding a parameterized keyword does not inflate
        /// every predicate, trigger, and effect that can mention a keyword.
        cost: &'static ManaCost,
    },
    /// Suspend granted to a card already in exile. It has no hand action of
    /// its own; the granting effect separately moves the card and adds time
    /// counters.
    Granted,
}

impl SuspendAbilityDef {
    #[must_use]
    pub const fn fixed(time: u16, cost: &'static ManaCost) -> Self {
        Self::Hand {
            time: SuspendTimeDef::Fixed(time),
            cost,
        }
    }

    #[must_use]
    pub const fn chosen_x(cost: &'static ManaCost, minimum: u16) -> Self {
        Self::Hand {
            time: SuspendTimeDef::ChosenX { minimum },
            cost,
        }
    }

    #[must_use]
    pub const fn granted() -> Self {
        Self::Granted
    }
}

impl BandingQuality {
    /// Every printed quality, for the rules that have to try each one.
    pub const ALL: [Self; 2] = [Self::LegendaryCreatures, Self::WolvesOfTheHunt];

    /// What a creature must be to join a band formed on this quality.
    #[must_use]
    pub const fn predicate(self) -> &'static ObjectPredicateDef {
        match self {
            Self::LegendaryCreatures => &LEGENDARY_CREATURE,
            Self::WolvesOfTheHunt => &WOLF_OF_THE_HUNT,
        }
    }
}

static LEGENDARY_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
]);

static WOLF_OF_THE_HUNT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Named("Wolves of the Hunt"),
]);

/// A keyword ability carried as an ordinary, ordered rules clause.
///
/// The clause's [`AbilityCoverageDef`] says whether the engine currently
/// executes the keyword. This keeps unimplemented keywords such as banding
/// visible and accurately reflected in aggregate coverage without hiding them
/// in card-level booleans.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeywordAbility {
    /// CR 702.51. Each untapped creature the caster taps while paying for the
    /// spell pays for one generic mana or one mana of that creature's color.
    Convoke,
    /// CR 702.66. Each card exiled from the caster's graveyard while paying
    /// for the spell pays for one generic mana.
    Delve,
    /// CR 702.126. Each untapped artifact the caster taps while paying for
    /// the spell pays for one generic mana.
    Improvise,
    Flying,
    Trample,
    Haste,
    FirstStrike,
    DoubleStrike,
    Banding,
    /// CR 702.21j. Banding narrowed to a quality: the band's members must all
    /// have that quality, and at least one of them must have this ability.
    /// Unlike plain banding there is no free passenger, and the damage rule
    /// wants two qualifying creatures rather than one.
    BandsWithOther(BandingQuality),
    Vigilance,
    Defender,
    Deathtouch,
    Lifelink,
    Reach,
    Flash,
    Hexproof,
    Shroud,
    /// Unleash. The engine implements both halves: an optional +1/+1 counter
    /// as the permanent enters, and no blocking while it carries one.
    Unleash,
    Intimidate,
    /// CR 702.27. The creature can block or be blocked by only creatures
    /// with shadow. Both halves of one keyword: a creature with it is as
    /// unable to block the ordinary board as the ordinary board is unable to
    /// block it, which is what makes shadow a separate battlefield rather
    /// than plain evasion.
    Shadow,
    /// CR 702.110. The creature cannot be blocked except by two or more
    /// creatures. A constraint on the completed declaration rather than on
    /// any one block: the first blocker is always legal, and it is finishing
    /// with exactly one that the rules forbid.
    Menace,
    Undying,
    /// CR 702.79. Undying's mirror: a creature that dies with no -1/-1
    /// counter on it comes back with one, so it returns once and comes back
    /// smaller rather than bigger.
    Persist,
    Indestructible,
    /// "Attacks each combat if able." Not a printed keyword, but it behaves
    /// like one: a static requirement with no parameters that several cards
    /// state in the same words.
    AttacksEachCombatIfAble,
    /// CR 702.14. One keyword parameterized by land type: the creature cannot
    /// be blocked as long as the defending player controls a land of that
    /// type. The printed variants differ only in which type they name.
    Landwalk(BasicLandType),
    /// Landwalk naming a land supertype.
    LegendaryLandwalk,
    /// CR 702.16. Protection is one keyword per quality, represented by the
    /// predicate sources with that quality satisfy. Colors, card types,
    /// subtypes, spell status, controllers, and compositions of those all use
    /// the same D/E/B/T rules rather than growing parallel keyword variants.
    ProtectionFrom(&'static ObjectPredicateDef),
    /// CR 702.90. Infect changes what this source's damage does rather than
    /// how much it deals: to a player it gives that many poison counters,
    /// and to a creature it puts that many -1/-1 counters on it. Neither is
    /// damage, so neither marks the creature or costs the player life.
    Infect,
    /// CR 702.114. Devoid is a characteristic-defining ability rather than
    /// a behaviour: the object simply has no color. That is expressed by the
    /// card's printed color set being empty, so what this variant adds is
    /// the printed keyword itself -- the name and its reminder text.
    Devoid,
    /// CR 702.19. As long as a spell with split second is on the stack,
    /// players can't cast spells or activate abilities that aren't mana
    /// abilities. It is read off the spell rather than off any permanent,
    /// which is why it carries no index: nothing grants or removes it.
    SplitSecond,
    /// CR 702.150. A permanent cast using life for Phyrexian symbols enters
    /// with two fewer loyalty counters per such symbol. The payment count is
    /// recorded on the spell rather than inferred from mana spent.
    Compleated,
    /// CR 702.62. A special action from hand plus two triggered abilities in
    /// exile. The parameter owns both the time-counter count and its cost.
    Suspend(SuspendAbilityDef),
}

impl KeywordAbility {
    /// A dense index for the keywords that carry no parameter, so a set of
    /// them fits in a bitmask. Protection is excluded: it is really one
    /// keyword per quality, and the qualities are open-ended.
    #[must_use]
    pub const fn simple_index(self) -> Option<u32> {
        Some(match self {
            Self::Convoke => 30,
            Self::Delve => 32,
            Self::Improvise => 33,
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
            Self::Menace => 15,
            Self::AttacksEachCombatIfAble => 16,
            // 17 and 27 were the old dense bits for protection from creatures
            // and multicolored. Protection qualities are predicates now and
            // deliberately remain outside the simple-keyword bitset.
            Self::Indestructible => 18,
            Self::Shroud => 19,
            Self::Unleash => 26,
            // One index per land type, so a set of landwalks still packs into
            // the same bitmask as the parameterless keywords.
            Self::Landwalk(BasicLandType::Plains) => 20,
            Self::Landwalk(BasicLandType::Island) => 21,
            Self::Landwalk(BasicLandType::Swamp) => 22,
            Self::Landwalk(BasicLandType::Mountain) => 23,
            Self::Landwalk(BasicLandType::Forest) => 24,
            Self::LegendaryLandwalk => 25,
            Self::Devoid => 28,
            Self::Infect => 29,
            Self::Compleated => 31,
            Self::Shadow => 34,
            Self::Persist => 35,
            Self::ProtectionFrom(_)
            // Never granted, never removed, and never asked about as part of
            // a set: split second is read off the one spell that has it.
            | Self::SplitSecond
            | Self::Suspend(_)
            | Self::BandsWithOther(_) => return None,
        })
    }
}
