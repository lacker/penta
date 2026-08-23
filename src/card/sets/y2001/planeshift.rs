//! Planeshift cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef,
    BattlefieldEntryChoiceDestinationDef, BattlefieldEntryScalarChoiceDef, CardArt, CardRules,
    CardSet, CardType, ChoiceVisibilityDef, ChooseDef, CounterKind, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementChoiceDef,
    ReplacementEffectDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::ids::ObjectBindingIndex;
use crate::mana_cost;

// PLS 89 — Quirion Dryad
pub(in crate::card::sets) static QUIRION_DRYAD: CardRecord = CardRecord::new_with_legacy_id(
    291,
    "Quirion Dryad",
    CardArt::new("f6841ae6-b15f-488e-9cae-2cc5ec668278", "Don Hazeltine"),
    CardSet::Planeshift,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a spell that's white, blue, black, or red, put a +1/+1 counter on this creature.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::Color(ManaColor::Red),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

/// The lock is a player-facing rule rather than an object one: it names the
/// action, and the predicate reads the name the Mage chose on the way in.
static SPELLS_WITH_THE_CHOSEN_NAME: PlayRestrictionDef = PlayRestrictionDef::new(
    PlayActionMatcherDef::CastSpell,
    ObjectPredicateDef::HasSourcesChosenScalar(BattlefieldEntryChoiceDestinationDef::CardName),
);

// PLS 97 — Cavern Harpy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAVERN_HARPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adfb0804-50d6-4bca-8733-72e01030a543"),
    "Cavern Harpy",
    crate::card::CardArt::new("adfb0804-50d6-4bca-8733-72e01030a543", "Daren Bader"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 116 — Meddling Mage
pub(in crate::card::sets) static MEDDLING_MAGE: CardRecord = CardRecord::new_with_legacy_id(
    2050,
    "Meddling Mage",
    CardArt::new(
        "176f84c6-aa5e-449c-bd2b-cc91a898f0c7",
        "Christopher Moeller",
    ),
    CardSet::Planeshift,
    // Both players, which is why the mirror is miserable: the Mage does not
    // care who was going to cast the card it named.
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::replacement(
            "As this creature enters, choose a nonland card name.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::NONLAND_CARD_NAME,
            )),
        ),
        AbilityDef::static_ability(
            "Spells with the chosen name can't be cast.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    SPELLS_WITH_THE_CHOSEN_NAME,
                )),
            },
        ),
    ]),
);

/// A card from your own hand, whichever you can spare. The exile is the
/// upkeep cost of a land that would otherwise stay tapped forever.
static A_CARD_IN_YOUR_HAND: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Hand],
    PlayerSetDef::Related(PlayerRelation::You),
);

static CITY_EXILE_AND_UNTAP: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(A_CARD_IN_YOUR_HAND),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &EffectDef::Sequence(&[
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
            zone: ZoneKind::Exile,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
        EffectDef::Untap {
            object: EffectRecipientDef::Source,
        },
    ]),
});

// PLS 125 — Silver Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("726aa407-dadd-4575-aee2-b7888e67a722"),
    "Silver Drake",
    crate::card::CardArt::new("ac35ee86-96b2-47aa-a1ba-2988737f11ee", "Alan Pollack"),
    crate::card::CardSet::Planeshift,
    crate::card::CardRules::unsupported(),
);

// PLS 139 — Forsaken City
pub(in crate::card::sets) static FORSAKEN_CITY: CardRecord = CardRecord::new_with_legacy_id(
    2059,
    "Forsaken City",
    CardArt::new("676703fe-0e1a-4b40-9a2b-8b2e2c6b4a05", "Dana Knutson"),
    CardSet::Planeshift,
    // Perfect mana for a deck with cards to spare, and a dead land for one
    // without: the Stasis deck is holding a hand it is not casting anyway.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::static_ability(
            "This land doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may exile a card from your hand. If you do, untap this land.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &CITY_EXILE_AND_UNTAP,
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// PLS 143 — Treva's Ruins
pub(in crate::card::sets) static TREVAS_RUINS: CardRecord = CardRecord::new_with_legacy_id(
    2060,
    "Treva's Ruins",
    CardArt::new("8bae2458-7cfa-4e0e-9d55-2b2ef8d1c6a1", "Jerry Tiritilli"),
    CardSet::Planeshift,
    // Three colours for the price of a land drop you already made: the Lair
    // costs tempo rather than cards.
    CardRules::new_land(&["Lair"]).with_abilities(&[
        AbilityDef::triggered(
            "When this land enters, sacrifice it unless you return a non-Lair land you control to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::ReturnPermanentMatching(NON_LAIR_LAND_YOU_CONTROL),
                },
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&TREVA_COLORS)),
        ),
    ]),
);

static TREVA_COLORS: [ManaColor; 3] = [ManaColor::Green, ManaColor::White, ManaColor::Blue];

/// The Lair itself is excluded by its own subtype, so a second one cannot pay
/// for the first.
static NON_LAIR_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Lair")),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &QUIRION_DRYAD,
    &CAVERN_HARPY,
    &MEDDLING_MAGE,
    &SILVER_DRAKE,
    &FORSAKEN_CITY,
    &TREVAS_RUINS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
