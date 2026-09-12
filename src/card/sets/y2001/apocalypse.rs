//! Apocalypse cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::CardSupertype;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AdditionalCostValueDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::AttackEventMatcherDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::TargetChooserDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "APC",
    slug: "apocalypse",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// APC 1 — Angelfire Crusader
pub(in crate::card::sets) static ANGELFIRE_CRUSADER: CardRecord = CardRecord::new(
    "Angelfire Crusader",
    "a7af8350-9a51-437c-a55e-19f3e07acfa9",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier", "Knight"], 2, 3)
        .with_ability(AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// APC 2 — Coalition Flag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COALITION_FLAG: CardRecord = CardRecord::new(
    "Coalition Flag",
    "0e417461-a230-4548-bcc1-71377487f21b",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// APC 3 — Coalition Honor Guard
// Audit: unsupported — Needs a rule that constrains how an opponent chooses targets. AppliedRuleDef can forbid actions and require blocks, but nothing requires a target to be chosen, which is the whole of what a Flagbearer does.
pub(in crate::card::sets) static COALITION_HONOR_GUARD: CardRecord = CardRecord::new(
    "Coalition Honor Guard",
    "c5b7be3e-b4af-46d4-bcc6-b44c651f2012",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// APC 4 — Dega Disciple
pub(in crate::card::sets) static DEGA_DISCIPLE: CardRecord = CardRecord::new(
    "Dega Disciple",
    "fb9cd7d9-8aad-4607-890c-9c8efe016a92",
    "Alan Pollack",
    // A white body that only pays off with the other two Dega colors on hand.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}, {T}: Target creature gets -2/-0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}")), CostDef::TapSource],
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
        AbilityDef::activated_with_targets(
            "{R}, {T}: Target creature gets +2/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
    ]),
);

// APC 5 — Dega Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEGA_SANCTUARY: CardRecord = CardRecord::new(
    "Dega Sanctuary",
    "b7ddfdb5-3981-4954-af5f-2459d22ec575",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// APC 6 — Degavolver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEGAVOLVER: CardRecord = CardRecord::new(
    "Degavolver",
    "36a52c3a-2f58-4b4d-b3c6-f9a08e25c7de",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// APC 7 — Diversionary Tactics
pub(in crate::card::sets) static DIVERSIONARY_TACTICS: CardRecord = CardRecord::new(
    "Diversionary Tactics",
    "1e5061e4-a76d-4a7c-b196-96c81f94e0e5",
    "Jerry Tiritilli",
    // Two of your creatures to tap one of theirs, and the tapped pair are
    // not attacking either -- so it defends a board it has already emptied.
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "Tap two untapped creatures you control: Tap target creature.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 2,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// APC 8 — Divine Light
// Audit: unsupported — Needs a resolved damage-prevention recipient matcher for “creatures you control”. `MatchingObject` carries no controller and is rejected outside the shared runtime boundary when its predicate names one, and `PlayerAndCreaturesControlledBy` also covers the player, which this does not.
pub(in crate::card::sets) static DIVINE_LIGHT: CardRecord = CardRecord::new(
    "Divine Light",
    "8f596ce1-b754-4e34-98e3-e1ddda2fd9b0",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// APC 9 — Enlistment Officer
pub(in crate::card::sets) static ENLISTMENT_OFFICER: CardRecord = CardRecord::new(
    "Enlistment Officer",
    "38138bb4-25ea-4aaf-8b1c-e9e60678fc6b",
    "Wayne England",
// The white half of the Ringleader cycle: the same refill, traded from
    // haste onto a first-striking body.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        abilities::first_strike(),
        abilities::enters_trigger(
            "When this creature enters, reveal the top four cards of your library. Put all Soldier cards revealed this way into your hand and the rest on the bottom of your library in any order.",
            abilities::reveal_top_cards_put_matching_in_hand_rest_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")),
            ),
        ),
    ]),
);

// APC 10 — False Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_DAWN: CardRecord = CardRecord::new(
    "False Dawn",
    "1695e0ba-005a-4652-aea7-e1d1f9ff5d66",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// APC 11 — Gerrard Capashen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_CAPASHEN: CardRecord = CardRecord::new(
    "Gerrard Capashen",
    "ccca800f-e850-4bec-95d0-70280b51b7a7",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// APC 12 — Haunted Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTED_ANGEL: CardRecord = CardRecord::new(
    "Haunted Angel",
    "78d2d11b-12e4-4810-a32d-8f1cdda3ec49",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// APC 13 — Helionaut
pub(in crate::card::sets) static HELIONAUT: CardRecord = CardRecord::new(
    "Helionaut",
    "3a4d395e-d7d6-4e93-9761-b0bae63b7b1c",
    "Franz Vohwinkel",
    // An evasive body that doubles as filtering for the three-color decks the
    // set was built around, at the cost of not attacking that turn.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// APC 14 — Manacles of Decay
pub(in crate::card::sets) static MANACLES_OF_DECAY: CardRecord = CardRecord::new(
    "Manacles of Decay",
    "f3da5010-78b6-426f-aeb4-73c21d2af581",
    "Gary Ruddell",
    // A pacifism whose two activations are the other Dega colours: the Aura
    // holds the creature still and the mana finishes it off.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                },
            ),
            AbilityDef::activated(
                "{B}: Enchanted creature gets -1/-1 until end of turn.",
                &[CostDef::Mana(mana_cost!("{B}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{R}: Enchanted creature can't block this turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// APC 15 — Orim's Thunder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_THUNDER: CardRecord = CardRecord::new(
    "Orim's Thunder",
    "d00bf192-4baf-46ba-947b-a22d07635b04",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// APC 16 — Shield of Duty and Reason
pub(in crate::card::sets) static SHIELD_OF_DUTY_AND_REASON: CardRecord = CardRecord::new(
    "Shield of Duty and Reason",
    "4ddf4ee0-75d6-48a5-955c-97faf73b899f",
    "Anthony S. Waters",
    // Unlike the Alpha Wards this grants two colors and neither is its own, so
    // it needs no exception keeping the Aura attached.
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has protection from green and from blue.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::Green,
                        )),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::Blue,
                        )),
                    },
                ]),
            ),
        ]),
);

// APC 17 — Spectral Lynx
pub(in crate::card::sets) static SPECTRAL_LYNX: CardRecord = CardRecord::new(
    "Spectral Lynx",
    "13099abe-721e-42b4-9666-9e6b5f1d75c9",
    "Heather Hudson",
    // Protection dodges the green removal of its era, and the black activation
    // means the same body survives everything else for one mana at a time.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Spirit"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Green),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// APC 18 — Standard Bearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STANDARD_BEARER: CardRecord = CardRecord::new(
    "Standard Bearer",
    "e0f8e16a-55f0-4147-a01a-dba7938f31c4",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// APC 19 — Ceta Disciple
pub(in crate::card::sets) static CETA_DISCIPLE: CardRecord = CardRecord::new(
    "Ceta Disciple",
    "b1c40c26-3b82-4f72-acb5-85fbdd51665a",
    "Greg Hildebrandt",
    // The blue member of the Disciple cycle: both activations need one of the
    // other two Ceta colors, and each one costs the whole turn's tap.
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{R}, {T}: Target creature gets +2/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
        AbilityDef::activated_mana(
            "{G}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// APC 20 — Ceta Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CETA_SANCTUARY: CardRecord = CardRecord::new(
    "Ceta Sanctuary",
    "32cec6f3-295a-45e3-8466-e35fb043a596",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// APC 21 — Cetavolver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CETAVOLVER: CardRecord = CardRecord::new(
    "Cetavolver",
    "69063cc2-4f6e-4cce-bb09-ccd57b69b993",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// APC 22 — Coastal Drake
pub(in crate::card::sets) static COASTAL_DRAKE: CardRecord = CardRecord::new(
    "Coastal Drake",
    "f87aaa74-26c6-4057-84b9-a007383684a5",
    "John Gallagher",
    // The bounce only reads Kavu, which was the set's answer to its own
    // green-red creature theme rather than a general tempo tool.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{1}{U}, {T}: Return target Kavu to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kavu")),
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// APC 23 — Evasive Action
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVASIVE_ACTION: CardRecord = CardRecord::new(
    "Evasive Action",
    "5d0b4f29-ada4-41d2-8292-b5af537c6fd2",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// APC 24 — Ice Cave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAVE: CardRecord = CardRecord::new(
    "Ice Cave",
    "fc2877c2-4426-4c07-92a2-8ba5107d5e7e",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// APC 25 — Index
pub(in crate::card::sets) static INDEX: CardRecord = CardRecord::new(
    "Index",
    "637ebd57-ba92-48ff-9ad4-d40dad2ff418",
    "Kev Walker",
    // No shuffle and no draw: it arranges the next five turns and costs a
    // card doing it.
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Look at the top five cards of your library, then put them back in any order.",
        abilities::look_at_top_cards_and_reorder(
            PlayerRefDef::EffectController,
            ValueDef::Constant(5),
        ),
    )),
);

// APC 26 — Jaded Response
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JADED_RESPONSE: CardRecord = CardRecord::new(
    "Jaded Response",
    "6a9ab1f0-4e75-4165-85bc-6f838c221d6a",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// APC 27 — Jilt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JILT: CardRecord = CardRecord::new(
    "Jilt",
    "a010d2b1-960d-4032-a47a-61fe0998bee3",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// APC 28 — Living Airship
pub(in crate::card::sets) static LIVING_AIRSHIP: CardRecord = CardRecord::new(
    "Living Airship",
    "b0441eef-392e-4af4-b189-2f1fb8bf3fca",
    "Mark Tedin",
    // A blue flier that only stops dying once green mana is available too.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Metathran"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{2}{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
        ),
    ]),
);

// APC 29 — Reef Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REEF_SHAMAN: CardRecord = CardRecord::new(
    "Reef Shaman",
    "c6f56714-0baa-48f9-8da1-50d9279e759c",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// APC 30 — Shimmering Mirage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMERING_MIRAGE: CardRecord = CardRecord::new(
    "Shimmering Mirage",
    "7263e20e-5473-42e9-90c3-3bcd848644ca",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// APC 31 — Tidal Courier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_COURIER: CardRecord = CardRecord::new(
    "Tidal Courier",
    "80b7cd5d-e81a-4729-b5d3-45587756413a",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// APC 32 — Unnatural Selection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNNATURAL_SELECTION: CardRecord = CardRecord::new(
    "Unnatural Selection",
    "c575e2cb-3990-4c73-b81c-e16311ec6bbb",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// APC 33 — Vodalian Mystic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_MYSTIC: CardRecord = CardRecord::new(
    "Vodalian Mystic",
    "81ec203a-067e-4360-9b4d-2d67db472aab",
    "Bob Petillo",
    crate::card::CardRules::unsupported(),
);

// APC 34 — Whirlpool Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLPOOL_DRAKE: CardRecord = CardRecord::new(
    "Whirlpool Drake",
    "6e866093-89a3-458d-8ebc-de805ef7885e",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// APC 35 — Whirlpool Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLPOOL_RIDER: CardRecord = CardRecord::new(
    "Whirlpool Rider",
    "0de47f44-8c5e-4114-9064-145d2d8813c6",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// APC 36 — Whirlpool Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLPOOL_WARRIOR: CardRecord = CardRecord::new(
    "Whirlpool Warrior",
    "01f891ca-4e6a-4710-b1cf-5dabb5e1ad93",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// APC 37 — Dead Ringers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEAD_RINGERS: CardRecord = CardRecord::new(
    "Dead Ringers",
    "9b78028c-3ebd-432d-b628-e1fa284f08f3",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// APC 38 — Desolation Angel
pub(in crate::card::sets) static DESOLATION_ANGEL: CardRecord = CardRecord::new(
    "Desolation Angel",
    "445127d4-8afb-47cf-b2a1-564540b1fdae",
    "Brom",
    // A 5/4 flier that always costs you your lands, and for two more takes
    // everyone's -- the sort of card only a deck already ahead can cast.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Angel"], 5, 4).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{B}{B}{W}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {W}{W} (You may pay an additional {W}{W} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, destroy all lands you control. If it was kicked, \
             destroy all lands instead.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
                otherwise: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    then: None,
                },
            },
        ),
    ]),
);

// APC 39 — Foul Presence
pub(in crate::card::sets) static FOUL_PRESENCE: CardRecord = CardRecord::new(
    "Foul Presence",
    "7c5a6fa8-d422-4e56-9e7b-2ff2fc8aecfe",
    "Ray Lago",
// Shrinks what it lands on and turns it into removal, which is a deal
    // worth taking on a creature that was never going to attack.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1 and has \"{T}: Target creature gets -1/-1 until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                                "{T}: Target creature gets -1/-1 until end of turn.",
                                &[CostDef::TapSource],
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
                            )),
                    ]),
                },
            ),
        ]),
);

// APC 40 — Grave Defiler
pub(in crate::card::sets) static GRAVE_DEFILER: CardRecord = CardRecord::new(
    "Grave Defiler",
    "f8f76edc-6067-43bd-9582-1d59caf91597",
    "Tony Szczudlo",
// The black member of the Ringleader cycle, with the regeneration standing
    // in for haste.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, reveal the top four cards of your library. Put all Zombie cards revealed this way into your hand and the rest on the bottom of your library in any order.",
            abilities::reveal_top_cards_put_matching_in_hand_rest_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
            ),
        ),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// APC 41 — Last Caress
pub(in crate::card::sets) static LAST_CARESS: CardRecord = CardRecord::new(
    "Last Caress",
    "12317075-92a2-4b3a-a694-3b764132beaf",
    "Eric Peterson",
    // One point of drain that replaces itself, so it is a cantrip first and a
    // clock only in the decks that cast it many times.
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player loses 1 life and you gain 1 life.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// APC 42 — Mind Extraction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_EXTRACTION: CardRecord = CardRecord::new(
    "Mind Extraction",
    "7d77ddcc-e66b-4036-8a55-ec42953918d1",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// APC 43 — Mournful Zombie
pub(in crate::card::sets) static MOURNFUL_ZOMBIE: CardRecord = CardRecord::new(
    "Mournful Zombie",
    "9ba12fb1-de8c-46c6-b33f-e0580ed2a3ee",
    "John Matson",
    // Any player, so the life can go to an opponent when that is the way out
    // of a symmetrical drain.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Target player gains 1 life.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// APC 44 — Necra Disciple
pub(in crate::card::sets) static NECRA_DISCIPLE: CardRecord = CardRecord::new(
    "Necra Disciple",
    "ae7a771f-bd21-4388-857f-08160b24e26e",
    "Jeff Miracola",
    // The black member of the Disciple cycle, fixing on one half and buying a
    // point of damage on the other.
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_mana(
            "{G}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "{W}, {T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// APC 45 — Necra Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECRA_SANCTUARY: CardRecord = CardRecord::new(
    "Necra Sanctuary",
    "5a0bf165-d7eb-4ae6-b30a-4e9fd55f401d",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// APC 46 — Necravolver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECRAVOLVER: CardRecord = CardRecord::new(
    "Necravolver",
    "232c32d9-9b0c-458d-b1b3-e4219bd34c82",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// APC 47 — Phyrexian Arena
pub(in crate::card::sets) static PHYREXIAN_ARENA: CardRecord = CardRecord::new(
    "Phyrexian Arena",
    "84e19975-e3e1-453b-b902-a1b1fc1d8504",
    "Pete Venters",
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, you draw a card and you lose 1 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// APC 48 — Phyrexian Gargantua
pub(in crate::card::sets) static PHYREXIAN_GARGANTUA: CardRecord = CardRecord::new(
    "Phyrexian Gargantua",
    "47c80cdd-4287-4ecb-992b-f265cd422098",
    "Carl Critchlow",
    // Six mana for a body and two cards; the life is the Phyrexian discount
    // and is paid whether or not the draws matter.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Phyrexian", "Horror"], 4, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you draw two cards and you lose 2 life.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ),
);

// APC 49 — Phyrexian Rager
pub(in crate::card::sets) static PHYREXIAN_RAGER: CardRecord = CardRecord::new(
    "Phyrexian Rager",
    "3addf34c-ea54-42a3-bccd-b73453d964d2",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Horror"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you draw a card and you lose 1 life.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// APC 50 — Planar Despair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_DESPAIR: CardRecord = CardRecord::new(
    "Planar Despair",
    "3a92d454-3f23-45bf-921f-25b0da4ce138",
    "Mike Sass",
    crate::card::CardRules::unsupported(),
);

// APC 51 — Quagmire Druid
pub(in crate::card::sets) static QUAGMIRE_DRUID: CardRecord = CardRecord::new(
    "Quagmire Druid",
    "5a75a004-d150-4fc1-a9a9-3b337a63e3e5",
    "Dana Knutson",
    // Repeatable enchantment removal, but every use costs a creature, and the
    // Druid may be the only one left to feed it.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Druid"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}, Sacrifice a creature: Destroy target enchantment.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// APC 52 — Suppress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPPRESS: CardRecord = CardRecord::new(
    "Suppress",
    "642eefde-8727-44ff-9e04-373abfcd0679",
    "Terese Nielsen & Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// APC 53 — Urborg Uprising
pub(in crate::card::sets) static URBORG_UPRISING: CardRecord = CardRecord::new(
    "Urborg Uprising",
    "961619e3-f48b-4099-8a33-ca1e294085dd",
    "Adam Rex",
    // "Up to two" and the replacement draw mean an empty graveyard still
    // leaves a card, so this is never a blank five mana.
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Return up to two target creature cards from your graveyard to your hand.\nDraw a card.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
            2,
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// APC 54 — Zombie Boa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_BOA: CardRecord = CardRecord::new(
    "Zombie Boa",
    "1fb8c277-3154-47c9-835f-327cac297a5e",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// APC 55 — Bloodfire Colossus
pub(in crate::card::sets) static BLOODFIRE_COLOSSUS: CardRecord = CardRecord::new(
    "Bloodfire Colossus",
    "518145f3-9919-4ed6-9e2e-772ee349ea57",
    "Greg Staples",
    // The top of the Bloodfire cycle, and the one that ends games from either
    // side: eight mana of creature that can instead be a Fireball at everyone.
    CardRules::new_creature(mana_cost!("{6}{R}{R}"), &["Giant"], 6, 6).with_ability(
        AbilityDef::activated(
            "{R}, Sacrifice this creature: It deals 6 damage to each creature and each player.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::SacrificeSource],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::Constant(6),
                ),
                EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(6)),
            ]),
        ),
    ),
);

// APC 56 — Bloodfire Dwarf
pub(in crate::card::sets) static BLOODFIRE_DWARF: CardRecord = CardRecord::new(
    "Bloodfire Dwarf",
    "86b5c38e-7d74-4862-8187-f5db4a3d1e0f",
    "Ron Spencer",
    // The sweep is symmetrical and hits the Dwarf's own side, but the Dwarf
    // itself is already gone paying for it.
    CardRules::new_creature(mana_cost!("{R}"), &["Dwarf"], 1, 1).with_ability(
        AbilityDef::activated(
            "{R}, Sacrifice this creature: It deals 1 damage to each creature without flying.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::SacrificeSource],
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// APC 57 — Bloodfire Infusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODFIRE_INFUSION: CardRecord = CardRecord::new(
    "Bloodfire Infusion",
    "2639e9b7-ed8c-48fd-a8b7-b99d8dad4bc0",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// APC 58 — Bloodfire Kavu
pub(in crate::card::sets) static BLOODFIRE_KAVU: CardRecord = CardRecord::new(
    "Bloodfire Kavu",
    "1442b1f3-8c2c-4553-906f-c864fcdc6ae5",
    "Greg Staples",
    // The middle of the Bloodfire cycle: a bigger sweep than the Dwarf's, and
    // this one spares nothing in the air either.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Kavu"], 2, 2).with_ability(
        AbilityDef::activated(
            "{R}, Sacrifice this creature: It deals 2 damage to each creature.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::SacrificeSource],
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// APC 59 — Desolation Giant
pub(in crate::card::sets) static DESOLATION_GIANT: CardRecord = CardRecord::new(
    "Desolation Giant",
    "2e7291da-1d14-4763-8691-c67136ab67c7",
    "Alan Pollack",
    // The same bargain one card type over: a 3/3 that eats your board, or for
    // two white mana a sweeper that leaves itself standing.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 3, 3).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{R}{R}{W}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {W}{W} (You may pay an additional {W}{W} as you cast this spell.)"),
            EffectDef::None,
        ),
        abilities::enters_trigger(
            "When this creature enters, destroy all other creatures you control. If it was \
             kicked, destroy all other creatures instead.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
                otherwise: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    then: None,
                },
            },
        ),
    ]),
);

// APC 60 — Dwarven Landslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_LANDSLIDE: CardRecord = CardRecord::new(
    "Dwarven Landslide",
    "48ab243e-d08d-4ece-9725-4bb5f67b1c92",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// APC 61 — Dwarven Patrol
pub(in crate::card::sets) static DWARVEN_PATROL: CardRecord = CardRecord::new(
    "Dwarven Patrol",
    "03c08df5-f5e7-4498-ac80-25ccbe304b26",
    "Greg Hildebrandt",
    // A 4/2 for three that a mono-red deck can never untap: the drawback is
    // only paid off by splashing, which is the whole point of the set.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf"], 4, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast a nonred spell, untap this creature.",
            // Colorless spells are nonred too, so an artifact turns it back on.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Red)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// APC 62 — Goblin Ringleader
pub(in crate::card::sets) static GOBLIN_RINGLEADER: CardRecord = CardRecord::new(
    "Goblin Ringleader",
    "b6b2cd77-9552-48b1-80cb-26966323c1ea",
    "Mark Romanoski",
// Haste plus a refill is what keeps the deck from running out: each
    // Ringleader tends to find the next one.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger(
            "When this creature enters, reveal the top four cards of your library. Put all Goblin cards revealed this way into your hand and the rest on the bottom of your library in any order.",
            abilities::reveal_top_cards_put_matching_in_hand_rest_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
            ),
        ),
    ]),
);

// APC 63 — Illuminate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUMINATE: CardRecord = CardRecord::new(
    "Illuminate",
    "ceef2761-7301-42de-8f54-49b8cd1e457b",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// APC 64 — Kavu Glider
pub(in crate::card::sets) static KAVU_GLIDER: CardRecord = CardRecord::new(
    "Kavu Glider",
    "158aa5a8-2769-4a8a-b457-001abc862b35",
    "Heather Hudson",
    // Both activations are repeatable, so the only ceiling is how much white
    // and blue mana a red deck can actually produce.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::apply_to_self_until_end_of_turn(
            "{U}: This creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            AppliedEffectDef::add_ability(&abilities::flying()),
        ),
    ]),
);

// APC 65 — Minotaur Tactician
// Each query is read twice, once for power and once for toughness, so both
// earn a name.
static A_WHITE_CREATURE_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::controlled_by(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Color(ManaColor::White),
    ]),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

static A_BLUE_CREATURE_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::controlled_by(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Color(ManaColor::Blue),
    ]),
    &[ZoneKind::Battlefield],
    PlayerSetDef::Related(PlayerRelation::You),
);

pub(in crate::card::sets) static MINOTAUR_TACTICIAN: CardRecord = CardRecord::new(
    "Minotaur Tactician",
    "097decb6-03bd-4a84-ab9a-75becf85cae8",
    "Carl Critchlow",
    // Two separate clauses rather than one counting clause: a single creature
    // that is both white and blue turns both of them on.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a white creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&A_WHITE_CREATURE_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&A_WHITE_CREATURE_YOU_CONTROL),
                ),
            },
        ),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a blue creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&A_BLUE_CREATURE_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&A_BLUE_CREATURE_YOU_CONTROL),
                ),
            },
        ),
    ]),
);

// APC 66 — Raka Disciple
pub(in crate::card::sets) static RAKA_DISCIPLE: CardRecord = CardRecord::new(
    "Raka Disciple",
    "41462d43-4f9f-46ba-b79d-434597e74b6b",
    "Arnie Swekel",
    // The red member of the Disciple cycle, and the only one whose two halves
    // are both defensive.
    CardRules::new_creature(mana_cost!("{R}"), &["Minotaur", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}, {T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{U}, {T}: Target creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// APC 67 — Raka Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKA_SANCTUARY: CardRecord = CardRecord::new(
    "Raka Sanctuary",
    "62cab0be-589c-42a0-a297-1faaec46c73f",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// APC 68 — Rakavolver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKAVOLVER: CardRecord = CardRecord::new(
    "Rakavolver",
    "43787e24-0b7d-4005-8db4-68544476bd34",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// APC 69 — Smash
pub(in crate::card::sets) static SMASH: CardRecord = CardRecord::new(
    "Smash",
    "4a6c869c-74c2-42b6-bb23-a2f481c4b673",
    "Pete Venters",
    // The replacement draw is what makes this maindeckable: a dead Smash
    // still cycles, so the artifact hate costs no card.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// APC 70 — Tahngarth's Glare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAHNGARTH_S_GLARE: CardRecord = CardRecord::new(
    "Tahngarth's Glare",
    "442a4331-99ce-405e-b261-19b7f3375ddf",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// APC 71 — Tundra Kavu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUNDRA_KAVU: CardRecord = CardRecord::new(
    "Tundra Kavu",
    "fc34e735-ac3c-4954-a4c8-3ed55d811715",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// APC 72 — Wild Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_RESEARCH: CardRecord = CardRecord::new(
    "Wild Research",
    "8f00e6f1-e854-40b0-855d-7e0d7d233850",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// APC 73 — Ana Disciple
pub(in crate::card::sets) static ANA_DISCIPLE: CardRecord = CardRecord::new(
    "Ana Disciple",
    "7efe00f9-bf42-4d6f-9a22-b357b1c1e092",
    "Darrell Riche",
    // The green member of the Disciple cycle; both halves are combat tricks
    // rather than mana, so the tap is the real cost.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, {T}: Target creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{B}, {T}: Target creature gets -2/-0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}")), CostDef::TapSource],
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
    ]),
);

// APC 74 — Ana Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANA_SANCTUARY: CardRecord = CardRecord::new(
    "Ana Sanctuary",
    "9d1599bb-4f43-4ab3-985a-8be5219f2195",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// APC 75 — Anavolver
pub(in crate::card::sets) static ANAVOLVER: CardRecord = CardRecord::new(
    "Anavolver",
    "5e685a8c-fba6-495f-ac0f-1ff5456b22d0",
    "Matt Cavotta",
CardRules::new_creature(mana_cost!("{3}{G}"), &["Volver"], 3, 3).with_abilities(&[
        abilities::kicker_with_label(
            "Kicker {1}{U}",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
        ),
        abilities::kicker_with_label(
            "Kicker {B}",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
        AbilityDef::as_enters(
            "If this creature was kicked with its {1}{U} kicker, it enters with two +1/+1 counters on it and with flying.",
            crate::card::ReplacementEffectDef::ModifyBattlefieldEntry(
                crate::card::BattlefieldEntryModificationDef::AddCountersValue {
                    kind: crate::card::CounterKind::PlusOnePlusOne,
                    amount: ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::PRIMARY,
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    )),
                },
            ),
        ),
        AbilityDef::static_ability(
            "It has flying if its {1}{U} kicker was paid.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
        AbilityDef::as_enters(
            "If this creature was kicked with its {B} kicker, it enters with a +1/+1 counter on it and with \"Pay 3 life: Regenerate this creature.\"",
            crate::card::ReplacementEffectDef::ModifyBattlefieldEntry(
                crate::card::BattlefieldEntryModificationDef::AddCountersValue {
                    kind: crate::card::CounterKind::PlusOnePlusOne,
                    amount: ValueDef::IfAdditionalCostPaid(&AdditionalCostValueDef::new(
                        AdditionalCostIndex::SECONDARY,
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    )),
                },
            ),
        ),
        AbilityDef::static_ability(
            "It has \"Pay 3 life: Regenerate this creature.\" if its {B} kicker was paid.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    AdditionalCostIndex::SECONDARY,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::regenerate_self(
                        "Pay 3 life: Regenerate this creature.",
                        &[CostDef::PayLife(3)],
                    )),
                },
            },
        ),
    ]),
);

// APC 76 — Bog Gnarr
pub(in crate::card::sets) static BOG_GNARR: CardRecord = CardRecord::new(
    "Bog Gnarr",
    "3f230831-023c-41aa-832e-16ac81e68588",
    "Daren Bader",
    // It grows on the opponent's turn as readily as your own, so it is a
    // blocker that punishes the removal aimed at it.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever a player casts a black spell, this creature gets +2/+2 until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Black)),
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

// APC 77 — Gaea's Balance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_BALANCE: CardRecord = CardRecord::new(
    "Gaea's Balance",
    "f1ffc5f8-ff1c-4733-b046-8679fa16371b",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// APC 78 — Glade Gnarr
pub(in crate::card::sets) static GLADE_GNARR: CardRecord = CardRecord::new(
    "Glade Gnarr",
    "ee38eeae-918b-4d19-b37a-175ac5db37a4",
    "Daren Bader",
    // Six mana for a body that a control deck cannot answer without making
    // it bigger first.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Beast"], 4, 4).with_ability(
        AbilityDef::triggered(
            "Whenever a player casts a blue spell, this creature gets +2/+2 until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Blue)),
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

// APC 79 — Kavu Howler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_HOWLER: CardRecord = CardRecord::new(
    "Kavu Howler",
    "5fdf502f-445d-4724-b7d0-8fdd5bf557a8",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// APC 80 — Kavu Mauler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_MAULER: CardRecord = CardRecord::new(
    "Kavu Mauler",
    "79adc3af-5fa3-4cb6-9bbc-52ede0c69263",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// APC 81 — Lay of the Land
pub(in crate::card::sets) static LAY_OF_THE_LAND: CardRecord = CardRecord::new(
    "Lay of the Land",
    "90b10608-8917-4337-ad60-ab31ab8c0fc4",
    "Mark Zug",
CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// APC 82 — Penumbra Bobcat
pub(in crate::card::sets) static PENUMBRA_BOBCAT: CardRecord = CardRecord::new(
    "Penumbra Bobcat",
    "21049fee-a748-4856-99ae-3a225a168532",
    "Heather Hudson",
    // The smallest Penumbra: the token comes back the same size in the other
    // half of the set's color pair, so removal trades down rather than up.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Cat"], 2, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create a 2/1 black Cat creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Cat"], &[ManaColor::Black], 2, 1),
            ))),
        ),
    ),
);

// APC 83 — Penumbra Kavu
pub(in crate::card::sets) static PENUMBRA_KAVU: CardRecord = CardRecord::new(
    "Penumbra Kavu",
    "ee334211-4109-46ff-8676-856048221a1c",
    "Tony Szczudlo",
    // The middle Penumbra, and the one whose token keeps the Kavu type the
    // rest of the set cares about.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Kavu"], 3, 3).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create a 3/3 black Kavu creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Kavu"], &[ManaColor::Black], 3, 3),
            ))),
        ),
    ),
);

// APC 84 — Penumbra Wurm
pub(in crate::card::sets) static PENUMBRA_WURM: CardRecord = CardRecord::new(
    "Penumbra Wurm",
    "ae3dffe7-ecaf-4cf0-a43e-8e2746282992",
    "Jeff Easley",
    // The token copies the trample as well as the body, so seven mana really
    // buys twelve power of it.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::dies_trigger(
            "When this creature dies, create a 6/6 black Wurm creature token with trample.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Wurm"], &[ManaColor::Black], 6, 6)
                    .with_abilities(&[abilities::trample()]),
            ))),
        ),
    ]),
);

// APC 85 — Savage Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_GORILLA: CardRecord = CardRecord::new(
    "Savage Gorilla",
    "32ad3f87-9f25-455f-9933-3b0b0eaad467",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// APC 86 — Strength of Night
pub(in crate::card::sets) static STRENGTH_OF_NIGHT: CardRecord = CardRecord::new(
    "Strength of Night",
    "87aab031-4e44-44cd-89a7-6cffc7288cd1",
    "John Avon",
    // An anthem for the turn, and a much bigger one if the board happens to be
    // Zombies -- which in this block it often was.
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{G}{B}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {B} (You may pay an additional {B} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Creatures you control get +1/+1 until end of turn. If this spell was kicked, \
             Zombie creatures you control get an additional +2/+2 until end of turn.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// APC 87 — Sylvan Messenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYLVAN_MESSENGER: CardRecord = CardRecord::new(
    "Sylvan Messenger",
    "fd67d17e-23d2-47a0-a10b-c3d63cbf969a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// APC 88 — Symbiotic Deployment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYMBIOTIC_DEPLOYMENT: CardRecord = CardRecord::new(
    "Symbiotic Deployment",
    "a6e2b7e9-d52b-478e-b118-e890a81fd471",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// APC 89 — Tranquil Path
pub(in crate::card::sets) static TRANQUIL_PATH: CardRecord = CardRecord::new(
    "Tranquil Path",
    "2da8c059-3309-49a5-ae97-c048aefc922f",
    "John Avon",
    // It destroys your own enchantments too, so the card it draws is the
    // only reason to play it over a targeted answer.
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_ability(AbilityDef::spell(
        "Destroy all enchantments.\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// APC 90 — Urborg Elf
pub(in crate::card::sets) static URBORG_ELF: CardRecord = CardRecord::new(
    "Urborg Elf",
    "1d8521bf-d026-4d26-831e-a2f253307c93",
    "Bob Petillo",
    // Three colours out of a two-drop, aimed at exactly the wedge the set
    // was built around.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {B}, {G}, or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ),
);

// APC 91 — Aether Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_MUTATION: CardRecord = CardRecord::new(
    "Aether Mutation",
    "a9507116-ede8-40a1-8fa3-705e6f6f64c0",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// APC 92 — Captain's Maneuver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_S_MANEUVER: CardRecord = CardRecord::new(
    "Captain's Maneuver",
    "fb50813c-72df-49e7-bac5-e6e247649241",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// APC 93 — Consume Strength
pub(in crate::card::sets) static CONSUME_STRENGTH: CardRecord = CardRecord::new(
    "Consume Strength",
    "f005fc90-7e81-4bd4-a479-438337110979",
    "Adam Rex",
// Twice the swing for a second colour, which is what the gold cards in
    // the set were for.
    CardRules::new_instant(mana_cost!("{1}{B}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. Another target creature gets -2/-2 until end of turn.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            ))
            .another(),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// APC 94 — Cromat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROMAT: CardRecord = CardRecord::new(
    "Cromat",
    "7d9e0a23-d2a8-40a6-9076-ed6fb539141b",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// APC 95 — Death Grasp
pub(in crate::card::sets) static DEATH_GRASP: CardRecord = CardRecord::new(
    "Death Grasp",
    "0e893dd4-8c37-496e-bc39-cd83d42b4cc4",
    "Eric Peterson",
    // The life is gained whatever the damage was aimed at, so a swing of 2X
    // is available against the opponent's face alone.
    CardRules::new_sorcery(mana_cost!("{X}{W}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Death Grasp deals X damage to any target. You gain X life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::ChosenX,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
            },
        ]),
    )),
);

// APC 96 — Death Mutation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_MUTATION: CardRecord = CardRecord::new(
    "Death Mutation",
    "4c643d87-50bc-4380-b1d6-0a465eef5dbf",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// APC 97 — Ebony Treefolk
pub(in crate::card::sets) static EBONY_TREEFOLK: CardRecord = CardRecord::new(
    "Ebony Treefolk",
    "2b85dadb-351f-4975-a2c3-febf5e80bc85",
    "Matt Cavotta",
    // Two colours for one point, which is a mana sink only a deck already
    // in both colours can use.
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Treefolk"], 3, 3).with_ability(
        AbilityDef::activated(
            "{B}{G}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// APC 98 — Fervent Charge
pub(in crate::card::sets) static FERVENT_CHARGE: CardRecord = CardRecord::new(
    "Fervent Charge",
    "d610a9d5-c650-45ad-a9b0-b55113701e05",
    "Mark Tedin",
    // Two power on every attacker, which turns a board that was trading
    // evenly into one that wins every combat.
    CardRules::new_enchantment(mana_cost!("{1}{R}{W}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature you control attacks, it gets +2/+2 until end of turn.",
        TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ]))),
        EffectDef::Apply {
            recipient: EffectRecipientDef::TriggeringObject,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// APC 99 — Flowstone Charger
pub(in crate::card::sets) static FLOWSTONE_CHARGER: CardRecord = CardRecord::new(
    "Flowstone Charger",
    "c57abdab-d99c-418c-818d-b06a8722d733",
    "John Gallagher",
    // It attacks as a 5/2 and blocks as a 2/5, which is a fine pair of
    // bodies to have on one card.
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Beast"], 2, 5).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +3/-3 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// APC 100 — Fungal Shambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNGAL_SHAMBLER: CardRecord = CardRecord::new(
    "Fungal Shambler",
    "1b65f96b-019b-40a9-9b4d-acd4abf4a0f9",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// APC 101 — Gaea's Skyfolk
pub(in crate::card::sets) static GAEA_S_SKYFOLK: CardRecord = CardRecord::new(
    "Gaea's Skyfolk",
    "8a564432-c2b3-4cf6-b4bc-2e2600b92911",
    "Terese Nielsen",
    // A 2/2 flier for two, which neither colour gets alone: the gold cost is
    // exactly what the extra evasion is worth.
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Elf", "Merfolk"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// APC 102 — Gerrard's Verdict
pub(in crate::card::sets) static GERRARDS_VERDICT: CardRecord = CardRecord::new(
    "Gerrard's Verdict",
    "583740c0-68cf-4205-b682-2f97c0880d42",
    "Carl Critchlow",
    // Two cards for two mana, and the life is what makes it a fine turn-two
    // play against a deck full of lands.
    CardRules::new_sorcery(mana_cost!("{W}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards. You gain 3 life for each land card discarded this way.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: Some(DiscardFollowUpDef {
                counted: ObjectPredicateDef::HasType(CardType::Land),
                bound: Some(ParentBinding),
                // Three life a land, counted among the two cards that actually went. The
                // discard is the opponent's choice, so the payoff cannot be known until
                // they have made it.
                effect: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::BoundObjectCount(ParentBinding),
                        factor: 3,
                    }),
                },
            }),
        },
    )),
);

// APC 103 — Goblin Legionnaire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_LEGIONNAIRE: CardRecord = CardRecord::new(
    "Goblin Legionnaire",
    "c684407e-277a-4e32-a978-cdac9548acce",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// APC 104 — Goblin Trenches
pub(in crate::card::sets) static GOBLIN_TRENCHES: CardRecord = CardRecord::new(
    "Goblin Trenches",
    "2100844c-6a41-40f5-b7f8-9b426d5a6945",
    "Wayne England",
    // Every spare land becomes two bodies, which is what a deck flooding
    // out wants more than it wants another land drop.
    CardRules::new_enchantment(mana_cost!("{1}{R}{W}")).with_ability(AbilityDef::activated(
        "{2}, Sacrifice a land: Create two 1/1 red and white Goblin Soldier creature tokens.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Land),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                &["Goblin", "Soldier"],
                &[ManaColor::Red, ManaColor::White],
                1,
                1,
            )))
            .with_count(ValueDef::Constant(2)),
        ),
    )),
);

// APC 105 — Guided Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDED_PASSAGE: CardRecord = CardRecord::new(
    "Guided Passage",
    "0b2e8e58-aee1-4882-943a-17a6af2f8410",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// APC 106 — Jungle Barrier
pub(in crate::card::sets) static JUNGLE_BARRIER: CardRecord = CardRecord::new(
    "Jungle Barrier",
    "4eb114a4-44e5-4375-92b8-00a0b0acbe94",
    "Edward P. Beard, Jr.",
    // A wall that replaces itself, which is the only way a defensive card
    // is never a wasted draw.
    CardRules::new_creature(mana_cost!("{2}{G}{U}"), &["Plant", "Wall"], 2, 6).with_abilities(&[
        abilities::defender(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// APC 107 — Last Stand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_STAND: CardRecord = CardRecord::new(
    "Last Stand",
    "7dc3d054-6266-4ce0-89ed-f8b170794f2e",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// APC 108 — Lightning Angel
pub(in crate::card::sets) static LIGHTNING_ANGEL: CardRecord = CardRecord::new(
    "Lightning Angel",
    "6518d0c5-58ee-4089-bf19-5030d4319681",
    "rk post",
    // Haste and vigilance together are what make the four-mana body a clock
    // that never stops blocking.
    CardRules::new_creature(mana_cost!("{1}{U}{R}{W}"), &["Angel"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::haste(),
    ]),
);

// APC 109 — Llanowar Dead
pub(in crate::card::sets) static LLANOWAR_DEAD: CardRecord = CardRecord::new(
    "Llanowar Dead",
    "f271969e-1529-42d1-878b-011f80ab0f05",
    "Ben Thompson",
    // A gold two-drop that taps for one of its own colours: the body is real
    // and the mana is the reason it is played.
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Elf"], 2, 2)
        .with_ability(abilities::tap_for(ManaColor::Black)),
);

// APC 110 — Martyrs' Tomb
pub(in crate::card::sets) static MARTYRS_TOMB: CardRecord = CardRecord::new(
    "Martyrs' Tomb",
    "a906a775-7c2d-47b7-a20e-a325dd28d0bd",
    "Anthony S. Waters",
// Life into prevention, one point at a time, which only reads as a gain
    // when the damage was going to kill something.
    CardRules::new_enchantment(mana_cost!("{2}{W}{B}")).with_ability(AbilityDef::activated_with_targets(
    "Pay 2 life: Prevent the next 1 damage that would be dealt to target creature this turn.",
    &[CostDef::PayLife(2)],
    &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
        CardType::Creature,
    ))],
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::amount(
            DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
            ValueDef::Constant(1),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
)),
);

// APC 111 — Minotaur Illusionist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINOTAUR_ILLUSIONIST: CardRecord = CardRecord::new(
    "Minotaur Illusionist",
    "d8a49d29-6d01-4b1d-80c8-9e5378a76878",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// APC 112 — Mystic Snake
pub(in crate::card::sets) static MYSTIC_SNAKE: CardRecord = CardRecord::new(
    "Mystic Snake",
    "f098a28c-5f9b-4a2c-b109-c342365eb948",
    "Daren Bader",
    // A counterspell that stays on the board afterwards, which is why four
    // mana and three colours were considered a fair price.
    CardRules::new_creature(mana_cost!("{1}{G}{U}{U}"), &["Snake"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, counter target spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// APC 113 — Overgrown Estate
pub(in crate::card::sets) static OVERGROWN_ESTATE: CardRecord = CardRecord::new(
    "Overgrown Estate",
    "c1c48c58-3532-4022-9eec-1a870385cbf3",
    "Brian Snõddy",
    // Three life per land is a rate only a deck that has already stopped
    // needing lands can use, which is exactly the deck it was printed for.
    CardRules::new_enchantment(mana_cost!("{W}{B}{G}")).with_ability(AbilityDef::activated(
        "Sacrifice a land: You gain 3 life.",
        &[CostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Land),
            controller: PlayerRelation::You,
        }],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )),
);

// APC 114 — Pernicious Deed
// Audit: unsupported — Needs an activated ability's X inside a nontarget object query. ManaValueAtMostValue with ValueDef::ChosenX resolves in a target predicate on a spell with {X} in its mana cost, but an activated ability's X does not reach the query behind EffectRecipientDef::matching_objects, so the sweep destroys nothing at any X.
pub(in crate::card::sets) static PERNICIOUS_DEED: CardRecord = CardRecord::new(
    "Pernicious Deed",
    "ae4cbb33-4947-49f0-b612-a92141fbfaa6",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// APC 115 — Powerstone Minefield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWERSTONE_MINEFIELD: CardRecord = CardRecord::new(
    "Powerstone Minefield",
    "b17807b9-8feb-48ac-813a-829577f5b9e8",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// APC 116 — Prophetic Bolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROPHETIC_BOLT: CardRecord = CardRecord::new(
    "Prophetic Bolt",
    "79f74291-c452-4a60-bf5f-73efad6583d4",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// APC 117 — Putrid Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_WARRIOR: CardRecord = CardRecord::new(
    "Putrid Warrior",
    "17fce298-3338-4f41-8156-ab6322951a76",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// APC 118 — Quicksilver Dagger
pub(in crate::card::sets) static QUICKSILVER_DAGGER: CardRecord = CardRecord::new(
    "Quicksilver Dagger",
    "83c74012-6060-4fad-aa73-6e6afd33c482",
    "Alex Horley-Orlandelli",
// The damage is incidental; the card every turn is the reason to spend
    // three mana and a creature's attack on an Aura.
    CardRules::new_enchantment(mana_cost!("{1}{U}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has \"{T}: This creature deals 1 damage to target player or planeswalker. You draw a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "{T}: This creature deals 1 damage to target player or planeswalker. You draw a card.",
                            &[CostDef::TapSource],
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::PlayerOrPlaneswalker(
                                    PlayerRelation::Any,
                                ),
                            )],
                            EffectDef::Sequence(&[
                                EffectDef::damage(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ValueDef::Constant(1),
                                ),
                                EffectDef::DrawCards {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                },
                            ]),
                        )),
                },
            ),
        ]),
);

// APC 119 — Razorfin Hunter
pub(in crate::card::sets) static RAZORFIN_HUNTER: CardRecord = CardRecord::new(
    "Razorfin Hunter",
    "99829552-917a-4373-9772-4255dff542d6",
    "Jeff Easley",
    // Prodigal Sorcerer's ability on a two-mana body, paid for with a second
    // color rather than with a third mana.
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Merfolk", "Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// APC 120 — Soul Link
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_LINK: CardRecord = CardRecord::new(
    "Soul Link",
    "425e0ca4-8592-4802-b7c4-6e3323edd78c",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// APC 121 — Spiritmonger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITMONGER: CardRecord = CardRecord::new(
    "Spiritmonger",
    "b96d6e67-f690-4f19-bb25-a7c2d2aaf42f",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// APC 122 — Squee's Embrace
pub(in crate::card::sets) static SQUEE_S_EMBRACE: CardRecord = CardRecord::new(
    "Squee's Embrace",
    "e682a705-9341-4d1e-a9c5-d428e50b9a03",
    "Rebecca Guay",
    // The card comes back when the creature dies, so the Aura costs tempo
    // rather than a card -- which is the whole cycle's promise.
    CardRules::new_enchantment(mana_cost!("{R}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::dies_trigger_matching(
                "When enchanted creature dies, return that card to its owner's hand.",
                ObjectPredicateDef::AttachedToSource,
                EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// APC 123 — Squee's Revenge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUEE_S_REVENGE: CardRecord = CardRecord::new(
    "Squee's Revenge",
    "2b391ee3-c1cd-47bc-9540-977cbc32913e",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// APC 124 — Suffocating Blast
pub(in crate::card::sets) static SUFFOCATING_BLAST: CardRecord = CardRecord::new(
    "Suffocating Blast",
    "c2a70297-2a7b-4a0c-ace5-cd61bfe6dafd",
    "Christopher Moeller",
    // Four mana for two cards' worth of answers, which is what a deck playing
    // three colours is buying with its mana base.
    CardRules::new_instant(mana_cost!("{1}{U}{U}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell and Suffocating Blast deals 3 damage to target creature.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                },
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex(1)),
                    ValueDef::Constant(3),
                ),
            ]),
        ),
    ),
);

// APC 125 — Temporal Spring
pub(in crate::card::sets) static TEMPORAL_SPRING: CardRecord = CardRecord::new(
    "Temporal Spring",
    "b584dfd1-a56c-406e-8504-47ea136dc102",
    "John Matson",
    // The same tempo aimed at anything, which in three colours is what a
    // gold card had to be worth.
    CardRules::new_sorcery(mana_cost!("{1}{G}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target permanent on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Library,
            ZonePlacement::Top,
        ),
    )),
);

// APC 126 — Vindicate
pub(in crate::card::sets) static VINDICATE: CardRecord = CardRecord::new(
    "Vindicate",
    "2a1bfefd-dae8-49e9-9d56-cc852e3dc93b",
    "Brian Snõddy",
    CardRules::new_sorcery(mana_cost!("{1}{W}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target permanent.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
    )),
);

// APC 127 — Yavimaya's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_S_EMBRACE: CardRecord = CardRecord::new(
    "Yavimaya's Embrace",
    "36b41ff1-240a-447b-bb47-1b9be53ab3e6",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// APC 128 — Fire // Ice
pub(in crate::card::sets) static FIRE_ICE: CardRecord = CardRecord::new_split(
    "Fire // Ice",
    "f98f4538-5b5b-475d-b98f-49d01dae6f04",
    "David Martin & Franz Vohwinkel",
    &[
        (
            "Fire",
            CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Fire deals 2 damage divided as you choose among one or two targets.",
                    &[AbilityTargetDef {
                        predicate: AbilityTargetPredicate::AnyTarget,
                        minimum: 1,
                        maximum: 2,
                        exact_count: None,
                        divided_total: Some(DividedTotal::Fixed(2)),
                        another: false,
                        excludes_source: false,
                        chooser: TargetChooserDef::Controller,
                    }],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::DividedAmongTargets,
                    ),
                ),
            ),
        ),
        (
            "Ice",
            CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Tap target permanent.\nDraw a card.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::Any,
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Tap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
            ),
        ),
    ],
);

// APC 129 — Illusion // Reality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSION_REALITY: CardRecord = CardRecord::new(
    "Illusion // Reality",
    "8e7dd90a-4f93-43aa-b503-18289fdd571e",
    "John Avon & David Martin",
    crate::card::CardRules::unsupported(),
);

// APC 130 — Life // Death
pub(in crate::card::sets) static LIFE_DEATH: CardRecord = CardRecord::new_split(
    "Life // Death",
    "7ab75cdb-93a1-4f78-b404-37566295c321",
    "Anthony S. Waters & Edward P. Beard, Jr.",
&[
        (
            "Life",
            CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell(
                "All lands you control become 1/1 creatures until end of turn. They're still lands.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    // "They're still lands" is not flavour: adding the creature type rather
                    // than replacing the land one is what keeps them tapping for mana, and what
                    // makes a board wipe answer the whole mana base.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            crate::card::CardTypeSet::single(CardType::Creature),
                        ),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )),
        ),
        (
            "Death",
            CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard to the battlefield. You lose life equal to its mana value.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            // Reanimate takes one from any graveyard; this half is narrower.
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                            arrival: crate::card::BattlefieldArrivalDef {
                                controller: Some(PlayerRelation::You),
                                ..crate::card::BattlefieldArrivalDef::DEFAULT
                            },
                        },
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                        },
                    ]),
                ),
            ),
        ),
    ],
);

// APC 131 — Night // Day
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHT_DAY: CardRecord = CardRecord::new(
    "Night // Day",
    "e8f109f1-9798-4bd3-b51a-49f173251dfd",
    "Christopher Moeller & Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// APC 132 — Order // Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_CHAOS: CardRecord = CardRecord::new(
    "Order // Chaos",
    "14e4f5a4-b1ea-4816-b2d7-cf148468a388",
    "Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// APC 133 — Brass Herald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_HERALD: CardRecord = CardRecord::new(
    "Brass Herald",
    "89bd60a7-2ba4-4fce-bf74-2ea9b8fd4dbe",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// APC 134 — Dodecapod
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DODECAPOD: CardRecord = CardRecord::new(
    "Dodecapod",
    "ded8b992-a1c2-4e43-ad0a-ea3995a3c8b8",
    "John Howe",
    crate::card::CardRules::unsupported(),
);

// APC 135 — Dragon Arch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_ARCH: CardRecord = CardRecord::new(
    "Dragon Arch",
    "eec581b8-e509-420c-b142-afaa6dd06cc8",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// APC 136 — Emblazoned Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBLAZONED_GOLEM: CardRecord = CardRecord::new(
    "Emblazoned Golem",
    "98527fc6-4f4c-4ded-9e72-49186b7e5bd3",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// APC 137 — Legacy Weapon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGACY_WEAPON: CardRecord = CardRecord::new(
    "Legacy Weapon",
    "385d8691-b9bd-4b4d-86db-7a7cc6181104",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// APC 138 — Mask of Intolerance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASK_OF_INTOLERANCE: CardRecord = CardRecord::new(
    "Mask of Intolerance",
    "f623ae51-5f15-4153-b2ed-d03b57b7db54",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// APC 139 — Battlefield Forge
pub(in crate::card::sets) static BATTLEFIELD_FORGE: CardRecord = CardRecord::new(
    "Battlefield Forge",
    "a9c25e71-0140-48fe-8b9e-33b4b50c5c12",
    "Darrell Riche",
    // The red-white member of the cycle, which is the whole reason a
    // two-colour aggressive deck could keep its curve.
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {W}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::White],
    )),
);

// APC 140 — Caves of Koilos
pub(in crate::card::sets) static CAVES_OF_KOILOS: CardRecord = CardRecord::new(
    "Caves of Koilos",
    "144dd08e-451e-4438-b572-7a138e1a15f3",
    "Jim Nelson",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {B}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Black],
    )),
);

// APC 141 — Llanowar Wastes
pub(in crate::card::sets) static LLANOWAR_WASTES: CardRecord = CardRecord::new(
    "Llanowar Wastes",
    "610b7cd5-5532-45a9-acfe-24a818034d1c",
    "Rob Alexander",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {B} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Black, ManaColor::Green],
    )),
);

// APC 142 — Shivan Reef
pub(in crate::card::sets) static SHIVAN_REEF: CardRecord = CardRecord::new(
    "Shivan Reef",
    "c3403143-2b4e-4408-b138-c856bbc1e9a5",
    "Rob Alexander",
    // The blue-red member.
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {R}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Red],
    )),
);

// APC 143 — Yavimaya Coast
pub(in crate::card::sets) static YAVIMAYA_COAST: CardRecord = CardRecord::new(
    "Yavimaya Coast",
    "177ee102-d981-4fc3-9f09-9dd07755f22c",
    "Anthony S. Waters",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {G} or {U}. This land deals 1 damage to you.",
        &[ManaColor::Green, ManaColor::Blue],
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELFIRE_CRUSADER,
    &COALITION_FLAG,
    &COALITION_HONOR_GUARD,
    &DEGA_DISCIPLE,
    &DEGA_SANCTUARY,
    &DEGAVOLVER,
    &DIVERSIONARY_TACTICS,
    &DIVINE_LIGHT,
    &ENLISTMENT_OFFICER,
    &FALSE_DAWN,
    &GERRARD_CAPASHEN,
    &HAUNTED_ANGEL,
    &HELIONAUT,
    &MANACLES_OF_DECAY,
    &ORIM_S_THUNDER,
    &SHIELD_OF_DUTY_AND_REASON,
    &SPECTRAL_LYNX,
    &STANDARD_BEARER,
    &CETA_DISCIPLE,
    &CETA_SANCTUARY,
    &CETAVOLVER,
    &COASTAL_DRAKE,
    &EVASIVE_ACTION,
    &ICE_CAVE,
    &INDEX,
    &JADED_RESPONSE,
    &JILT,
    &LIVING_AIRSHIP,
    &REEF_SHAMAN,
    &SHIMMERING_MIRAGE,
    &TIDAL_COURIER,
    &UNNATURAL_SELECTION,
    &VODALIAN_MYSTIC,
    &WHIRLPOOL_DRAKE,
    &WHIRLPOOL_RIDER,
    &WHIRLPOOL_WARRIOR,
    &DEAD_RINGERS,
    &DESOLATION_ANGEL,
    &FOUL_PRESENCE,
    &GRAVE_DEFILER,
    &LAST_CARESS,
    &MIND_EXTRACTION,
    &MOURNFUL_ZOMBIE,
    &NECRA_DISCIPLE,
    &NECRA_SANCTUARY,
    &NECRAVOLVER,
    &PHYREXIAN_ARENA,
    &PHYREXIAN_GARGANTUA,
    &PHYREXIAN_RAGER,
    &PLANAR_DESPAIR,
    &QUAGMIRE_DRUID,
    &SUPPRESS,
    &URBORG_UPRISING,
    &ZOMBIE_BOA,
    &BLOODFIRE_COLOSSUS,
    &BLOODFIRE_DWARF,
    &BLOODFIRE_INFUSION,
    &BLOODFIRE_KAVU,
    &DESOLATION_GIANT,
    &DWARVEN_LANDSLIDE,
    &DWARVEN_PATROL,
    &GOBLIN_RINGLEADER,
    &ILLUMINATE,
    &KAVU_GLIDER,
    &MINOTAUR_TACTICIAN,
    &RAKA_DISCIPLE,
    &RAKA_SANCTUARY,
    &RAKAVOLVER,
    &SMASH,
    &TAHNGARTH_S_GLARE,
    &TUNDRA_KAVU,
    &WILD_RESEARCH,
    &ANA_DISCIPLE,
    &ANA_SANCTUARY,
    &ANAVOLVER,
    &BOG_GNARR,
    &GAEA_S_BALANCE,
    &GLADE_GNARR,
    &KAVU_HOWLER,
    &KAVU_MAULER,
    &LAY_OF_THE_LAND,
    &PENUMBRA_BOBCAT,
    &PENUMBRA_KAVU,
    &PENUMBRA_WURM,
    &SAVAGE_GORILLA,
    &STRENGTH_OF_NIGHT,
    &SYLVAN_MESSENGER,
    &SYMBIOTIC_DEPLOYMENT,
    &TRANQUIL_PATH,
    &URBORG_ELF,
    &AETHER_MUTATION,
    &CAPTAIN_S_MANEUVER,
    &CONSUME_STRENGTH,
    &CROMAT,
    &DEATH_GRASP,
    &DEATH_MUTATION,
    &EBONY_TREEFOLK,
    &FERVENT_CHARGE,
    &FLOWSTONE_CHARGER,
    &FUNGAL_SHAMBLER,
    &GAEA_S_SKYFOLK,
    &GERRARDS_VERDICT,
    &GOBLIN_LEGIONNAIRE,
    &GOBLIN_TRENCHES,
    &GUIDED_PASSAGE,
    &JUNGLE_BARRIER,
    &LAST_STAND,
    &LIGHTNING_ANGEL,
    &LLANOWAR_DEAD,
    &MARTYRS_TOMB,
    &MINOTAUR_ILLUSIONIST,
    &MYSTIC_SNAKE,
    &OVERGROWN_ESTATE,
    &PERNICIOUS_DEED,
    &POWERSTONE_MINEFIELD,
    &PROPHETIC_BOLT,
    &PUTRID_WARRIOR,
    &QUICKSILVER_DAGGER,
    &RAZORFIN_HUNTER,
    &SOUL_LINK,
    &SPIRITMONGER,
    &SQUEE_S_EMBRACE,
    &SQUEE_S_REVENGE,
    &SUFFOCATING_BLAST,
    &TEMPORAL_SPRING,
    &VINDICATE,
    &YAVIMAYA_S_EMBRACE,
    &FIRE_ICE,
    &ILLUSION_REALITY,
    &LIFE_DEATH,
    &NIGHT_DAY,
    &ORDER_CHAOS,
    &BRASS_HERALD,
    &DODECAPOD,
    &DRAGON_ARCH,
    &EMBLAZONED_GOLEM,
    &LEGACY_WEAPON,
    &MASK_OF_INTOLERANCE,
    &BATTLEFIELD_FORGE,
    &CAVES_OF_KOILOS,
    &LLANOWAR_WASTES,
    &SHIVAN_REEF,
    &YAVIMAYA_COAST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
