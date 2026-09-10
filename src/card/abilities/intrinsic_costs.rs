//! Constructors that append a mechanic's mandatory cost to a static slice.
//!
//! A const function cannot return a static slice containing its parameters.
//! These macros materialize the concatenated array at the declaration site;
//! the resulting ability contains the same plain cost slice as any activation.

pub use crate::{bloodrush, cycling, eternalize, ninjutsu, scavenge, typecycling};

#[doc(hidden)]
#[macro_export]
macro_rules! __append_mechanic_cost {
    ($costs:expr, $intrinsic:expr) => {
        const {
            let costs: &[$crate::CostDef] = $costs;
            let mut result = [$intrinsic; ($costs as &[$crate::CostDef]).len() + 1];
            let mut index = 0;
            while index < costs.len() {
                result[index] = costs[index];
                index += 1;
            }
            result
        }
    };
}

/// A Bloodrush ability activated from hand. Appends discarding this card;
/// the caller supplies the other costs, text, targets, and effect.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
#[macro_export]
macro_rules! bloodrush {
    ($costs:expr, $text:expr, $targets:expr, $effect:expr $(,)?) => {
        $crate::card::abilities::bloodrush_with_costs(
            &$crate::__append_mechanic_cost!($costs, $crate::CostDef::DiscardSource),
            $text,
            $targets,
            $effect,
        )
    };
}

/// Scavenge places counters equal to the exiled card's power. Appends
/// exiling the source from its owner's graveyard and enforces sorcery timing.
/// The caller supplies the scavenge cost and exact rules text.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
#[macro_export]
macro_rules! scavenge {
    ($costs:expr, $text:expr $(,)?) => {
        $crate::card::abilities::scavenge_with_costs(
            &$crate::__append_mechanic_cost!($costs, $crate::CostDef::ExileSource),
            $text,
        )
    };
}

/// "Cycling {cost} ({cost}, Discard this card: Draw a card.)"
///
/// Cycling is an activated ability that exists only while the card is in
/// hand, which is what keeps it off the battlefield version of the same
/// permanent. Nothing else about it is special: the discard is a cost, so it
/// happens on activation rather than on resolution, and the draw is what goes
/// on the stack. The caller supplies the printed text because the reminder
/// repeats the cost.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
#[macro_export]
macro_rules! cycling {
    ($text:expr, $costs:expr $(,)?) => {
        $crate::card::abilities::cycling_with_costs(
            $text,
            &$crate::__append_mechanic_cost!($costs, $crate::CostDef::DiscardSource),
        )
    };
}

/// "<Type>cycling {cost}" -- the same ability as [`cycling!`], except that
/// what it buys is a search rather than a draw. Failing to find is allowed,
/// so the minimum is zero: the discard has already been paid either way.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
#[macro_export]
macro_rules! typecycling {
    ($text:expr, $costs:expr, $object:expr $(,)?) => {
        $crate::card::abilities::typecycling_with_costs(
            $text,
            &$crate::__append_mechanic_cost!($costs, $crate::CostDef::DiscardSource),
            $object,
        )
    };
}

/// "Eternalize {cost}" (CR 702.129a).
///
/// An activated ability of the card in its owner's graveyard: it exiles
/// itself as a cost and makes a token copy of what it just exiled, except
/// for the four things the keyword fixes -- a 4/4 body, black, a Zombie on
/// top of the types it already had, and no mana cost. Sorcery timing,
/// because the reminder says so.
///
/// The caller supplies the printed text, which repeats both the cost and the
/// card's own creature types.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
///
/// ```
/// use penta::{card::abilities, mana_cost, CostDef};
/// const ABILITY: penta::AbilityDef = abilities::eternalize!(
///     "Eternalize {2}{G}{G}",
///     &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
/// );
/// ```
#[macro_export]
macro_rules! eternalize {
    ($text:expr, $costs:expr $(,)?) => {
        $crate::card::abilities::eternalize_with_costs(
            $text,
            &$crate::__append_mechanic_cost!($costs, $crate::CostDef::ExileSource),
        )
    };
}

/// Ninjutsu (CR 702.49): "`cost`, Return an unblocked attacker you control
/// to hand: Put this card onto the battlefield from your hand tapped and
/// attacking."
///
/// The return is a cost rather than an effect, so a Ninja whose activation
/// is answered has already swapped the attacker away. Activation waits for
/// blockers to be declared, since until then there is no unblocked attacker
/// to give back. The caller supplies the printed text, which repeats the
/// cost inside its own reminder.
///
/// `costs` is a constant slice containing only the variable mechanic cost.
#[macro_export]
macro_rules! ninjutsu {
    ($text:expr, $costs:expr $(,)?) => {
        $crate::card::abilities::ninjutsu_with_costs(
            $text,
            &$crate::__append_mechanic_cost!(
                $costs,
                $crate::CostDef::ReturnUnblockedAttackerToHand
            ),
        )
    };
}
