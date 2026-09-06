//! Legions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef,
    ConditionalStaticEffectDef, CostDef, DamageEventMatcherDef, DamageKindDef, DamagePreventionDef,
    DamageRecipientMatcherDef, DamageSourceMatcherDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef,
    ObjectSetPredicateDef, PlayerRelation, ResolvedEffectDurationDef, StaticApplyDef,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// LGN 1 — Akroma, Angel of Wrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKROMA_ANGEL_OF_WRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("814245de-6105-43ef-acbf-d12d304b6331"),
    "Akroma, Angel of Wrath",
    crate::card::CardArt::new("814245de-6105-43ef-acbf-d12d304b6331", "Ron Spears"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 2 — Akroma's Devoted
pub(in crate::card::sets) static AKROMA_S_DEVOTED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("798893df-e720-471d-822d-50284de23efd"),
    "Akroma's Devoted",
    CardArt::new("798893df-e720-471d-822d-50284de23efd", "Dave Dorman"),
    CardSet::Legions,
    // The same lord shape for a tribe that wanted to attack and hold the
    // ground in the same turn.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 2, 4).with_ability(
        AbilityDef::static_ability(
            "Cleric creatures have vigilance.",
            EffectDef::StaticApply {
                // "All", not "you control": it hands the keyword to the
                // opponent's as readily, which is the drawback these were
                // priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Cleric"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::vigilance() }),
            },
        ),
    ),
);

// LGN 3 — Aven Redeemer
pub(in crate::card::sets) static AVEN_REDEEMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a2fa0a3-e40f-49e4-a4fd-427e7e808afd"),
    "Aven Redeemer",
    CardArt::new("8a2fa0a3-e40f-49e4-a4fd-427e7e808afd", "Tim Hildebrandt"),
    CardSet::Legions,
    // Two points of prevention a turn, which in a format of two-power
    // creatures is a blocker that eats an attacker and walks away.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird", "Cleric"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 2 damage that would be dealt to any target this turn.",
            &[CostDef::TapSource],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )]
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LGN 4 — Aven Warhawk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_WARHAWK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("386a7062-6da8-4663-a218-75d894f7c0e0"),
    "Aven Warhawk",
    crate::card::CardArt::new("386a7062-6da8-4663-a218-75d894f7c0e0", "Glen Angus"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 5 — Beacon of Destiny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEACON_OF_DESTINY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30b1cad7-4e96-4ebe-8c99-4ed9217becf3"),
    "Beacon of Destiny",
    crate::card::CardArt::new("30b1cad7-4e96-4ebe-8c99-4ed9217becf3", "Tim Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 6 — Celestial Gatekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_GATEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b4dc1d3-53a1-411b-abf9-f5e4e80edc63"),
    "Celestial Gatekeeper",
    crate::card::CardArt::new(
        "0b4dc1d3-53a1-411b-abf9-f5e4e80edc63",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 7 — Cloudreach Cavalry
pub(in crate::card::sets) static CLOUDREACH_CAVALRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65680bda-b999-4c2a-99a8-b03287e00807"),
    "Cloudreach Cavalry",
    CardArt::new("65680bda-b999-4c2a-99a8-b03287e00807", "Kev Walker"),
    CardSet::Legions,
    // A two-mana 3/3 flier in the right deck and a bear in the wrong one,
    // which is the whole bargain Legions offered.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "As long as you control a Bird, this creature gets +2/+2 and has flying.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Bird"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    ]),
                },
            }),
        ),
    ),
);

// LGN 8 — Daru Mender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_MENDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de13fba8-3fee-4ce2-b84d-b518a99eefe0"),
    "Daru Mender",
    crate::card::CardArt::new("de13fba8-3fee-4ce2-b84d-b518a99eefe0", "Ben Thompson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 9 — Daru Sanctifier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_SANCTIFIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38b14a24-c74a-4465-9b36-8f5309e0a333"),
    "Daru Sanctifier",
    crate::card::CardArt::new("38b14a24-c74a-4465-9b36-8f5309e0a333", "Tony Szczudlo"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 10 — Daru Stinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARU_STINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff5866a4-f4c0-45bc-9b33-b77387441d34"),
    "Daru Stinger",
    crate::card::CardArt::new("ff5866a4-f4c0-45bc-9b33-b77387441d34", "Greg Staples"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 11 — Defender of the Order
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENDER_OF_THE_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("236b1c88-20a0-479e-91fb-16bb77f699fe"),
    "Defender of the Order",
    crate::card::CardArt::new("236b1c88-20a0-479e-91fb-16bb77f699fe", "Darrell Riche"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 12 — Deftblade Elite
// Audit: unsupported — Needs provoke. The requirement has to name the creature the trigger targeted, and neither MustBeBlockedBy (which takes a predicate over blockers, with no way to say "the target") nor MustBlockEachAttackerIfAble (which is every attacker) can say it.
pub(in crate::card::sets) static DEFTBLADE_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76ffbae4-7aad-493c-86a0-c6e6425da8fd"),
    "Deftblade Elite",
    crate::card::CardArt::new("76ffbae4-7aad-493c-86a0-c6e6425da8fd", "Alan Pollack"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 13 — Essence Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1346fa14-1d9f-4c6a-887d-d3a93de00743"),
    "Essence Sliver",
    crate::card::CardArt::new("1346fa14-1d9f-4c6a-887d-d3a93de00743", "Glen Angus"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 14 — Gempalm Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEMPALM_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbc66291-fdcc-4106-8875-94d2b0a70deb"),
    "Gempalm Avenger",
    crate::card::CardArt::new("dbc66291-fdcc-4106-8875-94d2b0a70deb", "Tim Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 15 — Glowrider
pub(in crate::card::sets) static GLOWRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ad94e39-0aac-46bb-a7f2-bd88c537cb9c"),
    "Glowrider",
    crate::card::CardArt::new("9ad94e39-0aac-46bb-a7f2-bd88c537cb9c", "Scott M. Fischer"),
    crate::card::CardSet::Legions,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 2, 1).with_ability(
        abilities::spell_cost_increase(
            "Noncreature spells cost {1} more to cast.",
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
            PlayerRelation::Any,
            mana_cost!("{1}"),
        ),
    ),
);

// LGN 16 — Liege of the Axe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIEGE_OF_THE_AXE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb518bf0-17ad-4bbf-b922-42ee76ffcbea"),
    "Liege of the Axe",
    crate::card::CardArt::new(
        "eb518bf0-17ad-4bbf-b922-42ee76ffcbea",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 17 — Lowland Tracker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOWLAND_TRACKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8eaaded-b18a-4614-b5b5-b4bb49a2e1b1"),
    "Lowland Tracker",
    crate::card::CardArt::new("f8eaaded-b18a-4614-b5b5-b4bb49a2e1b1", "Brian Snõddy"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 18 — Planar Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7087cb1e-f2e2-4b75-bacf-bc4153e398e3"),
    "Planar Guide",
    crate::card::CardArt::new("7087cb1e-f2e2-4b75-bacf-bc4153e398e3", "Eric Peterson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 19 — Plated Sliver
pub(in crate::card::sets) static PLATED_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82846d31-4981-4ef1-85c3-703569146a84"),
    "Plated Sliver",
    CardArt::new("82846d31-4981-4ef1-85c3-703569146a84", "Greg Staples"),
    CardSet::Legions,
    // One mana for a body and a toughness anthem, which is what makes the
    // Sliver deck survive its own curve.
    CardRules::new_creature(mana_cost!("{W}"), &["Sliver"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures get +0/+1.",
            EffectDef::StaticApply {
                // "All Sliver creatures", not "Sliver creatures you
                // control": the older Slivers pump the opponent's as
                // well, which is the drawback the cycle was priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// LGN 20 — Starlight Invoker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIGHT_INVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c66afc4-3d6d-4ce7-acfc-a4ad34aa3e99"),
    "Starlight Invoker",
    crate::card::CardArt::new("4c66afc4-3d6d-4ce7-acfc-a4ad34aa3e99", "Glen Angus"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 21 — Stoic Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOIC_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b69d619-c31b-472b-9ae8-d4503704680d"),
    "Stoic Champion",
    crate::card::CardArt::new("6b69d619-c31b-472b-9ae8-d4503704680d", "Greg Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 22 — Sunstrike Legionnaire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTRIKE_LEGIONNAIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f5d519a-9f11-4b10-97ad-edccfda639bb"),
    "Sunstrike Legionnaire",
    crate::card::CardArt::new("0f5d519a-9f11-4b10-97ad-edccfda639bb", "Mark Zug"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 23 — Swooping Talon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWOOPING_TALON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34c3d19c-d4c6-4c5c-85eb-11d55959a89c"),
    "Swooping Talon",
    crate::card::CardArt::new("34c3d19c-d4c6-4c5c-85eb-11d55959a89c", "Mark Zug"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 24 — Wall of Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b463b3e1-e314-4a65-a89e-0712f630b016"),
    "Wall of Hope",
    crate::card::CardArt::new("b463b3e1-e314-4a65-a89e-0712f630b016", "David Martin"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 25 — Ward Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARD_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e264369a-ab81-4938-9fa6-7c3e069442f4"),
    "Ward Sliver",
    crate::card::CardArt::new("e264369a-ab81-4938-9fa6-7c3e069442f4", "Pete Venters"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 26 — Whipgrass Entangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPGRASS_ENTANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0b18b09-b1ff-479d-bd1c-cb8620a34fe4"),
    "Whipgrass Entangler",
    crate::card::CardArt::new("c0b18b09-b1ff-479d-bd1c-cb8620a34fe4", "Ben Thompson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 27 — White Knight (reprint)

// LGN 28 — Windborn Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDBORN_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c45fd87-7b44-4e1a-b30f-41220b69d9e6"),
    "Windborn Muse",
    crate::card::CardArt::new("6c45fd87-7b44-4e1a-b30f-41220b69d9e6", "Adam Rex"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 29 — Wingbeat Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGBEAT_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd58d164-861d-4c80-ad2f-6283ed82faa1"),
    "Wingbeat Warrior",
    crate::card::CardArt::new("cd58d164-861d-4c80-ad2f-6283ed82faa1", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 30 — Aven Envoy
pub(in crate::card::sets) static AVEN_ENVOY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40ead30e-9f96-4fca-b619-fdc8d1b5e2e0"),
    "Aven Envoy",
    CardArt::new(
        "40ead30e-9f96-4fca-b619-fdc8d1b5e2e0",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Legions,
    // A 0/2 flier for one, which blocks the other one-drop fliers and does
    // nothing else at all.
    CardRules::new_creature(mana_cost!("{U}"), &["Bird", "Soldier"], 0, 2)
        .with_abilities(&[abilities::flying()]),
);

// LGN 31 — Cephalid Pathmage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_PATHMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88528929-4953-452a-b85e-dac15786e094"),
    "Cephalid Pathmage",
    crate::card::CardArt::new(
        "88528929-4953-452a-b85e-dac15786e094",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 32 — Chromeshell Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMESHELL_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e02a40a4-fa61-4595-810a-3796e0d71507"),
    "Chromeshell Crab",
    crate::card::CardArt::new("e02a40a4-fa61-4595-810a-3796e0d71507", "Ron Spencer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 33 — Covert Operative
pub(in crate::card::sets) static COVERT_OPERATIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dbda6799-3b55-4714-8305-713e1e198a15"),
    "Covert Operative",
    CardArt::new("dbda6799-3b55-4714-8305-713e1e198a15", "Kev Walker"),
    CardSet::Legions,
    // Five mana for three unblockable damage a turn, and a Wizard for the
    // tribe that cared.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Wizard"], 3, 2)
        .with_ability(abilities::cannot_be_blocked()),
);

// LGN 34 — Crookclaw Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROOKCLAW_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ced7275-3935-4bba-877d-81282bd171fd"),
    "Crookclaw Elder",
    crate::card::CardArt::new("8ced7275-3935-4bba-877d-81282bd171fd", "Ron Spencer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 35 — Dermoplasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DERMOPLASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf2f5dca-e01f-41e3-bb6f-a60162118c6d"),
    "Dermoplasm",
    crate::card::CardArt::new("cf2f5dca-e01f-41e3-bb6f-a60162118c6d", "John Avon"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 36 — Dreamborn Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMBORN_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e36cf11-5dfb-4593-8335-f739b7c7829c"),
    "Dreamborn Muse",
    crate::card::CardArt::new("9e36cf11-5dfb-4593-8335-f739b7c7829c", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 37 — Echo Tracer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECHO_TRACER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63390760-35a7-4b4c-8c68-5c84f90d0c58"),
    "Echo Tracer",
    crate::card::CardArt::new("63390760-35a7-4b4c-8c68-5c84f90d0c58", "Scott M. Fischer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 38 — Fugitive Wizard
pub(in crate::card::sets) static FUGITIVE_WIZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1020538-89c8-4986-9687-78ab326acb3e"),
    "Fugitive Wizard",
    CardArt::new("a1020538-89c8-4986-9687-78ab326acb3e", "Jim Nelson"),
    CardSet::Legions,
    // A vanilla 1/1 for one, printed because the Wizard tribe needed a body
    // cheap enough to be worth counting.
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1),
);

// LGN 39 — Gempalm Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEMPALM_SORCERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67bda65b-2e26-4531-9f6a-952df314c8f7"),
    "Gempalm Sorcerer",
    crate::card::CardArt::new("67bda65b-2e26-4531-9f6a-952df314c8f7", "Greg Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 40 — Glintwing Invoker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLINTWING_INVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16184709-f370-40cc-91f2-849a44ac451a"),
    "Glintwing Invoker",
    crate::card::CardArt::new("16184709-f370-40cc-91f2-849a44ac451a", "Jim Nelson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 41 — Keeneye Aven
pub(in crate::card::sets) static KEENEYE_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a355c58-cd28-4d2d-9df1-91b4196b01ef"),
    "Keeneye Aven",
    CardArt::new("1a355c58-cd28-4d2d-9df1-91b4196b01ef", "Greg Hildebrandt"),
    CardSet::Legions,
    // A flier that is never a dead draw, which is what every common in
    // the block was built to be.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Soldier"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// LGN 42 — Keeper of the Nine Gales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_THE_NINE_GALES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f75eef50-b474-44bb-8222-3e473928304a"),
    "Keeper of the Nine Gales",
    crate::card::CardArt::new("f75eef50-b474-44bb-8222-3e473928304a", "Jim Nelson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 43 — Master of the Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_OF_THE_VEIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7ce1755-9f4a-4741-b6e5-288595ec494d"),
    "Master of the Veil",
    crate::card::CardArt::new("d7ce1755-9f4a-4741-b6e5-288595ec494d", "Ron Spears"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 44 — Merchant of Secrets
pub(in crate::card::sets) static MERCHANT_OF_SECRETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1109bdd-a5ce-4e63-adee-54e43a4c4a1e"),
    "Merchant of Secrets",
    CardArt::new("d1109bdd-a5ce-4e63-adee-54e43a4c4a1e", "Greg Hildebrandt"),
    CardSet::Legions,
    // Three mana for a card and a body, which is exactly the rate a limited
    // deck wants and no constructed deck does.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// LGN 45 — Mistform Seaswift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SEASWIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2f6c73c-8162-499f-8d16-92f17c0c2bee"),
    "Mistform Seaswift",
    crate::card::CardArt::new("b2f6c73c-8162-499f-8d16-92f17c0c2bee", "Dany Orizio"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 46 — Mistform Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79a53c29-6753-4f6b-b4ee-00c1adf7e9c6"),
    "Mistform Sliver",
    crate::card::CardArt::new("79a53c29-6753-4f6b-b4ee-00c1adf7e9c6", "Ben Thompson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 47 — Mistform Ultimus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_ULTIMUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3be21c3-9b83-430b-be0a-792de9a680e3"),
    "Mistform Ultimus",
    crate::card::CardArt::new("e3be21c3-9b83-430b-be0a-792de9a680e3", "Anthony S. Waters"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 48 — Mistform Wakecaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFORM_WAKECASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e5cbfb9-9bd0-4f8b-a444-a480de4b9662"),
    "Mistform Wakecaster",
    crate::card::CardArt::new("1e5cbfb9-9bd0-4f8b-a444-a480de4b9662", "Glen Angus"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 49 — Primoc Escapee
pub(in crate::card::sets) static PRIMOC_ESCAPEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6cb3e72-bb64-4b1e-a54b-1fe4fb4ad4c9"),
    "Primoc Escapee",
    CardArt::new("e6cb3e72-bb64-4b1e-a54b-1fe4fb4ad4c9", "Tony Szczudlo"),
    CardSet::Legions,
    // Seven mana is unaffordable and two to cycle is not, so this is a
    // cantrip that occasionally wins a game.
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Bird", "Beast"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// LGN 50 — Riptide Director
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_DIRECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28d07de3-b176-4ac7-aaa7-497c06c08b55"),
    "Riptide Director",
    crate::card::CardArt::new("28d07de3-b176-4ac7-aaa7-497c06c08b55", "Scott M. Fischer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 51 — Riptide Mangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_MANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5314a802-85d6-4d7b-ae9a-ca64eec652cf"),
    "Riptide Mangler",
    crate::card::CardArt::new("5314a802-85d6-4d7b-ae9a-ca64eec652cf", "Arnie Swekel"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 52 — Shifting Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIFTING_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f68c4c2-91b5-4ffe-9dff-a6834038aa94"),
    "Shifting Sliver",
    crate::card::CardArt::new("1f68c4c2-91b5-4ffe-9dff-a6834038aa94", "Darrell Riche"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 53 — Synapse Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYNAPSE_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bf966ff-0fd0-404d-be91-5b0c21035d73"),
    "Synapse Sliver",
    crate::card::CardArt::new("8bf966ff-0fd0-404d-be91-5b0c21035d73", "Thomas M. Baxa"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 54 — Voidmage Apprentice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOIDMAGE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55924a25-e749-48f6-8ef1-1fa8376f96b1"),
    "Voidmage Apprentice",
    crate::card::CardArt::new("55924a25-e749-48f6-8ef1-1fa8376f96b1", "Jim Nelson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 55 — Wall of Deceit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_DECEIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1496d941-88fd-433e-8fae-1218316ef3a9"),
    "Wall of Deceit",
    crate::card::CardArt::new("1496d941-88fd-433e-8fae-1218316ef3a9", "John Avon"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 56 — Warped Researcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARPED_RESEARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5df94a4e-1371-4b75-a557-eeb83c23cf9d"),
    "Warped Researcher",
    crate::card::CardArt::new("5df94a4e-1371-4b75-a557-eeb83c23cf9d", "rk post"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 57 — Weaver of Lies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEAVER_OF_LIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12172d0e-0c73-4482-9f83-2c23ace9b7a0"),
    "Weaver of Lies",
    crate::card::CardArt::new("12172d0e-0c73-4482-9f83-2c23ace9b7a0", "Luca Zontini"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 58 — Willbender
// Audit: unsupported — Morph can turn this face up, but no shared trigger event observes that special action and freezes the targeted spell or ability before the target-change effect can run.
pub(in crate::card::sets) static WILLBENDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb33b35b-33c9-4d59-9ed6-7ad40ea82cb0"),
    "Willbender",
    crate::card::CardArt::new("fb33b35b-33c9-4d59-9ed6-7ad40ea82cb0", "Eric Peterson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 59 — Aphetto Exterminator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APHETTO_EXTERMINATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06be8f63-daf2-4dbe-bb07-2b246145cdab"),
    "Aphetto Exterminator",
    crate::card::CardArt::new("06be8f63-daf2-4dbe-bb07-2b246145cdab", "Scott M. Fischer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 60 — Bane of the Living
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BANE_OF_THE_LIVING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f45ebf65-77b8-41bc-b913-d864c4a00549"),
    "Bane of the Living",
    crate::card::CardArt::new("f45ebf65-77b8-41bc-b913-d864c4a00549", "Justin Sweet"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 61 — Blood Celebrant
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana; see Agent of Stromgald.
pub(in crate::card::sets) static BLOOD_CELEBRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("805de325-6f14-4a52-bb85-f9a9545d82a4"),
    "Blood Celebrant",
    CardArt::new("805de325-6f14-4a52-bb85-f9a9545d82a4", "Ben Thompson"),
    CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 62 — Corpse Harvester
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORPSE_HARVESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d09c2c8-526b-4693-bbaa-109911ce5281"),
    "Corpse Harvester",
    crate::card::CardArt::new("0d09c2c8-526b-4693-bbaa-109911ce5281", "Mark Tedin"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 63 — Crypt Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("507097eb-6b50-47ae-a545-df76b743b2bd"),
    "Crypt Sliver",
    crate::card::CardArt::new(
        "507097eb-6b50-47ae-a545-df76b743b2bd",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 64 — Dark Supplicant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_SUPPLICANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb685932-5df5-4f26-9633-b1daa8925359"),
    "Dark Supplicant",
    crate::card::CardArt::new("eb685932-5df5-4f26-9633-b1daa8925359", "Mark Zug"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 65 — Deathmark Prelate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATHMARK_PRELATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b54fb4b2-ecce-4a6c-8d76-4b5879ba836f"),
    "Deathmark Prelate",
    crate::card::CardArt::new("b54fb4b2-ecce-4a6c-8d76-4b5879ba836f", "Tony Szczudlo"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 66 — Drinker of Sorrow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRINKER_OF_SORROW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bc8758b-68cc-45ab-85d0-b870cef7dd85"),
    "Drinker of Sorrow",
    crate::card::CardArt::new("2bc8758b-68cc-45ab-85d0-b870cef7dd85", "Carl Critchlow"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 67 — Dripping Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIPPING_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdb3b483-01a8-4f54-9a3a-0d3f5aa3cd8b"),
    "Dripping Dead",
    crate::card::CardArt::new("cdb3b483-01a8-4f54-9a3a-0d3f5aa3cd8b", "Thomas M. Baxa"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 68 — Earthblighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHBLIGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("830a4048-48ac-4856-9af9-5052ec146518"),
    "Earthblighter",
    crate::card::CardArt::new(
        "830a4048-48ac-4856-9af9-5052ec146518",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 69 — Embalmed Brawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBALMED_BRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e214da0-68c0-4cf6-ba12-e2b2394909c1"),
    "Embalmed Brawler",
    crate::card::CardArt::new("2e214da0-68c0-4cf6-ba12-e2b2394909c1", "Justin Sweet"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 70 — Gempalm Polluter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEMPALM_POLLUTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e9943ac-9e3f-4ee0-b5fd-3b0fb17097d8"),
    "Gempalm Polluter",
    crate::card::CardArt::new("8e9943ac-9e3f-4ee0-b5fd-3b0fb17097d8", "Dany Orizio"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 71 — Ghastly Remains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHASTLY_REMAINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63e67323-df54-4043-a6b6-18bb89ef1f62"),
    "Ghastly Remains",
    crate::card::CardArt::new(
        "63e67323-df54-4043-a6b6-18bb89ef1f62",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 72 — Goblin Turncoat
pub(in crate::card::sets) static GOBLIN_TURNCOAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ac74e64-8831-4af2-9c6d-22c533389144"),
    "Goblin Turncoat",
    CardArt::new("2ac74e64-8831-4af2-9c6d-22c533389144", "Jim Nelson"),
    CardSet::Legions,
    // Goblins are the currency, and a deck with any of them has more
    // than it can profitably attack with.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Goblin", "Mercenary"], 2, 1).with_ability(
        abilities::regenerate_self(
            "Sacrifice a Goblin: Regenerate this creature.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
        ),
    ),
);

// LGN 73 — Graveborn Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEBORN_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa432e4e-ff23-4ad2-8d0a-403efee86f11"),
    "Graveborn Muse",
    crate::card::CardArt::new("aa432e4e-ff23-4ad2-8d0a-403efee86f11", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 74 — Havoc Demon
pub(in crate::card::sets) static HAVOC_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6477802a-349d-41e1-b050-58da0d806abf"),
    "Havoc Demon",
    CardArt::new("6477802a-349d-41e1-b050-58da0d806abf", "Thomas M. Baxa"),
    CardSet::Legions,
    // Seven mana for a flier that sweeps the board when answered, so the
    // opponent loses either way.
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Demon"], 5, 5).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, all creatures get -5/-5 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-5),
                    ValueDef::Constant(-5),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LGN 75 — Hollow Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOW_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2db779fd-0e01-417b-aee2-786db2c0b8c8"),
    "Hollow Specter",
    crate::card::CardArt::new("2db779fd-0e01-417b-aee2-786db2c0b8c8", "rk post"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 76 — Infernal Caretaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_CARETAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a028a30-6242-4d87-9501-d1826ecb69b0"),
    "Infernal Caretaker",
    crate::card::CardArt::new(
        "8a028a30-6242-4d87-9501-d1826ecb69b0",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 77 — Noxious Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9d3b33d-25b4-42b4-a93e-2a6b69832030"),
    "Noxious Ghoul",
    crate::card::CardArt::new("f9d3b33d-25b4-42b4-a93e-2a6b69832030", "Luca Zontini"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 78 — Phage the Untouchable
pub(in crate::card::sets) static PHAGE_THE_UNTOUCHABLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a410b933-99d0-4383-b54b-4839a76eb6fe"),
    "Phage the Untouchable",
    CardArt::new("a410b933-99d0-4383-b54b-4839a76eb6fe", "Ron Spears"),
    CardSet::Legions,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}{B}"), &["Avatar", "Minion"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_if(
                "When this creature enters, if you didn't cast it from your hand, you lose the game.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::Not(&TriggerConditionDef::SourceCastFrom(ZoneKind::Hand)),
                EffectDef::LoseTheGame {
                    player: EffectRecipientDef::Controller,
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a creature, destroy that creature. It can't be regenerated.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Combat,
                    source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                    recipient: DamageRecipientMatcherDef::MatchingObject(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ),
                }),
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::Destroy {
                        object: EffectRecipientDef::DamagedObject,
                        then: None,
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, that player loses the game.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::LoseTheGame {
                    player: EffectRecipientDef::EventPlayer,
                },
            ),
        ]),
);

// LGN 79 — Scion of Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCION_OF_DARKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("497c2629-1263-48a4-9c31-7f052808b2b8"),
    "Scion of Darkness",
    crate::card::CardArt::new("497c2629-1263-48a4-9c31-7f052808b2b8", "Mark Zug"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 80 — Skinthinner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKINTHINNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89b8c392-da68-4894-b6e8-eb430141a0d7"),
    "Skinthinner",
    crate::card::CardArt::new("89b8c392-da68-4894-b6e8-eb430141a0d7", "Dany Orizio"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 81 — Smokespew Invoker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOKESPEW_INVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fea393a4-58c8-4a42-bd95-a3312504f2e2"),
    "Smokespew Invoker",
    crate::card::CardArt::new("fea393a4-58c8-4a42-bd95-a3312504f2e2", "Thomas M. Baxa"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 82 — Sootfeather Flock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOOTFEATHER_FLOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("216a2ccc-8847-452b-b030-27d8506675bd"),
    "Sootfeather Flock",
    crate::card::CardArt::new("216a2ccc-8847-452b-b030-27d8506675bd", "David Martin"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 83 — Spectral Sliver
pub(in crate::card::sets) static SPECTRAL_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bec97e3c-7b75-4abb-a50e-86bc8cc3bf06"),
    "Spectral Sliver",
    CardArt::new("bec97e3c-7b75-4abb-a50e-86bc8cc3bf06", "Pete Venters"),
    CardSet::Legions,
    // Both halves on one Sliver, which is what Legions charged a black card
    // for.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Sliver", "Spirit"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures have \"{2}: This creature gets +1/+1 until end of turn.\"",
            EffectDef::StaticApply {
                // "All Sliver creatures", so the opponent's get it too.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                // "This creature" inside the granted ability is
                // whichever Sliver has it, which is that ability's own
                // source rather than this one.
                effect: AppliedEffectDef::add_ability(
                    &const {
                        AbilityDef::activated(
                            "{2}: This creature gets +1/+1 until end of turn.",
                            &[CostDef::Mana(mana_cost!("{2}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )
                    },
                ),
            },
        ),
    ),
);

// LGN 84 — Toxin Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOXIN_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c04ab6b6-27ee-4c93-a87c-cbc3743f4faf"),
    "Toxin Sliver",
    crate::card::CardArt::new("c04ab6b6-27ee-4c93-a87c-cbc3743f4faf", "Lars Grant-West"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 85 — Vile Deacon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILE_DEACON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2641bd5-c845-47a1-8038-bb28b06f896e"),
    "Vile Deacon",
    crate::card::CardArt::new("b2641bd5-c845-47a1-8038-bb28b06f896e", "Matthew D. Wilson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 86 — Withered Wretch
pub(in crate::card::sets) static WITHERED_WRETCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("691135e8-1c41-41c4-9426-48bffe23bfd8"),
    "Withered Wretch",
    CardArt::new("b8a82948-503f-4ad4-9e3c-c080c16afd63", "Tim Hildebrandt"),
    CardSet::Legions,
    // A one-mana graveyard answer as often as the mana lasts, which is why
    // it beat every one-shot printed alongside it.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Zombie", "Cleric"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}: Exile target card from a graveyard.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// LGN 87 — Zombie Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_BRUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b37db470-3aef-4fc4-98ce-63b5fb2546f6"),
    "Zombie Brute",
    crate::card::CardArt::new("b37db470-3aef-4fc4-98ce-63b5fb2546f6", "Greg Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 88 — Blade Sliver
pub(in crate::card::sets) static BLADE_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8d6f7a6-7b6a-44f4-be04-7c02806b9f09"),
    "Blade Sliver",
    CardArt::new("a8d6f7a6-7b6a-44f4-be04-7c02806b9f09", "David Martin"),
    CardSet::Legions,
    // The aggressive half of the same anthem cycle.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Sliver"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "All Sliver creatures get +1/+0.",
            EffectDef::StaticApply {
                // "All Sliver creatures", not "Sliver creatures you
                // control": the older Slivers pump the opponent's as
                // well, which is the drawback the cycle was priced on.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Sliver"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// LGN 89 — Bloodstoke Howler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODSTOKE_HOWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("743779d4-fee8-4b8d-a5ac-27f355e006e5"),
    "Bloodstoke Howler",
    crate::card::CardArt::new(
        "743779d4-fee8-4b8d-a5ac-27f355e006e5",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 90 — Clickslither
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLICKSLITHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf1c3f62-f275-46e1-8c26-c219683effb1"),
    "Clickslither",
    crate::card::CardArt::new("bf1c3f62-f275-46e1-8c26-c219683effb1", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 91 — Crested Craghorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRESTED_CRAGHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aadb40c8-3d54-4705-82dc-54e8d6e315d5"),
    "Crested Craghorn",
    crate::card::CardArt::new("aadb40c8-3d54-4705-82dc-54e8d6e315d5", "Matt Cavotta"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 92 — Flamewave Invoker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMEWAVE_INVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13a68534-2d9a-47e9-9d2a-cb6df4362aa9"),
    "Flamewave Invoker",
    crate::card::CardArt::new("13a68534-2d9a-47e9-9d2a-cb6df4362aa9", "Dave Dorman"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 93 — Frenetic Raptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRENETIC_RAPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f6bc3c0-2d6e-4a09-84c4-b26a352186bb"),
    "Frenetic Raptor",
    crate::card::CardArt::new("8f6bc3c0-2d6e-4a09-84c4-b26a352186bb", "Daren Bader"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 94 — Gempalm Incinerator
pub(in crate::card::sets) static GEMPALM_INCINERATOR: CardRecord = CardRecord::new_with_legacy_id(
    2026,
    "Gempalm Incinerator",
    CardArt::new("2687c311-fd0c-4fe0-bce8-e3f412216796", "Luca Zontini"),
    CardSet::Legions,
    // The card is played as removal far more often than as a creature, and
    // the Incinerator itself is not on the battlefield when it counts -- it
    // is in the graveyard, so it never counts itself.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 2, 1).with_abilities(&[
        abilities::cycling(
            "Cycling {1}{R} ({1}{R}, Discard this card: Draw a card.)",
            mana_cost!("{1}{R}"),
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, you may have it deal X damage to target creature, where X is the number of Goblins on the battlefield.",
            TriggerEventDef::Cycled,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    // Every Goblin on the battlefield, whoever controls it -- the count is of
                    // the board, not of your side of it.
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Goblin"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                },
            },
        ),
    ]),
);

// LGN 95 — Goblin Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57ec836f-6dcf-45f9-8e95-487762742a1e"),
    "Goblin Assassin",
    crate::card::CardArt::new("57ec836f-6dcf-45f9-8e95-487762742a1e", "Dave Dorman"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 96 — Goblin Clearcutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CLEARCUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e07c0cae-852c-444c-8994-68a6d81b4cd4"),
    "Goblin Clearcutter",
    crate::card::CardArt::new("e07c0cae-852c-444c-8994-68a6d81b4cd4", "Eric Peterson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 97 — Goblin Dynamo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_DYNAMO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9462cb4e-a38c-4a41-bad2-4ea3b22b0edb"),
    "Goblin Dynamo",
    crate::card::CardArt::new("9462cb4e-a38c-4a41-bad2-4ea3b22b0edb", "Ron Spencer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 98 — Goblin Firebug
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_FIREBUG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2370d319-d1d2-4bca-9275-ff72fb400709"),
    "Goblin Firebug",
    crate::card::CardArt::new(
        "2370d319-d1d2-4bca-9275-ff72fb400709",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 99 — Goblin Goon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c77cac8-fe95-4925-a815-8c514cc41b22"),
    "Goblin Goon",
    crate::card::CardArt::new("6c77cac8-fe95-4925-a815-8c514cc41b22", "Greg Staples"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 100 — Goblin Grappler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GRAPPLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c948872-295c-41b9-8094-db7db7578b0d"),
    "Goblin Grappler",
    crate::card::CardArt::new(
        "5c948872-295c-41b9-8094-db7db7578b0d",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 101 — Goblin Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_LOOKOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23bbe84a-8857-467a-a4a1-e57086cc9501"),
    "Goblin Lookout",
    crate::card::CardArt::new("23bbe84a-8857-467a-a4a1-e57086cc9501", "Jim Nelson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 102 — Hunter Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTER_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca9aea1a-6f50-4f66-9f36-2e214dce41b4"),
    "Hunter Sliver",
    crate::card::CardArt::new("ca9aea1a-6f50-4f66-9f36-2e214dce41b4", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 103 — Imperial Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPERIAL_HELLKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fc3c5f3-f71b-4a1e-bd90-365d23889925"),
    "Imperial Hellkite",
    crate::card::CardArt::new("1fc3c5f3-f71b-4a1e-bd90-365d23889925", "Matt Cavotta"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 104 — Kilnmouth Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KILNMOUTH_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("effe13c3-3c8b-4faa-bdd4-491039bfa82b"),
    "Kilnmouth Dragon",
    crate::card::CardArt::new("effe13c3-3c8b-4faa-bdd4-491039bfa82b", "Carl Critchlow"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 105 — Lavaborn Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVABORN_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cbc94fb-9e3f-4075-bb6a-8f04862dc585"),
    "Lavaborn Muse",
    crate::card::CardArt::new("4cbc94fb-9e3f-4075-bb6a-8f04862dc585", "Brian Snõddy"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 106 — Macetail Hystrodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MACETAIL_HYSTRODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8451ab3f-5d61-4f35-ab70-5a5060caf53d"),
    "Macetail Hystrodon",
    crate::card::CardArt::new("8451ab3f-5d61-4f35-ab70-5a5060caf53d", "Daren Bader"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 107 — Magma Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGMA_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9091d908-456f-4127-857d-b22fdb4f2fd9"),
    "Magma Sliver",
    crate::card::CardArt::new("9091d908-456f-4127-857d-b22fdb4f2fd9", "Wayne England"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 108 — Ridgetop Raptor
pub(in crate::card::sets) static RIDGETOP_RAPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1013cbc4-09f4-484f-b328-9f7403225149"),
    "Ridgetop Raptor",
    CardArt::new("1013cbc4-09f4-484f-b328-9f7403225149", "Daren Bader"),
    CardSet::Legions,
    // Double strike on a 2/1, which trades up against anything without
    // first strike and dies to everything.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Dinosaur", "Beast"], 2, 1)
        .with_ability(abilities::double_strike()),
);

// LGN 109 — Rockshard Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKSHARD_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d6343c0-3fb5-4bac-bea7-cba36498cd69"),
    "Rockshard Elemental",
    crate::card::CardArt::new("2d6343c0-3fb5-4bac-bea7-cba36498cd69", "Anthony S. Waters"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 110 — Shaleskin Plower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHALESKIN_PLOWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42658b33-9a12-403b-bc7d-807fbe1f1a36"),
    "Shaleskin Plower",
    crate::card::CardArt::new("42658b33-9a12-403b-bc7d-807fbe1f1a36", "Daren Bader"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 111 — Skirk Alarmist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_ALARMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd1c1d41-8666-4c1d-9498-0e259472958d"),
    "Skirk Alarmist",
    crate::card::CardArt::new("fd1c1d41-8666-4c1d-9498-0e259472958d", "Justin Sweet"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 112 — Skirk Drill Sergeant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_DRILL_SERGEANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("359b2d1a-4027-46d9-b780-bcac8d60ecdb"),
    "Skirk Drill Sergeant",
    crate::card::CardArt::new(
        "359b2d1a-4027-46d9-b780-bcac8d60ecdb",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 113 — Skirk Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRK_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24d9c8ea-bdb5-4d9a-9d1e-218540c4ad2c"),
    "Skirk Marauder",
    crate::card::CardArt::new("bbd2ff12-c6f7-4986-801f-225ad6f59278", "Pete Venters"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 114 — Skirk Outrider
pub(in crate::card::sets) static SKIRK_OUTRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("416de0f4-1540-4286-a1ac-4f57301c54e9"),
    "Skirk Outrider",
    CardArt::new("416de0f4-1540-4286-a1ac-4f57301c54e9", "Greg Staples"),
    CardSet::Legions,
    // Goblins and Beasts were not the same deck, which is exactly why this
    // one asks you to build it that way.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "As long as you control a Beast, this creature gets +2/+2 and has trample.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Beast"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&const { abilities::trample() }),
                    ]),
                },
            }),
        ),
    ),
);

// LGN 115 — Unstable Hulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSTABLE_HULK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("889cfde2-42fa-4278-ae4e-7e4dd993cda8"),
    "Unstable Hulk",
    crate::card::CardArt::new("889cfde2-42fa-4278-ae4e-7e4dd993cda8", "Ron Spencer"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 116 — Warbreak Trumpeter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARBREAK_TRUMPETER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc942957-1067-428c-8ee1-01f9e260efe1"),
    "Warbreak Trumpeter",
    crate::card::CardArt::new("fc942957-1067-428c-8ee1-01f9e260efe1", "Dany Orizio"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 117 — Berserk Murlodont
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BERSERK_MURLODONT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("499c4674-dd9f-4848-8447-721f842a0213"),
    "Berserk Murlodont",
    crate::card::CardArt::new("499c4674-dd9f-4848-8447-721f842a0213", "Arnie Swekel"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 118 — Branchsnap Lorian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRANCHSNAP_LORIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52118ff1-ad76-4b97-9fdc-6adfe80140f8"),
    "Branchsnap Lorian",
    crate::card::CardArt::new("52118ff1-ad76-4b97-9fdc-6adfe80140f8", "Heather Hudson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 119 — Brontotherium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRONTOTHERIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a171f5e2-ed3d-4675-a4fc-953ebb907aa0"),
    "Brontotherium",
    crate::card::CardArt::new("a171f5e2-ed3d-4675-a4fc-953ebb907aa0", "Carl Critchlow"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 120 — Brood Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROOD_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33803c12-1d78-49fe-a3a3-7f47c60a96b6"),
    "Brood Sliver",
    crate::card::CardArt::new("33803c12-1d78-49fe-a3a3-7f47c60a96b6", "Ron Spears"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 121 — Caller of the Claw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALLER_OF_THE_CLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a073459e-1f00-47e0-a1b3-d30203aa35d1"),
    "Caller of the Claw",
    crate::card::CardArt::new("a073459e-1f00-47e0-a1b3-d30203aa35d1", "Matt Cavotta"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 122 — Canopy Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ccdc9d7-71b5-4304-8d19-a63952e17a6b"),
    "Canopy Crawler",
    crate::card::CardArt::new("0ccdc9d7-71b5-4304-8d19-a63952e17a6b", "Anthony S. Waters"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 123 — Defiant Elf
pub(in crate::card::sets) static DEFIANT_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b7a0b8f-6942-40b0-8efc-234ae77855b4"),
    "Defiant Elf",
    CardArt::new("3b7a0b8f-6942-40b0-8efc-234ae77855b4", "Pete Venters"),
    CardSet::Legions,
    // Trample on a 1/1 is nearly nothing, which is the joke: it gets through
    // for one exactly when nothing is blocking it anyway.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1)
        .with_abilities(&[abilities::trample()]),
);

// LGN 124 — Elvish Soultiller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_SOULTILLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e2c8de5-bc80-4fad-af09-6d0a639f6e18"),
    "Elvish Soultiller",
    crate::card::CardArt::new("9e2c8de5-bc80-4fad-af09-6d0a639f6e18", "Ron Spears"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 125 — Enormous Baloth
pub(in crate::card::sets) static ENORMOUS_BALOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cebfb5a6-9052-47be-b931-834b5064df31"),
    "Enormous Baloth",
    CardArt::new("cebfb5a6-9052-47be-b931-834b5064df31", "Mark Tedin"),
    CardSet::Legions,
    // Seven mana for a 7/7, the plain end of the Beast curve this block was
    // built around.
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Beast"], 7, 7),
);

// LGN 126 — Feral Throwback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERAL_THROWBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5111a9a3-a92d-4677-8974-20800256dd4f"),
    "Feral Throwback",
    crate::card::CardArt::new("49e0c5e5-b293-419e-aac5-3b81af4b6498", "Carl Critchlow"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 127 — Gempalm Strider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEMPALM_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f93d89f5-3e77-4dc0-935b-e6f6a3e968d2"),
    "Gempalm Strider",
    crate::card::CardArt::new("f93d89f5-3e77-4dc0-935b-e6f6a3e968d2", "Tim Hildebrandt"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 128 — Glowering Rogon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOWERING_ROGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("974b0881-bd26-4074-93dd-a1e3600347c4"),
    "Glowering Rogon",
    crate::card::CardArt::new("974b0881-bd26-4074-93dd-a1e3600347c4", "Kev Walker"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 129 — Hundroog
pub(in crate::card::sets) static HUNDROOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f525c356-88ca-4e2e-8f06-663be101e34f"),
    "Hundroog",
    CardArt::new("f525c356-88ca-4e2e-8f06-663be101e34f", "Wayne England"),
    CardSet::Legions,
    // Seven mana is unaffordable and three to cycle is not, so it is
    // really a cantrip with an emergency body attached.
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Beast"], 4, 7).with_ability(
        abilities::cycling(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            mana_cost!("{3}"),
        ),
    ),
);

// LGN 130 — Krosan Cloudscraper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_CLOUDSCRAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51ef4cda-e55b-45a8-9c02-4e77e5b15a9e"),
    "Krosan Cloudscraper",
    crate::card::CardArt::new("51ef4cda-e55b-45a8-9c02-4e77e5b15a9e", "Ron Spears"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 131 — Krosan Vorine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_VORINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7d1c6c6-16b3-4a52-aeda-683b1aeb0e7f"),
    "Krosan Vorine",
    crate::card::CardArt::new("b7d1c6c6-16b3-4a52-aeda-683b1aeb0e7f", "Carl Critchlow"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 132 — Nantuko Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_VIGILANTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7474849-a6b4-4f3b-a836-37b88c26047b"),
    "Nantuko Vigilante",
    crate::card::CardArt::new(
        "e7474849-a6b4-4f3b-a836-37b88c26047b",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 133 — Needleshot Gourna
pub(in crate::card::sets) static NEEDLESHOT_GOURNA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9b1628d-aacd-4e19-9ebb-bcd9b2842c91"),
    "Needleshot Gourna",
    CardArt::new(
        "f9b1628d-aacd-4e19-9ebb-bcd9b2842c91",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legions,
    // A 3/6 reach wall that happens to be a Beast, which is what Legions
    // cared about.
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 3, 6)
        .with_ability(abilities::reach()),
);

// LGN 134 — Patron of the Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRON_OF_THE_WILD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f7a0810-3970-454f-8381-700d6c6aefdc"),
    "Patron of the Wild",
    crate::card::CardArt::new("7f7a0810-3970-454f-8381-700d6c6aefdc", "Dave Dorman"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 135 — Primal Whisperer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_WHISPERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c777432f-7965-4ad8-8d53-93919ae767d4"),
    "Primal Whisperer",
    crate::card::CardArt::new("c777432f-7965-4ad8-8d53-93919ae767d4", "Greg Staples"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 136 — Quick Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICK_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30a60b2d-aeeb-4dbf-bf1a-20a274fe323f"),
    "Quick Sliver",
    crate::card::CardArt::new("30a60b2d-aeeb-4dbf-bf1a-20a274fe323f", "John Avon"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 137 — Root Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdf5a106-5fb7-40e4-82a7-db559302a923"),
    "Root Sliver",
    crate::card::CardArt::new("fdf5a106-5fb7-40e4-82a7-db559302a923", "Matt Thompson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 138 — Seedborn Muse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDBORN_MUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35b13321-e429-4497-aef2-93a9df421d38"),
    "Seedborn Muse",
    crate::card::CardArt::new("35b13321-e429-4497-aef2-93a9df421d38", "Adam Rex"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 139 — Stonewood Invoker
pub(in crate::card::sets) static STONEWOOD_INVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94d0235d-7176-44a2-8e95-eb231f4af441"),
    "Stonewood Invoker",
    CardArt::new("94d0235d-7176-44a2-8e95-eb231f4af441", "Eric Peterson"),
    CardSet::Legions,
    // A two-drop with a late-game button: eight mana turns it into a 7/7,
    // which is what an Invoker is for.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Mutant"], 2, 2).with_ability(
        AbilityDef::activated(
            "{7}{G}: This creature gets +5/+5 until end of turn.",
            &[CostDef::Mana(mana_cost!("{7}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(5),
                    ValueDef::Constant(5),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LGN 140 — Timberwatch Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMBERWATCH_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("045ae4ec-07f2-4098-a2d9-4bfcbd0273b2"),
    "Timberwatch Elf",
    crate::card::CardArt::new("045ae4ec-07f2-4098-a2d9-4bfcbd0273b2", "Dave Dorman"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 141 — Totem Speaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOTEM_SPEAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce12115b-2667-47f7-bd24-17c982a4f79a"),
    "Totem Speaker",
    crate::card::CardArt::new("ce12115b-2667-47f7-bd24-17c982a4f79a", "Darrell Riche"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 142 — Tribal Forcemage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_FORCEMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("104735d7-6cea-4d4a-8cc8-e1934883da97"),
    "Tribal Forcemage",
    crate::card::CardArt::new("104735d7-6cea-4d4a-8cc8-e1934883da97", "Greg Staples"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 143 — Vexing Beetle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEXING_BEETLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d599d35f-1b73-498b-9a21-831c908a95d8"),
    "Vexing Beetle",
    crate::card::CardArt::new("d599d35f-1b73-498b-9a21-831c908a95d8", "Matt Thompson"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 144 — Wirewood Channeler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_CHANNELER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36e5579e-dab7-49db-a141-a5bc5b5aee90"),
    "Wirewood Channeler",
    crate::card::CardArt::new("36e5579e-dab7-49db-a141-a5bc5b5aee90", "Alan Pollack"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 145 — Wirewood Hivemaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIREWOOD_HIVEMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea55b4fc-366f-4906-9eaa-9085f6a22612"),
    "Wirewood Hivemaster",
    crate::card::CardArt::new("ea55b4fc-366f-4906-9eaa-9085f6a22612", "Darrell Riche"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKROMA_ANGEL_OF_WRATH,
    &AKROMA_S_DEVOTED,
    &AVEN_REDEEMER,
    &AVEN_WARHAWK,
    &BEACON_OF_DESTINY,
    &CELESTIAL_GATEKEEPER,
    &CLOUDREACH_CAVALRY,
    &DARU_MENDER,
    &DARU_SANCTIFIER,
    &DARU_STINGER,
    &DEFENDER_OF_THE_ORDER,
    &DEFTBLADE_ELITE,
    &ESSENCE_SLIVER,
    &GEMPALM_AVENGER,
    &GLOWRIDER,
    &LIEGE_OF_THE_AXE,
    &LOWLAND_TRACKER,
    &PLANAR_GUIDE,
    &PLATED_SLIVER,
    &STARLIGHT_INVOKER,
    &STOIC_CHAMPION,
    &SUNSTRIKE_LEGIONNAIRE,
    &SWOOPING_TALON,
    &WALL_OF_HOPE,
    &WARD_SLIVER,
    &WHIPGRASS_ENTANGLER,
    &WINDBORN_MUSE,
    &WINGBEAT_WARRIOR,
    &AVEN_ENVOY,
    &CEPHALID_PATHMAGE,
    &CHROMESHELL_CRAB,
    &COVERT_OPERATIVE,
    &CROOKCLAW_ELDER,
    &DERMOPLASM,
    &DREAMBORN_MUSE,
    &ECHO_TRACER,
    &FUGITIVE_WIZARD,
    &GEMPALM_SORCERER,
    &GLINTWING_INVOKER,
    &KEENEYE_AVEN,
    &KEEPER_OF_THE_NINE_GALES,
    &MASTER_OF_THE_VEIL,
    &MERCHANT_OF_SECRETS,
    &MISTFORM_SEASWIFT,
    &MISTFORM_SLIVER,
    &MISTFORM_ULTIMUS,
    &MISTFORM_WAKECASTER,
    &PRIMOC_ESCAPEE,
    &RIPTIDE_DIRECTOR,
    &RIPTIDE_MANGLER,
    &SHIFTING_SLIVER,
    &SYNAPSE_SLIVER,
    &VOIDMAGE_APPRENTICE,
    &WALL_OF_DECEIT,
    &WARPED_RESEARCHER,
    &WEAVER_OF_LIES,
    &WILLBENDER,
    &APHETTO_EXTERMINATOR,
    &BANE_OF_THE_LIVING,
    &BLOOD_CELEBRANT,
    &CORPSE_HARVESTER,
    &CRYPT_SLIVER,
    &DARK_SUPPLICANT,
    &DEATHMARK_PRELATE,
    &DRINKER_OF_SORROW,
    &DRIPPING_DEAD,
    &EARTHBLIGHTER,
    &EMBALMED_BRAWLER,
    &GEMPALM_POLLUTER,
    &GHASTLY_REMAINS,
    &GOBLIN_TURNCOAT,
    &GRAVEBORN_MUSE,
    &HAVOC_DEMON,
    &HOLLOW_SPECTER,
    &INFERNAL_CARETAKER,
    &NOXIOUS_GHOUL,
    &PHAGE_THE_UNTOUCHABLE,
    &SCION_OF_DARKNESS,
    &SKINTHINNER,
    &SMOKESPEW_INVOKER,
    &SOOTFEATHER_FLOCK,
    &SPECTRAL_SLIVER,
    &TOXIN_SLIVER,
    &VILE_DEACON,
    &WITHERED_WRETCH,
    &ZOMBIE_BRUTE,
    &BLADE_SLIVER,
    &BLOODSTOKE_HOWLER,
    &CLICKSLITHER,
    &CRESTED_CRAGHORN,
    &FLAMEWAVE_INVOKER,
    &FRENETIC_RAPTOR,
    &GEMPALM_INCINERATOR,
    &GOBLIN_ASSASSIN,
    &GOBLIN_CLEARCUTTER,
    &GOBLIN_DYNAMO,
    &GOBLIN_FIREBUG,
    &GOBLIN_GOON,
    &GOBLIN_GRAPPLER,
    &GOBLIN_LOOKOUT,
    &HUNTER_SLIVER,
    &IMPERIAL_HELLKITE,
    &KILNMOUTH_DRAGON,
    &LAVABORN_MUSE,
    &MACETAIL_HYSTRODON,
    &MAGMA_SLIVER,
    &RIDGETOP_RAPTOR,
    &ROCKSHARD_ELEMENTAL,
    &SHALESKIN_PLOWER,
    &SKIRK_ALARMIST,
    &SKIRK_DRILL_SERGEANT,
    &SKIRK_MARAUDER,
    &SKIRK_OUTRIDER,
    &UNSTABLE_HULK,
    &WARBREAK_TRUMPETER,
    &BERSERK_MURLODONT,
    &BRANCHSNAP_LORIAN,
    &BRONTOTHERIUM,
    &BROOD_SLIVER,
    &CALLER_OF_THE_CLAW,
    &CANOPY_CRAWLER,
    &DEFIANT_ELF,
    &ELVISH_SOULTILLER,
    &ENORMOUS_BALOTH,
    &FERAL_THROWBACK,
    &GEMPALM_STRIDER,
    &GLOWERING_ROGON,
    &HUNDROOG,
    &KROSAN_CLOUDSCRAPER,
    &KROSAN_VORINE,
    &NANTUKO_VIGILANTE,
    &NEEDLESHOT_GOURNA,
    &PATRON_OF_THE_WILD,
    &PRIMAL_WHISPERER,
    &QUICK_SLIVER,
    &ROOT_SLIVER,
    &SEEDBORN_MUSE,
    &STONEWOOD_INVOKER,
    &TIMBERWATCH_ELF,
    &TOTEM_SPEAKER,
    &TRIBAL_FORCEMAGE,
    &VEXING_BEETLE,
    &WIREWOOD_CHANNELER,
    &WIREWOOD_HIVEMASTER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_lea::WHITE_KNIGHT), // LGN 27
];
