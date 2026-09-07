/// Cycling and typecycling share one rules identity (CR 702.29f).
pub const CYCLING: crate::MechanicId = crate::MechanicId::from_name("mtg:cycling");

/// A self-cycling trigger follows the discarded card into its actual zone.
/// These are listening zones, not activation permissions (CR 702.29c).
pub const CYCLED_CARD_ZONES: &[ZoneKind] = &[
    ZoneKind::Battlefield,
    ZoneKind::Hand,
    ZoneKind::Library,
    ZoneKind::Graveyard,
    ZoneKind::Exile,
];

/// "Cycling {cost} ({cost}, Discard this card: Draw a card.)"
///
/// The ability exists in every zone but can be activated only from hand.
/// Its labeled discard payment publishes the occurrence; the draw resolves
/// independently on the stack (CR 702.29a-c).
#[must_use]
pub const fn cycling(text: &'static str, cost: ManaCost) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(
            CostDef::Mana(cost),
            CostDef::named(CYCLING, &CostDef::DiscardSource),
        ),
        &[],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )
    .with_source_zones(&[ZoneKind::Hand])
    .with_mechanics(&[CYCLING])
}

/// "<Type>cycling {cost}" -- the same ability as [`cycling`], except that
/// what it buys is a search rather than a draw. Failing to find is allowed,
/// so the minimum is zero: the discard has already been paid either way.
#[must_use]
pub const fn typecycling(
    text: &'static str,
    cost: ManaCost,
    object: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(
            CostDef::Mana(cost),
            CostDef::named(CYCLING, &CostDef::DiscardSource),
        ),
        &[],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )
    .with_source_zones(&[ZoneKind::Hand])
    .with_mechanics(&[CYCLING])
}
