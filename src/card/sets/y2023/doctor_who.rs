//! Doctor Who cards cataloged for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WHO",
    slug: "doctor-who",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WHO 146 — The Master, Multiplied
// Audit: unsupported — Needs a player rule that prevents triggered abilities from causing sacrifice or exile of creature tokens.
pub(in crate::card::sets) static THE_MASTER_MULTIPLIED: CardRecord = CardRecord::new(
    "The Master, Multiplied",
    "7f734ca0-91bc-4496-9bd7-2d09415e850f",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// WHO 186 — Sonic Screwdriver
pub(in crate::card::sets) static SONIC_SCREWDRIVER_186: CardRecord = CardRecord::new(
    "Sonic Screwdriver",
    "2a55d514-b670-4fe2-825b-9ba955977cac",
    "Pauline Voss",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
abilities::tap_for_mana("{T}: Add one mana of any color.", AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue, ManaColor::Black, ManaColor::Red, ManaColor::Green])),
AbilityDef::activated_with_targets("{1}, {T}: Untap another target artifact.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]))], EffectDef::Untap { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) }),
AbilityDef::activated("{2}, {T}: Scry 1. (Look at the top card of your library. You may put that card on the bottom.)", &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource], abilities::scry(ValueDef::Constant(1))),
AbilityDef::activated_with_targets("{3}, {T}: Target creature can't be blocked this turn.", &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// WHO 189 — Ominous Cemetery
pub(in crate::card::sets) static OMINOUS_CEMETERY_189: CardRecord = CardRecord::new(
    "Ominous Cemetery",
    "2e843c57-fae3-4127-94e7-cad8c8bb9486",
    "Anato Finnstark",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{5}, {T}, Exile this land: Target creature's owner shuffles it into their library.",
            &[
                CostDef::Mana(mana_cost!("{5}")),
                CostDef::TapSource,
                CostDef::ExileSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                },
            ]),
        ),
    ]),
);

// WHO 353 — Auton Soldier
pub(in crate::card::sets) static AUTON_SOLDIER_353: CardRecord = CardRecord::new(
    "Auton Soldier",
    "2697012c-81a6-48e7-96cd-e6745078e0d9",
    "Greg Opalinski",
    CardRules::new_artifact_creature(mana_cost!("{4}{U}{U}"), &["Alien", "Soldier"], 0, 0).with_abilities(&[
AbilityDef::replacement("You may have this creature enter as a copy of any creature on the battlefield, except it isn't legendary, is an artifact in addition to its other types, and has myriad. (Whenever it attacks, for each opponent other than defending player, you may create a token copy that's tapped and attacking that player or a planeswalker they control. Exile the tokens at end of combat.)", ReplacementEffectDef::CopyEntering { object: ObjectPredicateDef::HasType(CardType::Creature), exceptions: CopyExceptionsDef::NONE.without_supertypes(&[CardSupertype::Legendary]).with_added_types(CardTypeSet::single(CardType::Artifact)).with_abilities(&[CopyAbilityDef::Ability(&abilities::myriad())]) })
]),
);

// WHO 359 — Flesh Duplicate
// Audit: unsupported — CopyEntering has fixed copy exceptions. It cannot condition the added vanishing ability on whether the chosen original already has vanishing.
pub(in crate::card::sets) static FLESH_DUPLICATE_359: CardRecord = CardRecord::new(
    "Flesh Duplicate",
    "0ebb1d26-13d1-4907-95d6-8766b233fc73",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// WHO 408 — Dinosaurs on a Spaceship
// Audit: unsupported — CountersRemoved emits one grouped event for a removal of several counters. This card must trigger once for each time counter removed, so creating that many tokens in one trigger would change copying, countering, and trigger-order behavior.
pub(in crate::card::sets) static DINOSAURS_ON_A_SPACESHIP_408: CardRecord = CardRecord::new(
    "Dinosaurs on a Spaceship",
    "391e1747-69b0-47e0-9210-317f5febca59",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// WHO 707 — The Foretold Soldier
// Audit: unsupported — Exiling a card face down does not mark it as foretold. The shared foretell special action owns that state, and no resolving effect can mark this self-exile as foretold.
pub(in crate::card::sets) static THE_FORETOLD_SOLDIER_707: CardRecord = CardRecord::new(
    "The Foretold Soldier",
    "923ce269-f8a6-4d4b-adbf-463241a001b0",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WHO 745 — Last Night Together
// Audit: unsupported — Additional combat scheduling is supported, but no phase-scoped attack restriction can retain just the two selected creatures as the allowed attackers for the newly scheduled combat.
pub(in crate::card::sets) static LAST_NIGHT_TOGETHER_745: CardRecord = CardRecord::new(
    "Last Night Together",
    "08fbaf77-8afa-41c7-804b-ba7b740fffe8",
    "Pierre Loyvet",
    crate::card::CardRules::unsupported(),
);

// WHO 1053 — Gallifrey Council Chamber
// Audit: unsupported — Mana restrictions are conjunctive. No restriction permits either casting a Time Lord or Alien spell or activating an ability of such a source while excluding unrelated payments.
pub(in crate::card::sets) static GALLIFREY_COUNCIL_CHAMBER_1053: CardRecord = CardRecord::new(
    "Gallifrey Council Chamber",
    "0e03370a-05a7-4f84-aadd-9eba459e2696",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_MASTER_MULTIPLIED,
    &SONIC_SCREWDRIVER_186,
    &OMINOUS_CEMETERY_189,
    &AUTON_SOLDIER_353,
    &FLESH_DUPLICATE_359,
    &DINOSAURS_ON_A_SPACESHIP_408,
    &THE_FORETOLD_SOLDIER_707,
    &LAST_NIGHT_TOGETHER_745,
    &GALLIFREY_COUNCIL_CHAMBER_1053,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
