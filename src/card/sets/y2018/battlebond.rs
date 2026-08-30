//! Battlebond cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AppliedEffectDef, AttackEventMatcherDef, CardArt, CardRules,
    CardSet, CardSupertype, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectRefDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// BBD 41 — Spellseeker
pub(in crate::card::sets) static SPELLSEEKER: CardRecord = CardRecord::new_with_legacy_id(
    2150,
    "Spellseeker",
    CardArt::new("74b4c336-5d4c-4bc5-b82a-35084a6ad808", "Igor Kieryluk"),
    CardSet::Battlebond,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        abilities::enters_trigger("When this creature enters, you may search your library for an instant or sorcery card with mana value 2 or less, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // A cheap instant or sorcery: the body is beside the point, and what it
                    // fetches is whichever answer the board is asking for.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        ObjectPredicateDef::ManaValueAtMost(2),
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
            }),
    ),
);

// BBD 71 — Grothama, All-Devouring
// Audit: partial — The attack-triggered fights are implemented; the leaves-the-battlefield ability needs per-recipient damage amounts grouped by each source's controller.
pub(in crate::card::sets) static GROTHAMA_ALL_DEVOURING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab8935b1-ec87-4330-9952-9ef8cd344531"),
    "Grothama, All-Devouring",
    CardArt::new("ab8935b1-ec87-4330-9952-9ef8cd344531", "Mark Behm"),
    CardSet::Battlebond,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wurm"], 10, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other creatures have \"Whenever this creature attacks, you may have it fight Grothama, All-Devouring.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature attacks, you may have it fight Grothama, All-Devouring.",
                        TriggerEventDef::Attacks(AttackEventMatcherDef::any(ObjectPredicateDef::Source)),
                        EffectDef::May {
                            player: EffectRecipientDef::Controller,
                            effect: &EffectDef::Fight {
                                first: ObjectRefDef::Source,
                                second: ObjectRefDef::AbilityGrantSource,
                                excess: None,
                            },
                        },
                    )),
                },
            ),
            AbilityDef::triggered(
                "When Grothama leaves the battlefield, each player draws cards equal to the amount of damage dealt to Grothama this turn by sources they controlled.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::Special("draw per player from damage dealt to the departed source"),
            )
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Needs per-recipient damage amounts grouped by each source's controller.",
            )),
        ]),
);

// BBD 209 — Pulse of Murasa
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PULSE_OF_MURASA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0c8057f-b45b-4f67-90cd-c808b5e9cbfa"),
    "Pulse of Murasa",
    crate::card::CardArt::new("c591c615-69e8-4661-a089-8c4e152adac7", "Matt Stewart"),
    crate::card::CardSet::Battlebond,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SPELLSEEKER, &GROTHAMA_ALL_DEVOURING, &PULSE_OF_MURASA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
