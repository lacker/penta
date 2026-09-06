//! Onslaught cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y1999::urzas_destiny as catalog_uds;
use crate::card::sets::y1999::urzas_legacy as catalog_ulg;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CostDef, CounterKind, DamageEventMatcherDef, DamagePreventionDef,
    DiscardSelectionDef, EffectDef, EffectPaymentDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation,
    PlayerRuleDef, PlayerSetDef, ReplacementEffectDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, ScaledValueDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, tokens,
};
use crate::{TargetIndex, TurnStepDef, mana_cost};

const fn fetch_land(text: &'static str, land_types: &'static [BasicLandType]) -> CardRules {
    CardRules::new_land(&[]).with_ability(abilities::fetch_land_ability(
        text,
        ObjectPredicateDef::HasAnyBasicLandType(land_types),
    ))
}

// ONS 1 — Akroma's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKROMA_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3710c68-3f71-4d76-8bd2-001f0e8036f5"),
    "Akroma's Blessing",
    crate::card::CardArt::new("c3710c68-3f71-4d76-8bd2-001f0e8036f5", "Adam Rex"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 2 — Akroma's Vengeance
pub(in crate::card::sets) static AKROMAS_VENGEANCE: CardRecord = CardRecord::new_with_legacy_id(
    2023,
    "Akroma's Vengeance",
    CardArt::new(
        "5e33aaf7-7490-4b64-a966-82fbf7ca8686",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Onslaught,
    // Six mana is a lot for a sweeper, and the cycling is what makes it
    // maindeckable anyway: the card is never dead.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all artifacts, creatures, and enchantments.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
        ),
        abilities::cycling(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            mana_cost!("{3}"),
        ),
    ]),
);

// ONS 3 — Ancestor's Prophet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTOR_S_PROPHET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdee956e-76b1-4ba7-a387-2fbfb853507d"),
    "Ancestor's Prophet",
    crate::card::CardArt::new("cdee956e-76b1-4ba7-a387-2fbfb853507d", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 4 — Astral Slide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASTRAL_SLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d14993b6-ed8d-4b9b-b54c-2837b343a61e"),
    "Astral Slide",
    crate::card::CardArt::new("d14993b6-ed8d-4b9b-b54c-2837b343a61e", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 5 — Aura Extraction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_EXTRACTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55d16883-5e98-4dd2-92dd-0ba92f1099cb"),
    "Aura Extraction",
    crate::card::CardArt::new("55d16883-5e98-4dd2-92dd-0ba92f1099cb", "Luca Zontini"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 6 — Aurification
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURIFICATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93d9e9ea-9f88-4206-8960-b5ebe839ee16"),
    "Aurification",
    crate::card::CardArt::new("93d9e9ea-9f88-4206-8960-b5ebe839ee16", "Gary Ruddell"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 7 — Aven Brigadier
pub(in crate::card::sets) static AVEN_BRIGADIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da24ef56-8d54-4146-97e9-4abded807545"),
    "Aven Brigadier",
    CardArt::new("da24ef56-8d54-4146-97e9-4abded807545", "Greg Staples"),
    CardSet::Onslaught,
    // Six mana and three white pips for two anthems, which only a deck that
    // is both tribes at once ever gets paid for.
    CardRules::new_creature(mana_cost!("{3}{W}{W}{W}"), &["Bird", "Soldier"], 3, 5).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::static_ability(
                "Other Bird creatures get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Bird"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Other Soldier creatures get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Soldier"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ],
    ),
);

// ONS 8 — Aven Soulgazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_SOULGAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5189f152-f075-4090-97dd-b7686d813865"),
    "Aven Soulgazer",
    crate::card::CardArt::new("5189f152-f075-4090-97dd-b7686d813865", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 9 — Battlefield Medic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEFIELD_MEDIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c444503-42a8-4952-819b-bbca89b06abc"),
    "Battlefield Medic",
    crate::card::CardArt::new("9c444503-42a8-4952-819b-bbca89b06abc", "Matt Thompson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 10 — Catapult Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATAPULT_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a74d7aa2-c6ff-432d-b671-cef58c6736c6"),
    "Catapult Master",
    crate::card::CardArt::new("a74d7aa2-c6ff-432d-b671-cef58c6736c6", "Terese Nielsen"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 11 — Catapult Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATAPULT_SQUAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75a71d29-29eb-43c4-b0f3-457435e8f629"),
    "Catapult Squad",
    crate::card::CardArt::new("75a71d29-29eb-43c4-b0f3-457435e8f629", "Brian Snõddy"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 12 — Chain of Silence
pub(in crate::card::sets) static CHAIN_OF_SILENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a60ac8e-11eb-433f-86f9-8e593b38c617"),
    "Chain of Silence",
    crate::card::CardArt::new("9a60ac8e-11eb-433f-86f9-8e593b38c617", "Randy Gallegos"),
    crate::card::CardSet::Onslaught,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all damage target creature would deal this turn. That creature's controller may sacrifice a land of their choice. If the player does, they may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    cost: CostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                },
                &EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                        object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                        controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                },
            )),
        ]),
    )),
);

// ONS 13 — Circle of Solace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLE_OF_SOLACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07f567dc-8a60-40e1-b947-199872d8df08"),
    "Circle of Solace",
    crate::card::CardArt::new(
        "07f567dc-8a60-40e1-b947-199872d8df08",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 14 — Convalescent Care
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONVALESCENT_CARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48f3ad80-d000-496a-b704-d09e07981b6e"),
    "Convalescent Care",
    crate::card::CardArt::new("48f3ad80-d000-496a-b704-d09e07981b6e", "Greg Hildebrandt"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 15 — Crowd Favorites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWD_FAVORITES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1038436d-aea5-4508-8b37-c2cfa32c2771"),
    "Crowd Favorites",
    crate::card::CardArt::new("1038436d-aea5-4508-8b37-c2cfa32c2771", "Roger Raupp"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 16 — Crown of Awe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_AWE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aeaea4bc-dcea-4340-a039-ebc97b944673"),
    "Crown of Awe",
    crate::card::CardArt::new("aeaea4bc-dcea-4340-a039-ebc97b944673", "Randy Elliott"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 17 — Crude Rampart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUDE_RAMPART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af5d1be2-d6ae-4820-aa01-62f261b0f110"),
    "Crude Rampart",
    crate::card::CardArt::new("af5d1be2-d6ae-4820-aa01-62f261b0f110", "Sam Wood"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 18 — Daru Cavalier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_CAVALIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb2e9b7e-434e-477f-b3e8-e85ceb913650"),
    "Daru Cavalier",
    crate::card::CardArt::new("eb2e9b7e-434e-477f-b3e8-e85ceb913650", "Dany Orizio"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 19 — Daru Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e4f3eff-ac99-41e2-9003-9630cdb3ae23"),
    "Daru Healer",
    crate::card::CardArt::new("0e4f3eff-ac99-41e2-9003-9630cdb3ae23", "Dany Orizio"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 20 — Daru Lancer
pub(in crate::card::sets) static DARU_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd888ca8-0ebe-46f0-9317-3b193ccc43fb"),
    "Daru Lancer",
    CardArt::new("cd888ca8-0ebe-46f0-9317-3b193ccc43fb", "Brian Snõddy"),
    CardSet::Onslaught,
    // Six mana for a 3/4 first striker nobody casts, or three for a 2/2 that
    // wins a fight later. Morph is the whole reason the card sees play.
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Soldier"], 3, 4).with_abilities(
        &[
            abilities::first_strike(),
            AbilityDef::alternative_cast(
                mana_cost!("{3}"),
                crate::card::face_down::morph_cast(),
                Some(
                    "Morph {2}{W}{W} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
                ),
                EffectDef::None,
            ),
        ],
    ),
);

// ONS 21 — Daunting Defender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUNTING_DEFENDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38737f38-26bd-417c-b6b4-53f26e4e8044"),
    "Daunting Defender",
    crate::card::CardArt::new("38737f38-26bd-417c-b6b4-53f26e4e8044", "Carl Critchlow"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 22 — Dawning Purist
pub(in crate::card::sets) static DAWNING_PURIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8cb25b0-e4c3-4a4e-b722-ea30e695f917"),
    "Dawning Purist",
    CardArt::new("b8cb25b0-e4c3-4a4e-b722-ea30e695f917", "Brian Snõddy"),
    CardSet::Onslaught,
    // Enchantment removal that has to earn the right to fire, which is the
    // trade a sideboard card makes to be worth a maindeck slot.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 2, 2)
        .with_morph(mana_cost!("{1}{W}"))
        .with_ability(AbilityDef::triggered_with_targets(
            "Whenever this creature deals combat damage to a player, you may destroy target enchantment that player controls.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        )),
);

// ONS 23 — Defensive Maneuvers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSIVE_MANEUVERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58f9eb25-4140-4ecf-bcaa-1b193d884007"),
    "Defensive Maneuvers",
    crate::card::CardArt::new("58f9eb25-4140-4ecf-bcaa-1b193d884007", "Luca Zontini"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 24 — Demystify (reprint)

// ONS 25 — Disciple of Grace (reprint)

// ONS 26 — Dive Bomber
pub(in crate::card::sets) static DIVE_BOMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65162b24-8a3b-4b92-a831-6f23f809c76f"),
    "Dive Bomber",
    CardArt::new("65162b24-8a3b-4b92-a831-6f23f809c76f", "Randy Gallegos"),
    CardSet::Onslaught,
    // A flier that trades itself for whatever is already in combat, which is
    // two cards' worth of work in a format of small creatures.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals 2 damage to target attacking or blocking creature.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]))]
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ONS 27 — Doubtless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUBTLESS_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0dedef8a-5527-40dc-9ad9-bcee4cf30a76"),
    "Doubtless One",
    crate::card::CardArt::new("0dedef8a-5527-40dc-9ad9-bcee4cf30a76", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 28 — Exalted Angel
pub(in crate::card::sets) static EXALTED_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    2076,
    "Exalted Angel",
    CardArt::new("d75cc975-0f7e-48e7-a693-453306e5a907", "Michael Sutfin"),
    CardSet::Onslaught,
    // Six mana is more than a control deck wants to pay on turn four, so it
    // comes down face down on three and stands up on the next turn instead.
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel"], 4, 5)
        .with_morph(mana_cost!("{2}{W}{W}"))
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{3}"),
                crate::card::face_down::morph_cast(),
                Some(
                    "Morph {2}{W}{W} (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its morph cost.)",
                ),
                EffectDef::None,
            ),
        ]),
);

// ONS 29 — Foothill Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOOTHILL_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("409adb7b-6dcb-4e7f-a5dd-c0adf12140a4"),
    "Foothill Guide",
    crate::card::CardArt::new("409adb7b-6dcb-4e7f-a5dd-c0adf12140a4", "Eric Peterson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 30 — Glarecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLARECASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e505e8e-51aa-4415-81e6-cf022279edb0"),
    "Glarecaster",
    crate::card::CardArt::new("7e505e8e-51aa-4415-81e6-cf022279edb0", "Dan Frazier"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 31 — Glory Seeker
pub(in crate::card::sets) static GLORY_SEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9047075e-9fca-484d-bb79-32c0d6821281"),
    "Glory Seeker",
    CardArt::new("9047075e-9fca-484d-bb79-32c0d6821281", "Dave Dorman"),
    CardSet::Onslaught,
    // A vanilla 2/2 for two, printed as a Soldier for the tribal deck that
    // cares which body it is.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 2),
);

// ONS 32 — Grassland Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRASSLAND_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c129f361-8769-4f9a-9745-eb5d0c085b88"),
    "Grassland Crusader",
    crate::card::CardArt::new("c129f361-8769-4f9a-9745-eb5d0c085b88", "Mark Tedin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 33 — Gravel Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEL_SLINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87551307-6b5f-4f12-aa1f-4beebefad3b3"),
    "Gravel Slinger",
    crate::card::CardArt::new("87551307-6b5f-4f12-aa1f-4beebefad3b3", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 34 — Gustcloak Harrier
pub(in crate::card::sets) static GUSTCLOAK_HARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5ff5c7d-7823-4d1e-8abb-77e2d8126996"),
    "Gustcloak Harrier",
    CardArt::new("b5ff5c7d-7823-4d1e-8abb-77e2d8126996", "Dan Frazier"),
    CardSet::Onslaught,
    // Flying and an escape, so blocking it needs a flier and stops nothing
    // even then.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Bird", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may untap it and remove it from combat.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            },
        ),
    ]),
);

// ONS 35 — Gustcloak Runner
pub(in crate::card::sets) static GUSTCLOAK_RUNNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb227f65-9189-41ed-94a0-2aa21cad26f5"),
    "Gustcloak Runner",
    CardArt::new("eb227f65-9189-41ed-94a0-2aa21cad26f5", "Glen Angus"),
    CardSet::Onslaught,
    // Blocking it achieves nothing, so it either connects or wastes the
    // defender's turn -- and it is untapped either way.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may untap it and remove it from combat.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            },
        ),
    ),
);

// ONS 36 — Gustcloak Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTCLOAK_SAVIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e9d6e81-1869-4ab7-8a4e-477d5c4aed6b"),
    "Gustcloak Savior",
    crate::card::CardArt::new("0e9d6e81-1869-4ab7-8a4e-477d5c4aed6b", "Jim Nelson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 37 — Gustcloak Sentinel
pub(in crate::card::sets) static GUSTCLOAK_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b90da5c3-fd8f-445d-809f-e129870d7449"),
    "Gustcloak Sentinel",
    CardArt::new("b90da5c3-fd8f-445d-809f-e129870d7449", "Mark Zug"),
    CardSet::Onslaught,
    // The same escape on a body big enough that blocking it was the
    // defender's only real option.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Soldier"], 3, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may untap it and remove it from combat.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            },
        ),
    ),
);

// ONS 38 — Gustcloak Skirmisher
pub(in crate::card::sets) static GUSTCLOAK_SKIRMISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbbff06c-5f92-4320-8b70-df3c8344f600"),
    "Gustcloak Skirmisher",
    CardArt::new("cbbff06c-5f92-4320-8b70-df3c8344f600", "Dan Frazier"),
    CardSet::Onslaught,
    // The same escape a size up, where the extra toughness makes blocking
    // it pointless twice over.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may untap it and remove it from combat.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            },
        ),
    ]),
);

// ONS 39 — Harsh Mercy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARSH_MERCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6473b4d-1f59-4216-ace9-f3e5306266fb"),
    "Harsh Mercy",
    crate::card::CardArt::new("b6473b4d-1f59-4216-ace9-f3e5306266fb", "John Matson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 40 — Improvised Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPROVISED_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d7d5d79-73d8-4f1a-9dda-4de5f41539d9"),
    "Improvised Armor",
    crate::card::CardArt::new("8d7d5d79-73d8-4f1a-9dda-4de5f41539d9", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 41 — Inspirit
pub(in crate::card::sets) static INSPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55e0e300-db79-4328-ba1d-9c3910e47f52"),
    "Inspirit",
    CardArt::new("55e0e300-db79-4328-ba1d-9c3910e47f52", "Keith Garletts"),
    CardSet::Onslaught,
    // The white version, which trades a point of power for two of
    // toughness and a mana.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target creature. It gets +2/+4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ONS 42 — Ironfist Crusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRONFIST_CRUSHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7284e32-de54-4c83-a7de-7b249c47319a"),
    "Ironfist Crusher",
    crate::card::CardArt::new("c7284e32-de54-4c83-a7de-7b249c47319a", "Iain McCaig"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 43 — Jareth, Leonine Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JARETH_LEONINE_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65dd1364-ff36-4cb9-ad93-e6fcbcb942cf"),
    "Jareth, Leonine Titan",
    crate::card::CardArt::new("65dd1364-ff36-4cb9-ad93-e6fcbcb942cf", "Daren Bader"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 44 — Mobilization
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOBILIZATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("653cc07b-0f53-4b5b-9c5f-885b8b4a6e5f"),
    "Mobilization",
    crate::card::CardArt::new("653cc07b-0f53-4b5b-9c5f-885b8b4a6e5f", "Carl Critchlow"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 45 — Nova Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOVA_CLERIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2048d84-b5e6-405c-9091-1997a0c4e1a5"),
    "Nova Cleric",
    crate::card::CardArt::new("b2048d84-b5e6-405c-9091-1997a0c4e1a5", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 46 — Oblation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBLATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58561356-4a97-467b-88e5-412e633715fb"),
    "Oblation",
    crate::card::CardArt::new("58561356-4a97-467b-88e5-412e633715fb", "Doug Chaffee"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 47 — Pacifism (reprint)

// ONS 48 — Pearlspear Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEARLSPEAR_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1ea7219-6ab6-471a-afe7-d7da1df434c7"),
    "Pearlspear Courier",
    crate::card::CardArt::new("a1ea7219-6ab6-471a-afe7-d7da1df434c7", "Dany Orizio"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 49 — Piety Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIETY_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bc2da43-c0e1-4fbf-b309-a75e105c29c1"),
    "Piety Charm",
    crate::card::CardArt::new("1bc2da43-c0e1-4fbf-b309-a75e105c29c1", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 50 — Renewed Faith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENEWED_FAITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ea572b5-ff68-45aa-8200-78ee7f64a0ce"),
    "Renewed Faith",
    crate::card::CardArt::new("1ea572b5-ff68-45aa-8200-78ee7f64a0ce", "Dave Dorman"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 51 — Righteous Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIGHTEOUS_CAUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83c6245-4b37-430d-af10-2581804fff08"),
    "Righteous Cause",
    crate::card::CardArt::new("b83c6245-4b37-430d-af10-2581804fff08", "Scott M. Fischer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 52 — Sandskin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSKIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80b59844-c9d4-4bc1-86e6-4cc596d9165d"),
    "Sandskin",
    crate::card::CardArt::new("80b59844-c9d4-4bc1-86e6-4cc596d9165d", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 53 — Shared Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARED_TRIUMPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d07ebe6-76cf-4345-b59b-9954496c44d0"),
    "Shared Triumph",
    crate::card::CardArt::new("0d07ebe6-76cf-4345-b59b-9954496c44d0", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 54 — Shieldmage Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELDMAGE_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efa2d660-7c93-4087-a6e5-49c2ad21eb5a"),
    "Shieldmage Elder",
    crate::card::CardArt::new(
        "efa2d660-7c93-4087-a6e5-49c2ad21eb5a",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 55 — Sigil of the New Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIGIL_OF_THE_NEW_DAWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca1babca-b285-4b00-8b46-ed946c9a027f"),
    "Sigil of the New Dawn",
    crate::card::CardArt::new("ca1babca-b285-4b00-8b46-ed946c9a027f", "Tony Szczudlo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 56 — Sunfire Balm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNFIRE_BALM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d563ebb-ecd1-406c-9d69-c101acdeced7"),
    "Sunfire Balm",
    crate::card::CardArt::new(
        "0d563ebb-ecd1-406c-9d69-c101acdeced7",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 57 — True Believer
pub(in crate::card::sets) static TRUE_BELIEVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4289bdcb-6eea-458f-a4eb-89e26264673a"),
    "True Believer",
    CardArt::new(
        "4289bdcb-6eea-458f-a4eb-89e26264673a",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Onslaught,
    // The same protection on a body, which is the whole reason it is
    // worth killing and the whole reason that is hard.
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "You have shroud.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(PlayerRuleDef::Shroud)),
            },
        ),
    ),
);

// ONS 58 — Unified Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNIFIED_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29906eca-0823-4cd6-890f-e5b93cc50a11"),
    "Unified Strike",
    crate::card::CardArt::new("29906eca-0823-4cd6-890f-e5b93cc50a11", "Dave Dorman"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 59 — Weathered Wayfarer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHERED_WAYFARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6601ab1-3862-4aff-82be-be15493fe4b0"),
    "Weathered Wayfarer",
    crate::card::CardArt::new(
        "f6601ab1-3862-4aff-82be-be15493fe4b0",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 60 — Whipcorder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPCORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bf6987e-a6e4-4a88-af0b-cf3b2d2b80c7"),
    "Whipcorder",
    crate::card::CardArt::new("3bf6987e-a6e4-4a88-af0b-cf3b2d2b80c7", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 61 — Words of Worship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WORSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ea5c6e0-8361-4214-997b-32a66b19fae9"),
    "Words of Worship",
    crate::card::CardArt::new("0ea5c6e0-8361-4214-997b-32a66b19fae9", "Rebecca Guay"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 62 — Airborne Aid
pub(in crate::card::sets) static AIRBORNE_AID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0aaa43b0-601f-4b99-a328-541b04d5696d"),
    "Airborne Aid",
    CardArt::new("0aaa43b0-601f-4b99-a328-541b04d5696d", "Bradley Williams"),
    CardSet::Onslaught,
    // It counts every Bird on the table, including the opponent's, which
    // is generous right up until they have none.
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Draw a card for each Bird on the battlefield.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype("Bird"),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            )),
        },
    )),
);

// ONS 63 — Annex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c95d5cb7-3121-430b-80c3-84c75e5f869e"),
    "Annex",
    crate::card::CardArt::new("c95d5cb7-3121-430b-80c3-84c75e5f869e", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 64 — Aphetto Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_ALCHEMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfd2628f-63c4-4e19-83ea-26041650faab"),
    "Aphetto Alchemist",
    crate::card::CardArt::new("dfd2628f-63c4-4e19-83ea-26041650faab", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 65 — Aphetto Grifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_GRIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a7a7bf3-1b0c-415d-9c57-73ac55b1f915"),
    "Aphetto Grifter",
    crate::card::CardArt::new("3a7a7bf3-1b0c-415d-9c57-73ac55b1f915", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 66 — Arcanis the Omnipotent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCANIS_THE_OMNIPOTENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90865f52-c062-4505-a204-b4d7d4b3fc4c"),
    "Arcanis the Omnipotent",
    crate::card::CardArt::new("90865f52-c062-4505-a204-b4d7d4b3fc4c", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 67 — Artificial Evolution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARTIFICIAL_EVOLUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f46894d1-2503-43fa-938e-7bbf19101d13"),
    "Artificial Evolution",
    crate::card::CardArt::new("f46894d1-2503-43fa-938e-7bbf19101d13", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 68 — Ascending Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASCENDING_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd8b17df-615c-4cc1-af1a-2fc35a985af9"),
    "Ascending Aven",
    crate::card::CardArt::new("bd8b17df-615c-4cc1-af1a-2fc35a985af9", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 69 — Aven Fateshaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_FATESHAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a4b41c4-0d14-4b9c-8e0c-a626ba6b104d"),
    "Aven Fateshaper",
    crate::card::CardArt::new("7a4b41c4-0d14-4b9c-8e0c-a626ba6b104d", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 70 — Backslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACKSLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47c40269-80a5-454f-83dd-dae1c11500c0"),
    "Backslide",
    crate::card::CardArt::new("47c40269-80a5-454f-83dd-dae1c11500c0", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 71 — Blatant Thievery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLATANT_THIEVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8284476c-a7c8-4a6c-8021-ee997e9270ce"),
    "Blatant Thievery",
    crate::card::CardArt::new("8284476c-a7c8-4a6c-8021-ee997e9270ce", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 72 — Callous Oppressor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_OPPRESSOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3dd3ce7-e0e3-4412-9983-ff933584f59b"),
    "Callous Oppressor",
    crate::card::CardArt::new("b3dd3ce7-e0e3-4412-9983-ff933584f59b", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 73 — Chain of Vapor
pub(in crate::card::sets) static CHAIN_OF_VAPOR: CardRecord = CardRecord::new_with_legacy_id(
    2062,
    "Chain of Vapor",
    CardArt::new("30f6b4a2-4e64-4d0e-9dbb-2b6a5b8f5b1f", "Carl Critchlow"),
    CardSet::Onslaught,
    // One mana to undo anything, and the chain is the opponent's to continue
    // or stop.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. Then that permanent's controller may sacrifice a land of their choice. If the player does, they may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
},
            // A land of their choice, sacrificed by whoever just had a permanent
            // bounced. Paying buys the copy, which is what turns one Chain of Vapor into
            // a board sweep in a deck holding the lands to spend.
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef {
                    payer: PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    cost: CostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                },
                &EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                        object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                        controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                },
            )),
        ]),
    )),
);

// ONS 74 — Choking Tethers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_TETHERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4de14d1-441f-4d65-bd12-df0506530015"),
    "Choking Tethers",
    crate::card::CardArt::new("d4de14d1-441f-4d65-bd12-df0506530015", "Carl Critchlow"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 75 — Clone (reprint)

// ONS 76 — Complicate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPLICATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33f69670-e494-42b8-9148-fe105ec61aa0"),
    "Complicate",
    crate::card::CardArt::new("33f69670-e494-42b8-9148-fe105ec61aa0", "Scott M. Fischer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 77 — Crafty Pathmage
pub(in crate::card::sets) static CRAFTY_PATHMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5d91378-f831-40ef-a79b-b044af1470e0"),
    "Crafty Pathmage",
    CardArt::new("c5d91378-f831-40ef-a79b-b044af1470e0", "Wayne England"),
    CardSet::Onslaught,
    // The blue printing of the same effect, and a Wizard, which is what
    // Onslaught cared about.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature with power 2 or less can't be blocked this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    // Read when the ability is activated and again when it
                    // resolves, so a creature pumped in response is no
                    // longer a legal target.
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 78 — Crown of Ascension
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_ASCENSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fe86733-7851-4c2a-8d94-dba6f071b94d"),
    "Crown of Ascension",
    crate::card::CardArt::new("2fe86733-7851-4c2a-8d94-dba6f071b94d", "Bradley Williams"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 79 — Discombobulate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCOMBOBULATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cef584c5-6e2d-419b-9c11-a1b6c9c9ab2a"),
    "Discombobulate",
    crate::card::CardArt::new(
        "cef584c5-6e2d-419b-9c11-a1b6c9c9ab2a",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 80 — Dispersing Orb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISPERSING_ORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69db0298-f6d5-450f-add3-a28c0a43f33f"),
    "Dispersing Orb",
    crate::card::CardArt::new("69db0298-f6d5-450f-add3-a28c0a43f33f", "Rebecca Guay"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 81 — Disruptive Pitmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPTIVE_PITMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b0d9c2f-356c-4f27-8560-8ffceadac31c"),
    "Disruptive Pitmage",
    crate::card::CardArt::new("5b0d9c2f-356c-4f27-8560-8ffceadac31c", "Darrell Riche"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 82 — Essence Fracture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FRACTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df0b6c7a-0891-492d-8e07-6a198bf2ccc4"),
    "Essence Fracture",
    crate::card::CardArt::new("df0b6c7a-0891-492d-8e07-6a198bf2ccc4", "Wayne England"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 83 — Fleeting Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETING_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("246a2758-0096-43b9-8193-d6ae5b41b6e6"),
    "Fleeting Aven",
    crate::card::CardArt::new("246a2758-0096-43b9-8193-d6ae5b41b6e6", "Iain McCaig"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 84 — Future Sight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUTURE_SIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("688bd665-4948-4961-aec5-f17782257f9b"),
    "Future Sight",
    crate::card::CardArt::new("688bd665-4948-4961-aec5-f17782257f9b", "Matt Cavotta"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 85 — Ghosthelm Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTHELM_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd6cc30a-9ed4-4f36-95cb-6f0a2b8dce02"),
    "Ghosthelm Courier",
    crate::card::CardArt::new(
        "cd6cc30a-9ed4-4f36-95cb-6f0a2b8dce02",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 86 — Graxiplon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAXIPLON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c16e565-0b7f-46b1-a091-64c47c923a9f"),
    "Graxiplon",
    crate::card::CardArt::new("0c16e565-0b7f-46b1-a091-64c47c923a9f", "Iain McCaig"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 87 — Imagecrafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMAGECRAFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91be6441-8a45-43e4-8d12-a886dcaadbd3"),
    "Imagecrafter",
    crate::card::CardArt::new("91be6441-8a45-43e4-8d12-a886dcaadbd3", "Terese Nielsen"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 88 — Information Dealer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFORMATION_DEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a45ac59c-654d-44de-b266-532d44b34137"),
    "Information Dealer",
    crate::card::CardArt::new("a45ac59c-654d-44de-b266-532d44b34137", "Jerry Tiritilli"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 89 — Ixidor, Reality Sculptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IXIDOR_REALITY_SCULPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("314d5e89-55f7-42b4-af19-d4d0f499a265"),
    "Ixidor, Reality Sculptor",
    crate::card::CardArt::new("314d5e89-55f7-42b4-af19-d4d0f499a265", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 90 — Ixidor's Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IXIDOR_S_WILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b713448-853a-41ee-a302-963e9c1c1c65"),
    "Ixidor's Will",
    crate::card::CardArt::new("1b713448-853a-41ee-a302-963e9c1c1c65", "Eric Peterson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 91 — Mage's Guile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGE_S_GUILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("301cb538-a931-4916-927b-4986046b1158"),
    "Mage's Guile",
    crate::card::CardArt::new(
        "301cb538-a931-4916-927b-4986046b1158",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 92 — Meddle (reprint)

// ONS 93 — Mistform Dreamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_DREAMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff34e303-c94a-4f5f-b9f6-8d48e6aac383"),
    "Mistform Dreamer",
    crate::card::CardArt::new("ff34e303-c94a-4f5f-b9f6-8d48e6aac383", "Matthew Mitchell"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 94 — Mistform Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fbbb075-5795-425f-9e33-70cb922eea16"),
    "Mistform Mask",
    crate::card::CardArt::new(
        "7fbbb075-5795-425f-9e33-70cb922eea16",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 95 — Mistform Mutant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_MUTANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a25b2697-5d7f-490a-8474-c775096e681e"),
    "Mistform Mutant",
    crate::card::CardArt::new("a25b2697-5d7f-490a-8474-c775096e681e", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 96 — Mistform Shrieker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SHRIEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1082eea2-5e83-48d4-b02b-a22e7cbe2054"),
    "Mistform Shrieker",
    crate::card::CardArt::new("1082eea2-5e83-48d4-b02b-a22e7cbe2054", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 97 — Mistform Skyreaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SKYREAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e394e096-ea70-4813-9039-e4bd065d0a17"),
    "Mistform Skyreaver",
    crate::card::CardArt::new("e394e096-ea70-4813-9039-e4bd065d0a17", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 98 — Mistform Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e80d109-b73f-4b5d-b9e4-534e8d69633f"),
    "Mistform Stalker",
    crate::card::CardArt::new("9e80d109-b73f-4b5d-b9e4-534e8d69633f", "Randy Gallegos"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 99 — Mistform Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebaa7a26-8516-4d71-a524-77b2d3f030d5"),
    "Mistform Wall",
    crate::card::CardArt::new("ebaa7a26-8516-4d71-a524-77b2d3f030d5", "Franz Vohwinkel"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 100 — Nameless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAMELESS_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79cf3535-3f80-4b76-aad3-dd851e6885a6"),
    "Nameless One",
    crate::card::CardArt::new("79cf3535-3f80-4b76-aad3-dd851e6885a6", "Mark Tedin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 101 — Peer Pressure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEER_PRESSURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be0110ba-49e4-4729-8a84-4d408b20df53"),
    "Peer Pressure",
    crate::card::CardArt::new(
        "be0110ba-49e4-4729-8a84-4d408b20df53",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 102 — Psychic Trance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_TRANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5e55695-16cc-4373-8078-959f1ded4c6d"),
    "Psychic Trance",
    crate::card::CardArt::new("d5e55695-16cc-4373-8078-959f1ded4c6d", "Rebecca Guay"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 103 — Quicksilver Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e93577bd-2711-443c-aa88-a235345d7800"),
    "Quicksilver Dragon",
    crate::card::CardArt::new("e93577bd-2711-443c-aa88-a235345d7800", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 104 — Read the Runes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static READ_THE_RUNES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc148c21-cbe6-4cea-899b-e62501b59a00"),
    "Read the Runes",
    crate::card::CardArt::new("bc148c21-cbe6-4cea-899b-e62501b59a00", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 105 — Reminisce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMINISCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5f246e3-2193-4820-9c59-07b480300fbe"),
    "Reminisce",
    crate::card::CardArt::new("b5f246e3-2193-4820-9c59-07b480300fbe", "Bradley Williams"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 106 — Riptide Biologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_BIOLOGIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d399b71-c365-492c-976e-2c79d97d08bc"),
    "Riptide Biologist",
    crate::card::CardArt::new("4d399b71-c365-492c-976e-2c79d97d08bc", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 107 — Riptide Chronologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_CHRONOLOGIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3767f568-36b1-4064-835e-4dd7576b7b8b"),
    "Riptide Chronologist",
    crate::card::CardArt::new("3767f568-36b1-4064-835e-4dd7576b7b8b", "Roger Raupp"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 108 — Riptide Entrancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_ENTRANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cd9abc9-f289-4294-bc0f-4addc8b92a4e"),
    "Riptide Entrancer",
    crate::card::CardArt::new("2cd9abc9-f289-4294-bc0f-4addc8b92a4e", "Scott Hampton"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 109 — Riptide Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_SHAPESHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85be34ac-7bc2-4da2-8d9c-2412b9946073"),
    "Riptide Shapeshifter",
    crate::card::CardArt::new("85be34ac-7bc2-4da2-8d9c-2412b9946073", "Arnie Swekel"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 110 — Rummaging Wizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUMMAGING_WIZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad96e158-bf2b-4f3e-9692-0f79efdd94f5"),
    "Rummaging Wizard",
    crate::card::CardArt::new("ad96e158-bf2b-4f3e-9692-0f79efdd94f5", "Jerry Tiritilli"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 111 — Sage Aven
pub(in crate::card::sets) static SAGE_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c03afc5-7ca3-4ac6-a06e-091e2cce13a0"),
    "Sage Aven",
    CardArt::new("4c03afc5-7ca3-4ac6-a06e-091e2cce13a0", "Randy Gallegos"),
    CardSet::Onslaught,
    // The same trigger on a body that survives combat, and a Wizard, which
    // is what Onslaught was counting.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Wizard"], 1, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your library, then \
             put them back in any order.",
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// ONS 112 — Screaming Seahawk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREAMING_SEAHAWK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc5856ac-e710-44ee-8516-6070f4f31ce5"),
    "Screaming Seahawk",
    crate::card::CardArt::new("cc5856ac-e710-44ee-8516-6070f4f31ce5", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 113 — Sea's Claim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_S_CLAIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb652a5c-464e-4ba4-a4ab-1181be70cf7a"),
    "Sea's Claim",
    crate::card::CardArt::new("fb652a5c-464e-4ba4-a4ab-1181be70cf7a", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 114 — Slipstream Eel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIPSTREAM_EEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9d06a1f-00b7-440d-849d-efc466d73f29"),
    "Slipstream Eel",
    crate::card::CardArt::new("e9d06a1f-00b7-440d-849d-efc466d73f29", "Mark Tedin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 115 — Spy Network
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPY_NETWORK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a4bed3f-845c-4822-b8af-8b511dce6fe2"),
    "Spy Network",
    crate::card::CardArt::new("8a4bed3f-845c-4822-b8af-8b511dce6fe2", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 116 — Standardize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STANDARDIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2c79e64-91bf-4e87-a4fd-3136ea67c5bb"),
    "Standardize",
    crate::card::CardArt::new("f2c79e64-91bf-4e87-a4fd-3136ea67c5bb", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 117 — Supreme Inquisitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPREME_INQUISITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("867de3d2-2178-4931-823e-ff439e1a45ea"),
    "Supreme Inquisitor",
    crate::card::CardArt::new("867de3d2-2178-4931-823e-ff439e1a45ea", "rk post"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 118 — Trade Secrets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_SECRETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e92e197e-ef7e-46bb-9533-5f9819d545b2"),
    "Trade Secrets",
    crate::card::CardArt::new("e92e197e-ef7e-46bb-9533-5f9819d545b2", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 119 — Trickery Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICKERY_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32a2ee45-7f1d-40a8-82b4-ab3b705417ea"),
    "Trickery Charm",
    crate::card::CardArt::new("32a2ee45-7f1d-40a8-82b4-ab3b705417ea", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 120 — Voidmage Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOIDMAGE_PRODIGY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7441e7f9-a326-4f61-b7b1-e0dbed06046f"),
    "Voidmage Prodigy",
    crate::card::CardArt::new("7441e7f9-a326-4f61-b7b1-e0dbed06046f", "Scott M. Fischer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 121 — Wheel and Deal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHEEL_AND_DEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61f50a1a-f3d0-4fcf-bd32-0e173b0d3247"),
    "Wheel and Deal",
    crate::card::CardArt::new("61f50a1a-f3d0-4fcf-bd32-0e173b0d3247", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 122 — Words of Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5595a57a-a76c-467b-afaf-5affffc24f35"),
    "Words of Wind",
    crate::card::CardArt::new("5595a57a-a76c-467b-afaf-5affffc24f35", "Eric Peterson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 123 — Accursed Centaur
pub(in crate::card::sets) static ACCURSED_CENTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("894556d8-6d5c-431b-a45d-26cd37c5f456"),
    "Accursed Centaur",
    CardArt::new("894556d8-6d5c-431b-a45d-26cd37c5f456", "Jerry Tiritilli"),
    CardSet::Onslaught,
    // A 2/2 for one that costs a creature, which is free only on an empty
    // board and never again.
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie", "Centaur"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, sacrifice a creature.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// ONS 124 — Anurid Murkdiver
pub(in crate::card::sets) static ANURID_MURKDIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e43d62c-488a-4c8d-b193-bacbf8037761"),
    "Anurid Murkdiver",
    CardArt::new("9e43d62c-488a-4c8d-b193-bacbf8037761", "Dany Orizio"),
    CardSet::Onslaught,
    // Six mana for four evasive damage a turn, but only against the deck
    // that also has Swamps.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Zombie", "Frog", "Beast"], 4, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// ONS 125 — Aphetto Dredging
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_DREDGING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4e7fadf-40f1-45ff-97ef-5830381accc9"),
    "Aphetto Dredging",
    crate::card::CardArt::new(
        "c4e7fadf-40f1-45ff-97ef-5830381accc9",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 126 — Aphetto Vulture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_VULTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("107492b9-03a8-4d53-a0cf-4814ffbec409"),
    "Aphetto Vulture",
    crate::card::CardArt::new("107492b9-03a8-4d53-a0cf-4814ffbec409", "Tony Szczudlo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 127 — Blackmail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACKMAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b40f6eb-e2a4-46d2-8822-b0f3dc508b73"),
    "Blackmail",
    crate::card::CardArt::new(
        "9b40f6eb-e2a4-46d2-8822-b0f3dc508b73",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 128 — Boneknitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONEKNITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9d58030-a95a-4221-93bc-30a59344e30b"),
    "Boneknitter",
    crate::card::CardArt::new("c9d58030-a95a-4221-93bc-30a59344e30b", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 129 — Cabal Archon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_ARCHON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bdf6e2a-1bf5-4d63-a58b-883cfb1ea0fa"),
    "Cabal Archon",
    crate::card::CardArt::new("4bdf6e2a-1bf5-4d63-a58b-883cfb1ea0fa", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 130 — Cabal Executioner
pub(in crate::card::sets) static CABAL_EXECUTIONER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd7727a7-0cdf-4fd5-82b4-e6587c10ca80"),
    "Cabal Executioner",
    CardArt::new("cd7727a7-0cdf-4fd5-82b4-e6587c10ca80", "Rebecca Guay"),
    CardSet::Onslaught,
    // Edict on a stick, which asks the defender to keep a blocker back and
    // punishes them for it in the same swing.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Cleric"], 2, 2)
        .with_morph(mana_cost!("{3}{B}{B}"))
        .with_ability(AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player sacrifices a creature of their choice.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        )),
);

// ONS 131 — Cabal Slaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_SLAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9c04fd3-021a-4011-be9b-0d268557aa06"),
    "Cabal Slaver",
    crate::card::CardArt::new("b9c04fd3-021a-4011-be9b-0d268557aa06", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 132 — Chain of Smog
pub(in crate::card::sets) static CHAIN_OF_SMOG: CardRecord = CardRecord::new_with_legacy_id(
    2155,
    "Chain of Smog",
    CardArt::new("6bfe64f9-8b03-41f6-a47b-fade397ad9d1", "Greg Staples"),
    CardSet::Onslaught,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards. That player may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            // The copy costs nothing here, unlike Chain of Vapor's land. Whoever was
            // just hit decides whether to pass it on, and picks the next target -- which
            // is why the chain usually stops at whoever cannot afford to keep it going.
            EffectDef::May {
                player: EffectRecipientDef::player(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                    controller: PlayerRefDef::Target(TargetIndex::PRIMARY),
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ]),
    )),
);

// ONS 133 — Cover of Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COVER_OF_DARKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d6d7d88-d82b-40f4-bf57-ec5d7c480689"),
    "Cover of Darkness",
    crate::card::CardArt::new("0d6d7d88-d82b-40f4-bf57-ec5d7c480689", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 134 — Crown of Suspicion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_SUSPICION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8953e11b-cc3a-4c8d-9d7e-04bf90c77027"),
    "Crown of Suspicion",
    crate::card::CardArt::new("8953e11b-cc3a-4c8d-9d7e-04bf90c77027", "Wayne England"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 135 — Cruel Revival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_REVIVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("245aba23-2abb-4084-b4cb-d06e46de2108"),
    "Cruel Revival",
    crate::card::CardArt::new("245aba23-2abb-4084-b4cb-d06e46de2108", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 136 — Death Match
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_MATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("143e9057-267a-4c78-b72a-4f8018b627a8"),
    "Death Match",
    crate::card::CardArt::new("143e9057-267a-4c78-b72a-4f8018b627a8", "rk post"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 137 — Death Pulse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_PULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("524fd470-e535-47ea-98a0-6187e429dfe1"),
    "Death Pulse",
    crate::card::CardArt::new("524fd470-e535-47ea-98a0-6187e429dfe1", "Tony Szczudlo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 138 — Dirge of Dread
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRGE_OF_DREAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8496e9c2-4c13-4307-bda7-b88512a21a6a"),
    "Dirge of Dread",
    crate::card::CardArt::new("8496e9c2-4c13-4307-bda7-b88512a21a6a", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 139 — Disciple of Malice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCIPLE_OF_MALICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74cc7ab0-a5db-4ae9-af9a-89fd5aaaab57"),
    "Disciple of Malice",
    crate::card::CardArt::new("74cc7ab0-a5db-4ae9-af9a-89fd5aaaab57", "Matt Cavotta"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 140 — Doomed Necromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOMED_NECROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ca3e348-47cc-41d6-999a-60d1206aaf06"),
    "Doomed Necromancer",
    crate::card::CardArt::new("3ca3e348-47cc-41d6-999a-60d1206aaf06", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 141 — Ebonblade Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EBONBLADE_REAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16ebef2c-8bb2-4816-a628-0062f95e512e"),
    "Ebonblade Reaper",
    crate::card::CardArt::new("16ebef2c-8bb2-4816-a628-0062f95e512e", "Wayne England"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 142 — Endemic Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDEMIC_PLAGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15326971-a53b-45f2-8f1d-1b82935286e1"),
    "Endemic Plague",
    crate::card::CardArt::new("15326971-a53b-45f2-8f1d-1b82935286e1", "Nelson DeCastro"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 143 — Entrails Feaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTRAILS_FEASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdddab92-3e1f-49dc-afd0-8c84d0d952c2"),
    "Entrails Feaster",
    crate::card::CardArt::new("cdddab92-3e1f-49dc-afd0-8c84d0d952c2", "John Matson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 144 — Fade from Memory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FADE_FROM_MEMORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56b34afa-0183-49aa-aa5f-03e070020136"),
    "Fade from Memory",
    crate::card::CardArt::new("56b34afa-0183-49aa-aa5f-03e070020136", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 145 — Fallen Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLEN_CLERIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7652dc61-9170-4895-a0bf-c32a1ee0350e"),
    "Fallen Cleric",
    crate::card::CardArt::new("7652dc61-9170-4895-a0bf-c32a1ee0350e", "Dave Dorman"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 146 — False Cure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_CURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef397db1-2d99-4cb0-a6e9-6f72d615ebad"),
    "False Cure",
    crate::card::CardArt::new("ef397db1-2d99-4cb0-a6e9-6f72d615ebad", "Bradley Williams"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 147 — Feeding Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEEDING_FRENZY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6d74c30-ebca-4684-ad84-3ca19193ad88"),
    "Feeding Frenzy",
    crate::card::CardArt::new("a6d74c30-ebca-4684-ad84-3ca19193ad88", "Nelson DeCastro"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 148 — Festering Goblin
pub(in crate::card::sets) static FESTERING_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7209cc8-b519-4f27-87d8-b12e239a121f"),
    "Festering Goblin",
    CardArt::new("e7209cc8-b519-4f27-87d8-b12e239a121f", "Thomas M. Baxa"),
    CardSet::Onslaught,
    // A one-drop whose death is the point: it blocks an X/1 and kills it
    // from the graveyard.
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie", "Goblin"], 1, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, target creature gets -1/-1 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 149 — Frightshroud Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRIGHTSHROUD_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a0fa75a-a82b-44cd-965f-07e0fe7a111a"),
    "Frightshroud Courier",
    crate::card::CardArt::new("4a0fa75a-a82b-44cd-965f-07e0fe7a111a", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 150 — Gangrenous Goliath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANGRENOUS_GOLIATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69b58b6b-24cd-4440-b99c-d88d44b3c41c"),
    "Gangrenous Goliath",
    crate::card::CardArt::new("69b58b6b-24cd-4440-b99c-d88d44b3c41c", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 151 — Gluttonous Zombie
pub(in crate::card::sets) static GLUTTONOUS_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db909e95-7979-41f0-b17a-874c4137fcc1"),
    "Gluttonous Zombie",
    CardArt::new("db909e95-7979-41f0-b17a-874c4137fcc1", "Thomas M. Baxa"),
    CardSet::Onslaught,
    // Five mana for a 3/3 nobody blocks, which is what an unevasive 3/3 at
    // that cost would never be worth.
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie"], 3, 3)
        .with_ability(abilities::fear()),
);

// ONS 152 — Gravespawn Sovereign
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVESPAWN_SOVEREIGN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e18dc249-a343-4198-bef9-e8092a2bac15"),
    "Gravespawn Sovereign",
    crate::card::CardArt::new("e18dc249-a343-4198-bef9-e8092a2bac15", "Adam Rex"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 153 — Grinning Demon
pub(in crate::card::sets) static GRINNING_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72de2f66-0b86-4c21-b4c8-c2d97e3fd095"),
    "Grinning Demon",
    CardArt::new("72de2f66-0b86-4c21-b4c8-c2d97e3fd095", "Mark Zug"),
    CardSet::Onslaught,
    // A 6/6 on turn four, with the two life a turn standing in for the
    // colour's usual price of a card.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Demon"], 6, 6)
        .with_morph(mana_cost!("{2}{B}{B}"))
        .with_ability(AbilityDef::triggered(
            "At the beginning of your upkeep, you lose 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        )),
);

// ONS 154 — Haunted Cadaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_CADAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a164420c-3619-4f5e-81cf-2aa5a4553bc3"),
    "Haunted Cadaver",
    crate::card::CardArt::new("a164420c-3619-4f5e-81cf-2aa5a4553bc3", "Randy Gallegos"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 155 — Head Games
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAD_GAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86ecc098-aa2b-4bae-80d5-4d02128ef837"),
    "Head Games",
    crate::card::CardArt::new("86ecc098-aa2b-4bae-80d5-4d02128ef837", "Terese Nielsen"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 156 — Headhunter
pub(in crate::card::sets) static HEADHUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cbd82d5-d64f-4833-b1a9-9652fcfa1578"),
    "Headhunter",
    CardArt::new("3cbd82d5-d64f-4833-b1a9-9652fcfa1578", "Matt Cavotta"),
    CardSet::Onslaught,
    // A body this small connecting is not the threat; the card it takes
    // every time it does is, and a deck with no blockers pays over and over.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Cleric"], 1, 1)
        .with_morph(mana_cost!("{B}"))
        .with_ability(AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a card.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )),
);

// ONS 157 — Infest
pub(in crate::card::sets) static INFEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7890ba2-aa42-4c8d-bbc1-94fb1d4150fc"),
    "Infest",
    CardArt::new("b7890ba2-aa42-4c8d-bbc1-94fb1d4150fc", "Ben Thompson"),
    CardSet::Onslaught,
    // Two points off everything, small enough to leave the opposing bombs
    // alive and large enough to clear the tokens under them.
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -2/-2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// ONS 158 — Misery Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISERY_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2be66eaf-222b-4c40-a9fa-aad56b9218e0"),
    "Misery Charm",
    crate::card::CardArt::new("2be66eaf-222b-4c40-a9fa-aad56b9218e0", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 159 — Nantuko Husk
pub(in crate::card::sets) static NANTUKO_HUSK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ff31ece-f132-4107-9415-fcf30e251167"),
    "Nantuko Husk",
    CardArt::new("1ff31ece-f132-4107-9415-fcf30e251167", "Carl Critchlow"),
    CardSet::Onslaught,
    // Phyrexian Ghoul again, and the reason both exist is that the outlet
    // costs nothing: the board empties at instant speed.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Insect"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: This creature gets +2/+2 until end of turn.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 160 — Oversold Cemetery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERSOLD_CEMETERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bbfd715-0772-4516-8cd8-89495dbccf4a"),
    "Oversold Cemetery",
    crate::card::CardArt::new("3bbfd715-0772-4516-8cd8-89495dbccf4a", "Thomas M. Baxa"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 161 — Patriarch's Bidding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRIARCH_S_BIDDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2deba175-8c02-492d-b404-5d842910c095"),
    "Patriarch's Bidding",
    crate::card::CardArt::new("2deba175-8c02-492d-b404-5d842910c095", "Ben Thompson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 162 — Profane Prayers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROFANE_PRAYERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc8320ef-af97-4cf6-9aaf-17818174d842"),
    "Profane Prayers",
    crate::card::CardArt::new("bc8320ef-af97-4cf6-9aaf-17818174d842", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 163 — Prowling Pangolin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROWLING_PANGOLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f037e99-75fb-4a2a-b4c6-448ef21b16a3"),
    "Prowling Pangolin",
    crate::card::CardArt::new("0f037e99-75fb-4a2a-b4c6-448ef21b16a3", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 164 — Rotlung Reanimator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTLUNG_REANIMATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87b29d1e-9c06-4ad1-8178-b3eaa212f6f1"),
    "Rotlung Reanimator",
    crate::card::CardArt::new("87b29d1e-9c06-4ad1-8178-b3eaa212f6f1", "Thomas M. Baxa"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 165 — Screeching Buzzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCREECHING_BUZZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d4b887a-d928-4f6c-aa37-a0b09e87b91e"),
    "Screeching Buzzard",
    crate::card::CardArt::new("1d4b887a-d928-4f6c-aa37-a0b09e87b91e", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 166 — Severed Legion
pub(in crate::card::sets) static SEVERED_LEGION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efe12afd-da41-436e-af84-fa3b36a58030"),
    "Severed Legion",
    CardArt::new("efe12afd-da41-436e-af84-fa3b36a58030", "Dany Orizio"),
    CardSet::Onslaught,
    // A 2/2 for three that attacks every turn regardless of the board,
    // which is the whole of mono-black's beatdown plan.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Zombie"], 2, 2)
        .with_ability(abilities::fear()),
);

// ONS 167 — Shade's Breath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADE_S_BREATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a37be9a8-ef69-4c62-8455-e129e62fe69a"),
    "Shade's Breath",
    crate::card::CardArt::new("a37be9a8-ef69-4c62-8455-e129e62fe69a", "Franz Vohwinkel"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 168 — Shepherd of Rot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHEPHERD_OF_ROT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("952c021f-74c9-455f-9cd9-f0d354e8bea8"),
    "Shepherd of Rot",
    crate::card::CardArt::new("952c021f-74c9-455f-9cd9-f0d354e8bea8", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 169 — Silent Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILENT_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28a3f78c-f7c1-4257-95cc-09d10022abba"),
    "Silent Specter",
    crate::card::CardArt::new("bfd891ba-cf6a-4b83-a421-3a7c346ada31", "Daren Bader"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 170 — Smother
pub(in crate::card::sets) static SMOTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a8321af-d667-44e7-8c03-3957286604b9"),
    "Smother",
    CardArt::new("9a8321af-d667-44e7-8c03-3957286604b9", "Carl Critchlow"),
    CardSet::Onslaught,
    // Instant-speed removal that answers everything the aggressive decks
    // are actually made of, and nothing the control decks win with.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with mana value 3 or less. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ManaValueAtMost(3),
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        },
    )),
);

// ONS 171 — Soulless One
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULLESS_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c826d786-0d96-4f77-94ae-6907fbce51e0"),
    "Soulless One",
    crate::card::CardArt::new("c826d786-0d96-4f77-94ae-6907fbce51e0", "Thomas M. Baxa"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 172 — Spined Basher
pub(in crate::card::sets) static SPINED_BASHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d0d666a-8e31-466c-937f-54df910f664e"),
    "Spined Basher",
    CardArt::new("4d0d666a-8e31-466c-937f-54df910f664e", "Thomas M. Baxa"),
    CardSet::Onslaught,
    // A 3/1 that turns up to win a block, which is the trick every morph
    // creature in the set is selling.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Beast"], 3, 1).with_ability(
        AbilityDef::alternative_cast(
            mana_cost!("{3}"),
            crate::card::face_down::morph_cast(),
            Some(
                "Morph {2}{B} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
            ),
            EffectDef::None,
        ),
    ),
);

// ONS 173 — Strongarm Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGARM_TACTICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57dcf434-5c67-440a-8b67-2df7307e92bd"),
    "Strongarm Tactics",
    crate::card::CardArt::new(
        "57dcf434-5c67-440a-8b67-2df7307e92bd",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 174 — Swat (reprint)

// ONS 175 — Syphon Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYPHON_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b0d8543-78c9-4d7f-b45e-44ecf023d276"),
    "Syphon Mind",
    crate::card::CardArt::new("0b0d8543-78c9-4d7f-b45e-44ecf023d276", "Jeff Easley"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 176 — Syphon Soul (reprint)

// ONS 177 — Thrashing Mudspawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASHING_MUDSPAWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da84de0e-a4cd-4dff-8ee3-87c9debf0969"),
    "Thrashing Mudspawn",
    crate::card::CardArt::new("da84de0e-a4cd-4dff-8ee3-87c9debf0969", "Thomas M. Baxa"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 178 — Undead Gladiator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDEAD_GLADIATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbc779d9-3200-4369-9289-1a8e90e243b9"),
    "Undead Gladiator",
    crate::card::CardArt::new("bbc779d9-3200-4369-9289-1a8e90e243b9", "Jeff Easley"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 179 — Visara the Dreadful
pub(in crate::card::sets) static VISARA_THE_DREADFUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce6adcfe-b0f7-4a96-bab2-f76c84ef5ca6"),
    "Visara the Dreadful",
    CardArt::new("ce6adcfe-b0f7-4a96-bab2-f76c84ef5ca6", "Kev Walker"),
    CardSet::Onslaught,
    // A 5/5 flier that kills something every turn, and the clause that says
    // it stays dead is what made it the format's best creature.
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Gorgon"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{T}: Destroy target creature. It can't be regenerated.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                },
            ),
        ]),
);

// ONS 180 — Walking Desecration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_DESECRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c39f3e91-571a-4990-b1e8-db2a5bac34af"),
    "Walking Desecration",
    crate::card::CardArt::new("c39f3e91-571a-4990-b1e8-db2a5bac34af", "Daren Bader"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 181 — Withering Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERING_HEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ce4be1e-97dd-45ec-89e5-2fb56145c098"),
    "Withering Hex",
    crate::card::CardArt::new(
        "9ce4be1e-97dd-45ec-89e5-2fb56145c098",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 182 — Words of Waste
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WASTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2dcb8ed-23e7-4cee-9f43-042232c6035a"),
    "Words of Waste",
    crate::card::CardArt::new("d2dcb8ed-23e7-4cee-9f43-042232c6035a", "Jerry Tiritilli"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 183 — Wretched Anurid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRETCHED_ANURID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aab525ad-1f62-4d9c-9b74-c7b0048da452"),
    "Wretched Anurid",
    crate::card::CardArt::new("aab525ad-1f62-4d9c-9b74-c7b0048da452", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 184 — Aether Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05df2792-4971-49e8-a8f2-17700e247500"),
    "Aether Charge",
    crate::card::CardArt::new("05df2792-4971-49e8-a8f2-17700e247500", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 185 — Aggravated Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRAVATED_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c99c5707-d5f2-4675-bfca-e801e6b0f627"),
    "Aggravated Assault",
    crate::card::CardArt::new("c99c5707-d5f2-4675-bfca-e801e6b0f627", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 186 — Airdrop Condor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIRDROP_CONDOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec9796ac-11e2-4295-bf00-f684d0111970"),
    "Airdrop Condor",
    crate::card::CardArt::new("ec9796ac-11e2-4295-bf00-f684d0111970", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 187 — Avarax
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVARAX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae76705f-ec95-48b0-9e26-84ce40c9514b"),
    "Avarax",
    crate::card::CardArt::new("ae76705f-ec95-48b0-9e26-84ce40c9514b", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 188 — Battering Craghorn
pub(in crate::card::sets) static BATTERING_CRAGHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ef71f42-87e5-4b1d-aac1-3752b81cee7c"),
    "Battering Craghorn",
    CardArt::new("9ef71f42-87e5-4b1d-aac1-3752b81cee7c", "Matt Cavotta"),
    CardSet::Onslaught,
    // The 3/1 first striker is a blowout when it is turned up blocking, and
    // a card nobody would cast for four otherwise.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goat", "Beast"], 3, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::alternative_cast(
            mana_cost!("{3}"),
            crate::card::face_down::morph_cast(),
            Some(
                "Morph {1}{R}{R} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
            ),
            EffectDef::None,
        ),
    ]),
);

// ONS 189 — Blistering Firecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLISTERING_FIRECAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0ddcf4a-1943-49dd-a02c-75804ce4bc3e"),
    "Blistering Firecat",
    crate::card::CardArt::new("e0ddcf4a-1943-49dd-a02c-75804ce4bc3e", "Arnie Swekel"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 190 — Break Open
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAK_OPEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5ae8050-b644-41db-b1e9-d9bad2173485"),
    "Break Open",
    crate::card::CardArt::new(
        "a5ae8050-b644-41db-b1e9-d9bad2173485",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 191 — Brightstone Ritual
pub(in crate::card::sets) static BRIGHTSTONE_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b08b0a6-c94e-4407-8a24-c8202497b5f2"),
    "Brightstone Ritual",
    CardArt::new("5b08b0a6-c94e-4407-8a24-c8202497b5f2", "Wayne England"),
    CardSet::Onslaught,
    // The same idea counting the board instead, which a Goblin deck has
    // filled by the time it matters.
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Add {R} for each Goblin on the battlefield.",
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_variable_amount(
            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype("Goblin"),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            )),
        )),
    )),
);

// ONS 192 — Butcher Orgg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUTCHER_ORGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f2a29cf-4b2e-44c0-af73-512d6fed0dae"),
    "Butcher Orgg",
    crate::card::CardArt::new("7f2a29cf-4b2e-44c0-af73-512d6fed0dae", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 193 — Chain of Plasma
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAIN_OF_PLASMA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f94aa774-9036-4016-8880-4bde2710cb90"),
    "Chain of Plasma",
    crate::card::CardArt::new("f94aa774-9036-4016-8880-4bde2710cb90", "Gary Ruddell"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 194 — Charging Slateback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_SLATEBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2cfff37-655f-4107-abf3-e6f63d0e4de2"),
    "Charging Slateback",
    crate::card::CardArt::new("d2cfff37-655f-4107-abf3-e6f63d0e4de2", "Mark Tedin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 195 — Commando Raid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDO_RAID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb237330-ac2e-411d-836c-6628f96f3262"),
    "Commando Raid",
    crate::card::CardArt::new("bb237330-ac2e-411d-836c-6628f96f3262", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 196 — Crown of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6caae974-f531-469d-8c6a-2077c4f3294a"),
    "Crown of Fury",
    crate::card::CardArt::new("6caae974-f531-469d-8c6a-2077c4f3294a", "Bradley Williams"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 197 — Custody Battle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUSTODY_BATTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b72257f5-0cf9-45ca-8dc7-a1a93bd7dd1e"),
    "Custody Battle",
    crate::card::CardArt::new(
        "b72257f5-0cf9-45ca-8dc7-a1a93bd7dd1e",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 198 — Dragon Roost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_ROOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95e4f28b-c7a7-4450-b477-73e4559f0276"),
    "Dragon Roost",
    crate::card::CardArt::new("95e4f28b-c7a7-4450-b477-73e4559f0276", "Luca Zontini"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 199 — Dwarven Blastminer
pub(in crate::card::sets) static DWARVEN_BLASTMINER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2970831a-738b-476f-9d46-39f10a1f91e7"),
    "Dwarven Blastminer",
    CardArt::new("2970831a-738b-476f-9d46-39f10a1f91e7", "Gary Ruddell"),
    CardSet::Onslaught,
    // The same land destruction on a body cheap enough to matter, which is
    // what made nonbasic mana bases a real risk.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf"], 1, 1)
        .with_morph(mana_cost!("{R}"))
        .with_ability(AbilityDef::activated_with_targets(
            "{2}{R}, {T}: Destroy target nonbasic land.",
            &[CostDef::Mana(mana_cost!("{2}{R}")), CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                )]
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        )),
);

// ONS 200 — Embermage Goblin (alternate printing)

// ONS 200★ — Embermage Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERMAGE_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ee5aa80-32cc-486e-bbb2-5386eadaf4ca"),
    "Embermage Goblin",
    crate::card::CardArt::new("0ee5aa80-32cc-486e-bbb2-5386eadaf4ca", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 201 — Erratic Explosion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRATIC_EXPLOSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f608a7e-5555-4554-a6e7-fe00e0bbe753"),
    "Erratic Explosion",
    crate::card::CardArt::new("9f608a7e-5555-4554-a6e7-fe00e0bbe753", "Gary Ruddell"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 202 — Fever Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEVER_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("830d1980-f460-4be2-9379-c3f74c8318f3"),
    "Fever Charm",
    crate::card::CardArt::new("830d1980-f460-4be2-9379-c3f74c8318f3", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 203 — Flamestick Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMESTICK_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e822161d-0434-4578-aecd-c9ef0b84bd4e"),
    "Flamestick Courier",
    crate::card::CardArt::new("e822161d-0434-4578-aecd-c9ef0b84bd4e", "Luca Zontini"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 204 — Goblin Machinist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MACHINIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5874e312-1010-43f2-b330-82bc9fcc9f53"),
    "Goblin Machinist",
    crate::card::CardArt::new("5874e312-1010-43f2-b330-82bc9fcc9f53", "Doug Chaffee"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 205 — Goblin Piledriver
pub(in crate::card::sets) static GOBLIN_PILEDRIVER: CardRecord = CardRecord::new_with_legacy_id(
    2019,
    "Goblin Piledriver",
    CardArt::new("f6c4df1f-f148-42ec-8e22-e7114216927d", "Matt Cavotta"),
    CardSet::Onslaught,
    // Protection from blue is half the card: it walks past the format's
    // blue blockers while the rest of the team makes it enormous.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Blue),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn for each other attacking Goblin.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&ScaledValueDef {
                        // "Each other attacking Goblin", so the Piledriver never counts itself and
                        // a lone one gets nothing.
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype("Goblin"),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        factor: 2,
                    }),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONS 206 — Goblin Pyromancer
pub(in crate::card::sets) static GOBLIN_PYROMANCER: CardRecord = CardRecord::new_with_legacy_id(
    307,
    "Goblin Pyromancer",
    CardArt::new(
        "bb4815b7-fc20-44a4-ad1c-66d92993557f",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Wizard"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, Goblin creatures get +3/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the end step, destroy all Goblins.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype("Goblin"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
        ),
    ]),
);

// ONS 207 — Goblin Sharpshooter
pub(in crate::card::sets) static GOBLIN_SHARPSHOOTER: CardRecord = CardRecord::new_with_legacy_id(
    292,
    "Goblin Sharpshooter",
    CardArt::new("7e689df7-b85d-4346-bee8-5e978b5cbbbc", "Greg Staples"),
    CardSet::Onslaught,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature dies, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ONS 208 — Goblin Sky Raider
pub(in crate::card::sets) static GOBLIN_SKY_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("738cbf9b-e3d3-4568-93ce-7915b248e5b3"),
    "Goblin Sky Raider",
    CardArt::new("738cbf9b-e3d3-4568-93ce-7915b248e5b3", "Daren Bader"),
    CardSet::Onslaught,
    // A red flier at common, which the colour only gets when the format
    // needs a way to block one.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 1, 2)
        .with_ability(abilities::flying()),
);

// ONS 209 — Goblin Sledder
pub(in crate::card::sets) static GOBLIN_SLEDDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a9a1ecf-29f6-474e-bbcf-3455d388aa94"),
    "Goblin Sledder",
    CardArt::new("3a9a1ecf-29f6-474e-bbcf-3455d388aa94", "Ron Spencer"),
    CardSet::Onslaught,
    // Mogg Raider again, for the block that made Goblins a deck rather
    // than a theme.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a Goblin: Target creature gets +1/+1 until end of turn.",
            // "A Goblin", so it can eat itself, which is what makes it a
            // free sacrifice outlet as well as a combat trick.
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 210 — Goblin Taskmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_TASKMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("feff65ca-aedf-4434-b701-590d600d1a0b"),
    "Goblin Taskmaster",
    crate::card::CardArt::new("feff65ca-aedf-4434-b701-590d600d1a0b", "Trevor Hairsine"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 211 — Grand Melee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_MELEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a0d3142-4224-4b51-885d-33c8938418c1"),
    "Grand Melee",
    crate::card::CardArt::new("9a0d3142-4224-4b51-885d-33c8938418c1", "Trevor Hairsine"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 212 — Gratuitous Violence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRATUITOUS_VIOLENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0c5d14-4fab-4034-a2d3-0d851ef67cbd"),
    "Gratuitous Violence",
    crate::card::CardArt::new(
        "4b0c5d14-4fab-4034-a2d3-0d851ef67cbd",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 213 — Insurrection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSURRECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("998bad32-1927-4e12-9527-efa55b86cae0"),
    "Insurrection",
    crate::card::CardArt::new("998bad32-1927-4e12-9527-efa55b86cae0", "Mark Zug"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 214 — Kaboom!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KABOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e81e5fc-0e18-4dd8-a505-aa7dba8521a8"),
    "Kaboom!",
    crate::card::CardArt::new("1e81e5fc-0e18-4dd8-a505-aa7dba8521a8", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 215 — Lavamancer's Skill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVAMANCER_S_SKILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d4dd156-a2c1-4fab-b9f4-3302a4e8835a"),
    "Lavamancer's Skill",
    crate::card::CardArt::new(
        "0d4dd156-a2c1-4fab-b9f4-3302a4e8835a",
        "Monte Michael Moore",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 216 — Lay Waste (reprint)

// ONS 217 — Lightning Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d775d729-0ad9-4b14-9d44-6282f6936e07"),
    "Lightning Rift",
    crate::card::CardArt::new("d775d729-0ad9-4b14-9d44-6282f6936e07", "Eric Peterson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 218 — Mana Echoes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_ECHOES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b15d04c-62cb-4704-8cc7-9842cef27a1b"),
    "Mana Echoes",
    crate::card::CardArt::new(
        "1b15d04c-62cb-4704-8cc7-9842cef27a1b",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 219 — Menacing Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENACING_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5360a871-6932-45b2-bc94-1bd414e38906"),
    "Menacing Ogre",
    crate::card::CardArt::new("5360a871-6932-45b2-bc94-1bd414e38906", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 220 — Nosy Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOSY_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70ea023e-e66d-4049-b7bc-5e660804f088"),
    "Nosy Goblin",
    crate::card::CardArt::new("70ea023e-e66d-4049-b7bc-5e660804f088", "Thomas M. Baxa"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 221 — Pinpoint Avalanche
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINPOINT_AVALANCHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5cf8876-4c7d-4779-9363-d0a58bb7d851"),
    "Pinpoint Avalanche",
    crate::card::CardArt::new("d5cf8876-4c7d-4779-9363-d0a58bb7d851", "Darrell Riche"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 222 — Reckless One
pub(in crate::card::sets) static RECKLESS_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37775f40-10de-4f5d-abb2-c49e682039de"),
    "Reckless One",
    CardArt::new("37775f40-10de-4f5d-abb2-c49e682039de", "Ron Spencer"),
    CardSet::Onslaught,
    // Haste on a body the size of the Goblin deck that cast it, which is
    // the whole tribal payoff in one card.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Avatar"], 0, 0)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Reckless One's power and toughness are each equal to the number of Goblins on the battlefield.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Goblin"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Goblin"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    ),
                },
            ),
        ]),
);

// ONS 223 — Risky Move
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISKY_MOVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b09315c-d6ff-4fdb-8774-c6402b45e959"),
    "Risky Move",
    crate::card::CardArt::new("0b09315c-d6ff-4fdb-8774-c6402b45e959", "Jerry Tiritilli"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 224 — Rorix Bladewing
pub(in crate::card::sets) static RORIX_BLADEWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f2caba5-9f30-4b5e-833e-68c85a47ef7c"),
    "Rorix Bladewing",
    CardArt::new("7f2caba5-9f30-4b5e-833e-68c85a47ef7c", "Darrell Riche"),
    CardSet::Onslaught,
    // Six mana for six damage the turn it lands, which is what a Dragon was
    // worth before they all started doing something else as well.
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Dragon"], 6, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// ONS 225 — Searing Flesh
pub(in crate::card::sets) static SEARING_FLESH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d83db110-42e7-4823-a686-b83205faf503"),
    "Searing Flesh",
    CardArt::new("d83db110-42e7-4823-a686-b83205faf503", "Pete Venters"),
    CardSet::Onslaught,
    // Seven mana for seven damage that cannot go to a creature, which is a
    // finisher and nothing else.
    CardRules::new_sorcery(mana_cost!("{6}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Searing Flesh deals 7 damage to target opponent or planeswalker.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// ONS 226 — Shaleskin Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHALESKIN_BRUISER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc2de8a4-0d84-4f7c-bbe4-3a31172186ab"),
    "Shaleskin Bruiser",
    crate::card::CardArt::new("fc2de8a4-0d84-4f7c-bbe4-3a31172186ab", "Mark Zug"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 227 — Shock (reprint)

// ONS 228 — Skirk Commando
pub(in crate::card::sets) static SKIRK_COMMANDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c870a66-4cd5-4a8d-9948-feffa7d4ff11"),
    "Skirk Commando",
    CardArt::new("8c870a66-4cd5-4a8d-9948-feffa7d4ff11", "Dave Dorman"),
    CardSet::Onslaught,
    // Unblocked once and the blocker that would have stopped it next turn
    // is gone, which is what the morph cost is really buying.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin"], 2, 1)
        .with_morph(mana_cost!("{2}{R}"))
        .with_ability(AbilityDef::triggered_with_targets(
            "Whenever this creature deals combat damage to a player, you may have it deal 2 damage to target creature that player controls.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            },
        )),
);

// ONS 229 — Skirk Fire Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_FIRE_MARSHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b71117d0-5cf7-4041-b568-00bd8a975dd8"),
    "Skirk Fire Marshal",
    crate::card::CardArt::new(
        "b71117d0-5cf7-4041-b568-00bd8a975dd8",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 230 — Skirk Prospector
pub(in crate::card::sets) static SKIRK_PROSPECTOR: CardRecord = CardRecord::new_with_legacy_id(
    2028,
    "Skirk Prospector",
    CardArt::new("eb545dcd-3a7a-46a7-9c35-d28faebc6d17", "Doug Chaffee"),
    CardSet::Onslaught,
    // A one-drop that turns the rest of the board into mana, including
    // itself: the sacrifice names any Goblin, and the Prospector is one.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "Sacrifice a Goblin: Add {R}.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// ONS 231 — Skittish Valesk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTISH_VALESK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cc8a6e6-ed62-4784-ba9a-b1f703fc6119"),
    "Skittish Valesk",
    crate::card::CardArt::new("4cc8a6e6-ed62-4784-ba9a-b1f703fc6119", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 232 — Slice and Dice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICE_AND_DICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59262684-86e3-4485-9e35-202771c3eaa6"),
    "Slice and Dice",
    crate::card::CardArt::new("59262684-86e3-4485-9e35-202771c3eaa6", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 233 — Snapping Thragg
pub(in crate::card::sets) static SNAPPING_THRAGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8a47d41-b893-46b9-90c9-ccd8f9f78855"),
    "Snapping Thragg",
    CardArt::new("c8a47d41-b893-46b9-90c9-ccd8f9f78855", "Iain McCaig"),
    CardSet::Onslaught,
    // The same deal as the Commando one size up, on a body that gets there
    // without help.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Beast"], 3, 3)
        .with_morph(mana_cost!("{4}{R}{R}"))
        .with_ability(AbilityDef::triggered_with_targets(
            "Whenever this creature deals combat damage to a player, you may have it deal 3 damage to target creature that player controls.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::Source,
                players: PlayerRelation::Opponent,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            },
        )),
);

// ONS 234 — Solar Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLAR_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b36fc40c-6a68-4192-91d9-2031c7d32e05"),
    "Solar Blast",
    crate::card::CardArt::new("b36fc40c-6a68-4192-91d9-2031c7d32e05", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 235 — Sparksmith
/// "The number of Goblins on the battlefield" counts both sides, which is
/// what makes this hurt more in the mirror than against anything else.
static GOBLINS_ON_THE_BATTLEFIELD: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Goblin"),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static SPARKSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15a4460d-3fe8-4b1f-9990-0a19c3345367"),
    "Sparksmith",
    CardArt::new("15a4460d-3fe8-4b1f-9990-0a19c3345367", "Jim Nelson"),
    CardSet::Onslaught,
    // Sparksmith is itself a Goblin, so it always deals at least one, and a
    // wide goblin board makes the recoil as large as the shot.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals X damage to target creature and X damage to you, where X \
             is the number of Goblins on the battlefield.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            // Both halves read the same count once, at resolution: nothing
            // between them can change how many Goblins there are.
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::CountMatchingObjects(&GOBLINS_ON_THE_BATTLEFIELD),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&GOBLINS_ON_THE_BATTLEFIELD),
                },
            ]),
        ),
    ),
);

// ONS 236 — Spitfire Handler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITFIRE_HANDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efe72820-952f-4c53-9ee7-ea7ea54fc848"),
    "Spitfire Handler",
    crate::card::CardArt::new("efe72820-952f-4c53-9ee7-ea7ea54fc848", "Jim Nelson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 237 — Spurred Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPURRED_WOLVERINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46d7aaea-226b-4820-8db2-89dcdcbcc557"),
    "Spurred Wolverine",
    crate::card::CardArt::new("46d7aaea-226b-4820-8db2-89dcdcbcc557", "Daren Bader"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 238 — Starstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARSTORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b54d72ba-05ce-4299-a7c3-a9e9f126fffb"),
    "Starstorm",
    crate::card::CardArt::new("b54d72ba-05ce-4299-a7c3-a9e9f126fffb", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 239 — Tephraderm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEPHRADERM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41b65eba-140b-4c1d-b796-8134b7c1ede8"),
    "Tephraderm",
    crate::card::CardArt::new("41b65eba-140b-4c1d-b796-8134b7c1ede8", "Paolo Parente"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 240 — Thoughtbound Primoc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTBOUND_PRIMOC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e89156b5-8bdb-41d1-a7aa-63f770a9b070"),
    "Thoughtbound Primoc",
    crate::card::CardArt::new("e89156b5-8bdb-41d1-a7aa-63f770a9b070", "Jeff Miracola"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 241 — Threaten
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREATEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de9676b6-6812-44e5-ad70-f498fbad0e18"),
    "Threaten",
    crate::card::CardArt::new("de9676b6-6812-44e5-ad70-f498fbad0e18", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 242 — Thunder of Hooves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_OF_HOOVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e4f796a-6831-4d83-824d-88fd2148b4c1"),
    "Thunder of Hooves",
    crate::card::CardArt::new("9e4f796a-6831-4d83-824d-88fd2148b4c1", "Jim Nelson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 243 — Wave of Indifference
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_INDIFFERENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c88b942-06d5-45d8-a4d8-6ca864f65516"),
    "Wave of Indifference",
    crate::card::CardArt::new(
        "2c88b942-06d5-45d8-a4d8-6ca864f65516",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 244 — Words of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2593a6a6-dc21-4742-acb8-f7092931b1ce"),
    "Words of War",
    crate::card::CardArt::new("2593a6a6-dc21-4742-acb8-f7092931b1ce", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 245 — Animal Magnetism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANIMAL_MAGNETISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c33db646-b30d-4a15-9f8a-63bda74e2d81"),
    "Animal Magnetism",
    crate::card::CardArt::new("c33db646-b30d-4a15-9f8a-63bda74e2d81", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 246 — Barkhide Mauler
pub(in crate::card::sets) static BARKHIDE_MAULER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9196ce7-3ff4-4dda-a628-559ada11c9ba"),
    "Barkhide Mauler",
    CardArt::new("b9196ce7-3ff4-4dda-a628-559ada11c9ba", "Iain McCaig"),
    CardSet::Onslaught,
    // A fat body that is also never a dead draw, which is what cycling
    // bought every common of the era.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 4, 4).with_ability(
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ),
);

// ONS 247 — Biorhythm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIORHYTHM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a02d6d5-27be-4301-a467-5b49491d0d4f"),
    "Biorhythm",
    crate::card::CardArt::new("2a02d6d5-27be-4301-a467-5b49491d0d4f", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 248 — Birchlore Rangers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIRCHLORE_RANGERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ce3a3a1-3569-4909-a604-f78d4888781e"),
    "Birchlore Rangers",
    crate::card::CardArt::new("8ce3a3a1-3569-4909-a604-f78d4888781e", "Dany Orizio"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 249 — Bloodline Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODLINE_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fdfc473-8477-4c04-a4e7-ecac1b0a5716"),
    "Bloodline Shaman",
    crate::card::CardArt::new("5fdfc473-8477-4c04-a4e7-ecac1b0a5716", "Rebecca Guay"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 250 — Broodhatch Nantuko
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROODHATCH_NANTUKO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38315ba3-57a0-4aa0-b1bc-4b1fcdd763d4"),
    "Broodhatch Nantuko",
    crate::card::CardArt::new("38315ba3-57a0-4aa0-b1bc-4b1fcdd763d4", "Keith Garletts"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 251 — Centaur Glade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_GLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c75f9c8-9640-4f64-b32a-916436e461fc"),
    "Centaur Glade",
    crate::card::CardArt::new(
        "1c75f9c8-9640-4f64-b32a-916436e461fc",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 252 — Chain of Acid
pub(in crate::card::sets) static CHAIN_OF_ACID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d47ddca-a363-4ab7-b7f2-d0e0043c9916"),
    "Chain of Acid",
    crate::card::CardArt::new("1d47ddca-a363-4ab7-b7f2-d0e0043c9916", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target noncreature permanent. Then that permanent's controller may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::May {
                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                    controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ]),
    )),
);

// ONS 253 — Crown of Vigor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_VIGOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7e320a6-88e2-4be1-97e2-30e0f3c2e450"),
    "Crown of Vigor",
    crate::card::CardArt::new("e7e320a6-88e2-4be1-97e2-30e0f3c2e450", "Matt Cavotta"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 254 — Elven Riders (reprint)

// ONS 255 — Elvish Guidance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_GUIDANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8698c46b-2628-4482-88f9-e37a01ade274"),
    "Elvish Guidance",
    crate::card::CardArt::new(
        "8698c46b-2628-4482-88f9-e37a01ade274",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 256 — Elvish Pathcutter
pub(in crate::card::sets) static ELVISH_PATHCUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7d810b8-1a15-46cc-9d9d-871ac43b7036"),
    "Elvish Pathcutter",
    CardArt::new("c7d810b8-1a15-46cc-9d9d-871ac43b7036", "Todd Lockwood"),
    CardSet::Onslaught,
    // Evasion for the tribe one member at a time, which is slow enough that
    // the deck had to be wide first.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Scout"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{G}: Target Elf creature gains forestwalk until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Elf"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::forestwalk() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 257 — Elvish Pioneer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_PIONEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e71fc2d-643b-4fad-89a8-624d330895d6"),
    "Elvish Pioneer",
    crate::card::CardArt::new("7e71fc2d-643b-4fad-89a8-624d330895d6", "Christopher Rush"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 258 — Elvish Scrapper
pub(in crate::card::sets) static ELVISH_SCRAPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae85fafb-114b-4fd8-ac4c-5ada57054705"),
    "Elvish Scrapper",
    CardArt::new(
        "ae85fafb-114b-4fd8-ac4c-5ada57054705",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Onslaught,
    // Artifact removal held in play until it is needed, which beats holding
    // it in hand against a deck that plays around one.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}, Sacrifice this creature: Destroy target artifact.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// ONS 259 — Elvish Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_VANGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("455c6923-8d0e-4a7f-a5c0-add8db519ee3"),
    "Elvish Vanguard",
    crate::card::CardArt::new("455c6923-8d0e-4a7f-a5c0-add8db519ee3", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 260 — Elvish Warrior
pub(in crate::card::sets) static ELVISH_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c6b767b-49e5-4845-9b3f-29540e5fa330"),
    "Elvish Warrior",
    crate::card::CardArt::new(
        "2c6b767b-49e5-4845-9b3f-29540e5fa330",
        "Christopher Moeller",
    ),
    CardSet::Onslaught,
    // A vanilla 2/3 for two, which is what green paid for a body that beats
    // every two-drop in a colour that was not trying to.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Elf", "Warrior"], 2, 3),
);

// ONS 261 — Enchantress's Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCHANTRESS_S_PRESENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75def198-99d6-4b0a-8878-5151f44bc0a4"),
    "Enchantress's Presence",
    crate::card::CardArt::new("75def198-99d6-4b0a-8878-5151f44bc0a4", "Rebecca Guay"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 262 — Everglove Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVERGLOVE_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13bf5786-e41a-4839-b8a0-5c7a413b23d0"),
    "Everglove Courier",
    crate::card::CardArt::new("13bf5786-e41a-4839-b8a0-5c7a413b23d0", "Darrell Riche"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 263 — Explosive Vegetation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_VEGETATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da6efd31-ab5e-46ff-80d2-9382438e302c"),
    "Explosive Vegetation",
    crate::card::CardArt::new("da6efd31-ab5e-46ff-80d2-9382438e302c", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 264 — Gigapede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIGAPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a96a608-9237-41c1-824c-89d5fad939ad"),
    "Gigapede",
    crate::card::CardArt::new("0a96a608-9237-41c1-824c-89d5fad939ad", "Glen Angus"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 265 — Heedless One
pub(in crate::card::sets) static HEEDLESS_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea338499-26a0-44e5-8999-f264644184d1"),
    "Heedless One",
    CardArt::new("ea338499-26a0-44e5-8999-f264644184d1", "Mark Zug"),
    CardSet::Onslaught,
    // The Elf version, and trample means the size is never wasted against
    // a chump block.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Avatar"], 0, 0)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::static_ability(
                "Heedless One's power and toughness are each equal to the number of Elves on the battlefield.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Elf"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Elf"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    ),
                },
            ),
        ]),
);

// ONS 266 — Hystrodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYSTRODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c964473-7c54-4c2d-a3eb-dba01c842103"),
    "Hystrodon",
    crate::card::CardArt::new("1c964473-7c54-4c2d-a3eb-dba01c842103", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 267 — Invigorating Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVIGORATING_BOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c46f324b-63c6-4fb5-a80a-e9da51c3eb77"),
    "Invigorating Boon",
    crate::card::CardArt::new(
        "c46f324b-63c6-4fb5-a80a-e9da51c3eb77",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 268 — Kamahl, Fist of Krosa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_FIST_OF_KROSA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("150d5229-b1a5-42cf-bf6a-04d246f1124f"),
    "Kamahl, Fist of Krosa",
    crate::card::CardArt::new("150d5229-b1a5-42cf-bf6a-04d246f1124f", "Matthew D. Wilson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 269 — Kamahl's Summons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_S_SUMMONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0edc37c6-b6a8-424f-95dd-928d03c28542"),
    "Kamahl's Summons",
    crate::card::CardArt::new("0edc37c6-b6a8-424f-95dd-928d03c28542", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 270 — Krosan Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_COLOSSUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a804f3c0-5ebf-43ca-b200-09f7c1bbe902"),
    "Krosan Colossus",
    crate::card::CardArt::new("a804f3c0-5ebf-43ca-b200-09f7c1bbe902", "Kev Walker"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 271 — Krosan Groundshaker
pub(in crate::card::sets) static KROSAN_GROUNDSHAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82105090-5f71-4690-9ade-187354311ae3"),
    "Krosan Groundshaker",
    CardArt::new("82105090-5f71-4690-9ade-187354311ae3", "Wayne England"),
    CardSet::Onslaught,
    // Trample for the tribe one member at a time, on a body big enough that
    // it usually only needs to do it once.
    CardRules::new_creature(mana_cost!("{4}{G}{G}{G}"), &["Beast"], 6, 6).with_ability(
        AbilityDef::activated_with_targets(
            "{G}: Target Beast creature gains trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Beast"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::trample() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ONS 272 — Krosan Tusker (reprint)

// ONS 273 — Leery Fogbeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEERY_FOGBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56125660-2307-4270-a947-f1f4ad63841c"),
    "Leery Fogbeast",
    crate::card::CardArt::new("56125660-2307-4270-a947-f1f4ad63841c", "Matt Cavotta"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 274 — Mythic Proportions
pub(in crate::card::sets) static MYTHIC_PROPORTIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("829069cf-7e63-4443-b679-65ad15d6ca5e"),
    "Mythic Proportions",
    CardArt::new("829069cf-7e63-4443-b679-65ad15d6ca5e", "Jim Nelson"),
    CardSet::Onslaught,
    // Seven mana on an Aura, which loses to any removal spell and wins
    // the game against none.
    CardRules::new_enchantment(mana_cost!("{4}{G}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +8/+8 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(8),
                            ValueDef::Constant(8),
                        ),
                        AppliedEffectDef::add_ability(&const { abilities::trample() }),
                    ]),
                },
            ),
        ]),
);

// ONS 275 — Naturalize
pub(in crate::card::sets) static NATURALIZE: CardRecord = CardRecord::new_with_legacy_id(
    270,
    "Naturalize",
    CardArt::new("c0acc41f-b55b-47cb-8803-d39d72788799", "Ron Spears"),
    CardSet::Onslaught,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact or enchantment.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ])),
    )),
);

// ONS 276 — Overwhelming Instinct
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERWHELMING_INSTINCT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d9e3793-7ddc-45c5-b25d-acd5cb96026f"),
    "Overwhelming Instinct",
    crate::card::CardArt::new("2d9e3793-7ddc-45c5-b25d-acd5cb96026f", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 277 — Primal Boost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_BOOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1b91a5a-9328-4fc6-a2f6-a7879281e145"),
    "Primal Boost",
    crate::card::CardArt::new("f1b91a5a-9328-4fc6-a2f6-a7879281e145", "Eric Peterson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 278 — Ravenous Baloth
pub(in crate::card::sets) static RAVENOUS_BALOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c98182d6-5b25-4493-9286-f29633e1bec4"),
    "Ravenous Baloth",
    CardArt::new("c98182d6-5b25-4493-9286-f29633e1bec4", "Arnie Swekel"),
    CardSet::Onslaught,
    // Four life at instant speed on a 4/4, which is why removal aimed at it
    // never quite worked.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 4, 4).with_ability(
        AbilityDef::activated(
            "Sacrifice a Beast: You gain 4 life.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Beast"),
                controller: PlayerRelation::You,
            }],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// ONS 279 — Run Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUN_WILD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("939a7354-162c-489d-955d-4df17b930e1c"),
    "Run Wild",
    crate::card::CardArt::new("939a7354-162c-489d-955d-4df17b930e1c", "Alan Pollack"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 280 — Serpentine Basilisk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENTINE_BASILISK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4052a5af-20b2-4817-8c94-78d488ee220f"),
    "Serpentine Basilisk",
    crate::card::CardArt::new("4052a5af-20b2-4817-8c94-78d488ee220f", "Franz Vohwinkel"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 281 — Silklash Spider (reprint)

// ONS 282 — Silvos, Rogue Elemental
pub(in crate::card::sets) static SILVOS_ROGUE_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e48715c-6ff7-4b0c-aa7e-a2c901215426"),
    "Silvos, Rogue Elemental",
    CardArt::new("3e48715c-6ff7-4b0c-aa7e-a2c901215426", "Carl Critchlow"),
    CardSet::Onslaught,
    // Eight power that regenerates for one, which is as close to
    // unanswerable as green gets without protection.
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Elemental"], 8, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            abilities::regenerate_self(
                "{G}: Regenerate Silvos.",
                &[CostDef::Mana(mana_cost!("{G}"))],
            ),
        ]),
);

// ONS 283 — Snarling Undorak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNARLING_UNDORAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05788d63-6210-44f2-9ae4-e55e9507a3a9"),
    "Snarling Undorak",
    crate::card::CardArt::new("05788d63-6210-44f2-9ae4-e55e9507a3a9", "Justin Sweet"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 284 — Spitting Gourna
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_GOURNA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("746b98bf-5398-4a00-b4fe-a990ea9cfd77"),
    "Spitting Gourna",
    crate::card::CardArt::new("746b98bf-5398-4a00-b4fe-a990ea9cfd77", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 285 — Stag Beetle
pub(in crate::card::sets) static STAG_BEETLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72cc64b9-f5b9-42d3-9921-564c4c9f2c77"),
    "Stag Beetle",
    CardArt::new("72cc64b9-f5b9-42d3-9921-564c4c9f2c77", "Anthony S. Waters"),
    CardSet::Onslaught,
    // It counts the opponent's board too, so the worse the game is going the
    // bigger it arrives -- which is the only five-drop that reads that way.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Insect"], 0, 0).with_ability(
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it, where X is the number of other creatures on the battlefield.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                },
            ),
        ),
    ),
);

// ONS 286 — Steely Resolve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEELY_RESOLVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b88c530a-abc3-4cc4-8a48-5b76e1504a3c"),
    "Steely Resolve",
    crate::card::CardArt::new("b88c530a-abc3-4cc4-8a48-5b76e1504a3c", "Greg Staples"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 287 — Symbiotic Beast
pub(in crate::card::sets) static SYMBIOTIC_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb61443d-e47a-4fe1-b777-67a3670a5a56"),
    "Symbiotic Beast",
    CardArt::new("bb61443d-e47a-4fe1-b777-67a3670a5a56", "Franz Vohwinkel"),
    CardSet::Onslaught,
    // Six mana for eight power spread across five bodies, which no single
    // removal spell touches.
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Insect", "Beast"], 4, 4).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create four 1/1 green Insect creature tokens.",
            EffectDef::create_token(tokens::creature(&["Insect"], &[ManaColor::Green], 1, 1))
                .with_amount(4),
        ),
    ),
);

// ONS 288 — Symbiotic Elf
pub(in crate::card::sets) static SYMBIOTIC_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33af35c6-7802-4366-ad20-1e330b4957ef"),
    "Symbiotic Elf",
    CardArt::new("33af35c6-7802-4366-ad20-1e330b4957ef", "Wayne England"),
    CardSet::Onslaught,
    // The body is worth the same either way, so answering it is never
    // profitable -- only necessary.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf"], 2, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create two 1/1 green Insect creature tokens.",
            EffectDef::create_token(tokens::creature(&["Insect"], &[ManaColor::Green], 1, 1))
                .with_amount(2),
        ),
    ),
);

// ONS 289 — Symbiotic Wurm
pub(in crate::card::sets) static SYMBIOTIC_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a60313ca-10cc-4c33-a557-1401c5721e3b"),
    "Symbiotic Wurm",
    CardArt::new("a60313ca-10cc-4c33-a557-1401c5721e3b", "Matt Cavotta"),
    CardSet::Onslaught,
    // Eight mana for a 7/7 that leaves seven creatures behind, which is
    // the top of a cycle that punishes removal at every rung.
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Wurm"], 7, 7).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create seven 1/1 green Insect creature tokens.",
            EffectDef::create_token(tokens::creature(&["Insect"], &[ManaColor::Green], 1, 1))
                .with_amount(7),
        ),
    ),
);

// ONS 290 — Taunting Elf (reprint)

// ONS 291 — Tempting Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPTING_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("857c2b6c-cfdf-4c88-a334-2937cb7db603"),
    "Tempting Wurm",
    crate::card::CardArt::new("857c2b6c-cfdf-4c88-a334-2937cb7db603", "Bob Petillo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 292 — Towering Baloth
pub(in crate::card::sets) static TOWERING_BALOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a8cc948-28ff-4bbe-b8c9-71de37478023"),
    "Towering Baloth",
    CardArt::new("2a8cc948-28ff-4bbe-b8c9-71de37478023", "Arnie Swekel"),
    CardSet::Onslaught,
    // Eight mana, or three now and seven later. Morph is what makes a card
    // this expensive castable at all.
    CardRules::new_creature(mana_cost!("{6}{G}{G}"), &["Beast"], 7, 6).with_ability(
        AbilityDef::alternative_cast(
            mana_cost!("{3}"),
            crate::card::face_down::morph_cast(),
            Some(
                "Morph {6}{G} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
            ),
            EffectDef::None,
        ),
    ),
);

// ONS 293 — Treespring Lorian
pub(in crate::card::sets) static TREESPRING_LORIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f525d7ce-37d3-4989-beb4-173447cb5294"),
    "Treespring Lorian",
    CardArt::new("f525d7ce-37d3-4989-beb4-173447cb5294", "Heather Hudson"),
    CardSet::Onslaught,
    // A 5/4 for six, or a 2/2 on turn three that becomes one later.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Beast"], 5, 4).with_ability(
        AbilityDef::alternative_cast(
            mana_cost!("{3}"),
            crate::card::face_down::morph_cast(),
            Some(
                "Morph {5}{G} (You may cast this card face down as a 2/2 creature for {3}. \
                 Turn it face up any time for its morph cost.)",
            ),
            EffectDef::None,
        ),
    ),
);

// ONS 294 — Tribal Unity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_UNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f7b5ddf-d5a6-42bf-a196-7e834dbdb3dc"),
    "Tribal Unity",
    crate::card::CardArt::new("6f7b5ddf-d5a6-42bf-a196-7e834dbdb3dc", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 295 — Venomspout Brackus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMSPOUT_BRACKUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0774771c-5373-4636-9174-d06e7d635183"),
    "Venomspout Brackus",
    crate::card::CardArt::new("0774771c-5373-4636-9174-d06e7d635183", "Ron Spencer"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 296 — Vitality Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITALITY_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1abae21-ed8f-4e21-b227-f721b840c11f"),
    "Vitality Charm",
    crate::card::CardArt::new("e1abae21-ed8f-4e21-b227-f721b840c11f", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 297 — Voice of the Woods
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_THE_WOODS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ebb4668-eebf-4b7e-ae29-75fff5963868"),
    "Voice of the Woods",
    crate::card::CardArt::new("1ebb4668-eebf-4b7e-ae29-75fff5963868", "Pete Venters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 298 — Wall of Mulch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_MULCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b3b4448-50f0-4996-94a1-db9ce356d925"),
    "Wall of Mulch",
    crate::card::CardArt::new("8b3b4448-50f0-4996-94a1-db9ce356d925", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 299 — Weird Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEIRD_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cdfa8b3-393b-4bb6-9265-faa4ab7126d2"),
    "Weird Harvest",
    crate::card::CardArt::new("3cdfa8b3-393b-4bb6-9265-faa4ab7126d2", "Bob Petillo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 300 — Wellwisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELLWISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be95ab7c-0e77-4293-aa48-ee54902a363f"),
    "Wellwisher",
    crate::card::CardArt::new("be95ab7c-0e77-4293-aa48-ee54902a363f", "Christopher Rush"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 301 — Wirewood Elf
pub(in crate::card::sets) static WIREWOOD_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10a34e31-97f1-40e8-9d91-a8139af7f096"),
    "Wirewood Elf",
    CardArt::new("10a34e31-97f1-40e8-9d91-a8139af7f096", "Jerry Tiritilli"),
    CardSet::Onslaught,
    // A two-mana Llanowar Elves, which is what the Elf deck played once it
    // had run out of the one-mana ones.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 2)
        .with_ability(abilities::tap_for(ManaColor::Green)),
);

// ONS 302 — Wirewood Herald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_HERALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35724e9f-efa6-47e7-ab4d-7defe38ba576"),
    "Wirewood Herald",
    crate::card::CardArt::new(
        "35724e9f-efa6-47e7-ab4d-7defe38ba576",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 303 — Wirewood Pride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_PRIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a559e844-06c9-4953-bc2c-a58e4170fe47"),
    "Wirewood Pride",
    crate::card::CardArt::new("a559e844-06c9-4953-bc2c-a58e4170fe47", "Dave Dorman"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 304 — Wirewood Savage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_SAVAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99982622-98bc-45ae-8642-41cd543f32a8"),
    "Wirewood Savage",
    crate::card::CardArt::new("99982622-98bc-45ae-8642-41cd543f32a8", "DiTerlizzi"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 305 — Words of Wilding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORDS_OF_WILDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdb9565f-5b09-4127-b169-3146079dab84"),
    "Words of Wilding",
    crate::card::CardArt::new("fdb9565f-5b09-4127-b169-3146079dab84", "Wayne England"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 306 — Cryptic Gateway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTIC_GATEWAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f379966-6a0a-434c-8682-1cf528a9a4a1"),
    "Cryptic Gateway",
    crate::card::CardArt::new("7f379966-6a0a-434c-8682-1cf528a9a4a1", "David Martin"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 307 — Doom Cannon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOM_CANNON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4abde0d7-266b-41bd-ade1-c4d93507eb16"),
    "Doom Cannon",
    crate::card::CardArt::new("4abde0d7-266b-41bd-ade1-c4d93507eb16", "Matthew Mitchell"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 308 — Dream Chisel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_CHISEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e89610e9-f1d3-4332-901a-2598bf01d61d"),
    "Dream Chisel",
    crate::card::CardArt::new("e89610e9-f1d3-4332-901a-2598bf01d61d", "Ron Spears"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 309 — Riptide Replicator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_REPLICATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41bb314f-237a-43fc-95c8-b26188dc4476"),
    "Riptide Replicator",
    crate::card::CardArt::new("41bb314f-237a-43fc-95c8-b26188dc4476", "Doug Chaffee"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 310 — Slate of Ancestry
pub(in crate::card::sets) static SLATE_OF_ANCESTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae596e8c-04f5-48b0-b5e2-683c74912e85"),
    "Slate of Ancestry",
    CardArt::new("ae596e8c-04f5-48b0-b5e2-683c74912e85", "Corey D. Macourek"),
    CardSet::Onslaught,
    // The hand is spent either way, so it is only a deal for the deck that
    // has already emptied it onto the board.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated(
        "{4}, {T}, Discard your hand: Draw a card for each creature you control.",
        &[
            CostDef::Mana(mana_cost!("{4}")),
            CostDef::TapSource,
            CostDef::DiscardHand,
        ],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// ONS 311 — Tribal Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e208be1-8b24-4048-90b2-6389f08043d1"),
    "Tribal Golem",
    crate::card::CardArt::new(
        "6e208be1-8b24-4048-90b2-6389f08043d1",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 312 — Barren Moor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARREN_MOOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45be3811-a223-4c45-9b24-0317f2d53c60"),
    "Barren Moor",
    crate::card::CardArt::new("45be3811-a223-4c45-9b24-0317f2d53c60", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 313 — Bloodstained Mire
pub(in crate::card::sets) static BLOODSTAINED_MIRE: CardRecord = CardRecord::new_with_legacy_id(
    1363,
    "Bloodstained Mire",
    CardArt::new("68c72226-6f52-4322-8b14-18737293dfa0", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Swamp, BasicLandType::Mountain],
    ),
);

// ONS 314 — Contested Cliffs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTESTED_CLIFFS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d6363ea-3814-4014-ad9e-1066c72d907c"),
    "Contested Cliffs",
    crate::card::CardArt::new("8d6363ea-3814-4014-ad9e-1066c72d907c", "Anthony S. Waters"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 315 — Daru Encampment
pub(in crate::card::sets) static DARU_ENCAMPMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5869f08-fac8-44b6-8142-7d7ecccab414"),
    "Daru Encampment",
    CardArt::new("c5869f08-fac8-44b6-8142-7d7ecccab414", "Tony Szczudlo"),
    CardSet::Onslaught,
    // A pump the opponent cannot answer and the deck pays nothing to run,
    // which is what a tribal land is for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{W}, {T}: Target Soldier creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Soldier"),
                    ]),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONS 316 — Flooded Strand
pub(in crate::card::sets) static FLOODED_STRAND: CardRecord = CardRecord::new_with_legacy_id(
    283,
    "Flooded Strand",
    CardArt::new("b4e3d844-d3b4-41d8-921d-c1cb3af343f8", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Plains or Island card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Plains, BasicLandType::Island],
    ),
);

// ONS 317 — Forgotten Cave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_CAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5202668-a32c-4473-b272-e86264992576"),
    "Forgotten Cave",
    crate::card::CardArt::new("c5202668-a32c-4473-b272-e86264992576", "Tony Szczudlo"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 318 — Goblin Burrows
pub(in crate::card::sets) static GOBLIN_BURROWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5064cd2-8762-4e08-8c3c-be6f31e9ab61"),
    "Goblin Burrows",
    CardArt::new("a5064cd2-8762-4e08-8c3c-be6f31e9ab61", "David Martin"),
    CardSet::Onslaught,
    // Two power for two mana out of a land, which turns a board of one-drops
    // into a real attack.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}{R}, {T}: Target Goblin creature gets +2/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONS 319 — Grand Coliseum
pub(in crate::card::sets) static GRAND_COLISEUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2dc8061-a855-4a81-9eb7-350b355a9b3f"),
    "Grand Coliseum",
    CardArt::new("c2dc8061-a855-4a81-9eb7-350b355a9b3f", "Carl Critchlow"),
    CardSet::Onslaught,
    // Every colour for a life apiece and a turn up front, which only a
    // five-colour deck can justify.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. This land deals 1 damage to you.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_damage_to_controller(1)),
        ),
    ]),
);

// ONS 320 — Lonely Sandbar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONELY_SANDBAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8ddab06-aff7-4c40-bcaa-10cbfe899dd9"),
    "Lonely Sandbar",
    crate::card::CardArt::new("d8ddab06-aff7-4c40-bcaa-10cbfe899dd9", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 321 — Polluted Delta
pub(in crate::card::sets) static POLLUTED_DELTA: CardRecord = CardRecord::new_with_legacy_id(
    1364,
    "Polluted Delta",
    CardArt::new("0f7585c8-9e21-4eef-afc1-2852de23db2f", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for an Island or Swamp card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Island, BasicLandType::Swamp],
    ),
);

// ONS 322 — Riptide Laboratory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_LABORATORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d993c973-2eb6-423c-8ee9-10749a751524"),
    "Riptide Laboratory",
    crate::card::CardArt::new("d993c973-2eb6-423c-8ee9-10749a751524", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 323 — Seaside Haven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASIDE_HAVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c940a6b-3c5e-4ce2-92b6-63e2cb575c15"),
    "Seaside Haven",
    crate::card::CardArt::new("9c940a6b-3c5e-4ce2-92b6-63e2cb575c15", "Mark Brill"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 324 — Secluded Steppe
pub(in crate::card::sets) static SECLUDED_STEPPE: CardRecord = CardRecord::new_with_legacy_id(
    2024,
    "Secluded Steppe",
    CardArt::new("ea454280-f7f4-4315-bb46-b56050c02c97", "Heather Hudson"),
    CardSet::Onslaught,
    // The tapped land you play on a turn you had nothing to do, or the card
    // you cycle away on a turn you did.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        abilities::cycling(
            "Cycling {W} ({W}, Discard this card: Draw a card.)",
            mana_cost!("{W}"),
        ),
    ]),
);

// ONS 325 — Starlit Sanctum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIT_SANCTUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ace5e601-2583-4d9c-8bdf-aa33666c717c"),
    "Starlit Sanctum",
    crate::card::CardArt::new("ace5e601-2583-4d9c-8bdf-aa33666c717c", "Ben Thompson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 326 — Tranquil Thicket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_THICKET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afcb7cef-8aeb-4c84-88e9-6df17768e292"),
    "Tranquil Thicket",
    crate::card::CardArt::new("afcb7cef-8aeb-4c84-88e9-6df17768e292", "Heather Hudson"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 327 — Unholy Grotto
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNHOLY_GROTTO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52f464a9-586c-4cf3-894b-b407c9f4dcb8"),
    "Unholy Grotto",
    crate::card::CardArt::new("52f464a9-586c-4cf3-894b-b407c9f4dcb8", "John Avon"),
    crate::card::CardSet::Onslaught,
    crate::card::CardRules::unsupported(),
);

// ONS 328 — Windswept Heath
pub(in crate::card::sets) static WINDSWEPT_HEATH: CardRecord = CardRecord::new_with_legacy_id(
    1365,
    "Windswept Heath",
    CardArt::new("7a7c5941-9c8a-4a40-9efb-a84f05c58e53", "Anthony S. Waters"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Forest or Plains card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Forest, BasicLandType::Plains],
    ),
);

// ONS 329 — Wirewood Lodge
pub(in crate::card::sets) static WIREWOOD_LODGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d251490-41bb-4ad3-bfd0-a5e66ee42598"),
    "Wirewood Lodge",
    CardArt::new("3d251490-41bb-4ad3-bfd0-a5e66ee42598", "Anthony S. Waters"),
    CardSet::Onslaught,
    // Untapping an Elf is untapping a mana source, which is why this land
    // reads as a combo piece rather than a tribal trick.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{G}, {T}: Untap target Elf.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype("Elf"),
                )]
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// ONS 330 — Wooded Foothills
pub(in crate::card::sets) static WOODED_FOOTHILLS: CardRecord = CardRecord::new_with_legacy_id(
    284,
    "Wooded Foothills",
    CardArt::new("cdad38f7-9dfa-4f1b-9fac-41ab2b253f53", "Rob Alexander"),
    CardSet::Onslaught,
    fetch_land(
        "{T}, Pay 1 life, Sacrifice this land: Search your library for a Mountain or Forest card, put it onto the battlefield, then shuffle.",
        &[BasicLandType::Mountain, BasicLandType::Forest],
    ),
);

// ONS 331 — Plains (reprint)

// ONS 332 — Plains (alternate printing)

// ONS 333 — Plains (alternate printing)

// ONS 334 — Plains (alternate printing)

// ONS 335 — Island (reprint)

// ONS 336 — Island (alternate printing)

// ONS 337 — Island (alternate printing)

// ONS 338 — Island (alternate printing)

// ONS 339 — Swamp (reprint)

// ONS 340 — Swamp (alternate printing)

// ONS 341 — Swamp (alternate printing)

// ONS 342 — Swamp (alternate printing)

// ONS 343 — Mountain (reprint)

// ONS 344 — Mountain (alternate printing)

// ONS 345 — Mountain (alternate printing)

// ONS 346 — Mountain (alternate printing)

// ONS 347 — Forest (reprint)

// ONS 348 — Forest (alternate printing)

// ONS 349 — Forest (alternate printing)

// ONS 350 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMA_S_BLESSING,
    &AKROMAS_VENGEANCE,
    &ANCESTOR_S_PROPHET,
    &ASTRAL_SLIDE,
    &AURA_EXTRACTION,
    &AURIFICATION,
    &AVEN_BRIGADIER,
    &AVEN_SOULGAZER,
    &BATTLEFIELD_MEDIC,
    &CATAPULT_MASTER,
    &CATAPULT_SQUAD,
    &CHAIN_OF_SILENCE,
    &CIRCLE_OF_SOLACE,
    &CONVALESCENT_CARE,
    &CROWD_FAVORITES,
    &CROWN_OF_AWE,
    &CRUDE_RAMPART,
    &DARU_CAVALIER,
    &DARU_HEALER,
    &DARU_LANCER,
    &DAUNTING_DEFENDER,
    &DAWNING_PURIST,
    &DEFENSIVE_MANEUVERS,
    &DIVE_BOMBER,
    &DOUBTLESS_ONE,
    &EXALTED_ANGEL,
    &FOOTHILL_GUIDE,
    &GLARECASTER,
    &GLORY_SEEKER,
    &GRASSLAND_CRUSADER,
    &GRAVEL_SLINGER,
    &GUSTCLOAK_HARRIER,
    &GUSTCLOAK_RUNNER,
    &GUSTCLOAK_SAVIOR,
    &GUSTCLOAK_SENTINEL,
    &GUSTCLOAK_SKIRMISHER,
    &HARSH_MERCY,
    &IMPROVISED_ARMOR,
    &INSPIRIT,
    &IRONFIST_CRUSHER,
    &JARETH_LEONINE_TITAN,
    &MOBILIZATION,
    &NOVA_CLERIC,
    &OBLATION,
    &PEARLSPEAR_COURIER,
    &PIETY_CHARM,
    &RENEWED_FAITH,
    &RIGHTEOUS_CAUSE,
    &SANDSKIN,
    &SHARED_TRIUMPH,
    &SHIELDMAGE_ELDER,
    &SIGIL_OF_THE_NEW_DAWN,
    &SUNFIRE_BALM,
    &TRUE_BELIEVER,
    &UNIFIED_STRIKE,
    &WEATHERED_WAYFARER,
    &WHIPCORDER,
    &WORDS_OF_WORSHIP,
    &AIRBORNE_AID,
    &ANNEX,
    &APHETTO_ALCHEMIST,
    &APHETTO_GRIFTER,
    &ARCANIS_THE_OMNIPOTENT,
    &ARTIFICIAL_EVOLUTION,
    &ASCENDING_AVEN,
    &AVEN_FATESHAPER,
    &BACKSLIDE,
    &BLATANT_THIEVERY,
    &CALLOUS_OPPRESSOR,
    &CHAIN_OF_VAPOR,
    &CHOKING_TETHERS,
    &COMPLICATE,
    &CRAFTY_PATHMAGE,
    &CROWN_OF_ASCENSION,
    &DISCOMBOBULATE,
    &DISPERSING_ORB,
    &DISRUPTIVE_PITMAGE,
    &ESSENCE_FRACTURE,
    &FLEETING_AVEN,
    &FUTURE_SIGHT,
    &GHOSTHELM_COURIER,
    &GRAXIPLON,
    &IMAGECRAFTER,
    &INFORMATION_DEALER,
    &IXIDOR_REALITY_SCULPTOR,
    &IXIDOR_S_WILL,
    &MAGE_S_GUILE,
    &MISTFORM_DREAMER,
    &MISTFORM_MASK,
    &MISTFORM_MUTANT,
    &MISTFORM_SHRIEKER,
    &MISTFORM_SKYREAVER,
    &MISTFORM_STALKER,
    &MISTFORM_WALL,
    &NAMELESS_ONE,
    &PEER_PRESSURE,
    &PSYCHIC_TRANCE,
    &QUICKSILVER_DRAGON,
    &READ_THE_RUNES,
    &REMINISCE,
    &RIPTIDE_BIOLOGIST,
    &RIPTIDE_CHRONOLOGIST,
    &RIPTIDE_ENTRANCER,
    &RIPTIDE_SHAPESHIFTER,
    &RUMMAGING_WIZARD,
    &SAGE_AVEN,
    &SCREAMING_SEAHAWK,
    &SEA_S_CLAIM,
    &SLIPSTREAM_EEL,
    &SPY_NETWORK,
    &STANDARDIZE,
    &SUPREME_INQUISITOR,
    &TRADE_SECRETS,
    &TRICKERY_CHARM,
    &VOIDMAGE_PRODIGY,
    &WHEEL_AND_DEAL,
    &WORDS_OF_WIND,
    &ACCURSED_CENTAUR,
    &ANURID_MURKDIVER,
    &APHETTO_DREDGING,
    &APHETTO_VULTURE,
    &BLACKMAIL,
    &BONEKNITTER,
    &CABAL_ARCHON,
    &CABAL_EXECUTIONER,
    &CABAL_SLAVER,
    &CHAIN_OF_SMOG,
    &COVER_OF_DARKNESS,
    &CROWN_OF_SUSPICION,
    &CRUEL_REVIVAL,
    &DEATH_MATCH,
    &DEATH_PULSE,
    &DIRGE_OF_DREAD,
    &DISCIPLE_OF_MALICE,
    &DOOMED_NECROMANCER,
    &EBONBLADE_REAPER,
    &ENDEMIC_PLAGUE,
    &ENTRAILS_FEASTER,
    &FADE_FROM_MEMORY,
    &FALLEN_CLERIC,
    &FALSE_CURE,
    &FEEDING_FRENZY,
    &FESTERING_GOBLIN,
    &FRIGHTSHROUD_COURIER,
    &GANGRENOUS_GOLIATH,
    &GLUTTONOUS_ZOMBIE,
    &GRAVESPAWN_SOVEREIGN,
    &GRINNING_DEMON,
    &HAUNTED_CADAVER,
    &HEAD_GAMES,
    &HEADHUNTER,
    &INFEST,
    &MISERY_CHARM,
    &NANTUKO_HUSK,
    &OVERSOLD_CEMETERY,
    &PATRIARCH_S_BIDDING,
    &PROFANE_PRAYERS,
    &PROWLING_PANGOLIN,
    &ROTLUNG_REANIMATOR,
    &SCREECHING_BUZZARD,
    &SEVERED_LEGION,
    &SHADE_S_BREATH,
    &SHEPHERD_OF_ROT,
    &SILENT_SPECTER,
    &SMOTHER,
    &SOULLESS_ONE,
    &SPINED_BASHER,
    &STRONGARM_TACTICS,
    &SYPHON_MIND,
    &THRASHING_MUDSPAWN,
    &UNDEAD_GLADIATOR,
    &VISARA_THE_DREADFUL,
    &WALKING_DESECRATION,
    &WITHERING_HEX,
    &WORDS_OF_WASTE,
    &WRETCHED_ANURID,
    &AETHER_CHARGE,
    &AGGRAVATED_ASSAULT,
    &AIRDROP_CONDOR,
    &AVARAX,
    &BATTERING_CRAGHORN,
    &BLISTERING_FIRECAT,
    &BREAK_OPEN,
    &BRIGHTSTONE_RITUAL,
    &BUTCHER_ORGG,
    &CHAIN_OF_PLASMA,
    &CHARGING_SLATEBACK,
    &COMMANDO_RAID,
    &CROWN_OF_FURY,
    &CUSTODY_BATTLE,
    &DRAGON_ROOST,
    &DWARVEN_BLASTMINER,
    &EMBERMAGE_GOBLIN,
    &ERRATIC_EXPLOSION,
    &FEVER_CHARM,
    &FLAMESTICK_COURIER,
    &GOBLIN_MACHINIST,
    &GOBLIN_PILEDRIVER,
    &GOBLIN_PYROMANCER,
    &GOBLIN_SHARPSHOOTER,
    &GOBLIN_SKY_RAIDER,
    &GOBLIN_SLEDDER,
    &GOBLIN_TASKMASTER,
    &GRAND_MELEE,
    &GRATUITOUS_VIOLENCE,
    &INSURRECTION,
    &KABOOM,
    &LAVAMANCER_S_SKILL,
    &LIGHTNING_RIFT,
    &MANA_ECHOES,
    &MENACING_OGRE,
    &NOSY_GOBLIN,
    &PINPOINT_AVALANCHE,
    &RECKLESS_ONE,
    &RISKY_MOVE,
    &RORIX_BLADEWING,
    &SEARING_FLESH,
    &SHALESKIN_BRUISER,
    &SKIRK_COMMANDO,
    &SKIRK_FIRE_MARSHAL,
    &SKIRK_PROSPECTOR,
    &SKITTISH_VALESK,
    &SLICE_AND_DICE,
    &SNAPPING_THRAGG,
    &SOLAR_BLAST,
    &SPARKSMITH,
    &SPITFIRE_HANDLER,
    &SPURRED_WOLVERINE,
    &STARSTORM,
    &TEPHRADERM,
    &THOUGHTBOUND_PRIMOC,
    &THREATEN,
    &THUNDER_OF_HOOVES,
    &WAVE_OF_INDIFFERENCE,
    &WORDS_OF_WAR,
    &ANIMAL_MAGNETISM,
    &BARKHIDE_MAULER,
    &BIORHYTHM,
    &BIRCHLORE_RANGERS,
    &BLOODLINE_SHAMAN,
    &BROODHATCH_NANTUKO,
    &CENTAUR_GLADE,
    &CHAIN_OF_ACID,
    &CROWN_OF_VIGOR,
    &ELVISH_GUIDANCE,
    &ELVISH_PATHCUTTER,
    &ELVISH_PIONEER,
    &ELVISH_SCRAPPER,
    &ELVISH_VANGUARD,
    &ELVISH_WARRIOR,
    &ENCHANTRESS_S_PRESENCE,
    &EVERGLOVE_COURIER,
    &EXPLOSIVE_VEGETATION,
    &GIGAPEDE,
    &HEEDLESS_ONE,
    &HYSTRODON,
    &INVIGORATING_BOON,
    &KAMAHL_FIST_OF_KROSA,
    &KAMAHL_S_SUMMONS,
    &KROSAN_COLOSSUS,
    &KROSAN_GROUNDSHAKER,
    &LEERY_FOGBEAST,
    &MYTHIC_PROPORTIONS,
    &NATURALIZE,
    &OVERWHELMING_INSTINCT,
    &PRIMAL_BOOST,
    &RAVENOUS_BALOTH,
    &RUN_WILD,
    &SERPENTINE_BASILISK,
    &SILVOS_ROGUE_ELEMENTAL,
    &SNARLING_UNDORAK,
    &SPITTING_GOURNA,
    &STAG_BEETLE,
    &STEELY_RESOLVE,
    &SYMBIOTIC_BEAST,
    &SYMBIOTIC_ELF,
    &SYMBIOTIC_WURM,
    &TEMPTING_WURM,
    &TOWERING_BALOTH,
    &TREESPRING_LORIAN,
    &TRIBAL_UNITY,
    &VENOMSPOUT_BRACKUS,
    &VITALITY_CHARM,
    &VOICE_OF_THE_WOODS,
    &WALL_OF_MULCH,
    &WEIRD_HARVEST,
    &WELLWISHER,
    &WIREWOOD_ELF,
    &WIREWOOD_HERALD,
    &WIREWOOD_PRIDE,
    &WIREWOOD_SAVAGE,
    &WORDS_OF_WILDING,
    &CRYPTIC_GATEWAY,
    &DOOM_CANNON,
    &DREAM_CHISEL,
    &RIPTIDE_REPLICATOR,
    &SLATE_OF_ANCESTRY,
    &TRIBAL_GOLEM,
    &BARREN_MOOR,
    &BLOODSTAINED_MIRE,
    &CONTESTED_CLIFFS,
    &DARU_ENCAMPMENT,
    &FLOODED_STRAND,
    &FORGOTTEN_CAVE,
    &GOBLIN_BURROWS,
    &GRAND_COLISEUM,
    &LONELY_SANDBAR,
    &POLLUTED_DELTA,
    &RIPTIDE_LABORATORY,
    &SEASIDE_HAVEN,
    &SECLUDED_STEPPE,
    &STARLIT_SANCTUM,
    &TRANQUIL_THICKET,
    &UNHOLY_GROTTO,
    &WINDSWEPT_HEATH,
    &WIREWOOD_LODGE,
    &WOODED_FOOTHILLS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_m12::DEMYSTIFY), // ONS 24
    PrintingRecord::reprint(&catalog_usg::DISCIPLE_OF_GRACE), // ONS 25
    PrintingRecord::reprint(&catalog_m13::PACIFISM),  // ONS 47
    PrintingRecord::reprint(&catalog_lea::CLONE),     // ONS 75
    PrintingRecord::reprint(&catalog_mir::MEDDLE),    // ONS 92
    PrintingRecord::reprint(&catalog_ulg::SWAT),      // ONS 174
    PrintingRecord::reprint(&catalog_leg::SYPHON_SOUL), // ONS 176
    PrintingRecord::alternate(&EMBERMAGE_GOBLIN, 1),  // ONS 200
    PrintingRecord::reprint(&catalog_usg::LAY_WASTE), // ONS 216
    PrintingRecord::reprint(&catalog_m14::SHOCK),     // ONS 227
    PrintingRecord::reprint(&catalog_leg::ELVEN_RIDERS), // ONS 254
    PrintingRecord::reprint(&catalog_mh1::KROSAN_TUSKER), // ONS 272
    PrintingRecord::reprint(&catalog_m13::SILKLASH_SPIDER), // ONS 281
    PrintingRecord::reprint(&catalog_uds::TAUNTING_ELF), // ONS 290
    PrintingRecord::reprint(&catalog_lea::PLAINS),    // ONS 331
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1), // ONS 332
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2), // ONS 333
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3), // ONS 334
    PrintingRecord::reprint(&catalog_lea::ISLAND),    // ONS 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1), // ONS 336
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2), // ONS 337
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3), // ONS 338
    PrintingRecord::reprint(&catalog_lea::SWAMP),     // ONS 339
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1), // ONS 340
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2), // ONS 341
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3), // ONS 342
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),  // ONS 343
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1), // ONS 344
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2), // ONS 345
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3), // ONS 346
    PrintingRecord::reprint(&catalog_lea::FOREST),    // ONS 347
    PrintingRecord::alternate(&catalog_lea::FOREST, 1), // ONS 348
    PrintingRecord::alternate(&catalog_lea::FOREST, 2), // ONS 349
    PrintingRecord::alternate(&catalog_lea::FOREST, 3), // ONS 350
];
