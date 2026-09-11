//! Shadowmoor cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::AppliedRuleDef;
use crate::BasicLandType;
use crate::KeywordAbility;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::OptionalAdditionalCostAbilityDef;
use crate::card::OptionalAdditionalCostKindDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Conspire is one optional creature-tapping cast cost plus the cast trigger
/// that copies the spell when that cost was paid. Each card supplies the
/// creature predicate that shares one of its colors.
const fn conspire(spell: &'static AbilityDef, costs: &'static [CostDef]) -> [AbilityDef; 3] {
    [
        *spell,
        AbilityDef::optional_additional_cost(
            "Conspire (As you cast this spell, you may tap two untapped creatures you control that share a color with it.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Conspire,
                label: "Conspire",
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
                costs,
            },
        ),
        AbilityDef::triggered_if(
            "When you conspire, copy this spell. You may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            &TriggerConditionDef::SourcePaidAdditionalCost(AdditionalCostIndex::PRIMARY),
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::TriggeringObject,
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ]
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SHM",
    slug: "shadowmoor",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SHM 33 — Counterbore
pub(in crate::card::sets) static COUNTERBORE: CardRecord = CardRecord::new(
    "Counterbore",
    "f4228b80-d87d-4ebe-ae92-04e4a7d0dc43",
    "Wayne England",
CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. Search its controller's graveyard, hand, and library for all cards with the same name as that spell and exile them. Then that player shuffles.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(Binding!("counterbore_target")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Sequence(&[
                    EffectDef::Counter {
                        object: EffectRecipientDef::object(ObjectRefDef::Binding(Binding!(
                            "counterbore_target"
                        ))),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                    abilities::search_controllers_zone_and_exile(
                        ZoneKind::Graveyard,
                        Binding!("counterbore_target"),
                    ),
                    abilities::search_controllers_zone_and_exile(
                        ZoneKind::Hand,
                        Binding!("counterbore_target"),
                    ),
                    abilities::search_controllers_zone_and_exile(
                        ZoneKind::Library,
                        Binding!("counterbore_target"),
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Binding(Binding!("counterbore_target")),
                        )),
                    },
                ]),
            }),
        ),
    ),
);

// SHM 57 — Beseech the Queen
pub(in crate::card::sets) static BESEECH_THE_QUEEN: CardRecord = CardRecord::new(
    "Beseech the Queen",
    "64ee0a93-0f6d-42be-bdca-1de5422d8d54",
    "Jason Chan",
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

// SHM 86 — Burn Trail
pub(in crate::card::sets) static BURN_TRAIL: CardRecord = CardRecord::new(
    "Burn Trail",
    "7f01f9a0-f1d0-4241-a270-df4ed673d1fd",
    "Nils Hamm",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&conspire(
        &AbilityDef::spell_with_targets(
            "Burn Trail deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
        &[CostDef::Tap {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
            quantity: CostQuantityDef::Fixed(2),
        }],
    )),
);

// SHM 117 — Gloomwidow
pub(in crate::card::sets) static GLOOMWIDOW: CardRecord = CardRecord::new(
    "Gloomwidow",
    "99bda306-1e37-4359-a649-fcd8a5a7e2fc",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spider"], 3, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// SHM 119 — Howl of the Night Pack
pub(in crate::card::sets) static HOWL_OF_THE_NIGHT_PACK: CardRecord = CardRecord::new(
    "Howl of the Night Pack",
    "293f7768-6279-4f26-979f-ea4e48095ae5",
    "Lars Grant-West",
    CardRules::new_sorcery(mana_cost!("{6}{G}")).with_ability(AbilityDef::spell(
        "Create a 2/2 green Wolf creature token for each Forest you control.",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Wolf"], &[ManaColor::Green], 2, 2).with_art(
                    CardArt::new("309f1bd4-78af-4722-9d45-b5f40b001570", "Lars Grant-West"),
                ),
            ))
            .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
        ),
    )),
);

// SHM 123 — Mossbridge Troll
pub(in crate::card::sets) static MOSSBRIDGE_TROLL: CardRecord = CardRecord::new(
    "Mossbridge Troll",
    "537c39cc-44d3-4869-9e76-dd9c2c68ee90",
    "Jeremy Jarvis",
CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Troll"], 5, 5).with_abilities(&[
        abilities::regenerates_if_destroyed(
            "If this creature would be destroyed, regenerate it.",
        ),
        AbilityDef::activated(
            "Tap any number of untapped creatures you control other than this creature with total power 10 or greater: This creature gets +20/+20 until end of turn.",
            &[CostDef::TapCreaturesWithTotalPower { minimum: 10 }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(20),
                    ValueDef::Constant(20),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SHM 135 — Woodfall Primus
pub(in crate::card::sets) static WOODFALL_PRIMUS: CardRecord = CardRecord::new(
    "Woodfall Primus",
    "43aa7e35-55ee-4e02-a8aa-ea2b267055d1",
    "Adam Rex",
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
                    then: None,
                },
            ),
            abilities::persist(),
        ]),
);

// SHM 211 — Manamorphose
pub(in crate::card::sets) static MANAMORPHOSE: CardRecord = CardRecord::new(
    "Manamorphose",
    "50283122-b8c4-4fb3-8eba-6252b72222f4",
    "Jeff Miracola",
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
pub(in crate::card::sets) static BARKSHELL_BLESSING: CardRecord = CardRecord::new(
    "Barkshell Blessing",
    "cd273ef2-4aed-4c7e-8c97-fe8b1af9ce69",
    "Steven Belledin",
    CardRules::new_instant(mana_cost!("{G/W}")).with_abilities(&conspire(
        &AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.",
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
        ),
        &[CostDef::Tap {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
            quantity: CostQuantityDef::Fixed(2),
        }],
    )),
);

// SHM 245 — Wilt-Leaf Liege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILT_LEAF_LIEGE: CardRecord = CardRecord::new(
    "Wilt-Leaf Liege",
    "e6a2881f-e771-47d7-a39e-692054ee727f",
    "Jason Chan",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COUNTERBORE,
    &BESEECH_THE_QUEEN,
    &BURN_TRAIL,
    &GLOOMWIDOW,
    &HOWL_OF_THE_NIGHT_PACK,
    &MOSSBRIDGE_TROLL,
    &WOODFALL_PRIMUS,
    &MANAMORPHOSE,
    &BARKSHELL_BLESSING,
    &WILT_LEAF_LIEGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
