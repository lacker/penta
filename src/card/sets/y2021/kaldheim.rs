//! Kaldheim cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CopyExceptionsDef, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRelation, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

// KHM 46 — Behold the Multiverse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEHOLD_THE_MULTIVERSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27855a38-a682-4f97-ad22-ac625e86faec"),
    "Behold the Multiverse",
    crate::card::CardArt::new("27855a38-a682-4f97-ad22-ac625e86faec", "Magali Villeneuve"),
    crate::card::CardSet::Kaldheim,
    crate::card::CardRules::unsupported(),
);

// KHM 117 — Village Rites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILLAGE_RITES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c0f60a6-b5c8-4704-8b61-94e8fc463e5d"),
    "Village Rites",
    crate::card::CardArt::new("0fab9ee8-776a-48e5-b309-bcd381e67bf7", "Igor Kieryluk"),
    crate::card::CardSet::Kaldheim,
    crate::card::CardRules::unsupported(),
);

// KHM 139 — Goldspan Dragon
pub(in crate::card::sets) static GOLDSPAN_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d914868-9000-4df2-a818-0ef8a7f636ae"),
    "Goldspan Dragon",
    CardArt::new("9d914868-9000-4df2-a818-0ef8a7f636ae", "Andrew Mar"),
    CardSet::Kaldheim,
    // Five mana for a hasty 4/4 flier that attacks for four and pays for
    // itself: every attack and every removal spell aimed at him is two mana
    // back, which is why he so often lands and casts something the same turn.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature attacks or becomes the target of a spell, create a Treasure \
                 token.",
            // He pays for anything he is answered with: the Treasure lands whether the
            // spell that named him resolves or not, since the trigger is the targeting
            // rather than what it does.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                TriggerEventDef::BecomesTargetOfSpell(ObjectPredicateDef::Any),
            ]),
            EffectDef::create_token(tokens::treasure()).with_art(CardArt::new(
                "4ae9f454-4f8c-4123-9886-674bc439dfe7",
                "Olena Richards",
            )),
        ),
        AbilityDef::static_ability(
            "Treasures you control have \"{T}, Sacrifice this artifact: Add two mana of any one \
                 color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Treasure"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}, Sacrifice this artifact: Add two mana of any one color.",
                    // The granted ability sits beside the Treasure's own rather than replacing
                    // it, so a Treasure under him may still be cashed for one mana of any
                    // colour -- there is simply no reason to.
                    &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(2)),
                )),
            },
        ),
    ]),
);

// KHM 142 — Magda, Brazen Outlaw
pub(in crate::card::sets) static MAGDA_BRAZEN_OUTLAW: CardRecord = CardRecord::new_with_legacy_id(
    2298,
    "Magda, Brazen Outlaw",
    CardArt::new("079e6263-e54c-4899-a336-5315909b9322", "Slawomir Maniak"),
    CardSet::Kaldheim,
    // Two mana that turns every tap into a Treasure, and five Treasures into
    // whatever artifact the deck is built around.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Dwarves you control get +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        // "Other Dwarves you control": Magda pumps the rest of the Dwarves and not
                        // herself, which is the whole reason she is a 2/1 rather than a 3/1.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype("Dwarf"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Dwarf you control becomes tapped, create a Treasure token.",
                // Any Dwarf you control becoming tapped, not just an attack: tapping one
                // for mana or to pay a cost makes a Treasure just the same.
                TriggerEventDef::tapped(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Dwarf"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::create_token(tokens::treasure()).with_art(CardArt::new(
                    "4ae9f454-4f8c-4123-9886-674bc439dfe7",
                    "Olena Richards",
                )),
            ),
            AbilityDef::activated(
                "Sacrifice five Treasures: Search your library for an artifact or Dragon card, put that \
                 card onto the battlefield, then shuffle.",
                &[AbilityCostDef::SacrificePermanents {
                    object: ObjectPredicateDef::Subtype("Treasure"),
                    controller: PlayerRelation::You,
                    count: 5,
                }],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Subtype("Dragon"),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// KHM 157 — Tuskeri Firewalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUSKERI_FIREWALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a54d0170-a375-4e65-b98d-3e94a3aeef90"),
    "Tuskeri Firewalker",
    crate::card::CardArt::new(
        "a54d0170-a375-4e65-b98d-3e94a3aeef90",
        "Victor Adame Minguez",
    ),
    crate::card::CardSet::Kaldheim,
    crate::card::CardRules::unsupported(),
);

// KHM 192 — Sarulf's Packmate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SARULF_S_PACKMATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6061113e-7dd8-4739-b4dd-55bb7f9e39a2"),
    "Sarulf's Packmate",
    crate::card::CardArt::new("6061113e-7dd8-4739-b4dd-55bb7f9e39a2", "Ilse Gort"),
    crate::card::CardSet::Kaldheim,
    crate::card::CardRules::unsupported(),
);

// KHM 194 — Snakeskin Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAKESKIN_VEIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e692c208-c171-4964-9207-43c2cbc62845"),
    "Snakeskin Veil",
    crate::card::CardArt::new("e692c208-c171-4964-9207-43c2cbc62845", "Matt Stewart"),
    crate::card::CardSet::Kaldheim,
    crate::card::CardRules::unsupported(),
);

// KHM 315 — Esika's Chariot
pub(in crate::card::sets) static ESIKA_S_CHARIOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57a7d7e5-428d-4f42-8f13-9908fc65dcb4"),
    "Esika's Chariot",
    CardArt::new("57a7d7e5-428d-4f42-8f13-9908fc65dcb4", "WolfSkullJack"),
    CardSet::Kaldheim,
    // Four mana for four power of Cats, which then crew the Chariot they
    // came with -- and every attack after that is another one of them.
    CardRules::new_vehicle(mana_cost!("{3}{G}"), 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Esika's Chariot enters, create two 2/2 green Cat creature tokens.",
                EffectDef::create_creature_token(&["Cat"], &[ManaColor::Green], 2, 2)
                    .with_count(ValueDef::Constant(2))
                    .with_art(CardArt::new(
                        "2e07758f-0d1c-47d9-ba5a-43bc2a7423cd",
                        "Raoul Vitale",
                    )),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Esika's Chariot attacks, create a token that's a copy of target token you \
                 control.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // A token you control, which the Chariot itself is not: what it copies is
                // one of the Cats it brought, or anything else a token-making deck has
                // lying around.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                )],
                EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                        object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        exceptions: CopyExceptionsDef::NONE,
                    }),
            ),
            abilities::crew(
                "Crew 4 (Tap any number of creatures you control with total power 4 or more: This \
                 Vehicle becomes an artifact creature until end of turn.)",
                4,
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BEHOLD_THE_MULTIVERSE,
    &VILLAGE_RITES,
    &GOLDSPAN_DRAGON,
    &MAGDA_BRAZEN_OUTLAW,
    &TUSKERI_FIREWALKER,
    &SARULF_S_PACKMATE,
    &SNAKESKIN_VEIL,
    &ESIKA_S_CHARIOT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
