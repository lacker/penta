use crate::ids::{ObjectSetBindingIndex, TargetIndex};

use super::super::{
    BasicLandType, ComparisonDef, CounterKind, ManaColor, ObjectPredicateDef, PlayerRelation,
    ZoneKind,
};
use super::{DamageSourceGroupDef, ObjectRefDef, PlayerSetDef};

/// Where an object must sit relative to another object in the same ordered
/// zone. Libraries and graveyards are stored bottom/oldest first, so `Above`
/// means a larger zone index and `Below` a smaller one.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneRelativePositionDef {
    Above(ObjectRefDef),
    Below(ObjectRefDef),
}

/// Spells cast during the current turn, filtered by who cast them and by the
/// characteristics they had on the stack.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpellCastQueryDef {
    pub player: PlayerRelation,
    pub spell: ObjectPredicateDef,
}

/// The two branches of a conditional value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConditionalValueDef {
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

/// A value that depends on how many card types a graveyard holds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraveyardTypeConditionDef {
    pub player: PlayerRelation,
    pub minimum: u8,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LifeConditionDef {
    pub threshold: u16,
    pub then: ValueDef,
    pub otherwise: ValueDef,
}

impl LifeConditionDef {
    #[must_use]
    pub const fn new(threshold: u16, then: ValueDef, otherwise: ValueDef) -> Self {
        Self {
            threshold,
            then,
            otherwise,
        }
    }
}

/// A conditional value that asks how many objects match.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CountConditionDef {
    pub query: ObjectQueryDef,
    /// How the count is compared, so that a threshold -- metalcraft's three
    /// or more artifacts -- reads the same way an exact count does. The
    /// mirror of [`super::ObjectCountConditionDef`], which asks the same
    /// question of a condition rather than of an amount.
    pub comparison: ComparisonDef,
    pub amount: u8,
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

/// A set of objects with independent controller and owner constraints.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectQueryDef {
    pub object: ObjectPredicateDef,
    pub zones: &'static [ZoneKind],
    /// A zone-relative constraint: controller for battlefield and stack
    /// objects, owner for cards in every other zone. This preserves the
    /// ordinary "you control" / "in your graveyard" query vocabulary even
    /// when one query spans both kinds of zone.
    pub related_player: Option<PlayerSetDef>,
    pub controller: Option<PlayerSetDef>,
    pub owner: Option<PlayerSetDef>,
    /// A position relative to another object in the same ordered zone.
    pub relative_position: Option<ZoneRelativePositionDef>,
    /// The object chosen for this target slot does not count, which is the
    /// "other than that creature" a clause adds once it has already named
    /// one. Only a resolving effect knows its targets, so a query read
    /// outside one excludes nothing.
    pub excluding_target: Option<TargetIndex>,
}

/// Permanents attached to a player in one relation and matching one object
/// predicate. Held behind a reference by `ValueDef` so ordinary values remain
/// compact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayerAttachmentQueryDef {
    pub player: PlayerRelation,
    pub object: ObjectPredicateDef,
}

impl PlayerAttachmentQueryDef {
    #[must_use]
    pub const fn new(player: PlayerRelation, object: ObjectPredicateDef) -> Self {
        Self { player, object }
    }
}

impl ObjectQueryDef {
    #[must_use]
    pub const fn new(object: ObjectPredicateDef, zones: &'static [ZoneKind]) -> Self {
        Self {
            object,
            zones,
            related_player: None,
            controller: None,
            owner: None,
            relative_position: None,
            excluding_target: None,
        }
    }

    #[must_use]
    pub const fn controlled_by(
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller: PlayerSetDef,
    ) -> Self {
        Self {
            object,
            zones,
            related_player: None,
            controller: Some(controller),
            owner: None,
            relative_position: None,
            excluding_target: None,
        }
    }

    #[must_use]
    pub const fn owned_by(
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        owner: PlayerSetDef,
    ) -> Self {
        Self {
            object,
            zones,
            related_player: None,
            controller: None,
            owner: Some(owner),
            relative_position: None,
            excluding_target: None,
        }
    }

    /// Compatibility constructor for the old zone-relative query spelling:
    /// battlefield/stack objects are related by controller, while cards in
    /// other zones are related by owner.
    #[must_use]
    pub const fn matching(
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        controller_or_owner: PlayerRelation,
    ) -> Self {
        Self {
            object,
            zones,
            related_player: Some(PlayerSetDef::Related(controller_or_owner)),
            controller: None,
            owner: None,
            relative_position: None,
            excluding_target: None,
        }
    }

    /// The same query with whatever was chosen for `target` left out, which
    /// is what "other than that creature" asks for.
    #[must_use]
    pub const fn excluding_target(mut self, target: TargetIndex) -> Self {
        self.excluding_target = Some(target);
        self
    }

    #[must_use]
    pub const fn above(mut self, object: ObjectRefDef) -> Self {
        self.relative_position = Some(ZoneRelativePositionDef::Above(object));
        self
    }

    #[must_use]
    pub const fn below(mut self, object: ObjectRefDef) -> Self {
        self.relative_position = Some(ZoneRelativePositionDef::Below(object));
        self
    }

    /// Lands a matching player controls with the named effective basic land
    /// type. Dual lands and lands changed by continuous effects match too.
    #[must_use]
    pub const fn controlled_basic_land_type(
        player: PlayerRelation,
        land_type: BasicLandType,
    ) -> Self {
        Self::matching(
            ObjectPredicateDef::HasAnyBasicLandType(land_type.singleton()),
            &[ZoneKind::Battlefield],
            player,
        )
    }
}

/// A value evaluated from the resolving spell or ability and its captured
/// event. `SourcePower` and `SourceToughness` deliberately leave current-versus
/// last-known-information selection to the runtime source reference.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ValueDef {
    Constant(i32),
    ChosenX,
    /// The X chosen for the spell that put the ability's source onto the
    /// battlefield. An enters trigger is a new object, so [`Self::ChosenX`]
    /// reads nothing there; this reads it off the permanent instead.
    SourceCastX,
    /// The mana value of the permanent a static effect is being applied to,
    /// rather than of the permanent applying it. Opalescence gives each
    /// other enchantment a body its own cost decides, so the number is the
    /// affected object's and changes from one to the next.
    AffectedManaValue,
    /// The number of colors the permanent a static effect is being applied
    /// to currently has. Civic Saber measures the equipped creature rather
    /// than the Equipment applying the bonus, and the value follows later
    /// color-changing effects.
    AffectedColorCount,
    /// The printed power, or toughness, of every card exiled with the
    /// ability's own source, added up. Sutured Ghoul is the body its own
    /// entry assembled, so the number is read off that pile rather than off
    /// the board.
    TotalPowerOfLinkedExiles,
    TotalToughnessOfLinkedExiles,
    SourcePower,
    SourceToughness,
    TriggerEventAmount,
    CardsInHandAbove {
        player: PlayerRelation,
        threshold: u8,
    },
    /// How much damage a player has been dealt so far this turn, optionally
    /// only from one named source group. Accumulated as the damage lands,
    /// because a group such as "unblocked creatures" stops being answerable
    /// once combat is over.
    DamageTakenThisTurn {
        player: PlayerRelation,
        source: Option<DamageSourceGroupDef>,
    },
    /// The largest power among the matching objects, or zero when nothing
    /// matches. Distinct from a count: "the greatest power among creatures
    /// you control" asks one creature how big it is rather than asking how
    /// many there are.
    GreatestPowerAmong(&'static ObjectQueryDef),
    /// How many objects match, for the "for each" clauses. Held by reference
    /// so that `ValueDef` stays small enough to embed freely.
    CountMatchingObjects(&'static ObjectQueryDef),
    /// How many matching permanents are attached to the named player.
    CountMatchingPlayerAttachments(&'static PlayerAttachmentQueryDef),
    /// How many spells matching the query have been cast this turn.
    CountSpellsCastThisTurn(&'static SpellCastQueryDef),
    /// One when at least one object matches, zero otherwise. "As long as you
    /// control a Mountain" is a condition rather than a count, so counting
    /// matches would pay a second Mountain twice.
    AnyMatchingObject(&'static ObjectQueryDef),
    /// The negation of another value, so a "for each" penalty can reuse the
    /// same count a bonus would.
    Negate(&'static ValueDef),
    /// Another value multiplied by a constant, for the clauses that pay more
    /// than one per thing counted. Held by reference for the same reason
    /// [`Self::Negate`] is: `ValueDef` stays one word wide.
    Scaled(&'static ScaledValueDef),
    /// Two values added together, for "1 plus the power of ...". Held by
    /// reference like the other compound forms so that `ValueDef` stays one
    /// word wide.
    Sum(&'static SumValueDef),
    /// Half of another value, rounded the way the card says. Rounding is only
    /// visible when a value is divided, so the direction belongs to the
    /// division rather than being a separate step over it.
    Halved(&'static HalvedValueDef),
    /// How many counters of one kind sit on the ability's own source.
    CountersOnSource(CounterKind),
    /// How many counters of one kind a player has. The player-held pile
    /// rather than a permanent's: experience, poison, and energy are all
    /// counted here.
    PlayerCounters {
        player: PlayerRelation,
        kind: CounterKind,
    },
    /// How many spells were cast before this one this turn, by anybody. The
    /// spell carrying the ability is already counted when it is cast, so this
    /// subtracts it: storm copies what came before, not itself.
    SpellsCastBeforeThisTurn,
    /// How many times the source spell paid a repeatable optional additional
    /// cost. Replicate is the only one, and this is what its cast trigger
    /// counts copies with: the payment appears once per payment in the cast's
    /// own record, so the number of times it was paid is how many are there.
    TimesAdditionalCostPaid,
    /// What the permanents sacrificed to pay this ability's activation cost
    /// added up to in mana value, read from last-known information: they are
    /// already gone by the time the ability resolves, because paying is what
    /// put it on the stack. A cost that sacrifices exactly one permanent --
    /// every printed one so far -- reads that permanent's mana value.
    SacrificedManaValue,
    /// How many objects the step before this one matched: the land cards a
    /// discard actually took. Zero without such a step behind it.
    MatchedCount,
    /// How many distinct card types those matched objects had between them,
    /// which is what "for each card type among cards discarded this way"
    /// counts. One card that is both an artifact and a creature contributes
    /// two; two creatures contribute one.
    MatchedCardTypes,
    /// What those same matched objects add up to in mana value. "You lose
    /// life equal to that card's mana value" reads the card the step before
    /// it revealed, and a mill follow-up reads the cards the mill just moved.
    /// Those cards are gone from their old zone by the time this is asked,
    /// so the number travels rather than the card.
    MatchedManaValue,
    /// How many objects an earlier step in this resolution bound. "For each
    /// creature exiled this way" counts what the exile actually took, which
    /// the board no longer holds by the time the follow-up runs. Zero
    /// without such a step behind it.
    BoundObjectCount(ObjectSetBindingIndex),
    /// What was actually paid for a [`super::EffectPaymentCostDef::ChosenGenericMana`]
    /// payment in this resolution. Zero anywhere else, so a branch that reads
    /// it without a payment behind it does nothing rather than guessing.
    /// "Your devotion to blue" (CR 702.10c): each coloured mana symbol of
    /// that colour in the mana costs of permanents this player controls. A
    /// hybrid symbol counts once for each of its colours, and a permanent
    /// with no mana cost contributes nothing.
    DevotionTo(ManaColor),
    /// Domain (CR 702.5a): how many of the five basic land types are among
    /// the lands this player controls. Counted over effective types, so a
    /// dual land is two of them and a land somebody turned into a Swamp
    /// counts as one.
    BasicLandTypesControlled(PlayerRelation),
    /// How many cards a player's library holds. The mirror of
    /// [`Self::TargetLibrarySize`] for the clauses that name a player by
    /// relation rather than by pointing at one.
    LibrarySize(PlayerRelation),
    /// How many spells a player has cast this game. Kept for the whole game
    /// rather than reset with the turn, which is what makes "the first spell
    /// you've cast this game" answerable; the spell asking is counted as it
    /// goes on the stack, so a clause read while casting sees the ones
    /// before it.
    SpellsCastThisGame(PlayerRelation),
    /// "The number of cards you've drawn this turn." Counts every draw,
    /// including the turn's own draw step, and resets when the turn does.
    CardsDrawnThisTurn(PlayerRelation),
    /// "The amount of life you gained this turn." A running total rather
    /// than a net change: losing it again afterwards does not take it back,
    /// and it resets when the turn does.
    LifeGainedThisTurn(PlayerRelation),
    /// "The number of colors of mana spent to cast this spell" (CR 702.86a,
    /// converge). Colorless is a mana type rather than a color and never
    /// counts. A copy of a spell was never cast, so nothing was spent on it
    /// and this reads zero however the original was paid for.
    ColorsOfManaSpent,
    PaidAmount,
    /// How many creatures have died this turn, for "for each creature that
    /// died this turn". Counted as they die rather than read off a zone,
    /// because a graveyard is not a record of this turn.
    CreaturesDiedThisTurn,
    /// "For each opponent who lost life this turn." A count of players
    /// rather than of life: one opponent who lost twelve counts once, and
    /// how the life went does not matter.
    OpponentsWhoLostLifeThisTurn,
    /// How many distinct card types appear among the cards in the graveyards
    /// of matching players. Types, not cards: ten artifact cards and ten
    /// creature cards are still two. A Lhurgoyf counts every graveyard;
    /// delirium counts one.
    CardTypesAmongGraveyards(PlayerRelation),
    /// One value once a graveyard holds at least so many card types, and
    /// another below it. This is delirium, which changes an amount rather
    /// than switching an effect on.
    IfCardTypesAmongGraveyards(&'static GraveyardTypeConditionDef),
    /// How many distinct names appear among the objects a query matches.
    /// Names, not objects: four Mountains are one, and the count is what
    /// "seven or more lands with different names" asks about. A nameless
    /// object -- a token with no card behind it -- counts for nothing,
    /// since it shares its name with every other one.
    DistinctNamesAmong(&'static ObjectQueryDef),
    /// The morbid condition. Held by reference so that `ValueDef` stays one
    /// word wide; a second inline value would grow everything embedding it.
    IfCreatureDiedThisTurn(&'static ConditionalValueDef),
    /// One value while the ability's controller is at or below this life
    /// total, another otherwise. The fateful-hour "instead" clauses, which
    /// replace an amount rather than adding a second effect beside it.
    IfControllerLifeAtMost(&'static LifeConditionDef),
    /// One value when the chosen target matches, another when it does not.
    /// Held by reference for the same reason.
    IfTargetMatches(&'static TargetConditionDef),
    /// One value when exactly that many objects match, another otherwise.
    /// This is how an intervening-if condition becomes an amount.
    IfMatchingObjectCount(&'static CountConditionDef),
    /// The number of distinct players or objects chosen as targets while a
    /// spell is being cast. Repeating the same target in two slots counts it
    /// once, which is the quantity Hinata changes a spell's cost by.
    DistinctTargets,
    /// How much of a divided total the target being affected takes. Only
    /// meaningful for an effect aimed at a slot the card divides.
    DividedAmongTargets,
    /// The power of what a target slot points at, for "damage equal to its
    /// power".
    /// The triggering object's power, read with last-known information. A
    /// death trigger asks this about a creature that has already left, which
    /// is the only time it is interesting.
    TriggeringObjectPower,
    /// The triggering object's toughness, read the same way and for the same
    /// reason: a death trigger asks about a creature that has already left.
    TriggeringObjectToughness,
    TargetPower(TargetIndex),
    /// The toughness of what a target slot points at. Read the same way
    /// as its power, and no harder to reach -- the card simply has to say
    /// which characteristic it wants.
    TargetToughness(TargetIndex),
    /// How many cards are in the library of the player a target slot
    /// points at. Read live, so an effect that mills as it goes sees
    /// the library it started with only if it asks first.
    TargetLibrarySize(TargetIndex),
    /// A player's life total, read live. Distinct from the fateful-hour
    /// conditions, which compare it: this is the number itself, for the
    /// clauses that spend it as an amount.
    LifeTotal(PlayerRelation),
    /// The mana value of what a target slot points at, read from last-known
    /// information after a permanent or spell has left its zone.
    TargetManaValue(TargetIndex),
    /// The power of one named object, wherever it is, using last-known
    /// information after it leaves its zone.
    ObjectPower(ObjectRefDef),
    /// The mana value of one named object, wherever it is. Distinct from
    /// [`Self::TargetManaValue`], which can only name a target slot: this
    /// reads a card an earlier step in the same resolution bound, which is
    /// how "where X is the mana value of the exiled card" finds the card it
    /// means.
    ObjectManaValue(ObjectRefDef),
}

/// A value and the constant it is multiplied by, for "+N/+N for each ...".
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScaledValueDef {
    pub value: ValueDef,
    pub factor: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SumValueDef {
    pub left: ValueDef,
    pub right: ValueDef,
}

impl SumValueDef {
    #[must_use]
    pub const fn new(left: ValueDef, right: ValueDef) -> Self {
        Self { left, right }
    }
}

/// Which way a halved value rounds. A card that halves says so explicitly,
/// and the two halves of "half rounded down and half rounded up" are what
/// make a single count into two different numbers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RoundingDef {
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalvedValueDef {
    pub value: ValueDef,
    pub rounding: RoundingDef,
}

impl HalvedValueDef {
    #[must_use]
    pub const fn new(value: ValueDef, rounding: RoundingDef) -> Self {
        Self { value, rounding }
    }

    /// Halves `total` the way this definition says. Rounding is applied
    /// towards the named direction for negative totals too, so a negative
    /// count does not quietly change which way it goes.
    #[must_use]
    pub const fn apply(&self, total: i32) -> i32 {
        match self.rounding {
            RoundingDef::Down => total.div_euclid(2),
            RoundingDef::Up => total.div_euclid(2) + total.rem_euclid(2),
        }
    }
}

impl ScaledValueDef {
    #[must_use]
    pub const fn new(value: ValueDef, factor: i32) -> Self {
        Self { value, factor }
    }
}
