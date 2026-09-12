//! The Hobbit card inventory.

fn adventure(record: &CardRecord, name: &'static str, alternate: &CardRules) -> CardComposition {
    let primary_name = record
        .name
        .split(" // ")
        .next()
        .expect("Adventure has a primary name");
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, primary_name, record.rules),
            CardPart::new(CardPartId(1), name, *alternate),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                primary_name,
                SpellForm::Part(CardPartId::PRIMARY),
                record.rules.mana_cost().expect("printed primary cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                name,
                SpellForm::Part(CardPartId(1)),
                alternate.mana_cost().expect("printed Adventure cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

use super::{CardRecord, PrintingRecord};
use crate::CardPartId;
use crate::PlayOptionId;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternateSpellKind;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::ArrivalAttachmentDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardEffectStatus;
use crate::card::CardPart;
use crate::card::CardRules;
use crate::card::CardStructure;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::ConditionalValueDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamagePreventionDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayOptionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealAndClassifyCardsDef;
use crate::card::RevealObjectsDef;
use crate::card::SpellCastQueryDef;
use crate::card::SpellForm;
use crate::card::SpellResolutionDestinationDef;
use crate::card::StackObjectEventDef;
use crate::card::StackObjectEventMatcherDef;
use crate::card::StackTargetAggregationDef;
use crate::card::StackTargetFilterDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1997::portal as catalog_por;
use crate::card::sets::y2017::ixalan as catalog_xln;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "HOB",
    slug: "the-hobbit",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("c6e096bb-ad9e-4a8b-8b42-26852fa32c1d", "Leonardo Santanna"),
);

const HUMAN_SOLDIER_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Human", "Soldier"], &[ManaColor::White], 1, 1).with_art(
        CardArt::new("6007af81-4541-4b55-90ea-03d365362ae5", "Jarel Threat"),
    );
const DWARF_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Dwarf"], &[ManaColor::Red], 2, 2);
const GOBLIN_ARMY_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Goblin", "Army"], &[ManaColor::Black], 0, 0).with_art(
        CardArt::new(
            "2e2028b1-34c0-40b6-8f65-79f79a279996",
            "Stanislav Sherbakov",
        ),
    );
const DRAGON_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Dragon"], &[ManaColor::Red], 6, 6)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "1e4408fa-8037-42f1-989e-2da84867f76c",
            "Nino Is",
        ));
const ELF_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Elf"], &[ManaColor::Green], 1, 1).with_art(CardArt::new(
        "761c7c31-c6c5-44e2-a845-f590542b6eda",
        "Hristo D. Chukov",
    ));

// HOB 1 — Long-Bodied Grey Dog
pub(in crate::card::sets) static LONG_BODIED_GREY_DOG: CardRecord = CardRecord::new(
    "Long-Bodied Grey Dog",
    "d1a1e520-1fe2-4529-8afb-c187bb80da3c",
    "Anna Podedworna",
    CardRules::new_creature(mana_cost!("{3}"), &["Dog"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::reach(),
        abilities::enters_trigger(
            "When this creature enters, create a tapped Treasure token. \
             (It's an artifact with \"{T}, Sacrifice this token: Add one \
             mana of any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1))
                    .entering_tapped(),
            ),
        ),
    ]),
);

// HOB 2 — Old Thrush
pub(in crate::card::sets) static OLD_THRUSH: CardRecord = CardRecord::new(
    "Old Thrush",
    "3ad02b56-13ec-46ef-92bd-ae078b8bb517",
    "Francisco Miyara",
    CardRules::new_creature(mana_cost!("{2}"), &["Bird"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life. You may search \
             your library for a basic land card, reveal it, then shuffle \
             and put that card on top.",
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Library,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
            ]),
        ),
    ]),
);

// HOB 3 — Troop of Ponies
pub(in crate::card::sets) static TROOP_OF_PONIES: CardRecord = CardRecord::new(
    "Troop of Ponies",
    "0b4b1c59-bcec-4779-9e27-0e6f9feb4e11",
    "Christina Kraus",
    CardRules::new_creature(mana_cost!("{2}"), &["Horse"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "{2}, {T}, Sacrifice this creature: Search your library for up \
             to two basic land cards, reveal them, put one onto the \
             battlefield tapped and the other into your hand, then \
             shuffle.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: true,
                    destination: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                    shuffle: false,
                    enters_tapped: false,
                    attachment: None,
                    binding: Some(crate::Binding!("found")),
                    then: Some(&EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("found")),
                                predicate: ObjectSetPredicateDef {
                                    filter: None,
                                    comparison: ComparisonDef::Greater,
                                    amount: 0,
                                },
                            },
                        ),
                        then: &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Object(crate::Binding!("land")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Binding(crate::Binding!("found")),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::BindObjects(BindObjectsDef {
                                source: ObjectCollectionSourceDef::ObjectSet(
                                    ObjectSetDef::ExceptObject {
                                        objects: &ObjectSetDef::Binding(crate::Binding!("found")),
                                        object: ObjectRefDef::Binding(crate::Binding!("land")),
                                    },
                                ),
                                binding: crate::Binding!("rest"),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::WithBattlefieldArrival {
                                        effect: &EffectDef::move_to_zone(
                                            EffectRecipientDef::object(ObjectRefDef::Binding(
                                                crate::Binding!("land"),
                                            )),
                                            ZoneKind::Battlefield,
                                            ZonePlacement::Top,
                                        ),
                                        arrival: BattlefieldArrivalDef {
                                            modifications: &[
                                                BattlefieldEntryModificationDef::Tapped,
                                            ],
                                            ..BattlefieldArrivalDef::DEFAULT
                                        },
                                    },
                                    EffectDef::move_to_zone(
                                        EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("rest"),
                                        )),
                                        ZoneKind::Hand,
                                        ZonePlacement::Top,
                                    ),
                                ]),
                            }),
                        }),
                    }),
                },
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
            ]),
        ),
    ]),
);

// HOB 4 — Belladonna Took
// Audit: unsupported — Needs per-ability resolution history retained after its source leaves the battlefield; SourceResolutionsThisTurn currently reads only a live permanent, so queued second and third resolutions lose their rewards.
pub(in crate::card::sets) static BELLADONNA_TOOK: CardRecord = CardRecord::new(
    "Belladonna Took",
    "88f0c189-c9ed-4ea3-ae62-3d8ac6c7fecf",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// HOB 5 — Bilbo's Gambit
// Audit: unsupported — Needs an optional cast-time gift promise and its give-gift event at resolution, independent of ordinary additional costs.
pub(in crate::card::sets) static BILBO_S_GAMBIT: CardRecord = CardRecord::new(
    "Bilbo's Gambit",
    "45ad01f0-cda8-4931-82bb-cb4949e56ae9",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// HOB 6 — Bofur, Reliable Guardian // Concerted Care
pub(in crate::card::sets) static BOFUR_RELIABLE_GUARDIAN: CardRecord = CardRecord::new(
    "Bofur, Reliable Guardian // Concerted Care",
    "6b8e6435-7de4-41d5-bc7d-8e24c11897d0",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{W}"), &["Dwarf", "Scout"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::lifelink()]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_instant(mana_cost!("{1}{W}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Target artifact or creature you control gains hexproof and \
                 indestructible until end of turn. (Then exile this card. You \
                 may cast the creature later from exile.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&BOFUR_RELIABLE_GUARDIAN, "Concerted Care", &ALTERNATE)
});

// HOB 7 — Celebrate the Mountain-king
pub(in crate::card::sets) static CELEBRATE_THE_MOUNTAIN_KING: CardRecord = CardRecord::new(
    "Celebrate the Mountain-king",
    "42fbd61d-e1a6-465d-b1a3-f5ee0869d3af",
    "Tomas Duchek",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, for each opponent, exile up to \
             one target nonland permanent that player controls until this \
             enchantment leaves the battlefield.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                until_source_leaves: true,
                then: None,
            },
        ),
        abilities::enters_trigger(
            "When this enchantment enters, recruit. (Draw a card, then \
             discard a card. If you discarded a nonland card, create a 1/1 \
             white Human Soldier creature token.)",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
    ]),
);

// HOB 8 — Dáin, Lord of the Iron Hills
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static DAIN_LORD_OF_THE_IRON_HILLS: CardRecord = CardRecord::new(
    "Dáin, Lord of the Iron Hills",
    "99d27749-d16c-45e9-accc-6a01351c17f9",
    "Tomas Duchek",
    CardRules::unsupported(),
);

// HOB 9 — Dwarven Provisioner
pub(in crate::card::sets) static DWARVEN_PROVISIONER: CardRecord = CardRecord::new(
    "Dwarven Provisioner",
    "1f9a61a1-454e-4d5b-a6dd-1a79fe9dedf3",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dwarf", "Citizen"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{3}{W}: Creatures you control get +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 10 — Dwarven Shortsword
pub(in crate::card::sets) static DWARVEN_SHORTSWORD: CardRecord = CardRecord::new(
    "Dwarven Shortsword",
    "f2341cf3-4d2c-4a4f-9aea-8834104a8910",
    "Manuel Castañón",
    CardRules::new_artifact(mana_cost!("{3}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 2/2 red Dwarf creature \
                 token, then attach this Equipment to it.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(DWARF_TOKEN)).with_created_tokens(
                        CreatedTokensDef {
                            binding: crate::Binding!("dwarf"),
                            then: &EffectDef::Attach {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("dwarf"),
                                )),
                            },
                        },
                    ),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// HOB 11 — Eagle of the Great Shelf
pub(in crate::card::sets) static EAGLE_OF_THE_GREAT_SHELF: CardRecord = CardRecord::new(
    "Eagle of the Great Shelf",
    "3feca644-5f65-4477-bbc8-d505cec6f3a5",
    "Yuhong Ding",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird", "Soldier"], 2, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/+1 until end of \
             turn for each other creature you control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 12 — The Eagles Are Coming!
// Audit: unsupported — Needs target cardinality conditional on paying kicker: exactly one un-kicked, but zero through any number when kicked. Current target counts cannot change both minimum and maximum from an optional cast-cost choice.
pub(in crate::card::sets) static THE_EAGLES_ARE_COMING: CardRecord = CardRecord::new(
    "The Eagles Are Coming!",
    "0bda1b62-47fc-42c2-a841-ccad8ea0db48",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// HOB 13 — Esgaroth Garrison
pub(in crate::card::sets) static ESGAROTH_GARRISON: CardRecord = CardRecord::new(
    "Esgaroth Garrison",
    "573f67b0-6ce8-4857-a703-4a5728640736",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 0, 5).with_abilities(&[
        AbilityDef::static_ability(
            "Esgaroth Garrison's power is equal to the number of creatures \
             you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            },
        ),
        abilities::enters_trigger(
            "When this creature enters, recruit. (Draw a card, then \
             discard a card. If you discarded a nonland card, create a 1/1 \
             white Human Soldier creature token.)",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
    ]),
);

// HOB 14 — Fíli the Pathfinder
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static FILI_THE_PATHFINDER: CardRecord = CardRecord::new(
    "Fíli the Pathfinder",
    "b02142f3-5e55-40dc-a02c-9113fb7d763c",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// HOB 15 — Gleaming Splendor
pub(in crate::card::sets) static GLEAMING_SPLENDOR: CardRecord = CardRecord::new(
    "Gleaming Splendor",
    "3b087bd4-bbb7-4963-bdb6-0a700ff19a04",
    "Kekai Kotaki",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an opponent draws their second card each turn, you \
             create a Treasure token.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(
                PlayerRelation::Opponent,
                2,
            )),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}{W}: Two target players each draw a card.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            &[AbilityTargetDef {
                minimum: 2,
                maximum: 2,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any))
            }],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 16 — Iron Hills Blacksmith
pub(in crate::card::sets) static IRON_HILLS_BLACKSMITH: CardRecord = CardRecord::new(
    "Iron Hills Blacksmith",
    "370e09c2-36c5-4662-8350-1db798afad3e",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dwarf", "Artificer"], 1, 1).with_abilities(&[
        abilities::double_strike(),
        abilities::enters_trigger(
            "When this creature enters, create a colorless Equipment \
             artifact token named Axe with \"Equipped creature gets \
             +1/+0\" and equip {2}.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact(&["Equipment"], &[])
                    .with_name("Axe")
                    .with_abilities(&[
                        AbilityDef::static_ability(
                            "Equipped creature gets +1/+0.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::AttachedPermanent,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(0),
                                ),
                            },
                        ),
                        abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
                    ]),
            ))),
        ),
    ]),
);

// HOB 17 — Kíli the Resourceful
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static KILI_THE_RESOURCEFUL: CardRecord = CardRecord::new(
    "Kíli the Resourceful",
    "1805532f-6d99-47d0-9529-5f5831a7fdc8",
    "Yuhong Ding",
    CardRules::unsupported(),
);

// HOB 18 — Lake-town Lookout
pub(in crate::card::sets) static LAKE_TOWN_LOOKOUT: CardRecord = CardRecord::new(
    "Lake-town Lookout",
    "178c4cf6-6b11-40e4-9673-c560d6818a6b",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Scout"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, recruit. (Draw a card, then discard \
             a card. If you discarded a nonland card, create a 1/1 white \
             Human Soldier creature token.)",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
    ]),
);

// HOB 19 — Lake-town Toymaker
pub(in crate::card::sets) static LAKE_TOWN_TOYMAKER: CardRecord = CardRecord::new(
    "Lake-town Toymaker",
    "67304269-c595-4cf0-8dbf-fcb2e9e01fe2",
    "Marina Ortega Lorente",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Artificer"], 3, 4).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "At the beginning of combat on your turn, if you've drawn two \
             or more cards this turn, another target creature you control \
             gets +3/+0 and gains first strike until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 20 — Magnificent End
// Audit: unsupported — Needs a self-cost reduction that can inspect whether the chosen creature target is tapped; the current self-cost evaluator cannot read target characteristics.
pub(in crate::card::sets) static MAGNIFICENT_END: CardRecord = CardRecord::new(
    "Magnificent End",
    "430c8916-1167-400b-9cad-d301f59d5e5d",
    "Yuhong Ding",
    CardRules::unsupported(),
);

// HOB 21 — Moment of Glory
pub(in crate::card::sets) static MOMENT_OF_GLORY: CardRecord = CardRecord::new(
    "Moment of Glory",
    "0a6a6ff0-b1cd-4b06-bd31-612690094e0e",
    "Jarel Threat",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature you control. If this \
             spell was cast from a graveyard, also put a +1/+1 counter on \
             each other creature you control.",
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
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef {
                            excluding_target: Some(TargetIndex::PRIMARY),
                            ..ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )
                        })),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{W}"))]),
    ]),
);

// HOB 22 — The Mountain-king's Return
pub(in crate::card::sets) static THE_MOUNTAIN_KING_S_RETURN: CardRecord = CardRecord::new(
    "The Mountain-king's Return",
    "68f4893d-e9a5-4f89-ade3-9ab78a834ad5",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Recruit. (Draw a card, then discard a card. If you \
                 discarded a nonland card, create a 1/1 white Human Soldier \
                 creature token.)",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                            bound: Some(crate::Binding!("recruits")),
                            effect: &EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))
                                    .with_count(ValueDef::BoundObjectCount(crate::Binding!(
                                        "recruits"
                                    ))),
                            ),
                        }),
                    },
                ]),
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Return target creature card with mana value 3 or less \
                 from your graveyard to the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMost(3),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Put a +1/+1 counter on up to one target creature.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// HOB 23 — Ori, Keeper of Songs
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static ORI_KEEPER_OF_SONGS: CardRecord = CardRecord::new(
    "Ori, Keeper of Songs",
    "c5727af5-a487-4b16-8278-81c3c928c417",
    "Yigit Koroglu",
    CardRules::unsupported(),
);

// HOB 24 — The Queen of Dale
pub(in crate::card::sets) static THE_QUEEN_OF_DALE: CardRecord = CardRecord::new(
    "The Queen of Dale",
    "c977fb5f-4436-41d0-af68-93b6d05897e5",
    "Magali Villeneuve",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Noble"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever an opponent casts their first noncreature spell each \
             turn, you recruit. (Draw a card, then discard a card. If you \
             discarded a nonland card, create a 1/1 white Human Soldier \
             creature token.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ])),
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::EventPlayer,
                        spell: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Creature,
                        )),
                    }),
                    comparison: ComparisonDef::Equal,
                    right: ValueDef::Constant(1),
                }),
            },
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        )]),
);

// HOB 25 — Roads Go Ever, Ever On
// Audit: unsupported — Needs a source-independent installed trigger that chooses a new creature target for each attack this turn; installed triggers currently require an empty target list.
pub(in crate::card::sets) static ROADS_GO_EVER_EVER_ON: CardRecord = CardRecord::new(
    "Roads Go Ever, Ever On",
    "b3c1ebd6-967f-4b8c-8f1f-442ce8c1da24",
    "Rovina Cai",
    CardRules::unsupported(),
);

// HOB 26 — Settle the Wreckage (reprint)
const SETTLE_THE_WRECKAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SETTLE_THE_WRECKAGE,
    "31a7a5e2-4cb8-48fc-8351-18344b4a7560",
    "Chris Cold",
);

// HOB 27 — Stone by Sunlight
pub(in crate::card::sets) static STONE_BY_SUNLIGHT: CardRecord = CardRecord::new(
    "Stone by Sunlight",
    "c5752731-253c-4b41-bdd8-94c26d715206",
    "Kamila Szutenberg",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target creature with power 4 or greater.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Until end of turn, target creature becomes an artifact in \
                 addition to its other types and gains indestructible. (Damage \
                 and effects that say \"destroy\" don't destroy it.)",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// HOB 28 — Thorin's Last Stand
pub(in crate::card::sets) static THORIN_S_LAST_STAND: CardRecord = CardRecord::new(
    "Thorin's Last Stand",
    "127367b6-9cfe-4516-9bfd-5b951468a25c",
    "John Di Giovanni",
    CardRules::new_instant(mana_cost!("{2}{W}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact or enchantment. You gain 2 life.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ],
    )]),
);

// HOB 29 — An Unexpected Party // At the Door
pub(in crate::card::sets) static AN_UNEXPECTED_PARTY: CardRecord = CardRecord::new(
    "An Unexpected Party // At the Door",
    "3aa29fe8-1687-486f-b4df-c04977869ab1",
    "Matt Stewart",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this enchantment enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Creatures you control of the chosen type get +2/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasSourcesChosenScalar(
                                BattlefieldEntryChoiceDestinationDef::CreatureType,
                            ),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            },
        ),
    ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{X}{2}{W}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Create X 2/2 red Dwarf creature tokens. (Then exile this \
                 card. You may cast the enchantment later from exile.)",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(DWARF_TOKEN))
                        .with_count(ValueDef::ChosenX),
                ),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&AN_UNEXPECTED_PARTY, "At the Door", &ALTERNATE)
});

// HOB 30 — Velvetwing Butterflies // Gaze in Wonder
pub(in crate::card::sets) static VELVETWING_BUTTERFLIES: CardRecord = CardRecord::new(
    "Velvetwing Butterflies // Gaze in Wonder",
    "5cc0f994-5048-4898-926e-b56cbc97e0ca",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Insect"], 2, 2)
        .with_abilities(&[abilities::flying()]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_instant(mana_cost!("{1}{W}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Tap one or two target creatures. (Then exile this card. You \
                 may cast the creature later from exile.)",
                &[AbilityTargetDef {
                    minimum: 1,
                    maximum: 2,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))
                }],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&VELVETWING_BUTTERFLIES, "Gaze in Wonder", &ALTERNATE)
});

// HOB 31 — Vow to Erebor
// Audit: unsupported — Needs a general attachment move between a chosen Equipment and a separately targeted host; current Attach and AttachToSource always use the resolving ability source as one endpoint.
pub(in crate::card::sets) static VOW_TO_EREBOR: CardRecord = CardRecord::new(
    "Vow to Erebor",
    "8d4f3eb5-fedf-45d6-8bd8-aacbe0ce33b2",
    "Andreia Ugrai",
    CardRules::unsupported(),
);

// HOB 32 — Bilbo, Luckwearer // Burglar's Plot
// Audit: unsupported — Needs two targets constrained to share a card type when chosen and rechecked on resolution; the existing atomic ExchangeControl effect does not supply that cross-target legality predicate.
pub(in crate::card::sets) static BILBO_LUCKWEARER: CardRecord = CardRecord::new(
    "Bilbo, Luckwearer // Burglar's Plot",
    "8bff0aa6-16d9-4c83-b598-ef00a3b33d2c",
    "Anna Steinbauer",
    CardRules::unsupported(),
);

// HOB 33 — Bilbo, Thief in the Night
// Audit: unsupported — Needs an immediate optional graveyard cast at its ordinary mana cost, with an exile-instead-of-graveyard rider limited to instant and sorcery spells cast through that permission.
pub(in crate::card::sets) static BILBO_THIEF_IN_THE_NIGHT: CardRecord = CardRecord::new(
    "Bilbo, Thief in the Night",
    "484c7f83-8339-4ae1-8350-68ce1f7d05a3",
    "Nia Kovalevski",
    CardRules::unsupported(),
);

// HOB 34 — Bilbo Baggins, Burglar // Take a Glance
pub(in crate::card::sets) static BILBO_BAGGINS_BURGLAR: CardRecord = CardRecord::new(
    "Bilbo Baggins, Burglar // Take a Glance",
    "6a109b3e-9f5b-4625-abb7-6b992c10530b",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Halfling", "Rogue"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::enters_trigger(
            "When Bilbo Baggins enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        )]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{U}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Scry 2. (Then exile this card. You may cast the creature \
                 later from exile.)",
                abilities::scry(ValueDef::Constant(2)),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&BILBO_BAGGINS_BURGLAR, "Take a Glance", &ALTERNATE)
});

// HOB 35 — Confusticate and Bebother
pub(in crate::card::sets) static CONFUSTICATE_AND_BEBOTHER: CardRecord = CardRecord::new(
    "Confusticate and Bebother",
    "9de48690-e5ae-495a-addf-305f1db7ec21",
    "Colin Boyer",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Counter target spell unless its controller pays {4}.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::PayOr(
                    PayOrDef::unless(
                        &[CostDef::Mana(mana_cost!("{4}"))],
                        &EffectDef::counter_target(TargetIndex::PRIMARY),
                    )
                    .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    ))),
                ),
            ),
            AbilityDef::spell(
                "Draw two cards, then discard a card.",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ],
    )]),
);

// HOB 36 — Elrond, Moon-Reader
// Audit: unsupported — Needs an ability-activation event matching the source creature, including immediate mana abilities; stack target-selection and cast events cannot represent all activations.
pub(in crate::card::sets) static ELROND_MOON_READER: CardRecord = CardRecord::new(
    "Elrond, Moon-Reader",
    "fbcb310c-be73-46f8-8e65-8632454ccc6e",
    "Christina Kraus",
    CardRules::unsupported(),
);

// HOB 37 — Elven Raft-Steerer
pub(in crate::card::sets) static ELVEN_RAFT_STEERER: CardRecord = CardRecord::new(
    "Elven Raft-Steerer",
    "c141695c-c108-41d5-85cb-1f7485d9d533",
    "Christina Kraus",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elf", "Pilot"], 3, 2).with_abilities(&[
        AbilityDef::modal_triggered(
            "Landfall — Whenever a land you control enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Tap target creature an opponent controls.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::Opponent),
                            owner: None,
                        },
                    )],
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Untap target creature you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
            ],
        ),
    ]),
);

// HOB 38 — Elvenking's Harper
pub(in crate::card::sets) static ELVENKING_S_HARPER: CardRecord = CardRecord::new(
    "Elvenking's Harper",
    "9c50656d-c74a-4e90-9ef7-afa237682516",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Elf", "Bard"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{4}{U}: Target creature can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{4}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 39 — Enchanted River's Grasp
pub(in crate::card::sets) static ENCHANTED_RIVER_S_GRASP: CardRecord = CardRecord::new(
    "Enchanted River's Grasp",
    "ad40a4b9-9fab-49c1-8e9f-6e0776966833",
    "Javier Charro",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature and remove all \
                 counters from it.",
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::AttachedPermanent,
                    },
                    EffectDef::RemoveAllCounters {
                        object: EffectRecipientDef::AttachedPermanent,
                        kind: None,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Enchanted creature loses all abilities and doesn't untap \
                 during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                    ]),
                },
            ),
        ]),
);

// HOB 40 — Fateful Discovery
pub(in crate::card::sets) static FATEFUL_DISCOVERY: CardRecord = CardRecord::new(
    "Fateful Discovery",
    "a1142fa1-b861-4876-aa48-402af35aaa63",
    "Irvin Rodriguez",
    CardRules::new_enchantment(mana_cost!("{3}{U}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever an artifact you control enters, draw a card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        abilities::draw_cards(ValueDef::Constant(1)),
    )]),
);

// HOB 41 — Gandalf, Wandering Wizard
pub(in crate::card::sets) static GANDALF_WANDERING_WIZARD: CardRecord = CardRecord::new(
    "Gandalf, Wandering Wizard",
    "1f8403a2-849c-4a59-b0ed-c8803995028d",
    "Irvin Rodriguez",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Avatar", "Wizard"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(&[CostDef::Mana(mana_cost!("{3}"))], "Ward {3}"),
            AbilityDef::activated(
                "{6}: Gandalf's owner shuffles him into their library and \
                 draws three cards.",
                &[CostDef::Mana(mana_cost!("{6}"))],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Source,
                        )),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Source,
                        )),
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ]),
);

// HOB 42 — Great Gilded Boat
pub(in crate::card::sets) static GREAT_GILDED_BOAT: CardRecord = CardRecord::new(
    "Great Gilded Boat",
    "b2fb3995-5b43-4776-88b2-346d353edee0",
    "Josu Solano",
    CardRules::new_vehicle(mana_cost!("{2}{U}"), 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you attack, recruit. (Draw a card, then discard a \
             card. If you discarded a nonland card, create a 1/1 white \
             Human Soldier creature token.)",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// HOB 43 — Lakeshore Apothecary
pub(in crate::card::sets) static LAKESHORE_APOTHECARY: CardRecord = CardRecord::new(
    "Lakeshore Apothecary",
    "abfbb255-a39b-4df5-bfb6-5298584e89f0",
    "Wei Guan",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Cleric"], 1, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 44 — Lake-town Mariners // Gone Fishing
pub(in crate::card::sets) static LAKE_TOWN_MARINERS: CardRecord = CardRecord::new(
    "Lake-town Mariners // Gone Fishing",
    "4202a678-a5f4-47f9-9c18-e88ab9ad20a4",
    "Wei Guan",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Human", "Citizen"], 6, 5).with_abilities(
        &[
            abilities::vigilance(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        ],
    ),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_instant(mana_cost!("{3}{U}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Exile two target creatures and/or lands you control, then \
                 return them to the battlefield under their owner's control.",
                &[AbilityTargetDef {
                    minimum: 2,
                    maximum: 2,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]))
                }],
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("exiled"),
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                            crate::Binding!("exiled"),
                        )),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                },
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&LAKE_TOWN_MARINERS, "Gone Fishing", &ALTERNATE)
});

// HOB 45 — Long Lake Nuisance
pub(in crate::card::sets) static LONG_LAKE_NUISANCE: CardRecord = CardRecord::new(
    "Long Lake Nuisance",
    "cd5af94d-6321-4834-8e5f-e5d0261b3ef3",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 3, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, recruit. (Draw a card, then \
             discard a card. If you discarded a nonland card, create a 1/1 \
             white Human Soldier creature token.)",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
    ]),
);

// HOB 46 — The Lord of the Eagles
// Audit: unsupported — Needs a self-cost reduction by the aggregate power of controlled flying creatures; the cost-value evaluator does not support ObjectValueAggregate.
pub(in crate::card::sets) static THE_LORD_OF_THE_EAGLES: CardRecord = CardRecord::new(
    "The Lord of the Eagles",
    "fa0554fc-9448-4ae2-8712-4f4f7af3c7b4",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// HOB 47 — Master's Councillors
pub(in crate::card::sets) static MASTER_S_COUNCILLORS: CardRecord = CardRecord::new(
    "Master's Councillors",
    "addcefdd-e012-4adf-9052-e60376a8d2d3",
    "Narendra Bintara Adi",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor"], 1, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "This creature gets +2/+0 for each graveyard with seven or \
             more cards in it.",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(7),
                    }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::Opponent,
                        )),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(7),
                    }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                    },
                },
            ]),
        ),
        AbilityDef::triggered_with_targets(
            "Whenever you draw your second card each turn, target player \
             mills three cards. (They put the top three cards of their \
             library into their graveyard.)",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// HOB 48 — Mirkwood Meditator
pub(in crate::card::sets) static MIRKWOOD_MEDITATOR: CardRecord = CardRecord::new(
    "Mirkwood Meditator",
    "ad7ed4e6-3fe2-40f1-909b-a03b2a3c941a",
    "Francisco Miyara",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elf", "Druid"], 2, 4).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, you may have \
             this creature's base power and toughness become 4/2 until end \
             of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// HOB 49 — Most Decrepit Old Bird // Speak Secrets
pub(in crate::card::sets) static MOST_DECREPIT_OLD_BIRD: CardRecord = CardRecord::new(
    "Most Decrepit Old Bird // Speak Secrets",
    "2d838feb-89f2-4cdb-a5ab-ec880f28d873",
    "Josu Solano",
    CardRules::new_creature(mana_cost!("{U}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Threshold — This creature gets +1/+1 as long as there are \
             seven or more cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{1}{U}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Mill four cards, then put an instant or sorcery card from \
                 among them into your hand.",
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(4),
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ])),
                        },
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&MOST_DECREPIT_OLD_BIRD, "Speak Secrets", &ALTERNATE)
});

// HOB 50 — Old Fat Spider Can't See Me
pub(in crate::card::sets) static OLD_FAT_SPIDER_CAN_T_SEE_ME: CardRecord = CardRecord::new(
    "Old Fat Spider Can't See Me",
    "4a865cea-f947-4736-8ace-ba478fceeb22",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Target creature you control gains hexproof for as long as \
                 this Saga remains on the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::WhileSourceRemains,
                },
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Prevent all damage that would be dealt by up to one \
                 target creature for as long as this Saga remains on the \
                 battlefield.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                        kind: DamageKindDef::Any,
                        source: DamageSourceMatcherDef::Object(ObjectRefDef::Target(
                            TargetIndex::PRIMARY,
                        )),
                        recipient: DamageRecipientMatcherDef::Any,
                    }),
                    duration: ResolvedEffectDurationDef::WhileSourceRemains,
                },
            ),
            abilities::saga_chapter(
                3,
                "III, IV — Draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            abilities::saga_chapter(
                4,
                "III, IV — Draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// HOB 51 — Plunder the Trollshaws
pub(in crate::card::sets) static PLUNDER_THE_TROLLSHAWS: CardRecord = CardRecord::new(
    "Plunder the Trollshaws",
    "afb73190-b9bd-4744-a011-a37cd9c0148d",
    "Alexander Mokhov",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card. If this spell was cast from a graveyard, draw \
             two cards instead.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                then: &abilities::draw_cards(ValueDef::Constant(2)),
                otherwise: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// HOB 52 — Ravenhill Flock
pub(in crate::card::sets) static RAVENHILL_FLOCK: CardRecord = CardRecord::new(
    "Ravenhill Flock",
    "acbb4d32-2771-469e-a6de-0df15155cc62",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you draw a card, put a +1/+1 counter on this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 53 — Riddles in the Dark
// Audit: unsupported — Needs an opponent pile choice presenting one revealed pile and one concealed pile with distinct visibility; current ChooseGroup uses a single visibility policy for both groups.
pub(in crate::card::sets) static RIDDLES_IN_THE_DARK: CardRecord = CardRecord::new(
    "Riddles in the Dark",
    "a6129286-7437-4ba4-be55-586a22cd67ca",
    "Lorenzo Mastroianni",
    CardRules::unsupported(),
);

// HOB 54 — Roll-Roll-Roll-Roll
pub(in crate::card::sets) static ROLL_ROLL_ROLL_ROLL: CardRecord = CardRecord::new(
    "Roll-Roll-Roll-Roll",
    "a2e4099e-86bd-461f-87fa-7f7850ae7eec",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I, II, III, IV — Exile up to one target creature or land you \
                 control. If you do, return it to the battlefield under its \
                 owner's control at the beginning of the next end step.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
            abilities::saga_chapter_with_targets(
                2,
                "I, II, III, IV — Exile up to one target creature or land you \
                 control. If you do, return it to the battlefield under its \
                 owner's control at the beginning of the next end step.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "I, II, III, IV — Exile up to one target creature or land you \
                 control. If you do, return it to the battlefield under its \
                 owner's control at the beginning of the next end step.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
            abilities::saga_chapter_with_targets(
                4,
                "I, II, III, IV — Exile up to one target creature or land you \
                 control. If you do, return it to the battlefield under its \
                 owner's control at the beginning of the next end step.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
            ),
        ]),
);

// HOB 55 — Sound the Trumpets
pub(in crate::card::sets) static SOUND_THE_TRUMPETS: CardRecord = CardRecord::new(
    "Sound the Trumpets",
    "dd32a1dd-3541-4572-a717-1deabc14b827",
    "Nereida",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell. If that spell's mana value was 2 or \
             less, recruit. (Draw a card, then discard a card. If you \
             discarded a nonland card, create a 1/1 white Human Soldier \
             creature token.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::counter_target(TargetIndex::PRIMARY),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::ManaValueAtMost(2),
                    },
                    then: &EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(1)),
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: Some(DiscardFollowUpDef {
                                counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                                bound: Some(crate::Binding!("recruits")),
                                effect: &EffectDef::CreateToken(
                                    CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))
                                        .with_count(ValueDef::BoundObjectCount(crate::Binding!(
                                            "recruits"
                                        ))),
                                ),
                            }),
                        },
                    ]),
                },
            ]),
        ),
    ]),
);

// HOB 56 — Thranduil's Decree
// Audit: unsupported — Needs a free-cast permission on a bound countered-and-exiled card lasting while it remains exiled; FreePlay currently supports only the resolving instruction or the current turn.
pub(in crate::card::sets) static THRANDUIL_S_DECREE: CardRecord = CardRecord::new(
    "Thranduil's Decree",
    "e4ded4c1-0e3e-47c5-8fdc-e7c187f68b12",
    "Javier Charro",
    CardRules::unsupported(),
);

// HOB 57 — Uncover the Moon-Letters
// Audit: unsupported — Needs the triggering spell's actual mana-paid total, retained as a cast receipt and exposed as an effect value.
pub(in crate::card::sets) static UNCOVER_THE_MOON_LETTERS: CardRecord = CardRecord::new(
    "Uncover the Moon-Letters",
    "79edf5f6-f6b6-4271-bd2a-14a980f30616",
    "Leon Tukker",
    CardRules::unsupported(),
);

// HOB 58 — Uneasy Partings
// Audit: unsupported — Needs a self-cost reduction based on chosen target characteristics (attacking and nontoken); current self-cost values have no chosen-target context.
pub(in crate::card::sets) static UNEASY_PARTINGS: CardRecord = CardRecord::new(
    "Uneasy Partings",
    "e49866d4-966a-40f9-b08d-18e5af6d726b",
    "Javier Charro",
    CardRules::unsupported(),
);

// HOB 59 — Wizard's Staff
// Audit: unsupported — Needs an additional-trigger modifier for every triggered ability of the equipped creature; the existing modifier is limited to triggers caused by battlefield entries.
pub(in crate::card::sets) static WIZARD_S_STAFF: CardRecord = CardRecord::new(
    "Wizard's Staff",
    "0de529a7-bdc5-4581-a169-1ad123bc099a",
    "Gaboleps",
    CardRules::unsupported(),
);

// HOB 60 — Along the Crooked Way
// Audit: unsupported — Needs per-card graveyard-departure events for all destinations, including hand and exile. The engine records a per-turn graveyard-departure flag but publishes ordinary zone-change triggers only for some destinations.
pub(in crate::card::sets) static ALONG_THE_CROOKED_WAY: CardRecord = CardRecord::new(
    "Along the Crooked Way",
    "3696d65c-fffd-4685-bb2d-e8769bf476e3",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// HOB 61 — Azog, Moria's Ruin
// Audit: unsupported — Needs a frozen amass amount from the targeted creature's power before creating or choosing an Army. The target can survive destruction, so rereading its power after Army creation can change X.
pub(in crate::card::sets) static AZOG_MORIA_S_RUIN: CardRecord = CardRecord::new(
    "Azog, Moria's Ruin",
    "135da718-affc-46ba-be57-c12c23b54dad",
    "Miklós Ligeti",
    CardRules::unsupported(),
);

// HOB 62 — Bilbo's Deadly Slice
pub(in crate::card::sets) static BILBO_S_DEADLY_SLICE: CardRecord = CardRecord::new(
    "Bilbo's Deadly Slice",
    "17892c93-b9b2-4720-933b-998ed0200492",
    "Henry Peters",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// HOB 63 — Crude Bent Blade
pub(in crate::card::sets) static CRUDE_BENT_BLADE: CardRecord = CardRecord::new(
    "Crude Bent Blade",
    "fa8fd3c4-bd00-485d-80b1-2b67f5786fce",
    "Russell Dongjun Lu",
    CardRules::new_artifact(mana_cost!("{2}{B}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, target opponent sacrifices a \
                 creature of their choice.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// HOB 64 — Desolation Prowler
pub(in crate::card::sets) static DESOLATION_PROWLER: CardRecord = CardRecord::new(
    "Desolation Prowler",
    "63c87009-ff1b-44b9-88b1-e26219094c67",
    "Harkalé Linaï",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Wolf"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "Pay 2 life: This creature gets +2/+2 until end of turn. \
             Activate only once each turn.",
            &[CostDef::PayLife(2)],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(1),
    ]),
);

// HOB 65 — Down, Down to Goblin-town
pub(in crate::card::sets) static DOWN_DOWN_TO_GOBLIN_TOWN: CardRecord = CardRecord::new(
    "Down, Down to Goblin-town",
    "b72e193c-e030-4936-9b79-c636eff750e1",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Target opponent reveals their hand. You choose a nonland \
                 card from it. That player discards that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )),
            ),
            abilities::saga_chapter(
                2,
                "II — Amass Goblins 1. (Put a +1/+1 counter on an Army you \
                 control. It's also a Goblin. If you don't control an Army, \
                 create a 0/0 black Goblin Army creature token first.)",
                EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Equal,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            GOBLIN_ARMY_TOKEN,
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Goblin"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    }),
                ]),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III, IV — Target opponent loses 1 life and you gain 1 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            abilities::saga_chapter_with_targets(
                4,
                "III, IV — Target opponent loses 1 life and you gain 1 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// HOB 66 — Dreaded Bat-Cloud
pub(in crate::card::sets) static DREADED_BAT_CLOUD: CardRecord = CardRecord::new(
    "Dreaded Bat-Cloud",
    "67d52db5-597e-46d5-af39-c3a2de107d30",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Bat"], 4, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {3} less to cast if a creature died this turn.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfCreatureDiedThisTurn(
                &ConditionalValueDef {
                    then: ValueDef::Constant(3),
                    otherwise: ValueDef::Constant(0),
                },
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::flying(),
        abilities::deathtouch(),
    ]),
);

// HOB 67 — Front Porch Sentries
pub(in crate::card::sets) static FRONT_PORCH_SENTRIES: CardRecord = CardRecord::new(
    "Front Porch Sentries",
    "07bfc803-e11b-47ab-9f25-0ace7e174200",
    "Stanislav Sherbakov",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Goblin", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature dies, target creature an opponent controls \
             gets -1/-1 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 68 — Gathering of Darkness
pub(in crate::card::sets) static GATHERING_OF_DARKNESS: CardRecord = CardRecord::new(
    "Gathering of Darkness",
    "2ce066be-e5ad-4b93-8245-1b5018990d03",
    "Pavel Kolomeyets",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to one target creature card from your graveyard to \
         your hand.\nAmass Goblins 3. (Put three +1/+1 counters on an \
         Army you control. It's also a Goblin. If you don't control an \
         Army, create a 0/0 black Goblin Army creature token first.)",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(3),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ]),
    )]),
);

// HOB 69 — Gnashing of Teeth
pub(in crate::card::sets) static GNASHING_OF_TEETH: CardRecord = CardRecord::new(
    "Gnashing of Teeth",
    "5d485d70-c7b9-40a4-9089-5e7f1c2b9213",
    "Nereida",
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature gets -5/-5 until end of turn. If that \
                 creature would die this turn, exile it instead.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-5),
                            ValueDef::Constant(-5),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Creatures target player controls get -1/-1 until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::controlled_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// HOB 70 — Gollum, Riddle Master
// Audit: unsupported — Needs a durable odd/even entry choice and per-source history of exhausted triggered modes; current modal triggers cannot exclude modes chosen by earlier resolutions.
pub(in crate::card::sets) static GOLLUM_RIDDLE_MASTER: CardRecord = CardRecord::new(
    "Gollum, Riddle Master",
    "bbdc7e37-c65a-497a-92b7-a30a6e369c71",
    "Irvin Rodriguez",
    CardRules::unsupported(),
);

// HOB 71 — Gollum, Silent Slinker // Meager Meal
pub(in crate::card::sets) static GOLLUM_SILENT_SLINKER: CardRecord = CardRecord::new(
    "Gollum, Silent Slinker // Meager Meal",
    "6cfaa182-3fec-4907-8814-b4d29c33cec3",
    "Miklós Ligeti",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Halfling", "Horror"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::menace()]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{B}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on up to one target creature. Target \
                 player gains 2 life. (Then exile this card. You may cast the \
                 creature later from exile.)",
                &[
                    AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                        1,
                    ),
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                        PlayerRelation::Any,
                    )),
                ],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        amount: ValueDef::Constant(2),
                    },
                ]),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&GOLLUM_SILENT_SLINKER, "Meager Meal", &ALTERNATE)
});

// HOB 72 — Gollum the Abandoned
// Audit: unsupported — Needs a graveyard activation that pays an artifact-or-creature sacrifice cost; the graveyard activation payer does not support sacrifice-permanent costs.
pub(in crate::card::sets) static GOLLUM_THE_ABANDONED: CardRecord = CardRecord::new(
    "Gollum the Abandoned",
    "50d91ef3-6f5d-4255-8d47-be731b5dad30",
    "Andrea Piparo",
    CardRules::unsupported(),
);

// HOB 73 — Great Fierce Bee
pub(in crate::card::sets) static GREAT_FIERCE_BEE: CardRecord = CardRecord::new(
    "Great Fierce Bee",
    "9d9ef88f-d208-4788-9553-cd672b3be1fe",
    "Leesha Hannigan",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever one or more other creatures die, scry 1. (Look at \
             the top card of your library. You may put that card on the \
             bottom.)",
            TriggerEventDef::ObjectsDied {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            },
            abilities::scry(ValueDef::Constant(1)),
        ),
    ]),
);

// HOB 74 — Great Ugly-Looking Goblin // Clap! Snap!
pub(in crate::card::sets) static GREAT_UGLY_LOOKING_GOBLIN: CardRecord = CardRecord::new(
    "Great Ugly-Looking Goblin // Clap! Snap!",
    "c87f6004-e1cf-42b2-9647-322bc4939339",
    "Jason Kang",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Goblin", "Soldier"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has \
             menace. (It can't be blocked except by two or more \
             creatures.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::menace()),
            },
        ),
    ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{1}{B}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Amass Goblins 2. (Then exile this card. You may cast the \
                 creature later from exile.)",
                EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Equal,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            GOBLIN_ARMY_TOKEN,
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(2),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Goblin"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    }),
                ]),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&GREAT_UGLY_LOOKING_GOBLIN, "Clap! Snap!", &ALTERNATE)
});

// HOB 75 — Head of the Hunt
// Audit: unsupported — Needs a source-independent reflexive trigger emitted when the death replacement actually exiles a creature; creating the Wolf inside the replacement would have the wrong timing.
pub(in crate::card::sets) static HEAD_OF_THE_HUNT: CardRecord = CardRecord::new(
    "Head of the Hunt",
    "3ffe34d4-72f4-4562-a948-8909b9321e59",
    "Andrea Piparo",
    CardRules::unsupported(),
);

// HOB 76 — Inside Information
// Audit: unsupported — Needs a persistent exile play permission with a life payment equal to each chosen spell's mana value as its alternative cast cost.
pub(in crate::card::sets) static INSIDE_INFORMATION: CardRecord = CardRecord::new(
    "Inside Information",
    "9763bd56-fa4b-4907-ad15-c3f040c5fc0a",
    "Sean Vo",
    CardRules::unsupported(),
);

// HOB 77 — The Master of Lake-town
// Audit: unsupported — Needs a life-loss event carrying the affected player and actual lost amount, including damage and life payments; current life-gain and damage events do not cover all life loss.
pub(in crate::card::sets) static THE_MASTER_OF_LAKE_TOWN: CardRecord = CardRecord::new(
    "The Master of Lake-town",
    "3788ada6-34a9-41af-a31c-2d090550e503",
    "Marius Bota",
    CardRules::unsupported(),
);

// HOB 78 — Nighthowl Pursuer
pub(in crate::card::sets) static NIGHTHOWL_PURSUER: CardRecord = CardRecord::new(
    "Nighthowl Pursuer",
    "d3cbe830-7e95-4019-89c4-cfb36bcf00f8",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{B}"), &["Wolf"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Ferocious — Whenever this creature attacks while you control \
             a creature with power 4 or greater, this creature gets +2/+2 \
             until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 79 — Rage into the Valley
pub(in crate::card::sets) static RAGE_INTO_THE_VALLEY: CardRecord = CardRecord::new(
    "Rage into the Valley",
    "8651958c-3b94-47a9-a751-faf8f6236a42",
    "Antonio José Manzanedo",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell(
        "You draw a card and lose 1 life.\nAmass Goblins 2. (Put two \
         +1/+1 counters on an Army you control. It's also a Goblin. If \
         you don't control an Army, create a 0/0 black Goblin Army \
         creature token first.)",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ]),
    )]),
);

// HOB 80 — Ravening Warg
pub(in crate::card::sets) static RAVENING_WARG: CardRecord = CardRecord::new(
    "Ravening Warg",
    "ea7b5052-b343-466d-879e-2a211657ef0a",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Wolf"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Ferocious — Whenever this creature attacks while you control \
             a creature with power 4 or greater, you gain 2 life.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// HOB 81 — Reverent Howl
pub(in crate::card::sets) static REVERENT_HOWL: CardRecord = CardRecord::new(
    "Reverent Howl",
    "16765eb2-d497-4cd6-b683-20eac2f10bbf",
    "Antonio José Manzanedo",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target player draws two cards and loses 2 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Target creature gets +2/+2 and gains lifelink until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// HOB 82 — Rhovanion Rampager
pub(in crate::card::sets) static RHOVANION_RAMPAGER: CardRecord = CardRecord::new(
    "Rhovanion Rampager",
    "5ee45a5e-3650-47b3-8d31-6b1de9e27a14",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Wolf"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice another \
             creature. If you do, put a number of +1/+1 counters on this \
             creature equal to the sacrificed creature's power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(crate::Binding!("victim")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::BindObjects(BindObjectsDef {
                    binding: crate::Binding!("sacrificed"),
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                        ObjectRefDef::Binding(crate::Binding!("victim")),
                    )),
                    then: &EffectDef::Sequence(&[
                        crate::card::actions::sacrifice_yours(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("sacrificed")),
                        ))
                        .as_effect(),
                        EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::CountObjects(
                                    &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                        "sacrificed"
                                    )),
                                ),
                                comparison: ComparisonDef::Greater,
                                right: ValueDef::Constant(0),
                            }),
                            then: &EffectDef::AddCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::ObjectPower(ObjectRefDef::Binding(
                                    crate::Binding!("victim"),
                                )),
                            },
                        },
                    ]),
                }),
            }),
        ),
        abilities::dies_trigger(
            "When this creature dies, amass Goblins X, where X is this \
             creature's power. (Put X +1/+1 counters on an Army you \
             control. It's also a Goblin. If you don't control an Army, \
             create a 0/0 black Goblin Army creature token first.)",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::SourcePower,
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ),
    ]),
);

// HOB 83 — The Sackville-Bagginses
pub(in crate::card::sets) static THE_SACKVILLE_BAGGINSES: CardRecord = CardRecord::new(
    "The Sackville-Bagginses",
    "ed87b471-79f9-45ec-9188-69e970f6121e",
    "Denman Rooke",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Halfling", "Citizen"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When The Sackville-Bagginses enter, you may sacrifice another \
                 creature or artifact. If you do, draw a card and create a \
                 Treasure token.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[crate::card::actions::choose_sacrifice(1)
                        .matching(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]))
                        .as_cost()],
                    &EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(1)),
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    ]),
                )),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you sacrifice a token, target opponent loses 1 life.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::Token,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// HOB 84 — Stir Up Trouble
pub(in crate::card::sets) static STIR_UP_TROUBLE: CardRecord = CardRecord::new(
    "Stir Up Trouble",
    "fd145e3a-c889-4390-accb-863dbcc845ce",
    "Andreia Ugrai",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice an \
             artifact or creature or pay {4}.\nDestroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            CostDef::Choice(&[
                CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ])),
                CostDef::Mana(mana_cost!("{4}")),
            ]),
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// HOB 85 — Stony-Voiced Goblins
pub(in crate::card::sets) static STONY_VOICED_GOBLINS: CardRecord = CardRecord::new(
    "Stony-Voiced Goblins",
    "6fcc3699-b475-4612-884d-81bd4f21e9c1",
    "Marius Bota",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Goblin", "Bard"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// HOB 86 — Supper for Spiders
// Audit: unsupported — Needs noncopiable Food artifact/type/ability changes on the prospective returning cards before they enter, together with battlefield-to-graveyard-this-turn identity history.
pub(in crate::card::sets) static SUPPER_FOR_SPIDERS: CardRecord = CardRecord::new(
    "Supper for Spiders",
    "5b25e454-06bb-43ca-9a9f-57164f7a70c4",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// HOB 87 — Balin, Loremaster
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static BALIN_LOREMASTER: CardRecord = CardRecord::new(
    "Balin, Loremaster",
    "42d7ca7b-c983-40fd-ad57-59f6972bb375",
    "Colin Boyer",
    CardRules::unsupported(),
);

// HOB 88 — Bombur, Gentle Dreamer
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static BOMBUR_GENTLE_DREAMER: CardRecord = CardRecord::new(
    "Bombur, Gentle Dreamer",
    "63c317e7-432c-4817-8db4-3670a1d84be3",
    "Eric Deschamps",
    CardRules::unsupported(),
);

// HOB 89 — Bothersome Noisemaker
pub(in crate::card::sets) static BOTHERSOME_NOISEMAKER: CardRecord = CardRecord::new(
    "Bothersome Noisemaker",
    "cb25b11a-6bf5-4a9a-b60f-d4dcac3816d6",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Bard"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, amass Goblins 1. (Put \
             a +1/+1 counter on an Army you control. It's also a Goblin. \
             If you don't control an Army, create a 0/0 black Goblin Army \
             creature token first.)",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ),
    ]),
);

// HOB 90 — Burn, Burn, Tree and Fern
pub(in crate::card::sets) static BURN_BURN_TREE_AND_FERN: CardRecord = CardRecord::new(
    "Burn, Burn, Tree and Fern",
    "fceb1a2d-121e-49ad-acf2-1bb5aebec116",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{3}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — This Saga deals 6 damage to target creature an opponent \
                 controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(6),
                ),
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Destroy target artifact an opponent controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            abilities::saga_chapter(
                3,
                "III, IV — Add {R}.",
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
            ),
            abilities::saga_chapter(
                4,
                "III, IV — Add {R}.",
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
            ),
        ]),
);

// HOB 91 — Dáin Ironfoot
// Audit: unsupported — Needs a reflexive trigger after token creation, choosing the attachment target after the Axe token exists and retaining it if the source leaves.
pub(in crate::card::sets) static DAIN_IRONFOOT: CardRecord = CardRecord::new(
    "Dáin Ironfoot",
    "7112e460-9160-4535-ad94-93f1f4ac04cf",
    "Tomas Duchek",
    CardRules::unsupported(),
);

// HOB 92 — Desert Were-Worm
// Audit: unsupported — Needs the attack-event condition reader to sum attacking creatures' power before applying the first-qualifying-trigger limit; AggregateObjectValues is evaluated during resolution but not when this trigger is captured.
pub(in crate::card::sets) static DESERT_WERE_WORM: CardRecord = CardRecord::new(
    "Desert Were-Worm",
    "fc12c22a-11ff-4fb0-bc42-dd8490b8efb7",
    "Aldo Domínguez",
    CardRules::unsupported(),
);

// HOB 93 — Desolation of Smaug
pub(in crate::card::sets) static DESOLATION_OF_SMAUG: CardRecord = CardRecord::new(
    "Desolation of Smaug",
    "2462358b-52c8-49b2-8d97-d65a9188f8f7",
    "Irvin Rodriguez",
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Desolation of Smaug deals 3 damage to each non-Dragon \
         creature.\nAdd four mana in any combination of colors. Spend \
         this mana only to cast Dragon spells.",
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Dragon",
                        ))),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                ValueDef::Constant(3),
            ),
            EffectDef::AddMana(
                AddManaEffectDef::combination(&ManaColor::COLORS, 4).with_restrictions(&[
                    ManaRestrictionDef::CastSpell(ObjectPredicateDef::Subtype(
                        SubtypeDef::Literal("Dragon"),
                    )),
                ]),
            ),
        ]),
    )]),
);

// HOB 94 — Dori, Bearer of Friends
pub(in crate::card::sets) static DORI_BEARER_OF_FRIENDS: CardRecord = CardRecord::new(
    "Dori, Bearer of Friends",
    "d2f60ad0-c887-4585-85f8-afcf72fb80d0",
    "Irvin Rodriguez",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Warrior"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger(
                "When Dori enters, create a Treasure token. (It's an artifact \
                 with \"{T}, Sacrifice this token: Add one mana of any \
                 color.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
        ]),
);

// HOB 95 — Dwarven Mauler
// Audit: unsupported — Needs equip-cost reduction based on the activation's chosen target; the current ability-cost modifier matches the ability source, not its recipient.
pub(in crate::card::sets) static DWARVEN_MAULER: CardRecord = CardRecord::new(
    "Dwarven Mauler",
    "bd0f0415-43af-4f5d-8999-853c5d42780d",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// HOB 96 — Gandalf, Goblins' Bane // Flameshape
// Audit: unsupported — Needs a face-down exile play permission gated dynamically on controlling a Wizard and lasting while each card remains exiled; current exile permissions do not carry such conditions.
pub(in crate::card::sets) static GANDALF_GOBLINS_BANE: CardRecord = CardRecord::new(
    "Gandalf, Goblins' Bane // Flameshape",
    "9b0d29a1-7da9-4fb3-8536-8ff8d8acae0b",
    "Francisco Miyara",
    CardRules::unsupported(),
);

// HOB 97 — Gandalf, Spark Starter
pub(in crate::card::sets) static GANDALF_SPARK_STARTER: CardRecord = CardRecord::new(
    "Gandalf, Spark Starter",
    "7c5c6f1c-35cf-4172-b5a1-b73222b0723b",
    "Javier Charro",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Avatar", "Wizard"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            abilities::enters_trigger_with_targets(
                "When Gandalf enters, he deals 3 damage divided as you choose \
                 among one, two, or three targets.",
                &[AbilityTargetDef {
                    minimum: 1,
                    maximum: 3,
                    divided_total: Some(crate::DividedTotal::Fixed(3)),
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)
                }],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::DividedAmongTargets,
                ),
            ),
        ]),
);

// HOB 98 — Getaway Barrel
// Audit: unsupported — Needs selecting the first member of a randomized matching-card collection without a player choice; current RandomizeObjectOrder can bind the shuffled group but cannot project its first member.
pub(in crate::card::sets) static GETAWAY_BARREL: CardRecord = CardRecord::new(
    "Getaway Barrel",
    "e4819aa6-5d28-4a37-942d-89523e30c4e1",
    "Pablo Mendoza",
    CardRules::unsupported(),
);

// HOB 99 — Glóin the Mighty // Easy Pickings
pub(in crate::card::sets) static GLOIN_THE_MIGHTY: CardRecord = CardRecord::new(
    "Glóin the Mighty // Easy Pickings",
    "5793b8eb-2fc5-454d-8fa2-20346fef167a",
    "Colin Boyer",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Dwarf", "Warrior"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your first main phase, add {R}{R}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
        )]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{2}{R}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Easy Pickings deals 1 damage to each creature your opponents \
                 control. (Then exile this card. You may cast the creature \
                 later from exile.)",
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ))),
                    ValueDef::Constant(1),
                ),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&GLOIN_THE_MIGHTY, "Easy Pickings", &ALTERNATE)
});

// HOB 100 — Goblin-town Flunkies
pub(in crate::card::sets) static GOBLIN_TOWN_FLUNKIES: CardRecord = CardRecord::new(
    "Goblin-town Flunkies",
    "ccff7382-8609-494c-aeee-cd1436456dd0",
    "Jason Kang",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Soldier"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger(
            "When this creature enters, amass Goblins 1. (Put a +1/+1 \
             counter on an Army you control. It's also a Goblin. If you \
             don't control an Army, create a 0/0 black Goblin Army \
             creature token first.)",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ),
    ]),
);

// HOB 101 — Gundabad Opportunist
// Audit: unsupported — Needs an exile play permission that expires at the end of the controller's next turn; the existing next-turn permission expires after a later opponent turn.
pub(in crate::card::sets) static GUNDABAD_OPPORTUNIST: CardRecord = CardRecord::new(
    "Gundabad Opportunist",
    "bc4a60b8-a5bb-4dbf-8d48-95caf757eac3",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// HOB 102 — Iron Hills Stalwart
// Audit: unsupported — Needs a general attachment move between two target slots; current Attach and AttachToSource always use the resolving ability source as one endpoint.
pub(in crate::card::sets) static IRON_HILLS_STALWART: CardRecord = CardRecord::new(
    "Iron Hills Stalwart",
    "46daa9ac-0ac7-4df9-b9d2-e03ab5b56c72",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// HOB 103 — Last Light of Durin's Day
// Audit: unsupported — Needs a single search over any chosen subset of hand and library, preserving whether the library was actually searched for the conditional shuffle; current searches have one fixed source.
pub(in crate::card::sets) static LAST_LIGHT_OF_DURIN_S_DAY: CardRecord = CardRecord::new(
    "Last Light of Durin's Day",
    "df29484b-de4b-4bab-995a-7605745780d9",
    "Harkalé Linaï",
    CardRules::unsupported(),
);

// HOB 104 — The Misty Mountains Cold
pub(in crate::card::sets) static THE_MISTY_MOUNTAINS_COLD: CardRecord = CardRecord::new(
    "The Misty Mountains Cold",
    "3d5f35ff-4146-4844-9da5-031461cc8c05",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I, II, III, IV — Create a Treasure token. Then if you control \
                 four or more Treasures, sacrifice this Saga. If you do, \
                 create a 6/6 red Dragon creature token with flying. (A \
                 Treasure token is an artifact with \"{T}, Sacrifice this \
                 token: Add one mana of any color.\")",
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                        then: &EffectDef::BindObjects(BindObjectsDef {
                            binding: crate::Binding!("saga"),
                            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                                ObjectRefDef::Source,
                            )),
                            then: &EffectDef::Sequence(&[
                                crate::card::actions::sacrifice_yours(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("saga")),
                                ))
                                .as_effect(),
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountObjects(
                                                &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                    crate::Binding!("saga"),
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::Constant(0),
                                        },
                                    ),
                                    then: &EffectDef::CreateToken(CreateTokenDef::new(
                                        TokenDef::Literal(DRAGON_TOKEN),
                                    )),
                                },
                            ]),
                        }),
                    },
                ]),
            ),
            abilities::saga_chapter(
                2,
                "I, II, III, IV — Create a Treasure token. Then if you control \
                 four or more Treasures, sacrifice this Saga. If you do, \
                 create a 6/6 red Dragon creature token with flying. (A \
                 Treasure token is an artifact with \"{T}, Sacrifice this \
                 token: Add one mana of any color.\")",
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                        then: &EffectDef::BindObjects(BindObjectsDef {
                            binding: crate::Binding!("saga"),
                            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                                ObjectRefDef::Source,
                            )),
                            then: &EffectDef::Sequence(&[
                                crate::card::actions::sacrifice_yours(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("saga")),
                                ))
                                .as_effect(),
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountObjects(
                                                &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                    crate::Binding!("saga"),
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::Constant(0),
                                        },
                                    ),
                                    then: &EffectDef::CreateToken(CreateTokenDef::new(
                                        TokenDef::Literal(DRAGON_TOKEN),
                                    )),
                                },
                            ]),
                        }),
                    },
                ]),
            ),
            abilities::saga_chapter(
                3,
                "I, II, III, IV — Create a Treasure token. Then if you control \
                 four or more Treasures, sacrifice this Saga. If you do, \
                 create a 6/6 red Dragon creature token with flying. (A \
                 Treasure token is an artifact with \"{T}, Sacrifice this \
                 token: Add one mana of any color.\")",
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                        then: &EffectDef::BindObjects(BindObjectsDef {
                            binding: crate::Binding!("saga"),
                            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                                ObjectRefDef::Source,
                            )),
                            then: &EffectDef::Sequence(&[
                                crate::card::actions::sacrifice_yours(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("saga")),
                                ))
                                .as_effect(),
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountObjects(
                                                &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                    crate::Binding!("saga"),
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::Constant(0),
                                        },
                                    ),
                                    then: &EffectDef::CreateToken(CreateTokenDef::new(
                                        TokenDef::Literal(DRAGON_TOKEN),
                                    )),
                                },
                            ]),
                        }),
                    },
                ]),
            ),
            abilities::saga_chapter(
                4,
                "I, II, III, IV — Create a Treasure token. Then if you control \
                 four or more Treasures, sacrifice this Saga. If you do, \
                 create a 6/6 red Dragon creature token with flying. (A \
                 Treasure token is an artifact with \"{T}, Sacrifice this \
                 token: Add one mana of any color.\")",
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                        then: &EffectDef::BindObjects(BindObjectsDef {
                            binding: crate::Binding!("saga"),
                            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                                ObjectRefDef::Source,
                            )),
                            then: &EffectDef::Sequence(&[
                                crate::card::actions::sacrifice_yours(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("saga")),
                                ))
                                .as_effect(),
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountObjects(
                                                &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                    crate::Binding!("saga"),
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::Constant(0),
                                        },
                                    ),
                                    then: &EffectDef::CreateToken(CreateTokenDef::new(
                                        TokenDef::Literal(DRAGON_TOKEN),
                                    )),
                                },
                            ]),
                        }),
                    },
                ]),
            ),
        ]),
);

// HOB 105 — Misty Mountains Raider
pub(in crate::card::sets) static MISTY_MOUNTAINS_RAIDER: CardRecord = CardRecord::new(
    "Misty Mountains Raider",
    "6dff14cd-b60b-48f4-9d9f-c9019b55df4c",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Goblin", "Soldier"], 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you attack, amass Goblins 2. (Put two +1/+1 counters \
             on an Army you control. It's also a Goblin. If you don't \
             control an Army, create a 0/0 black Goblin Army creature \
             token first.)",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        ),
    ]),
);

// HOB 106 — Óin the Brave
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static OIN_THE_BRAVE: CardRecord = CardRecord::new(
    "Óin the Brave",
    "9984b9ef-e81c-48f4-aa33-0504171a2d3c",
    "Colin Boyer",
    CardRules::unsupported(),
);

// HOB 107 — Pinecone Strike
pub(in crate::card::sets) static PINECONE_STRIKE: CardRecord = CardRecord::new(
    "Pinecone Strike",
    "ea174cea-40e5-424e-9734-e39aae6c6b17",
    "Javier Charro",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or both —",
        &[
            AbilityDef::spell_with_targets(
                "Pinecone Strike deals 3 damage to target creature. If that \
                 creature would die this turn, exile it instead.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(3),
                    ),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact token.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Token,
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// HOB 108 — Ragged Short Spear
pub(in crate::card::sets) static RAGGED_SHORT_SPEAR: CardRecord = CardRecord::new(
    "Ragged Short Spear",
    "7bf81a8b-52ad-49f5-a3d4-22613cad3a3d",
    "Miklós Ligeti",
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, you may discard a card. If you \
                 do, draw two cards.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::DiscardCards(1)],
                    &abilities::draw_cards(ValueDef::Constant(2)),
                )),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// HOB 109 — Smaug, the Great Calamity // Spew Flame
pub(in crate::card::sets) static SMAUG_THE_GREAT_CALAMITY: CardRecord = CardRecord::new(
    "Smaug, the Great Calamity // Spew Flame",
    "419ca9e5-8413-4378-a4ef-eda5a1024218",
    "Chris Cold",
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Dragon"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::flying()]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{4}{R}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_targets(
                "Spew Flame deals 5 damage to target creature. (Then exile \
                 this card. You may cast the creature later from exile.)",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(5),
                ),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&SMAUG_THE_GREAT_CALAMITY, "Spew Flame", &ALTERNATE)
});

// HOB 110 — Smaug the Magnificent
pub(in crate::card::sets) static SMAUG_THE_MAGNIFICENT: CardRecord = CardRecord::new(
    "Smaug the Magnificent",
    "6a5d8fad-2ffd-4645-8c49-907999b6cecf",
    "Francisco Miyara",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::haste(),
            AbilityDef::triggered_with_targets(
                "Whenever Smaug attacks, he deals damage equal to the number \
                 of Treasures you control to any target.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, create a Treasure token.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
        ]),
);

// HOB 111 — Smaug's Fury
pub(in crate::card::sets) static SMAUG_S_FURY: CardRecord = CardRecord::new(
    "Smaug's Fury",
    "a16f203a-785e-4c78-9410-fb9f8a0ffa01",
    "Andrea Piparo",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains reach and first strike \
         until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::reach()),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// HOB 112 — Snowslope Hunter
// Audit: unsupported — Needs an exile play permission that expires at the end of the controller's next turn; the existing next-turn permission expires after a later opponent turn.
pub(in crate::card::sets) static SNOWSLOPE_HUNTER: CardRecord = CardRecord::new(
    "Snowslope Hunter",
    "47666099-ffb2-4d07-a801-70524dba0837",
    "Adrián Rodríguez Pérez",
    CardRules::unsupported(),
);

// HOB 113 — Stone-Giant of High Pass
pub(in crate::card::sets) static STONE_GIANT_OF_HIGH_PASS: CardRecord = CardRecord::new(
    "Stone-Giant of High Pass",
    "5f4f4683-ffd2-447a-932b-276f7fa17cca",
    "Miklós Ligeti",
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Giant"], 7, 7).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, create a 3/1 \
             colorless Wall artifact creature token with defender named \
             Stone Boulder.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::artifact_creature(&["Wall"], &[], 3, 1)
                    .with_name("Stone Boulder")
                    .with_abilities(&[abilities::defender()]),
            ))),
        ),
        AbilityDef::activated_with_targets(
            "{2}{R}, Sacrifice an artifact: This creature deals 4 damage \
             to any target.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Artifact)),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// HOB 114 — Thorin, Mountain-king
// Audit: unsupported — Needs a source-independent reflexive trigger after one or more Equipment successfully attach, with its damage target chosen after those attachment moves.
pub(in crate::card::sets) static THORIN_MOUNTAIN_KING: CardRecord = CardRecord::new(
    "Thorin, Mountain-king",
    "117347af-0dd7-4350-901d-8c8a81387e22",
    "Javier Charro",
    CardRules::unsupported(),
);

// HOB 115 — Tidings of War
pub(in crate::card::sets) static TIDINGS_OF_WAR: CardRecord = CardRecord::new(
    "Tidings of War",
    "38c16a0a-375e-48cb-9720-dbbc08c603ae",
    "Pavel Kolomeyets",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell(
            "Amass Goblins 1. If this spell was cast from a graveyard, \
             amass Goblins 3 instead. (To amass Goblins X, put X +1/+1 \
             counters on an Army you control. It's also a Goblin. If you \
             don't control an Army, create a 0/0 black Goblin Army \
             creature token first.)",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                then: &EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Equal,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            GOBLIN_ARMY_TOKEN,
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("graveyard_army")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("graveyard_army"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(3),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("graveyard_army"),
                                )),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Goblin"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    }),
                ]),
                otherwise: &EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Equal,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            GOBLIN_ARMY_TOKEN,
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Goblin"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    }),
                ]),
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{R}"))]),
    ]),
);

// HOB 116 — Attercop
pub(in crate::card::sets) static ATTERCOP: CardRecord = CardRecord::new(
    "Attercop",
    "81263d5d-e402-4813-9458-161112da27ab",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 2, 1).with_abilities(&[
        abilities::reach(),
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             gets +1/+1 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 117 — Bejeweled Warg
pub(in crate::card::sets) static BEJEWELED_WARG: CardRecord = CardRecord::new(
    "Bejeweled Warg",
    "e95eba5c-e0d6-46b4-a0be-8e373b2185ea",
    "John Di Giovanni",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 3, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::modal_triggered(
            "Whenever this creature deals combat damage to a player, \
             choose one —",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            &[
                AbilityDef::spell_with_targets(
                    "Put a +1/+1 counter on target Wolf you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wolf")),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::spell(
                    "Create a Treasure token. (It's an artifact with \"{T}, \
                     Sacrifice this token: Add one mana of any color.\")",
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                ),
            ],
        ),
    ]),
);

// HOB 118 — Beorn, Reluctant Host // Till and Tend
// Audit: unsupported — Needs a temporary additional-land-play allowance for Till and Tend; MayPlayAdditionalLands currently works only as a static rule on a permanent, so neither Adventure face is enabled.
pub(in crate::card::sets) static BEORN_RELUCTANT_HOST: CardRecord = CardRecord::new(
    "Beorn, Reluctant Host // Till and Tend",
    "804589b7-3ef9-473d-97cc-c61a2d41f70d",
    "Javier Charro",
    CardRules::unsupported(),
);

// HOB 119 — Beorn the Fierce
pub(in crate::card::sets) static BEORN_THE_FIERCE: CardRecord = CardRecord::new(
    "Beorn the Fierce",
    "367d5f8b-77ee-47f7-bc71-972d62c280a9",
    "Nia Kovalevski",
    CardRules::new_creature(
        mana_cost!("{3}{G}{G}"),
        &["Bear", "Shapeshifter", "Warrior"],
        6,
        6,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Other Bears you control get +2/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bear")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, put a trample \
             counter on up to one target creature you control. It becomes \
             a Bear in addition to its other types. Then if you control \
             three or more Bears, draw two cards.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Trample,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                        "Bear",
                    ])),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bear")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(3),
                    }),
                    then: &abilities::draw_cards(ValueDef::Constant(2)),
                },
            ]),
        ),
    ]),
);

// HOB 120 — Beorn's Hospitality
// Audit: unsupported — Needs a resolving effect that grants an executable characteristic-defining static ability; permanent animation can set fixed base power and toughness, but cannot grant the continually updated land-count definition.
pub(in crate::card::sets) static BEORN_S_HOSPITALITY: CardRecord = CardRecord::new(
    "Beorn's Hospitality",
    "153ca57e-30f0-4ad7-ae9d-c55cbf0fd4c9",
    "Harkalé Linaï",
    CardRules::unsupported(),
);

// HOB 121 — Boughside Wanderers
pub(in crate::card::sets) static BOUGHSIDE_WANDERERS: CardRecord = CardRecord::new(
    "Boughside Wanderers",
    "71bec005-2925-4944-be16-2cc5eb30f5d6",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elf", "Scout"], 4, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal a permanent card from among them and \
             put it into your hand. Put the rest on the bottom of your \
             library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("randomized"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "randomized"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             gets +2/+2 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HOB 122 — Cantankerous Keepers
pub(in crate::card::sets) static CANTANKEROUS_KEEPERS: CardRecord = CardRecord::new(
    "Cantankerous Keepers",
    "fae46a70-a6d3-4584-859d-6c7425fb1508",
    "Ramza Psyru",
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Elf", "Soldier"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Elves (This spell costs {1} less to cast for \
             each Elf you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger(
            "When this creature enters, mill four cards, then put all Elf \
             cards from among them into your hand.",
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: crate::Binding!("milled"),
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(4),
                    },
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Subtype(
                            SubtypeDef::Literal("Elf"),
                        )),
                    }),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ]),
        ),
    ]),
);

// HOB 123 — Dancing from Dark to Dawn
pub(in crate::card::sets) static DANCING_FROM_DARK_TO_DAWN: CardRecord = CardRecord::new(
    "Dancing from Dark to Dawn",
    "550cd0b6-ca61-4db7-9d20-0b68c48066f9",
    "Leesha Hannigan",
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you cast a creature spell, put X +1/+1 counters on \
             target creature you control, where X is that spell's mana \
             value.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject),
            },
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, create a 2/2 \
             green Bear creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Bear"], &[ManaColor::Green], 2, 2),
            ))),
        ),
    ]),
);

// HOB 124 — Down in the Valley
pub(in crate::card::sets) static DOWN_IN_THE_VALLEY: CardRecord = CardRecord::new(
    "Down in the Valley",
    "c8aa5179-475b-4cc8-b21e-205b475eb4cf",
    "Rovina Cai",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Search your library for a basic land card, reveal it, put \
                 it into your hand, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
            ),
            abilities::saga_chapter(
                2,
                "II — This Saga gains \"Landfall — Whenever a land you control \
                 enters, create a 1/1 green Elf creature token.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Landfall — Whenever a land you control enters, create a 1/1 \
                         green Elf creature token.",
                        TriggerEventDef::zone_changed(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ]),
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELF_TOKEN))),
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                3,
                "III, IV — Elves you control get +1/+0 and gain vigilance \
                 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::saga_chapter(
                4,
                "III, IV — Elves you control get +1/+0 and gain vigilance \
                 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// HOB 125 — Galion, Elvenking's Butler
pub(in crate::card::sets) static GALION_ELVENKING_S_BUTLER: CardRecord = CardRecord::new(
    "Galion, Elvenking's Butler",
    "985bd676-58c4-42c7-a570-1b413e9aa94c",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Advisor"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever Galion attacks, choose up to one other target \
             creature you control. Its base power and toughness become \
             equal to Galion's power and toughness until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_base_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::SourceToughness,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// HOB 126 — Gigantic Big Bear
pub(in crate::card::sets) static GIGANTIC_BIG_BEAR: CardRecord = CardRecord::new(
    "Gigantic Big Bear",
    "7d6ece3d-8e7a-41ad-974f-3c9748de4825",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Bear"], 10, 7).with_abilities(&[
        abilities::cannot_be_countered(),
        abilities::hexproof(),
        abilities::haste(),
    ]),
);

// HOB 127 — Guardian of the Halls
pub(in crate::card::sets) static GUARDIAN_OF_THE_HALLS: CardRecord = CardRecord::new(
    "Guardian of the Halls",
    "4265caec-8c28-44cd-8e6b-90b5af926d3c",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Soldier"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{5}{G}{G}: Put three +1/+1 counters on this creature.",
            &[CostDef::Mana(mana_cost!("{5}{G}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// HOB 128 — Little Bear
pub(in crate::card::sets) static LITTLE_BEAR: CardRecord = CardRecord::new(
    "Little Bear",
    "8a50858a-33b5-4c45-9c31-5956ae5a33a6",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear"], 3, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, untap another target creature you \
             control. If that creature is a Bear, put a +1/+1 counter on \
             it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bear")),
                    },
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
    ]),
);

// HOB 129 — Mirkwood Pathmaker
pub(in crate::card::sets) static MIRKWOOD_PATHMAKER: CardRecord = CardRecord::new(
    "Mirkwood Pathmaker",
    "50fbedc0-bc66-4ffb-87f6-a2df69995091",
    "Sean Vo",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Ranger"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Mirkwood Pathmaker's power and toughness are each equal to \
             the number of lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
    ),
);

// HOB 130 — Nasty Little Rabbit
pub(in crate::card::sets) static NASTY_LITTLE_RABBIT: CardRecord = CardRecord::new(
    "Nasty Little Rabbit",
    "96bc7d25-2828-478a-8fe5-a1f4ede8c9c0",
    "Harkalé Linaï",
    CardRules::new_creature(mana_cost!("{G}"), &["Rabbit"], 1, 2).with_abilities(&[
        AbilityDef::triggered_if(
            "Ferocious — At the beginning of combat on your turn, if you \
             control a creature with power 4 or greater, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 131 — The Notary Hobbits
pub(in crate::card::sets) static THE_NOTARY_HOBBITS: CardRecord = CardRecord::new(
    "The Notary Hobbits",
    "d876315f-b269-4254-a517-905c6e927462",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Halfling", "Advisor"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_if(
                "When The Notary Hobbits enter, if they're not a token, create \
                 two tokens that are copies of them, except the tokens aren't \
                 legendary.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                        object: &EffectRecipientDef::Source,
                        exceptions: CopyExceptionsDef {
                            removed_supertypes: &[CardSupertype::Legendary],
                            ..CopyExceptionsDef::NONE
                        },
                    }))
                    .with_count(ValueDef::Constant(2)),
                ),
            ),
            AbilityDef::activated_mana(
                "{T}: Add {C} for each Halfling you control.",
                &[CostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Colorless).with_variable_amount(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Halfling")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                ),
            ),
        ]),
);

// HOB 132 — Old Fat Spider
pub(in crate::card::sets) static OLD_FAT_SPIDER: CardRecord = CardRecord::new(
    "Old Fat Spider",
    "e0c0f842-40fe-4776-a988-a35216bcfd47",
    "Lorenzo Mastroianni",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Spider"], 6, 7).with_abilities(&[
        abilities::reach(),
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with power 2 or less.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell or \
             ability an opponent controls, draw a card.",
            TriggerEventDef::StackObject(StackObjectEventMatcherDef {
                object: ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                event: StackObjectEventDef::TargetSelection {
                    target: StackTargetFilterDef::Permanent(ObjectPredicateDef::Source),
                    aggregation: StackTargetAggregationDef::EachMatchingTarget,
                },
            }),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// HOB 133 — Ordinary Bear
pub(in crate::card::sets) static ORDINARY_BEAR: CardRecord = CardRecord::new(
    "Ordinary Bear",
    "0feb9817-56e1-465a-851c-b2fe202aa8ae",
    "Andrea Piparo",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Bear"], 4, 5),
);

// HOB 134 — Part in Friendship
pub(in crate::card::sets) static PART_IN_FRIENDSHIP: CardRecord = CardRecord::new(
    "Part in Friendship",
    "b4ff1eac-6d97-40ab-9b7c-c2fdca0917d9",
    "Jarel Threat",
    CardRules::new_enchantment(mana_cost!("{4}{G}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a nontoken creature you control dies, reveal cards \
         from the top of your library until you reveal a creature \
         card. If its mana value is less than or equal to the number \
         of lands you control, put it onto the battlefield. Otherwise, \
         put it into your hand. Put the rest on the bottom of your \
         library in a random order. This ability triggers only once \
         each turn.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
            source: ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                player: PlayerRefDef::EffectController,
                object: ObjectPredicateDef::HasType(CardType::Creature),
            },
            object: ObjectPredicateDef::HasType(CardType::Creature),
            matching: crate::Binding!("found"),
            remainder: crate::Binding!("rest"),
            then: &EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("found")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Object(crate::Binding!("creature")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::Binding!("found")),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::IfElseCondition {
                            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::ObjectManaValue(ObjectRefDef::Binding(
                                    crate::Binding!("creature"),
                                )),
                                comparison: ComparisonDef::LessOrEqual,
                                right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                            }),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("found"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            otherwise: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("found"),
                                )),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                        },
                    }),
                },
                EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                    input: ObjectSetDef::Binding(crate::Binding!("rest")),
                    randomized: crate::Binding!("randomized"),
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "randomized"
                        ))),
                        ZoneKind::Library,
                        ZonePlacement::Bottom,
                    ),
                }),
            ]),
        }),
    )
    .triggering_at_most(1)]),
);

// HOB 135 — Quarrel
pub(in crate::card::sets) static QUARREL: CardRecord = CardRecord::new(
    "Quarrel",
    "5900a0b4-aa89-4019-94c9-7e9ea3b4792e",
    "Denman Rooke",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power \
         to target creature an opponent controls.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex(1)),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )]),
);

// HOB 136 — Radagast of Rhosgobel
// Audit: unsupported — Needs a battlefield spell-cost modifier conditioned on having cast no creature spells this turn; cost modifiers ignore enclosing IfCondition and their amount reader cannot evaluate that conditional history.
pub(in crate::card::sets) static RADAGAST_OF_RHOSGOBEL: CardRecord = CardRecord::new(
    "Radagast of Rhosgobel",
    "5741bbad-a6e4-45e0-b827-73f48c9975bf",
    "Anna Podedworna",
    CardRules::unsupported(),
);

// HOB 137 — Through the Forest Gate
pub(in crate::card::sets) static THROUGH_THE_FOREST_GATE: CardRecord = CardRecord::new(
    "Through the Forest Gate",
    "880adfc8-69cf-4062-a804-e65b6cb6056d",
    "Leon Tukker",
    CardRules::new_sorcery(mana_cost!("{6}{G}{G}")).with_abilities(&[AbilityDef::spell(
        "Look at the top twenty cards of your library, put any number \
         of land cards from among them onto the battlefield tapped, \
         then shuffle. You gain 8 life.",
        EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(20),
            },
            actor: PlayerRefDef::EffectController,
            inspection: CollectionInspectionDef::Look,
            object: ObjectPredicateDef::HasType(CardType::Land),
            minimum: 0,
            maximum: 255,
            chosen: crate::Binding!("chosen"),
            remainder: crate::Binding!("rest"),
            then: &EffectDef::Sequence(&[
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("rest"))),
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(8),
                },
            ]),
        }),
    )]),
);

// HOB 138 — Troll Negotiations
pub(in crate::card::sets) static TROLL_NEGOTIATIONS: CardRecord = CardRecord::new(
    "Troll Negotiations",
    "ca0f7bf4-b8a2-4ec4-ad7e-b639de9fa76a",
    "Wero Gallo",
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put two +1/+1 counters on target creature you control. Then \
             it fights target creature an opponent controls. (Each deals \
             damage equal to its power to the other.)",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Fight {
                    first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    second: ObjectRefDef::Target(TargetIndex(1)),
                    excess: None,
                },
            ]),
        ),
    ]),
);

// HOB 139 — Warg Tactics
pub(in crate::card::sets) static WARG_TACTICS: CardRecord = CardRecord::new(
    "Warg Tactics",
    "b06d9cee-bb0f-4fe7-ab2a-b55d36461aec",
    "Adrián Rodríguez Pérez",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Keyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on target creature you control. It gains \
                 trample and hexproof until end of turn. (It can't be the \
                 target of spells or abilities your opponents control.)",
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
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::trample()),
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ],
    )]),
);

// HOB 140 — Wargling
pub(in crate::card::sets) static WARGLING: CardRecord = CardRecord::new(
    "Wargling",
    "1ccbf823-846f-4f09-9c67-1deebb5d1d92",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Ferocious — Whenever this creature attacks while you control \
             a creature with power 4 or greater, until end of turn, this \
             creature gets +1/+0 and creatures you control gain trample.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// HOB 141 — Wilderland Scrounger
pub(in crate::card::sets) static WILDERLAND_SCROUNGER: CardRecord = CardRecord::new(
    "Wilderland Scrounger",
    "63078f42-f404-4c61-86be-45d934393b0a",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Wolf"], 3, 6).with_abilities(&[
        AbilityDef::triggered(
            "Ferocious — Whenever this creature attacks while you control \
             a creature with power 4 or greater, put a +1/+1 counter on \
             each creature you control.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 142 — Wood Elves (reprint)
const WOOD_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_por::WOOD_ELVES,
    "2476e42b-b209-4207-8ed9-cb668f89b218",
    "Andreia Ugrai",
);

// HOB 143 — Woodland Weavemaster
// Audit: unsupported — Needs a single mana permission allowing either Elf spell casts or Elf-source activations; current restriction lists are conjunctions, so combining CastSpell and ActivateAbility permits neither use.
pub(in crate::card::sets) static WOODLAND_WEAVEMASTER: CardRecord = CardRecord::new(
    "Woodland Weavemaster",
    "fe2b4bcf-56de-44d3-83af-aeb27f82c25e",
    "Nia Kovalevski",
    CardRules::unsupported(),
);

// HOB 144 — Bard, King of Dale
pub(in crate::card::sets) static BARD_KING_OF_DALE: CardRecord = CardRecord::new(
    "Bard, King of Dale",
    "c05c2aa6-29c7-40f8-872e-91099b9225c4",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{4}{W}{U}"), &["Human", "Noble", "Archer"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            abilities::vigilance(),
            AbilityDef::replacement_for(
                "If you would draw a card except the first one you draw in \
                 each of your draw steps, draw two cards instead.",
                ReplacementEventDef::WouldDraw {
                    player: PlayerRelation::You,
                    during_own_draw_step: false,
                    except_first_in_draw_step: true,
                },
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::ReplaceEventWithNothing,
                    ReplacementEffectDef::Perform(&EffectDef::Sequence(&[
                        EffectDef::ContinueReplacedDraw,
                        EffectDef::ContinueReplacedDraw,
                    ])),
                ]),
            ),
            AbilityDef::static_ability(
                "If one or more tokens would be created under your control, \
                 twice that many of those tokens are created instead.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoublesTokensCreated),
                },
            ),
        ]),
);

// HOB 145 — Bard the Bowman
pub(in crate::card::sets) static BARD_THE_BOWMAN: CardRecord = CardRecord::new(
    "Bard the Bowman",
    "0b84e232-428c-424a-848c-ef95debc6e50",
    "Miklós Ligeti",
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Human", "Archer"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered_with_targets(
                "Whenever you draw your second card each turn, put a +1/+1 \
                 counter on target creature. It gains lifelink until end of \
                 turn.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(
                    PlayerRelation::You,
                    2,
                )),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// HOB 146 — Bard's Company
pub(in crate::card::sets) static BARD_S_COMPANY: CardRecord = CardRecord::new(
    "Bard's Company",
    "d14aa2ff-7bbd-47a6-8e36-481e56302a62",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Human", "Citizen"], 2, 3).with_abilities(
        &[
            AbilityDef::static_ability(
                "You may cast this spell as though it had flash if you control \
                 a Human.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Human")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flash()),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Other creatures you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, recruit. (Draw a \
                 card, then discard a card. If you discarded a nonland card, \
                 create a 1/1 white Human Soldier creature token.)",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                            bound: Some(crate::Binding!("recruits")),
                            effect: &EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN))
                                    .with_count(ValueDef::BoundObjectCount(crate::Binding!(
                                        "recruits"
                                    ))),
                            ),
                        }),
                    },
                ]),
            ),
        ],
    ),
);

// HOB 147 — Bifur, Melodic Rider
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static BIFUR_MELODIC_RIDER: CardRecord = CardRecord::new(
    "Bifur, Melodic Rider",
    "ee88cc80-8fbf-451c-b2b8-09158426c26a",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// HOB 148 — Bolg of the North
// Audit: unsupported — Needs a source-independent reflexive trigger after sacrificing the creature, with late damage targeting and an actual excess-damage receipt for the following amass amount.
pub(in crate::card::sets) static BOLG_OF_THE_NORTH: CardRecord = CardRecord::new(
    "Bolg of the North",
    "7b2d2a7f-88e0-45a9-8579-a6736bcd66eb",
    "Miklós Ligeti",
    CardRules::unsupported(),
);

// HOB 149 — Bolg's Company
pub(in crate::card::sets) static BOLG_S_COMPANY: CardRecord = CardRecord::new(
    "Bolg's Company",
    "ea3f5644-f7e3-40de-ada5-cea2e9113cfb",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature has haste as long as you control another Goblin.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice another Goblin: Add {B}{R}.",
            &[
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Black,
                ManaColor::Red,
            )),
        ),
    ]),
);

// HOB 150 — The Chief Warg
pub(in crate::card::sets) static THE_CHIEF_WARG: CardRecord = CardRecord::new(
    "The Chief Warg",
    "c397a298-bf7f-49d7-a26a-206ccf9e8120",
    "Tomas Duchek",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Wolf"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::triggered(
                "Ferocious — Whenever you attack while you control a creature \
                 with power 4 or greater, you draw a card and lose 1 life.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attack_declared(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        1,
                        None,
                    ),
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::PowerAtLeast(4),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// HOB 151 — Chief Warg's Company
pub(in crate::card::sets) static CHIEF_WARG_S_COMPANY: CardRecord = CardRecord::new(
    "Chief Warg's Company",
    "bbc634af-63d2-444a-8123-85f16fe3e364",
    "Jason Kang",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Wolf"], 5, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature can't attack unless you control two or more \
             other Wolves.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wolf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Less,
                    right: ValueDef::Constant(2),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                },
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, create a 2/2 green Wolf \
             creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Wolf"], &[ManaColor::Green], 2, 2),
            ))),
        ),
    ]),
);

// HOB 152 — Dáin's Company
pub(in crate::card::sets) static DAIN_S_COMPANY: CardRecord = CardRecord::new(
    "Dáin's Company",
    "36db4405-8589-481f-b627-f26087488337",
    "Erikas Perl",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Dwarf", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature has lifelink as long as you control another Dwarf.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                },
            },
        ),
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal a Dwarf or Equipment card from among \
             them and put it into your hand. Put the rest on the bottom of \
             your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("randomized"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "randomized"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        ),
    ]),
);

// HOB 153 — Duskwatch Hunter
pub(in crate::card::sets) static DUSKWATCH_HUNTER: CardRecord = CardRecord::new(
    "Duskwatch Hunter",
    "3685c783-d837-4466-a960-ab3098db64c3",
    "Samuele Bandini",
    CardRules::new_creature(mana_cost!("{2}{B/G}"), &["Wolf"], 3, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by tokens.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Token,
                )),
            },
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target \
             creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HOB 154 — Dwalin, Weaponmaster
// Audit: unsupported — Needs hone counters with their intrinsic Equipment effect granting +1/+0 per counter to the equipped creature, independent of printed abilities and preserved through ability removal.
pub(in crate::card::sets) static DWALIN_WEAPONMASTER: CardRecord = CardRecord::new(
    "Dwalin, Weaponmaster",
    "196d9287-a37d-4b27-a83b-a5489a54f081",
    "Marco Teixeira",
    CardRules::unsupported(),
);

// HOB 155 — Eagle's Rescue
pub(in crate::card::sets) static EAGLE_S_RESCUE: CardRecord = CardRecord::new(
    "Eagle's Rescue",
    "12c8f2cc-ac9d-4cf6-9025-efe366b4e07f",
    "Ramza Psyru",
    CardRules::new_enchantment(mana_cost!("{2}{W/U}{W/U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            AbilityDef::activated_with_targets(
                "{2}{W/U}{W/U}: Return this card from your graveyard to the \
                 battlefield attached to target creature you control with \
                 power 1 or less. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{2}{W/U}{W/U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerLessThan(ValueDef::Constant(2)),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        attachment: Some(ArrivalAttachmentDef::ArrivalToHost(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// HOB 156 — Fearsome Goblin Pair
pub(in crate::card::sets) static FEARSOME_GOBLIN_PAIR: CardRecord = CardRecord::new(
    "Fearsome Goblin Pair",
    "2efe2dc7-3eaa-47f6-b1ae-f974c4a8ae79",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{2}{B/R}"), &["Goblin", "Soldier"], 1, 1).with_abilities(
        &[abilities::dies_trigger(
            "When this creature dies, amass Goblins 4. (Put four +1/+1 \
             counters on an Army you control. It's also a Goblin. If you \
             don't control an Army, create a 0/0 black Goblin Army \
             creature token first.)",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        GOBLIN_ARMY_TOKEN,
                    ))),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(4),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            effect: AppliedEffectDef::add_creature_types(
                                CreatureTypeSetDef::named(&["Goblin"]),
                            ),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                }),
            ]),
        )],
    ),
);

// HOB 157 — Goblin Plate Mail
pub(in crate::card::sets) static GOBLIN_PLATE_MAIL: CardRecord = CardRecord::new(
    "Goblin Plate Mail",
    "cb982607-da37-4894-91a5-cf6307d4d703",
    "Miklós Ligeti",
    CardRules::new_artifact(mana_cost!("{1}{B/R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, amass Goblins 1, then attach this \
                 Equipment to the amassed Army. (To amass Goblins 1, put a \
                 +1/+1 counter on an Army you control. It's also a Goblin. If \
                 you don't control an Army, create a 0/0 black Goblin Army \
                 creature token first.)",
                EffectDef::Sequence(&[
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Equal,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            GOBLIN_ARMY_TOKEN,
                        ))),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Army")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Goblin"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                            EffectDef::Attach {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                            },
                        ]),
                    }),
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has menace.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::menace()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// HOB 158 — The Great Goblin
// Audit: unsupported — Needs counter-placement events matching any counter kind and identifying the placing player, plus correctly bounded next-turn exile play permissions.
pub(in crate::card::sets) static THE_GREAT_GOBLIN: CardRecord = CardRecord::new(
    "The Great Goblin",
    "78d8f53e-537d-4eaa-99e3-cac57fa53d22",
    "Miklós Ligeti",
    CardRules::unsupported(),
);

// HOB 159 — Large Bear
pub(in crate::card::sets) static LARGE_BEAR: CardRecord = CardRecord::new(
    "Large Bear",
    "50202288-f433-4b56-8f60-349bda7b4f6b",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{3}{B/G}{B/G}"), &["Bear"], 5, 5).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::haste(),
    ]),
);

// HOB 160 — Mirkwood Nurturer
pub(in crate::card::sets) static MIRKWOOD_NURTURER: CardRecord = CardRecord::new(
    "Mirkwood Nurturer",
    "704b45e4-566e-40f6-a33a-9151018b44e5",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{2}{G/U}"), &["Elf", "Ranger"], 3, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one other target \
             permanent you control to its owner's hand. If you do, put a \
             +1/+1 counter on this creature.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("returned"),
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountObjects(&ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                            crate::Binding!("returned"),
                        )),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            },
        ),
    ]),
);

// HOB 161 — Nori, Teller of Tales
pub(in crate::card::sets) static NORI_TELLER_OF_TALES: CardRecord = CardRecord::new(
    "Nori, Teller of Tales",
    "b05adb48-980c-49a0-9ce6-7c7f3f20715d",
    "Marco Teixeira",
    CardRules::new_creature(mana_cost!("{1}{R/W}"), &["Dwarf", "Bard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever Nori attacks, target attacking creature gains first \
             strike until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// HOB 162 — Patient Instructor
pub(in crate::card::sets) static PATIENT_INSTRUCTOR: CardRecord = CardRecord::new(
    "Patient Instructor",
    "e4800508-8bb9-41bb-8712-b55fba7a80a5",
    "Joshua Raphael",
    CardRules::new_creature(mana_cost!("{2}{W/U}"), &["Human", "Citizen"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, recruit. (Draw a card, then \
             discard a card. If you discarded a nonland card, create a 1/1 \
             white Human Soldier creature token.)",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("recruits")),
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(HUMAN_SOLDIER_TOKEN)).with_count(
                                ValueDef::BoundObjectCount(crate::Binding!("recruits")),
                            ),
                        ),
                    }),
                },
            ]),
        ),
    ]),
);

// HOB 163 — Silvan Reveler
pub(in crate::card::sets) static SILVAN_REVELER: CardRecord = CardRecord::new(
    "Silvan Reveler",
    "c71b74fb-fb0c-4953-b536-7a3f283c6918",
    "Bokun An",
    CardRules::new_creature(mana_cost!("{2}{G}{U}"), &["Elf", "Citizen"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card, then discard a card. \
             If you discard a land card this way, put it from your \
             graveyard onto the battlefield tapped.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::HasType(CardType::Land),
                        bound: Some(crate::Binding!("discarded_land")),
                        effect: &EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("discarded_land"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                modifications: &[BattlefieldEntryModificationDef::Tapped],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                    }),
                },
            ]),
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, you may pay \
             {1}{G}{U}. If you do, return this card from your graveyard to \
             your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}{G}{U}"))],
                &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// HOB 164 — Smaug, Wicked Worm
// Audit: unsupported — Needs the spell's cast receipt to record mana produced by Treasure sources even when those sources were sacrificed and no longer exist.
pub(in crate::card::sets) static SMAUG_WICKED_WORM: CardRecord = CardRecord::new(
    "Smaug, Wicked Worm",
    "19cc91f0-e724-41ac-b6d8-9a293bd63b42",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// HOB 165 — Thorin Oakenshield
// Audit: unsupported — Needs the enduring-story player designation, acquired once when the artifact/legendary/Saga union reaches three and retained after those permanents leave; a live object count does not implement storied.
pub(in crate::card::sets) static THORIN_OAKENSHIELD: CardRecord = CardRecord::new(
    "Thorin Oakenshield",
    "c7e18609-d1ed-4829-be11-f2ce2cfcbc49",
    "Francisco Miyara",
    CardRules::unsupported(),
);

// HOB 166 — Thranduil, Sindarin Liege // Silvan Rally
pub(in crate::card::sets) static THRANDUIL_SINDARIN_LIEGE: CardRecord = CardRecord::new(
    "Thranduil, Sindarin Liege // Silvan Rally",
    "481870ee-d1f7-421b-86e1-570ea933bbbc",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{2}{G/U}{G/U}"), &["Elf", "Noble"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Elves you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, create a 1/1 \
                 green Elf creature token.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELF_TOKEN))),
            ),
        ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{1}{G/U}{G/U}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Mill four cards, then put up to two land cards from among \
                 them into your hand. (Then exile this card. You may cast the \
                 creature later from exile.)",
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(4),
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 2,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&THRANDUIL_SINDARIN_LIEGE, "Silvan Rally", &ALTERNATE)
});

// HOB 167 — Thranduil, the Elvenking
// Audit: unsupported — Needs a dynamic grant of activated abilities from all qualifying graveyard Elf cards, including their source remapping and activation cost contexts.
pub(in crate::card::sets) static THRANDUIL_THE_ELVENKING: CardRecord = CardRecord::new(
    "Thranduil, the Elvenking",
    "fe2fe8fa-3b99-44c1-bab9-922e5c864952",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// HOB 168 — Thranduil's Company
pub(in crate::card::sets) static THRANDUIL_S_COMPANY: CardRecord = CardRecord::new(
    "Thranduil's Company",
    "abdb9d4e-e6ca-409b-b589-0cf71724340b",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{2}{G}{U}"), &["Elf", "Soldier"], 3, 4).with_abilities(&[
        AbilityDef::static_ability(
            "As long as you control another Elf, you may play an \
             additional land on each of your turns.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
                },
            },
        ),
        AbilityDef::triggered_with_targets(
            "Landfall — Whenever a land you control enters, put two +1/+1 \
             counters on target creature you control. It gains vigilance \
             until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// HOB 169 — Tom, Bert, and William
// Audit: unsupported — Needs a noncopiable artifact-only type change on the prospective returning permanent before it enters; applying it after entry changes triggers and replacement eligibility.
pub(in crate::card::sets) static TOM_BERT_AND_WILLIAM: CardRecord = CardRecord::new(
    "Tom, Bert, and William",
    "211a9764-3c60-46ba-bb53-e6692640ec8f",
    "Leonardo Borazio",
    CardRules::unsupported(),
);

// HOB 170 — The Arkenstone // Seek the Heart
pub(in crate::card::sets) static THE_ARKENSTONE: CardRecord = CardRecord::new(
    "The Arkenstone // Seek the Heart",
    "a56a88ba-fcfa-4b56-bdae-a080b297b871",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_sorcery(mana_cost!("{2}{W}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell(
                "Search your library for a legendary creature card, reveal it, \
                 put it into your hand, then shuffle. (Then exile this card. \
                 You may cast the artifact later from exile.)",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
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
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&THE_ARKENSTONE, "Seek the Heart", &ALTERNATE)
});

// HOB 171 — The Black Arrow
// Audit: unsupported — Needs a bound receipt identifying whether the targeted Dragon actually received damage, including prevention and redirection, before deciding to destroy it.
pub(in crate::card::sets) static THE_BLACK_ARROW: CardRecord = CardRecord::new(
    "The Black Arrow",
    "ab181190-d53d-4972-8cd5-8e54b45f2276",
    "Kevin Sidharta",
    CardRules::unsupported(),
);

// HOB 172 — Dwarven Mattock
pub(in crate::card::sets) static DWARVEN_MATTOCK: CardRecord = CardRecord::new(
    "Dwarven Mattock",
    "92c6f09d-b525-4e8c-a87c-a74df9dc3b1e",
    "Nino Is",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target Dwarf you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2 and has ward {1}. (Whenever \
                 equipped creature becomes the target of a spell or ability an \
                 opponent controls, counter it unless that player pays {1}.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::ward(
                            &[CostDef::Mana(mana_cost!("{1}"))],
                            "Ward {1}",
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// HOB 173 — Giant's Boulder
pub(in crate::card::sets) static GIANT_S_BOULDER: CardRecord = CardRecord::new(
    "Giant's Boulder",
    "ce254758-c928-4b43-a952-13fac1845668",
    "Miklós Ligeti",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, scry 2. (Look at the top two cards \
             of your library, then put any number of them on the bottom \
             and the rest on top in any order.)",
            abilities::scry(ValueDef::Constant(2)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "{7}, {T}, Sacrifice this artifact: Destroy target permanent.",
            &[
                CostDef::Mana(mana_cost!("{7}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// HOB 174 — Glamdring, Foe-hammer // Gleam of Death
// Audit: unsupported — Needs a spell-cost reduction read from the currently equipped creature's power; the cost evaluator cannot read ObjectPower(AttachedToSource).
pub(in crate::card::sets) static GLAMDRING_FOE_HAMMER: CardRecord = CardRecord::new(
    "Glamdring, Foe-hammer // Gleam of Death",
    "a5cfbfde-783e-46ca-b3cf-11f16209d6cb",
    "Chris Cold",
    CardRules::unsupported(),
);

// HOB 175 — Key to the Side-Door
// Audit: unsupported — Needs a discard-cost predicate joining the candidate legendary card's name to the names of legendary permanents its payer controls.
pub(in crate::card::sets) static KEY_TO_THE_SIDE_DOOR: CardRecord = CardRecord::new(
    "Key to the Side-Door",
    "898c14a2-d897-4341-83ed-eee666df9648",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// HOB 176 — My Precious // Allure of Power
pub(in crate::card::sets) static MY_PRECIOUS: CardRecord = CardRecord::new(
    "My Precious // Allure of Power",
    "15ae4d50-be2f-412c-bb6b-b0a06b60474a",
    "Valera Lutfullina",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has hexproof and can't be blocked.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::PayLife(2)],
                "Equip—{2}, Pay 2 life.",
            ),
        ]),
)
.with_composition(|| {
    const ALTERNATE: CardRules = CardRules::new_instant(mana_cost!("{1}{B}"))
        .with_subtypes(&["Adventure"])
        .with_ability(
            AbilityDef::spell_with_additional_cost(
                "As an additional cost to cast this spell, sacrifice a \
                 creature.\nDraw two cards.",
                &[],
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
                abilities::draw_cards(ValueDef::Constant(2)),
            )
            .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
        );
    adventure(&MY_PRECIOUS, "Allure of Power", &ALTERNATE)
});

// HOB 177 — Orcrist, Goblin-cleaver
// Audit: unsupported — Needs a resolving creature-type choice bound to the following token-count query; creature-type choices currently exist only as battlefield-entry choices.
pub(in crate::card::sets) static ORCRIST_GOBLIN_CLEAVER: CardRecord = CardRecord::new(
    "Orcrist, Goblin-cleaver",
    "f54f1c1d-6a22-43e9-a842-0a1ae25b323c",
    "Erikas Perl",
    CardRules::unsupported(),
);

// HOB 178 — Sting, Bilbo's Sword
// Audit: unsupported — Needs hone counters with their intrinsic Equipment effect granting +1/+0 per counter to the equipped creature, independent of printed abilities and preserved through ability removal.
pub(in crate::card::sets) static STING_BILBO_S_SWORD: CardRecord = CardRecord::new(
    "Sting, Bilbo's Sword",
    "d6a8d698-c454-42c4-ad4e-9a7625d5569f",
    "Tomas Duchek",
    CardRules::unsupported(),
);

// HOB 179 — Thrór's Map
pub(in crate::card::sets) static THROR_S_MAP: CardRecord = CardRecord::new(
    "Thrór's Map",
    "ad0dba36-d056-4bc1-987a-391da26ad267",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Thrór's Map enters, search your library for a basic land \
                 card, reveal it, put it into your hand, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
            ),
            AbilityDef::activated(
                "{2}, {T}: Draw a card, then discard a card.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ]),
);

// HOB 180 — Well-Worn Spatula
pub(in crate::card::sets) static WELL_WORN_SPATULA: CardRecord = CardRecord::new(
    "Well-Worn Spatula",
    "659b687f-4068-496f-81b2-7b606bf07ec1",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, you gain 2 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// HOB 181 — Elven Passage
// Audit: unsupported — Needs a behold-Elf choice over a controlled permanent or a revealed hand card, retaining the optional choice through the search-result untap continuation; the existing named-action vocabulary has no behold action.
pub(in crate::card::sets) static ELVEN_PASSAGE: CardRecord = CardRecord::new(
    "Elven Passage",
    "dd1fd2ab-2565-4798-a832-fc849df82f74",
    "Shahab Alizadeh",
    CardRules::unsupported(),
);

// HOB 182 — Elvenking's Halls
pub(in crate::card::sets) static ELVENKING_S_HALLS: CardRecord = CardRecord::new(
    "Elvenking's Halls",
    "cd477096-41b1-4907-9cb3-852cb22c9ba2",
    "Leon Tukker",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}{G}{U}, {T}, Sacrifice this land: Put two +1/+1 counters \
             on target Elf you control. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{G}{U}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// HOB 183 — Goblin-town
pub(in crate::card::sets) static GOBLIN_TOWN: CardRecord = CardRecord::new(
    "Goblin-town",
    "d76df9d0-56cf-4351-a5e8-e6ae6fc791d1",
    "Sean Vo",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}{B}{R}, {T}, Sacrifice this land: Put two +1/+1 counters \
             on target Goblin or Orc you control. Activate only as a \
             sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{B}{R}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Orc")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// HOB 184 — Hobbit Hole
pub(in crate::card::sets) static HOBBIT_HOLE: CardRecord = CardRecord::new(
    "Hobbit Hole",
    "0365c439-30bf-4d32-a791-166751bdb996",
    "Shahab Alizadeh",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice this land: Search your library for a basic \
             land card, put it onto the battlefield tapped, then shuffle.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        abilities::typecycling!(
            "Halflingcycling {4} ({4}, Discard this card: Search your \
             library for a Halfling card, reveal it, put it into your \
             hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{4}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Halfling"))
        ),
    ]),
);

// HOB 185 — Iron Hills
pub(in crate::card::sets) static IRON_HILLS: CardRecord = CardRecord::new(
    "Iron Hills",
    "78045c43-5cbe-48ff-837d-e7c6baac2937",
    "Marina Ortega Lorente",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}{R}{W}, {T}, Sacrifice this land: Put two +1/+1 counters \
             on target Dwarf you control. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}{W}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// HOB 186 — Lake-town
pub(in crate::card::sets) static LAKE_TOWN: CardRecord = CardRecord::new(
    "Lake-town",
    "2fbd0584-81a7-4c47-8af1-1c8635899a97",
    "Marina Ortega Lorente",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}{W}{U}, {T}, Sacrifice this land: Put two +1/+1 counters \
             on target Human you control. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{W}{U}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Human")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// HOB 187 — The Lonely Mountain
pub(in crate::card::sets) static THE_LONELY_MOUNTAIN: CardRecord = CardRecord::new(
    "The Lonely Mountain",
    "b39ebc4d-a01a-4401-ab3a-bf6142c93b47",
    "Leon Tukker",
    CardRules::new_land(&["Mountain"]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control an Equipment.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        AbilityDef::activated(
            "{4}{R}, {T}: Create a 2/2 red Dwarf creature token. This \
             ability costs {1} less to activate for each Equipment you \
             control. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{4}{R}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DWARF_TOKEN))),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::static_ability(
            "This ability costs {1} less to activate for each Equipment \
             you control.",
            EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                permanent: ObjectPredicateDef::Source,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                minimum: 0,
            }),
        ),
    ]),
);

// HOB 188 — Mirkwood
pub(in crate::card::sets) static MIRKWOOD: CardRecord = CardRecord::new(
    "Mirkwood",
    "612cf954-f86c-4629-99df-4874d56fded3",
    "Leon Tukker",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated_with_targets(
            "{2}{B}{G}, {T}, Sacrifice this land: Put two +1/+1 counters \
             on target Bear, Spider, or Wolf you control. Activate only as \
             a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{B}{G}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bear")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wolf")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// HOB 189 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "7b7c408b-8660-4db5-9a16-5003c11b4ac1",
    "Chris Cold",
);

// HOB 190 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "c6aa89a8-3584-4906-b9a9-41ef2f021f8e",
    "Kamila Szutenberg",
);

// HOB 191 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "4031e5e4-e573-4130-8d20-4a606edef0a0",
    "Erikas Perl",
);

// HOB 192 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "c49d378e-9549-4320-b3c6-1aeb216d1e98",
    "Shahab Alizadeh",
);

// HOB 193 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "c3e84b42-5423-4d4d-b8fc-cfbb2c53a4ca",
    "Kamila Szutenberg",
);

// HOB 194 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "24dc369c-020a-4115-a4bb-d60a44de64e3",
    "WFlemming Illustration",
);

// HOB 195 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "739aaaac-c424-4ea7-a084-62a6fc0438b0",
    "WFlemming Illustration",
);

// HOB 196 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "c5f590a3-9993-4ac4-a93c-1beb44eda17b",
    "WFlemming Illustration",
);

// HOB 197 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "51acfb01-4b0b-48fc-9704-a9b4a1e43a23",
    "WFlemming Illustration",
);

// HOB 198 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "5f533364-0f91-4e49-aaeb-83c4c1f6d316",
    "WFlemming Illustration",
);

// HOB 199 — Troop of Ponies (alternate printing)
const TROOP_OF_PONIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TROOP_OF_PONIES,
    1,
    "6cbcd606-b864-4b81-9596-e7788befdd1f",
    "Ted Nasmith",
);

// HOB 200 — Rage into the Valley (alternate printing)
const RAGE_INTO_THE_VALLEY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGE_INTO_THE_VALLEY,
    1,
    "349d7464-e778-4a24-a528-d31d36ff4799",
    "Ted Nasmith",
);

// HOB 201 — The Great Goblin (alternate printing)
const THE_GREAT_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GREAT_GOBLIN,
    1,
    "720ae752-c3a7-4bc0-bc0f-a9772bea3960",
    "Ted Nasmith",
);

// HOB 202 — Thorin Oakenshield (alternate printing)
const THORIN_OAKENSHIELD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_OAKENSHIELD,
    1,
    "b6d8d14d-4713-40ea-9509-81ce68b2184d",
    "Ted Nasmith",
);

// HOB 203 — Gandalf, Spark Starter (alternate printing)
const GANDALF_SPARK_STARTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_SPARK_STARTER,
    1,
    "ccf2923e-5358-4e40-bc64-2953ac2d1692",
    "Ted Nasmith",
);

// HOB 204 — Glamdring, Foe-hammer // Gleam of Death (alternate printing)
const GLAMDRING_FOE_HAMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLAMDRING_FOE_HAMMER,
    1,
    "53cd94e9-6006-4d43-8032-7920103740fa",
    "Ted Nasmith",
);

// HOB 205 — The Eagles Are Coming! (alternate printing)
const THE_EAGLES_ARE_COMING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EAGLES_ARE_COMING,
    1,
    "62a7ff97-b940-4745-9e2e-3258fb51b846",
    "Denman Rooke",
);

// HOB 206 — Dreaded Bat-Cloud (alternate printing)
const DREADED_BAT_CLOUD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREADED_BAT_CLOUD,
    1,
    "1d84b0f8-aa9d-46b5-b46d-cb291aa6af9a",
    "Denman Rooke",
);

// HOB 207 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    1,
    "0015ae49-bc21-4cf3-bb46-3df52760e183",
    "Denman Rooke",
);

// HOB 208 — Chief Warg's Company (alternate printing)
const CHIEF_WARG_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHIEF_WARG_S_COMPANY,
    1,
    "0dd7b2d8-ca9d-4125-b57e-9b14deae8d9b",
    "Denman Rooke",
);

// HOB 209 — Thorin's Last Stand (alternate printing)
const THORIN_S_LAST_STAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_S_LAST_STAND,
    1,
    "089e8e4b-c367-4d86-a69a-951f7d779aaf",
    "Denman Rooke",
);

// HOB 210 — Bard's Company (alternate printing)
const BARD_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARD_S_COMPANY,
    1,
    "7ca1300a-0ac3-413b-9617-5e8d1b5f1ca2",
    "Denman Rooke",
);

// HOB 211 — Bolg's Company (alternate printing)
const BOLG_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOLG_S_COMPANY,
    1,
    "a2ba9d82-83b6-43dd-8eb8-2401be0df754",
    "Denman Rooke",
);

// HOB 212 — Dáin's Company (alternate printing)
const DAIN_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAIN_S_COMPANY,
    1,
    "323f7ae8-9247-479e-aead-f46018a3d81d",
    "Denman Rooke",
);

// HOB 213 — Thranduil's Company (alternate printing)
const THRANDUIL_S_COMPANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_S_COMPANY,
    1,
    "fcdfce79-c8da-4662-93c2-054e5328e18c",
    "Denman Rooke",
);

// HOB 214 — Belladonna Took (alternate printing)
const BELLADONNA_TOOK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BELLADONNA_TOOK,
    1,
    "0ff767b1-b04d-4d1d-bc91-3bd1361c5ee8",
    "Kristina Carroll",
);

// HOB 215 — Bofur, Reliable Guardian // Concerted Care (alternate printing)
const BOFUR_RELIABLE_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOFUR_RELIABLE_GUARDIAN,
    1,
    "a221baee-6ed1-4f11-b38c-e0be8531e170",
    "Cory Godbey",
);

// HOB 216 — Iron Hills Blacksmith (alternate printing)
const IRON_HILLS_BLACKSMITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_HILLS_BLACKSMITH,
    1,
    "6b428a9a-3a59-4416-b00b-78e2cb73e2d6",
    "Francisco Badilla",
);

// HOB 217 — The Queen of Dale (alternate printing)
const THE_QUEEN_OF_DALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_QUEEN_OF_DALE,
    1,
    "7d635dfb-0a48-4965-82d1-d98f943bb28d",
    "Ashley Mackenzie",
);

// HOB 218 — Bilbo, Luckwearer // Burglar's Plot (alternate printing)
const BILBO_LUCKWEARER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_LUCKWEARER,
    1,
    "b5fe776d-00c2-4d5d-9493-77dd754aa728",
    "Cory Godbey",
);

// HOB 219 — Bilbo, Thief in the Night (alternate printing)
const BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_THIEF_IN_THE_NIGHT,
    1,
    "dda07973-ed78-4f50-8794-f5cbfc6e8975",
    "Ashley Mackenzie",
);

// HOB 220 — Fateful Discovery (alternate printing)
const FATEFUL_DISCOVERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FATEFUL_DISCOVERY,
    1,
    "958f0de2-d209-4e97-9ad4-dedf5da304b8",
    "Ashley Mackenzie",
);

// HOB 221 — Most Decrepit Old Bird // Speak Secrets (alternate printing)
const MOST_DECREPIT_OLD_BIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOST_DECREPIT_OLD_BIRD,
    1,
    "9093b067-e18e-4a6a-b4a6-cd67bcd4d5a7",
    "Andrea Radeck",
);

// HOB 222 — Azog, Moria's Ruin (alternate printing)
const AZOG_MORIA_S_RUIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AZOG_MORIA_S_RUIN,
    1,
    "0b9ef17a-02a4-44c6-b0b3-9eccf7f324fd",
    "Dibujante Nocturno",
);

// HOB 223 — Great Ugly-Looking Goblin // Clap! Snap! (alternate printing)
const GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_UGLY_LOOKING_GOBLIN,
    1,
    "7f63f810-15a8-4bec-bd39-dbca453bcc1e",
    "Andrea Radeck",
);

// HOB 224 — Head of the Hunt (alternate printing)
const HEAD_OF_THE_HUNT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEAD_OF_THE_HUNT,
    1,
    "d670ed30-e7e0-4908-ac9c-bdcab61092ef",
    "Benjamin Ee",
);

// HOB 225 — Desert Were-Worm (alternate printing)
const DESERT_WERE_WORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DESERT_WERE_WORM,
    1,
    "3f4794de-701d-4619-8b8b-cbfdd860c567",
    "Dibujante Nocturno",
);

// HOB 226 — Desolation of Smaug (alternate printing)
const DESOLATION_OF_SMAUG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DESOLATION_OF_SMAUG,
    1,
    "738d50ff-1bff-4841-93de-ccf521b2542f",
    "Dibujante Nocturno",
);

// HOB 227 — Glóin the Mighty // Easy Pickings (alternate printing)
const GLOIN_THE_MIGHTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOIN_THE_MIGHTY,
    1,
    "7c9bfc6e-648a-482b-9483-9e8538726f53",
    "Francisco Badilla",
);

// HOB 228 — Last Light of Durin's Day (alternate printing)
const LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAST_LIGHT_OF_DURIN_S_DAY,
    1,
    "ba42b0f3-3f2f-4fbe-b56d-c2cb477f1182",
    "Francisco Badilla",
);

// HOB 229 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    1,
    "4edcf8eb-d0b0-4aec-bec0-64c56982cdce",
    "John Tedrick",
);

// HOB 230 — Beorn the Fierce (alternate printing)
const BEORN_THE_FIERCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEORN_THE_FIERCE,
    1,
    "949e4c23-d0e7-4e11-abd4-33ff78b1c963",
    "Dibujante Nocturno",
);

// HOB 231 — Dancing from Dark to Dawn (alternate printing)
const DANCING_FROM_DARK_TO_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DANCING_FROM_DARK_TO_DAWN,
    1,
    "85f488cf-eb8a-43f6-9ed8-523daba284fa",
    "Francisco Badilla",
);

// HOB 232 — The Notary Hobbits (alternate printing)
const THE_NOTARY_HOBBITS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_NOTARY_HOBBITS,
    1,
    "b2b6ac15-2266-442a-99e5-96db904e1fd7",
    "Ashley Mackenzie",
);

// HOB 233 — Thranduil, Sindarin Liege // Silvan Rally (alternate printing)
const THRANDUIL_SINDARIN_LIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_SINDARIN_LIEGE,
    1,
    "0992ce8b-7bef-4799-a52d-7a46021a8672",
    "Cory Godbey",
);

// HOB 234 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    1,
    "ba3ff39b-b591-4434-8d41-4826d0809ca7",
    "Francisco Badilla",
);

// HOB 235 — My Precious // Allure of Power (alternate printing)
const MY_PRECIOUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MY_PRECIOUS,
    1,
    "ceb193ed-b05f-4820-a17f-e2982bf547ae",
    "Barbara Rosiak",
);

// HOB 236 — Orcrist, Goblin-cleaver (alternate printing)
const ORCRIST_GOBLIN_CLEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORCRIST_GOBLIN_CLEAVER,
    1,
    "758b4398-4522-49c8-94de-b0d647532544",
    "Francisco Badilla",
);

// HOB 237 — Sting, Bilbo's Sword (alternate printing)
const STING_BILBO_S_SWORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STING_BILBO_S_SWORD,
    1,
    "30420a65-65f2-4331-9a5d-b1771da936b0",
    "Barbara Rosiak",
);

// HOB 238 — Elven Passage (alternate printing)
const ELVEN_PASSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_PASSAGE,
    1,
    "c9789696-a702-4294-afd7-c48e5fc84398",
    "Kristina Carroll",
);

// HOB 239 — Gleaming Splendor (alternate printing)
const GLEAMING_SPLENDOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_SPLENDOR,
    1,
    "4a4ca473-3000-4e98-9881-03372c4f79a5",
    "James Bousema",
);

// HOB 240 — The Lord of the Eagles (alternate printing)
const THE_LORD_OF_THE_EAGLES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LORD_OF_THE_EAGLES,
    1,
    "8acfaac6-5a63-4fd6-999b-562e989e2ce1",
    "Tyler Jacobson",
);

// HOB 241 — Gollum, Riddle Master (alternate printing)
const GOLLUM_RIDDLE_MASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLLUM_RIDDLE_MASTER,
    1,
    "0c13e282-bbfb-4601-94ff-803874634371",
    "Tyler Jacobson",
);

// HOB 242 — Gandalf, Goblins' Bane // Flameshape (alternate printing)
const GANDALF_GOBLINS_BANE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_GOBLINS_BANE,
    1,
    "efc66a6b-8ac7-4cb7-82f1-710fb0e86f82",
    "James Bousema",
);

// HOB 243 — Thorin, Mountain-king (alternate printing)
const THORIN_MOUNTAIN_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORIN_MOUNTAIN_KING,
    1,
    "2401d2a5-f65b-4a13-906c-999990589a1e",
    "James Bousema",
);

// HOB 244 — Bard, King of Dale (alternate printing)
const BARD_KING_OF_DALE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARD_KING_OF_DALE,
    1,
    "a4adb258-381e-4e64-9f93-9fcf943be360",
    "James Bousema",
);

// HOB 245 — Smaug, Wicked Worm (alternate printing)
const SMAUG_WICKED_WORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_WICKED_WORM,
    1,
    "189a7d55-c3ec-4402-a3b3-72817f533e87",
    "Tyler Jacobson",
);

// HOB 246 — Thranduil, the Elvenking (alternate printing)
const THRANDUIL_THE_ELVENKING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_THE_ELVENKING,
    1,
    "a416ca91-e646-40b4-824c-24aed6b9c683",
    "James Bousema",
);

// HOB 247 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    2,
    "6c6193e8-453b-49dd-ad65-ee6e363bff93",
    "Tyler Jacobson",
);

// HOB 248 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    2,
    "d9919a30-d60e-438e-b132-22e56b3cdf68",
    "Tyler Jacobson",
);

// HOB 249 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    2,
    "43c56e15-ba46-4a27-b9b0-55cde9f8c933",
    "Ted Nasmith",
);

// HOB 250 — Belladonna Took (alternate printing)
const BELLADONNA_TOOK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BELLADONNA_TOOK,
    2,
    "f2522398-39d2-43d3-9019-349e22438f65",
    "Kristina Carroll",
);

// HOB 251 — Bofur, Reliable Guardian // Concerted Care (alternate printing)
const BOFUR_RELIABLE_GUARDIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BOFUR_RELIABLE_GUARDIAN,
    2,
    "7d9a89d1-3122-4fd8-b77f-5163c01b560c",
    "Cory Godbey",
);

// HOB 252 — Iron Hills Blacksmith (alternate printing)
const IRON_HILLS_BLACKSMITH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &IRON_HILLS_BLACKSMITH,
    2,
    "8b592de0-0bf9-4972-9f43-c8ac1c3a1299",
    "Francisco Badilla",
);

// HOB 253 — The Queen of Dale (alternate printing)
const THE_QUEEN_OF_DALE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_QUEEN_OF_DALE,
    2,
    "4cff1878-3f23-4a0a-ad79-28a44921cca8",
    "Ashley Mackenzie",
);

// HOB 254 — Bilbo, Luckwearer // Burglar's Plot (alternate printing)
const BILBO_LUCKWEARER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BILBO_LUCKWEARER,
    2,
    "79892a5d-80df-4ee1-b482-30e57aaabf21",
    "Cory Godbey",
);

// HOB 255 — Bilbo, Thief in the Night (alternate printing)
const BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BILBO_THIEF_IN_THE_NIGHT,
    2,
    "c0c313d6-5a76-459d-9db1-9491c57861fe",
    "Ashley Mackenzie",
);

// HOB 256 — Fateful Discovery (alternate printing)
const FATEFUL_DISCOVERY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FATEFUL_DISCOVERY,
    2,
    "6a928a14-b906-402f-96e4-8b9391d6536c",
    "Ashley Mackenzie",
);

// HOB 257 — Most Decrepit Old Bird // Speak Secrets (alternate printing)
const MOST_DECREPIT_OLD_BIRD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOST_DECREPIT_OLD_BIRD,
    2,
    "289a6c72-3d0d-402d-8e27-9ad0e254d5f6",
    "Andrea Radeck",
);

// HOB 258 — Azog, Moria's Ruin (alternate printing)
const AZOG_MORIA_S_RUIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AZOG_MORIA_S_RUIN,
    2,
    "2e359014-f003-4dd6-bc97-ef2a5c515a23",
    "Dibujante Nocturno",
);

// HOB 259 — Great Ugly-Looking Goblin // Clap! Snap! (alternate printing)
const GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GREAT_UGLY_LOOKING_GOBLIN,
    2,
    "68fe7914-21b4-44eb-9432-3b1637864bc1",
    "Andrea Radeck",
);

// HOB 260 — Head of the Hunt (alternate printing)
const HEAD_OF_THE_HUNT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HEAD_OF_THE_HUNT,
    2,
    "77d2a543-d94f-4814-a60f-72c224e5768a",
    "Benjamin Ee",
);

// HOB 261 — Desert Were-Worm (alternate printing)
const DESERT_WERE_WORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DESERT_WERE_WORM,
    2,
    "cfe91642-7651-4902-8386-b18cfb506925",
    "Dibujante Nocturno",
);

// HOB 262 — Desolation of Smaug (alternate printing)
const DESOLATION_OF_SMAUG_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DESOLATION_OF_SMAUG,
    2,
    "4bb4033e-8b96-4c35-ab03-ff188002ae73",
    "Dibujante Nocturno",
);

// HOB 263 — Glóin the Mighty // Easy Pickings (alternate printing)
const GLOIN_THE_MIGHTY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLOIN_THE_MIGHTY,
    2,
    "5546390b-3957-4e3b-aa64-85692381f53f",
    "Francisco Badilla",
);

// HOB 264 — Last Light of Durin's Day (alternate printing)
const LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LAST_LIGHT_OF_DURIN_S_DAY,
    2,
    "7cb22e85-8d54-48a9-a006-d6e2f3ff00e1",
    "Francisco Badilla",
);

// HOB 265 — Smaug the Magnificent (alternate printing)
const SMAUG_THE_MAGNIFICENT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_THE_MAGNIFICENT,
    3,
    "b6529666-0ec8-45a8-8740-3bffc19d2265",
    "John Tedrick",
);

// HOB 266 — Beorn the Fierce (alternate printing)
const BEORN_THE_FIERCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BEORN_THE_FIERCE,
    2,
    "9f921e6b-12f0-4497-86ee-f2b4f4fa319a",
    "Dibujante Nocturno",
);

// HOB 267 — Dancing from Dark to Dawn (alternate printing)
const DANCING_FROM_DARK_TO_DAWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DANCING_FROM_DARK_TO_DAWN,
    2,
    "9fa9a9de-ca46-4ea1-8fdb-4ccd3ef86805",
    "Francisco Badilla",
);

// HOB 268 — The Notary Hobbits (alternate printing)
const THE_NOTARY_HOBBITS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_NOTARY_HOBBITS,
    2,
    "0e88b614-f5b3-4e1e-beed-3c49e366be11",
    "Ashley Mackenzie",
);

// HOB 269 — Thranduil, Sindarin Liege // Silvan Rally (alternate printing)
const THRANDUIL_SINDARIN_LIEGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_SINDARIN_LIEGE,
    2,
    "a84cd965-fe5f-42e5-9775-d0285fe84308",
    "Cory Godbey",
);

// HOB 270 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    3,
    "2d08a806-c70d-4421-a9eb-e29e54e4268f",
    "Francisco Badilla",
);

// HOB 271 — My Precious // Allure of Power (alternate printing)
const MY_PRECIOUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MY_PRECIOUS,
    2,
    "ce239994-1bc6-4083-b9cf-776e862c5479",
    "Barbara Rosiak",
);

// HOB 272 — Orcrist, Goblin-cleaver (alternate printing)
const ORCRIST_GOBLIN_CLEAVER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORCRIST_GOBLIN_CLEAVER,
    2,
    "a496a9e4-8dd7-44a2-a4b2-d4ecfc8ee7c8",
    "Francisco Badilla",
);

// HOB 273 — Sting, Bilbo's Sword (alternate printing)
const STING_BILBO_S_SWORD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STING_BILBO_S_SWORD,
    2,
    "7d57f11f-1698-4cf3-bb0e-bd9b0ea8442d",
    "Barbara Rosiak",
);

// HOB 274 — Elven Passage (alternate printing)
const ELVEN_PASSAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_PASSAGE,
    2,
    "75cb4095-b252-49b0-ad2e-0ea52ae4709a",
    "Kristina Carroll",
);

// HOB 275 — Gleaming Splendor (alternate printing)
const GLEAMING_SPLENDOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_SPLENDOR,
    2,
    "42a1986c-9585-4544-b5a7-bee4be5c4506",
    "James Bousema",
);

// HOB 276 — The Lord of the Eagles (alternate printing)
const THE_LORD_OF_THE_EAGLES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LORD_OF_THE_EAGLES,
    2,
    "b0f29cce-324d-4229-8e29-692594b2e3c7",
    "Tyler Jacobson",
);

// HOB 277 — Gollum, Riddle Master (alternate printing)
const GOLLUM_RIDDLE_MASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOLLUM_RIDDLE_MASTER,
    2,
    "12d441b7-9349-47a1-b61e-9d563bc085b5",
    "Tyler Jacobson",
);

// HOB 278 — Gandalf, Goblins' Bane // Flameshape (alternate printing)
const GANDALF_GOBLINS_BANE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GANDALF_GOBLINS_BANE,
    2,
    "0db45ef8-5bc5-4914-9918-dae4ebf48153",
    "James Bousema",
);

// HOB 279 — Thorin, Mountain-king (alternate printing)
const THORIN_MOUNTAIN_KING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THORIN_MOUNTAIN_KING,
    2,
    "0e29ba51-763c-48ec-95ec-9d6916f4db44",
    "James Bousema",
);

// HOB 280 — Bard, King of Dale (alternate printing)
const BARD_KING_OF_DALE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BARD_KING_OF_DALE,
    2,
    "f7c75c32-a24f-46e5-9128-2df9a2ec11f9",
    "James Bousema",
);

// HOB 281 — Smaug, Wicked Worm (alternate printing)
const SMAUG_WICKED_WORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SMAUG_WICKED_WORM,
    2,
    "5091e2bd-247d-4a1d-adf4-6e66324ee20b",
    "Tyler Jacobson",
);

// HOB 282 — Thranduil, the Elvenking (alternate printing)
const THRANDUIL_THE_ELVENKING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THRANDUIL_THE_ELVENKING,
    2,
    "49539856-989e-4f95-8e67-9cff4408cd87",
    "James Bousema",
);

// HOB 283 — The Arkenstone // Seek the Heart (alternate printing)
const THE_ARKENSTONE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &THE_ARKENSTONE,
    4,
    "9a0d62ce-8fd3-4465-b3aa-5d5052df30dc",
    "Tyler Jacobson",
);

// HOB 284 — The Lonely Mountain (alternate printing)
const THE_LONELY_MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_LONELY_MOUNTAIN,
    3,
    "edff761b-2c2a-414f-b0a3-25c3fdbcb0bc",
    "Tyler Jacobson",
);

// HOB 285 — Bilbo's Gambit (alternate printing)
const BILBO_S_GAMBIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BILBO_S_GAMBIT,
    1,
    "fa74ca8a-8bcd-4dc5-ab2b-a2e18a70978e",
    "Randy Gallegos",
);

// HOB 286 — Fíli the Pathfinder (alternate printing)
const FILI_THE_PATHFINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FILI_THE_PATHFINDER,
    1,
    "0e4a1096-c40e-4471-8a44-9c2acad85769",
    "Valera Lutfullina",
);

// HOB 287 — Kíli the Resourceful (alternate printing)
const KILI_THE_RESOURCEFUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KILI_THE_RESOURCEFUL,
    1,
    "c241a594-24bc-4fd9-ac6d-501d11dddad1",
    "Yuhong Ding",
);

// HOB 288 — Settle the Wreckage (alternate printing)
const SETTLE_THE_WRECKAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_xln::SETTLE_THE_WRECKAGE,
    1,
    "a132a86d-1df2-42d1-a410-2a8479c84a55",
    "Chris Cold",
);

// HOB 289 — An Unexpected Party // At the Door (alternate printing)
const AN_UNEXPECTED_PARTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AN_UNEXPECTED_PARTY,
    1,
    "f75207fd-89d2-417b-8f36-4ea96b4e3794",
    "Matt Stewart",
);

// HOB 290 — Elrond, Moon-Reader (alternate printing)
const ELROND_MOON_READER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELROND_MOON_READER,
    1,
    "3e95e9c2-cdbf-48b8-a7d0-87267f58e5ac",
    "Christina Kraus",
);

// HOB 291 — Great Gilded Boat (alternate printing)
const GREAT_GILDED_BOAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_GILDED_BOAT,
    1,
    "d2b59872-0e11-41c3-9858-3e2dd5a1c3c3",
    "Josu Solano",
);

// HOB 292 — Riddles in the Dark (alternate printing)
const RIDDLES_IN_THE_DARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIDDLES_IN_THE_DARK,
    1,
    "234e941d-ad86-4a3e-bd8c-0e680944919a",
    "Lorenzo Mastroianni",
);

// HOB 293 — Uncover the Moon-Letters (alternate printing)
const UNCOVER_THE_MOON_LETTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNCOVER_THE_MOON_LETTERS,
    1,
    "1849eb57-7560-4ffc-9400-02387b1a71e6",
    "Leon Tukker",
);

// HOB 294 — Wizard's Staff (alternate printing)
const WIZARD_S_STAFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WIZARD_S_STAFF,
    1,
    "14e69ca4-5148-44ec-bb21-330e656a6833",
    "Gaboleps",
);

// HOB 295 — Along the Crooked Way (alternate printing)
const ALONG_THE_CROOKED_WAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALONG_THE_CROOKED_WAY,
    1,
    "e340aa1c-a497-4fc4-9be9-26e7c982f893",
    "Bruce Brenneise",
);

// HOB 296 — Inside Information (alternate printing)
const INSIDE_INFORMATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSIDE_INFORMATION,
    1,
    "5913d004-0ef6-4dce-8f74-a8fbfc794d43",
    "Sean Vo",
);

// HOB 297 — The Master of Lake-town (alternate printing)
const THE_MASTER_OF_LAKE_TOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MASTER_OF_LAKE_TOWN,
    1,
    "d36d9c58-2bfe-418c-8512-fc4a3f229535",
    "Marius Bota",
);

// HOB 298 — Rhovanion Rampager (alternate printing)
const RHOVANION_RAMPAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RHOVANION_RAMPAGER,
    1,
    "df7bdf15-9448-4cb0-a809-c5ac128e6b02",
    "Kevin Sidharta",
);

// HOB 299 — The Sackville-Bagginses (alternate printing)
const THE_SACKVILLE_BAGGINSES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SACKVILLE_BAGGINSES,
    1,
    "11144267-15d1-47b4-8c26-d57328e11422",
    "Denman Rooke",
);

// HOB 300 — Supper for Spiders (alternate printing)
const SUPPER_FOR_SPIDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPPER_FOR_SPIDERS,
    1,
    "de7a0139-ce52-4212-9514-b2daba282288",
    "Michele Giorgi",
);

// HOB 301 — Balin, Loremaster (alternate printing)
const BALIN_LOREMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALIN_LOREMASTER,
    1,
    "f3a8e4e3-58c5-4380-94b6-6252f8ccb285",
    "Colin Boyer",
);

// HOB 302 — Dáin Ironfoot (alternate printing)
const DAIN_IRONFOOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAIN_IRONFOOT,
    1,
    "ff35d11c-6429-4aab-b9c8-0c0306f13db9",
    "Tomas Duchek",
);

// HOB 303 — Getaway Barrel (alternate printing)
const GETAWAY_BARREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GETAWAY_BARREL,
    1,
    "8db47144-e048-4996-9f9a-1184dd86151f",
    "Pablo Mendoza",
);

// HOB 304 — Stone-Giant of High Pass (alternate printing)
const STONE_GIANT_OF_HIGH_PASS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STONE_GIANT_OF_HIGH_PASS,
    1,
    "1adfc347-5c69-45e1-829b-aaae19e04b96",
    "Miklós Ligeti",
);

// HOB 305 — Bejeweled Warg (alternate printing)
const BEJEWELED_WARG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEJEWELED_WARG,
    1,
    "2b689d91-1ac0-474b-ae58-2c320e1af5e4",
    "John Di Giovanni",
);

// HOB 306 — Cantankerous Keepers (alternate printing)
const CANTANKEROUS_KEEPERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CANTANKEROUS_KEEPERS,
    1,
    "ace0642f-fa57-4645-9891-43331b4963ed",
    "Ramza Psyru",
);

// HOB 307 — Gigantic Big Bear (alternate printing)
const GIGANTIC_BIG_BEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GIGANTIC_BIG_BEAR,
    1,
    "0b6ed366-24a3-4697-8a87-12c54da7c238",
    "Xabi Gaztelua",
);

// HOB 308 — Part in Friendship (alternate printing)
const PART_IN_FRIENDSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PART_IN_FRIENDSHIP,
    1,
    "46b3b094-0d1b-412e-8655-353ecbf983bb",
    "Jarel Threat",
);

// HOB 309 — Radagast of Rhosgobel (alternate printing)
const RADAGAST_OF_RHOSGOBEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADAGAST_OF_RHOSGOBEL,
    1,
    "3ea01785-add0-487d-b708-5bbf71033899",
    "Anna Podedworna",
);

// HOB 310 — Through the Forest Gate (alternate printing)
const THROUGH_THE_FOREST_GATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THROUGH_THE_FOREST_GATE,
    1,
    "f4408b22-c2b0-46ff-8d10-81d150c992ca",
    "Leon Tukker",
);

// HOB 311 — Dwalin, Weaponmaster (alternate printing)
const DWALIN_WEAPONMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DWALIN_WEAPONMASTER,
    1,
    "6833cb81-a373-4b83-93b4-b1ed5f82ce54",
    "Marco Teixeira",
);

// HOB 312 — Tom, Bert, and William (alternate printing)
const TOM_BERT_AND_WILLIAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOM_BERT_AND_WILLIAM,
    1,
    "3909bdc4-e690-47fa-8cbc-4b437f7ab10c",
    "Leonardo Borazio",
);

// HOB 313 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "c5b472d5-da70-4fd9-a68c-10e17797ad0d",
    "David Petersen",
);

// HOB 314 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "a3ae6956-7f2d-4569-a9f8-3972b5ec5e56",
    "David Petersen",
);

// HOB 315 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "8e07d3e6-c602-446f-a453-f54f13bfc55a",
    "David Petersen",
);

// HOB 316 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "15dd9a36-55df-4942-a7eb-1c49a7999e68",
    "David Petersen",
);

// HOB 317 — Plains (alternate printing)
const PLAINS_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    6,
    "3550f914-cacb-4849-af06-1a7b25c2c95f",
    "David Petersen",
);

// HOB 318 — Plains (alternate printing)
const PLAINS_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    7,
    "f82548c7-24ed-4529-8b57-7407dc0b8c8b",
    "David Petersen",
);

// HOB 319 — Plains (alternate printing)
const PLAINS_ALTERNATE_8: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    8,
    "46c75f8e-6590-4338-abbd-a37e08dbd58c",
    "David Petersen",
);

// HOB 320 — Plains (alternate printing)
const PLAINS_ALTERNATE_9: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    9,
    "128226f4-a733-4f0a-ab2b-7789cfa8b443",
    "David Petersen",
);

// HOB 321 — The Misty Mountains Cold (alternate printing)
const THE_MISTY_MOUNTAINS_COLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MISTY_MOUNTAINS_COLD,
    1,
    "1aaf8a8a-1032-4ee5-9c18-2debb40ec561",
    "Rovina Cai",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LONG_BODIED_GREY_DOG,
    &OLD_THRUSH,
    &TROOP_OF_PONIES,
    &BELLADONNA_TOOK,
    &BILBO_S_GAMBIT,
    &BOFUR_RELIABLE_GUARDIAN,
    &CELEBRATE_THE_MOUNTAIN_KING,
    &DAIN_LORD_OF_THE_IRON_HILLS,
    &DWARVEN_PROVISIONER,
    &DWARVEN_SHORTSWORD,
    &EAGLE_OF_THE_GREAT_SHELF,
    &THE_EAGLES_ARE_COMING,
    &ESGAROTH_GARRISON,
    &FILI_THE_PATHFINDER,
    &GLEAMING_SPLENDOR,
    &IRON_HILLS_BLACKSMITH,
    &KILI_THE_RESOURCEFUL,
    &LAKE_TOWN_LOOKOUT,
    &LAKE_TOWN_TOYMAKER,
    &MAGNIFICENT_END,
    &MOMENT_OF_GLORY,
    &THE_MOUNTAIN_KING_S_RETURN,
    &ORI_KEEPER_OF_SONGS,
    &THE_QUEEN_OF_DALE,
    &ROADS_GO_EVER_EVER_ON,
    &STONE_BY_SUNLIGHT,
    &THORIN_S_LAST_STAND,
    &AN_UNEXPECTED_PARTY,
    &VELVETWING_BUTTERFLIES,
    &VOW_TO_EREBOR,
    &BILBO_LUCKWEARER,
    &BILBO_THIEF_IN_THE_NIGHT,
    &BILBO_BAGGINS_BURGLAR,
    &CONFUSTICATE_AND_BEBOTHER,
    &ELROND_MOON_READER,
    &ELVEN_RAFT_STEERER,
    &ELVENKING_S_HARPER,
    &ENCHANTED_RIVER_S_GRASP,
    &FATEFUL_DISCOVERY,
    &GANDALF_WANDERING_WIZARD,
    &GREAT_GILDED_BOAT,
    &LAKESHORE_APOTHECARY,
    &LAKE_TOWN_MARINERS,
    &LONG_LAKE_NUISANCE,
    &THE_LORD_OF_THE_EAGLES,
    &MASTER_S_COUNCILLORS,
    &MIRKWOOD_MEDITATOR,
    &MOST_DECREPIT_OLD_BIRD,
    &OLD_FAT_SPIDER_CAN_T_SEE_ME,
    &PLUNDER_THE_TROLLSHAWS,
    &RAVENHILL_FLOCK,
    &RIDDLES_IN_THE_DARK,
    &ROLL_ROLL_ROLL_ROLL,
    &SOUND_THE_TRUMPETS,
    &THRANDUIL_S_DECREE,
    &UNCOVER_THE_MOON_LETTERS,
    &UNEASY_PARTINGS,
    &WIZARD_S_STAFF,
    &ALONG_THE_CROOKED_WAY,
    &AZOG_MORIA_S_RUIN,
    &BILBO_S_DEADLY_SLICE,
    &CRUDE_BENT_BLADE,
    &DESOLATION_PROWLER,
    &DOWN_DOWN_TO_GOBLIN_TOWN,
    &DREADED_BAT_CLOUD,
    &FRONT_PORCH_SENTRIES,
    &GATHERING_OF_DARKNESS,
    &GNASHING_OF_TEETH,
    &GOLLUM_RIDDLE_MASTER,
    &GOLLUM_SILENT_SLINKER,
    &GOLLUM_THE_ABANDONED,
    &GREAT_FIERCE_BEE,
    &GREAT_UGLY_LOOKING_GOBLIN,
    &HEAD_OF_THE_HUNT,
    &INSIDE_INFORMATION,
    &THE_MASTER_OF_LAKE_TOWN,
    &NIGHTHOWL_PURSUER,
    &RAGE_INTO_THE_VALLEY,
    &RAVENING_WARG,
    &REVERENT_HOWL,
    &RHOVANION_RAMPAGER,
    &THE_SACKVILLE_BAGGINSES,
    &STIR_UP_TROUBLE,
    &STONY_VOICED_GOBLINS,
    &SUPPER_FOR_SPIDERS,
    &BALIN_LOREMASTER,
    &BOMBUR_GENTLE_DREAMER,
    &BOTHERSOME_NOISEMAKER,
    &BURN_BURN_TREE_AND_FERN,
    &DAIN_IRONFOOT,
    &DESERT_WERE_WORM,
    &DESOLATION_OF_SMAUG,
    &DORI_BEARER_OF_FRIENDS,
    &DWARVEN_MAULER,
    &GANDALF_GOBLINS_BANE,
    &GANDALF_SPARK_STARTER,
    &GETAWAY_BARREL,
    &GLOIN_THE_MIGHTY,
    &GOBLIN_TOWN_FLUNKIES,
    &GUNDABAD_OPPORTUNIST,
    &IRON_HILLS_STALWART,
    &LAST_LIGHT_OF_DURIN_S_DAY,
    &THE_MISTY_MOUNTAINS_COLD,
    &MISTY_MOUNTAINS_RAIDER,
    &OIN_THE_BRAVE,
    &PINECONE_STRIKE,
    &RAGGED_SHORT_SPEAR,
    &SMAUG_THE_GREAT_CALAMITY,
    &SMAUG_THE_MAGNIFICENT,
    &SMAUG_S_FURY,
    &SNOWSLOPE_HUNTER,
    &STONE_GIANT_OF_HIGH_PASS,
    &THORIN_MOUNTAIN_KING,
    &TIDINGS_OF_WAR,
    &ATTERCOP,
    &BEJEWELED_WARG,
    &BEORN_RELUCTANT_HOST,
    &BEORN_THE_FIERCE,
    &BEORN_S_HOSPITALITY,
    &BOUGHSIDE_WANDERERS,
    &CANTANKEROUS_KEEPERS,
    &DANCING_FROM_DARK_TO_DAWN,
    &DOWN_IN_THE_VALLEY,
    &GALION_ELVENKING_S_BUTLER,
    &GIGANTIC_BIG_BEAR,
    &GUARDIAN_OF_THE_HALLS,
    &LITTLE_BEAR,
    &MIRKWOOD_PATHMAKER,
    &NASTY_LITTLE_RABBIT,
    &THE_NOTARY_HOBBITS,
    &OLD_FAT_SPIDER,
    &ORDINARY_BEAR,
    &PART_IN_FRIENDSHIP,
    &QUARREL,
    &RADAGAST_OF_RHOSGOBEL,
    &THROUGH_THE_FOREST_GATE,
    &TROLL_NEGOTIATIONS,
    &WARG_TACTICS,
    &WARGLING,
    &WILDERLAND_SCROUNGER,
    &WOODLAND_WEAVEMASTER,
    &BARD_KING_OF_DALE,
    &BARD_THE_BOWMAN,
    &BARD_S_COMPANY,
    &BIFUR_MELODIC_RIDER,
    &BOLG_OF_THE_NORTH,
    &BOLG_S_COMPANY,
    &THE_CHIEF_WARG,
    &CHIEF_WARG_S_COMPANY,
    &DAIN_S_COMPANY,
    &DUSKWATCH_HUNTER,
    &DWALIN_WEAPONMASTER,
    &EAGLE_S_RESCUE,
    &FEARSOME_GOBLIN_PAIR,
    &GOBLIN_PLATE_MAIL,
    &THE_GREAT_GOBLIN,
    &LARGE_BEAR,
    &MIRKWOOD_NURTURER,
    &NORI_TELLER_OF_TALES,
    &PATIENT_INSTRUCTOR,
    &SILVAN_REVELER,
    &SMAUG_WICKED_WORM,
    &THORIN_OAKENSHIELD,
    &THRANDUIL_SINDARIN_LIEGE,
    &THRANDUIL_THE_ELVENKING,
    &THRANDUIL_S_COMPANY,
    &TOM_BERT_AND_WILLIAM,
    &THE_ARKENSTONE,
    &THE_BLACK_ARROW,
    &DWARVEN_MATTOCK,
    &GIANT_S_BOULDER,
    &GLAMDRING_FOE_HAMMER,
    &KEY_TO_THE_SIDE_DOOR,
    &MY_PRECIOUS,
    &ORCRIST_GOBLIN_CLEAVER,
    &STING_BILBO_S_SWORD,
    &THROR_S_MAP,
    &WELL_WORN_SPATULA,
    &ELVEN_PASSAGE,
    &ELVENKING_S_HALLS,
    &GOBLIN_TOWN,
    &HOBBIT_HOLE,
    &IRON_HILLS,
    &LAKE_TOWN,
    &THE_LONELY_MOUNTAIN,
    &MIRKWOOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    SETTLE_THE_WRECKAGE_REPRINT,
    WOOD_ELVES_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    TROOP_OF_PONIES_ALTERNATE_1,
    RAGE_INTO_THE_VALLEY_ALTERNATE_1,
    THE_GREAT_GOBLIN_ALTERNATE_1,
    THORIN_OAKENSHIELD_ALTERNATE_1,
    GANDALF_SPARK_STARTER_ALTERNATE_1,
    GLAMDRING_FOE_HAMMER_ALTERNATE_1,
    THE_EAGLES_ARE_COMING_ALTERNATE_1,
    DREADED_BAT_CLOUD_ALTERNATE_1,
    THE_LONELY_MOUNTAIN_ALTERNATE_1,
    CHIEF_WARG_S_COMPANY_ALTERNATE_1,
    THORIN_S_LAST_STAND_ALTERNATE_1,
    BARD_S_COMPANY_ALTERNATE_1,
    BOLG_S_COMPANY_ALTERNATE_1,
    DAIN_S_COMPANY_ALTERNATE_1,
    THRANDUIL_S_COMPANY_ALTERNATE_1,
    BELLADONNA_TOOK_ALTERNATE_1,
    BOFUR_RELIABLE_GUARDIAN_ALTERNATE_1,
    IRON_HILLS_BLACKSMITH_ALTERNATE_1,
    THE_QUEEN_OF_DALE_ALTERNATE_1,
    BILBO_LUCKWEARER_ALTERNATE_1,
    BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_1,
    FATEFUL_DISCOVERY_ALTERNATE_1,
    MOST_DECREPIT_OLD_BIRD_ALTERNATE_1,
    AZOG_MORIA_S_RUIN_ALTERNATE_1,
    GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_1,
    HEAD_OF_THE_HUNT_ALTERNATE_1,
    DESERT_WERE_WORM_ALTERNATE_1,
    DESOLATION_OF_SMAUG_ALTERNATE_1,
    GLOIN_THE_MIGHTY_ALTERNATE_1,
    LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_1,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_1,
    BEORN_THE_FIERCE_ALTERNATE_1,
    DANCING_FROM_DARK_TO_DAWN_ALTERNATE_1,
    THE_NOTARY_HOBBITS_ALTERNATE_1,
    THRANDUIL_SINDARIN_LIEGE_ALTERNATE_1,
    THE_ARKENSTONE_ALTERNATE_1,
    MY_PRECIOUS_ALTERNATE_1,
    ORCRIST_GOBLIN_CLEAVER_ALTERNATE_1,
    STING_BILBO_S_SWORD_ALTERNATE_1,
    ELVEN_PASSAGE_ALTERNATE_1,
    GLEAMING_SPLENDOR_ALTERNATE_1,
    THE_LORD_OF_THE_EAGLES_ALTERNATE_1,
    GOLLUM_RIDDLE_MASTER_ALTERNATE_1,
    GANDALF_GOBLINS_BANE_ALTERNATE_1,
    THORIN_MOUNTAIN_KING_ALTERNATE_1,
    BARD_KING_OF_DALE_ALTERNATE_1,
    SMAUG_WICKED_WORM_ALTERNATE_1,
    THRANDUIL_THE_ELVENKING_ALTERNATE_1,
    THE_ARKENSTONE_ALTERNATE_2,
    THE_LONELY_MOUNTAIN_ALTERNATE_2,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_2,
    BELLADONNA_TOOK_ALTERNATE_2,
    BOFUR_RELIABLE_GUARDIAN_ALTERNATE_2,
    IRON_HILLS_BLACKSMITH_ALTERNATE_2,
    THE_QUEEN_OF_DALE_ALTERNATE_2,
    BILBO_LUCKWEARER_ALTERNATE_2,
    BILBO_THIEF_IN_THE_NIGHT_ALTERNATE_2,
    FATEFUL_DISCOVERY_ALTERNATE_2,
    MOST_DECREPIT_OLD_BIRD_ALTERNATE_2,
    AZOG_MORIA_S_RUIN_ALTERNATE_2,
    GREAT_UGLY_LOOKING_GOBLIN_ALTERNATE_2,
    HEAD_OF_THE_HUNT_ALTERNATE_2,
    DESERT_WERE_WORM_ALTERNATE_2,
    DESOLATION_OF_SMAUG_ALTERNATE_2,
    GLOIN_THE_MIGHTY_ALTERNATE_2,
    LAST_LIGHT_OF_DURIN_S_DAY_ALTERNATE_2,
    SMAUG_THE_MAGNIFICENT_ALTERNATE_3,
    BEORN_THE_FIERCE_ALTERNATE_2,
    DANCING_FROM_DARK_TO_DAWN_ALTERNATE_2,
    THE_NOTARY_HOBBITS_ALTERNATE_2,
    THRANDUIL_SINDARIN_LIEGE_ALTERNATE_2,
    THE_ARKENSTONE_ALTERNATE_3,
    MY_PRECIOUS_ALTERNATE_2,
    ORCRIST_GOBLIN_CLEAVER_ALTERNATE_2,
    STING_BILBO_S_SWORD_ALTERNATE_2,
    ELVEN_PASSAGE_ALTERNATE_2,
    GLEAMING_SPLENDOR_ALTERNATE_2,
    THE_LORD_OF_THE_EAGLES_ALTERNATE_2,
    GOLLUM_RIDDLE_MASTER_ALTERNATE_2,
    GANDALF_GOBLINS_BANE_ALTERNATE_2,
    THORIN_MOUNTAIN_KING_ALTERNATE_2,
    BARD_KING_OF_DALE_ALTERNATE_2,
    SMAUG_WICKED_WORM_ALTERNATE_2,
    THRANDUIL_THE_ELVENKING_ALTERNATE_2,
    THE_ARKENSTONE_ALTERNATE_4,
    THE_LONELY_MOUNTAIN_ALTERNATE_3,
    BILBO_S_GAMBIT_ALTERNATE_1,
    FILI_THE_PATHFINDER_ALTERNATE_1,
    KILI_THE_RESOURCEFUL_ALTERNATE_1,
    SETTLE_THE_WRECKAGE_ALTERNATE_1,
    AN_UNEXPECTED_PARTY_ALTERNATE_1,
    ELROND_MOON_READER_ALTERNATE_1,
    GREAT_GILDED_BOAT_ALTERNATE_1,
    RIDDLES_IN_THE_DARK_ALTERNATE_1,
    UNCOVER_THE_MOON_LETTERS_ALTERNATE_1,
    WIZARD_S_STAFF_ALTERNATE_1,
    ALONG_THE_CROOKED_WAY_ALTERNATE_1,
    INSIDE_INFORMATION_ALTERNATE_1,
    THE_MASTER_OF_LAKE_TOWN_ALTERNATE_1,
    RHOVANION_RAMPAGER_ALTERNATE_1,
    THE_SACKVILLE_BAGGINSES_ALTERNATE_1,
    SUPPER_FOR_SPIDERS_ALTERNATE_1,
    BALIN_LOREMASTER_ALTERNATE_1,
    DAIN_IRONFOOT_ALTERNATE_1,
    GETAWAY_BARREL_ALTERNATE_1,
    STONE_GIANT_OF_HIGH_PASS_ALTERNATE_1,
    BEJEWELED_WARG_ALTERNATE_1,
    CANTANKEROUS_KEEPERS_ALTERNATE_1,
    GIGANTIC_BIG_BEAR_ALTERNATE_1,
    PART_IN_FRIENDSHIP_ALTERNATE_1,
    RADAGAST_OF_RHOSGOBEL_ALTERNATE_1,
    THROUGH_THE_FOREST_GATE_ALTERNATE_1,
    DWALIN_WEAPONMASTER_ALTERNATE_1,
    TOM_BERT_AND_WILLIAM_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    PLAINS_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    PLAINS_ALTERNATE_6,
    PLAINS_ALTERNATE_7,
    PLAINS_ALTERNATE_8,
    PLAINS_ALTERNATE_9,
    THE_MISTY_MOUNTAINS_COLD_ALTERNATE_1,
];
