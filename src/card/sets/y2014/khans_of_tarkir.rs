//! Khans of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardSupertype, CardType, CounterKind, EffectDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, PlayerRelation, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, TriggerEventDef, TurnKindDef, ValueDef, ZoneKind, ZoneMoveCauseDef,
    abilities,
};
use crate::mana_cost;

// KTK 3 — Ainok Bond-Kin
pub(in crate::card::sets) static AINOK_BOND_KIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22d2a844-17fc-4628-9591-684555e98f7b"),
    "Ainok Bond-Kin",
    CardArt::new("22d2a844-17fc-4628-9591-684555e98f7b", "Jeff Simpson"),
    CardSet::KhansOfTarkir,
    // Outlast is slow enough that the anthem is the reason to play it: a
    // counters deck gets first strike on the whole board for free.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog", "Soldier"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "Outlast {1}{W} ({1}{W}, {T}: Put a +1/+1 counter on this creature. Outlast only as \
             a sorcery.)",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has first strike.",
            EffectDef::StaticApply {
                // Itself included once it has outlasted, which is what makes
                // the slow ability worth activating at all.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ]),
);

// KTK 22 — Seeker of the Way
pub(in crate::card::sets) static SEEKER_OF_THE_WAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c17e350-44f7-4413-ad24-7c5d6616effd"),
    "Seeker of the Way",
    CardArt::new("3c17e350-44f7-4413-ad24-7c5d6616effd", "Craig J Spearing"),
    CardSet::KhansOfTarkir,
    // Prowess and lifelink on the same trigger is what turns one cheap spell
    // into a four-point life swing, which is why this ends races.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        abilities::prowess(),
        // A second printed ability watching the same event, not a rider on
        // prowess: two spells in a turn grant lifelink twice, harmlessly,
        // and each grows the body separately.
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature gains lifelink until end of \
             turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// KTK 59 — Treasure Cruise
pub(in crate::card::sets) static TREASURE_CRUISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a59d4b1-6cf4-44ec-8a96-1bb7094fea21"),
    "Treasure Cruise",
    CardArt::new("7a59d4b1-6cf4-44ec-8a96-1bb7094fea21", "Cynthia Sheppard"),
    CardSet::KhansOfTarkir,
    CardRules::new_sorcery(mana_cost!("{7}{U}")).with_abilities(&[
        abilities::delve(),
        AbilityDef::spell(
            "Draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// KTK 78 — Mardu Skullhunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARDU_SKULLHUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd3ca5e7-96f3-4326-9315-34bb396a054c"),
    "Mardu Skullhunter",
    crate::card::CardArt::new("dd3ca5e7-96f3-4326-9315-34bb396a054c", "Jason Rainville"),
    crate::card::CardSet::KhansOfTarkir,
    crate::card::CardRules::unsupported(),
);

// KTK 111 — Hordeling Outburst
pub(in crate::card::sets) static HORDELING_OUTBURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5c1bf52-2737-423a-b340-07448afcaea6"),
    "Hordeling Outburst",
    CardArt::new("a5c1bf52-2737-423a-b340-07448afcaea6", "Zoltan Boros"),
    CardSet::KhansOfTarkir,
    // Three bodies from one card is what a go-wide deck is buying; the
    // sorcery speed is the price for not paying one mana each.
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(AbilityDef::spell(
        "Create three 1/1 red Goblin creature tokens.",
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_amount(3),
    )),
);

// KTK 118 — Monastery Swiftspear
pub(in crate::card::sets) static MONASTERY_SWIFTSPEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b81c6c8b-a9cf-4866-89ba-7f8ad077b836"),
    "Monastery Swiftspear",
    CardArt::new("b81c6c8b-a9cf-4866-89ba-7f8ad077b836", "Steve Argyle"),
    CardSet::KhansOfTarkir,
    // Haste is what makes prowess pay on the turn it lands rather than the
    // turn after, which is the whole card.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Monk"], 1, 2)
        .with_abilities(&[abilities::haste(), abilities::prowess()]),
);

// KTK 137 — Hooting Mandrills
pub(in crate::card::sets) static HOOTING_MANDRILLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("090d678c-f0e4-4757-8900-93dfe67aefe9"),
    "Hooting Mandrills",
    CardArt::new("090d678c-f0e4-4757-8900-93dfe67aefe9", "Mike Bierek"),
    CardSet::KhansOfTarkir,
    // Trample is what separates this from the other delve fatties: a
    // graveyard deck casts it early, when nothing on the far side blocks it
    // profitably anyway.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Ape"], 4, 4)
        .with_abilities(&[abilities::delve(), abilities::trample()]),
);

// KTK 227 — Ugin's Nexus
pub(in crate::card::sets) static UGINS_NEXUS: CardRecord = CardRecord::new_with_legacy_id(
    1368,
    "Ugin's Nexus",
    CardArt::new("94002868-a48a-4ea8-bfce-17257078f5db", "Sam Burley"),
    CardSet::KhansOfTarkir,
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::replacement_for(
                "If a player would begin an extra turn, that player skips that turn instead.",
                ReplacementEventDef::WouldBeginTurn {
                    player: PlayerRelation::Any,
                    kind: TurnKindDef::Extra,
                },
                ReplacementEffectDef::ReplaceEventWithNothing,
            ),
            AbilityDef::replacement_for(
                "If Ugin's Nexus would be put into a graveyard from the battlefield, instead exile it and take an extra turn after this one.",
                ReplacementEventDef::WouldMove {
                    from: Some(ZoneKind::Battlefield),
                    to: ZoneKind::Graveyard,
                    cause: ZoneMoveCauseDef::Any,
                },
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ReplacementEffectDef::Perform(&EffectDef::TakeExtraTurn {
                        player: EffectRecipientDef::Controller,
                    }),
                ]),
            ),
        ]),
);

// KTK 242 — Scoured Barrens
pub(in crate::card::sets) static SCOURED_BARRENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0824a960-dd89-45c5-90f0-3ec9eb47d9ce"),
    "Scoured Barrens",
    CardArt::new("0824a960-dd89-45c5-90f0-3ec9eb47d9ce", "Eytan Zana"),
    CardSet::KhansOfTarkir,
    // A tapped dual with a life attached: the life is what a limited deck
    // is paid for the turn it loses.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// KTK 246 — Tranquil Cove
pub(in crate::card::sets) static TRANQUIL_COVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f840bd2-c4f5-4ac4-918c-91b4feeb8783"),
    "Tranquil Cove",
    CardArt::new("0f840bd2-c4f5-4ac4-918c-91b4feeb8783", "John Avon"),
    CardSet::KhansOfTarkir,
    // A gain land: the tempo is the whole cost, and the life is what makes
    // the tapped land bearable in a slow deck.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AINOK_BOND_KIN,
    &SEEKER_OF_THE_WAY,
    &TREASURE_CRUISE,
    &MARDU_SKULLHUNTER,
    &HORDELING_OUTBURST,
    &MONASTERY_SWIFTSPEAR,
    &HOOTING_MANDRILLS,
    &UGINS_NEXUS,
    &SCOURED_BARRENS,
    &TRANQUIL_COVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
