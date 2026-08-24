use crate::action::AbilityOrigin;
use crate::card::AbilityTargetDef;
use crate::ids::{GameObjectId, PlayerId};

use super::{Game, ScopedEffect};

impl Game {
    pub(super) fn record_spell_cast(&mut self, player: PlayerId, spell: GameObjectId) {
        self.spells_cast_this_turn[player.index()] =
            self.spells_cast_this_turn[player.index()].saturating_add(1);
        self.spell_cast_history_this_turn.push(spell);
        self.total_spells_cast[player.index()] =
            self.total_spells_cast[player.index()].saturating_add(1);
    }
}

/// The executable target/effect layout obtained after one concrete set of
/// spell modes has been selected. Building both vectors together keeps their
/// positional mapping atomic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SelectedSpellPlan {
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) mode_effects: Vec<ScopedEffect>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CastSourceZone {
    Hand,
    Graveyard,
    /// A card on an adventure, which its owner may cast from exile as the
    /// creature it is on the other half.
    Exile,
    /// The top card of the caster's own library, for the permissions that
    /// reach up there. Only ever the topmost one: a permission to play from
    /// the top of a library names one card, not the library.
    LibraryTop,
}

impl CastSourceZone {
    /// The zone a card-facing clause names. The top of a library is the
    /// library: "cast from your library" is what a player would say, and no
    /// printed clause distinguishes the topmost card from the rest.
    pub(super) const fn zone(self) -> crate::card::ZoneKind {
        match self {
            Self::Hand => crate::card::ZoneKind::Hand,
            Self::Graveyard => crate::card::ZoneKind::Graveyard,
            Self::Exile => crate::card::ZoneKind::Exile,
            Self::LibraryTop => crate::card::ZoneKind::Library,
        }
    }

    /// The stable wire label for this zone, for a checkpoint that names it
    /// rather than storing an enum whose order could move.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Hand => "hand",
            Self::Graveyard => "graveyard",
            Self::Exile => "exile",
            Self::LibraryTop => "libraryTop",
        }
    }
}

/// The inverse of [`CastSourceZone::label`]. An unknown label reads back as
/// nothing, which is what a spell nobody cast carries anyway.
pub(super) fn cast_source_zone_from_label(label: &str) -> Option<CastSourceZone> {
    match label {
        "hand" => Some(CastSourceZone::Hand),
        "graveyard" => Some(CastSourceZone::Graveyard),
        "exile" => Some(CastSourceZone::Exile),
        "libraryTop" => Some(CastSourceZone::LibraryTop),
        _ => None,
    }
}

/// A cast that must be taken while a resolving instruction offers it, or not
/// at all. The standing decision is the permission; no separate game flag is
/// needed to remember that the card may be cast.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CastOffer {
    pub(super) player: PlayerId,
    pub(super) card: GameObjectId,
    pub(super) source_zone: CastSourceZone,
    pub(super) cost: CastOfferCost,
}

/// Which costs a one-shot cast offer permits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CastOfferCost {
    /// The instruction permits any way the named card can otherwise be cast.
    Any,
    /// The instruction permits exactly one alternative printed on the card.
    /// Its origin identifies the authored clause even when several clauses
    /// share the same keyword kind.
    PrintedAlternative(AbilityOrigin),
    /// The instruction permits exactly one temporary grant. The slot is
    /// stable while the standing decision exists and keeps this hot context
    /// small even though a complete `AbilityDef` is comparatively large.
    GrantedAlternative(usize),
}

/// The zone and any one-shot permission governing which cost configurations
/// may be selected for a cast.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CastCostContext {
    pub(super) source_zone: CastSourceZone,
    pub(super) offer: Option<CastOfferCost>,
}
