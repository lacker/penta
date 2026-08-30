//! Facts and choices carried from a spell's casting through resolution.
//!
//! A spell copy keeps the original spell's casting choices and references to
//! objects used to pay its costs (CR 707.10), but it was not itself cast and
//! no mana or life was spent on it. Keeping those categories together here
//! makes that copy boundary explicit instead of rediscovering a vanished
//! casting permission from a copied signature.

use super::{CastSourceZone, Game, GameObjectId, RetiredObject, StackObject};
use crate::{AlternativeCastKindDef, ColorSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CastContext {
    /// The zone the spell was actually cast from. `None` means this object is
    /// a spell copy rather than a cast spell.
    pub(super) source_zone: Option<CastSourceZone>,
    /// The selected alternative-cost family. This is a copied casting choice,
    /// even though [`Self::source_zone`] is cleared on a spell copy.
    pub(super) alternative: Option<AlternativeCastKindDef>,
    /// Whether the actual cast happened outside an ordinary sorcery window.
    pub(super) at_instant_speed: bool,
    /// The announced X and optional additional-cost payments. These are cast
    /// choices copied with a spell under CR 707.10.
    pub(super) x: u16,
    pub(super) repeatable_additional_costs: u16,
    pub(super) additional_costs: Vec<u16>,
    /// Mana and life actually spent on this object. A spell copy resets these
    /// because it paid no costs of its own.
    pub(super) colors_of_mana_spent: ColorSet,
    pub(super) phyrexian_symbols_paid_with_life: u16,
    /// New exile-zone identities of cards used to pay the spell's costs.
    /// A spell copy refers to the same payment objects as the original.
    pub(super) exiled_payment_cards: Vec<GameObjectId>,
    /// Resolution riders supplied by the cast procedure rather than copied
    /// choices. Copies neither flash back nor arrive from suspend.
    pub(super) via_flashback: bool,
    pub(super) via_suspend: bool,
}

impl CastContext {
    pub(super) fn for_cast(
        source_zone: CastSourceZone,
        alternative: Option<AlternativeCastKindDef>,
        at_instant_speed: bool,
        x: u16,
        repeatable_additional_costs: u16,
        additional_costs: Vec<u16>,
        via_flashback: bool,
    ) -> Self {
        Self {
            source_zone: Some(source_zone),
            alternative,
            at_instant_speed,
            x,
            repeatable_additional_costs,
            additional_costs,
            colors_of_mana_spent: ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
            exiled_payment_cards: Vec::new(),
            via_flashback,
            via_suspend: false,
        }
    }

    pub(super) fn for_spell_copy(&self) -> Self {
        let mut copied = self.clone();
        copied.source_zone = None;
        copied.at_instant_speed = false;
        copied.colors_of_mana_spent = ColorSet::empty();
        copied.phyrexian_symbols_paid_with_life = 0;
        copied.via_flashback = false;
        copied.via_suspend = false;
        copied
    }

    pub(super) const fn was_cast(&self) -> bool {
        self.source_zone.is_some()
    }

    pub(super) fn colors_spent_count(&self) -> u8 {
        self.colors_of_mana_spent
            .to_flags()
            .into_iter()
            .filter(|spent| *spent)
            .count()
            .try_into()
            .unwrap_or(u8::MAX)
    }
}

impl Game {
    pub(super) fn cast_context_for(
        &self,
        source: GameObjectId,
        resolving: Option<&StackObject>,
    ) -> Option<CastContext> {
        resolving
            .filter(|object| object.id == source)
            .and_then(|object| object.cast.clone())
            .or_else(|| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .and_then(|permanent| permanent.cast.clone())
            })
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|object| object.id == source)
                    .and_then(|object| object.cast.clone())
            })
            .or_else(|| match self.retired_objects.get(&source) {
                Some(RetiredObject::Stack(object)) => object.cast.clone(),
                Some(RetiredObject::Permanent { permanent, .. }) => permanent.cast.clone(),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }

    /// Current exile-zone objects linked to `source`, whether an effect
    /// exiled them with that source or the spell used them to pay a cost.
    pub(super) fn linked_exile_ids(&self, source: GameObjectId) -> Vec<GameObjectId> {
        self.linked_exile_ids_with_cast(source, None)
    }

    pub(super) fn linked_exile_ids_with_cast(
        &self,
        source: GameObjectId,
        entering_cast: Option<&CastContext>,
    ) -> Vec<GameObjectId> {
        let mut linked = entering_cast.map_or_else(
            || {
                self.cast_context_for(source, None)
                    .map_or_else(Vec::new, |cast| cast.exiled_payment_cards)
            },
            |cast| cast.exiled_payment_cards.clone(),
        );
        for exiled in self
            .linked_exiles
            .iter()
            .filter_map(|(linked_source, exiled)| (*linked_source == source).then_some(*exiled))
        {
            if !linked.contains(&exiled) {
                linked.push(exiled);
            }
        }
        linked.retain(|exiled| {
            self.card_in_nonbattlefield_zone(*exiled)
                .is_some_and(|(zone, _)| zone == crate::card::ZoneKind::Exile)
        });
        linked
    }
}
