//! Compact constructors for shared game action programs.
//!
//! Every constructor returns the ordinary inspectable [`GameActionDef`]. Use
//! its `as_cost()` or `as_effect()` method to supply the execution contract,
//! including for programs assembled directly from explicit action definitions.

use super::{
    Binding, ChoiceVisibilityDef, ControlDurationDef, EffectRecipientDef, GameActionChoiceDef,
    GameActionDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ValueDef, ZoneKind,
};
use crate::ids::ParentBinding;

/// Discard already identified cards without introducing a selection.
#[must_use]
pub const fn discard_cards(object: EffectRecipientDef) -> GameActionDef {
    GameActionDef::DiscardCards { object }
}

/// Have each named permanent's controller sacrifice it.
#[must_use]
pub const fn sacrifice(object: EffectRecipientDef) -> GameActionDef {
    GameActionDef::Sacrifice { object }
}

/// Have the executing player sacrifice the named permanents they control.
#[must_use]
pub const fn sacrifice_yours(object: EffectRecipientDef) -> GameActionDef {
    GameActionDef::SacrificeYours { object }
}

/// Give the named player control for the specified duration.
#[must_use]
pub const fn gain_control(
    object: EffectRecipientDef,
    controller: PlayerRefDef,
    duration: ControlDurationDef,
) -> GameActionDef {
    GameActionDef::GainControl {
        object,
        controller,
        duration,
    }
}

/// Execute action programs in order, finishing each before starting the next.
#[must_use]
pub const fn sequence(actions: &'static [GameActionDef]) -> GameActionDef {
    GameActionDef::Sequence(actions)
}

/// Choose one complete action program; affordability is checked before selection.
#[must_use]
pub const fn choice(actions: &'static [GameActionDef]) -> GameActionDef {
    GameActionDef::Choice(actions)
}

/// Exile already identified objects from the specified zone.
/// Currently the shared executor supports graveyard cards.
#[must_use]
pub const fn exile(object: EffectRecipientDef, from: ZoneKind) -> GameActionDef {
    GameActionDef::Exile { object, from }
}

/// Publicly select cards from your graveyard, then exile them as one action.
#[must_use]
pub const fn choose_exile_from_graveyard(amount: u16) -> GameActionDef {
    choose(
        ParentBinding,
        ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Any,
            &[ZoneKind::Graveyard],
            PlayerSetDef::Related(PlayerRelation::You),
        )),
        &EXILE_CHOSEN,
    )
    .with_amount(ValueDef::Constant(amount as i32))
}

/// Let the executing player publicly select one object, bind it, and perform
/// `then`. Selection builders override the amount, chooser, and visibility.
/// The body names the supplied binding explicitly, so custom programs can
/// compose scopes without depending on a convenience constructor's binding.
#[must_use]
pub const fn choose(
    binding: Binding,
    candidates: ObjectSetDef,
    then: &'static GameActionDef,
) -> GameActionDef {
    GameActionDef::Choose(GameActionChoiceDef {
        binding,
        chooser: PlayerRefDef::EffectController,
        candidates,
        amount: ValueDef::Constant(1),
        visibility: ChoiceVisibilityDef::Public,
        then,
    })
}

/// Privately choose cards from your hand and discard them. Use `matching()`
/// for a card restriction and `with_amount()` for a computed quantity.
#[must_use]
pub const fn choose_discard(amount: u16) -> GameActionDef {
    choose(
        ParentBinding,
        ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Any,
            &[ZoneKind::Hand],
            PlayerSetDef::Related(PlayerRelation::You),
        )),
        &DISCARD_CHOSEN,
    )
    .with_amount(ValueDef::Constant(amount as i32))
    .with_visibility(ChoiceVisibilityDef::Private)
}

/// Publicly choose permanents you control and sacrifice them. Use `matching()`
/// for a permanent restriction and `with_amount()` for a computed quantity.
#[must_use]
pub const fn choose_sacrifice(amount: u16) -> GameActionDef {
    choose(
        ParentBinding,
        ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::Any,
            &[ZoneKind::Battlefield],
            PlayerSetDef::Related(PlayerRelation::You),
        )),
        &SACRIFICE_CHOSEN,
    )
    .with_amount(ValueDef::Constant(amount as i32))
}

/// Publicly choose permanents you do not control and gain control indefinitely.
/// Use `matching()` for a restriction and `with_amount()` for a computed
/// quantity. A different control duration belongs in an explicit `choose()`
/// program whose body uses `gain_control()`.
#[must_use]
pub const fn choose_gain_control(amount: u16) -> GameActionDef {
    choose(
        ParentBinding,
        ObjectSetDef::Query(ObjectQueryDef::controlled_by(
            ObjectPredicateDef::Any,
            &[ZoneKind::Battlefield],
            PlayerSetDef::Related(PlayerRelation::NotYou),
        )),
        &GAIN_CONTROL_OF_CHOSEN,
    )
    .with_amount(ValueDef::Constant(amount as i32))
}

const CHOSEN: EffectRecipientDef =
    EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding));
const EXILE_CHOSEN: GameActionDef = exile(CHOSEN, ZoneKind::Graveyard);
const DISCARD_CHOSEN: GameActionDef = discard_cards(CHOSEN);
const SACRIFICE_CHOSEN: GameActionDef = sacrifice_yours(CHOSEN);
const GAIN_CONTROL_OF_CHOSEN: GameActionDef = gain_control(
    CHOSEN,
    PlayerRefDef::EffectController,
    ControlDurationDef::Indefinitely,
);

/// Move already identified objects, preserving their exact zone-scoped identity.
#[must_use]
pub const fn move_to_zone(
    object: EffectRecipientDef,
    zone: ZoneKind,
    placement: crate::card::ZonePlacement,
) -> GameActionDef {
    GameActionDef::MoveToZone {
        object,
        zone,
        placement,
    }
}
