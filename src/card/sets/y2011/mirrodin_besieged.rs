//! Mirrodin Besieged cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef, ManaColor, ReplacementEffectDef,
    ReplacementEventDef, SpellResolutionDestinationDef, TokenCharacteristics, ValueDef, ZoneKind,
    ZoneMoveCauseDef, abilities,
};
use crate::{TargetIndex, mana_cost};

// MBS 19 — White Sun's Zenith
pub(in crate::card::sets) static WHITE_SUNS_ZENITH: CardRecord = CardRecord::new_with_legacy_id(
    1707,
    "White Sun's Zenith",
    CardArt::new("a879940e-6632-47c5-a30e-d29a82d16e9d", "Mike Bierek"),
    CardSet::MirrodinBesieged,
    CardRules::new_instant(mana_cost!("{X}{W}{W}")).with_ability(
        AbilityDef::spell(
            "Create X 2/2 white Cat creature tokens. Shuffle White Sun's Zenith into its owner's library.",
            EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 2, 2).with_art(CardArt::new("5252ab51-43e8-4b24-9830-de0ad9b9d3dc", "Scott Chou")).with_count(ValueDef::ChosenX),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
    ),
);

/// Revealed and shuffled back rather than exiled, so the deck keeps it and
/// nothing gets to answer it permanently. The reveal is what makes the
/// shuffle honest: everyone knows the card went back in.
static COLOSSUS_RETURNS: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::MoveToZone(ZoneKind::Library),
    ReplacementEffectDef::Perform(&EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Controller,
    }),
];

/// Watched from everywhere the card can be, because "from anywhere" is the
/// point: countered on the stack, discarded from hand, and milled from the
/// library all come back the same way.
static COLOSSUS_ZONES: [ZoneKind; 5] = [
    ZoneKind::Battlefield,
    ZoneKind::Stack,
    ZoneKind::Hand,
    ZoneKind::Library,
    ZoneKind::Graveyard,
];

static COLOSSUS_ABILITIES: [AbilityDef; 4] = [
    abilities::trample(),
    abilities::infect(),
    abilities::indestructible(),
    AbilityDef::replacement_for(
        "If this creature would be put into a graveyard from anywhere, reveal it and shuffle it into its owner's library instead.",
        ReplacementEventDef::WouldMove {
            from: None,
            to: ZoneKind::Graveyard,
            cause: ZoneMoveCauseDef::Any,
        },
        ReplacementEffectDef::Sequence(&COLOSSUS_RETURNS),
    )
    .with_source_zones(&COLOSSUS_ZONES),
];

// MBS 99 — Blightsteel Colossus
pub(in crate::card::sets) static BLIGHTSTEEL_COLOSSUS: CardRecord = CardRecord::new_with_legacy_id(
    2183,
    "Blightsteel Colossus",
    CardArt::new("7928bb14-7631-4830-a756-26d1ea832ba2", "Chris Rahn"),
    CardSet::MirrodinBesieged,
    // Eleven infect damage is one hit from a win, and the deck that plays it
    // is not paying twelve mana honestly -- it is cheating it into play and
    // attacking once.
    CardRules::new_artifact_creature(mana_cost!("{12}"), &["Phyrexian", "Golem"], 11, 11)
        .with_abilities(&COLOSSUS_ABILITIES),
);

// MBS 115 — Mortarpod
pub(in crate::card::sets) static MORTARPOD: CardRecord = CardRecord::new_with_legacy_id(
    1704,
    "Mortarpod",
    CardArt::new("fbd23da5-421f-41d0-bb60-59560da7dece", "Eric Deschamps"),
    CardSet::MirrodinBesieged,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(
                TokenCharacteristics::creature(
                    &["Phyrexian", "Germ"],
                    &[ManaColor::Black],
                    0,
                    0,
                )
                .with_art(CardArt::new(
                    "65c65445-1016-4fd3-963e-1c9eb252d4a6",
                    "Igor Kieryluk",
                )),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +0/+1 and has \"Sacrifice this creature: This creature deals 1 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "Sacrifice this creature: This creature deals 1 damage to any target.",
                            &[AbilityCostDef::SacrificeSource],
                            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&WHITE_SUNS_ZENITH, &BLIGHTSTEEL_COLOSSUS, &MORTARPOD];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
