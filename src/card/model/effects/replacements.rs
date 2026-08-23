//! The declarative vocabulary for replacement abilities.
//!
//! A replacement ability is described by the event it watches, the condition
//! that gates it, and the operations it performs instead. These live apart
//! from the ordinary effect vocabulary because nothing outside a replacement
//! ability reads them.

use super::{
    ConditionDef, CounterKind, EffectDef, EffectPaymentDef, ObjectPredicateDef, PlayerRelation,
    TurnKindDef, ZoneKind,
};
use crate::card::AlternativeCastKindDef;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEventDef {
    /// The object carrying this ability would enter the battlefield.
    SourceEntersBattlefield,
    /// A matching object would enter the battlefield.
    ObjectEntersBattlefield {
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        /// Whether the object has to have been cast, when the clause cares.
        /// A permanent spell that resolves enters from the stack and nothing
        /// else does, so this is read off the zone it is arriving from --
        /// which is why a creature cast from a graveyard still counts as cast.
        cast: Option<bool>,
    },
    /// This ability's source would move between the named zones for the
    /// specified reason. Matching happens before the object leaves `from`.
    WouldMove {
        /// Which zone it would leave. `None` is "from anywhere", which is
        /// what Blightsteel Colossus means: it comes back whether it died,
        /// was countered, was discarded, or was milled.
        from: Option<ZoneKind>,
        to: ZoneKind,
        cause: ZoneMoveCauseDef,
    },
    /// A player would gain life, matched relative to the replacement
    /// ability's controller.
    WouldGainLife(PlayerRelation),
    /// A player would draw a card. When `during_own_draw_step` is true, the
    /// prospective draw must occur during that same player's draw step; this
    /// is narrower than merely requiring the current step to be Draw because
    /// another player can draw during it.
    WouldDraw {
        player: PlayerRelation,
        during_own_draw_step: bool,
    },
    /// A matching player would begin a turn. The turn is still prospective:
    /// none of its turn-based actions, counters, or beginning-of-turn events
    /// have happened yet.
    WouldBeginTurn {
        player: PlayerRelation,
        kind: TurnKindDef,
    },
    /// Any object anywhere would be put into this zone. Unlike
    /// [`Self::WouldMove`] this does not describe the moving object's own
    /// ability: the replacement source watches from the battlefield.
    AnyObjectWouldMove {
        to: ZoneKind,
        /// Whose zone, relative to this ability's controller. Rest in Peace
        /// says "a graveyard" and means everyone's; Forgotten Cellar says
        /// "your graveyard" and means one.
        owner: PlayerRelation,
        /// Whether tokens count. "A card or token" is Rest in Peace's
        /// wording; a clause that says only "a card" leaves tokens to go to
        /// the graveyard and cease to exist there as they always would.
        tokens: bool,
    },
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

/// A condition checked while deciding whether a replacement ability applies
/// to its prospective event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementConditionDef {
    /// The permanent carrying the replacement ability is currently tapped.
    SourceTapped,
    /// The spell this permanent came from was cast a particular way.
    /// Impending's counters are put on as the permanent enters and only when
    /// it was paid for that way, so the entry has to ask.
    SourceCastWith(AlternativeCastKindDef),
    /// A creature died at some point this turn, which is what morbid asks.
    /// Read as the replacement applies, so a creature dying in response
    /// changes the answer.
    CreatureDiedThisTurn,
    /// The replacement's controller has no more than this many cards in
    /// hand, read as the replacement applies. "As long as you have one or
    /// fewer cards in hand" is a condition on the event rather than on the
    /// permanent, which is why it lives here.
    ControllerHandAtMost(u8),
}

/// A typed modification to the permanent an object would become as it enters
/// the battlefield.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BattlefieldEntryModificationDef {
    Tapped,
    AddCounters {
        kind: CounterKind,
        amount: u16,
    },
    /// As many counters as the X its spell was cast for. "This creature
    /// enters with X +1/+1 counters on it" cannot name a number, and the
    /// entering permanent already knows what it was paid for.
    AddCastXCounters {
        kind: CounterKind,
    },
}

/// The catalog-derived vocabulary presented by a scalar entry choice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScalarChoiceListDef {
    /// The two players, presented relative to the entering permanent's
    /// controller. Kept in the scalar entry-choice path so the prospective
    /// entry and its checkpoint continuation use the same public decision
    /// machinery as card-name and creature-type choices.
    Players,
    /// Every independently nameable card part in the catalog, including split
    /// halves and back faces rather than only top-level card identities.
    CardNames,
    /// The same, without the lands. "Choose a nonland card name" is a real
    /// restriction on Meddling Mage rather than flavor: naming a land would
    /// otherwise lock out a fetch that was never castable anyway.
    NonlandCardNames,
    /// Every creature subtype available to the current game.
    CreatureTypes,
    /// The five basic land types, which are fixed rather than catalog-derived.
    BasicLandTypes,
}

/// The field on an entering permanent that receives a scalar choice.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BattlefieldEntryChoiceDestinationDef {
    Player,
    CardName,
    CreatureType,
    /// A basic land type, which the permanent then *is* rather than merely
    /// remembers: Multiversal Passage names one on the way in and reads it
    /// back in layer 4.
    BasicLandType,
}

/// A catalog-derived scalar choice made while applying an entry replacement.
///
/// Keeping the choice list and destination as separate typed axes lets one
/// decision procedure serve every string-valued entry choice without
/// hard-coding a continuation for each card characteristic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BattlefieldEntryScalarChoiceDef {
    pub list: ScalarChoiceListDef,
    pub destination: BattlefieldEntryChoiceDestinationDef,
}

impl BattlefieldEntryScalarChoiceDef {
    pub const PLAYER: Self = Self {
        list: ScalarChoiceListDef::Players,
        destination: BattlefieldEntryChoiceDestinationDef::Player,
    };

    pub const CARD_NAME: Self = Self {
        list: ScalarChoiceListDef::CardNames,
        destination: BattlefieldEntryChoiceDestinationDef::CardName,
    };

    pub const NONLAND_CARD_NAME: Self = Self {
        list: ScalarChoiceListDef::NonlandCardNames,
        destination: BattlefieldEntryChoiceDestinationDef::CardName,
    };

    pub const CREATURE_TYPE: Self = Self {
        list: ScalarChoiceListDef::CreatureTypes,
        destination: BattlefieldEntryChoiceDestinationDef::CreatureType,
    };

    pub const BASIC_LAND_TYPE: Self = Self {
        list: ScalarChoiceListDef::BasicLandTypes,
        destination: BattlefieldEntryChoiceDestinationDef::BasicLandType,
    };
}

/// A choice made while applying a source-entry replacement.
///
/// The entering object is implicit: replacement programs operate on the
/// prospective event rather than naming an already-existing game object.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementChoiceDef {
    Scalar(BattlefieldEntryScalarChoiceDef),
    Player(PlayerRelation),
    /// Any number of matching cards in the entering permanent's controller's
    /// graveyard, exiled and linked to it as it arrives. The link is the
    /// point: Sutured Ghoul's body is read off the pile it took, so the two
    /// have to stay tied together.
    ExileMatchingFromGraveyard(ObjectPredicateDef),
}

/// Declarative operations performed by a replacement ability.
///
/// Branches are slices so complex replacements remain const-friendly and can
/// be resumed around a player choice without baking card names into the game
/// engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplacementEffectDef {
    Sequence(&'static [ReplacementEffectDef]),
    /// Consume the prospective event without committing it.
    ReplaceEventWithNothing,
    /// Change the destination of a prospective zone move. The source object
    /// has not left its current zone while this operation is interpreted.
    MoveToZone(ZoneKind),
    /// Perform an ordinary declarative effect as part of replacing the event.
    /// The replacement source and controller provide the effect context.
    Perform(&'static EffectDef),
    ModifyBattlefieldEntry(BattlefieldEntryModificationDef),
    /// Multiply the amount carried by the prospective event.
    MultiplyEventAmount(u8),
    /// Add to the amount carried by the prospective event. "You draw that
    /// many cards plus one instead" is one replacement of the whole
    /// instruction rather than one per card, which is why it is counted
    /// where the instruction is rather than where a card is drawn.
    AddToEventAmount(u16),
    /// Record a choice on the object that is entering.
    Choose(ReplacementChoiceDef),
    /// Optionally use another permanent's copiable values for the entering
    /// object, retaining the named card types in addition to the copy.
    CopyEntering {
        object: ObjectPredicateDef,
        added_types: super::CardTypeSet,
        /// Whether the copy keeps the copying card's own printed subtypes on
        /// top of the ones it copies. "Except it's an Illusion in addition to
        /// its other types" names the subtype line the card already prints,
        /// so nothing has to write it down twice.
        retain_printed_subtypes: bool,
        /// Printed abilities of the copying card the copy keeps, by position.
        /// A copy takes the other permanent's abilities wholesale, so an
        /// "except it has ..." clause has to hand its own back.
        retained_abilities: &'static [crate::ids::AbilityId],
    },
    Conditional {
        condition: ConditionDef,
        if_true: &'static [ReplacementEffectDef],
        if_false: &'static [ReplacementEffectDef],
    },
    PayOr {
        payment: EffectPaymentDef,
        if_paid: &'static [ReplacementEffectDef],
        if_declined: &'static [ReplacementEffectDef],
    },
}
