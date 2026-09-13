//! Conflux cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityTargetPredicate;
use crate::AppliedRuleDef;
use crate::ControlDurationDef;
use crate::DiscardSelectionDef;
use crate::PlayerRefDef;
use crate::PlayerSetDef;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BasicLandType;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaTypeSetDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::SacrificedAmountDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CON",
    slug: "conflux",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CON 3 — Aven Squire
pub(in crate::card::sets) static AVEN_SQUIRE: CardRecord = CardRecord::new(
    "Aven Squire",
    "60301dbd-40d1-4af8-8e2b-797febfa859f",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird", "Soldier"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::exalted()]),
);

// CON 5 — Celestial Purge
pub(in crate::card::sets) static CELESTIAL_PURGE: CardRecord = CardRecord::new(
    "Celestial Purge",
    "31c404e8-1241-4675-b259-fbbf1dba15c4",
    "David Palumbo",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target black or red permanent.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Black),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )),
);

// CON 15 — Path to Exile
pub(in crate::card::sets) static PATH_TO_EXILE: CardRecord = CardRecord::new(
    "Path to Exile",
    "29b7a8b1-b98e-483a-87a4-73bd831c03d4",
    "Todd Lockwood",
CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature. Its controller may search their library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
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
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ]),
    )),
);

// CON 31 — Master Transmuter
// Audit: unsupported — Returning a chosen battlefield permanent is not supported by the activated-cost planner; ReturnToHand currently has a casting-cost path only.
pub(in crate::card::sets) static MASTER_TRANSMUTER_31: CardRecord = CardRecord::new(
    "Master Transmuter",
    "252482b2-aaa7-49f3-af8c-30923ca98994",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// CON 48 — Kederekt Parasite
pub(in crate::card::sets) static KEDEREKT_PARASITE_48: CardRecord = CardRecord::new(
    "Kederekt Parasite",
    "878c7d8c-4df0-43ac-8197-d89c8be5e70d",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{B}"), &["Horror"], 1, 1).with_abilities(&[
AbilityDef::triggered_if("Whenever an opponent draws a card, if you control a red permanent, you may have this creature deal 1 damage to that player.", TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)), &TriggerConditionDef::ObjectCount { query: ObjectQueryDef::matching(ObjectPredicateDef::Color(ManaColor::Red), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 1 }, EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::damage(EffectRecipientDef::player(PlayerRefDef::EventPlayer), ValueDef::Constant(1)) })
]),
);

// CON 60 — Canyon Minotaur
pub(in crate::card::sets) static CANYON_MINOTAUR: CardRecord = CardRecord::new(
    "Canyon Minotaur",
    "9b200790-43c7-42ae-9edf-89c8198a385b",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur", "Warrior"], 3, 3),
);

// CON 87 — Noble Hierarch
pub(in crate::card::sets) static NOBLE_HIERARCH: CardRecord = CardRecord::new(
    "Noble Hierarch",
    "6adfe928-1305-444d-b709-1e714544daaf",
    "Mark Zug",
    // A one-mana accelerant for three colours whose body is beside the
    // point, except that exalted makes the 0/1 into a real attacker's
    // dividend on any turn nothing else attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Druid"], 0, 1).with_abilities(&[
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {G}, {W}, or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// CON 113 — Knight of the Reliquary
/// Land cards rather than creature cards, and your own graveyard: what she
/// sacrifices to fetch is what makes her bigger, so each activation pays
/// twice.
static RELIQUARY_LAND_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

pub(in crate::card::sets) static KNIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    "Knight of the Reliquary",
    "ad8b8518-c09e-4cb7-95b2-08e4e370d89c",
    "Michael Komarck",
// Three mana for a body that grows a point every time it fetches, which
    // is what makes the utility lands in the deck worth a card each.
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature gets +1/+1 for each land card in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&RELIQUARY_LAND_CARDS),
                        ValueDef::CountMatchingObjects(&RELIQUARY_LAND_CARDS),
                    ),
                },
            ),
            AbilityDef::activated(
                "{T}, Sacrifice a Forest or Plains: Search your library for a land card, put it onto the \
                 battlefield, then shuffle.",
                &[
                    CostDef::TapSource,
                    CostDef::SacrificePermanent {
                        // A Forest or a Plains by basic land type rather than by name, so a dual
                        // with either type pays for her too.
                        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest, BasicLandType::Plains]),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    // Untapped, unlike the Wight's: the land she finds can be used
                    // the turn it arrives.
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// CON 116 — Magister Sphinx
pub(in crate::card::sets) static MAGISTER_SPHINX_116: CardRecord = CardRecord::new(
    "Magister Sphinx",
    "cd2abff9-6927-42cc-8cf1-a0876d3a45d7",
    "Steven Belledin",
    CardRules::new_artifact_creature(mana_cost!("{4}{W}{U}{B}"), &["Sphinx"], 5, 5).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target player’s life total becomes 10.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::SetLifeTotal {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    total: ValueDef::Constant(10),
                },
            ),
        ],
    ),
);

// CON 120 — Nicol Bolas, Planeswalker
pub(in crate::card::sets) static NICOL_BOLAS_PLANESWALKER: CardRecord = CardRecord::new(
    "Nicol Bolas, Planeswalker",
    "48ee3939-bc12-4275-a446-9de36f0b4672",
    "D. Alexander Gregory",
CardRules::new_planeswalker(mana_cost!("{4}{U}{B}{B}{R}"), &["Bolas"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+3: Destroy target noncreature permanent.",
                &[CostDef::Loyalty(3)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Gain control of target creature.",
                &[CostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))],
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::Indefinitely,
                ),
            ),
            AbilityDef::activated_with_targets(
                "−9: Nicol Bolas deals 7 damage to target player or planeswalker. That player or that planeswalker's controller discards seven then sacrifices seven permanents of their choice.",
                &[CostDef::Loyalty(-9)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any))],
                EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(7),
                    ),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(7),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::SacrificeOfChoice {
                        player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        object: ObjectPredicateDef::Any,
                        count: ValueDef::Constant(7),
                        then: None,
                        amount: SacrificedAmountDef::Power,
                        otherwise: None,
                        optional: false,
                    },
                ]),
            ),
        ]),
);

// CON 121 — Progenitus
// Audit: unsupported — Needs the hidden-zone graveyard replacement to reveal the redirected card to every player before shuffling it into its owner's library; the movement replacement currently redirects and shuffles without publishing that reveal.
pub(in crate::card::sets) static PROGENITUS: CardRecord = CardRecord::new(
    "Progenitus",
    "bcc764b0-3046-4bde-b424-c0f4e1a6169b",
    "Jaime Jones",
    CardRules::unsupported(),
);

// CON 135 — Bone Saw
pub(in crate::card::sets) static BONE_SAW_135: CardRecord = CardRecord::new(
    "Bone Saw",
    "a3bf79d6-4b4a-4fdd-a831-36eff2523661",
    "Pete Venters",
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::object(ObjectRefDef::AttachedToSource),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// CON 141 — Ancient Ziggurat
pub(in crate::card::sets) static ANCIENT_ZIGGURAT: CardRecord = CardRecord::new(
    "Ancient Ziggurat",
    "0348247d-0a70-4961-8590-9de41386c69b",
    "John Avon",
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color. Spend this mana only to cast a creature spell.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
            ManaRestrictionDef::CastSpell(ObjectPredicateDef::HasType(CardType::Creature)),
        ])),
    )),
);

// CON 142 — Exotic Orchard
pub(in crate::card::sets) static EXOTIC_ORCHARD: CardRecord = CardRecord::new(
    "Exotic Orchard",
    "6aae6480-4e71-4d94-a648-f80d3849d792",
    "Steven Belledin",
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[CostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice_from(
            ManaTypeSetDef::could_be_produced_by(&ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )))
            .colors_only(),
        )),
    )),
);

// CON 143 — Reliquary Tower
pub(in crate::card::sets) static RELIQUARY_TOWER: CardRecord = CardRecord::new(
    "Reliquary Tower",
    "c5c0c1a5-dce7-4c7d-8a5b-0bf93ba68ace",
    "Jesper Ejsing",
    // "You", so it does nothing for the opponent, and it is read at cleanup
    // rather than captured -- losing the Tower on your own turn puts the
    // limit straight back.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::static_ability(
            "You have no maximum hand size.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    crate::card::PlayerRuleDef::NoMaximumHandSize,
                )),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AVEN_SQUIRE,
    &CELESTIAL_PURGE,
    &PATH_TO_EXILE,
    &MASTER_TRANSMUTER_31,
    &KEDEREKT_PARASITE_48,
    &CANYON_MINOTAUR,
    &NOBLE_HIERARCH,
    &KNIGHT_OF_THE_RELIQUARY,
    &MAGISTER_SPHINX_116,
    &NICOL_BOLAS_PLANESWALKER,
    &PROGENITUS,
    &BONE_SAW_135,
    &ANCIENT_ZIGGURAT,
    &EXOTIC_ORCHARD,
    &RELIQUARY_TOWER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
