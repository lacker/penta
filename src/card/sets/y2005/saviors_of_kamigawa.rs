//! SOK card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardNameDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SOK",
    slug: "saviors-of-kamigawa",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SOK 2 — Araba Mothrider
pub(in crate::card::sets) static ARABA_MOTHRIDER: CardRecord = CardRecord::new(
    "Araba Mothrider",
    "f6e4a170-1075-47e4-abe6-996b161573c1",
    "Anthony S. Waters",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Samurai"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::bushido(ValueDef::Constant(1)),
    ]),
);

// SOK 38 — Freed from the Real
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREED_FROM_THE_REAL_38: CardRecord = CardRecord::new(
    "Freed from the Real",
    "e9ecee02-12c0-4aed-a679-41bce95e0cda",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// SOK 48 — Oboro Breezecaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBORO_BREEZECALLER_48: CardRecord = CardRecord::new(
    "Oboro Breezecaller",
    "a382a9b8-0b19-46c2-a547-a22d6e23d0ac",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// SOK 53 — Sakashima the Impostor
static SAKASHIMA_RETURN: AbilityDef = AbilityDef::activated(
    "{2}{U}{U}: Return Sakashima the Impostor to its owner's hand \
     at the beginning of the next end step.",
    &[CostDef::Mana(mana_cost!("{2}{U}{U}"))],
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, return Sakashima the \
         Impostor to its owner's hand.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::move_to_zone(
            EffectRecipientDef::Source,
            crate::card::ZoneKind::Hand,
            crate::card::ZonePlacement::Top,
        ),
    ))),
);

pub(in crate::card::sets) static SAKASHIMA_THE_IMPOSTOR: CardRecord = CardRecord::new(
    "Sakashima the Impostor",
    "61dc2f54-3637-4caa-9741-36ff14dc5527",
    "rk post",
CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Rogue"], 3, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::replacement(
            "You may have Sakashima the Impostor enter as a copy of any creature on the battlefield, except its name is Sakashima the Impostor, it's legendary in addition to its other types, and it has \"{2}{U}{U}: Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.\"",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                exceptions: CopyExceptionsDef::NONE
                    .with_name("Sakashima the Impostor")
                    .with_added_supertypes(&[CardSupertype::Legendary])
                    .with_abilities(&[CopyAbilityDef::Ability(&SAKASHIMA_RETURN)]),
            },
            ),
            SAKASHIMA_RETURN,
        ]),
);

// SOK 60 — Twincast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWINCAST_60: CardRecord = CardRecord::new(
    "Twincast",
    "1a367559-1d84-4f6f-9e6e-ff90de420389",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// SOK 63 — Death Denied
pub(in crate::card::sets) static DEATH_DENIED: CardRecord = CardRecord::new(
    "Death Denied",
    "8f66ddc5-f5e6-44de-8189-87b6521d1fea",
    "Greg Hildebrandt",
    // Two black on top of X is a real tax, which is why this is a late-game
    // rebuild rather than a way to buy back one creature.
    CardRules::new_instant(mana_cost!("{X}{B}{B}"))
        .with_subtypes(&["Arcane"])
        .with_ability(AbilityDef::spell_with_targets(
            "Return X target creature cards from your graveyard to your hand.",
            // Exactly X, not up to X: a graveyard with fewer creatures than
            // the X paid for cannot legally cast it.
            &[AbilityTargetDef::exactly_value(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                ValueDef::ChosenX,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )),
);

// SOK 102 — Hidetsugu's Second Rite
// Audit: unsupported — Needs a value expression reading the life total of an indexed target player; LifeTotal accepts source and event player relations but cannot resolve a target slot.
pub(in crate::card::sets) static HIDETSUGU_S_SECOND_RITE: CardRecord = CardRecord::new(
    "Hidetsugu's Second Rite",
    "2e48eb77-3bd7-444a-9262-799cc706c05a",
    "Jeff Miracola",
    CardRules::unsupported(),
);

// SOK 104 — Iizuka the Ruthless
pub(in crate::card::sets) static IIZUKA_THE_RUTHLESS: CardRecord = CardRecord::new(
    "Iizuka the Ruthless",
    "9ce461f7-385d-4379-83de-49571247c30d",
    "Darrell Riche",
CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Samurai"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::bushido(ValueDef::Constant(2)),
            AbilityDef::activated(
                "{2}{R}, Sacrifice a Samurai: Samurai creatures you control gain double strike until end of turn.",
                &[
                    CostDef::Mana(mana_cost!("{2}{R}")),
                    CostDef::SacrificePermanent {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Samurai")),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Samurai")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// SOK 118 — Thoughts of Ruin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTS_OF_RUIN_118: CardRecord = CardRecord::new(
    "Thoughts of Ruin",
    "2a0f2db3-41a6-4283-9812-46b6ae6d1df6",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SOK 147 — Seek the Horizon
pub(in crate::card::sets) static SEEK_THE_HORIZON: CardRecord = CardRecord::new(
    "Seek the Horizon",
    "49f8a9e7-f505-4fc5-b820-0af1ee1960c7",
    "Eric Polak",
CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to three basic land reveal them, put them into your hand, then shuffle.",
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
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// SOK 158 — Pithing Needle
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new(
    "Pithing Needle",
    "78eb9e1d-113e-45ff-8435-32ee42fa5631",
    "Pete Venters",
CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, choose a card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("pithing_needle_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::AllCardNames,
                ),
            },
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            CardNameDef::Binding(Binding!("pithing_needle_name")),
        ),
    ]),
);

// SOK 162 — Mikokoro, Center of the Sea
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIKOKORO_CENTER_OF_THE_SEA_162: CardRecord = CardRecord::new(
    "Mikokoro, Center of the Sea",
    "7ef72797-328e-4303-8ffb-9686086648b8",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// SOK 164 — Oboro, Palace in the Clouds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBORO_PALACE_IN_THE_CLOUDS_164: CardRecord = CardRecord::new(
    "Oboro, Palace in the Clouds",
    "ffc2d68e-6543-43ec-b67a-afff1325a32f",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARABA_MOTHRIDER,
    &FREED_FROM_THE_REAL_38,
    &OBORO_BREEZECALLER_48,
    &SAKASHIMA_THE_IMPOSTOR,
    &TWINCAST_60,
    &DEATH_DENIED,
    &HIDETSUGU_S_SECOND_RITE,
    &IIZUKA_THE_RUTHLESS,
    &THOUGHTS_OF_RUIN_118,
    &SEEK_THE_HORIZON,
    &PITHING_NEEDLE,
    &MIKOKORO_CENTER_OF_THE_SEA_162,
    &OBORO_PALACE_IN_THE_CLOUDS_164,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
