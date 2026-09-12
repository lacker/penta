//! Shadowmoor cards cataloged for the Vintage Cube pool.

use crate::card::ColorSet;
use crate::card::ExilePlayDurationDef;
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
            "Conspire (As you cast this spell, you may tap two untapped \
             creatures you control that share a color with it.)",
            OptionalAdditionalCostAbilityDef {
                kind: OptionalAdditionalCostKindDef::Conspire,
                label: "Conspire",
                resolution_destination: SpellResolutionDestinationDef::Graveyard,
                costs,
            },
        ),
        AbilityDef::triggered_if(
            "When you conspire, copy this spell. You may choose new \
             targets for the copy.",
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

// SHM 31 — Cerulean Wisps
pub(in crate::card::sets) static CERULEAN_WISPS_31: CardRecord = CardRecord::new(
    "Cerulean Wisps",
    "0dca4f46-0aad-484f-b4ea-ed61a4fc1a89",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature becomes blue until end of turn. Untap that creature.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

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

// SHM 42 — Knacksaw Clique
pub(in crate::card::sets) static KNACKSAW_CLIQUE_42: CardRecord = CardRecord::new(
    "Knacksaw Clique",
    "22353590-f248-460e-a1a5-0b7431a4c82d",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Rogue"], 1, 4).with_abilities(&[
abilities::flying(),
AbilityDef::activated_with_targets("{1}{U}, {Q}: Target opponent exiles the top card of their library. Until end of turn, you may play that card. ({Q} is the untap symbol.)", &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::UntapSource], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent))], EffectDef::ExileTopOfLibraryToPlay { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), amount: ValueDef::Constant(1), free: false, face_down: false, duration: ExilePlayDurationDef::ThisTurn, spend_any_color: false, play_condition: None, cast_only: false })
]),
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

// SHM 66 — Faerie Macabre
pub(in crate::card::sets) static FAERIE_MACABRE_66: CardRecord = CardRecord::new(
    "Faerie Macabre",
    "ead8befa-27dd-4ec4-b317-1c231407e0ac",
    "rk post",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Faerie", "Rogue"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Discard this card: Exile up to two target cards from graveyards.",
            &[CostDef::DiscardSource],
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
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

// SHM 88 — Crimson Wisps
pub(in crate::card::sets) static CRIMSON_WISPS_88: CardRecord = CardRecord::new(
    "Crimson Wisps",
    "a65c81ff-fc5d-4191-93fb-52eb806457b7",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
AbilityDef::spell_with_targets("Target creature becomes red and gains haste until end of turn. (It can attack and {T} this turn.)\nDraw a card.", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])), AppliedEffectDef::add_ability(&abilities::haste())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }, abilities::draw_cards(ValueDef::Constant(1))]))
]),
);

// SHM 110 — Devoted Druid
// Audit: unsupported — PutCountersOnSource is supported in resolving payments but not activation-cost enumeration or payment; the counter must be paid before this ability goes on the stack.
pub(in crate::card::sets) static DEVOTED_DRUID_110: CardRecord = CardRecord::new(
    "Devoted Druid",
    "820e2f07-f637-4144-b45a-0e1430dcf55e",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
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

// SHM 166 — Helm of the Ghastlord
pub(in crate::card::sets) static HELM_OF_THE_GHASTLORD_166: CardRecord = CardRecord::new(
    "Helm of the Ghastlord",
    "653dd5b7-1ad3-4df1-bd2a-4e6ae362a8a2",
    "Franz Vohwinkel",
    CardRules::new_enchantment(mana_cost!("{3}{U/B}")).with_subtypes(&["Aura"]).with_abilities(&[
abilities::aura_spell("Enchant creature", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))]),
AbilityDef::static_ability("As long as enchanted creature is blue, it gets +1/+1 and has \"Whenever this creature deals damage to an opponent, draw a card.\"", EffectDef::IfCondition { condition: &TriggerConditionDef::AttachedPermanentMatches { object: ObjectPredicateDef::Color(ManaColor::Blue) }, then: &EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)), AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature deals damage to an opponent, draw a card.", TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent), abilities::draw_cards(ValueDef::Constant(1))))]) } }),
AbilityDef::static_ability("As long as enchanted creature is black, it gets +1/+1 and has \"Whenever this creature deals damage to an opponent, that player discards a card.\"", EffectDef::IfCondition { condition: &TriggerConditionDef::AttachedPermanentMatches { object: ObjectPredicateDef::Color(ManaColor::Black) }, then: &EffectDef::StaticApply { recipient: EffectRecipientDef::AttachedPermanent, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)), AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature deals damage to an opponent, that player discards a card.", TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent), EffectDef::Discard { recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer), amount: ValueDef::Constant(1), selection: crate::card::DiscardSelectionDef::RecipientChooses, then: None }))]) } })
]),
);

// SHM 192 — Murderous Redcap
pub(in crate::card::sets) static MURDEROUS_REDCAP_192: CardRecord = CardRecord::new(
    "Murderous Redcap",
    "5f7171f0-7d42-47c9-ab46-93c2bb42c914",
    "Dave Allsop",
    CardRules::new_creature(mana_cost!("{2}{B/R}{B/R}"), &["Goblin", "Assassin"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, it deals damage equal to its power to any target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::SourcePower,
                ),
            ),
            abilities::persist(),
        ]),
);

// SHM 208 — Guttural Response
pub(in crate::card::sets) static GUTTURAL_RESPONSE_208: CardRecord = CardRecord::new(
    "Guttural Response",
    "e0662ab6-b475-4b8d-ae77-a9b654e611da",
    "Matt Cavotta",
    CardRules::new_instant(mana_cost!("{R/G}")).with_abilities(&[AbilityDef::counter_target(
        "Counter target blue instant spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::Color(ManaColor::Blue),
                ObjectPredicateDef::HasType(CardType::Instant),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )]),
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

// SHM 241 — Seedcradle Witch
pub(in crate::card::sets) static SEEDCRADLE_WITCH_241: CardRecord = CardRecord::new(
    "Seedcradle Witch",
    "a0ae8525-ce10-40bd-8980-a05fb81a0fac",
    "Steven Belledin",
    CardRules::new_creature(mana_cost!("{G/W}"), &["Elf", "Shaman"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}{G}{W}: Target creature gets +3/+3 until end of turn. Untap that creature.",
            &[CostDef::Mana(mana_cost!("{2}{G}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// SHM 245 — Wilt-Leaf Liege
// Audit: unsupported — Needs a prospective discard event carrying the causing spell or ability's controller, then replacing the discard destination with battlefield entry; a generic graveyard move cannot distinguish discard from other moves.
pub(in crate::card::sets) static WILT_LEAF_LIEGE: CardRecord = CardRecord::new(
    "Wilt-Leaf Liege",
    "e6a2881f-e771-47d7-a39e-692054ee727f",
    "Jason Chan",
    CardRules::unsupported(),
);

// SHM 258 — Pili-Pala
// Audit: unsupported — The mana-ability eligibility/payment path rejects UntapSource costs; ordinary nonmana untap-symbol activations do not make this mana ability executable.
pub(in crate::card::sets) static PILI_PALA_258: CardRecord = CardRecord::new(
    "Pili-Pala",
    "4892c152-1f4a-4616-8e7f-0ca4911e621a",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// SHM 267 — Umbral Mantle
pub(in crate::card::sets) static UMBRAL_MANTLE_267: CardRecord = CardRecord::new(
    "Umbral Mantle",
    "10e35711-aec9-4024-a2a6-9efff8c71df2",
    "Richard Sardinha",
    CardRules::new_artifact(mana_cost!("{3}")).with_subtypes(&["Equipment"]).with_abilities(&[
AbilityDef::static_ability("Equipped creature has \"{3}, {Q}: This creature gets +2/+2 until end of turn.\" ({Q} is the untap symbol.)", EffectDef::StaticApply { recipient: EffectRecipientDef::object(ObjectRefDef::AttachedToSource), effect: AppliedEffectDef::add_ability(&AbilityDef::activated("{3}, {Q}: This creature gets +2/+2 until end of turn.", &[CostDef::Mana(mana_cost!("{3}")), CostDef::UntapSource], EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)), duration: ResolvedEffectDurationDef::UntilEndOfTurn })) }),
abilities::equip(&[CostDef::Mana(mana_cost!("{0}"))], "Equip {0}")
]),
);

// SHM 271 — Fire-Lit Thicket
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static FIRE_LIT_THICKET_271: CardRecord = CardRecord::new(
    "Fire-Lit Thicket",
    "0ab9d6ad-f819-4a1e-b4ff-8dc00791f0fd",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// SHM 277 — Mystic Gate
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static MYSTIC_GATE_277: CardRecord = CardRecord::new(
    "Mystic Gate",
    "3dfa866b-93e2-4365-91b0-f12d1f7c5395",
    "Fred Fields",
    crate::card::CardRules::unsupported(),
);

// SHM 280 — Sunken Ruins
// Audit: unsupported — Mana-ability payment rejects hybrid symbols in activation costs; it cannot choose which half of the filter cost to pay.
pub(in crate::card::sets) static SUNKEN_RUINS_280: CardRecord = CardRecord::new(
    "Sunken Ruins",
    "9d91a31c-b70a-45bd-a8dd-48d49b277f24",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CERULEAN_WISPS_31,
    &COUNTERBORE,
    &KNACKSAW_CLIQUE_42,
    &BESEECH_THE_QUEEN,
    &FAERIE_MACABRE_66,
    &BURN_TRAIL,
    &CRIMSON_WISPS_88,
    &DEVOTED_DRUID_110,
    &GLOOMWIDOW,
    &HOWL_OF_THE_NIGHT_PACK,
    &MOSSBRIDGE_TROLL,
    &WOODFALL_PRIMUS,
    &HELM_OF_THE_GHASTLORD_166,
    &MURDEROUS_REDCAP_192,
    &GUTTURAL_RESPONSE_208,
    &MANAMORPHOSE,
    &BARKSHELL_BLESSING,
    &SEEDCRADLE_WITCH_241,
    &WILT_LEAF_LIEGE,
    &PILI_PALA_258,
    &UMBRAL_MANTLE_267,
    &FIRE_LIT_THICKET_271,
    &MYSTIC_GATE_277,
    &SUNKEN_RUINS_280,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
