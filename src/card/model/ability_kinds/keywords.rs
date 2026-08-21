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
    /// CR 702.110. The creature cannot be blocked except by two or more
    /// creatures. A constraint on the completed declaration rather than on
    /// any one block: the first blocker is always legal, and it is finishing
    /// with exactly one that the rules forbid.
    Menace,
    Undying,
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
    ProtectionFrom(ManaColor),
    /// CR 702.16. Protection is really one keyword per quality, and a quality
    /// need not be a color: "protection from Zombies" names a creature type
    /// and behaves identically otherwise.
    ProtectionFromCreatureType(ProtectedCreatureType),
    /// Protection from the card type, which is one quality rather than a
    /// family of them and so carries no parameter at all.
    ProtectionFromCreatures,
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
    /// CR 702.150. A permanent cast using life for Phyrexian symbols enters
    /// with two fewer loyalty counters per such symbol. The payment count is
    /// recorded on the spell rather than inferred from mana spent.
    Compleated,
    /// Protection from every object of two or more colors. Not the union of
    /// the five color qualities: a monocolored source gets through, and a
    /// two-color one is stopped even where neither of its colors alone would
    /// have been.
    ProtectionFromMulticolored,
}

/// The creature types a printed protection clause names. A closed set for the
/// same reason [`BasicLandType`] is: every consumer that has to name one --
/// the checkpoint tag among them -- stays exhaustive, and a new printing adds
/// one variant here rather than an open string nobody can round-trip.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProtectedCreatureType {
    Zombie,
    Vampire,
    Werewolf,
}

impl ProtectedCreatureType {
    /// The subtype string as it appears on a creature's type line.
    #[must_use]
    pub const fn subtype(self) -> &'static str {
        match self {
            Self::Zombie => "Zombie",
            Self::Vampire => "Vampire",
            Self::Werewolf => "Werewolf",
        }
    }
}

impl KeywordAbility {
    /// A dense index for the keywords that carry no parameter, so a set of
    /// them fits in a bitmask. Protection is excluded: it is really one
    /// keyword per quality, and the qualities are open-ended.
    #[must_use]
    pub const fn simple_index(self) -> Option<u32> {
        Some(match self {
            Self::Convoke => 30,
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
            Self::ProtectionFromCreatures => 17,
            Self::ProtectionFromMulticolored => 27,
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
            Self::ProtectionFrom(_)
            | Self::ProtectionFromCreatureType(_)
            | Self::BandsWithOther(_) => return None,
        })
    }
}
