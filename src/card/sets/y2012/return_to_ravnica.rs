//! Return to Ravnica card records used by the built-in ISD–DGM Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::{y1993::alpha, y1999::mercadian_masques as mmq, y2012::magic_2013};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardBehavior,
    CardRules, CardSet, CardSupertype, CardType, CardTypeSet, ColorSet, ComparisonDef,
    ControlDurationDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef,
    DamagePreventionDef, DiscardSelectionDef, EffectDef, EffectPaymentDef, EffectRecipientDef,
    InstalledTriggerDef, KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementChoiceDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneChangeEventMatcherDef,
    ZoneKind, ZoneMoveCauseDef, ZonePlacement, abilities, cards,
};
use crate::ids::{CardDefinitionId, TargetIndex};
use crate::mana_cost;

#[allow(clippy::too_many_arguments)]
const fn vanilla_creature(
    id: CardDefinitionId,
    name: &'static str,
    scryfall_id: &'static str,
    artist: &'static str,
    mana_cost: crate::card::ManaCost,
    subtypes: &'static [&'static str],
    power: i16,
    toughness: i16,
) -> CardRecord {
    CardRecord::new(
        id,
        name,
        CardArt::new(scryfall_id, artist),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost, subtypes, power, toughness),
    )
}

#[allow(clippy::large_types_passed_by_value, clippy::too_many_arguments)]
const fn keyword_creature(
    id: CardDefinitionId,
    name: &'static str,
    scryfall_id: &'static str,
    artist: &'static str,
    mana_cost: crate::card::ManaCost,
    subtypes: &'static [&'static str],
    power: i16,
    toughness: i16,
    ability: AbilityDef,
) -> CardRecord {
    CardRecord::new(
        id,
        name,
        CardArt::new(scryfall_id, artist),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost, subtypes, power, toughness).with_ability(ability),
    )
}

// RTR 1 — Angel of Serenity
pub(in crate::card::sets) static ANGEL_OF_SERENITY: CardRecord = CardRecord::new(
    cards::ANGEL_OF_SERENITY,
    "Angel of Serenity",
    CardArt::new("f10d82f7-7759-457e-a9bb-f9a5bd968f82", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{4}{W}{W}{W}"),
        &["Angel"],
        5,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets("When this creature enters, you may exile up to three other target creatures from the battlefield and/or creature cards from graveyards.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield, ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
            3,
        )], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            }),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, return the exiled cards to their owners' hands.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                arrival_effect: None,
                zone: ZoneKind::Hand,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// RTR 2 — Armory Guard
// Audit: blocked — Needs a continuous Gate-control condition that grants vigilance only while a Gate remains under your control.

static DAMAGE_DEALER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::DealtDamageThisTurn,
    ]),
)];

// RTR 4 — Avenging Arrow
pub(in crate::card::sets) static AVENGING_ARROW: CardRecord = CardRecord::new(
    cards::AVENGING_ARROW,
    "Avenging Arrow",
    CardArt::new("696678ff-44dc-4fe4-bf17-024e86cd0220", "James Ryman"),
    CardSet::ReturnToRavnica,
    // The revenge is for damage to anything, so a creature that traded in
    // combat is as legal a target as one that connected.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature that dealt damage this turn.",
        &DAMAGE_DEALER_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )),
);

// RTR 5 — Azorius Arrester
pub(in crate::card::sets) static AZORIUS_ARRESTER: CardRecord = CardRecord::new(
    cards::AZORIUS_ARRESTER,
    "Azorius Arrester",
    CardArt::new("199f7563-563e-483c-8317-5380a83db955", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 6 — Azorius Justiciar
pub(in crate::card::sets) static AZORIUS_JUSTICIAR: CardRecord = CardRecord::new(
    cards::AZORIUS_JUSTICIAR,
    "Azorius Justiciar",
    CardArt::new("9f56272e-c05e-446b-8871-e3783dd29a8b", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, detain up to two target creatures your opponents control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &UP_TO_TWO_OPPOSING_CREATURES,
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

static UP_TO_TWO_OPPOSING_CREATURES: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
    2,
)];

// RTR 7 — Bazaar Krovod
pub(in crate::card::sets) static BAZAAR_KROVOD: CardRecord = CardRecord::new(
    cards::BAZAAR_KROVOD,
    "Bazaar Krovod",
    CardArt::new("b07bb2fe-3a9b-47d0-864b-99a662d9544b", "Lars Grant-West"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Beast"], 2, 5).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target attacking creature gets +0/+2 until end of turn. Untap that creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ),
);

// RTR 8 — Concordia Pegasus
pub(in crate::card::sets) static CONCORDIA_PEGASUS: CardRecord = keyword_creature(
    cards::CONCORDIA_PEGASUS,
    "Concordia Pegasus",
    "f0333d0b-ae42-48aa-83d8-a4f2c7483a46",
    "Winona Nelson",
    mana_cost!("{1}{W}"),
    &["Pegasus"],
    1,
    3,
    abilities::flying(),
);

static ETHEREAL_ARMOR_FIRST_STRIKE: AbilityDef = abilities::first_strike();

/// The Armor is itself an enchantment you control, so it always counts at
/// least one -- and every other Aura and enchantment adds to it live.
static ETHEREAL_ARMOR_ENCHANTMENTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Enchantment),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static ETHEREAL_ARMOR_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(
        ValueDef::CountMatchingObjects(&ETHEREAL_ARMOR_ENCHANTMENTS),
        ValueDef::CountMatchingObjects(&ETHEREAL_ARMOR_ENCHANTMENTS),
    ),
    AppliedEffectDef::add_ability(&ETHEREAL_ARMOR_FIRST_STRIKE),
];

// RTR 9 — Ethereal Armor
pub(in crate::card::sets) static ETHEREAL_ARMOR: CardRecord = CardRecord::new(
    cards::ETHEREAL_ARMOR,
    "Ethereal Armor",
    CardArt::new("76960e65-e5c7-4414-b9a5-37d7b2ded4a0", "Daarken"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each enchantment you control and has first \
                 strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&ETHEREAL_ARMOR_BONUS),
                },
            ),
        ]),
);

static EYES_IN_THE_SKIES_EFFECTS: [EffectDef; 2] = [
    EffectDef::CreateToken {
        token: cards::BIRD_TOKEN_1_1_WHITE,
        controller: None,
        count: ValueDef::Constant(1),
        tapped: false,
        attacking: false,
        counters: None,
        created: None,
    },
    abilities::populate(),
];

// RTR 10 — Eyes in the Skies
pub(in crate::card::sets) static EYES_IN_THE_SKIES: CardRecord = CardRecord::new(
    cards::EYES_IN_THE_SKIES,
    "Eyes in the Skies",
    CardArt::new("befef095-3429-4dd7-aa01-2f7f619675d4", "James Ryman"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Create a 1/1 white Bird creature token with flying, then populate.",
        EffectDef::Sequence(&EYES_IN_THE_SKIES_EFFECTS),
    )),
);

// RTR 11 — Fencing Ace
pub(in crate::card::sets) static FENCING_ACE: CardRecord = keyword_creature(
    cards::FENCING_ACE,
    "Fencing Ace",
    "a42d3066-f4ec-4d28-83ab-e48141206c72",
    "David Rapoza",
    mana_cost!("{1}{W}"),
    &["Human", "Soldier"],
    1,
    1,
    abilities::double_strike(),
);

// RTR 12 — Keening Apparition
pub(in crate::card::sets) static KEENING_APPARITION: CardRecord = CardRecord::new(
    cards::KEENING_APPARITION,
    "Keening Apparition",
    CardArt::new("657b242c-46cb-44d1-86fd-fb2485144a5b", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Destroy target enchantment.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// RTR 13 — Knightly Valor
pub(in crate::card::sets) static KNIGHTLY_VALOR: CardRecord = CardRecord::new(
    cards::KNIGHTLY_VALOR,
    "Knightly Valor",
    CardArt::new("122d821f-c8dd-4a3c-a6d7-b42fe5491f02", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{4}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When this Aura enters, create a 2/2 white Knight creature token with vigilance.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::CreateToken {
                    token: cards::KNIGHT_TOKEN_2_2_WHITE,
                    controller: None,
                    count: ValueDef::Constant(1),
                    tapped: false,
                    attacking: false,
                    counters: None,
                    created: None,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
        ]),
);

// RTR 14 — Martial Law
pub(in crate::card::sets) static MARTIAL_LAW: CardRecord = CardRecord::new(
    cards::MARTIAL_LAW,
    "Martial Law",
    CardArt::new("21078b6f-a39d-4ec8-879e-ad10d97c3ff6", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, detain target creature an opponent controls.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// RTR 15 — Palisade Giant
// Audit: blocked — Needs a damage-redirection replacement covering you and every other permanent you control.

static PHANTOM_GENERAL_TOKENS: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Token,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// RTR 16 — Phantom General
pub(in crate::card::sets) static PHANTOM_GENERAL: CardRecord = CardRecord::new(
    cards::PHANTOM_GENERAL,
    "Phantom General",
    CardArt::new(
        "11f42791-070a-4e3a-91c8-b801980abb76",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    // The General is not itself a token, so its anthem never reaches it.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit", "Soldier"], 2, 3).with_ability(
        AbilityDef::static_ability(
            "Creature tokens you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: PHANTOM_GENERAL_TOKENS,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// RTR 17 — Precinct Captain
pub(in crate::card::sets) static PRECINCT_CAPTAIN: CardRecord = CardRecord::new(
    cards::PRECINCT_CAPTAIN,
    "Precinct Captain",
    CardArt::new("5f1f6178-4071-401f-bd0d-cac0c5967661", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, create a 1/1 white Soldier creature token.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::CreateToken {
                    token: cards::SOLDIER_TOKEN_1_1_WHITE,
                    controller: None,
                    count: ValueDef::Constant(1),
                    tapped: false,
                    attacking: false,
                counters: None,
                created: None,},
            ),
        ]),
);

// RTR 18 — Rest in Peace
pub(in crate::card::sets) static REST_IN_PEACE: CardRecord = CardRecord::new(
    cards::REST_IN_PEACE,
    "Rest in Peace",
    CardArt::new("37c2b1d1-faa0-40fd-82f4-216604ce7635", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "When this enchantment enters, exile all graveyards.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
        AbilityDef::replacement_for(
            "If a card or token would be put into a graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                to: ZoneKind::Graveyard,
                owner: PlayerRelation::Any,
                tokens: true,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

static ROOTBORN_DEFENSES_EFFECTS: [EffectDef; 2] = [
    abilities::populate(),
    EffectDef::Apply {
        recipient: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::HasType(CardType::Creature),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&ROOTBORN_INDESTRUCTIBLE),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static ROOTBORN_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

// RTR 19 — Rootborn Defenses
pub(in crate::card::sets) static ROOTBORN_DEFENSES: CardRecord = CardRecord::new(
    cards::ROOTBORN_DEFENSES,
    "Rootborn Defenses",
    CardArt::new("deccfa48-b8df-4dcc-ba1b-920f8352def7", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Populate. Creatures you control gain indestructible until end of turn.",
        EffectDef::Sequence(&ROOTBORN_DEFENSES_EFFECTS),
    )),
);

// RTR 20 — Security Blockade
// Audit: blocked — Needs a turn-long “prevent the next 1 damage” shield granted as a land activation.

// RTR 21 — Selesnya Sentry
pub(in crate::card::sets) static SELESNYA_SENTRY: CardRecord = CardRecord::new(
    cards::SELESNYA_SENTRY,
    "Selesnya Sentry",
    CardArt::new("9c34c1f5-d509-4c66-ba41-c7958ef5ee44", "Wesley Burt"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Elephant", "Soldier"], 3, 2).with_ability(
        abilities::regenerate_self(
            "{5}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{5}{G}"))],
        ),
    ),
);

// RTR 22 — Seller of Songbirds
pub(in crate::card::sets) static SELLER_OF_SONGBIRDS: CardRecord = CardRecord::new(
    cards::SELLER_OF_SONGBIRDS,
    "Seller of Songbirds",
    CardArt::new(
        "2a41edbe-4c5a-4535-a082-235dc3ffe60a",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human"], 1, 2).with_ability(
        AbilityDef::triggered(
            "When this creature enters, create a 1/1 white Bird creature token with flying.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken {
                token: cards::BIRD_TOKEN_1_1_WHITE,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
    ),
);

// RTR 23 — Soul Tithe
// Audit: blocked — Needs an upkeep payer derived from the enchanted permanent and an unless-payment amount equal to that permanent's mana value.

// RTR 24 — Sphere of Safety
// Audit: blocked — Needs a per-attacker combat tax whose amount dynamically counts enchantments you control.

// RTR 25 — Sunspire Griffin
pub(in crate::card::sets) static SUNSPIRE_GRIFFIN: CardRecord = keyword_creature(
    cards::SUNSPIRE_GRIFFIN,
    "Sunspire Griffin",
    "1388ce6e-8199-46c1-8ee3-71266b0929bf",
    "Johannes Voss",
    mana_cost!("{1}{W}{W}"),
    &["Griffin"],
    2,
    3,
    abilities::flying(),
);

// RTR 26 — Swift Justice
pub(in crate::card::sets) static SWIFT_JUSTICE: CardRecord = CardRecord::new(
    cards::SWIFT_JUSTICE,
    "Swift Justice",
    CardArt::new("a94801ba-0295-4611-abda-4c6508d69cc3", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target creature gets +1/+0 and gains first strike and lifelink.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
                AppliedEffectDef::add_ability(&abilities::lifelink()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// RTR 27 — Trained Caracal
pub(in crate::card::sets) static TRAINED_CARACAL: CardRecord = keyword_creature(
    cards::TRAINED_CARACAL,
    "Trained Caracal",
    "797e45d1-d17d-40c0-bfdf-ec533784e676",
    "James Ryman",
    mana_cost!("{W}"),
    &["Cat"],
    1,
    1,
    abilities::lifelink(),
);

static TROSTANIS_JUDGMENT_EFFECTS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    abilities::populate(),
];

// RTR 28 — Trostani's Judgment
pub(in crate::card::sets) static TROSTANIS_JUDGMENT: CardRecord = CardRecord::new(
    cards::TROSTANIS_JUDGMENT,
    "Trostani's Judgment",
    CardArt::new(
        "d707bdb1-1f8c-4cc0-be01-496f3f03b878",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{5}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature, then populate.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&TROSTANIS_JUDGMENT_EFFECTS),
    )),
);

// RTR 29 — Aquus Steed
pub(in crate::card::sets) static AQUUS_STEED: CardRecord = CardRecord::new(
    cards::AQUUS_STEED,
    "Aquus Steed",
    CardArt::new("af643949-7a9b-4195-8ab8-d43b1928b85a", "Warren Mahy"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Beast"], 1, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{U}, {T}: Target creature gets -2/-0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 30 — Blustersquall
pub(in crate::card::sets) static BLUSTERSQUALL: CardRecord = CardRecord::new(
    cards::BLUSTERSQUALL,
    "Blustersquall",
    CardArt::new("d998847b-323d-406a-b08e-0da66edcc7b3", "Willian Murai"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}"),
            "Tap each creature you don't control.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
            },
        ),
    ]),
);

// RTR 31 — Cancel
pub(in crate::card::sets) static CANCEL: CardRecord = CardRecord::new(
    cards::CANCEL,
    "Cancel",
    CardArt::new("fd994a26-65ff-43be-8d52-476e887d3ed2", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any),
    )),
);

// RTR 32 — Chronic Flooding
// Audit: blocked — Needs a trigger for the attached land becoming tapped and the attached land's controller as the mill recipient.

// RTR 33 — Conjured Currency
// Audit: blocked — Needs an exchange-of-control procedure involving the source and a targeted permanent you neither own nor control.

// RTR 34 — Crosstown Courier
pub(in crate::card::sets) static CROSSTOWN_COURIER: CardRecord = CardRecord::new(
    cards::CROSSTOWN_COURIER,
    "Crosstown Courier",
    CardArt::new("8c8875a3-9f56-4947-9655-aa5d95f06de0", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Vedalken"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player mills that many cards.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Mill {
                player: EffectRecipientDef::EventPlayer,
                amount: ValueDef::TriggerEventAmount,
                binding: None,
                then: None,
            },
        ),
    ),
);

// RTR 35 — Cyclonic Rift
pub(in crate::card::sets) static CYCLONIC_RIFT: CardRecord = CardRecord::new(
    cards::CYCLONIC_RIFT,
    "Cyclonic Rift",
    CardArt::new("205c4689-8b02-4d40-9274-3c1fcafa8b82", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target nonland permanent you don't control to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
        abilities::overload(
            mana_cost!("{6}{U}"),
            "Return each nonland permanent you don't control to its owner's hand.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
    ]),
);

// RTR 36 — Dispel
pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new(
    cards::DISPEL,
    "Dispel",
    CardArt::new("08d4a8d7-c136-472f-8146-a1100701ca4f", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Instant)),
    )),
);

static DEFENDERS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Defender),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// RTR 37 — Doorkeeper
pub(in crate::card::sets) static DOORKEEPER: CardRecord = CardRecord::new(
    cards::DOORKEEPER,
    "Doorkeeper",
    CardArt::new("5c31221f-3753-4d5c-905a-6b558ab648ae", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Homunculus"], 0, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated_with_targets(
            "{2}{U}, {T}: Target player mills X cards, where X is the number of creatures you control with defender.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&DEFENDERS_YOU_CONTROL),
                binding: None,
                then: None,
            },
        ),
    ]),
);

// RTR 38 — Downsize
pub(in crate::card::sets) static DOWNSIZE: CardRecord = CardRecord::new(
    cards::DOWNSIZE,
    "Downsize",
    CardArt::new("e8408a52-d34a-4e03-9312-700dec75d844", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you don't control gets -4/-0 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{2}{U}"),
            "Each creature you don't control gets -4/-0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 39 — Faerie Impostor
// Audit: blocked — Needs an ETB unless-payment whose cost returns another chosen creature you control to its owner's hand.

// RTR 40 — Hover Barrier
pub(in crate::card::sets) static HOVER_BARRIER: CardRecord = CardRecord::new(
    cards::HOVER_BARRIER,
    "Hover Barrier",
    CardArt::new("884afdb3-0d5f-45a1-b57e-6c3760aa0031", "Mathias Kollros"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Illusion", "Wall"], 0, 6)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// RTR 41 — Inaction Injunction
pub(in crate::card::sets) static INACTION_INJUNCTION: CardRecord = CardRecord::new(
    cards::INACTION_INJUNCTION,
    "Inaction Injunction",
    CardArt::new("5342ec3c-9d26-474a-9df5-c21ac90bb233", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Detain target creature an opponent controls. Draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Sequence(&INACTION_INJUNCTION_EFFECTS),
    )),
);

static INACTION_INJUNCTION_EFFECTS: [EffectDef; 2] = [
    EffectDef::Detain {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

// RTR 42 — Inspiration
pub(in crate::card::sets) static INSPIRATION: CardRecord = CardRecord::new(
    cards::INSPIRATION,
    "Inspiration",
    CardArt::new("e3cf9dc0-0a12-459c-88e2-97ed94653058", "Izzy"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// RTR 43 — Isperia's Skywatch
pub(in crate::card::sets) static ISPERIAS_SKYWATCH: CardRecord = CardRecord::new(
    cards::ISPERIAS_SKYWATCH,
    "Isperia's Skywatch",
    CardArt::new("019ba84d-d236-45c5-a8ad-75def7736d0c", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Vedalken", "Knight"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

/// The ability Jace's first one leaves behind. It belongs to no permanent,
/// so "an opponent" is read against the player who installed it.
static JACE_ATTACK_TAX: AbilityDef = AbilityDef::triggered(
    "Whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.",
    TriggerEventDef::attacks(ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
    ])),
    EffectDef::Apply {
        recipient: EffectRecipientDef::TriggeringObject,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(-1),
            ValueDef::Constant(0),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

static JACE_ARCHITECT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::InstallTrigger(InstalledTriggerDef::until_next_turn(
            &JACE_ATTACK_TAX,
            PlayerRefDef::EffectController,
        )),
    ),
    AbilityDef::activated(
        "−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.",
        &[AbilityCostDef::Loyalty(-2)],
        abilities::split_top_of_library_into_piles(
            ValueDef::Constant(3),
            &JACE_ARCHITECT_PILE_MOVES,
        ),
    ),
    AbilityDef::not_implemented(
        "−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.",
        "Casting an exiled card without paying its mana cost is not an available alternative cost.",
    ),
];

static JACE_ARCHITECT_PILE_MOVES: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        object: abilities::CHOSEN_PILE,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::MoveToZone {
        object: abilities::UNCHOSEN_PILE,
        zone: ZoneKind::Library,
        placement: ZonePlacement::Bottom,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
]);

// RTR 44 — Jace, Architect of Thought
// Audit: partial — The -8 cannot search every player's library and grant permission to cast the exiled cards without paying their mana costs.
pub(in crate::card::sets) static JACE_ARCHITECT_OF_THOUGHT: CardRecord = CardRecord::new(
    cards::JACE_ARCHITECT_OF_THOUGHT,
    "Jace, Architect of Thought",
    CardArt::new("d4df3a38-678e-42dc-a3fd-d1d399368f07", "Jaime Jones"),
    CardSet::ReturnToRavnica,
    CardRules::new_planeswalker(mana_cost!("{2}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&JACE_ARCHITECT_ABILITIES),
);

static MIZZIUM_SKIN_EFFECT: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
    AppliedEffectDef::add_ability(&abilities::hexproof()),
]);

// RTR 45 — Mizzium Skin
pub(in crate::card::sets) static MIZZIUM_SKIN: CardRecord = CardRecord::new(
    cards::MIZZIUM_SKIN,
    "Mizzium Skin",
    CardArt::new("d9859344-4efc-4b87-a3fb-147e496cee68", "Scott Chou"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +0/+1 and gains hexproof until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: MIZZIUM_SKIN_EFFECT,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{U}"),
            "Each creature you control gets +0/+1 and gains hexproof until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: MIZZIUM_SKIN_EFFECT,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 46 — Paralyzing Grasp
pub(in crate::card::sets) static PARALYZING_GRASP: CardRecord = CardRecord::new(
    cards::PARALYZING_GRASP,
    "Paralyzing Grasp",
    CardArt::new("3dfd97b3-d83e-406f-af45-40eec6347462", "Scott Chou"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// RTR 47 — Psychic Spiral
// Audit: blocked — Needs to preserve the number of graveyard cards shuffled into the library for the later mill amount.

// RTR 48 — Runewing
pub(in crate::card::sets) static RUNEWING: CardRecord = CardRecord::new(
    cards::RUNEWING,
    "Runewing",
    CardArt::new("749961e6-b135-4629-ae9d-124de0d70db9", "Martina Pilcerova"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature dies, draw a card.",
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
    ]),
);

// RTR 49 — Search the City
// Audit: blocked — Needs source-linked top-card exile, name matching against those cards, and the conditional extra-turn continuation.

// RTR 50 — Skyline Predator
pub(in crate::card::sets) static SKYLINE_PREDATOR: CardRecord = CardRecord::new(
    cards::SKYLINE_PREDATOR,
    "Skyline Predator",
    CardArt::new("5839556c-6635-44c4-96ed-666e4466b929", "Wesley Burt"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Drake"], 3, 4)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// RTR 51 — Soulsworn Spirit
pub(in crate::card::sets) static SOULSWORN_SPIRIT: CardRecord = CardRecord::new(
    cards::SOULSWORN_SPIRIT,
    "Soulsworn Spirit",
    CardArt::new("32602ed9-c0a7-4498-a333-235eaae628df", "James Ryman"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::cannot_be_blocked("This creature can't be blocked."),
        AbilityDef::triggered_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 52 — Sphinx of the Chimes
// Audit: blocked — Needs choosing two same-named nonland cards from hand as a single activation cost.

// RTR 53 — Stealer of Secrets
pub(in crate::card::sets) static STEALER_OF_SECRETS: CardRecord = CardRecord::new(
    cards::STEALER_OF_SECRETS,
    "Stealer of Secrets",
    CardArt::new("30ae7001-4d0f-4160-b41c-2fcb83fdb60b", "Michael C. Hayes"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// RTR 54 — Syncopate
pub(in crate::card::sets) static SYNCOPATE: CardRecord = CardRecord::new(
    cards::SYNCOPATE,
    "Syncopate",
    CardArt::new("ba6f218f-83b0-4b68-a00f-0327cd79f32a", "Clint Cearley"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(
        AbilityDef::spell_with_targets("Counter target spell unless its controller pays {X}. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )], abilities::counter_target_to_exile_unless_paid(ValueDef::ChosenX)),
    ),
);

// RTR 55 — Tower Drake
pub(in crate::card::sets) static TOWER_DRAKE: CardRecord = CardRecord::new(
    cards::TOWER_DRAKE,
    "Tower Drake",
    CardArt::new("5d759d6f-daf0-47f4-8a35-81c9d6437495", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static VOIDWIELDER_RETURN: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Hand,
    controller: None,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
};

// RTR 56 — Voidwielder
pub(in crate::card::sets) static VOIDWIELDER: CardRecord = CardRecord::new(
    cards::VOIDWIELDER,
    "Voidwielder",
    CardArt::new("23723bc7-a68e-4810-bc87-60df916cbb8a", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Wizard"], 1, 4).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may return target creature to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &VOIDWIELDER_RETURN,
            },
        ),
    ),
);

// RTR 57 — Assassin's Strike
pub(in crate::card::sets) static ASSASSINS_STRIKE: CardRecord = CardRecord::new(
    cards::ASSASSINS_STRIKE,
    "Assassin's Strike",
    CardArt::new("f796e320-9898-45d4-9d7a-6d35de53c9ab", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. Its controller discards a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// RTR 58 — Catacomb Slug
pub(in crate::card::sets) static CATACOMB_SLUG: CardRecord = vanilla_creature(
    cards::CATACOMB_SLUG,
    "Catacomb Slug",
    "53b36fba-6a0e-4f03-8bee-03919062537f",
    "Nils Hamm",
    mana_cost!("{4}{B}"),
    &["Slug"],
    2,
    6,
);

// RTR 59 — Cremate
pub(in crate::card::sets) static CREMATE: CardRecord = CardRecord::new(
    cards::CREMATE,
    "Cremate",
    CardArt::new("013d5260-f906-4f6a-97ed-725197743b60", "Cynthia Sheppard"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target card from a graveyard. Draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RTR 60 — Daggerdrome Imp
pub(in crate::card::sets) static DAGGERDROME_IMP: CardRecord = CardRecord::new(
    cards::DAGGERDROME_IMP,
    "Daggerdrome Imp",
    CardArt::new("70639887-bdba-4879-a3f8-c716f97fc325", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Imp"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// RTR 61 — Dark Revenant
// Audit: blocked — A dies trigger cannot address the card after it becomes a new graveyard object to move it onto its owner's library.

// RTR 62 — Dead Reveler
pub(in crate::card::sets) static DEAD_REVELER: CardRecord = CardRecord::new(
    cards::DEAD_REVELER,
    "Dead Reveler",
    CardArt::new("909a0a38-2c22-4b49-8938-1a8162c077e6", "David Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 3)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

/// What the Demon takes when an opponent feeds it: it stays home for the turn
/// and grows permanently.
static DESECRATION_DEMON_TRIBUTE: EffectDef = EffectDef::Sequence(&[
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
]);

// RTR 63 — Desecration Demon
pub(in crate::card::sets) static DESECRATION_DEMON: CardRecord = CardRecord::new(
    cards::DESECRATION_DEMON,
    "Desecration Demon",
    CardArt::new("8242fade-754c-4404-b3fb-f3cccf84b3b6", "Jason Chan"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Demon"],
        6,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of each combat, any opponent may sacrifice a creature of their choice. If a player does, tap this creature and put a +1/+1 counter on it.",
            // Each combat, so on either player's turn.
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: Some(&DESECRATION_DEMON_TRIBUTE),
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: true,
            },
        ),
    ]),
);

// RTR 64 — Destroy the Evidence
// Audit: blocked — Needs revealing and milling cards from a targeted land's controller until a land card is revealed.

static DEVIANT_GLEE_TRAMPLE: AbilityDef = AbilityDef::activated(
    "{R}: This creature gains trample until end of turn.",
    &[AbilityCostDef::Mana(mana_cost!("{R}"))],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&abilities::trample()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

// RTR 65 — Deviant Glee
pub(in crate::card::sets) static DEVIANT_GLEE: CardRecord = CardRecord::new(
    cards::DEVIANT_GLEE,
    "Deviant Glee",
    CardArt::new("e150896e-8745-42ac-894b-8f42a92bd7a7", "Michael C. Hayes"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1 and has \"{R}: This creature gains trample until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(1)),
                        AppliedEffectDef::add_ability(&DEVIANT_GLEE_TRAMPLE),
                    ]),
                },
            ),
        ]),
);

static DRAINPIPE_VERMIN_DISCARD: EffectDef = EffectDef::Discard {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    amount: ValueDef::Constant(1),
    selection: DiscardSelectionDef::RecipientChooses,
    then: None,
};

// RTR 66 — Drainpipe Vermin
pub(in crate::card::sets) static DRAINPIPE_VERMIN: CardRecord = CardRecord::new(
    cards::DRAINPIPE_VERMIN,
    "Drainpipe Vermin",
    CardArt::new("4d7251f3-df66-4611-a84c-1897f74431f7", "Trevor Claxton"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, you may pay {B}. If you do, target player discards a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{B}"),
                ),
                &DRAINPIPE_VERMIN_DISCARD,
            )),
        ),
    ),
);

// RTR 67 — Grave Betrayal
// Audit: blocked — Needs a delayed next-end-step return linked to each dead creature plus persistent color and Zombie type changes.

// RTR 68 — Grim Roustabout
pub(in crate::card::sets) static GRIM_ROUSTABOUT: CardRecord = CardRecord::new(
    cards::GRIM_ROUSTABOUT,
    "Grim Roustabout",
    CardArt::new("1a5ae3f5-5466-4058-a2cd-1a036cb38a8e", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Warrior"], 1, 1).with_abilities(
        &[
            abilities::unleash(),
            abilities::unleash_counter(),
            AbilityDef::activated(
                "{1}{B}: Regenerate this creature.",
                &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Source,
                },
            ),
        ],
    ),
);

// RTR 69 — Launch Party
// Audit: blocked — Needs choosing and sacrificing a creature as an additional spell-casting cost.

// RTR 71 — Necropolis Regent
pub(in crate::card::sets) static NECROPOLIS_REGENT: CardRecord = CardRecord::new(
    cards::NECROPOLIS_REGENT,
    "Necropolis Regent",
    CardArt::new("b421dcc9-0299-416d-86bc-c70ef49bcf98", "Winona Nelson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Vampire"], 6, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature you control deals combat damage to a player, put that many +1/+1 counters on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ]),
);

static OGRE_JAILBREAKER_HAS_A_GATE: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Gate"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

/// A permission rather than an ability removal: the Ogre keeps defender, so
/// anything reading "a creature with defender" still finds one.
static OGRE_JAILBREAKER_PERMISSION: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
};

// RTR 72 — Ogre Jailbreaker
pub(in crate::card::sets) static OGRE_JAILBREAKER: CardRecord = CardRecord::new(
    cards::OGRE_JAILBREAKER,
    "Ogre Jailbreaker",
    CardArt::new("9a96c83d-96d9-4f8f-8020-77990130ad81", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Ogre", "Rogue"], 4, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "This creature can attack as though it didn't have defender as long as you control a \
             Gate.",
            EffectDef::IfCondition {
                condition: &OGRE_JAILBREAKER_HAS_A_GATE,
                then: &OGRE_JAILBREAKER_PERMISSION,
            },
        ),
    ]),
);

// RTR 73 — Pack Rat
// Audit: blocked — Needs dynamic Rat-count power and toughness and creation of a copiable token copy of the source.

// RTR 74 — Perilous Shadow
pub(in crate::card::sets) static PERILOUS_SHADOW: CardRecord = CardRecord::new(
    cards::PERILOUS_SHADOW,
    "Perilous Shadow",
    CardArt::new("2c101171-a988-4c1d-9954-634e2f1c6f01", "Clint Cearley"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Insect", "Shade"], 0, 4).with_ability(
        AbilityDef::activated(
            "{1}{B}: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
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

// RTR 75 — Sewer Shambler
pub(in crate::card::sets) static SEWER_SHAMBLER: CardRecord = CardRecord::new(
    cards::SEWER_SHAMBLER,
    "Sewer Shambler",
    CardArt::new("0ae7ba14-c901-4266-ae31-812e001916d3", "Nils Hamm"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 1).with_abilities(&[
        abilities::landwalk(BasicLandType::Swamp),
        abilities::scavenge(
            mana_cost!("{2}{B}"),
            "Scavenge {2}{B} ({2}{B}, Exile this card from your graveyard: Put a number of \
             +1/+1 counters equal to this card's power on target creature. Scavenge only as a \
             sorcery.)",
        ),
    ]),
);

static SHRIEKING_AFFLICTION_HAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Any,
    &[ZoneKind::Hand],
    PlayerRelation::EventPlayer,
);
static SHRIEKING_AFFLICTION_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: SHRIEKING_AFFLICTION_HAND,
    comparison: ComparisonDef::LessOrEqual,
    amount: 1,
};

// RTR 76 — Shrieking Affliction
pub(in crate::card::sets) static SHRIEKING_AFFLICTION: CardRecord = CardRecord::new(
    cards::SHRIEKING_AFFLICTION,
    "Shrieking Affliction",
    CardArt::new("dfd08894-2534-4114-9365-40809ba95eb2", "Johann Bodin"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{B}")).with_ability(AbilityDef::triggered_if(
        "At the beginning of each opponent's upkeep, if that player has one or fewer cards in hand, they lose 3 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Opponent,
        },
        &SHRIEKING_AFFLICTION_CONDITION,
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(3),
        },
    )),
);

// RTR 77 — Slum Reaper
pub(in crate::card::sets) static SLUM_REAPER: CardRecord = CardRecord::new(
    cards::SLUM_REAPER,
    "Slum Reaper",
    CardArt::new("6f0fea13-63cf-4574-8752-3c357eee4524", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Horror"], 4, 2).with_ability(
        AbilityDef::triggered(
            "When this creature enters, each player sacrifices a creature of their choice.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// RTR 78 — Stab Wound
pub(in crate::card::sets) static STAB_WOUND: CardRecord = CardRecord::new(
    cards::STAB_WOUND,
    "Stab Wound",
    CardArt::new("7b562269-e6ec-4f8d-844e-26b272248d9d", "Scott Chou"),
    CardSet::ReturnToRavnica,
    // The drain follows the creature: gaining control of it means taking the
    // two a turn as well.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, that player \
                 loses 2 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// RTR 79 — Tavern Swindler
// Audit: blocked — Coin flips and their replay-visible random outcomes are unavailable.

// RTR 80 — Terrus Wurm
pub(in crate::card::sets) static TERRUS_WURM: CardRecord = CardRecord::new(
    cards::TERRUS_WURM,
    "Terrus Wurm",
    CardArt::new("1998135c-7b7f-402b-a8a5-4f4af131b1bc", "Cliff Childs"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{B}"), &["Zombie", "Wurm"], 5, 5).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{6}{B}"),
            "Scavenge {6}{B} ({6}{B}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 81 — Thrill-Kill Assassin
pub(in crate::card::sets) static THRILL_KILL_ASSASSIN: CardRecord = CardRecord::new(
    cards::THRILL_KILL_ASSASSIN,
    "Thrill-Kill Assassin",
    CardArt::new("a9f32204-eda7-4184-92e5-9da8b15b2359", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Assassin"], 1, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// RTR 82 — Ultimate Price
pub(in crate::card::sets) static ULTIMATE_PRICE: CardRecord = CardRecord::new(
    cards::ULTIMATE_PRICE,
    "Ultimate Price",
    CardArt::new("d2b4912a-83a2-4870-8fac-81fa79da2830", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target monocolored creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ColorCount(1),
        ])),
        true,
    )),
);

static UNDERWORLD_CONNECTIONS_DRAW: AbilityDef = AbilityDef::activated(
    "{T}, Pay 1 life: Draw a card.",
    &[AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)],
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

// RTR 83 — Underworld Connections
pub(in crate::card::sets) static UNDERWORLD_CONNECTIONS: CardRecord = CardRecord::new(
    cards::UNDERWORLD_CONNECTIONS,
    "Underworld Connections",
    CardArt::new("19c52e3b-b3b8-4243-96fe-fa4c8eea7c59", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}, Pay 1 life: Draw a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&UNDERWORLD_CONNECTIONS_DRAW),
                },
            ),
        ]),
);

// RTR 84 — Zanikev Locust
pub(in crate::card::sets) static ZANIKEV_LOCUST: CardRecord = CardRecord::new(
    cards::ZANIKEV_LOCUST,
    "Zanikev Locust",
    CardArt::new("adcbd7ee-8958-46fe-abb0-e899e7d2e654", "Cliff Childs"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Insect"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::scavenge(
            mana_cost!("{2}{B}{B}"),
            "Scavenge {2}{B}{B} ({2}{B}{B}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 85 — Annihilating Fire
// Audit: blocked — Needs a damage-linked, turn-long replacement that exiles a creature if it dies after being dealt this damage.

// RTR 86 — Ash Zealot
// Audit: blocked — Needs a spell-cast trigger predicate that identifies spells cast specifically from a graveyard.

static BATTERHORN_DESTROY: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
};

// RTR 87 — Batterhorn
pub(in crate::card::sets) static BATTERHORN: CardRecord = CardRecord::new(
    cards::BATTERHORN,
    "Batterhorn",
    CardArt::new("a7b40f74-893f-4bfc-87b2-7f8df4c912d8", "Dave Kendall"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Beast"], 4, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target artifact.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &BATTERHORN_DESTROY,
            },
        ),
    ),
);

// RTR 88 — Bellows Lizard
pub(in crate::card::sets) static BELLOWS_LIZARD: CardRecord = CardRecord::new(
    cards::BELLOWS_LIZARD,
    "Bellows Lizard",
    CardArt::new("5da4a644-9809-4591-9007-6b70b5f9d923", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{R}"), &["Lizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 89 — Bloodfray Giant
pub(in crate::card::sets) static BLOODFRAY_GIANT: CardRecord = CardRecord::new(
    cards::BLOODFRAY_GIANT,
    "Bloodfray Giant",
    CardArt::new("a8c468cd-1255-4257-9a69-c6b40a27c427", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 4, 3).with_abilities(&[
        abilities::trample(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

static CHAOS_IMPS_HAS_A_COUNTER: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::PlusOnePlusOne,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static CHAOS_IMPS_TRAMPLE_GRANT: AbilityDef = abilities::trample();

static CHAOS_IMPS_TRAMPLE: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&CHAOS_IMPS_TRAMPLE_GRANT),
};

// RTR 90 — Chaos Imps
pub(in crate::card::sets) static CHAOS_IMPS: CardRecord = CardRecord::new(
    cards::CHAOS_IMPS,
    "Chaos Imps",
    CardArt::new("c70a702a-c9be-4bee-9087-22b5905f783a", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Imp"], 6, 5).with_abilities(&[
        abilities::flying(),
        abilities::unleash(),
        abilities::unleash_counter(),
        AbilityDef::static_ability(
            "This creature has trample as long as it has a +1/+1 counter on it.",
            EffectDef::IfCondition {
                condition: &CHAOS_IMPS_HAS_A_COUNTER,
                then: &CHAOS_IMPS_TRAMPLE,
            },
        ),
    ]),
);

// RTR 91 — Cobblebrute
pub(in crate::card::sets) static COBBLEBRUTE: CardRecord = vanilla_creature(
    cards::COBBLEBRUTE,
    "Cobblebrute",
    "4e038376-801f-454e-a635-0e2d58ccbf7c",
    "Eytan Zana",
    mana_cost!("{3}{R}"),
    &["Elemental"],
    5,
    2,
);

// RTR 92 — Dynacharge
pub(in crate::card::sets) static DYNACHARGE: CardRecord = CardRecord::new(
    cards::DYNACHARGE,
    "Dynacharge",
    CardArt::new("e612a032-39be-44cd-a78c-29f89dc384b0", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +2/+0 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{2}{R}"),
            "Each creature you control gets +2/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 93 — Electrickery
pub(in crate::card::sets) static ELECTRICKERY: CardRecord = CardRecord::new(
    cards::ELECTRICKERY,
    "Electrickery",
    CardArt::new("5ed81ee8-d5e4-4127-876e-9bff81f9c726", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Electrickery deals 1 damage to target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::overload(
            mana_cost!("{1}{R}"),
            "Electrickery deals 1 damage to each creature you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// RTR 94 — Explosive Impact
pub(in crate::card::sets) static EXPLOSIVE_IMPACT: CardRecord = CardRecord::new(
    cards::EXPLOSIVE_IMPACT,
    "Explosive Impact",
    CardArt::new("3a3e2b45-b086-4ffd-aa1a-1d03046e0d61", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{5}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Explosive Impact deals 5 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// RTR 95 — Goblin Rally
pub(in crate::card::sets) static GOBLIN_RALLY: CardRecord = CardRecord::new(
    cards::GOBLIN_RALLY,
    "Goblin Rally",
    CardArt::new("e4ec8ada-09a6-449a-ac4a-7d3acbd08014", "Nic Klein"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_ability(AbilityDef::spell(
        "Create four 1/1 red Goblin creature tokens.",
        EffectDef::CreateToken {
            token: cards::GOBLIN_TOKEN_1_1_RED,
            controller: None,
            count: ValueDef::Constant(4),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    )),
);

// RTR 96 — Gore-House Chainwalker
pub(in crate::card::sets) static GORE_HOUSE_CHAINWALKER: CardRecord = CardRecord::new(
    cards::GORE_HOUSE_CHAINWALKER,
    "Gore-House Chainwalker",
    CardArt::new("56ba132f-95fc-4b99-a1dc-ebe6f622bb41", "Dan Murayama Scott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 2, 1)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 97 — Guild Feud
// Audit: blocked — Needs two linked top-three selections, optional creature entries, graveyard placement, and a conditional fight between the chosen creatures.

// RTR 98 — Guttersnipe
pub(in crate::card::sets) static GUTTERSNIPE: CardRecord = CardRecord::new(
    cards::GUTTERSNIPE,
    "Guttersnipe",
    CardArt::new("9d8590ea-512c-4e09-97cc-7f07d0706f2b", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature deals 2 damage to each opponent.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

static MULTICOLORED_SPELL: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::ColorCount(2),
    ObjectPredicateDef::ColorCount(3),
    ObjectPredicateDef::ColorCount(4),
    ObjectPredicateDef::ColorCount(5),
]);

// RTR 99 — Lobber Crew
pub(in crate::card::sets) static LOBBER_CREW: CardRecord = CardRecord::new(
    cards::LOBBER_CREW,
    "Lobber Crew",
    CardArt::new("b9d4aa15-a3c2-42a3-a87a-443e7dd20c04", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 0, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{T}: This creature deals 1 damage to each opponent.",
            &[AbilityCostDef::TapSource],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast a multicolored spell, untap this creature.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                MULTICOLORED_SPELL,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// RTR 100 — Minotaur Aggressor
pub(in crate::card::sets) static MINOTAUR_AGGRESSOR: CardRecord = CardRecord::new(
    cards::MINOTAUR_AGGRESSOR,
    "Minotaur Aggressor",
    CardArt::new("e22959dc-8759-454e-80b9-623a799af354", "Lucas Graciano"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Minotaur", "Berserker"], 6, 2)
        .with_abilities(&[abilities::first_strike(), abilities::haste()]),
);

// RTR 101 — Mizzium Mortars
pub(in crate::card::sets) static MIZZIUM_MORTARS: CardRecord = CardRecord::new(
    cards::MIZZIUM_MORTARS,
    "Mizzium Mortars",
    CardArt::new("d4ded88d-2688-4f5e-a8b2-16216cf9c792", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Mizzium Mortars deals 4 damage to target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        abilities::overload(
            mana_cost!("{3}{R}{R}{R}"),
            "Mizzium Mortars deals 4 damage to each creature you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

static PURSUIT_OF_FLIGHT_FLYING: AbilityDef = AbilityDef::activated(
    "{U}: This creature gains flying until end of turn.",
    &[AbilityCostDef::Mana(mana_cost!("{U}"))],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&abilities::flying()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

// RTR 102 — Pursuit of Flight
pub(in crate::card::sets) static PURSUIT_OF_FLIGHT: CardRecord = CardRecord::new(
    cards::PURSUIT_OF_FLIGHT,
    "Pursuit of Flight",
    CardArt::new("37a6290c-a0a8-4032-972b-84a7eef04dae", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has \"{U}: This creature gains flying until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                        AppliedEffectDef::add_ability(&PURSUIT_OF_FLIGHT_FLYING),
                    ]),
                },
            ),
        ]),
);

// RTR 103 — Pyroconvergence
pub(in crate::card::sets) static PYROCONVERGENCE: CardRecord = CardRecord::new(
    cards::PYROCONVERGENCE,
    "Pyroconvergence",
    CardArt::new("6cff95b7-79eb-4796-9a01-31ff355681ab", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{4}{R}"))
        .with_ability(AbilityDef::triggered_with_targets(
        "Whenever you cast a multicolored spell, this enchantment deals 2 damage to any target.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            MULTICOLORED_SPELL,
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

static RACECOURSE_FURY_HASTE: AbilityDef = AbilityDef::activated_with_targets(
    "{T}: Target creature gains haste until end of turn.",
    &[AbilityCostDef::TapSource],
    &[AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )],
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&abilities::haste()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
);

// RTR 104 — Racecourse Fury
pub(in crate::card::sets) static RACECOURSE_FURY: CardRecord = CardRecord::new(
    cards::RACECOURSE_FURY,
    "Racecourse Fury",
    CardArt::new("15d13d35-b5f7-4d3d-a1fa-84178b28acae", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Target creature gains haste until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&RACECOURSE_FURY_HASTE),
                },
            ),
        ]),
);

// RTR 105 — Splatter Thug
pub(in crate::card::sets) static SPLATTER_THUG: CardRecord = CardRecord::new(
    cards::SPLATTER_THUG,
    "Splatter Thug",
    CardArt::new("7c511805-3392-4033-8679-811711a0aaca", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

/// "Creature without flying you don't control", shared by both halves: the
/// overload changes only whether it is one of them or all of them.
static STREET_SPASM_GROUNDED: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(KeywordAbility::Flying)),
]);

// RTR 106 — Street Spasm
pub(in crate::card::sets) static STREET_SPASM: CardRecord = CardRecord::new(
    cards::STREET_SPASM,
    "Street Spasm",
    CardArt::new("9a19d3b8-80d0-480f-8900-47be527d0e53", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    // The overload cost doubles X, so the same X costs two more mana and
    // reaches every grounded creature instead of one.
    CardRules::new_instant(mana_cost!("{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Street Spasm deals X damage to target creature without flying you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: STREET_SPASM_GROUNDED,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
        abilities::overload(
            mana_cost!("{X}{X}{R}{R}"),
            "Street Spasm deals X damage to each creature without flying you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    STREET_SPASM_GROUNDED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// RTR 107 — Survey the Wreckage
pub(in crate::card::sets) static SURVEY_THE_WRECKAGE: CardRecord = CardRecord::new(
    cards::SURVEY_THE_WRECKAGE,
    "Survey the Wreckage",
    CardArt::new("a6e750f9-ad86-4d60-98a3-78d11cd52cd1", "Warren Mahy"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Create a 1/1 red Goblin creature token.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::CreateToken {
                token: cards::GOBLIN_TOKEN_1_1_RED,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ]),
    )),
);

// RTR 108 — Tenement Crasher
pub(in crate::card::sets) static TENEMENT_CRASHER: CardRecord = keyword_creature(
    cards::TENEMENT_CRASHER,
    "Tenement Crasher",
    "44af9170-bd99-4fde-b673-62d988312b2d",
    "Warren Mahy",
    mana_cost!("{5}{R}"),
    &["Beast"],
    5,
    4,
    abilities::haste(),
);

// RTR 109 — Traitorous Instinct
pub(in crate::card::sets) static TRAITOROUS_INSTINCT: CardRecord = CardRecord::new(
    cards::TRAITOROUS_INSTINCT,
    "Traitorous Instinct",
    CardArt::new("d4456951-844a-4847-b933-c32cfafbfef0", "Daarken"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap that creature. Until end of turn, it gets +2/+0 and gains haste.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                    controller: PlayerRefDef::EffectController,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// RTR 110 — Utvara Hellkite
pub(in crate::card::sets) static UTVARA_HELLKITE: CardRecord = CardRecord::new(
    cards::UTVARA_HELLKITE,
    "Utvara Hellkite",
    CardArt::new("f17c6478-dd80-4854-9560-bfc5ef597872", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{R}{R}"), &["Dragon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a Dragon you control attacks, create a 6/6 red Dragon creature token with flying.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Subtype("Dragon"),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken {
                token: cards::DRAGON_TOKEN_6_6_RED,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
            counters: None,
            created: None,},
        ),
    ]),
);

// RTR 111 — Vandalblast
pub(in crate::card::sets) static VANDALBLAST: CardRecord = CardRecord::new(
    cards::VANDALBLAST,
    "Vandalblast",
    CardArt::new("5925c559-3e3c-481b-ba95-20a405cbffce", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
        abilities::overload(
            mana_cost!("{4}{R}"),
            "Destroy each artifact you don't control.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                can_regenerate: true,
            },
        ),
    ]),
);

// RTR 112 — Viashino Racketeer
// Audit: partial — Discarding a card is not supported as an optional triggered-ability payment.
pub(in crate::card::sets) static VIASHINO_RACKETEER: CardRecord = CardRecord::new(
    cards::VIASHINO_RACKETEER,
    "Viashino Racketeer",
    CardArt::new("bf4c2d22-9c36-42cc-854d-f96410bb5cf1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard", "Rogue"], 2, 1).with_ability(
        AbilityDef::not_implemented(
            "When this creature enters, you may discard a card. If you do, draw a card.",
            "Discarding a card is not supported as an optional triggered-ability payment.",
        ),
    ),
);

// RTR 113 — Aerial Predation
pub(in crate::card::sets) static AERIAL_PREDATION: CardRecord = CardRecord::new(
    cards::AERIAL_PREDATION,
    "Aerial Predation",
    CardArt::new("ec3c023c-037e-495a-b7df-32be42a75f36", "BD"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with flying. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// RTR 114 — Archweaver
pub(in crate::card::sets) static ARCHWEAVER: CardRecord = CardRecord::new(
    cards::ARCHWEAVER,
    "Archweaver",
    CardArt::new("f99dc8ff-932c-4d56-9253-99ce9e145306", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Spider"], 5, 5)
        .with_abilities(&[abilities::reach(), abilities::trample()]),
);

// RTR 115 — Axebane Guardian
// Audit: blocked — Needs a dynamic amount of mana distributed in an arbitrary combination of colors.

// RTR 116 — Axebane Stag
pub(in crate::card::sets) static AXEBANE_STAG: CardRecord = vanilla_creature(
    cards::AXEBANE_STAG,
    "Axebane Stag",
    "bfce7c02-ccc3-44cd-8087-627eaa6a072e",
    "Martina Pilcerova",
    mana_cost!("{6}{G}"),
    &["Elk"],
    6,
    7,
);

// RTR 117 — Brushstrider
pub(in crate::card::sets) static BRUSHSTRIDER: CardRecord = keyword_creature(
    cards::BRUSHSTRIDER,
    "Brushstrider",
    "59bd1534-52d1-4946-b430-d26f039a9067",
    "Raoul Vitale",
    mana_cost!("{1}{G}"),
    &["Beast"],
    3,
    1,
    abilities::vigilance(),
);

// RTR 118 — Centaur's Herald
pub(in crate::card::sets) static CENTAURS_HERALD: CardRecord = CardRecord::new(
    cards::CENTAURS_HERALD,
    "Centaur's Herald",
    CardArt::new("08598b2b-6fd2-4a1d-8d74-7ca6d93ad382", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 0, 1).with_ability(
        AbilityDef::activated(
            "{2}{G}, Sacrifice this creature: Create a 3/3 green Centaur creature token.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::CreateToken {
                token: cards::CENTAUR_TOKEN_3_3_GREEN,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
    ),
);

static CHORUS_OF_MIGHT_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// RTR 119 — Chorus of Might
pub(in crate::card::sets) static CHORUS_OF_MIGHT: CardRecord = CardRecord::new(
    cards::CHORUS_OF_MIGHT,
    "Chorus of Might",
    CardArt::new("214dc9c1-154d-4c35-845d-dd2928f1e142", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Until end of turn, target creature gets +1/+1 for each creature you control and gains trample.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CHORUS_OF_MIGHT_CREATURES), ValueDef::CountMatchingObjects(&CHORUS_OF_MIGHT_CREATURES)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 120 — Deadbridge Goliath
pub(in crate::card::sets) static DEADBRIDGE_GOLIATH: CardRecord = CardRecord::new(
    cards::DEADBRIDGE_GOLIATH,
    "Deadbridge Goliath",
    CardArt::new("6ad03e99-25d3-4a09-819b-9192dfd8c9d2", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect"], 5, 5).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{4}{G}{G}"),
            "Scavenge {4}{G}{G} ({4}{G}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 121 — Death's Presence
pub(in crate::card::sets) static DEATHS_PRESENCE: CardRecord = CardRecord::new(
    cards::DEATHS_PRESENCE,
    "Death's Presence",
    CardArt::new("fa82c57d-4bb9-407c-b973-7abc793b6f47", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    // Nothing you lose is wasted: the body moves its power onto whatever is
    // left standing.
    CardRules::new_enchantment(mana_cost!("{5}{G}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a creature you control dies, put X +1/+1 counters on target creature you control, where X is the power of the creature that died.",
            TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            )),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                // Last known: the creature is in a graveyard by now, which is
                // the only time the reading is interesting.
                amount: ValueDef::TriggeringObjectPower,
            },
        ),
    ),
);

// RTR 122 — Drudge Beetle
pub(in crate::card::sets) static DRUDGE_BEETLE: CardRecord = CardRecord::new(
    cards::DRUDGE_BEETLE,
    "Drudge Beetle",
    CardArt::new("e4812e81-beca-4afc-b2f2-24d5ab27abff", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{5}{G}"),
            "Scavenge {5}{G} ({5}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

/// "Dealt to you", so the shield is scoped to its controller rather than
/// covering the whole combat the way a Fog does.
static DRUIDS_DELIVERANCE_EFFECTS: [EffectDef; 2] = [
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_to(
            EffectRecipientDef::Controller,
        )),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    abilities::populate(),
];

// RTR 123 — Druid's Deliverance
pub(in crate::card::sets) static DRUIDS_DELIVERANCE: CardRecord = CardRecord::new(
    cards::DRUIDS_DELIVERANCE,
    "Druid's Deliverance",
    CardArt::new("83b35961-f4d5-4e14-a793-335147110627", "Dan Murayama Scott"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt to you this turn. Populate.",
        EffectDef::Sequence(&DRUIDS_DELIVERANCE_EFFECTS),
    )),
);

// RTR 124 — Gatecreeper Vine
pub(in crate::card::sets) static GATECREEPER_VINE: CardRecord = CardRecord::new(
    cards::GATECREEPER_VINE,
    "Gatecreeper Vine",
    CardArt::new("5dabcc2f-7536-44e3-a495-bbfc526fdc5d", "Trevor Claxton"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 2).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "When this creature enters, you may search your library for a basic land card or a Gate card, reveal it, put it into your hand, then shuffle.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        ObjectPredicateDef::Subtype("Gate"),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    binding: None,
                    then: None,
                },
            },
        ),
    ]),
);

// RTR 126 — Gobbling Ooze
pub(in crate::card::sets) static GOBBLING_OOZE: CardRecord = CardRecord::new(
    cards::GOBBLING_OOZE,
    "Gobbling Ooze",
    CardArt::new("465d8a63-0ced-4aec-be34-2098b72c8af6", "Johann Bodin"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Ooze"], 3, 3).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice another creature: Put a +1/+1 counter on this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// RTR 127 — Golgari Decoy
pub(in crate::card::sets) static GOLGARI_DECOY: CardRecord = CardRecord::new(
    cards::GOLGARI_DECOY,
    "Golgari Decoy",
    CardArt::new("511a42a8-71ce-476f-98fa-fc0dc822edcf", "Marco Nelor"),
    CardSet::ReturnToRavnica,
    // A requirement, not a permission: it takes away the alternatives rather
    // than letting anything block that could not already.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All creatures able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )),
            },
        ),
        abilities::scavenge(
            mana_cost!("{3}{G}{G}"),
            "Scavenge {3}{G}{G} ({3}{G}{G}, Exile this card from your graveyard: Put a number \
             of +1/+1 counters equal to this card's power on target creature. Scavenge only as \
             a sorcery.)",
        ),
    ]),
);

static HORNCALLERS_CHANT_EFFECTS: [EffectDef; 2] = [
    EffectDef::CreateToken {
        token: cards::RHINO_TOKEN_4_4_GREEN,
        controller: None,
        count: ValueDef::Constant(1),
        tapped: false,
        attacking: false,
        counters: None,
        created: None,
    },
    abilities::populate(),
];

// RTR 128 — Horncaller's Chant
pub(in crate::card::sets) static HORNCALLERS_CHANT: CardRecord = CardRecord::new(
    cards::HORNCALLERS_CHANT,
    "Horncaller's Chant",
    CardArt::new("7b8d33ed-9ca2-41d1-ba35-fdeb5b88ad44", "Eric Velhagen"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{7}{G}")).with_ability(AbilityDef::spell(
        "Create a 4/4 green Rhino creature token with trample, then populate.",
        EffectDef::Sequence(&HORNCALLERS_CHANT_EFFECTS),
    )),
);

// RTR 129 — Korozda Monitor
pub(in crate::card::sets) static KOROZDA_MONITOR: CardRecord = CardRecord::new(
    cards::KOROZDA_MONITOR,
    "Korozda Monitor",
    CardArt::new("2f319d57-7a54-4b10-86d4-58d7b3994844", "Lars Grant-West"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Lizard"], 3, 3).with_abilities(&[
        abilities::trample(),
        abilities::scavenge(
            mana_cost!("{5}{G}{G}"),
            "Scavenge {5}{G}{G} ({5}{G}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 130 — Mana Bloom
// Audit: blocked — Needs X entry counters, a remove-counter mana cost limited to once each turn, and a no-charge-counter upkeep condition.

// RTR 131 — Oak Street Innkeeper
// Audit: blocked — Needs a continuous other-player-turn and tapped-state condition when granting hexproof.

// RTR 132 — Rubbleback Rhino
pub(in crate::card::sets) static RUBBLEBACK_RHINO: CardRecord = keyword_creature(
    cards::RUBBLEBACK_RHINO,
    "Rubbleback Rhino",
    "51daaf9b-d8a8-49a6-94e1-0c8be2c6188b",
    "Johann Bodin",
    mana_cost!("{4}{G}"),
    &["Rhino"],
    3,
    4,
    abilities::hexproof(),
);

// RTR 133 — Savage Surge
pub(in crate::card::sets) static SAVAGE_SURGE: CardRecord = CardRecord::new(
    cards::SAVAGE_SURGE,
    "Savage Surge",
    CardArt::new("0fa74aae-e857-410c-8836-953c8623d0b0", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. Untap that creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// RTR 134 — Seek the Horizon
pub(in crate::card::sets) static SEEK_THE_HORIZON: CardRecord = CardRecord::new(
    cards::SEEK_THE_HORIZON,
    "Seek the Horizon",
    CardArt::new(
        "b6f52ac7-933f-4b31-8576-338f5dcf4285",
        "Howard Lyon",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to three basic land cards, reveal them, put them into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(3),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )),
);

// RTR 135 — Slime Molding
// Audit: blocked — Token creation cannot produce a token whose power and toughness are the chosen X value.

// RTR 136 — Stonefare Crocodile
pub(in crate::card::sets) static STONEFARE_CROCODILE: CardRecord = CardRecord::new(
    cards::STONEFARE_CROCODILE,
    "Stonefare Crocodile",
    CardArt::new("a2517d74-0589-49dc-88f1-1fc02b27bc9d", "Tomasz Jedruszek"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Crocodile"], 3, 2).with_ability(
        AbilityDef::activated(
            "{2}{B}: This creature gains lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 137 — Towering Indrik
pub(in crate::card::sets) static TOWERING_INDRIK: CardRecord = keyword_creature(
    cards::TOWERING_INDRIK,
    "Towering Indrik",
    "c6049e92-6c52-44be-a3c7-aa8e8bf9c10a",
    "Lars Grant-West",
    mana_cost!("{3}{G}"),
    &["Beast"],
    2,
    4,
    abilities::reach(),
);

// RTR 138 — Urban Burgeoning
// Audit: blocked — Needs an Aura-granted untap action during each other player's untap step.

// RTR 139 — Wild Beastmaster
// Audit: blocked — Needs this creature's power captured as X when the attack trigger resolves so the resulting bonus remains fixed for the turn.

/// Both walks, because "from anywhere" needs both: the battlefield walk is
/// what sees a permanent die, and only the graveyard walk sees a card that
/// was milled or discarded. Neither sees the other's event, so the ability
/// listening from both fires once either way.
static WURM_GRAVEYARD_ZONES: [ZoneKind; 2] = [ZoneKind::Battlefield, ZoneKind::Graveyard];

static WURM_SHUFFLES_ITSELF_BACK: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Source,
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::players(PlayerSetDef::One(PlayerRefDef::OwnerOf(
            ObjectRefDef::Source,
        ))),
    },
];

static WORLDSPINE_WURM_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    // A separate ability from the shuffle below, and it only watches the
    // battlefield: a Wurm milled out of a library makes nothing.
    AbilityDef::triggered(
        "When this creature dies, create three 5/5 green Wurm creature tokens with trample.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::CreateToken {
            token: cards::WURM_TOKEN_5_5_GREEN,
            controller: None,
            count: ValueDef::Constant(3),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    ),
    // A trigger rather than a replacement, which is the whole reason the
    // tokens happen: the Wurm reaches the graveyard, both abilities see it
    // there, and only then does it go home.
    AbilityDef::triggered(
        "When Worldspine Wurm is put into a graveyard from anywhere, shuffle it into its owner's \
         library.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Graveyard)),
        EffectDef::Sequence(&WURM_SHUFFLES_ITSELF_BACK),
    )
    .with_source_zones(&WURM_GRAVEYARD_ZONES),
];

// RTR 140 — Worldspine Wurm
pub(in crate::card::sets) static WORLDSPINE_WURM: CardRecord = CardRecord::new(
    cards::WORLDSPINE_WURM,
    "Worldspine Wurm",
    CardArt::new("543d55cb-3a6b-4620-af25-10ae74ed32c4", "Richard Wright"),
    CardSet::ReturnToRavnica,
    // Eleven mana nobody pays: it is reanimated or put onto the battlefield
    // some other way, and the shuffle is what stops that from being repeatable.
    CardRules::new_creature(mana_cost!("{8}{G}{G}{G}"), &["Wurm"], 15, 15)
        .with_abilities(&WORLDSPINE_WURM_ABILITIES),
);

// RTR 141 — Abrupt Decay
pub(in crate::card::sets) static ABRUPT_DECAY: CardRecord = CardRecord::new(
    cards::ABRUPT_DECAY,
    "Abrupt Decay",
    CardArt::new("3b1e92b4-6e53-4dba-a572-c67e01965ac5", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Destroy target nonland permanent with mana value 3 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// RTR 142 — Archon of the Triumvirate
pub(in crate::card::sets) static ARCHON_OF_THE_TRIUMVIRATE: CardRecord = CardRecord::new(
    cards::ARCHON_OF_THE_TRIUMVIRATE,
    "Archon of the Triumvirate",
    CardArt::new("bf91d847-4a87-4a65-8d6d-e20d538c5cec", "David Rapoza"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{W}{U}"), &["Archon"], 4, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, detain up to two target nonland permanents your \
             opponents control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &UP_TO_TWO_OPPOSING_NONLANDS,
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

static UP_TO_TWO_OPPOSING_NONLANDS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
    2,
)];

// RTR 143 — Armada Wurm
pub(in crate::card::sets) static ARMADA_WURM: CardRecord = CardRecord::new(
    cards::ARMADA_WURM,
    "Armada Wurm",
    CardArt::new("50cb4bf3-70d1-4acc-a1fb-49f4ea74ca16", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}{W}{W}"), &["Wurm"], 5, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "When this creature enters, create a 5/5 green Wurm creature token with trample.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken {
                token: cards::WURM_TOKEN_5_5_GREEN,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
    ]),
);

// RTR 144 — Auger Spree
pub(in crate::card::sets) static AUGER_SPREE: CardRecord = CardRecord::new(
    cards::AUGER_SPREE,
    "Auger Spree",
    CardArt::new("9580a40b-b413-4f0d-9b38-13903a9d367d", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +4/-4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(4),
                ValueDef::Constant(-4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// RTR 145 — Azorius Charm
pub(in crate::card::sets) static AZORIUS_CHARM: CardRecord = CardRecord::new(
    cards::AZORIUS_CHARM,
    "Azorius Charm",
    CardArt::new("26adc211-d089-4102-91e5-225bbeb5f382", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Creatures you control gain lifelink until end of turn.\n• Draw a card.\n• Put target attacking or blocking creature on top of its owner's library.",
        &[
            AbilityDef::spell(
                "Creatures you control gain lifelink until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Draw a card",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::spell_with_targets("Put an attacking or blocking creature on top of its owner's library", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )], EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Library,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                }),
        ],
    )),
);

// RTR 146 — Call of the Conclave
pub(in crate::card::sets) static CALL_OF_THE_CONCLAVE: CardRecord = CardRecord::new(
    cards::CALL_OF_THE_CONCLAVE,
    "Call of the Conclave",
    CardArt::new("c6df8f4d-a07a-4664-878d-efec8b2affb9", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 3/3 green Centaur creature token.",
        EffectDef::CreateToken {
            token: cards::CENTAUR_TOKEN_3_3_GREEN,
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    )),
);

// RTR 147 — Carnival Hellsteed
pub(in crate::card::sets) static CARNIVAL_HELLSTEED: CardRecord = CardRecord::new(
    cards::CARNIVAL_HELLSTEED,
    "Carnival Hellsteed",
    CardArt::new("d8ada7ce-c693-48f0-a6e3-766f61d93370", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{B}{R}"), &["Nightmare", "Horse"], 5, 4).with_abilities(
        &[
            abilities::first_strike(),
            abilities::haste(),
            abilities::unleash(),
            abilities::unleash_counter(),
        ],
    ),
);

// RTR 148 — Centaur Healer
pub(in crate::card::sets) static CENTAUR_HEALER: CardRecord = CardRecord::new(
    cards::CENTAUR_HEALER,
    "Centaur Healer",
    CardArt::new("833835d1-9beb-4ad8-b675-7adebdbd7d82", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Centaur", "Cleric"], 3, 3).with_ability(
        AbilityDef::triggered(
            "When this creature enters, you gain 3 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

static CHEMISTERS_TRICK_ATTACK: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks this turn if able.");

// RTR 149 — Chemister's Trick
pub(in crate::card::sets) static CHEMISTERS_TRICK: CardRecord = CardRecord::new(
    cards::CHEMISTERS_TRICK,
    "Chemister's Trick",
    CardArt::new("dbfc2748-351f-4b5d-8a7e-bc51851578bb", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you don't control gets -2/-0 until end of turn and attacks this turn if able.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&CHEMISTERS_TRICK_ATTACK),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}{R}"),
            "Each creature you don't control gets -2/-0 until end of turn and attacks this turn if able.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::NotYou),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&CHEMISTERS_TRICK_ATTACK),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 150 — Collective Blessing
pub(in crate::card::sets) static COLLECTIVE_BLESSING: CardRecord = CardRecord::new(
    cards::COLLECTIVE_BLESSING,
    "Collective Blessing",
    CardArt::new("53c84c4d-e6d6-4eac-9d14-5b6cba914c3d", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}{W}")).with_ability(
        AbilityDef::static_ability(
            "Creatures you control get +3/+3.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
            },
        ),
    ),
);

// RTR 151 — Common Bond
pub(in crate::card::sets) static COMMON_BOND: CardRecord = CardRecord::new(
    cards::COMMON_BOND,
    "Common Bond",
    CardArt::new("59965953-1522-4103-9a6c-5534205d34d9", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature. Put a +1/+1 counter on target creature.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RTR 152 — Corpsejack Menace
// Audit: blocked — Needs a replacement effect that doubles +1/+1 counters placed on creatures you control.

// RTR 153 — Counterflux
pub(in crate::card::sets) static COUNTERFLUX: CardRecord = CardRecord::new(
    cards::COUNTERFLUX,
    "Counterflux",
    CardArt::new("94e4b773-40a4-4272-85dd-f728ada22748", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{U}{R}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Counter target spell you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{U}{U}{R}"),
            "Counter each spell you don't control.",
            EffectDef::Counter {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Spell,
                    &[ZoneKind::Stack],
                    PlayerRelation::NotYou,
                ),
                zone: ZoneKind::Graveyard,
            },
        ),
    ]),
);

static COURSERS_ACCORD_EFFECTS: [EffectDef; 2] = [
    EffectDef::CreateToken {
        token: cards::CENTAUR_TOKEN_3_3_GREEN,
        controller: None,
        count: ValueDef::Constant(1),
        tapped: false,
        attacking: false,
        counters: None,
        created: None,
    },
    abilities::populate(),
];

// RTR 154 — Coursers' Accord
pub(in crate::card::sets) static COURSERS_ACCORD: CardRecord = CardRecord::new(
    cards::COURSERS_ACCORD,
    "Coursers' Accord",
    CardArt::new("f027ceb0-5d2b-4cf6-87ad-e6b9b1e20634", "Nils Hamm"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 3/3 green Centaur creature token, then populate.",
        EffectDef::Sequence(&COURSERS_ACCORD_EFFECTS),
    )),
);

// RTR 155 — Detention Sphere
pub(in crate::card::sets) static DETENTION_SPHERE: CardRecord = CardRecord::new(
    cards::DETENTION_SPHERE,
    "Detention Sphere",
    CardArt::new("afee5464-83b7-4d7a-b407-9ee7de21535b", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}{U}")).with_abilities(&[
        AbilityDef::triggered_with_targets("When this enchantment enters, you may exile target nonland permanent not named Detention Sphere and all other permanents with the same name as that permanent.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // By name rather than by identity, so a second Sphere is
                    // no more a legal target than this one.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::SharesNameWithSource),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                },
            }),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled cards to the battlefield under their owner's control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                arrival_effect: None,
                zone: ZoneKind::Battlefield,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// RTR 156 — Dramatic Rescue
pub(in crate::card::sets) static DRAMATIC_RESCUE: CardRecord = CardRecord::new(
    cards::DRAMATIC_RESCUE,
    "Dramatic Rescue",
    CardArt::new("041afd23-1ecc-4cca-9244-fe42203ad689", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// RTR 157 — Dreadbore
pub(in crate::card::sets) static DREADBORE: CardRecord = CardRecord::new(
    cards::DREADBORE,
    "Dreadbore",
    CardArt::new("a83945c6-4dc6-4d9a-9bc2-2d4a264e5422", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{B}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature or planeswalker.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ])),
        true,
    )),
);

// RTR 158 — Dreg Mangler
pub(in crate::card::sets) static DREG_MANGLER: CardRecord = CardRecord::new(
    cards::DREG_MANGLER,
    "Dreg Mangler",
    CardArt::new("28d42d6a-e9a0-449e-9b31-436c09b7c1ba", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Plant", "Zombie"], 3, 3).with_abilities(&[
        abilities::haste(),
        abilities::scavenge(
            mana_cost!("{3}{B}{G}"),
            "Scavenge {3}{B}{G} ({3}{B}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 159 — Epic Experiment
// Audit: blocked — Needs linked top-X exile, permission to cast qualifying cards without paying their costs, and cleanup of the uncast cards.

// RTR 160 — Essence Backlash
// Audit: partial — TargetPower cannot read a creature spell's power, so the post-counter damage amount resolves as zero.
pub(in crate::card::sets) static ESSENCE_BACKLASH: CardRecord = CardRecord::new(
    cards::ESSENCE_BACKLASH,
    "Essence Backlash",
    CardArt::new("a98609dc-ea90-4c7e-a191-5e5d0ba16847", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{U}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target creature spell. Essence Backlash deals damage equal to that spell's power to its controller.",
            &[AbilityTargetDef::exactly_one_spell(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
            ]),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "TargetPower supports battlefield permanents but not a targeted creature spell or its last-known stack characteristics.",
        )),
    ),
);

// RTR 161 — Fall of the Gavel
pub(in crate::card::sets) static FALL_OF_THE_GAVEL: CardRecord = CardRecord::new(
    cards::FALL_OF_THE_GAVEL,
    "Fall of the Gavel",
    CardArt::new("64f42848-963b-4b16-aeec-66d0f349758b", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. You gain 5 life.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ]),
    )),
);

// RTR 162 — Firemind's Foresight
// Audit: blocked — Needs three sequential hidden-library searches with distinct exact mana-value predicates before one final shuffle.

static INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

// RTR 163 — Goblin Electromancer
pub(in crate::card::sets) static GOBLIN_ELECTROMANCER: CardRecord = CardRecord::new(
    cards::GOBLIN_ELECTROMANCER,
    "Goblin Electromancer",
    CardArt::new("725b112d-2637-45c1-aec8-e89981ba5fa3", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Goblin", "Wizard"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            EffectDef::ReduceMatchingSpellCostBy {
                spell: INSTANT_OR_SORCERY,
                caster: PlayerRelation::You,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// RTR 164 — Golgari Charm
pub(in crate::card::sets) static GOLGARI_CHARM: CardRecord = CardRecord::new(
    cards::GOLGARI_CHARM,
    "Golgari Charm",
    CardArt::new("48fce388-eefc-4234-8dd9-1260c1ba97eb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• All creatures get -1/-1 until end of turn.\n• Destroy target enchantment.\n• Regenerate each creature you control.",
        &[
            AbilityDef::spell(
                "All creatures get -1/-1 until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-1), ValueDef::Constant(-1)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::destroy_target(
                "Destroy target enchantment",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Enchantment,
                )),
                true,
            ),
            AbilityDef::spell(
                "Regenerate each creature you control",
                EffectDef::Regenerate {
                    object: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                },
            ),
        ],
    )),
);

// RTR 165 — Grisly Salvage
pub(in crate::card::sets) static GRISLY_SALVAGE: CardRecord = CardRecord::new(
    cards::GRISLY_SALVAGE,
    "Grisly Salvage",
    CardArt::new("dcb5eb2a-ae7a-4416-970c-6e9306689c88", "Dave Kendall"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_ability(
        AbilityDef::custom_full(
            "Reveal the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest into your graveyard.",
            CardBehavior::GrislySalvage,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// RTR 166 — Havoc Festival
// Audit: blocked — Needs a player-wide life-gain prohibition and an upkeep loss amount of half that player's life rounded up.

// RTR 167 — Hellhole Flailer
pub(in crate::card::sets) static HELLHOLE_FLAILER: CardRecord = CardRecord::new(
    cards::HELLHOLE_FLAILER,
    "Hellhole Flailer",
    CardArt::new("4984a089-84af-4387-9a0d-819b119b5565", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Ogre", "Warrior"], 3, 2).with_abilities(&[
        abilities::unleash(),
        abilities::unleash_counter(),
        AbilityDef::activated_with_targets(
            "{2}{B}{R}, Sacrifice this creature: It deals damage equal to its power to target \
             player or planeswalker.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{R}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
        ),
    ]),
);

// RTR 168 — Heroes' Reunion
pub(in crate::card::sets) static HEROES_REUNION: CardRecord = CardRecord::new(
    cards::HEROES_REUNION,
    "Heroes' Reunion",
    CardArt::new("99b56515-f688-495c-b721-2b9abc6628c2", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 7 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// RTR 169 — Hussar Patrol
pub(in crate::card::sets) static HUSSAR_PATROL: CardRecord = CardRecord::new(
    cards::HUSSAR_PATROL,
    "Hussar Patrol",
    CardArt::new("dd775231-e1e0-41e2-ad9a-0726624f57f9", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::flash(), abilities::vigilance()]),
);

// RTR 170 — Hypersonic Dragon
// Audit: blocked — Needs a static timing permission that lets every sorcery spell you cast be cast as though it had flash.

// RTR 171 — Isperia, Supreme Judge
pub(in crate::card::sets) static ISPERIA_SUPREME_JUDGE: CardRecord = CardRecord::new(
    cards::ISPERIA_SUPREME_JUDGE,
    "Isperia, Supreme Judge",
    CardArt::new("b2cce2d4-3944-4ff0-98e8-80f19697f108", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{W}{U}{U}"), &["Sphinx"], 6, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever a creature attacks you or a planeswalker you control, you may draw a card.",
                TriggerEventDef::attacks(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
        ]),
);

// RTR 172 — Izzet Charm
pub(in crate::card::sets) static IZZET_CHARM: CardRecord = CardRecord::new(
    cards::IZZET_CHARM,
    "Izzet Charm",
    CardArt::new("1e3a5af6-5423-442b-a207-364e97a871d8", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{R}")).with_ability(
        AbilityDef::choose_one_spell(
            "Choose one —\n• Counter target noncreature spell unless its controller pays {2}.\n• Izzet Charm deals 2 damage to target creature.\n• Draw two cards, then discard two cards.",
            &[
                AbilityDef::spell_with_targets("Counter a noncreature spell unless its controller pays {2}", &[AbilityTargetDef::exactly_one_spell(
                    ObjectPredicateDef::NoncreatureSpell,
                )], abilities::counter_target_unless_paid(ValueDef::Constant(2))),
                AbilityDef::spell_with_targets("Deal 2 damage to a creature", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )], EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    }),
                AbilityDef::spell(
                    "Draw two cards, then discard two cards",
                    EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    ]),
                ),
            ],
        ),
    ),
);

// RTR 173 — Izzet Staticaster
pub(in crate::card::sets) static IZZET_STATICASTER: CardRecord = CardRecord::new(
    cards::IZZET_STATICASTER,
    "Izzet Staticaster",
    CardArt::new("190ac2fe-532d-4d7e-9d74-07ae6850aac8", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{U}{R}"),
        &["Human", "Wizard"],
        0,
        3,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::haste(),
        AbilityDef::activated_with_targets("{T}: This creature deals 1 damage to target creature and each other creature with the same name as that creature.", &[AbilityCostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )], // The target and every other creature sharing its name are one
            // set, so the two printed halves are a single sweep.
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            }),
    ]),
);

// RTR 174 — Jarad, Golgari Lich Lord
// Audit: blocked — Needs a dynamic creature-card graveyard bonus and sacrifice costs whose chosen object's power and land subtypes drive linked effects.

// RTR 175 — Jarad's Orders
// Audit: blocked — Needs a two-card hidden search followed by assigning one selected creature to hand and the other to graveyard.

/// One Saproling per point of toughness the sacrifice had.
static KOROZDA_GUILDMAGE_PAYOFF: EffectDef = EffectDef::CreateToken {
    token: cards::SAPROLING_TOKEN_1_1_GREEN,
    controller: None,
    count: ValueDef::TriggerEventAmount,
    tapped: false,
    attacking: false,
    counters: None,
    created: None,
};

// RTR 176 — Korozda Guildmage
pub(in crate::card::sets) static KOROZDA_GUILDMAGE: CardRecord = CardRecord::new(
    cards::KOROZDA_GUILDMAGE,
    "Korozda Guildmage",
    CardArt::new("761c16aa-d4a7-492d-9275-98d0e07de45a", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Elf", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{B}{G}: Target creature gets +1/+1 and gains intimidate until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::intimidate()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}{B}{G}, Sacrifice a nontoken creature: Create X 1/1 green Saproling creature tokens, where X is the sacrificed creature's toughness.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}{G}"))],
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                // Nontoken, so the Saprolings it makes cannot be fed back in.
                // Nontoken, so the Saprolings it makes cannot be fed back in.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                then: Some(&KOROZDA_GUILDMAGE_PAYOFF),
                amount: SacrificedAmountDef::Toughness,
                otherwise: None,
                optional: false,
            },
        ),
    ]),
);

// RTR 177 — Lotleth Troll
// Audit: blocked — Its discard-for-counter ability is expressible, but regeneration shields are not available for the whole card.

// RTR 178 — Loxodon Smiter
pub(in crate::card::sets) static LOXODON_SMITER: CardRecord = CardRecord::new(
    cards::LOXODON_SMITER,
    "Loxodon Smiter",
    CardArt::new("69247168-2bfb-4cce-a2a6-61459a0fbce4", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{G}{W}"),
        &["Elephant", "Soldier"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::replacement_for(
            "If a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
            ReplacementEventDef::WouldMove {
                from: Some(ZoneKind::Hand),
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::EffectControlledBy(PlayerRelation::Opponent),
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// RTR 179 — Lyev Skyknight
pub(in crate::card::sets) static LYEV_SKYKNIGHT: CardRecord = CardRecord::new(
    cards::LYEV_SKYKNIGHT,
    "Lyev Skyknight",
    CardArt::new("11cbeb3b-1579-4318-a024-4a2c06896eaf", "Johannes Voss"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Human", "Knight"], 3, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, detain target nonland permanent an opponent controls.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 180 — Mercurial Chemister
// Audit: blocked — The second ability needs the discarded card's mana value linked through its activation cost as the damage amount.

// RTR 181 — New Prahv Guildmage
pub(in crate::card::sets) static NEW_PRAHV_GUILDMAGE: CardRecord = CardRecord::new(
    cards::NEW_PRAHV_GUILDMAGE,
    "New Prahv Guildmage",
    CardArt::new("698b47d1-c72e-4dc3-b28b-7421e0163f22", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}{U}: Target creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&NEW_PRAHV_FLYING),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}{W}{U}: Detain target nonland permanent an opponent controls.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}{U}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

static NEW_PRAHV_FLYING: AbilityDef = abilities::flying();

// RTR 182 — Nivix Guildmage
// Audit: blocked — Its second activation needs copying a targeted instant or sorcery spell and optionally choosing new targets for the copy.

// RTR 183 — Niv-Mizzet, Dracogenius
pub(in crate::card::sets) static NIV_MIZZET_DRACOGENIUS: CardRecord = CardRecord::new(
    cards::NIV_MIZZET_DRACOGENIUS,
    "Niv-Mizzet, Dracogenius",
    CardArt::new("c345e475-8095-41b5-90b4-771fcf80b939", "Todd Lockwood"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}{U}{R}{R}"), &["Dragon", "Wizard"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever Niv-Mizzet deals damage to a player, you may draw a card.",
                TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
            AbilityDef::activated_with_targets(
                "{U}{R}: Niv-Mizzet deals 1 damage to any target.",
                &[AbilityCostDef::Mana(mana_cost!("{U}{R}"))],
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

// RTR 184 — Rakdos Charm
// Audit: blocked — Its third mode needs each creature to be the source of damage dealt to its own controller.

// RTR 185 — Rakdos Ragemutt
pub(in crate::card::sets) static RAKDOS_RAGEMUTT: CardRecord = CardRecord::new(
    cards::RAKDOS_RAGEMUTT,
    "Rakdos Ragemutt",
    CardArt::new("bb36840a-3f85-4fca-87ab-379dfce8e542", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Elemental", "Dog"], 3, 3)
        .with_abilities(&[abilities::lifelink(), abilities::haste()]),
);

// RTR 186 — Rakdos Ringleader
pub(in crate::card::sets) static RAKDOS_RINGLEADER: CardRecord = CardRecord::new(
    cards::RAKDOS_RINGLEADER,
    "Rakdos Ringleader",
    CardArt::new("6b54fbe8-324a-4066-bed1-dda1dca319fc", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{4}{B}{R}"),
        &["Skeleton", "Warrior"],
        3,
        1,
    )
    .with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a card at random.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// RTR 187 — Rakdos, Lord of Riots
// Audit: blocked — Needs a life-lost-this-turn cast restriction and a global creature-spell cost reduction derived from opponents' life loss.

// RTR 188 — Rakdos's Return
// Audit: blocked — Targeting cannot restrict a player-or-planeswalker union to an opponent while routing the discard to that player or the planeswalker's controller.

/// "Its controller's hand", not the Aura controller's, so gifting the
/// creature away moves the bonus and the extra draw with it.
static CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND: ValueDef = ValueDef::CardsInHandAbove {
    player: PlayerRelation::ControllerOfAttachedPermanent,
    threshold: 0,
};

// RTR 189 — Righteous Authority
pub(in crate::card::sets) static RIGHTEOUS_AUTHORITY: CardRecord = CardRecord::new(
    cards::RIGHTEOUS_AUTHORITY,
    "Righteous Authority",
    CardArt::new("6695e5bb-56a9-49ab-8940-72336e845875", "Scott Chou"),
    CardSet::ReturnToRavnica,
    // The extra draw feeds the bonus: one more card in hand is one more
    // power on the creature holding it.
    CardRules::new_enchantment(mana_cost!("{3}{W}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each card in its controller's hand.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND,
                        CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND,
                    ),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of the draw step of enchanted creature's controller, that \
                 player draws an additional card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Draw,
                    player: PlayerRelation::ControllerOfAttachedPermanent,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// RTR 190 — Risen Sanctuary
pub(in crate::card::sets) static RISEN_SANCTUARY: CardRecord = keyword_creature(
    cards::RISEN_SANCTUARY,
    "Risen Sanctuary",
    "a0b6c136-2bbe-48c1-ac53-2a8221b96936",
    "Chase Stone",
    mana_cost!("{5}{G}{W}"),
    &["Elemental"],
    8,
    8,
    abilities::vigilance(),
);

// RTR 191 — Rites of Reaping
// Audit: blocked — Needs two creature targets constrained to be different; ordinary target slots currently allow choosing the same creature twice.

// RTR 192 — Rix Maadi Guildmage
// Audit: blocked — Its second activation needs a target-player predicate for a player who lost life this turn.

// RTR 193 — Search Warrant
// Audit: blocked — Needs revealing a target player's hand and counting the cards in that player's hand for the life-gain amount.

// RTR 194 — Selesnya Charm
// Audit: partial — Its power target predicate ignores power changes from static continuous effects.
pub(in crate::card::sets) static SELESNYA_CHARM: CardRecord = CardRecord::new(
    cards::SELESNYA_CHARM,
    "Selesnya Charm",
    CardArt::new("a9848eab-1d3a-4ab0-adf6-c20858aa3afb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Target creature gets +2/+2 and gains trample until end of turn.\n• Exile target creature with power 5 or greater.\n• Create a 2/2 white Knight creature token with vigilance.",
        &[
            AbilityDef::spell_with_targets("Target creature gets +2/+2 and gains trample until end of turn", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                }),
            AbilityDef::spell_with_targets("Exile a creature with power 5 or greater", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(5),
                ]),
            )], EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                }).with_coverage(AbilityCoverageDef::partial(
                    "PowerAtLeast reads resolved power changes but not power changes supplied by static continuous effects.",
                )),
            AbilityDef::spell(
                "Create a 2/2 white Knight creature token with vigilance",
                EffectDef::CreateToken {
                    token: cards::KNIGHT_TOKEN_2_2_WHITE,
                    controller: None,
                    count: ValueDef::Constant(1),
                    tapped: false,
                    attacking: false,
                counters: None,
                created: None,},
            ),
        ],
    )),
);

// RTR 195 — Skull Rend
pub(in crate::card::sets) static SKULL_REND: CardRecord = CardRecord::new(
    cards::SKULL_REND,
    "Skull Rend",
    CardArt::new("1c8efb23-bac0-41d2-b4ee-27a6b1fe3134", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{B}{R}")).with_ability(AbilityDef::spell(
        "Skull Rend deals 2 damage to each opponent. Those players each discard two cards at random.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ]),
    )),
);

static SKYMARK_ROC_RETURN: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Hand,
    controller: None,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
};

// RTR 196 — Skymark Roc
// Audit: partial — Its toughness target predicate ignores changes from static continuous effects.
pub(in crate::card::sets) static SKYMARK_ROC: CardRecord = CardRecord::new(
    cards::SKYMARK_ROC,
    "Skymark Roc",
    CardArt::new("60601296-2229-4c48-94cc-1903926750ce", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Bird"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may return target creature defending player controls with toughness 2 or less to its owner's hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &SKYMARK_ROC_RETURN,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "ToughnessLessThan reads resolved power/toughness changes but not changes supplied by static continuous effects.",
        )),
    ]),
);

// RTR 197 — Slaughter Games
// Audit: blocked — Needs a nonland card-name choice and a name-linked search across an opponent's graveyard, hand, and library.

// RTR 198 — Sluiceway Scorpion
pub(in crate::card::sets) static SLUICEWAY_SCORPION: CardRecord = CardRecord::new(
    cards::SLUICEWAY_SCORPION,
    "Sluiceway Scorpion",
    CardArt::new("7b6dbadf-a6f7-4876-9c3f-44e4a33b2bee", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Scorpion"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::scavenge(
            mana_cost!("{1}{B}{G}"),
            "Scavenge {1}{B}{G} ({1}{B}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 199 — Spawn of Rix Maadi
pub(in crate::card::sets) static SPAWN_OF_RIX_MAADI: CardRecord = CardRecord::new(
    cards::SPAWN_OF_RIX_MAADI,
    "Spawn of Rix Maadi",
    CardArt::new("6196e702-7f76-49db-8ee4-bd343359d498", "Min Yum"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Horror"], 5, 3)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 200 — Sphinx's Revelation
pub(in crate::card::sets) static SPHINXS_REVELATION: CardRecord = CardRecord::new(
    cards::SPHINXS_REVELATION,
    "Sphinx's Revelation",
    CardArt::new("404d9413-ef57-4b6e-8584-48a1dc7fe6f1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{W}{U}{U}")).with_ability(AbilityDef::custom_full(
        "You gain X life and draw X cards.",
        CardBehavior::SphinxsRevelation,
        "Implemented by the named card-local special behavior.",
    )),
);

// RTR 201 — Supreme Verdict
pub(in crate::card::sets) static SUPREME_VERDICT: CardRecord = CardRecord::new(
    cards::SUPREME_VERDICT,
    "Supreme Verdict",
    CardArt::new("4e9648f9-7a67-4717-bca1-861d1f7fed43", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}{U}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell(
            "Destroy all creatures.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
            },
        ),
    ]),
);

// RTR 202 — Teleportal
pub(in crate::card::sets) static TELEPORTAL: CardRecord = CardRecord::new(
    cards::TELEPORTAL,
    "Teleportal",
    CardArt::new("e438c718-ac18-45d5-824e-5b697c5f0692", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +1/+0 until end of turn and can't be blocked this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                        ObjectPredicateDef::Any,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}{R}"),
            "Each creature you control gets +1/+0 until end of turn and can't be blocked this turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                        ObjectPredicateDef::Any,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 203 — Thoughtflare
pub(in crate::card::sets) static THOUGHTFLARE: CardRecord = CardRecord::new(
    cards::THOUGHTFLARE,
    "Thoughtflare",
    CardArt::new("d90514aa-e356-4502-9e0e-76ab7644a07a", "David Rapoza"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{U}{R}")).with_ability(AbilityDef::spell(
        "Draw four cards, then discard two cards.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// RTR 204 — Treasured Find
// Audit: partial — MoveToZone cannot move the resolving source spell from the stack to exile, so it goes to the graveyard normally.
pub(in crate::card::sets) static TREASURED_FIND: CardRecord = CardRecord::new(
    cards::TREASURED_FIND,
    "Treasured Find",
    CardArt::new("a2c0e00b-2290-493f-a3fc-3b9bff2830cc", "Jason Chan"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target card from your graveyard to your hand. Exile Treasured Find.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ]),
    )
    .with_coverage(AbilityCoverageDef::partial(
        "MoveToZone supports permanents and cards in nonbattlefield zones, but not the resolving source spell on the stack.",
    ))),
);

// RTR 205 — Trestle Troll
pub(in crate::card::sets) static TRESTLE_TROLL: CardRecord = CardRecord::new(
    cards::TRESTLE_TROLL,
    "Trestle Troll",
    CardArt::new("6d224279-83f3-4a29-9fd9-86b72407b87a", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Troll"], 1, 4).with_abilities(&[
        abilities::defender(),
        abilities::reach(),
        abilities::regenerate_self(
            "{1}{B}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}{G}"))],
        ),
    ]),
);

/// "Another creature you control", which is three conditions rather than one:
/// a creature, yours, and not Trostani herself.
static TROSTANI_ANOTHER_CREATURE: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

// RTR 206 — Trostani, Selesnya's Voice
pub(in crate::card::sets) static TROSTANI_SELESNYAS_VOICE: CardRecord = CardRecord::new(
    cards::TROSTANI_SELESNYAS_VOICE,
    "Trostani, Selesnya's Voice",
    CardArt::new("9d1d9d86-5666-4e59-9766-137657b4e040", "Chippy"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{G}{G}{W}{W}"), &["Dryad"], 2, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            // The toughness read is the entering creature's, so a token
            // copied by the ability below feeds this one on the way in.
            AbilityDef::triggered(
                "Whenever another creature you control enters, you gain life equal to that \
                 creature's toughness.",
                TriggerEventDef::zone_changed(
                    TROSTANI_ANOTHER_CREATURE,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggeringObjectToughness,
                },
            ),
            AbilityDef::activated(
                "{1}{G}{W}, {T}: Populate.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}{G}{W}")),
                    AbilityCostDef::TapSource,
                ],
                abilities::populate(),
            ),
        ]),
);

// RTR 207 — Vitu-Ghazi Guildmage
pub(in crate::card::sets) static VITU_GHAZI_GUILDMAGE: CardRecord = CardRecord::new(
    cards::VITU_GHAZI_GUILDMAGE,
    "Vitu-Ghazi Guildmage",
    CardArt::new("e54f8e61-550f-4493-b8ba-65f81b2457d3", "Jason Chan"),
    CardSet::ReturnToRavnica,
    // Two abilities rather than one modal ability: each has its own cost, so
    // there is nothing to choose between at activation time.
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Dryad", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{G}{W}: Create a 3/3 green Centaur creature token.",
            &[AbilityCostDef::Mana(mana_cost!("{4}{G}{W}"))],
            EffectDef::CreateToken {
                token: cards::CENTAUR_TOKEN_3_3_GREEN,
                controller: None,
                count: ValueDef::Constant(1),
                tapped: false,
                attacking: false,
                counters: None,
                created: None,
            },
        ),
        AbilityDef::activated(
            "{2}{G}{W}: Populate.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{W}"))],
            abilities::populate(),
        ),
    ]),
);

/// The delayed trigger Vraska's +1 hangs on herself. It reads damage arriving
/// at the planeswalker, which only became reachable once a creature could
/// attack one.
static VRASKA_RETALIATION: AbilityDef = AbilityDef::triggered(
    "Whenever a creature deals combat damage to Vraska, destroy that creature.",
    TriggerEventDef::combat_damage_to_source(ObjectPredicateDef::HasType(CardType::Creature)),
    EffectDef::Destroy {
        object: EffectRecipientDef::TriggeringObject,
        can_regenerate: true,
    },
);

static VRASKA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Until your next turn, whenever a creature deals combat damage to Vraska, destroy that creature.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&VRASKA_RETALIATION),
            duration: ResolvedEffectDurationDef::UntilYourNextTurn,
        },
    ),
    AbilityDef::activated_with_targets(
        "−3: Destroy target nonland permanent.",
        &[AbilityCostDef::Loyalty(-3)],
        &VRASKA_DESTROY_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    ),
    AbilityDef::activated(
        "−7: Create three 1/1 black Assassin creature tokens with \"Whenever this token deals combat damage to a player, that player loses the game.\"",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::CreateToken {
            token: cards::ASSASSIN_TOKEN_1_1_BLACK,
            controller: None,
            count: ValueDef::Constant(3),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    ),
];

static VRASKA_DESTROY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];
// RTR 208 — Vraska the Unseen
pub(in crate::card::sets) static VRASKA_THE_UNSEEN: CardRecord = CardRecord::new(
    cards::VRASKA_THE_UNSEEN,
    "Vraska the Unseen",
    CardArt::new("8971938c-cd26-4b83-96d7-1408cd0b0de6", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_planeswalker(mana_cost!("{3}{B}{G}"), &["Vraska"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&VRASKA_ABILITIES),
);

static WAYFARING_TEMPLE_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// Counted live and including itself, so a lone Temple is a 1/1.
static WAYFARING_TEMPLE_SIZE: AppliedEffectDef = AppliedEffectDef::set_base_power_toughness(
    ValueDef::CountMatchingObjects(&WAYFARING_TEMPLE_CREATURES),
    ValueDef::CountMatchingObjects(&WAYFARING_TEMPLE_CREATURES),
);

// RTR 209 — Wayfaring Temple
pub(in crate::card::sets) static WAYFARING_TEMPLE: CardRecord = CardRecord::new(
    cards::WAYFARING_TEMPLE,
    "Wayfaring Temple",
    CardArt::new("2125e6aa-f916-4dfa-a9fe-82bb546012af", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Elemental"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Wayfaring Temple's power and toughness are each equal to the number of creatures \
             you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: WAYFARING_TEMPLE_SIZE,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, populate.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::populate(),
        ),
    ]),
);

// RTR 210 — Azor's Elocutors
// Audit: blocked — Needs filibuster counters, a five-counter win condition, and damage-to-player triggers that remove that custom counter kind.

// RTR 211 — Blistercoil Weird
pub(in crate::card::sets) static BLISTERCOIL_WEIRD: CardRecord = CardRecord::new(
    cards::BLISTERCOIL_WEIRD,
    "Blistercoil Weird",
    CardArt::new("d2a8e716-ea33-4ae2-9ff8-5e78b0e50459", "Dan Murayama Scott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U/R}"), &["Weird"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature gets +1/+1 until end of turn. Untap it.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ),
);

// RTR 212 — Cryptborn Horror
// Audit: blocked — Needs the total life lost by all opponents this turn as a dynamic enters-with-counter replacement value.

// RTR 213 — Deathrite Shaman
// Audit: partial — A mana ability cannot both exile its graveyard target and produce a chosen color through the shared mana-ability path.
pub(in crate::card::sets) static DEATHRITE_SHAMAN: CardRecord = CardRecord::new(
    cards::DEATHRITE_SHAMAN,
    "Deathrite Shaman",
    CardArt::new("70496f16-c4c0-4c03-beef-454eb4824cd1", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/G}"), &["Elf", "Shaman"], 1, 2).with_abilities(&[
        AbilityDef::not_implemented(
            "{T}: Exile target land card from a graveyard. Add one mana of any color.",
            "The shared mana-ability path requires a direct AddMana effect and cannot first exile a targeted graveyard card.",
        ),
        AbilityDef::activated_with_targets(
            "{B}, {T}: Exile target instant or sorcery card from a graveyard. Each opponent loses 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        AbilityDef::activated_with_targets(
            "{G}, {T}: Exile target creature card from a graveyard. You gain 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// RTR 214 — Dryad Militant
// Audit: blocked — The graveyard replacement event cannot filter the moving object to instant or sorcery cards.

// RTR 215 — Frostburn Weird
pub(in crate::card::sets) static FROSTBURN_WEIRD: CardRecord = CardRecord::new(
    cards::FROSTBURN_WEIRD,
    "Frostburn Weird",
    CardArt::new("ba5a68d3-6bc9-4de8-bc06-e1106cf9b3d4", "Mike Bierek"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U/R}{U/R}"), &["Weird"], 1, 4).with_ability(
        AbilityDef::activated(
            "{U/R}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U/R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 216 — Golgari Longlegs
pub(in crate::card::sets) static GOLGARI_LONGLEGS: CardRecord = vanilla_creature(
    cards::GOLGARI_LONGLEGS,
    "Golgari Longlegs",
    "d44058ba-3419-4777-8d59-05dea5e864e1",
    "Volkan Baǵa",
    mana_cost!("{3}{B/G}{B/G}"),
    &["Insect"],
    5,
    4,
);

// RTR 217 — Growing Ranks
pub(in crate::card::sets) static GROWING_RANKS: CardRecord = CardRecord::new(
    cards::GROWING_RANKS,
    "Growing Ranks",
    CardArt::new("12f31616-1249-4964-b81a-4435405a2449", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{G/W}{G/W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, populate.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        abilities::populate(),
    )),
);

// RTR 218 — Judge's Familiar
pub(in crate::card::sets) static JUDGES_FAMILIAR: CardRecord = CardRecord::new(
    cards::JUDGES_FAMILIAR,
    "Judge's Familiar",
    CardArt::new("0fc51899-3970-416b-b7de-fadbc9678955", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W/U}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Counter target instant or sorcery spell unless its controller pays {1}.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]))],
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
    ]),
);

// RTR 219 — Nivmagus Elemental
// Audit: blocked — Needs exiling a chosen instant or sorcery spell you control from the stack as an activation cost.

// RTR 220 — Rakdos Cackler
pub(in crate::card::sets) static RAKDOS_CACKLER: CardRecord = CardRecord::new(
    cards::RAKDOS_CACKLER,
    "Rakdos Cackler",
    CardArt::new("5f873c0b-e779-4f09-8e9c-94a1765eb5da", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/R}"), &["Devil"], 1, 1)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 221 — Rakdos Shred-Freak
pub(in crate::card::sets) static RAKDOS_SHRED_FREAK: CardRecord = keyword_creature(
    cards::RAKDOS_SHRED_FREAK,
    "Rakdos Shred-Freak",
    "06899549-5534-4d11-86c1-afd1796e18b1",
    "Wayne Reynolds",
    mana_cost!("{B/R}{B/R}"),
    &["Human", "Berserker"],
    2,
    1,
    abilities::haste(),
);

// RTR 222 — Slitherhead
pub(in crate::card::sets) static SLITHERHEAD: CardRecord = CardRecord::new(
    cards::SLITHERHEAD,
    "Slitherhead",
    CardArt::new("d9327905-a254-4885-8310-69fc153ec52f", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/G}"), &["Plant", "Zombie"], 1, 1).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{0}"),
            "Scavenge {0} ({0}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

/// "Artifact or enchantment", which is one target slot rather than two.
static SUNDERING_GROWTH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Enchantment),
    ]),
)];

/// The destruction comes first, so a token the destroyed permanent was
/// keeping alive is already gone when the copy is chosen.
static SUNDERING_GROWTH_EFFECTS: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    abilities::populate(),
];

// RTR 223 — Sundering Growth
pub(in crate::card::sets) static SUNDERING_GROWTH: CardRecord = CardRecord::new(
    cards::SUNDERING_GROWTH,
    "Sundering Growth",
    CardArt::new("14d5048e-cb76-48c4-8a95-70dcc14775f6", "David Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G/W}{G/W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment, then populate.",
        &SUNDERING_GROWTH_TARGET,
        EffectDef::Sequence(&SUNDERING_GROWTH_EFFECTS),
    )),
);

// RTR 224 — Vassal Soul
pub(in crate::card::sets) static VASSAL_SOUL: CardRecord = keyword_creature(
    cards::VASSAL_SOUL,
    "Vassal Soul",
    "dfc61748-029f-4bae-a7ec-e08b7059226d",
    "Dan Murayama Scott",
    mana_cost!("{1}{W/U}{W/U}"),
    &["Spirit"],
    2,
    2,
    abilities::flying(),
);

const fn keyrune_animation(
    power: i32,
    toughness: i32,
    creature_types: &'static [&'static str],
    colors: ColorSet,
) -> [AppliedEffectDef; 4] {
    [
        AppliedEffectDef::add_card_types(
            CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
        ),
        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(creature_types)),
        AppliedEffectDef::set_colors(colors),
        AppliedEffectDef::set_base_power_toughness(
            ValueDef::Constant(power),
            ValueDef::Constant(toughness),
        ),
    ]
}

static AZORIUS_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    2,
    2,
    &["Bird"],
    ColorSet::from_colors(&[ManaColor::White, ManaColor::Blue]),
);

// RTR 225 — Azorius Keyrune
pub(in crate::card::sets) static AZORIUS_KEYRUNE: CardRecord = CardRecord::new(
    cards::AZORIUS_KEYRUNE,
    "Azorius Keyrune",
    CardArt::new("23a05db7-dcae-4180-8ad1-60ba6fb30816", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{W}{U}: This artifact becomes a 2/2 white and blue Bird artifact creature with flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&AZORIUS_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static CHROMATIC_LANTERN_MANA: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::choice(&[
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ])),
);

// RTR 226 — Chromatic Lantern
pub(in crate::card::sets) static CHROMATIC_LANTERN: CardRecord = CardRecord::new(
    cards::CHROMATIC_LANTERN,
    "Chromatic Lantern",
    CardArt::new("57f4e0f0-13d1-43ed-8d95-13cff94a26e7", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "Lands you control have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&CHROMATIC_LANTERN_MANA),
            },
        ),
        CHROMATIC_LANTERN_MANA,
    ]),
);

// RTR 227 — Civic Saber
pub(in crate::card::sets) static CIVIC_SABER: CardRecord = CardRecord::new(
    cards::CIVIC_SABER,
    "Civic Saber",
    CardArt::new("29c9247e-05f4-44bb-86e3-90a60e880374", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 for each of its colors.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::AffectedColorCount,
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// RTR 228 — Codex Shredder
pub(in crate::card::sets) static CODEX_SHREDDER: CardRecord = CardRecord::new(
    cards::CODEX_SHREDDER,
    "Codex Shredder",
    CardArt::new("8f7b632b-ee20-4082-8376-1dae53f91b70", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target player mills a card.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                binding: None,
                then: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{5}, {T}, Sacrifice this artifact: Return target card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{5}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
            },
        ),
    ]),
);

static GOLGARI_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    2,
    2,
    &["Insect"],
    ColorSet::from_colors(&[ManaColor::Black, ManaColor::Green]),
);

// RTR 229 — Golgari Keyrune
pub(in crate::card::sets) static GOLGARI_KEYRUNE: CardRecord = CardRecord::new(
    cards::GOLGARI_KEYRUNE,
    "Golgari Keyrune",
    CardArt::new("913b803f-ba82-4660-86be-49677d1e32c9", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated(
            "{B}{G}: This artifact becomes a 2/2 black and green Insect artifact creature with deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&GOLGARI_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static IZZET_KEYRUNE_LOOT: EffectDef = EffectDef::Sequence(&[
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
]);

static IZZET_KEYRUNE_COMBAT: AbilityDef = AbilityDef::triggered(
    "Whenever this artifact deals combat damage to a player, you may draw a card. If you do, discard a card.",
    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &IZZET_KEYRUNE_LOOT,
    },
);

static IZZET_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    2,
    1,
    &["Elemental"],
    ColorSet::from_colors(&[ManaColor::Blue, ManaColor::Red]),
);

// RTR 230 — Izzet Keyrune
pub(in crate::card::sets) static IZZET_KEYRUNE: CardRecord = CardRecord::new(
    cards::IZZET_KEYRUNE,
    "Izzet Keyrune",
    CardArt::new("83e4e83b-cbb8-4efd-b6c4-4459a29177ac", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated(
            "{U}{R}: Until end of turn, this artifact becomes a 2/1 blue and red Elemental artifact creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&IZZET_KEYRUNE_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        IZZET_KEYRUNE_COMBAT,
    ]),
);

// RTR 231 — Pithing Needle
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new(
    cards::PITHING_NEEDLE,
    "Pithing Needle",
    CardArt::new("786c1e91-9d75-46a3-9e0d-56d29fcb01a7", "Anthony Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::replacement(
            "As this artifact enters, choose a card name.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                crate::card::BattlefieldEntryScalarChoiceDef::CARD_NAME,
            )),
        ),
        // The named card's abilities are locked by the action generator, the
        // same place every other activation restriction is enforced.
        AbilityDef::static_ability(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The activation lock is enforced where activated abilities are offered.",
        )),
    ]),
);

static RAKDOS_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    3,
    1,
    &["Devil"],
    ColorSet::from_colors(&[ManaColor::Black, ManaColor::Red]),
);

// RTR 232 — Rakdos Keyrune
pub(in crate::card::sets) static RAKDOS_KEYRUNE: CardRecord = CardRecord::new(
    cards::RAKDOS_KEYRUNE,
    "Rakdos Keyrune",
    CardArt::new("f6124b4c-49a1-42e3-a6ff-62de231c7823", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated(
            "{B}{R}: This artifact becomes a 3/1 black and red Devil artifact creature with first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&RAKDOS_KEYRUNE_ANIMATION),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static SELESNYA_KEYRUNE_ANIMATION: [AppliedEffectDef; 4] = keyrune_animation(
    3,
    3,
    &["Wolf"],
    ColorSet::from_colors(&[ManaColor::Green, ManaColor::White]),
);

// RTR 233 — Selesnya Keyrune
pub(in crate::card::sets) static SELESNYA_KEYRUNE: CardRecord = CardRecord::new(
    cards::SELESNYA_KEYRUNE,
    "Selesnya Keyrune",
    CardArt::new("1645eafd-cd17-463a-9d7e-42f5f7b5196f", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated(
            "{G}{W}: This artifact becomes a 3/3 green and white Wolf artifact creature until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&SELESNYA_KEYRUNE_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 234 — Street Sweeper
// Audit: blocked — Needs selecting and destroying every Aura attached to the targeted land through an attachment-relationship predicate.

// RTR 235 — Tablet of the Guilds
// Audit: blocked — Needs choosing and storing two colors, matching cast spells against both, and counting how many chosen colors match.

// RTR 236 — Volatile Rig
// Audit: blocked — Needs coin flips with lose branches for both damage and death triggers.

// RTR 237 — Azorius Guildgate
pub(in crate::card::sets) static AZORIUS_GUILDGATE: CardRecord = CardRecord::new(
    cards::AZORIUS_GUILDGATE,
    "Azorius Guildgate",
    CardArt::new("984e37df-0734-493a-a958-f519a0c98580", "Drew Baker"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
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

// RTR 238 — Blood Crypt
pub(in crate::card::sets) static BLOOD_CRYPT: CardRecord = CardRecord::new(
    cards::BLOOD_CRYPT,
    "Blood Crypt",
    CardArt::new("8bd5828b-8dcd-4ce6-b834-ebe9cbaa12d1", "Vincent Proce"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Swamp", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// RTR 239 — Golgari Guildgate
pub(in crate::card::sets) static GOLGARI_GUILDGATE: CardRecord = CardRecord::new(
    cards::GOLGARI_GUILDGATE,
    "Golgari Guildgate",
    CardArt::new("8fe2fd1a-f7d3-48b4-bad8-be5ee45d6121", "Eytan Zana"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// RTR 240 — Grove of the Guardian
// Audit: blocked — Needs tapping two chosen untapped creatures you control as one activation cost.

// RTR 241 — Hallowed Fountain
pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new(
    cards::HALLOWED_FOUNTAIN,
    "Hallowed Fountain",
    CardArt::new("af7091c9-5f98-4078-a42b-c9e057346d9b", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Plains", "Island"]).with_ability(abilities::shock_land_enters()),
);

// RTR 242 — Izzet Guildgate
pub(in crate::card::sets) static IZZET_GUILDGATE: CardRecord = CardRecord::new(
    cards::IZZET_GUILDGATE,
    "Izzet Guildgate",
    CardArt::new("6951d84f-2d3c-4203-8d31-e08f4bc707f0", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// RTR 243 — Overgrown Tomb
pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new(
    cards::OVERGROWN_TOMB,
    "Overgrown Tomb",
    CardArt::new("1c7d50d6-b63a-4d8c-88fa-1d78ae693a45", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Swamp", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// RTR 244 — Rakdos Guildgate
pub(in crate::card::sets) static RAKDOS_GUILDGATE: CardRecord = CardRecord::new(
    cards::RAKDOS_GUILDGATE,
    "Rakdos Guildgate",
    CardArt::new("207048f5-268b-4cdb-b4e7-c8282cac1b28", "Eytan Zana"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// RTR 245 — Rogue's Passage
pub(in crate::card::sets) static ROGUES_PASSAGE: CardRecord = CardRecord::new(
    cards::ROGUES_PASSAGE,
    "Rogue's Passage",
    CardArt::new("f416e36a-15cb-43c8-b27f-82f65a95ddef", "Christine Choi"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{4}, {T}: Target creature can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 246 — Selesnya Guildgate
pub(in crate::card::sets) static SELESNYA_GUILDGATE: CardRecord = CardRecord::new(
    cards::SELESNYA_GUILDGATE,
    "Selesnya Guildgate",
    CardArt::new("ff61d4e4-3c8c-48f7-a994-ec2317bbd9a0", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// RTR 247 — Steam Vents
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new(
    cards::STEAM_VENTS,
    "Steam Vents",
    CardArt::new("de911c88-f5c8-4955-9fa5-1f28a9b17236", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Island", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// RTR 248 — Temple Garden
pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new(
    cards::TEMPLE_GARDEN,
    "Temple Garden",
    CardArt::new("b821e604-f9fd-47a4-b5ff-bfb5022834c2", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Forest", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// RTR 249 — Transguild Promenade
pub(in crate::card::sets) static TRANSGUILD_PROMENADE: CardRecord = CardRecord::new(
    cards::TRANSGUILD_PROMENADE,
    "Transguild Promenade",
    CardArt::new("90ce8115-41fe-44c2-8719-741ba87bcb17", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::triggered(
            "When this land enters, sacrifice it unless you pay {1}.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{1}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_SERENITY,
    &AVENGING_ARROW,
    &AZORIUS_ARRESTER,
    &AZORIUS_JUSTICIAR,
    &BAZAAR_KROVOD,
    &CONCORDIA_PEGASUS,
    &ETHEREAL_ARMOR,
    &EYES_IN_THE_SKIES,
    &FENCING_ACE,
    &KEENING_APPARITION,
    &KNIGHTLY_VALOR,
    &MARTIAL_LAW,
    &PHANTOM_GENERAL,
    &PRECINCT_CAPTAIN,
    &REST_IN_PEACE,
    &ROOTBORN_DEFENSES,
    &SELESNYA_SENTRY,
    &SELLER_OF_SONGBIRDS,
    &SUNSPIRE_GRIFFIN,
    &SWIFT_JUSTICE,
    &TRAINED_CARACAL,
    &TROSTANIS_JUDGMENT,
    &AQUUS_STEED,
    &BLUSTERSQUALL,
    &CANCEL,
    &CROSSTOWN_COURIER,
    &CYCLONIC_RIFT,
    &DISPEL,
    &DOORKEEPER,
    &DOWNSIZE,
    &HOVER_BARRIER,
    &INACTION_INJUNCTION,
    &INSPIRATION,
    &ISPERIAS_SKYWATCH,
    &JACE_ARCHITECT_OF_THOUGHT,
    &MIZZIUM_SKIN,
    &PARALYZING_GRASP,
    &RUNEWING,
    &SKYLINE_PREDATOR,
    &SOULSWORN_SPIRIT,
    &STEALER_OF_SECRETS,
    &SYNCOPATE,
    &TOWER_DRAKE,
    &VOIDWIELDER,
    &ASSASSINS_STRIKE,
    &CATACOMB_SLUG,
    &CREMATE,
    &DAGGERDROME_IMP,
    &DEAD_REVELER,
    &DESECRATION_DEMON,
    &DEVIANT_GLEE,
    &DRAINPIPE_VERMIN,
    &GRIM_ROUSTABOUT,
    &NECROPOLIS_REGENT,
    &OGRE_JAILBREAKER,
    &PERILOUS_SHADOW,
    &SEWER_SHAMBLER,
    &SHRIEKING_AFFLICTION,
    &SLUM_REAPER,
    &STAB_WOUND,
    &TERRUS_WURM,
    &THRILL_KILL_ASSASSIN,
    &ULTIMATE_PRICE,
    &UNDERWORLD_CONNECTIONS,
    &ZANIKEV_LOCUST,
    &BATTERHORN,
    &BELLOWS_LIZARD,
    &BLOODFRAY_GIANT,
    &CHAOS_IMPS,
    &COBBLEBRUTE,
    &DYNACHARGE,
    &ELECTRICKERY,
    &EXPLOSIVE_IMPACT,
    &GOBLIN_RALLY,
    &GORE_HOUSE_CHAINWALKER,
    &GUTTERSNIPE,
    &LOBBER_CREW,
    &MINOTAUR_AGGRESSOR,
    &MIZZIUM_MORTARS,
    &PURSUIT_OF_FLIGHT,
    &PYROCONVERGENCE,
    &RACECOURSE_FURY,
    &SPLATTER_THUG,
    &STREET_SPASM,
    &SURVEY_THE_WRECKAGE,
    &TENEMENT_CRASHER,
    &TRAITOROUS_INSTINCT,
    &UTVARA_HELLKITE,
    &VANDALBLAST,
    &VIASHINO_RACKETEER,
    &AERIAL_PREDATION,
    &ARCHWEAVER,
    &AXEBANE_STAG,
    &BRUSHSTRIDER,
    &CENTAURS_HERALD,
    &CHORUS_OF_MIGHT,
    &DEADBRIDGE_GOLIATH,
    &DEATHS_PRESENCE,
    &DRUDGE_BEETLE,
    &DRUIDS_DELIVERANCE,
    &GATECREEPER_VINE,
    &GOBBLING_OOZE,
    &GOLGARI_DECOY,
    &HORNCALLERS_CHANT,
    &KOROZDA_MONITOR,
    &RUBBLEBACK_RHINO,
    &SAVAGE_SURGE,
    &SEEK_THE_HORIZON,
    &STONEFARE_CROCODILE,
    &TOWERING_INDRIK,
    &WORLDSPINE_WURM,
    &ABRUPT_DECAY,
    &ARCHON_OF_THE_TRIUMVIRATE,
    &ARMADA_WURM,
    &AUGER_SPREE,
    &AZORIUS_CHARM,
    &CALL_OF_THE_CONCLAVE,
    &CARNIVAL_HELLSTEED,
    &CENTAUR_HEALER,
    &CHEMISTERS_TRICK,
    &COLLECTIVE_BLESSING,
    &COMMON_BOND,
    &COUNTERFLUX,
    &COURSERS_ACCORD,
    &DETENTION_SPHERE,
    &DRAMATIC_RESCUE,
    &DREADBORE,
    &DREG_MANGLER,
    &ESSENCE_BACKLASH,
    &FALL_OF_THE_GAVEL,
    &GOBLIN_ELECTROMANCER,
    &GOLGARI_CHARM,
    &GRISLY_SALVAGE,
    &HELLHOLE_FLAILER,
    &HEROES_REUNION,
    &HUSSAR_PATROL,
    &ISPERIA_SUPREME_JUDGE,
    &IZZET_CHARM,
    &IZZET_STATICASTER,
    &KOROZDA_GUILDMAGE,
    &LOXODON_SMITER,
    &LYEV_SKYKNIGHT,
    &NEW_PRAHV_GUILDMAGE,
    &NIV_MIZZET_DRACOGENIUS,
    &RAKDOS_RAGEMUTT,
    &RAKDOS_RINGLEADER,
    &RIGHTEOUS_AUTHORITY,
    &RISEN_SANCTUARY,
    &SELESNYA_CHARM,
    &SKULL_REND,
    &SKYMARK_ROC,
    &SLUICEWAY_SCORPION,
    &SPAWN_OF_RIX_MAADI,
    &SPHINXS_REVELATION,
    &SUPREME_VERDICT,
    &TELEPORTAL,
    &THOUGHTFLARE,
    &TREASURED_FIND,
    &TRESTLE_TROLL,
    &TROSTANI_SELESNYAS_VOICE,
    &VITU_GHAZI_GUILDMAGE,
    &VRASKA_THE_UNSEEN,
    &WAYFARING_TEMPLE,
    &BLISTERCOIL_WEIRD,
    &DEATHRITE_SHAMAN,
    &FROSTBURN_WEIRD,
    &GOLGARI_LONGLEGS,
    &GROWING_RANKS,
    &JUDGES_FAMILIAR,
    &RAKDOS_CACKLER,
    &RAKDOS_SHRED_FREAK,
    &SLITHERHEAD,
    &SUNDERING_GROWTH,
    &VASSAL_SOUL,
    &AZORIUS_KEYRUNE,
    &CHROMATIC_LANTERN,
    &CIVIC_SABER,
    &CODEX_SHREDDER,
    &GOLGARI_KEYRUNE,
    &IZZET_KEYRUNE,
    &PITHING_NEEDLE,
    &RAKDOS_KEYRUNE,
    &SELESNYA_KEYRUNE,
    &AZORIUS_GUILDGATE,
    &BLOOD_CRYPT,
    &GOLGARI_GUILDGATE,
    &HALLOWED_FOUNTAIN,
    &IZZET_GUILDGATE,
    &OVERGROWN_TOMB,
    &RAKDOS_GUILDGATE,
    &ROGUES_PASSAGE,
    &SELESNYA_GUILDGATE,
    &STEAM_VENTS,
    &TEMPLE_GARDEN,
    &TRANSGUILD_PROMENADE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&mmq::ARREST),          // RTR 3
    PrintingRecord::reprint(&magic_2013::MIND_ROT), // RTR 70
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),  // RTR 125
    PrintingRecord::reprint(&alpha::PLAINS),        // RTR 250
    PrintingRecord::alternate(&alpha::PLAINS, 1),   // RTR 251
    PrintingRecord::alternate(&alpha::PLAINS, 2),   // RTR 252
    PrintingRecord::alternate(&alpha::PLAINS, 3),   // RTR 253
    PrintingRecord::alternate(&alpha::PLAINS, 4),   // RTR 254
    PrintingRecord::reprint(&alpha::ISLAND),        // RTR 255
    PrintingRecord::alternate(&alpha::ISLAND, 1),   // RTR 256
    PrintingRecord::alternate(&alpha::ISLAND, 2),   // RTR 257
    PrintingRecord::alternate(&alpha::ISLAND, 3),   // RTR 258
    PrintingRecord::alternate(&alpha::ISLAND, 4),   // RTR 259
    PrintingRecord::reprint(&alpha::SWAMP),         // RTR 260
    PrintingRecord::alternate(&alpha::SWAMP, 1),    // RTR 261
    PrintingRecord::alternate(&alpha::SWAMP, 2),    // RTR 262
    PrintingRecord::alternate(&alpha::SWAMP, 3),    // RTR 263
    PrintingRecord::alternate(&alpha::SWAMP, 4),    // RTR 264
    PrintingRecord::reprint(&alpha::MOUNTAIN),      // RTR 265
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1), // RTR 266
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2), // RTR 267
    PrintingRecord::alternate(&alpha::MOUNTAIN, 3), // RTR 268
    PrintingRecord::alternate(&alpha::MOUNTAIN, 4), // RTR 269
    PrintingRecord::reprint(&alpha::FOREST),        // RTR 270
    PrintingRecord::alternate(&alpha::FOREST, 1),   // RTR 271
    PrintingRecord::alternate(&alpha::FOREST, 2),   // RTR 272
    PrintingRecord::alternate(&alpha::FOREST, 3),   // RTR 273
    PrintingRecord::alternate(&alpha::FOREST, 4),   // RTR 274
];
