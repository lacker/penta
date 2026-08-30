//! Shadowmoor cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, ResolvedEffectDurationDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// SHM 57 — Beseech the Queen
pub(in crate::card::sets) static BESEECH_THE_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64ee0a93-0f6d-42be-bdca-1de5422d8d54"),
    "Beseech the Queen",
    CardArt::new("64ee0a93-0f6d-42be-bdca-1de5422d8d54", "Jason Chan"),
    CardSet::Shadowmoor,
    CardRules::new_sorcery(mana_cost!("{2/B}{2/B}{2/B}")).with_ability(AbilityDef::spell(
        "Search your library for a card with mana value less than or equal to the number of lands you control, reveal it, put it into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::CountMatchingObjects(
                // The lands the caster controls when Beseech the Queen resolves.
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
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

// SHM 135 — Woodfall Primus
pub(in crate::card::sets) static WOODFALL_PRIMUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43aa7e35-55ee-4e02-a8aa-ea2b267055d1"),
    "Woodfall Primus",
    CardArt::new("43aa7e35-55ee-4e02-a8aa-ea2b267055d1", "Adam Rex"),
    CardSet::Shadowmoor,
    // Eight mana for two Naturalizes and a trampling body that has to be
    // answered twice.
    CardRules::new_creature(mana_cost!("{5}{G}{G}{G}"), &["Treefolk", "Shaman"], 6, 6)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy target noncreature permanent.",
                // A noncreature permanent: lands and artifacts above all, which is what
                // eight mana of Treefolk is being paid to answer twice.
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            abilities::persist(),
        ]),
);

// SHM 211 — Manamorphose
pub(in crate::card::sets) static MANAMORPHOSE: CardRecord = CardRecord::new_with_legacy_id(
    2238,
    "Manamorphose",
    CardArt::new("50283122-b8c4-4fb3-8eba-6252b72222f4", "Jeff Miracola"),
    CardSet::Shadowmoor,
    // It costs nothing and does nothing, which is the point: the deck that
    // wants it wants a spell that replaces itself and moves the storm count.
    CardRules::new_instant(mana_cost!("{1}{R/G}")).with_ability(AbilityDef::spell(
        "Add two mana in any combination of colors.\nDraw a card.",
        // "In any combination of colors" is one question per mana rather than one
        // for the pair, which is what lets it fix two colours at once.
        EffectDef::Sequence(&[
            EffectDef::AddMana(AddManaEffectDef::combination(
                &[
                    ManaColor::White,
                    ManaColor::Blue,
                    ManaColor::Black,
                    ManaColor::Red,
                    ManaColor::Green,
                ],
                2,
            )),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// SHM 224 — Barkshell Blessing
// Audit: partial — Conspire's creature-tapping cost and spell-copy trigger are not modeled; the targeted +2/+2 effect is executable.
pub(in crate::card::sets) static BARKSHELL_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69"),
    "Barkshell Blessing",
    CardArt::new("cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69", "Steven Belledin"),
    CardSet::Shadowmoor,
    CardRules::new_instant(mana_cost!("{G/W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.\nConspire (As you cast this spell, you may tap two untapped creatures you control that share a color with it. When you do, copy it and you may choose a new target for the copy.)",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The targeted +2/+2 effect is executable. Conspire's creature-tapping cast cost and spell-copy trigger are not modeled.",
        )),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BESEECH_THE_QUEEN,
    &WOODFALL_PRIMUS,
    &MANAMORPHOSE,
    &BARKSHELL_BLESSING,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
