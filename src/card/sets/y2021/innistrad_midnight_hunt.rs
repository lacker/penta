//! Innistrad: Midnight Hunt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, CounterKind, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// MID 1 — Adeline, Resplendent Cathar
pub(in crate::card::sets) static ADELINE_RESPLENDENT_CATHAR: CardRecord =
    CardRecord::new_with_legacy_id(
        2280,
        "Adeline, Resplendent Cathar",
        CardArt::new("18092f68-b96e-4084-9eba-b240d2195d81", "Bryan Sola"),
        CardSet::InnistradMidnightHunt,
        // Three mana that attacks for four the turn after it lands and for more
        // every turn after that, because each token it makes makes it bigger.
        CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 0, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::vigilance(),
                AbilityDef::static_ability(
                    "Adeline's power is equal to the number of creatures you control.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        // Adeline is a creature you control, so she counts herself, and
                        // every token she makes adds one more before damage. The count
                        // defines her power rather than adding to it, which is why it
                        // also answers in a hand or a graveyard.
                        effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                            &ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                    },
                ),
                // The token was never declared as an attacker, so nothing watching a
                // declaration sees it -- and with two players the one opponent is the
                // only thing it could be attacking.
                AbilityDef::triggered(
                    "Whenever you attack, for each opponent, create a 1/1 white Human creature token that's \
                     tapped and attacking that player or a planeswalker they control.",
                    // "Whenever you attack" is one or more creatures you control attacking,
                    // counted once for the declaration rather than once per attacker.
                    TriggerEventDef::attack_declared(
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        1,
                        None,
                    ),
                    EffectDef::create_creature_token(&["Human"], &[ManaColor::White], 1, 1)
                        .with_art(CardArt::new(
                            "7d13a93a-a43d-4cf5-8300-8341f3b7f1b1",
                            "Miguel Mercado",
                        ))
                        .entering_tapped()
                        .entering_attacking(),
                ),
            ]),
    );

// MID 10 — Cathar Commando
pub(in crate::card::sets) static CATHAR_COMMANDO: CardRecord = CardRecord::new_with_legacy_id(
    2273,
    "Cathar Commando",
    CardArt::new("98cbc1c2-b76e-4da3-aa43-00e10b2ce532", "Evyn Fong"),
    CardSet::InnistradMidnightHunt,
    // Flash is what makes the two halves one card: it can be held up as
    // removal and cashed in as a 3/1 when nothing needs killing.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 3, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// MID 24 — Homestead Courage
pub(in crate::card::sets) static HOMESTEAD_COURAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73a9c49f-fcd3-4572-bac7-6eb06fdc0815"),
    "Homestead Courage",
    CardArt::new("73a9c49f-fcd3-4572-bac7-6eb06fdc0815", "Colin Boyer"),
    CardSet::InnistradMidnightHunt,
    // A counter is permanent where the vigilance is not, so the second cast
    // out of the graveyard is what the card is really priced on.
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature you control. It gains vigilance until end of \
             turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{W}")),
    ]),
);

// MID 32 — Search Party Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARCH_PARTY_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964"),
    "Search Party Captain",
    crate::card::CardArt::new("cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964", "Mike Bierek"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 44 — Consider
pub(in crate::card::sets) static CONSIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b3f40a0-5f58-4157-aed9-b1a52e922c3c"),
    "Consider",
    CardArt::new("a211d505-4d40-4914-a9da-220770d6ddbc", "Zezhou Chen"),
    CardSet::InnistradMidnightHunt,
    // One mana to see two cards deep and choose which of them the deck is
    // better off having in the graveyard.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Surveil 1. (Look at the top card of your library. You may put it into your graveyard.)\n\
         Draw a card.",
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::Constant(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// MID 96 — Diregraf Horde
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIREGRAF_HORDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("153be768-ddad-44f2-bcdd-c40353c807d7"),
    "Diregraf Horde",
    crate::card::CardArt::new("153be768-ddad-44f2-bcdd-c40353c807d7", "Alex Negrea"),
    crate::card::CardSet::InnistradMidnightHunt,
    crate::card::CardRules::unsupported(),
);

// MID 100 — Ecstatic Awakener // Awoken Demon
pub(in crate::card::sets) static ECSTATIC_AWAKENER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("bbdad18e-e262-41f9-b252-1cbdcdd1b5f9"),
    "Ecstatic Awakener // Awoken Demon",
    CardArt::new("bbdad18e-e262-41f9-b252-1cbdcdd1b5f9", "Tuan Duong Chu"),
    CardSet::InnistradMidnightHunt,
    // A one-drop that turns a spare body into a card and a 4/4, which is
    // what a sacrifice deck wants from its cheapest slot.
    &[
        (
            "Ecstatic Awakener",
            CardRules::new_creature(mana_cost!("{B}"), &["Human", "Wizard"], 1, 1).with_ability(
                AbilityDef::activated(
                    "{2}{B}, Sacrifice another creature: Draw a card, then transform this \
                     creature. Activate only once each turn.",
                    &[
                        AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                        // "Another creature": this one is transforming rather
                        // than dying, so it cannot pay for its own ability.
                        AbilityCostDef::SacrificePermanent {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            controller: PlayerRelation::You,
                        },
                    ],
                    EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    ]),
                )
                .once_each_turn(),
            ),
        ),
        (
            "Awoken Demon",
            CardRules::new_creature_without_mana_cost(&["Demon"], 4, 4)
                .printed_colors(&[ManaColor::Black]),
        ),
    ],
);

// MID 107 — Infernal Grasp
pub(in crate::card::sets) static INFERNAL_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17824929-f131-4b8d-addb-66c25323155e"),
    "Infernal Grasp",
    CardArt::new("17824929-f131-4b8d-addb-66c25323155e", "Naomi Baker"),
    CardSet::InnistradMidnightHunt,
    // Two mana, no restriction on what it answers, and the two life is the
    // whole of the price.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. You lose 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The life is part of the resolution rather than a cost, so a target that
        // survives being destroyed still costs it -- and a Grasp that never
        // resolves at all costs nothing.
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// MID 128 — Ardent Elementalist
pub(in crate::card::sets) static ARDENT_ELEMENTALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f58592f7-1df5-428d-9dde-e6acd9a5d1d5"),
    "Ardent Elementalist",
    CardArt::new("f58592f7-1df5-428d-9dde-e6acd9a5d1d5", "Miguel Mercado"),
    CardSet::InnistradMidnightHunt,
    // Archaeomancer's trigger in red, on a body that trades rather than
    // blocks: the card it buys back is the whole reason to cast it.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Shaman"], 2, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target instant or sorcery card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADELINE_RESPLENDENT_CATHAR,
    &CATHAR_COMMANDO,
    &HOMESTEAD_COURAGE,
    &SEARCH_PARTY_CAPTAIN,
    &CONSIDER,
    &DIREGRAF_HORDE,
    &ECSTATIC_AWAKENER,
    &INFERNAL_GRASP,
    &ARDENT_ELEMENTALIST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
