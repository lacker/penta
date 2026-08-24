//! Champions of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AppliedEffectDef, CardArt, CardChoiceSourceDef,
    CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, TopCardSelectionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::mana_cost;

// CHK 193 — Through the Breach
static A_CREATURE_CARD_IN_HAND: [CardChoiceSourceDef; 1] =
    [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

static BREACH_HASTE: AbilityDef = abilities::haste();

/// The creature sacrifices itself rather than being named by a delayed
/// trigger the spell installs: it is the object that arrived, and it carries
/// the clause with it. Nothing else can name it -- the card was chosen only
/// as this spell resolved, and what entered is a new object.
static BREACH_SACRIFICE_AT_END: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, sacrifice this creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
);

static BREACH_ARRIVAL: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::add_ability(&BREACH_HASTE),
    AppliedEffectDef::add_ability(&BREACH_SACRIFICE_AT_END),
]);

/// A minimum of zero is the printed "you may": the offer may be answered
/// with nothing, and with no creature in hand it is never made at all.
static BREACH_PUT_ONTO_BATTLEFIELD: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &A_CREATURE_CARD_IN_HAND,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    minimum: 0,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    arrival_effect: Some(&BREACH_ARRIVAL),
};

pub(in crate::card::sets) static THROUGH_THE_BREACH: CardRecord = CardRecord::new_with_legacy_id(
    2190,
    "Through the Breach",
    CardArt::new("6da09e6a-2965-4855-bd41-41b41ba188fb", "Hugh Jamieson"),
    CardSet::ChampionsOfKamigawa,
    CardRules::new_instant(mana_cost!("{4}{R}"))
        .with_subtypes(&["Arcane"])
        .with_abilities(&[
            AbilityDef::spell(
                "You may put a creature card from your hand onto the battlefield. That creature gains haste. Sacrifice that creature at the beginning of the next end step.",
                BREACH_PUT_ONTO_BATTLEFIELD,
            ),
            // Not a second spell ability: splice is a cast-time option on
            // the card in hand, which is why it reads as a static permission
            // rather than as something this spell does on resolution.
            AbilityDef::static_ability(
                "Splice onto Arcane {2}{R}{R} (As you cast an Arcane spell, you may reveal this card from your hand and pay its splice cost. If you do, add this card's effects to that spell.)",
                EffectDef::Special("Splice onto Arcane {2}{R}{R}"),
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Splice is not offered: casting an Arcane spell has no window for revealing another card from hand and adding its clause.",
            )),
        ]),
);

// CHK 239 — Sakura-Tribe Elder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAKURA_TRIBE_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91c7707a-bae0-4196-bf26-d276f57b7369"),
    "Sakura-Tribe Elder",
    crate::card::CardArt::new("91c7707a-bae0-4196-bf26-d276f57b7369", "Carl Critchlow"),
    crate::card::CardSet::ChampionsOfKamigawa,
    crate::card::CardRules::unsupported(),
);

// CHK 268 — Sensei's Divining Top
/// Every card looked at is selected, which is what makes the choice an
/// ordering rather than a filter: all three go back on top, in the order
/// they were named.
static TOP_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(3),
    object: None,
    minimum: 3,
    maximum: 3,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: true,
    then: None,
};

/// The draw and the trip back to the library are one clause: the Top is on
/// the battlefield as the card is drawn and gone by the time anything could
/// answer it, which is why it is never really spent.
static TOP_DRAWS_AND_LEAVES: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Source,
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
];

pub(in crate::card::sets) static SENSEIS_DIVINING_TOP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a08ca06-58db-4ce6-b490-be4bea8956a1"),
    "Sensei's Divining Top",
    CardArt::new("4a08ca06-58db-4ce6-b490-be4bea8956a1", "Michael Sutfin"),
    CardSet::ChampionsOfKamigawa,
    // One mana that fixes every draw for the rest of the game: the tap
    // trades the card it just arranged for itself, and the {1} sets up the
    // next one.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{1}: Look at the top three cards of your library, then put them back in any order.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &TOP_LOOK,
            },
        ),
        AbilityDef::activated(
            "{T}: Draw a card, then put this artifact on top of its owner's library.",
            &[AbilityCostDef::TapSource],
            EffectDef::Sequence(&TOP_DRAWS_AND_LEAVES),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THROUGH_THE_BREACH,
    &SAKURA_TRIBE_ELDER,
    &SENSEIS_DIVINING_TOP,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
