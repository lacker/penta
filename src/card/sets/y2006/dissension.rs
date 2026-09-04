//! DIS card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::CardSupertype;
use crate::DiscardSelectionDef;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ManaColor;
use crate::PlayerRelation;
use crate::TargetIndex;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::{
    AbilityDef, CardRules, CardSet, CardType, KeywordAbility, ObjectPredicateDef, abilities,
};
use crate::mana_cost;

// DIS 10 — Guardian of the Guildpact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Guardian of the Guildpact",
    "c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e",
    "Fred Hooper",
    crate::card::CardRules::unsupported(),
);

// DIS 58 — Wit's End
pub(in crate::card::sets) static WITS_END: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Wit's End",
    "68f8e20c-6d8e-45a1-aabd-176d8df843db",
    "Kev Walker",
    CardRules::new_sorcery(mana_cost!("{5}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards their hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(i32::MAX),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// DIS 99 — Utopia Sprawl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTOPIA_SPRAWL: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Utopia Sprawl",
    "5047e271-fbf1-402c-9eb9-0806e5988f76",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// DIS 105 — Azorius First-Wing
pub(in crate::card::sets) static AZORIUS_FIRST_WING: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Azorius First-Wing",
    "b675c1e6-add5-4959-a5be-f2571ccebcb4",
    "Alex Horley-Orlandelli",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Griffin"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::keyword(
            "Protection from enchantments",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Enchantment)),
        ),
    ]),
);

// DIS 107 — Coiling Oracle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COILING_ORACLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Coiling Oracle",
    "55a6ba2a-b372-4b15-9a1e-09b41316eab7",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// DIS 170 — Azorius Chancery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AZORIUS_CHANCERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Azorius Chancery",
    "e58365d2-e4db-444b-b1a9-795668ad3038",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// DIS 171 — Blood Crypt
pub(in crate::card::sets) static BLOOD_CRYPT: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Blood Crypt",
    "f281e16f-0fe1-4095-bd63-0a4479f75c11",
    "Rob Alexander",
    CardRules::new_land(&["Swamp", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// DIS 172 — Breeding Pool
pub(in crate::card::sets) static BREEDING_POOL: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Breeding Pool",
    "b98b2a35-ec2b-47fe-903d-dd292e469a3c",
    "Rob Alexander",
    CardRules::new_land(&["Forest", "Island"]).with_ability(abilities::shock_land_enters()),
);

// DIS 173 — Ghost Quarter
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Ghost Quarter",
    "893eb7e4-5d8d-477b-aaa7-fb85ef2a54fc",
    "Heather Hudson",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.", &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )], EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
                // Declining the printed "may" skips the entire search, including
                // its shuffle. If accepted, the qualified hidden-zone search
                // may still legally fail to find. The controller is read after
                // destruction from last-known information.
                EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
                },
            ])),
    ]),
);

// DIS 174 — Hallowed Fountain
pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new(
    CardSet::Dissension,
    "Hallowed Fountain",
    "c28aea19-2a39-4934-afda-909e234fa3ba",
    "Rob Alexander",
    CardRules::new_land(&["Plains", "Island"]).with_ability(abilities::shock_land_enters()),
);

// DIS 178 — Rakdos Carnarium
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKDOS_CARNARIUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Rakdos Carnarium",
    "34f146f3-6541-4d2a-96e3-a3cd680c0a1e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// DIS 180 — Simic Growth Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIMIC_GROWTH_CHAMBER: CardRecord = CardRecord::new(
    crate::card::CardSet::Dissension,
    "Simic Growth Chamber",
    "407d0a0c-a6be-4bd5-8355-1715698c6bde",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GUARDIAN_OF_THE_GUILDPACT,
    &WITS_END,
    &UTOPIA_SPRAWL,
    &AZORIUS_FIRST_WING,
    &COILING_ORACLE,
    &AZORIUS_CHANCERY,
    &BLOOD_CRYPT,
    &BREEDING_POOL,
    &GHOST_QUARTER,
    &HALLOWED_FOUNTAIN,
    &RAKDOS_CARNARIUM,
    &SIMIC_GROWTH_CHAMBER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
