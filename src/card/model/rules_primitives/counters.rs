//! Counter identities, names, and intrinsic rules families.

use super::super::KeywordAbility;

/// A named counter whose meaning is supplied by the rules or by the cards
/// that refer to that name. Counter names are open vocabulary, not an engine
/// enum: adding an ordinary named counter does not change the representation
/// of every object that can carry counters (CR 122.1).
const COUNTER_FAMILY_SHIFT: u32 = 62;
const COUNTER_PAYLOAD_MASK: u64 = (1_u64 << COUNTER_FAMILY_SHIFT) - 1;

const fn counter_name_key(name: &str) -> u64 {
    let bytes = name.as_bytes();
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    hash & COUNTER_PAYLOAD_MASK
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CounterName(u64);

impl CounterName {
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self(counter_name_key(name))
    }

    #[must_use]
    pub const fn key(self) -> u64 {
        self.0
    }
}

/// The signed power/toughness modification carried by one power/toughness
/// counter (CR 122.1a). Its polarity is retained separately so `-0/-2`
/// remains distinct from `+0/+2` even though signed integer zero has no sign.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PowerToughnessCounter {
    negative: bool,
    power: u16,
    toughness: u16,
}

impl PowerToughnessCounter {
    #[must_use]
    /// Builds one of the rules' `+X/+Y` or `-X/-Y` counters.
    ///
    /// # Panics
    ///
    /// Panics when the two nonzero components have opposing signs or either
    /// component is `i16::MIN`, which cannot be represented as a magnitude.
    pub const fn new(power: i16, toughness: i16) -> Self {
        assert!(power != i16::MIN && toughness != i16::MIN);
        let has_positive = power > 0 || toughness > 0;
        let has_negative = power < 0 || toughness < 0;
        assert!(!(has_positive && has_negative));
        Self {
            negative: has_negative,
            power: power.unsigned_abs(),
            toughness: toughness.unsigned_abs(),
        }
    }

    #[must_use]
    pub const fn bonus(self) -> (i16, i16) {
        let power = self.power.cast_signed();
        let toughness = self.toughness.cast_signed();
        if self.negative {
            (-power, -toughness)
        } else {
            (power, toughness)
        }
    }
}

/// The keyword abilities the Comprehensive Rules permit keyword counters to
/// grant (CR 122.1b). This family is deliberately closed by the rules even
/// though ordinary named counter vocabulary is open.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KeywordCounter {
    Deathtouch,
    DoubleStrike,
    FirstStrike,
    Flying,
    Haste,
    Hexproof,
    Indestructible,
    Lifelink,
    Menace,
    Reach,
    Trample,
    Vigilance,
}

impl KeywordCounter {
    pub const ALL: [Self; 12] = [
        Self::Deathtouch,
        Self::DoubleStrike,
        Self::FirstStrike,
        Self::Flying,
        Self::Haste,
        Self::Hexproof,
        Self::Indestructible,
        Self::Lifelink,
        Self::Menace,
        Self::Reach,
        Self::Trample,
        Self::Vigilance,
    ];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Deathtouch => "deathtouch",
            Self::DoubleStrike => "double strike",
            Self::FirstStrike => "first strike",
            Self::Flying => "flying",
            Self::Haste => "haste",
            Self::Hexproof => "hexproof",
            Self::Indestructible => "indestructible",
            Self::Lifelink => "lifelink",
            Self::Menace => "menace",
            Self::Reach => "reach",
            Self::Trample => "trample",
            Self::Vigilance => "vigilance",
        }
    }

    #[must_use]
    pub const fn ability(self) -> KeywordAbility {
        match self {
            Self::Deathtouch => KeywordAbility::Deathtouch,
            Self::DoubleStrike => KeywordAbility::DoubleStrike,
            Self::FirstStrike => KeywordAbility::FirstStrike,
            Self::Flying => KeywordAbility::Flying,
            Self::Haste => KeywordAbility::Haste,
            Self::Hexproof => KeywordAbility::Hexproof,
            Self::Indestructible => KeywordAbility::Indestructible,
            Self::Lifelink => KeywordAbility::Lifelink,
            Self::Menace => KeywordAbility::Menace,
            Self::Reach => KeywordAbility::Reach,
            Self::Trample => KeywordAbility::Trample,
            Self::Vigilance => KeywordAbility::Vigilance,
        }
    }

    const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }
}

include!("intrinsic_counters.rs");

/// A counter's rules-shaped identity. Power/toughness counters, keyword
/// counters, and a small closed set of named counters have intrinsic behavior;
/// card- and mechanic-defined counters remain ordinary named values.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CounterFamily {
    PowerToughness(PowerToughnessCounter),
    Keyword(KeywordCounter),
    IntrinsicNamed(IntrinsicCounter),
    Named(CounterName),
}

/// Compact counter identity stored throughout the rules AST and game state.
/// The upper bits identify the rules family; ordinary named counters use a
/// stable 62-bit key derived from their name.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CounterKind(u64);

#[allow(non_upper_case_globals)]
impl CounterKind {
    pub const PlusOnePlusOne: Self = Self::power_toughness(1, 1);
    pub const MinusOneMinusOne: Self = Self::power_toughness(-1, -1);
    pub const PlusOnePlusTwo: Self = Self::power_toughness(1, 2);
    pub const MinusZeroMinusTwo: Self = Self::power_toughness(0, -2);

    pub const Deathtouch: Self = Self::keyword(KeywordCounter::Deathtouch);
    pub const DoubleStrike: Self = Self::keyword(KeywordCounter::DoubleStrike);
    pub const FirstStrike: Self = Self::keyword(KeywordCounter::FirstStrike);
    pub const Flying: Self = Self::keyword(KeywordCounter::Flying);
    pub const Haste: Self = Self::keyword(KeywordCounter::Haste);
    pub const Hexproof: Self = Self::keyword(KeywordCounter::Hexproof);
    pub const Indestructible: Self = Self::keyword(KeywordCounter::Indestructible);
    pub const Lifelink: Self = Self::keyword(KeywordCounter::Lifelink);
    pub const Menace: Self = Self::keyword(KeywordCounter::Menace);
    pub const Reach: Self = Self::keyword(KeywordCounter::Reach);
    pub const Trample: Self = Self::keyword(KeywordCounter::Trample);
    pub const Vigilance: Self = Self::keyword(KeywordCounter::Vigilance);

    pub const Loyalty: Self = Self::intrinsic_named(IntrinsicCounter::Loyalty);
    /// The counter a Saga counts its chapters with (CR 714). Placed by the
    /// rules rather than by anything printed on the card.
    pub const Lore: Self = Self::intrinsic_named(IntrinsicCounter::Lore);
    pub const Finality: Self = Self::intrinsic_named(IntrinsicCounter::Finality);
    pub const Stun: Self = Self::intrinsic_named(IntrinsicCounter::Stun);
    pub const Poison: Self = Self::intrinsic_named(IntrinsicCounter::Poison);

    /// Whether this is the counter a Saga reads its chapters with.
    #[must_use]
    pub const fn is_lore(self) -> bool {
        self.0 == Self::Lore.0
    }

    /// The counter names currently authored in the catalog or interpreted by
    /// the engine. This is a serialization registry, not a storage layout:
    /// its order has no rules or checkpoint meaning.
    pub const KNOWN: [Self; 67] = [
        Self::PlusOnePlusOne,
        Self::Lore,
        Self::named("javelin"),
        Self::named("muster"),
        Self::named("charge"),
        Self::Loyalty,
        Self::named("spore"),
        Self::MinusOneMinusOne,
        Self::PlusOnePlusTwo,
        Self::named("credit"),
        Self::named("tide"),
        Self::MinusZeroMinusTwo,
        Self::named("time"),
        Self::named("doom"),
        Self::named("carrion"),
        Self::named("pupa"),
        Self::named("sleep"),
        Self::named("vitality"),
        Self::named("corpse"),
        Self::named("wind"),
        Self::named("storage"),
        Self::named("mining"),
        Self::named("fuse"),
        Self::named("fade"),
        Self::named("depletion"),
        Self::named("wish"),
        Self::named("level"),
        Self::Finality,
        Self::Deathtouch,
        Self::DoubleStrike,
        Self::FirstStrike,
        Self::Flying,
        Self::Haste,
        Self::Hexproof,
        Self::Indestructible,
        Self::Lifelink,
        Self::Menace,
        Self::Reach,
        Self::Trample,
        Self::Vigilance,
        Self::named("age"),
        Self::named("chorus"),
        Self::named("silver"),
        Self::Stun,
        Self::named("rev"),
        Self::named("hatchling"),
        Self::Poison,
        Self::named("energy"),
        Self::named("void"),
        Self::named("ice"),
        Self::named("story"),
        Self::named("experience"),
        Self::named("burden"),
        Self::named("luck"),
        Self::named("dread"),
        Self::named("point"),
        Self::named("landmark"),
        Self::named("skewer"),
        Self::named("soul"),
        Self::named("incubation"),
        Self::named("loot"),
        Self::named("fire"),
        Self::named("conqueror"),
        Self::named("film"),
        Self::named("invasion"),
        Self::named("growth"),
        Self::named("page"),
    ];

    const KNOWN_NAMES: [&'static str; 67] = [
        "+1/+1",
        "lore",
        "javelin",
        "muster",
        "charge",
        "loyalty",
        "spore",
        "-1/-1",
        "+1/+2",
        "credit",
        "tide",
        "-0/-2",
        "time",
        "doom",
        "carrion",
        "pupa",
        "sleep",
        "vitality",
        "corpse",
        "wind",
        "storage",
        "mining",
        "fuse",
        "fade",
        "depletion",
        "wish",
        "level",
        "finality",
        "deathtouch",
        "double strike",
        "first strike",
        "flying",
        "haste",
        "hexproof",
        "indestructible",
        "lifelink",
        "menace",
        "reach",
        "trample",
        "vigilance",
        "age",
        "chorus",
        "silver",
        "stun",
        "rev",
        "hatchling",
        "poison",
        "energy",
        "void",
        "ice",
        "story",
        "experience",
        "burden",
        "luck",
        "dread",
        "point",
        "landmark",
        "skewer",
        "soul",
        "incubation",
        "loot",
        "fire",
        "conqueror",
        "film",
        "invasion",
        "growth",
        "page",
    ];

    #[must_use]
    pub const fn named(name: &'static str) -> Self {
        Self(CounterName::new(name).key())
    }

    const fn intrinsic_named(counter: IntrinsicCounter) -> Self {
        Self::named(counter.name())
    }

    #[must_use]
    pub const fn power_toughness(power: i16, toughness: i16) -> Self {
        let counter = PowerToughnessCounter::new(power, toughness);
        Self(
            (1_u64 << COUNTER_FAMILY_SHIFT)
                | ((counter.negative as u64) << 32)
                | ((counter.power as u64) << 16)
                | counter.toughness as u64,
        )
    }

    #[must_use]
    pub const fn keyword(keyword: KeywordCounter) -> Self {
        Self((2_u64 << COUNTER_FAMILY_SHIFT) | keyword as u64)
    }

    #[must_use]
    /// Returns this key's rules family.
    ///
    /// # Panics
    ///
    /// Panics only if internal code constructs a key with an unassigned
    /// family tag. Public constructors cannot create one.
    pub const fn family(self) -> CounterFamily {
        let bytes = self.0.to_le_bytes();
        match self.0 >> COUNTER_FAMILY_SHIFT {
            0 => {
                let name = CounterName(self.0 & COUNTER_PAYLOAD_MASK);
                match IntrinsicCounter::from_name(name) {
                    Some(counter) => CounterFamily::IntrinsicNamed(counter),
                    None => CounterFamily::Named(name),
                }
            }
            1 => CounterFamily::PowerToughness(PowerToughnessCounter {
                negative: ((self.0 >> 32) & 1) != 0,
                power: u16::from_le_bytes([bytes[2], bytes[3]]),
                toughness: u16::from_le_bytes([bytes[0], bytes[1]]),
            }),
            2 => CounterFamily::Keyword(KeywordCounter::from_index(bytes[0])),
            _ => panic!("invalid counter family"),
        }
    }

    /// What one counter of this kind adds to power and toughness. The kinds
    /// that are only markers add nothing; the card putting them there gives
    /// them whatever meaning they have.
    #[must_use]
    pub const fn power_toughness_bonus(self) -> (i16, i16) {
        match self.family() {
            CounterFamily::PowerToughness(counter) => counter.bonus(),
            CounterFamily::Keyword(_)
            | CounterFamily::IntrinsicNamed(_)
            | CounterFamily::Named(_) => (0, 0),
        }
    }

    #[must_use]
    /// Returns the canonical name registered for this counter key.
    ///
    /// # Panics
    ///
    /// Panics when a named or power/toughness key has not been added to the
    /// serialization registry. Card-authored counter names must be registered
    /// before they can enter a game.
    pub fn name(self) -> &'static str {
        if let CounterFamily::Keyword(counter) = self.family() {
            return counter.name();
        }
        for (known, name) in Self::KNOWN.into_iter().zip(Self::KNOWN_NAMES) {
            if self == known {
                return name;
            }
        }
        panic!("counter name is absent from the serialization registry")
    }

    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::KNOWN_NAMES
            .into_iter()
            .position(|known| known == name)
            .map(|index| Self::KNOWN[index])
    }

    /// The keyword a keyword counter grants (CR 122.1b), if this is one.
    /// Nothing about it is a grant with a duration: the permanent has the
    /// keyword exactly while the counter is on it.
    #[must_use]
    pub const fn granted_keyword(self) -> Option<KeywordAbility> {
        match self.family() {
            CounterFamily::Keyword(counter) => Some(counter.ability()),
            CounterFamily::PowerToughness(_)
            | CounterFamily::IntrinsicNamed(_)
            | CounterFamily::Named(_) => None,
        }
    }
}
