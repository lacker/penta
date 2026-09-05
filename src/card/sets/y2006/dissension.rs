//! DIS card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, BasicLandType,
    BattlefieldEntryScalarChoiceDef, CardArt, CardRules, CardSet, CardType, ClassifyObjectsDef,
    EffectDef, KeywordAbility, ManaColor, ManaTypeDef, MoveObjectsDef, ObjectPredicateDef,
    ObjectSetDef, PlayerRefDef, ReplacementChoiceDef, ReplacementEffectDef, RevealObjectsDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{Binding, ParentBinding};
use crate::mana_cost;

// DIS 10 — Guardian of the Guildpact
pub(in crate::card::sets) static GUARDIAN_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e"),
    "Guardian of the Guildpact",
    CardArt::new("c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e", "Wayne England"),
    CardSet::Dissension,
    // Nearly unkillable and nearly unblockable in a two-colour format: only
    // a gold or colourless source touches it, which is the whole card.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit"], 2, 3)
        .with_ability(abilities::protection_from_monocolored()),
);

// DIS 99 — Utopia Sprawl
pub(in crate::card::sets) static UTOPIA_SPRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5047e271-fbf1-402c-9eb9-0806e5988f76"),
    "Utopia Sprawl",
    CardArt::new("5047e271-fbf1-402c-9eb9-0806e5988f76", "Ron Spears"),
    CardSet::Dissension,
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
    PrintingAnchor::scryfall("b675c1e6-add5-4959-a5be-f2571ccebcb4"),
    "Azorius First-Wing",
    CardArt::new(
        "b675c1e6-add5-4959-a5be-f2571ccebcb4",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Dissension,
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
    PrintingAnchor::scryfall("0c7b0fa1-bfc2-4b15-80ea-47e41a17aa2c"),
    "Coiling Oracle",
    CardArt::new("55a6ba2a-b372-4b15-9a1e-09b41316eab7", "Mark Zug"),
    CardSet::Dissension,
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

// DIS 178 — Rakdos Carnarium
pub(in crate::card::sets) static RAKDOS_CARNARIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34f146f3-6541-4d2a-96e3-a3cd680c0a1e"),
    "Rakdos Carnarium",
    CardArt::new("34f146f3-6541-4d2a-96e3-a3cd680c0a1e", "John Avon"),
    CardSet::Dissension,
    // The black-red karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {B}{R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Black,
                ManaColor::Red,
            )),
        ),
    ]),
);

// DIS 180 — Simic Growth Chamber
pub(in crate::card::sets) static SIMIC_GROWTH_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("407d0a0c-a6be-4bd5-8355-1715698c6bde"),
    "Simic Growth Chamber",
    CardArt::new("407d0a0c-a6be-4bd5-8355-1715698c6bde", "John Avon"),
    CardSet::Dissension,
    // The green-blue karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {G}{U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Green,
                ManaColor::Blue,
            )),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GUARDIAN_OF_THE_GUILDPACT,
    &UTOPIA_SPRAWL,
    &AZORIUS_FIRST_WING,
    &COILING_ORACLE,
    &RAKDOS_CARNARIUM,
    &SIMIC_GROWTH_CHAMBER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
