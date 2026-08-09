//! Gatecrash card records used by the built-in ISD–RTR Standard decks.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CounterKind, EffectDef, EffectDurationDef, EffectRecipientDef, KeywordAbility, LandEntry,
    ManaColor, ObjectPredicateDef, PlayerRelation, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, abilities, cards,
};
use crate::ids::TargetSlotId;
use crate::mana_cost;

pub(in crate::card::sets) static ASSEMBLE_THE_LEGION: CardRecord = CardRecord::new(
    cards::ASSEMBLE_THE_LEGION,
    "Assemble the Legion",
    CardArt::new("43675ed7-ece1-4414-965e-9ebadcbf3dfb", "Eric Deschamps"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{3}{R}{W}")).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a muster counter on this enchantment. Then create a 1/1 red and white Soldier creature token with haste for each muster counter on this enchantment.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // The counter goes on first, so the very first upkeep already
            // musters one Soldier.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Muster,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken {
                    token: cards::SOLDIER_TOKEN_1_1_RED_WHITE,
                    count: ValueDef::CountersOnSource(CounterKind::Muster),
                },
            ]),
        ),
    ),
);

pub(in crate::card::sets) static AURELIAS_FURY: CardRecord = CardRecord::new(
    cards::AURELIAS_FURY,
    "Aurelia's Fury",
    CardArt::new("1a3465b6-ee7f-4553-bbf1-85fae9734b67", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{X}{R}{W}")).with_ability(
        AbilityDef::not_implemented(
            "Aurelia's Fury deals X damage divided as you choose among any number of targets. Tap each creature dealt damage this way. Players dealt damage this way can't cast noncreature spells this turn.",
            "Printed rules are cataloged but are not executed by the engine.",
        ),
    ),
);

pub(in crate::card::sets) static AURELIA_THE_WARLEADER: CardRecord = CardRecord::new(
    cards::AURELIA_THE_WARLEADER,
    "Aurelia, the Warleader",
    CardArt::new("4ec18e35-05e4-4bfc-b32b-c3e71c95a71d", "Slawomir Maniak"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}{W}{W}"),
        &["Angel"],
        3,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::haste(),
        AbilityDef::not_implemented(
            "Whenever Aurelia attacks for the first time each turn, untap all creatures you control. After this phase, there is an additional combat phase.",
            "The attack trigger and additional combat phase are not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static BLIND_OBEDIENCE: CardRecord = CardRecord::new(
    cards::BLIND_OBEDIENCE,
    "Blind Obedience",
    CardArt::new("07c3e78d-d917-4552-842f-feff99c059e0", "Seb McKinnon"),
    CardSet::Gatecrash,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::not_implemented(
            "Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each opponent loses 1 life and you gain that much life.)\nArtifacts and creatures your opponents control enter tapped.",
            "Printed rules are cataloged but are not executed by the engine.",
        ),
    ),
);

pub(in crate::card::sets) static BOROS_CHARM: CardRecord = CardRecord::new(
    cards::BOROS_CHARM,
    "Boros Charm",
    CardArt::new("d4ddf9cc-40a7-4b4f-bb51-b08171453c9a", "Zoltan Boros"),
    CardSet::Gatecrash,
    CardRules::new_instant(mana_cost!("{R}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Boros Charm deals 4 damage to target player or planeswalker.\n• Permanents you control gain indestructible until end of turn.\n• Target creature gains double strike until end of turn.",
        &[
            AbilityDef::spell(
                "Boros Charm deals 4 damage to target player or planeswalker",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::Constant(4),
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "player",
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )]),
            // Indestructible is not modeled.
            AbilityDef::unimplemented_spell(
                "Permanents you control gain indestructible until end of turn",
                "Printed mode is cataloged but is not executed by the engine.",
            ),
            AbilityDef::spell(
                "Target creature gains double strike until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetSlotId(1)),
                    effect: AppliedEffectDef::GrantAbility(&abilities::double_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one_permanent(
                TargetSlotId(1),
                "creature",
                ObjectPredicateDef::HasType(CardType::Creature),
            )]),
        ],
    )),
);

pub(in crate::card::sets) static BOROS_RECKONER: CardRecord = CardRecord::new(
    cards::BOROS_RECKONER,
    "Boros Reckoner",
    CardArt::new("82a18b07-38b8-4854-9735-3cfe83b11bf1", "Howard Lyon"),
    CardSet::Gatecrash,
    CardRules::new_creature(mana_cost!("{R/W}{R/W}{R/W}"), &["Minotaur", "Wizard"], 3, 3)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature is dealt damage, it deals that much damage to any target.",
                TriggerEventDef::DamageDealt {
                    source: ObjectPredicateDef::Any,
                    recipient: EffectRecipientDef::Source,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::TriggerEventAmount,
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "any target",
                AbilityTargetPredicate::AnyTarget,
            )]),
            AbilityDef::activated(
                "{R/W}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R/W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static DOMRI_RADE: CardRecord = CardRecord::new(
    cards::DOMRI_RADE,
    "Domri Rade",
    CardArt::new("21b48170-99dd-440f-9954-fc229d6094d3", "Tyler Jacobson"),
    CardSet::Gatecrash,
    CardRules::new_planeswalker(
        mana_cost!("{1}{R}{G}"),
        &["Domri"],
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_ability(AbilityDef::not_implemented(
        "+1: Look at the top card of your library. If it's a creature card, you may reveal it and put it into your hand.\n−2: Target creature you control fights another target creature.\n−7: You get an emblem with \"Creatures you control have double strike, trample, hexproof, and haste.\"",
        "Printed rules are cataloged but are not executed by the engine.",
    )),
);

pub(in crate::card::sets) static GHOR_CLAN_RAMPAGER: CardRecord = CardRecord::new(
    cards::GHOR_CLAN_RAMPAGER,
    "Ghor-Clan Rampager",
    CardArt::new("382048ec-0bf5-49a5-90d5-f80fbda08962", "Charles Urbach"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{2}{R}{G}"),
        &["Beast"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::trample(),
        AbilityDef::not_implemented(
            "Bloodrush — {R}{G}, Discard this card: Target attacking creature gets +4/+4 and gains trample until end of turn.",
            "The bloodrush activated ability is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new(
    cards::GODLESS_SHRINE,
    "Godless Shrine",
    CardArt::new("6fd672bb-18cf-44e3-8dda-5310b1e0fffe", "Cliff Childs"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Plains", "Swamp"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_ability(
        AbilityDef::replacement(
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The pay-life-or-tapped choice is implemented by the shared land-entry decision path.",
        }),
    ),
);

pub(in crate::card::sets) static OBZEDAT_GHOST_COUNCIL: CardRecord = CardRecord::new(
    cards::OBZEDAT_GHOST_COUNCIL,
    "Obzedat, Ghost Council",
    CardArt::new("4cc198d8-1f27-482d-8f5d-21e02c59797a", "Svetlin Velinov"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{1}{W}{W}{B}{B}"),
        &["Spirit", "Advisor"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::triggered(
            "When Obzedat enters, target opponent loses 2 life and you gain 2 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetSlotId(0)),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "opponent",
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )]),
        AbilityDef::triggered(
            "At the beginning of your end step, you may exile Obzedat. If you do, return it to the battlefield under its owner's control at the beginning of your next upkeep. It gains haste.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::May(&EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Source,
                },
                // Queued before the exile takes effect would be the same:
                // both read the source from the resolving ability, which the
                // exile does not disturb.
                EffectDef::AtNextStep {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                    effect: &EffectDef::ReturnLinkedExiles {
                        zone: ZoneKind::Battlefield,
                        grant: Some(KeywordAbility::Haste),
                    },
                },
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static SACRED_FOUNDRY: CardRecord = CardRecord::new(
    cards::SACRED_FOUNDRY,
    "Sacred Foundry",
    CardArt::new("0a26d900-c652-4f9c-8681-a35c5f8b1937", "Sam Burley"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Plains"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_ability(
        AbilityDef::replacement(
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The pay-life-or-tapped choice is implemented by the shared land-entry decision path.",
        }),
    ),
);

pub(in crate::card::sets) static SEPULCHRAL_PRIMORDIAL: CardRecord = CardRecord::new(
    cards::SEPULCHRAL_PRIMORDIAL,
    "Sepulchral Primordial",
    CardArt::new("eb0865cd-d9b4-43ea-87d2-ad5c65fc0459", "Stephan Martiniere"),
    CardSet::Gatecrash,
    CardRules::new_creature(
        mana_cost!("{5}{B}{B}"),
        &["Avatar"],
        5,
        4,
    )
    .with_abilities(&[
        abilities::intimidate().with_text(
            "Intimidate (This creature can't be blocked except by artifact creatures and/or creatures that share a color with it.)",
        ),
        AbilityDef::not_implemented(
            "When this creature enters, for each opponent, you may put up to one target creature card from that player's graveyard onto the battlefield under your control.",
            "The enters-the-battlefield reanimation trigger is not executed.",
        ),
    ]),
);

pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new(
    cards::STOMPING_GROUND,
    "Stomping Ground",
    CardArt::new("f29f3415-971c-4a5d-aae9-3893f4bdab1e", "David Palumbo"),
    CardSet::Gatecrash,
    CardRules::new_land(&["Mountain", "Forest"])
    .land_entry(LandEntry::PayLifeOrTapped(2))
    .with_ability(
        AbilityDef::replacement(
            "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
            EffectDef::Special("Choose whether to pay 2 life or have this land enter tapped"),
        )
        .with_implementation(AbilityImplementationDef::CustomFull {
            behavior: None,
            explanation: "The pay-life-or-tapped choice is implemented by the shared land-entry decision path.",
        }),
    ),
);

pub(in crate::card::sets) static THESPIANS_STAGE: CardRecord = CardRecord::new(
    cards::THESPIANS_STAGE,
    "Thespian's Stage",
    CardArt::new("b6f27909-e5cd-44c0-91c4-21624f692fd9", "John Avon"),
    CardSet::Gatecrash,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}: This land becomes a copy of target land, except it has this ability.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetSlotId(0)),
                retain_source_ability: true,
            },
        )
        .with_targets(&[AbilityTargetDef::exactly_one(
            TargetSlotId(0),
            "land",
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )]),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ASSEMBLE_THE_LEGION,
    &AURELIAS_FURY,
    &AURELIA_THE_WARLEADER,
    &BLIND_OBEDIENCE,
    &BOROS_CHARM,
    &BOROS_RECKONER,
    &DOMRI_RADE,
    &GHOR_CLAN_RAMPAGER,
    &GODLESS_SHRINE,
    &OBZEDAT_GHOST_COUNCIL,
    &SACRED_FOUNDRY,
    &SEPULCHRAL_PRIMORDIAL,
    &STOMPING_GROUND,
    &THESPIANS_STAGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
