//! Innistrad: Midnight Hunt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// MID 1 — Adeline, Resplendent Cathar
pub(in crate::card::sets) static ADELINE_RESPLENDENT_CATHAR: CardRecord =
    CardRecord::new(
        CardSet::InnistradMidnightHunt,
    "Adeline, Resplendent Cathar",
    "18092f68-b96e-4084-9eba-b240d2195d81",
    "Bryan Sola",
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
pub(in crate::card::sets) static CATHAR_COMMANDO: CardRecord = CardRecord::new(
    CardSet::InnistradMidnightHunt,
    "Cathar Commando",
    "98cbc1c2-b76e-4da3-aa43-00e10b2ce532",
    "Evyn Fong",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOMESTEAD_COURAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::InnistradMidnightHunt,
    "Homestead Courage",
    "73a9c49f-fcd3-4572-bac7-6eb06fdc0815",
    "Colin Boyer",
    crate::card::CardRules::unsupported(),
);

// MID 32 — Search Party Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARCH_PARTY_CAPTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::InnistradMidnightHunt,
    "Search Party Captain",
    "cb9006c1-2e6f-4bca-a1c4-3cf2a8b6e964",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// MID 44 — Consider
pub(in crate::card::sets) static CONSIDER: CardRecord = CardRecord::new(
    CardSet::InnistradMidnightHunt,
    "Consider",
    "a211d505-4d40-4914-a9da-220770d6ddbc",
    "Zezhou Chen",
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
    crate::card::CardSet::InnistradMidnightHunt,
    "Diregraf Horde",
    "153be768-ddad-44f2-bcdd-c40353c807d7",
    "Alex Negrea",
    crate::card::CardRules::unsupported(),
);

// MID 100 — Ecstatic Awakener // Awoken Demon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECSTATIC_AWAKENER: CardRecord = CardRecord::new(
    crate::card::CardSet::InnistradMidnightHunt,
    "Ecstatic Awakener",
    "bbdad18e-e262-41f9-b252-1cbdcdd1b5f9",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// MID 107 — Infernal Grasp
pub(in crate::card::sets) static INFERNAL_GRASP: CardRecord = CardRecord::new(
    CardSet::InnistradMidnightHunt,
    "Infernal Grasp",
    "17824929-f131-4b8d-addb-66c25323155e",
    "Naomi Baker",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_ELEMENTALIST: CardRecord = CardRecord::new(
    crate::card::CardSet::InnistradMidnightHunt,
    "Ardent Elementalist",
    "f58592f7-1df5-428d-9dde-e6acd9a5d1d5",
    "Miguel Mercado",
    crate::card::CardRules::unsupported(),
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
