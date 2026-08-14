use crate::ids::TargetIndex;

use super::{
    BasicLandType, CardSet, CardSupertype, CardType, KeywordAbility, ManaColor, PlayerRelation,
    ValueDef, ZoneKind,
};

/// A composable predicate over a card or game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectPredicateDef {
    Any,
    Source,
    /// The object this ability's source is currently attached to.
    AttachedToSource,
    /// Whether the object is a token rather than a card represented by a
    /// physical printing. Negate this for the common "nontoken" qualifier.
    Token,
    /// The permanent is currently tapped. Only a battlefield object can be,
    /// so this never matches a card in another zone.
    Tapped,
    HasType(CardType),
    /// A land with at least one of the listed effective basic land subtypes.
    ///
    /// This uses the object's prospective/effective type line, so continuous
    /// effects such as Blood Moon and Nylea's Presence participate.
    HasAnyBasicLandType(&'static [BasicLandType]),
    Spell,
    NoncreatureSpell,
    Color(ManaColor),
    /// Has exactly this many colors. Zero matches colorless objects and one
    /// matches monocolored objects.
    ColorCount(u8),
    Subtype(&'static str),
    /// Mana value at most this much, for "with mana value N or less".
    ManaValueAtMost(u8),
    /// Mana value exactly this much, where the number is read off the
    /// ability's own source rather than printed on the card.
    ManaValueEqualTo(ValueDef),
    /// Mana value at most a computed amount, for "with mana value X or less".
    ManaValueAtMostValue(ValueDef),
    /// Power at least this much, for "power N or greater". Target legality
    /// reads real current power, so a creature a Crusade has pumped qualifies.
    /// Trigger and static matching still read power without continuous
    /// statics, because that view is what static resolution is handed.
    PowerAtLeast(i16),
    /// Power exactly this much. Like [`Self::PowerAtLeast`] this reads
    /// current power for target legality, so "target 1/1 creature" stops being
    /// one the moment anything pumps it, a Crusade included.
    PowerExactly(i16),
    /// Toughness exactly this much, read the same way.
    ToughnessExactly(i16),
    /// Toughness strictly below a value computed from the ability's own
    /// source, for "toughness less than this creature's power".
    ToughnessLessThan(ValueDef),
    /// Power strictly above a computed value, for "greater power than this
    /// creature". The mirror of [`Self::ToughnessLessThan`], read the same
    /// way and against the same source.
    PowerGreaterThan(ValueDef),
    /// Toughness strictly above a computed value.
    ToughnessGreaterThan(ValueDef),
    /// Controlled by a player in this relation to the ability's controller,
    /// for "a creature you control" and "whenever you cast".
    ControlledBy(PlayerRelation),
    /// Carries this supertype. Negate it for "nonbasic".
    Supertype(CardSupertype),
    /// "With a name originally printed in the <set> expansion", which reads
    /// the card's debut set rather than the printing in front of you. Tokens
    /// were printed in no expansion, so they never match.
    DebutSet(CardSet),
    /// Has the same printed name as the ability's source. Negate it for
    /// "not named <this card>".
    SharesNameWithSource,
    /// Currently attacking or blocking. Only a battlefield object can be, so
    /// this never matches a card or a spell.
    AttackingOrBlocking,
    /// Has this keyword. Protection is not askable this way, because it is a
    /// keyword per color rather than one keyword.
    ///
    /// This reads keywords a continuous static effect grants or removes, so a
    /// creature wearing a Lord's granted keyword matches. The one place it
    /// does not is a static ability that grants or removes abilities choosing
    /// its own recipients, which is answered from the layer below itself.
    HasKeyword(KeywordAbility),
    /// Has at least one ordinary activated ability rather than only mana
    /// abilities. This is the distinction Tsabo's Web asks for.
    HasNonManaActivatedAbility,
    /// A creature currently declared as an attacker in combat.
    Attacking,
    /// A creature currently blocking one. This is the other half of
    /// [`Self::AttackingOrBlocking`], which neither of the single-sided
    /// predicates could express on its own.
    Blocking,
    /// A creature that was declared as an attacker at any point this turn,
    /// whether or not it is still attacking or even still in combat.
    AttackedThisTurn,
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
