//! Champions of Kamigawa cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AppliedEffectDef, CardArt, CardChoiceSourceDef, CardRules,
    CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    TriggerEventDef, TurnStepDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

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

// CHK 193 — Through the Breach
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&THROUGH_THE_BREACH, &SAKURA_TRIBE_ELDER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
