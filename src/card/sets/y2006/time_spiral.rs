//! TSP card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, PregameConditionDef,
    TokenCountersDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

// TSP 29 — Momentary Blink
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENTARY_BLINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("032e072a-0630-472b-9106-5df554dff785"),
    "Momentary Blink",
    crate::card::CardArt::new("032e072a-0630-472b-9106-5df554dff785", "Anthony S. Waters"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 40 — Serra Avenger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a"),
    "Serra Avenger",
    crate::card::CardArt::new("9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a", "Scott M. Fischer"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 66 — Looter il-Kor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOOTER_IL_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("368ee06f-9021-4b65-9f53-9c326bf3a27f"),
    "Looter il-Kor",
    crate::card::CardArt::new("368ee06f-9021-4b65-9f53-9c326bf3a27f", "Mike Dringenberg"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 104 — Dread Return
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_RETURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7e304fc-0ace-459e-8d2f-376f1899639c"),
    "Dread Return",
    crate::card::CardArt::new("d7e304fc-0ace-459e-8d2f-376f1899639c", "Kev Walker"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 180 — Sulfurous Blast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUROUS_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67511e0e-be09-4f4e-9949-b9ecbdc7f536"),
    "Sulfurous Blast",
    crate::card::CardArt::new("67511e0e-be09-4f4e-9949-b9ecbdc7f536", "Jeff Miracola"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 251 — Chromatic Star
static STAR_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificeSource,
];

/// The draw is a separate trigger rather than part of the mana ability,
/// which is the whole difference from Chromatic Sphere: the mana arrives at
/// once and the card waits on the stack, so anything that answers the Star
/// after it has been sacrificed is already too late.
static STAR_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{1}, {T}, Sacrifice this artifact: Add one mana of any color.",
        &STAR_COST,
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    ),
    AbilityDef::triggered(
        "When this artifact is put into a graveyard from the battlefield, draw a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static CHROMATIC_STAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d7a1357-debd-49b0-9fd5-560d5b3f589e"),
    "Chromatic Star",
    CardArt::new(
        "1d7a1357-debd-49b0-9fd5-560d5b3f589e",
        "Alex Horley-Orlandelli",
    ),
    CardSet::TimeSpiral,
    // A card that fixes one mana and replaces itself, and does the second
    // half however it dies rather than only when it is spent.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&STAR_ABILITIES),
);

// TSP 264 — Stuffy Doll
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STUFFY_DOLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14ca7425-a499-4864-b955-369ef2577849"),
    "Stuffy Doll",
    crate::card::CardArt::new("14ca7425-a499-4864-b955-369ef2577849", "Dave Allsop"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

static GEMSTONE_CAVERNS_OPENING_COST: [AbilityCostDef; 1] =
    [AbilityCostDef::ExileCardFromHand(ObjectPredicateDef::Any)];

// TSP 274 — Gemstone Caverns
// Audit: partial — The opening-hand action is declarative; its conditional mana replacement needs a mana ability that branches on a luck counter.
pub(in crate::card::sets) static GEMSTONE_CAVERNS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94d74254-4750-4fb3-9e53-473a5f98b315"),
    "Gemstone Caverns",
    CardArt::new("94d74254-4750-4fb3-9e53-473a5f98b315", "Martina Pilcerova"),
    CardSet::TimeSpiral,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::opening_hand_with(
                "If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.",
                PregameConditionDef::NotStartingPlayer,
                &GEMSTONE_CAVERNS_OPENING_COST,
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    from: Some(ZoneKind::Hand),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    controller: None,
                    arrival_effect: None,
                    attachment: None,
                    counters: Some(TokenCountersDef {
                        kind: CounterKind::Luck,
                        amount: ValueDef::Constant(1),
                    }),
                    tapped: false,
                },
            ),
            AbilityDef::not_implemented(
                "{T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.",
                "Needs a conditional activated-mana result keyed to a counter on its source.",
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOMENTARY_BLINK,
    &SERRA_AVENGER,
    &LOOTER_IL_KOR,
    &DREAD_RETURN,
    &SULFUROUS_BLAST,
    &CHROMATIC_STAR,
    &STUFFY_DOLL,
    &GEMSTONE_CAVERNS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
