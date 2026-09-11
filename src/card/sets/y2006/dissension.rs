//! DIS card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityTargetPredicate;
use crate::CardSupertype;
use crate::DiscardSelectionDef;
use crate::EffectRecipientDef;
use crate::PlayerRelation;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ClassifyObjectsDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::RevealObjectsDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::Binding;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DIS",
    slug: "dissension",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// DIS 10 — Guardian of the Guildpact
pub(in crate::card::sets) static GUARDIAN_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    "Guardian of the Guildpact",
    "c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e",
    "Fred Hooper",
    // Nearly unkillable and nearly unblockable in a two-colour format: only
    // a gold or colourless source touches it, which is the whole card.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit"], 2, 3)
        .with_ability(abilities::protection_from_monocolored()),
);

// DIS 58 — Wit's End
pub(in crate::card::sets) static WITS_END: CardRecord = CardRecord::new(
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
pub(in crate::card::sets) static UTOPIA_SPRAWL: CardRecord = CardRecord::new(
    "Utopia Sprawl",
    "5047e271-fbf1-402c-9eb9-0806e5988f76",
    "Ron Spears",
    // One mana of ramp that also fixes, at the cost of only ever going on a
    // Forest -- which is why it is a green deck's card and nobody else's.
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            // "Enchant Forest" is narrower than enchant land: the basic land
            // type, not the card type, so a dual with Forest on it qualifies
            // and a nonbasic without it does not.
            abilities::aura_spell(
                "Enchant Forest",
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    )]
                },
            ),
            AbilityDef::as_enters(
                "As this Aura enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::triggered_mana(
                "Whenever enchanted Forest is tapped for mana, its controller adds an additional \
                 one mana of the chosen color.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                // The land's controller, not the Aura's.
                EffectDef::AddMana(
                    AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// DIS 105 — Azorius First-Wing
pub(in crate::card::sets) static AZORIUS_FIRST_WING: CardRecord = CardRecord::new(
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
const ORACLE_LAND: Binding = Binding!("oracle_land");
const ORACLE_NONLAND: Binding = Binding!("oracle_nonland");

pub(in crate::card::sets) static COILING_ORACLE: CardRecord = CardRecord::new(
    "Coiling Oracle",
    "55a6ba2a-b372-4b15-9a1e-09b41316eab7",
    "Mark Zug",
// Either a land drop or a card, decided by the top of the library
    // rather than by its controller -- which is why it is a ramp spell in a
    // land-heavy deck and a cantrip in every other one.
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Snake", "Elf", "Druid"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, reveal the top card of your library. If it's a land card, put it onto the battlefield. Otherwise, put that card into your hand.",
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(1),
                &const {
                    EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                        // One card split into two bindings, exactly one of
                        // which is nonempty, so both moves below can run
                        // unconditionally.
                        EffectDef::ClassifyObjects(ClassifyObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            object: ObjectPredicateDef::HasType(CardType::Land),
                            matching: ORACLE_LAND,
                            remainder: ORACLE_NONLAND,
                            then: &const {
                                EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(ORACLE_LAND),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Battlefield,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(ORACLE_NONLAND),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Hand,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                ])
                            },
                        }),
                    ])
                },
            ),
        ),
    ),
);

// DIS 170 — Azorius Chancery
pub(in crate::card::sets) static AZORIUS_CHANCERY: CardRecord = CardRecord::new(
    "Azorius Chancery",
    "e58365d2-e4db-444b-b1a9-795668ad3038",
    "John Avon",
    // The blue-white karoo. Only the two colours below are its own; the rest
    // of the cycle prints the same two clauses word for word.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {W}{U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::White,
                ManaColor::Blue,
            )),
        ),
    ]),
);

// DIS 171 — Blood Crypt
pub(in crate::card::sets) static BLOOD_CRYPT: CardRecord = CardRecord::new(
    "Blood Crypt",
    "f281e16f-0fe1-4095-bd63-0a4479f75c11",
    "Rob Alexander",
    CardRules::new_land(&["Swamp", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// DIS 172 — Breeding Pool
pub(in crate::card::sets) static BREEDING_POOL: CardRecord = CardRecord::new(
    "Breeding Pool",
    "b98b2a35-ec2b-47fe-903d-dd292e469a3c",
    "Rob Alexander",
    CardRules::new_land(&["Forest", "Island"]).with_ability(abilities::shock_land_enters()),
);

// DIS 173 — Ghost Quarter
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    "Ghost Quarter",
    "893eb7e4-5d8d-477b-aaa7-fb85ef2a54fc",
    "Heather Hudson",
CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.", &[CostDef::TapSource, CostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )], EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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
    "Hallowed Fountain",
    "c28aea19-2a39-4934-afda-909e234fa3ba",
    "Rob Alexander",
    CardRules::new_land(&["Plains", "Island"]).with_ability(abilities::shock_land_enters()),
);

// DIS 178 — Rakdos Carnarium
pub(in crate::card::sets) static RAKDOS_CARNARIUM: CardRecord = CardRecord::new(
    "Rakdos Carnarium",
    "34f146f3-6541-4d2a-96e3-a3cd680c0a1e",
    "John Avon",
    // The black-red karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {B}{R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Black,
                ManaColor::Red,
            )),
        ),
    ]),
);

// DIS 180 — Simic Growth Chamber
pub(in crate::card::sets) static SIMIC_GROWTH_CHAMBER: CardRecord = CardRecord::new(
    "Simic Growth Chamber",
    "407d0a0c-a6be-4bd5-8355-1715698c6bde",
    "John Avon",
    // The green-blue karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {G}{U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Green,
                ManaColor::Blue,
            )),
        ),
    ]),
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
